import pathlib


class ReverseGeocoder:
    """
    A class that can perform a reverse-geocoding.

    :param csv: a string containing the CSV of lat, long, and whatever data you want to store
    :param value_sep: a single character that is used to separate the values in the CSV (usually a comma)
    """

    def __init__(
        self,
        csv: str,
        value_sep: str,
    ) -> None: ...

    def get_nearest_data(self, lat: float, lon: float) -> str:
        """
        Gets the data that is nearest to the given lat, lon

        :return: a string containing the data
        """

    def get_nearest_dict(self, lat: float, lon: float) -> dict[str, str]:
        """
        Gets the data that is nearest to the given lat, lon

        :return: a dictionary containing the data
        """

    def save(self, path: pathlib.Path) -> None:
        """
        Persists the whole data structure to a file that is faster to load
        than the CSV

        :param path: the path to save the data to
        """

    @classmethod
    def load(cls, path: pathlib.Path) -> "ReverseGeocoder":
        """
        Restores the data structure from a file

        :param path: the path to load the data from
        :return: the restored ReverseGeocoder object
        """
