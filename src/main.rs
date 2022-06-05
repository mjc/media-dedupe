mod cli;
mod locate;

fn main() {
    let cli = cli::parse();
    println!("Dry run: {:?}, path: {:?}", cli.dry_run(), cli.path());
}
