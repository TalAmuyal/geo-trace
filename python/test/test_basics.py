import pytest


def test_import(ReverseGeocoder):
    assert ReverseGeocoder is not None


def test_loading_with_default_value_separator(ReverseGeocoder):
    data = "37.77493,-122.41942,San Francisco"
    csv = (
        "A,B,C\n"
        f"{data}\n"
    )
    rg = ReverseGeocoder(csv)
    assert rg.get_nearest_data(37.774, -122.419) == data


@pytest.mark.parametrize("value_separator", [",", ";"])
def test_loading_with_value_separator(
    ReverseGeocoder,
    value_separator,
):
    data = f"37.77493{value_separator}-122.41942{value_separator}San Francisco"
    csv = (
        f"A{value_separator}B{value_separator}C\n"
        f"{data}\n"
    )
    rg = ReverseGeocoder(csv, value_separator)
    assert rg.get_nearest_data(37.774, -122.419) == data


def test_loading_and_lookup_with_escaped_comma(
    ReverseGeocoder,
):
    csv = (
        "lat,lon,names\n"
        "37.77493,-122.41942,\"David, Jonatan, and Joseph\"\n"
    )
    rg = ReverseGeocoder(csv)

    expected = {
        "lat": "37.77493",
        "lon": "-122.41942",
        "names": "David, Jonatan, and Joseph",
    }

    actual = rg.get_nearest_dict(37.774, -122.419)

    assert actual == expected


def test_loading_and_lookup_with_dropped_coordinates(
    ReverseGeocoder,
):
    csv = (
        "lat,lon,city\n"
        "37.77493,-122.41942,San Francisco\n"
    )
    rg = ReverseGeocoder(csv, drop_coordinates=True)

    expected_data = "San Francisco"
    actual_data = rg.get_nearest_data(37.774, -122.419)
    assert actual_data == expected_data

    expected_dict = {"city": "San Francisco"}
    actual_dict = rg.get_nearest_dict(37.774, -122.419)
    assert actual_dict == expected_dict
