import os
import pathlib

import pytest


SKIP_SLOW = os.getenv("SKIP_SLOW", "0") == "1"


@pytest.fixture(scope="session")
def rg(ReverseGeocoder):
    curr_dir = pathlib.Path(__file__).parent
    root_path = curr_dir.parent.parent
    path = root_path / "test_data" / "full_data.csv"
    return ReverseGeocoder(path.read_text())


@pytest.mark.skipif(SKIP_SLOW, reason="Skipping slow tests")
def test_load_comma_csv(rg):
    assert rg.get_nearest_data(37.7749, -122.4194) == "37.77493,-122.41942,San Francisco,California,San Francisco County,US"


@pytest.mark.skipif(SKIP_SLOW, reason="Skipping slow tests")
def test_lookup(rg):
    assert rg.get_nearest_data(37.7749, -122.4194) == "37.77493,-122.41942,San Francisco,California,San Francisco County,US"
    assert rg.get_nearest_data(37.774, -122.419) == "37.77493,-122.41942,San Francisco,California,San Francisco County,US"
    assert rg.get_nearest_data(37.77, -122.41) == "37.77493,-122.41942,San Francisco,California,San Francisco County,US"


@pytest.mark.skipif(SKIP_SLOW, reason="Skipping slow tests")
def test_lookup_from_loaded_file(
    ReverseGeocoder,
    rg,
    tmp_save_path,
):
    rg.save(tmp_save_path)
    loaded_rg = ReverseGeocoder.load(tmp_save_path)

    test_cases = [
        (
            (37.7749, -122.4194),
            "37.77493,-122.41942,San Francisco,California,San Francisco County,US",
        ),
        (
            (37.774, -122.419),
            "37.77493,-122.41942,San Francisco,California,San Francisco County,US",
        ),
        (
            (37.77, -122.41),
            "37.77493,-122.41942,San Francisco,California,San Francisco County,US",
        ),
    ]
    for (lat, lon), expected in test_cases:
        assert loaded_rg.get_nearest_data(lat, lon) == expected
