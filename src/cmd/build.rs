use crate::model::ThreatModel;
use anyhow::Result;
use std::{fs, path::Path};
use tera::{Context, Tera};

pub fn run_build(path: &str, out_dir: &str, inline: bool) -> Result<()> {
    let data = fs::read_to_string(path)?;
    let model: ThreatModel = serde_yaml::from_str(&data)?;
    let tera = Tera::new("templates/**/*")?;
    let mut ctx = Context::new();
    ctx.insert("model", &model);
    ctx.insert("inline_assets", &inline);
    if inline {
        let style_css = fs::read_to_string("templates/styles.css")?;
        let script_js = fs::read_to_string("templates/app.js")?;
        ctx.insert("style_css", &style_css);
        ctx.insert("script_js", &script_js);
    }
    let rendered = tera.render("index.html.tera", &ctx)?;
    let out = Path::new(out_dir);
    fs::create_dir_all(out)?;
    fs::write(out.join("index.html"), rendered)?;
    if !inline {
        fs::copy("templates/app.js", out.join("app.js"))?;
        fs::copy("templates/styles.css", out.join("styles.css"))?;
    }
    println!("Built site at {}/index.html", out_dir);
    Ok(())
}