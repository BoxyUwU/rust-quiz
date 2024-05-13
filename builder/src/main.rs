use std::{path::Path, thread};

use eyre::{Context, OptionExt};

fn main() -> eyre::Result<()> {
    let root_dir = Path::new("..");
    let examples_dir = root_dir.join("code").join("examples");

    let examples = std::fs::read_dir(&examples_dir)
        .wrap_err("opening ../code/examples, script must be run in ./builder")?;

    install_toolchain(&examples_dir).wrap_err("install toolchain")?;

    // Setup miri to avoid race condition in `cargo miri run` below...
    eprintln!("Setting up miri");
    std::process::Command::new("cargo")
        .arg("miri")
        .arg("setup")
        .output()
        .wrap_err("setting up miri")?;

    thread::scope(|scope| {
        let mut handles = Vec::new();

        for example in examples {
            handles.push(scope.spawn(|| {
                let example = example.wrap_err("reading example dir entry")?;
                if example
                    .file_type()
                    .wrap_err("getting file type of entry")?
                    .is_dir()
                {
                    return Ok(());
                }

                run_example(&examples_dir, &example.file_name().to_str().unwrap())
                    .wrap_err_with(|| format!("running {:?}", example.file_name()))
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

    Ok(())
}

fn run_example(examples_dir: &Path, filename: &str) -> eyre::Result<()> {
    let use_miri = filename.starts_with("unsafe_");

    let example_name = filename
        .strip_suffix(".rs")
        .ok_or_eyre("example filename does not end in .rs")?;

    eprintln!("Running {example_name}");

    let mut cmd = std::process::Command::new("cargo");
    cmd.current_dir(examples_dir);
    if use_miri {
        cmd.arg("miri");
    }
    cmd.arg("run").arg("--quiet").arg("--example");
    cmd.arg(example_name);

    remove_rustup_vars(&mut cmd);

    let out = cmd.output().wrap_err("spawning cargo")?;
    let stderr = String::from_utf8(out.stderr).wrap_err("stderr was invalid UTF-8")?;

    let stderr_dir = examples_dir.join("stderr");
    let path = stderr_dir.join(format!("{example_name}.stderr"));
    std::fs::write(&path, stderr)
        .wrap_err_with(|| format!("writing stderr to {}", path.display()))?;

    Ok(())
}

// Ensure there is output for the toolchain and that the installation doesn't pollute stderr.
fn install_toolchain(examples_dir: &Path) -> eyre::Result<()> {
    let mut toolchain_install = std::process::Command::new("rustc");
    toolchain_install.arg("-V");
    toolchain_install.current_dir(examples_dir);
    remove_rustup_vars(&mut toolchain_install);
    toolchain_install
        .spawn()
        .wrap_err("failed to spawn rustc")?
        .wait()
        .wrap_err("failed to wait for rustc")?;

    Ok(())
}

fn remove_rustup_vars(cmd: &mut std::process::Command) {
    // Ensure rustup picks up the rust-toolchain.toml file properly and doesn't get confused by this cargo run.
    cmd.env_remove("CARGO");
    cmd.env_remove("RUSTUP_TOOLCHAIN");
}
