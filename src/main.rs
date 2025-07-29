use clap::Parser;

/// CLI options for ifcx-gen
#[derive(Parser, Debug)]
#[command(name = "ifcx-gen")]
#[command(author = "Your Name <you@example.com>")]
#[command(version = "0.1.0")]
#[command(about = "Converts IFC 4.3 STEP files to IFC5 .ifcx format", long_about = None)]
struct Cli {
    /// Path to the input .ifc file
    #[arg(short, long)]
    input: String,

    /// Path to the output .ifcx file
    #[arg(short, long)]
    output: String,
}

fn main() {
    // Parse CLI arguments into `Cli` struct
    let args = Cli::parse();

    println!("Input file: {}", args.input);
    println!("Output file: {}", args.output);
}
