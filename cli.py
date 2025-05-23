#!/usr/bin/env python

import os
import pathlib

import psutil
import typer

from geo_trace import ReverseGeocoder


root_path = pathlib.Path(__file__).parent
while not (root_path / ".git").exists():
    root_path = root_path.parent

test_data_path = root_path / "test_data"
full_data_csv_path = test_data_path / "full_data.csv"
full_data_compact_path = test_data_path / "full_data.msgpack"


app = typer.Typer()


@app.command()
def measure_memory(
    path: pathlib.Path = full_data_csv_path,
) -> None:
    initial = get_memory_usage_mb()

    if path.suffix == ".msgpack":
        rg = ReverseGeocoder.load(path)
    else:
        rg = ReverseGeocoder(path.read_text())

    usage = get_memory_usage_mb() - initial
    del rg
    print(f"Memory usage: {usage:.2f} MB")


@app.command()
def compact(
    src: pathlib.Path = full_data_csv_path,
    dst: pathlib.Path = full_data_compact_path,
) -> None:
    rg = ReverseGeocoder(src.read_text())
    rg.save(dst)


def get_memory_usage_mb() -> float:
    process = psutil.Process(os.getpid())
    return process.memory_info().rss / (1024 * 1024)


if __name__ == "__main__":
    app()
