use std::path::Path;

use eyre::{Context, OptionExt};

fn main() -> eyre::Result<()> {
    let root_dir = Path::new("..");
    let examples_dir = root_dir.join("code").join("examples");

    let examples = std::fs::read_dir(&examples_dir)
        .wrap_err("opening ../code/examples, script must be run in ./builder")?;

    for example in examples {
        let example = example.wrap_err("reading example dir entry")?;
        if example
            .file_type()
            .wrap_err("getting file type of entry")?
            .is_dir()
        {
            continue;
        }

        run_example(&examples_dir, &example.file_name().to_str().unwrap())
            .wrap_err_with(|| format!("running {:?}", example.file_name()))?;
    }

    Ok(())
}

fn run_example(examples_dir: &Path, filename: &str) -> eyre::Result<()> {
    let use_miri = filename.starts_with("unsafe_");

    let example_name = filename
        .strip_suffix(".rs")
        .ok_or_eyre("example filename does not end in .rs")?;

    eprintln!("Running {example_name}");

    let mut proc = std::process::Command::new("cargo");
    proc.current_dir(examples_dir);
    if use_miri {
        proc.arg("miri");
    }
    proc.arg("run").arg("--quiet").arg("--example");
    proc.arg(example_name);

    let out = proc.output().wrap_err("spawning cargo")?;
    let stderr = String::from_utf8(out.stderr).wrap_err("stderr was invalid UTF-8")?;

    let stderr_dir = examples_dir.join("stderr");
    let path = stderr_dir.join(format!("{example_name}.stderr"));
    std::fs::write(&path, stderr)
        .wrap_err_with(|| format!("writing stderr to {}", path.display()))?;

    Ok(())
}
