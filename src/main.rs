use std::path::Path;
mod args;
mod data_loader;
mod formatter;
mod search;
mod display;

use args::parse_args;
use display::print_spell;
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
            if show_all {
                formatter::print_spell_name(best_match);
                formatter::print_all_fields(spell);
            } else {
                print_spell(spell, &matches, fields.as_ref(), best_match);
            }
        }
    } else {
        println!("\nNo matches found for '{}'", query);
    }
}
