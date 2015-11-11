extern crate clap;

use clap::{App, Arg};

fn main() {
    let matches = App::new("unidecode")
        .version("1.0")
        .author("Lucille Blumire")
        .about("Returns Unicode Values")
        .arg(Arg::with_name("DECIMAL")
             .short("d")
             .long("decimal")
             .help("Output a Decimal"))
        .arg(Arg::with_name("CHARS")
             .help("The Characters to decode")
             .required(true)
             .index(1))
        .get_matches();


    let chars = matches.value_of("CHARS").unwrap(); // Error handling by required parameter in clap

    if matches.is_present("DECIMAL") {
        for c in chars.chars() {
            println!("{}: {}", c, c as u32);
        }
    } else {
        for c in chars.chars() {
            println!("{}: {:x}", c, c as u32);
        }
    }
}
