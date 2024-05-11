use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{self, Context};
use csv::{
    ReaderBuilder as CsvReaderBuilder,
    StringRecord as CsvStringRecord,
    WriterBuilder as CsvWriterBuilder,
};
use kiddo::float::distance::SquaredEuclidean;
use kiddo::immutable::float::kdtree::ImmutableKdTree;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use rmp_serde::{Deserializer, Serializer};


type Point = [f64; 3];


pub fn point_from_lat_lon(lat: f64, lng: f64) -> Point {
    let lat = lat.to_radians();
    let lng = lng.to_radians();

    [
        lat.cos() * lng.cos(),
        lat.cos() * lng.sin(),
        lat.sin(),
    ]
}


#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct StringSlice {
    offset: isize,
    len: usize,
}


impl StringSlice {
    pub fn new(original: &str, str_slice: &str) -> StringSlice {
        let offset = unsafe {
            str_slice.as_ptr().offset_from(original.as_ptr())
        };

        StringSlice {
            offset: offset,
            len: str_slice.len(),
        }
    }

    fn to_str(&self, original: &str) -> &str {
        let ptr = unsafe {
            original.as_ptr().offset(self.offset)
        };

        unsafe {
            std::str::from_utf8_unchecked(
                std::slice::from_raw_parts(ptr, self.len)
            )
        }
    }

    pub fn to_string(&self, original: &str) -> String {
        self.to_str(original).to_string()
    }
}


fn u8_from_char(ch: char) -> anyhow::Result<u8> {
    if ch as u32 <= u8::max_value() as u32 {
        Ok(ch as u8)
    } else {
        Err(anyhow::anyhow!("Value separator must by an ASII character (got {})", ch as u32))
    }
}


fn parse_csv_line(
    line: &str,
    value_sep: u8,
) -> anyhow::Result<CsvStringRecord> {
    let mut reader = CsvReaderBuilder::new()
        .has_headers(false)
        .delimiter(value_sep)
        .from_reader(line.as_bytes());

    let mut record = CsvStringRecord::new();

    if reader.read_record(&mut record)? {
        Ok(record)
    } else {
        Err(anyhow::anyhow!("Internal error - failed to parse CSV line"))
    }
}


fn drop_record_coordinates(
    record: &CsvStringRecord,
) -> CsvStringRecord {
    CsvStringRecord::from_iter(record.iter().skip(2))
}


#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[pyclass]
pub struct ReverseGeocoder {
    csv: String,
    value_sep: char,

    columns: Vec<String>,

    indexed_data: Vec<StringSlice>,
    tree: ImmutableKdTree<f64, u32, 3, 32>,
}


impl ReverseGeocoder {
    pub fn new(
        csv: String,
        value_sep: char,
        drop_coordinates: bool,
    ) -> anyhow::Result<ReverseGeocoder> {
        let mut line_iter = csv.lines();

        let u8_value_sep = u8_from_char(value_sep)?;

        let header_line = line_iter.next().context("missing first line in CSV")?;
        let columns = parse_csv_line(header_line, u8_value_sep)?;

        let data_csv = if drop_coordinates { get_witout_coordinates(&csv, u8_value_sep)? } else { "".to_string() };
        let mut data_csv_line_iter = data_csv.lines();

        let lines_left = line_iter.clone().count();
        let mut points: Vec<Point> = Vec::with_capacity(lines_left);
        let mut indexed_data: Vec<StringSlice> = Vec::with_capacity(lines_left);

        for (i, csv_line) in line_iter.enumerate() {
            if i == lines_left - 1 {
                if csv_line.len() == 0 || csv_line == "\n" {
                    continue;
                }
            }

            let data_line = if drop_coordinates {
                data_csv_line_iter.next().context("CSV got out of sync")?
            } else {
                csv_line
            };

            let values = parse_csv_line(csv_line, u8_value_sep)?;
            if values.len() != columns.len() {
                return Err(
                    anyhow::anyhow!(
                        "column mismatch: header (line 0) has {} values, line {} has {} values",
                        columns.len(),
                        i + 1, // Since the header line is already consumed
                        values.len(),
                    )
                );
            }

            let lat = values[0].parse::<f64>()?;
            let lon = values[1].parse::<f64>()?;
            let point = point_from_lat_lon(lat, lon);
            points.push(point);

            if drop_coordinates {
                indexed_data.push(StringSlice::new(&data_csv, data_line));
            } else {
                indexed_data.push(StringSlice::new(&csv, csv_line));
            }
        }

        Ok(
            ReverseGeocoder {
                csv: if drop_coordinates { data_csv } else { csv },
                value_sep: value_sep,
                columns: {
                    let c = if drop_coordinates { drop_record_coordinates(&columns) } else { columns };
                    Vec::from_iter(c.iter().map(|x| x.to_string()))
                },
                indexed_data: indexed_data,
                tree: ImmutableKdTree::new_from_slice(&points),
            }
        )
    }

