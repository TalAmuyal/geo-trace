#[macro_use]
extern crate criterion;
extern crate geo_trace;

use std::path::PathBuf;
use std::{fs::File, io::Read, path::Path};
use criterion::Criterion;
use serde::Deserialize;
use geo_trace::ReverseGeocoder;

#[derive(Deserialize)]
struct Coords(Vec<Coordinate>);
type Coordinate = [f64; 2];

fn load_rg() -> ReverseGeocoder {
    let path: PathBuf = ["test_data", "full_data.msgpack"].iter().collect();

    ReverseGeocoder::read_fast_format(&path).unwrap()
}

fn load_test_coordinates() -> Vec<Coordinate> {
    let path = Path::new("test_data/selected_coordinates.json");
    let mut file = File::open(path).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let loaded: Coords = serde_json::from_str(&contents).unwrap();
    loaded.0
}

/*
use std::fs;

fn load_rg_from_csv() -> ReverseGeocoder {
    let csv_path = "test_data/full_data.csv";
    let csv: String = fs::read_to_string(csv_path).unwrap();

    ReverseGeocoder::load(csv, ',', false)
}
*/

fn criterion_benchmark(c: &mut Criterion) {
    let rg = load_rg();

    c.bench_function(
        "rg.get_nearest(45.0, 54.0)",
        |b| b.iter(
            || rg.get_nearest(45.0, 54.0)
        ),
    );

    let coords = load_test_coordinates();
    let coords_len = coords.len();
    let title2 = format!("rg.get_nearest for {coords_len} random coordinates");
    c.bench_function(
        &title2,
        |b| b.iter(
            || for [lat, lon] in &coords {
                rg.get_nearest(*lat, *lon);
            }
        ),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
