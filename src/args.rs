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
        // Add a flag for each field in the Spell struct with short options for specific fields
        .arg(Arg::new("probe").short('p').long("probe").help("Show the Probe field").action(clap::ArgAction::SetTrue))
        .arg(Arg::new("wirkung").short('w').long("wirkung").help("Show the Wirkung field").action(clap::ArgAction::SetTrue))
        .arg(Arg::new("kosten").short('k').long("kosten").help("Show the Kosten field").action(clap::ArgAction::SetTrue))
        .arg(Arg::new("reichweite").short('r').long("reichweite").help("Show the Reichweite field").action(clap::ArgAction::SetTrue))
        .arg(Arg::new("zauberdauer").short('z').long("zauberdauer").help("Show the Zauberdauer field").action(clap::ArgAction::SetTrue))
        .arg(Arg::new("modifikationen").short('m').long("modifikationen").help("Show the Modifikationen field").action(clap::ArgAction::SetTrue))
        .arg(Arg::new("varianten").short('v').long("varianten").help("Show the Varianten field").action(clap::ArgAction::SetTrue))
        .arg(
            Arg::new("field")
                .short('f')
                .long("field")
                .help("Specify fields to display (e.g., --field probe technik)")
                .num_args(1..) // Accepts one or more values
                .value_name("FIELD"),
        )
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .help("Display all available data for the spell")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches()
}
