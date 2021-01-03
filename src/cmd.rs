use crate::top::Top;
use ansi_term::Colour::*;
use ansi_term::Style;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "Example of a todo")]
pub enum Cmd {
    Version {},
}

fn blue(s: &dyn ToString) -> String {
    Style::new().fg(Blue).paint(s.to_string()).to_string()
}

fn run_version(_top: &Top) {
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    println!("hours v{}, (c) Vardhan Varma", blue(&VERSION.to_string()));
}

impl Cmd {
    pub fn run(&self, top: &mut Top) {
        match self {
            Self::Version {} => run_version(top),
        }
    }
}
