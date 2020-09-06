use std::fs::File;
use std::io::{Seek, SeekFrom};
use std::path::{Path, PathBuf};

use clap::{app_from_crate, Arg};

use crate::ident::Identifier;
use std::borrow::Cow;

mod ident;
mod types;

mod vanilla;
mod model;
mod writer;

fn main() {
    let matches = app_from_crate!()
        .arg(Arg::with_name("include").short('I').long("include").value_name("PATH").multiple_occurrences(true))
        .arg(Arg::with_name("output").short('o').long("output").value_name("PATH"))
        .arg(Arg::with_name("identifiers").short('i').long("identifiers"))
        .arg(Arg::with_name("type").short('t').long("type").default_value("auto").possible_values(&["auto", "blockstate", "model"]))
        .arg(Arg::with_name("debug").short('d').long("debug").multiple_occurrences(true))
        .arg(Arg::with_name("file").required(true))
        .get_matches();

    let include = matches.values_of_os("include").map_or_else(Vec::new, |iter| iter.map(Path::new).collect());
    let output = matches.value_of_os("output").map(Path::new);
    let identifiers = matches.is_present("identifiers");
    let typ = matches.value_of("type").unwrap();
    let debug = matches.occurrences_of("debug");
    let file = Path::new(matches.value_of_os("file").unwrap());

    let output = output.map(Cow::Borrowed).unwrap_or_else(|| file.file_name().map_or_else(|| "a.bin".into(), |s| Path::new(s).with_extension("bin")).into());

    let mut file = File::open(file).expect("Failed to open input file");
    let mut model = match typ {
        "auto" => {
            let model: vanilla::model::Model = serde_json::from_reader(&mut file).unwrap();

            // TODO parse blockstate
            // file.seek(SeekFrom::Start(0)).unwrap();

            model
        }
        "blockstate" => {
            unimplemented!()
        }
        "model" => {
            let model: vanilla::model::Model = serde_json::from_reader(file).expect("Failed to parse model");
            model
        }
        _ => unreachable!()
    };

    while let Some(parent_id) = &model.parent {
        let model_path = identifier_to_model_path(parent_id);
        let mut parent = None;
        for &root in include.iter() {
            let model_path = root.join(&model_path);
            if let Ok(file) = File::open(model_path) {
                parent = Some(serde_json::from_reader(file).expect("Failed to parse parent model"));
                break;
            }
        }
        match parent {
            None => panic!("Could not find referenced parent model {}", parent_id),
            Some(m) => {
                model = model.merge(m);
            }
        }
    }

    let model = model::Model::from_json_model(&model).unwrap();

    writer::write(&model, &mut File::create(output).unwrap()).unwrap();
}

fn identifier_to_model_path(id: &Identifier) -> PathBuf {
    let mut string = id.path.to_string();
    string.push_str(".json");
    Path::new(&id.namespace).join("models").join(string)
}