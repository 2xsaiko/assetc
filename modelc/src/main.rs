use std::fs::File;
use std::path::Path;

use clap::{app_from_crate, Arg};

use vanilla::BlockStateDef;

mod ident;
mod types;

mod vanilla;
mod model;
mod writer;

fn main() {
    // let matches = app_from_crate!()
    //     .arg(Arg::with_name("include").short('I').long("include").value_name("PATH").multiple_occurrences(true))
    //     .arg(Arg::with_name("output").short('o').long("output").value_name("PATH"))
    //     .arg(Arg::with_name("identifiers").short('i').long("identifiers"))
    //     .arg(Arg::with_name("type").short('t').long("type").default_value("auto").possible_values(&["auto", "blockstate", "model"]))
    //     .arg(Arg::with_name("debug").short('d').long("debug").multiple_occurrences(true))
    //     .arg(Arg::with_name("file").required(true))
    //     .get_matches();
    //
    // let include = matches.values_of_os("include").map_or_else(Vec::new, |iter| iter.map(Path::new).collect());
    // let output = matches.value_of_os("output").map(Path::new);
    // let identifiers = matches.is_present("identifiers");
    // let typ = matches.value_of("type").unwrap();
    // let debug = matches.occurrences_of("debug");
    // let file = Path::new(matches.value_of_os("file").unwrap());

    let model: vanilla::model::Model = serde_json::from_reader(File::open("modelc/testres/assets/rswires/models/item/and_gate.json").unwrap()).unwrap();
    let model = model::Model::from_json_model(&model);

    println!("{:?}", model);


    writer::write(&mut File::create("test.bin").unwrap()).unwrap();
}
