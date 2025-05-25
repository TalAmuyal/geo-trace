import functools
import json
import pathlib

from geo_trace import ReverseGeocoder


test_data_dir = pathlib.Path("test_data").resolve()
test_data_dir = pathlib.Path(__file__).resolve().parent.parent.parent / "test_data"
assert test_data_dir.exists(), str(test_data_dir) + " does not exist"


@functools.cache
def get_rg() -> ReverseGeocoder:
    return ReverseGeocoder.load(test_data_dir / "full_data.msgpack")


@functools.cache
def get_coordinates() -> list[list[float, float]]:
    with open(test_data_dir / "selected_coordinates.json", "r") as f:
        return json.load(f)


def setup() -> None:
    # Load ahead of time
    get_rg()
    get_coordinates()


def time_lookup_as_string() -> None:
    rg = get_rg()
    for lat, lon in get_coordinates():
        rg.get_nearest_as_string(lat, lon)


def time_lookup_as_dict() -> None:
    rg = get_rg()
    for lat, lon in get_coordinates():
        rg.get_nearest_as_dict(lat, lon)
