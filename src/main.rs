use flatgeobuf::{FallibleStreamingIterator, FgbReader};
use std::{fs::File, io::BufReader};
// use bevy::prelude::*;

fn main() {
    let mut file_buffer = BufReader::new(File::open("countries.fgb").unwrap());
    let mut flatgeobuf_reader = FgbReader::open(&mut file_buffer)
        .unwrap()
        .select_all()
        .unwrap();
    while let Some(simple_feature) = flatgeobuf_reader.next().unwrap() {
        println!("{:#?}", simple_feature.fbs_feature())
    }

    // App::new().add_plugins(DefaultPlugins).run();
}
