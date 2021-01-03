use crate::top::Top;
use structopt::StructOpt;

use crate::color::*;

#[derive(StructOpt)]
#[structopt(about = "Example of a todo")]
pub enum Cmd {
    Version {},
    Task { text: String },
    List {},
}

fn run_version(_top: &Top) {
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    println!("hours v{}, (c) Vardhan Varma", blue(&VERSION.to_string()));
}

impl Cmd {
    pub fn run(&self, top: &mut Top) {
        match self {
            Self::Version {} => run_version(top),
            Self::Task { text } => top.task_new_show(text),
            Self::List {} => top.task_list(),
        }
    }
}
