import pytest


def test_loading_empty_file(ReverseGeocoder):
    with pytest.raises(Exception, match="missing first line in CSV"):
        ReverseGeocoder("", ",")


def test_loading_file_with_missing_columns(ReverseGeocoder):
    csv = (
        "A,B,C\n"
        "37.77493,-122.41942,San Francisco,bla\n"
    )
    with pytest.raises(Exception, match="column mismatch"):
        ReverseGeocoder(csv)


def test_loading_file_with_invalid_latitude(ReverseGeocoder):
    csv = (
        "A,B,C\n"
        "a,b,San Francisco\n"
    )
    with pytest.raises(Exception, match="invalid float literal"):
        ReverseGeocoder(csv, ",")


@pytest.mark.parametrize(
    "value_separator",
    [
        "",  # Too shot
        "ab",  # Too long
    ],
)
def test_loading_with_invalid_value_seperator(
    ReverseGeocoder,
    value_separator,
):
    csv = (
        f"A{value_separator}B{value_separator}C\n"
        f"37.77493{value_separator}-122.41942{value_separator}San Francisco\n"
    )
    with pytest.raises(ValueError):
        ReverseGeocoder(csv, value_separator)
