use camino::Utf8PathBuf;
use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(short, long)]
    dry_run: bool,
    path: Option<Utf8PathBuf>,
}

impl Cli {
    pub fn dry_run(&self) -> bool {
        self.dry_run
    }

    pub fn path(&self) -> Option<&Utf8PathBuf> {
        self.path.as_ref()
    }
}

pub fn parse() -> Cli {
    Cli::parse()
}
