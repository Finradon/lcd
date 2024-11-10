use clap::{Arg, ArgMatches, Command};

/// Creates the argument parser
pub fn parse_args() -> ArgMatches {
    Command::new("lcd")
        .about("Search and display spell information from Liber Cantiones")
        .arg(
            Arg::new("query")
                .help("The name or part of the spell name to search for")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("field")
                .short('f')  // Short option for --field
                .long("field")
                .help("Specify fields to display (e.g., --field probe technik)")
                .num_args(1..)  // Accepts one or more values
                .value_name("FIELD")
        )
        .arg(
            Arg::new("all")
                .short('a')  // Short option for --all
                .long("all")
                .help("Display all available data for the spell")
                .action(clap::ArgAction::SetTrue)  // This makes `--all` a flag without a value
                .conflicts_with("field")
        )
        .get_matches()
}
