use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]  // Applies PascalCase conversion to all fields
// #[allow(dead_code)]
pub struct Spell {
    pub probe: String,
    pub technik: String,
    pub zauberdauer: String,
    pub wirkung: String,
    pub kosten: String,
    pub zielobjekt: String,
    pub reichweite: String,
    pub wirkungsdauer: String,
    pub reversalis: String,
    pub antimagie: String,
    pub merkmale: String,
    pub komplexität: String,
    pub repräsentationen: String,
    pub hintergrund: String,
    pub modifikationen: String,
    pub varianten: Option<Vec<String>>,
}

// Function to load and parse the JSON file
pub fn load_spells_from_file(path: &str) -> Result<HashMap<String, Spell>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let spells: HashMap<String, Spell> = serde_json::from_reader(reader)?;
    Ok(spells)
}
