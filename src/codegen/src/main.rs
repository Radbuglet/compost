use std::{env::current_dir, fs, path::PathBuf};

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    println!("Running in working directory {:?}.", current_dir()?);

    // Collect files
    println!(r#"Scanning for ".jinja" files..."#);
    let files = {
        const GLOB_ERR: &str = r#"failed to glob for ".jinja" files."#;

        glob::glob("**/*.jinja")
            .context(GLOB_ERR)?
            .map(|path_res| {
                path_res.context(GLOB_ERR).map(|path| {
                    (
                        path.file_name().unwrap().to_str().unwrap().to_string(),
                        path,
                    )
                })
            })
            .collect::<anyhow::Result<Vec<(String, PathBuf)>>>()?
    };

    // Collect sources
    println!(r#"Parsing ".jinja" files..."#);
    let source = {
        let mut source = minijinja::Source::new();

        for (name, path) in &files {
            println!("\tParsing {path:?}.");

            let file_contents =
                fs::read_to_string(path).context(r#"failed to read ".jinja" file."#)?;

            source
                .add_template(name, file_contents)
                .with_context(|| format!(r#"failed to parse ".jinja" file at path {:?}"#, path))?;
        }

        source
    };

    // Build environment
    let mut env = minijinja::Environment::new();
    env.set_source(source);

    let context = minijinja::context!();

    // Evaluate sources
    println!(r#"Evaluating ".jinja" files..."#);
    for (name, path) in &files {
        let out_path = path.with_extension("");
        println!("\tEvaluating {path:?} -> {out_path:?}.");

        let result = env.get_template(name)?.render(&context)?;
        fs::write(out_path, result)?;
    }

    Ok(())
}
