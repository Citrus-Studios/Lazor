use clap::Parser;

/// Args to the program
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long)]
    file: String,
}

fn main() {
    let args = Args::parse();
}
