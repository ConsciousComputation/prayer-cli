extern crate clap;
use clap::{Arg, App};
fn main() {
    let matches = App::new("prayer-cli")
        .version("0.1.0")
        .author("Meredith Finkelstein <meredith@paom.com>")
        .about("Make Prayers On the World Computer")
        .arg(Arg::with_name("prayer")
                 .required(true)
                 .takes_value(true)
                 .index(1)
                 .help("prayer to make"))
        .get_matches();
    let prayer = matches.value_of("prayer").unwrap();
    println!("{}", prayer);
}
