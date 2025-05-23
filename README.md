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
import pathlib

from geo_trace import ReverseGeocoder


# The constructor loads the CSV into memory and optimizes it for fast lookups
rg = ReverseGeocoder(
    csv="path/to/geo-trace.csv",
    value_sep=",",
)

# Get the row from the CSV as a string
str_row: str = rg.get_nearest_as_string(37.7749, -122.4194)
print(str_row)

# Get the row from the CSV as a dictionary
dict_row: dict = rg.get_nearest_as_dict(37.7749, -122.4194)
print(dict_row)

# Loading the CSV is relatively slow, so it's better to save the optimized result:
path = pathlib.Path("path/to/geo-trace-compact.msgpack")
rg.save()
geocoder_2 = ReverseGeocoder.load(path)  # Much faster than the original constructor
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


# TODOs

- In the README:
  - Explain what made the implementation have low-latency and low-memory usage and its trade-offs
- Add a CI/CD pipeline
  - Build and test for Python 3.8-3.13
  - Build and test for Windows, Linux, and MacOS
  - Build and test for x86, ARM, and PowerPC
  - Publish to PyPI
- Add API for:
  - Data optimization (like dropping columns)
  - Multi-lookup
  - Lightweight copy (put the CSV under an Arc + verify before and after)
  - Compress the table by moving cell values to an array and replace with an index


# License

This project is licensed under the MIT license (https://choosealicense.com/licenses/mit/).
See the `LICENSE.txt` file for more information.


# Attribution

All geo-location data was obtained from the Geo Names database (https://www.geonames.org/).
The Geo Names database is licensed under a Creative Commons Attribution 4.0 License (https://creativecommons.org/licenses/by/4.0/) at the time of writing.
