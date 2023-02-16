use anyhow::Result;
use chrono::offset::Utc;
use handlebars::Handlebars;
use std::{collections::HashMap, env, fs, path::Path};

const BINARIES: &[(&str, &str)] = &[
    ("gear-v0.1-pre-linux-x86_64", "tar.xz"),
    ("gear-v0.1-pre-macos-m", "tar.gz"),
    ("gear-v0.1-pre-macos-x86_64", "tar.gz"),
    ("gear-v0.1-pre-windows-x86_64", "zip"),
    ("gear-nightly-linux-x86_64", "tar.xz"),
    ("gear-nightly-macos-m", "tar.gz"),
    ("gear-nightly-macos-x86_64", "tar.gz"),
    ("gear-nightly-windows-x86_64", "zip"),
];

fn collect_info(dir: impl AsRef<Path>) -> HashMap<String, String> {
    let mut info = HashMap::new();
    for &(base_name, ext) in BINARIES {
        // Calculate file size
        let file_path = dir.as_ref().join(format!("{base_name}.{ext}"));
        let size_mb = fs::metadata(&file_path).map(|m| m.len()).unwrap_or(0) / 1048576;
        let base_key = base_name.replace('.', "-");
        info.insert(format!("{base_key}-size"), size_mb.to_string());

        // Get version
        if let Ok(version) =
            fs::read_to_string(dir.as_ref().join(format!("{base_name}-version.txt")))
        {
            let version = version.trim();
            if !version.is_empty() {
                info.insert(
                    format!("{base_key}-version"),
                    format!(" ({})", version.trim()),
                );
            }
        }
    }
    info.insert(
        "now".to_string(),
        Utc::now().format("%Y-%m-%d %H:%M UTC").to_string(),
    );
    info
}

fn main() -> Result<()> {
    let out_dir = env::current_dir()?.join("artifact");
    fs::create_dir_all(&out_dir)?;

    let index_html = out_dir.join("index.html");
    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("index", include_str!("index.hbs"))?;

    let info = collect_info(&out_dir);
    fs::write(index_html, handlebars.render("index", &info)?)?;

    Ok(())
}
