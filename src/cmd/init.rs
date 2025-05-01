use anyhow::Result;
use std::fs;

pub fn run_init(format: &str) -> Result<()> {
    let sample = include_str!("../../examples/threat.yaml");
    let filename = format!("threat.{}", format);
    fs::write(&filename, sample)?;
    println!("Created sample config: {}", filename);
    Ok(())
}