    pub fn get_nearest(&self, lat: f64, lon: f64) -> String {
        let query = point_from_lat_lon(lat, lon);
        let search_result = self.tree.nearest_one::<SquaredEuclidean>(&query);
        let index = search_result.item as usize;
        let data = &self.indexed_data[index];

        data.to_string(&self.csv)
    }

    pub fn write_fast_format(&self, path: &Path) -> anyhow::Result<()> {
        let mut file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(path)
            ?;

        self.serialize(&mut Serializer::new(&mut file))?;

        Ok(())
    }

    pub fn read_fast_format(path: &Path) -> anyhow::Result<ReverseGeocoder> {
        let mut file = fs::File::open(path)?;
        let rg = ReverseGeocoder::deserialize(&mut Deserializer::new(&mut file))?;

        Ok(rg)
    }
}


fn get_witout_coordinates(csv: &str, value_sep: u8) -> anyhow::Result<String> {
    let mut csv_writer = CsvWriterBuilder::new()
        .buffer_capacity(csv.len())
        .has_headers(false)
        .delimiter(value_sep)
        .from_writer(vec![]);

    for line in csv.lines().skip(1) {
        let record = parse_csv_line(line, value_sep)?;
        let trimmed_record = drop_record_coordinates(&record);
        csv_writer.write_record(&trimmed_record)?;
    }

    Ok(String::from_utf8(csv_writer.into_inner()?)?)
}


#[pymethods]
impl ReverseGeocoder {
    #[new]
    #[pyo3(signature = (csv, value_sep=',', drop_coordinates=false))]
    fn python_init(
        csv: String,
        value_sep: char,
        drop_coordinates: bool,
    ) -> anyhow::Result<Self> {
        ReverseGeocoder::new(csv, value_sep, drop_coordinates)
    }

    pub fn get_nearest_data(&self, lat: f64, lon: f64) -> PyResult<String> {
        Ok(self.get_nearest(lat, lon))
    }

    pub fn get_nearest_dict(&self, lat: f64, lon: f64) -> PyResult<HashMap<String, String>> {
        let data = self.get_nearest(lat, lon);

        let mut dict = HashMap::new();
        let values = parse_csv_line(&data, u8_from_char(self.value_sep)?)?;
        for (i, value) in values.iter().enumerate() {
            dict.insert(self.columns[i].clone(), value.to_string());
        }

        Ok(dict)
    }

    pub fn save(&self, path: PathBuf) -> anyhow::Result<()> {
        self.write_fast_format(&path)
    }

    #[staticmethod]
    pub fn load(path: PathBuf) -> anyhow::Result<Self> {
        ReverseGeocoder::read_fast_format(&path)
    }
}


