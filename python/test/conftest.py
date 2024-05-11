import pytest


@pytest.fixture(scope="session")
def ReverseGeocoder():
    import geo_trace

    return geo_trace.ReverseGeocoder


@pytest.fixture()
def tmp_save_path(tmp_path):
    yield tmp_path / "compact.msgpack"
