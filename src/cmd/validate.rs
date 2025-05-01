use crate::model::ThreatModel;
use anyhow::{bail, Result};
use std::collections::HashSet;
use std::fs;

pub fn run_validate(path: &str) -> Result<()> {
    let data = fs::read_to_string(path)?;
    let model: ThreatModel = serde_yaml::from_str(&data)?;
    let class_ids: HashSet<_> = model.adversary_classes.iter().map(|c| &c.id).collect();
    let adv_ids: HashSet<_> = model.adversaries.iter().map(|a| &a.id).collect();
    let sce_ids: HashSet<_> = model.scenarios.iter().map(|s| &s.id).collect();

    for adv in &model.adversaries {
        for cls in &adv.classes {
            if !class_ids.contains(cls) { bail!("Unknown class '{}' in adversary {}", cls, adv.id); }
        }
    }
    for inv in &model.invariants {
        for a in &inv.adversaries { if !adv_ids.contains(a) { bail!("Unknown adversary '{}' in invariant {}", a, inv.id); } }
        for s in &inv.scenarios  { if !sce_ids.contains(s) { bail!("Unknown scenario '{}' in invariant {}", s, inv.id); } }
    }
    for w in &model.weaknesses {
        for a in &w.adversaries { if !adv_ids.contains(a) { bail!("Unknown adversary '{}' in weakness {}", a, w.id); } }
        for s in &w.scenarios  { if !sce_ids.contains(s) { bail!("Unknown scenario '{}' in weakness {}", s, w.id); } }
    }
    println!("Validation succeeded: {}", path);
    Ok(())
}