#[pymodule]
fn geo_trace(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ReverseGeocoder>()?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_u8_from_char_usage() -> anyhow::Result<()> {
        assert_eq!(u8_from_char(',')?, b',');
        assert_eq!(u8_from_char('\t')?, b'\t');
        assert_eq!(u8_from_char(':')?, b':');

        Ok(())
    }

    #[test]
    fn parse_basic_csv_line() -> anyhow::Result<()> {
        let csv = "a,b,c";
        let parsed = parse_csv_line(csv, b',')?;
        assert_eq!(parsed.len(), 3);
        assert_eq!(&parsed[0], "a");
        assert_eq!(&parsed[1], "b");
        assert_eq!(&parsed[2], "c");

        Ok(())
    }

    #[test]
    fn parse_csv_line_with_line_termination() -> anyhow::Result<()> {
        let csv = "a,b,c\n";
        let parsed = parse_csv_line(csv, b',')?;
        assert_eq!(parsed.len(), 3);
        assert_eq!(&parsed[0], "a");
        assert_eq!(&parsed[1], "b");
        assert_eq!(&parsed[2], "c");

        Ok(())
    }

    #[test]
    fn parse_csv_line_with_quotation() -> anyhow::Result<()> {
        let csv = "a,\"b,c\"";
        let parsed = parse_csv_line(csv, b',')?;
        assert_eq!(parsed.len(), 2);
        assert_eq!(&parsed[0], "a");
        assert_eq!(&parsed[1], "b,c");

        Ok(())
    }

    #[test]
    fn basic_drop_record_coordinates() -> anyhow::Result<()> {
        let csv = "a,\"b,c\",d,e";
        let parsed = parse_csv_line(csv, b',')?;
        let trimmed = drop_record_coordinates(&parsed);
        assert_eq!(trimmed.len(), 2);
        assert_eq!(&trimmed[0], "d");
        assert_eq!(&trimmed[1], "e");

        Ok(())
    }

    #[test]
    fn loads_a_comma_csv() {
        let csv = "lat,lon,city,country
40.7831,-73.9712,New York,US
37.7749,-122.4194,San Francisco,US";

        let geocoder = ReverseGeocoder::new(csv.to_string(), ',', false).unwrap();
        let result = geocoder.get_nearest(40.783, -73.971);
        assert_eq!(result, "40.7831,-73.9712,New York,US");
    }

    #[test]
    fn loads_a_tab_csv() {
        let csv = "lat\tlon\tcity\tcountry
40.7831\t-73.9712\tNew York\tUS
37.7749\t-122.4194\tSan Francisco\tUS";

        let geocoder = ReverseGeocoder::new(csv.to_string(), '\t', false).unwrap();
        let result = geocoder.get_nearest(40.783, -73.971);
        assert_eq!(result, "40.7831\t-73.9712\tNew York\tUS");
    }

    #[test]
    fn dropped_coordinates() {
        let csv = "lat,lon,city,country
19.4326,-99.1332,Mexico City,MX
25.7617,-80.1918,Miami,US
35.6895,139.6917,Tokyo,JP
37.7749,-122.4194,San Francisco,US
40.7831,-73.9712,New York,US
48.8566,2.3522,Paris,FR
51.5074,-0.1278,London,GB
55.7558,37.6176,Moscow,RU
";
        let geocoder = ReverseGeocoder::new(csv.to_string(), ',', true).unwrap();

        assert_eq!(
            geocoder.get_nearest(19.4326, -99.1332),
            "Mexico City,MX",
        );
        assert_eq!(
            geocoder.get_nearest(19.432, -99.1332),
            "Mexico City,MX",
        );
        assert_eq!(
            geocoder.get_nearest(19.4326, -99.133),
            "Mexico City,MX",
        );
    }

    #[test]
    fn gets_nearest() {
        let csv = "lat,lon,city,country
19.4326,-99.1332,Mexico City,MX
25.7617,-80.1918,Miami,US
35.6895,139.6917,Tokyo,JP
37.7749,-122.4194,San Francisco,US
40.7831,-73.9712,New York,US
48.8566,2.3522,Paris,FR
51.5074,-0.1278,London,GB
55.7558,37.6176,Moscow,RU
";
        let geocoder = ReverseGeocoder::new(csv.to_string(), ',', false).unwrap();

        assert_eq!(
            geocoder.get_nearest(19.4326, -99.1332),
            "19.4326,-99.1332,Mexico City,MX",
        );
        assert_eq!(
            geocoder.get_nearest(19.432, -99.1332),
            "19.4326,-99.1332,Mexico City,MX",
        );
        assert_eq!(
            geocoder.get_nearest(19.4326, -99.133),
            "19.4326,-99.1332,Mexico City,MX",
        );
        assert_eq!(
            geocoder.get_nearest(19.432, -99.133),
            "19.4326,-99.1332,Mexico City,MX",
        );
        assert_eq!(
            geocoder.get_nearest(19.43, -99.13),
            "19.4326,-99.1332,Mexico City,MX",
        );
        assert_eq!(
            geocoder.get_nearest(18., -98.),
            "19.4326,-99.1332,Mexico City,MX",
        );

        assert_eq!(
            geocoder.get_nearest(25.7617, -80.1918),
            "25.7617,-80.1918,Miami,US",
        );
        assert_eq!(
            geocoder.get_nearest(25., -80.),
            "25.7617,-80.1918,Miami,US",
        );

        assert_eq!(
            geocoder.get_nearest(35., 139.),
            "35.6895,139.6917,Tokyo,JP",
        );

        assert_eq!(
            geocoder.get_nearest(37., -122.),
            "37.7749,-122.4194,San Francisco,US",
        );

        assert_eq!(
            geocoder.get_nearest(40., -73.),
            "40.7831,-73.9712,New York,US",
        );

        assert_eq!(
            geocoder.get_nearest(48., 2.),
            "48.8566,2.3522,Paris,FR",
        );

        assert_eq!(
            geocoder.get_nearest(51., -0.),
            "51.5074,-0.1278,London,GB",
        );

        assert_eq!(
            geocoder.get_nearest(55., 37.),
            "55.7558,37.6176,Moscow,RU",
        );
    }
}
