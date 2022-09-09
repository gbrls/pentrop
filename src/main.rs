use clap::Parser;
use shannon_entropy::ShannonEntropy;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(value_parser)]
    input: String,

    /// If specified the program will fail for inputs that do not exceed the threshold
    #[clap(short, long, value_parser)]
    threshold: Option<f32>,
}

fn main() {
    let args = Args::parse();
    let entropy = args.input.as_str().entropy();

    println!("{entropy}");

    if args.threshold.is_some() {
        let max = args.threshold.unwrap();

        if entropy > max {
            std::process::exit(exitcode::OK)
        } else {
            std::process::exit(exitcode::SOFTWARE)
        }
    } else {
        std::process::exit(exitcode::OK)
    }
}
