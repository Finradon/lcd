use std::path::Path;
mod args;
mod data_loader;
mod formatter;
mod search;

use args::parse_args;
use formatter::{print_all_fields, print_field, print_list, print_spell_name};
use search::fuzzy_search_best_match;

fn main() {
    let matches = parse_args();

    // Get the query from the command line
    let query = matches.get_one::<String>("query").unwrap();
    let fields: Option<Vec<&String>> = matches
        .get_many::<String>("field")
        .map(|vals| vals.collect());
    let show_all = matches.get_flag("all");

    // Determine the path to the JSON file
    // let default_path = "./data/libercantiones.json";
    let default_path = "./data/libercantiones.json";
    let fallback_path = "/usr/local/share/lcd/libercantiones.json";
    let json_path = if Path::new(default_path).exists() {
        default_path
    } else if Path::new(fallback_path).exists() {
        fallback_path
    } else {
        eprintln!("Error: libercantiones.json not found in either ./data/ or /usr/share/lcd/");
        return;
    };

    // Load spells from the determined path
    let spells = match data_loader::load_spells_from_file(json_path) {
        Ok(spells) => spells,
        Err(e) => {
            eprintln!("Error loading spells: {}", e);
            return;
        }
    };

    // Search for the best matching spell name
    let tolerance = 4; // Adjust as needed for fuzzy search
    if let Some(best_match) = fuzzy_search_best_match(&spells, query, tolerance) {
        if let Some(spell) = spells.get(best_match) {
            // Handle displaying all data or specific fields
            if show_all {
                print_spell_name(best_match);
                print_all_fields(spell); // Pretty print all details
            } else if let Some(field_list) = fields {
                print_spell_name(best_match);
                for field_name in field_list {
                    match field_name.to_lowercase().as_str() {
                        "probe" => print_field("Probe", &spell.probe),
                        "technik" => print_field("Technik", &spell.technik),
                        "zauberdauer" => print_field("Zauberdauer", &spell.zauberdauer),
                        "wirkung" => print_field("Wirkung", &spell.wirkung),
                        "kosten" => print_field("Kosten", &spell.kosten),
                        "merkmal" => print_field("Merkmale", &spell.merkmale),
                        "reichweite" => print_field("Reichweite", &spell.reichweite),
                        "wirkungsdauer" => print_field("Wirkungsdauer", &spell.wirkungsdauer),
                        "zielobjekt" => print_field("Zielobjekt", &spell.zielobjekt),
                        "reversalis" => print_field("Reversalis", &spell.reversalis),
                        "antimagie" => print_field("Antimagie", &spell.antimagie),
                        "komplexität" => print_field("Komplexität", &spell.komplexität),
                        "repräsentationen" => {
                            print_field("Repräsentationen", &spell.repräsentationen)
                        }
                        "hintergrund" => print_field("Hintergrund", &spell.hintergrund),
                        "modifikationen" => print_field("Modifikationen", &spell.modifikationen),
                        "varianten" => {
                            if let Some(varianten) = &spell.varianten {
                                print_list("Varianten", varianten);
                            } else {
                                print_field("Varianten", "Keine bekannt");
                            }
                        }
                        _ => println!("Field '{}' not recognized.", field_name),
                    }
                }
            } else {
                // Default output (most important information)
                print_spell_name(best_match);
                print_field("Probe", &spell.probe);
                print_field("Zauberdauer", &spell.zauberdauer);
                print_field("Wirkung", &spell.wirkung);
                print_field("Kosten", &spell.kosten);
                print_field("Zielobjekt", &spell.zielobjekt);
                print_field("Reichweite", &spell.reichweite);
                print_field("Wirkungsdauer", &spell.wirkungsdauer);
                print_field("Modifikationen", &spell.modifikationen);
            }
        }
    } else {
        println!("\nNo matches found for '{}'", query);
    }
}
