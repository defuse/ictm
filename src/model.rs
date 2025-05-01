use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AdversaryClass { pub id: String, pub name: String, pub description: String }

#[derive(Debug, Serialize, Deserialize)]
pub struct Adversary { pub id: String, pub name: String, pub classes: Vec<String>, pub capabilities: Vec<String>, pub goals: Vec<String> }

#[derive(Debug, Serialize, Deserialize)]
pub struct Scenario { pub id: String, pub description: String, #[serde(default)] pub always_on: bool }

#[derive(Debug, Serialize, Deserialize)]
pub struct Invariant { pub id: String, pub description: String, pub adversaries: Vec<String>, pub scenarios: Vec<String> }

#[derive(Debug, Serialize, Deserialize)]
pub struct Weakness { pub id: String, pub description: String, pub severity: String, pub adversaries: Vec<String>, pub scenarios: Vec<String> }

#[derive(Debug, Serialize, Deserialize)]
pub struct ThreatModel { pub adversary_classes: Vec<AdversaryClass>, pub adversaries: Vec<Adversary>, pub scenarios: Vec<Scenario>, pub invariants: Vec<Invariant>, pub weaknesses: Vec<Weakness> }