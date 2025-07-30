use clap::Parser;

/// CLI options for ifcx-gen
/// these below provide metadata and attributes to the CLI application 
/// It will allow me to call from console: $ ifcx-gen --help

#[command(name = "ifcx-gen")]
#[command(author = "Your Name <you@example.com>")]
#[command(version = "0.1.0")]
#[command(about = "Converts IFC 4.3 STEP files to IFC5 .ifcx format", long_about = None)]

/// using the `clap` crate
/// which is used for parsing command line arguments.
/// The `Parser` derive macro automatically generates the necessary code
/// to parse command line arguments into a structured format.
/// The `command` attributes provide metadata such as the name, author, version,
/// and description of the CLI application.
// #[derive(Parser)] tells clap to implement CLI parsing for this struct.
//Debug allows printing the struct with {:?} for debugging.
#[derive(Parser, Debug)]
struct Cli {
    /// Path to the input .ifc file
    #[arg(short, long)]
    input: String,

    /// Path to the output .ifcx file
    #[arg(short, long)]
    output: String,

/*This declares the command-line arguments as fields on a struct.

#[arg(short, long)]:

Enables short flags like -i and long flags like --input

So both of these work: */
    // ifcx-gen -i input.ifc -o output.ifcx
    // ifcx-gen --input input.ifc --output output.ifcx
}

fn main() {
    // Parse CLI arguments into `Cli` struct
    // This parses the actual CLI arguments into a Cli instance.
    let args = Cli::parse();

    println!("Input file: {}", args.input);
    println!("Output file: {}", args.output);
}

/*Test it with:
cargo run -- --input ./models/input.ifc --output ./out/model.ifcx
the -- before the input and output is necessary to separate cargo's arguments from the program's arguments.
*/