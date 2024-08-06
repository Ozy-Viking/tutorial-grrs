use anyhow::{Context, Result};
use clap::Parser;
use grrs::find_matches;
use std::{thread, time::Duration};

#[derive(Debug, Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let path = args.path;
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("could not read file `{}`", &path.display()))?;
    find_matches(&content, &args.pattern, std::io::stdout());
    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
    })
    .expect("Error setting Ctrl-C handler");

    // Following code does the actual work, and can be interrupted by pressing
    // Ctrl-C. As an example: Let's wait a few seconds.
    // thread::sleep(Duration::from_secs(2));

    let cfg = confy::load("grrs", None)?;
    println!("{:#?}", cfg);
    Ok(())
}
