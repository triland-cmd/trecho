use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input text
    #[arg(value_name = "TEXT", required = true)]
    text: Vec<String>,

    /// Do not print newline
    #[arg(short = 'n', default_value_t = false)]
    omit_newline: bool,
}

fn main() {
    let args = Args::parse();
    let ending = if args.omit_newline { "" } else { "\n" };
    let output = format!("{}{}", args.text.join(" "), ending);

    print!("{}", output);
}
