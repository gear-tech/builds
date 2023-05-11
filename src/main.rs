use anyhow::Result;
use chrono::offset::Utc;
use handlebars::Handlebars;
use std::{collections::HashMap, env, fs, path::Path};

const BINARIES: &[(&str, &str)] = &[
    ("gear-nightly-x86_64-unknown-linux-gnu", "tar.xz"),
    ("gear-nightly-aarch64-apple-darwin", "tar.gz"),
    ("gear-nightly-x86_64-apple-darwin", "tar.gz"),
    ("gear-nightly-x86_64-pc-windows-msvc", "zip"),
    ("gear-v0.1.5-x86_64-unknown-linux-gnu", "tar.xz"),
    ("gear-v0.1.5-aarch64-apple-darwin", "tar.gz"),
    ("gear-v0.1.5-x86_64-apple-darwin", "tar.gz"),
    ("gear-v0.1.5-x86_64-pc-windows-msvc", "zip"),
    ("gear-v0.1.4-x86_64-unknown-linux-gnu", "tar.xz"),
    ("gear-v0.1.4-aarch64-apple-darwin", "tar.gz"),
    ("gear-v0.1.4-x86_64-apple-darwin", "tar.gz"),
    ("gear-v0.1.4-x86_64-pc-windows-msvc", "zip"),
    ("gear-v0.1.3-x86_64-unknown-linux-gnu", "tar.xz"),
    ("gear-v0.1.3-aarch64-apple-darwin", "tar.gz"),
    ("gear-v0.1.3-x86_64-apple-darwin", "tar.gz"),
    ("gear-v0.1.3-x86_64-pc-windows-msvc", "zip"),
    ("gear-v0.1.2-x86_64-unknown-linux-gnu", "tar.xz"),
    ("gear-v0.1.2-aarch64-apple-darwin", "tar.gz"),
    ("gear-v0.1.2-x86_64-apple-darwin", "tar.gz"),
    ("gear-v0.1.2-x86_64-pc-windows-msvc", "zip"),
    ("gear-v0.1.1-x86_64-unknown-linux-gnu", "tar.xz"),
    ("gear-v0.1.1-aarch64-apple-darwin", "tar.gz"),
    ("gear-v0.1.1-x86_64-apple-darwin", "tar.gz"),
    ("gear-v0.1.1-x86_64-pc-windows-msvc", "zip"),
    ("vara-testnet-x86_64-unknown-linux-gnu", "tar.xz"),
    ("vara-testnet-aarch64-apple-darwin", "tar.gz"),
    ("vara-testnet-x86_64-apple-darwin", "tar.gz"),
    ("vara-testnet-x86_64-pc-windows-msvc", "zip"),
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
