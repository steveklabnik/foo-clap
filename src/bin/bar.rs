
extern crate foo;

#[macro_use]
extern crate clap;

use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
}
