use clap::{
    builder::{PossibleValuesParser, TypedValueParser},
    Parser,
};
use tracing::level_filters::LevelFilter;

fn main() {
    // parse CLI
    let cfg = Cli::parse();

    // init tracing
    tracing_subscriber::fmt::fmt()
        .with_max_level(cfg.verbose)
        .with_target(false)
        .with_writer(std::io::stderr)
        .init();

    // do the thing
    let val = aria_of_borrow::lament_of_dependence();

    // output the result
    if cfg.cool {
        eprintln!("wow!!! {val} 🛹");
    } else {
        eprintln!("{val}");
    }
}

#[derive(Parser)]
struct Cli {
    /// How verbose logging should be (log level)
    #[clap(long, short)]
    #[clap(default_value_t = LevelFilter::WARN)]
    #[clap(value_parser = PossibleValuesParser::new(["off", "error", "warn", "info", "debug", "trace"]).map(|s| s.parse::<LevelFilter>().expect("possible values are valid")))]
    #[clap(help_heading = "GLOBAL OPTIONS", global = true)]
    pub verbose: LevelFilter,

    #[clap(long)]
    cool: bool,
}