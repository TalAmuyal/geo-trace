/*
use std::fs;

use geo_trace::InnerReverseGeocoder;

#[test]
fn benchmark_loading_full_data_file() {
    let csv_path = "test_data/full_data.csv";
    let csv: String = fs::read_to_string(csv_path).unwrap();
    let geocoder = InnerReverseGeocoder::new(csv, ',');

    let temp_path = "/tmp/kaki.msgpack";
    geocoder.write_fast_format(temp_path);
    let geocoder = InnerReverseGeocoder::read_fast_format(temp_path);
    assert_eq!(
        geocoder.get_nearest(55., 37.),
        "55.7558,37.6176,Moscow,RU",
    );
}

#[test]
fn benchmark_loading_fast_data_file() {
    let temp_path = "/tmp/kaki.msgpack";
    let geocoder = InnerReverseGeocoder::read_fast_format(temp_path);
    assert_eq!(
        geocoder.get_nearest(55.75, 37.61),
        "55.75222,37.61556,Moscow,Moscow,,RU",
    );
}
*/
