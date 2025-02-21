Geo Trace is a Rust Reverse Geocoder that aims to be:

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
geocoder = ReverseGeocoder(
    csv="path/to/geo-trace.csv",
    value_sep=",",
)

str_row: str = geocoder.get_nearest_data(37.7749, -122.4194)  # Returns the CSV row as a string
print(str_row)

dict_row: dict = geocoder.get_nearest_dict(37.7749, -122.4194)  # Returns the CSV row as a dictionary
print(dict_row)

# Loading the CSV is relatively slow, so it's better to save the optimized result:
path = pathlib.Path("path/to/geo-trace-compact.msgpack")
geocoder.save()
geocoder_2 = ReverseGeocoder.load(path)
```


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


# License

This project is licensed under the MIT license (https://choosealicense.com/licenses/mit/).
See the `LICENSE.txt` file for more information.


# Attribution

All geo-location data was obtained from the Geo Names database (https://www.geonames.org/).
The Geo Names database is licensed under a Creative Commons Attribution 4.0 License (https://creativecommons.org/licenses/by/4.0/) at the time of writing.
