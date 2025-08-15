use std::{collections::HashMap, path::Path, str::FromStr, thread};

use eyre::{bail, ensure, Context, Ok, OptionExt};

fn main() -> eyre::Result<()> {
    // Ensure rustup picks up the rust-toolchain.toml file properly and doesn't get confused by this cargo run.
    std::env::remove_var("CARGO");
    std::env::remove_var("RUSTUP_TOOLCHAIN");

    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let examples_dir = root_dir.join("code").join("examples");

    // Change the current directory to ensure the correct rustup toolchains are used.
    std::env::set_current_dir(&examples_dir)
        .wrap_err("changing current directory to code/examples")?;

    let example_files = std::fs::read_dir(&examples_dir)
        .wrap_err("opening ../code/examples, script must be run in ./builder")?;

    install_toolchain().wrap_err("install toolchain")?;
    // Setup miri to avoid race condition in `cargo miri run` below...
    setup_miri(&examples_dir).wrap_err("setting up miri sysroot")?;

    let mut examples = Vec::new();
    for example in example_files {
        let example = example.wrap_err("reading example dir entry")?;
        if example
            .file_type()
            .wrap_err("getting file type of entry")?
            .is_dir()
        {
            continue;
        }
        examples.push(example.file_name().to_str().unwrap().to_owned());
    }

    thread::scope(|scope| {
        let mut handles = Vec::new();

        for example in &examples {
            let examples_dir = &examples_dir;
            handles.push(scope.spawn(move || {
                run_example(&examples_dir, example)
                    .wrap_err_with(|| format!("running {:?}", example))
            }));
        }

        handles
            .into_iter()
            .map(|handle| {
                handle
                    .join()
                    .unwrap_or_else(|payload| std::panic::resume_unwind(payload))
            })
            .filter_map(|res| res.err())
            .for_each(|err| eprint!("error while running example: {err}"));
    });

    let mut questions: HashMap<QName, Question> = HashMap::new();

    #[derive(Default)]
    struct Question {
        examples: Vec<String>,
        header: Option<String>,
        explanation: Option<String>,
    }

    for example in examples {
        let name = example.parse::<QName>()?;

        let question = questions.entry(name).or_default();
        question.examples.push(example);
    }

    let explanations =
        std::fs::read_dir(root_dir.join("explanations")).wrap_err("failed to read explanations")?;
    for expl in explanations {
        let expl = expl?;
        let name = expl.file_name().to_str().unwrap().parse::<QName>()?;
        let expl = std::fs::read_to_string(expl.path()).wrap_err("reading explanation")?;
        let Some((header, expl)) = expl.split_once('\n') else {
            bail!("explanation is missing header");
        };
        let question = questions.entry(name).or_default();
        question.header = Some(header.to_owned());
        question.explanation = Some(expl.trim().to_owned());
    }

    let xxx = root_dir.join("xxx");
    std::fs::remove_dir_all(&xxx)?;
    std::fs::create_dir_all(&xxx)?;
    for (qname, q) in questions {
        let filename = xxx.join(format!("{}_{}.md", qname.category, qname.number));

        let mut content = String::new();

        let Some(header) = q.header else {
            // TODO: this should be an error
            continue;
        };

        content.push_str(&header);
        content.push_str("\n\n");
        content.push_str("{{#include ../src/include/quiz-is-wip.md}}\n\n");

        for example in &q.examples {
            content.push_str("```rust\n");
            content.push_str(&format!("{{{{#include ../code/examples/{example}}}}}\n"));
            content.push_str("```\n");
        }

        content.push_str("<details><summary>Solution</summary>\n\n");
        for example in &q.examples {
            content.push_str("```rust\n");
            let example = example.replace(".rs", ".stderr");
            content.push_str(&format!(
                "{{{{#include ../code/examples/stderr/{example}}}}}\n"
            ));
            content.push_str("```\n");
        }
        content.push_str("\n");
        content.push_str(&q.explanation.unwrap_or_default());
        content.push_str("\n\n");
        content.push_str("</details>\n");

        std::fs::write(filename, content).wrap_err("writing output file")?;
    }

    Ok(())
}

#[derive(PartialEq, Eq, Hash)]
struct QName {
    category: String,
    number: String,
}
impl FromStr for QName {
    type Err = eyre::Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.split('.').next().unwrap_or_default();
        let mut parts = s.split('_');
        let mut category = parts
            .next()
            .ok_or_eyre("category missing in file name")?
            .to_owned();
        let mut number = parts.next().ok_or_eyre("number missing in file name")?;
        if number.parse::<u16>().is_err() {
            // the category has an underscore
            category = format!("{category}_{number}");
            number = parts.next().ok_or_eyre("number missing in file name")?;
        }
        Ok(Self {
            category,
            number: number.to_owned(),
        })
    }
}

fn setup_miri(dir: &Path) -> eyre::Result<()> {
    eprintln!("Setting up miri");
    let output = std::process::Command::new("cargo")
        .arg("miri")
        .arg("setup")
        .current_dir(dir)
        .spawn()
        .wrap_err("spawning miri")?
        .wait()
        .wrap_err("waiting for miri setup")?;
    ensure!(output.success());
    Ok(())
}

fn run_example(examples_dir: &Path, filename: &str) -> eyre::Result<()> {
    let use_miri = filename.starts_with("unsafe_");

    let example_name = filename
        .strip_suffix(".rs")
        .ok_or_eyre("example filename does not end in .rs")?;

    eprintln!("Running {example_name}");

    let mut cmd = std::process::Command::new("cargo");
    if use_miri {
        cmd.arg("miri");
    }
    cmd.arg("run").arg("--quiet").arg("--example");
    cmd.arg(example_name);

    let out = cmd.output().wrap_err("spawning cargo")?;
    let stderr = String::from_utf8(out.stderr).wrap_err("stderr was invalid UTF-8")?;

    let stderr_dir = examples_dir.join("stderr");
    let path = stderr_dir.join(format!("{example_name}.stderr"));
    std::fs::write(&path, stderr)
        .wrap_err_with(|| format!("writing stderr to {}", path.display()))?;

    Ok(())
}

/// Ensures there is output for the toolchain and that the installation doesn't pollute stderr.
fn install_toolchain() -> eyre::Result<()> {
    let mut toolchain_install = std::process::Command::new("rustc");
    toolchain_install.arg("-V");
    let output = toolchain_install
        .spawn()
        .wrap_err("failed to spawn rustc")?
        .wait()
        .wrap_err("failed to wait for rustc")?;
    ensure!(output.success());
    Ok(())
}
