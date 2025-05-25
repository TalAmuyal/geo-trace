import pathlib

from geo_trace import ReverseGeocoder


test_data_dir = pathlib.Path("test_data").resolve()
test_data_dir = pathlib.Path(__file__).resolve().parent.parent.parent / "test_data"
assert test_data_dir.exists(), str(test_data_dir) + " does not exist"


def peakmem_loading() -> None:
    ReverseGeocoder.load(test_data_dir / "full_data.msgpack")
