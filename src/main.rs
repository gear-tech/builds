use anyhow::Result;
use handlebars::Handlebars;
use std::{collections::HashMap, env, fs, path::Path};

const BINARIES: [&str; 4] = [
    "gear-nightly-linux-x86_64.tar.xz",
    "gear-nightly-macos-m.tar.gz",
    "gear-nightly-macos-x86_64.tar.gz",
    "gear-nightly-windows-x86_64.zip",
];

fn collect_info(dir: impl AsRef<Path>) -> HashMap<String, u64> {
    let delimiters = ['-', '.'];
    BINARIES
        .map(|file| {
            let file_path = dir.as_ref().join(file);
            let size_mb = fs::metadata(file_path).map(|m| m.len()).unwrap_or(0) / 1048576;
            let key = file.split_terminator(&delimiters).take(4).collect::<Vec<_>>().join("-");
            (key, size_mb)
        })
        .into()
}

fn main() -> Result<()> {
    let out_dir = env::current_dir()?.join("artifact");
    fs::create_dir_all(&out_dir)?;

    let index_html = out_dir.join("index.html");
    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("index", include_str!("index.hbs"))?;

    let info = collect_info(&out_dir);
    fs::write(&index_html, handlebars.render("index", &info)?)?;

    Ok(())
}
