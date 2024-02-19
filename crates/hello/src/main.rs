use glama::prelude::*;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Name to operate on
    name: String,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

fn main() {
    let cli = Cli::parse();

    // Print the name (a required argument)
    println!("Hello, {}!", cli.name);

    // Print the path to an optionally specified configuration file
    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }
}
