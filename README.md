Geo Trace is a Rust implementation of a Reverse Geocoder that aims to be:

1. Offline
2. Fast (optimizing for a single-lookup at a time)
3. Memory efficient
4. Customizable

And in that order.

Non-goals:

- Real-time updates.
- Fast initialization.


# Installation

```bash
pip install geo-trace
```

# Usage

```python
from geo_trace import ReverseGeocoder

rg = ReverseGeocoder(csv_data)

str_row: str = rg.get_nearest_as_string(37.7749, -122.4194)
print(str_row)

dict_row: dict = rg.get_nearest_as_dict(37.7749, -122.4194)
print(dict_row)
```

(see complete example below)


## Getting the data

TL;DR: You need a CSV file with latitude and longitude data, which can be downloaded from the [Geo Names](https://www.geonames.org/) database.

Geo Trace needs to be provided with the date, which is then parsed and optimized for fast lookups and low memory footprint.
The data should be a CSV with:
1. A header row (the fist line is the name of the columns)
2. The first column for the latitude and the second column for the longitude
3. The rest of the columns for any data you want to associate with that location

The column names in the header row can be anything and will be used as keys in the dictionary returned by `get_nearest_as_dict`.

Such a CSV can be downloaded from the [Geo Names] (https://www.geonames.org/) database.

## Recommendations

1. Drop any unnecessary columns from the CSV
    - This can dramatically reduce memory usage
    - See the complete example below
2. After the data is loaded for the first time, save the optimized object to a compact format for future use using the `save` method
    - This will create a `.msgpack` file that is optimized for fast loading
    - See the complete example below
3. If the associated coordinates are not needed in the lookup result, drop the coordinates using the `drop_coordinates` parameter
    - This will reduce the memory footprint
    - See the complete example below


## Complete example

```python
import io
import time

import pandas

from geo_trace import ReverseGeocoder


# Load only the necessary columns from the CSV into a DataFrame
csv_buffer = io.StringIO()
pandas.read_csv(
    "test_data/full_data.csv",
    usecols=["lat", "lon", "name", "cc"],
).to_csv(
    csv_buffer,
    index=False,
    header=True,
)

# The constructor parser the data and optimizes it for fast lookup and small memory footprint
print("Optimizing will take a while... (depending on the size of the data)")
start_time = time.perf_counter()
rg = ReverseGeocoder(
    csv=csv_buffer.getvalue(),  # [Required] The CSV data as a string
    value_sep=",",  #             [Optional] Defaults to `,`
    drop_coordinates=True,  #     [Optional] Defaults to `False` - Useful if the coordinates are not needed as part of the lookup result
)
print(f"Took {time.perf_counter() - start_time:.2f} seconds")
# Took 116.32 seconds (on a modest desktop PC with an SSD)

# Save the optimized data to a compact format that loads MUCH faster
rg.save("test_data/relevant_data.msgpack")
start_time = time.perf_counter()
rg = ReverseGeocoder.load("test_data/relevant_data.msgpack")
print(f"Took {time.perf_counter() - start_time:.2f} seconds")
# Took 1.34 seconds (on the same machine as above)

# Get the nearest location as a dictionary
nearest_location = rg.get_nearest_as_dict(37.7749, -122.4194)
print(nearest_location)
```


# Development

There is a `shell.nix` file that can be used to create a Nix shell with the required dependencies. It also has aliases for ease of use.
The aliases are usually the gist of the command with a `j` prefix.
For example, `jinstall` is an alias for `make install`, `jtest` is an alias for `make test`, `jmeasure-memory` is an alias for `.venv/bin/python ./cli.py measure-memory`, and so on.

## Prepare the environment

This project is built using Python and Rust.
You can run it in a Nix shell (`nix-shell`) or install Python and Rust on your system.

Then create a virtual environment and install the dependencies:

(not needed if you are using Nix)

```bash
python -m venv .venv
source .venv/bin/activate
make install
```

Note, if you choose to activate the virtual environment, you will not need to use the `.venv/bin/python` prefix for the commands below.

## Run the tests

```bash
make test
```

Or just `jtest` if you are in the Nix shell.

## Measure memory usage

```bash
time .venv/bin/python ./cli.py measure-memory --path test_data/full_data.csv
time .venv/bin/python ./cli.py compact --src test_data/full_data.csv --dst test_data/full_data.msgpack
time .venv/bin/python ./cli.py measure-memory --path test_data/full_data.msgpack
```

Or the `j` variant if you are in the Nix shell.

Running the above on a modest (and otherwise idle) desktop PC with an SSD yielded the following results:

```bash
# time .venv/bin/python ./cli.py measure-memory --path test_data/full_data.csv
Memory usage: 25.70 MB
real    1m37.054s
user    1m36.926s
sys     0m0.056s

# time .venv/bin/python ./cli.py compact --src test_data/full_data.csv --dst test_data/full_data.msgpack
real    1m39.136s
user    1m38.228s
sys     0m0.853s

# time .venv/bin/python ./cli.py measure-memory --path test_data/full_data.msgpack
Memory usage: 14.15 MB
real    0m1.497s
user    0m0.796s
sys     0m0.700s
```

The memory usage needed to load the compact version was half as much and it took about 1.54% of the time.


# License

This project is licensed under the MIT license (https://choosealicense.com/licenses/mit/).
See the `LICENSE.txt` file for more information.


# Attribution

All geo-location data was obtained from the Geo Names database (https://www.geonames.org/).
The Geo Names database is licensed under a Creative Commons Attribution 4.0 License (https://creativecommons.org/licenses/by/4.0/) at the time of writing.
