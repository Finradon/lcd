use colored::*;
use crate::data_loader::Spell;

/// Prints the spell name in yellow without a label.
pub fn print_spell_name(name: &str) {
    println!("{}", name.bold().yellow());  // Print the spell name in bold yellow
}

/// Formats and prints the specified field with bold and colored labels, indented by four spaces.
/// If the value is long, it wraps the text to a specified width.
pub fn print_field(label: &str, value: &str) {
    let formatted_label = label.bold().blue();  // Label in bold blue text
    println!("{}: {}", formatted_label, wrap_text(value, 80));  // Indented with four spaces
}

/// Formats and prints a list of strings (used for 'Varianten') in a bullet-point style, wrapping long lines.
pub fn print_list(label: &str, list: &[String]) {
    let formatted_label = label.bold().blue();
    println!("{}:", formatted_label);  // Indented with four spaces
    for item in list.iter() {
        // Wrap the text for each item and print it with further indentation
        let wrapped_item = wrap_text(item, 76);  // Slightly narrower width for the bullet and spaces
        println!("{} {}", "🔹".bold().green(), wrapped_item.replace("\n", "\n        "));  // Indent wrapped lines
    }
}

/// Wraps text at the specified width, breaking lines only at word boundaries.
fn wrap_text(text: &str, width: usize) -> String {
    let mut wrapped_text = String::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        if current_line.len() + word.len() + 1 > width {
            // Add the current line to the wrapped text and start a new line
            wrapped_text.push_str(&format!("{}\n", current_line.trim_end()));
            current_line.clear();
        }
        current_line.push_str(&format!("{} ", word));
    }

    // Add any remaining text to the wrapped text
    if !current_line.is_empty() {
        wrapped_text.push_str(&current_line.trim_end());
    }

    wrapped_text
}

/// Pretty prints all fields of a spell when `--all` is passed, with each field indented.
pub fn print_all_fields(spell: &Spell) {
    print_field("Probe", &spell.probe);
    print_field("Technik", &spell.technik);
    print_field("Zauberdauer", &spell.zauberdauer);
    print_field("Wirkung", &spell.wirkung);
    print_field("Kosten", &spell.kosten);
    print_field("Zielobjekt", &spell.zielobjekt);
    print_field("Reichweite", &spell.reichweite);
    print_field("Wirkungsdauer", &spell.wirkungsdauer);
    print_field("Modifikationen", &spell.modifikationen);
    if let Some(varianten) = &spell.varianten {
        print_list("Varianten", varianten);
    } else {
        print_field("Varianten", "None");
    }
    print_field("Reversalis", &spell.reversalis);
    print_field("Antimagie", &spell.antimagie);
    print_field("Merkmale", &spell.merkmale);
    print_field("Komplexität", &spell.komplexität);
    print_field("Repräsentationen", &spell.repräsentationen);
    print_field("Hintergrund", &spell.hintergrund);
}
