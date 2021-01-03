#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate structopt;

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use structopt::StructOpt;

mod cmd;
mod color;
mod top;

use cmd::Cmd;
use top::Top;

#[derive(StructOpt)]
#[structopt(about = "An todo list.")]
struct Opt {
    #[structopt(default_value = "~/hours.json", short, long)]
    file: String,

    #[structopt(subcommand)]
    cmd: Option<Cmd>,
}

fn main() {
    let opt = Opt::from_args();

    let path = shellexpand::full(&opt.file).unwrap().to_string();
    //println!("Using path {}", path);

    let path_actual = PathBuf::from(&path);
    let path_temp = PathBuf::from(format!("{}.tmp", &path));
    let path_backup = PathBuf::from(format!("{}.bak", &path));

    let mut top = if path_actual.exists() {
        let opened_file = File::open(&path_actual).unwrap();
        let top: Top = serde_json::from_reader(BufReader::new(opened_file)).unwrap();
        top
    } else {
        Top::new()
    };

    // default is 'info'
    let cmd = match opt.cmd {
        Some(cmd) => cmd,
        None => Cmd::Version {},
    };

    cmd.run(&mut top);

    // save file.
    serde_json::to_writer_pretty(File::create(PathBuf::from(&path_temp)).unwrap(), &top).unwrap();
    if path_actual.exists() {
        std::fs::rename(&path_actual, &path_backup).unwrap();
    }
    std::fs::rename(&path_temp, &path_actual).unwrap();
}
