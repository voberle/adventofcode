#!/usr/bin/python3

import os
import shutil
import subprocess
from pathlib import Path
import json

year = os.path.basename(os.getcwd())

# Days that are too slow for now.
skip = {
    "2015": [],
    "2016": [5],
    "2017": [],
    "2019": [],
    "2020": [],
    "2021": [19, 24],
    "2022": [],
    "2023": [],
}

bench_dir = Path("bench")
bench_dir.mkdir(exist_ok=True)
for file in bench_dir.glob("*"):
    file.unlink()

subprocess.run(["cargo", "build", "--release"], check=True)

# Initialize the results file with the desired header
results_file = Path("benchmarks.md")
results_file.write_text("| Command | Mean [ms] |\n|:---|---:|\n")

for day in range(1, 26):
    if day in skip[year]:
        print(f"Skipping {year} {day}")
        results_file.write_text(results_file.read_text() + f"| `{year} Day {day}` | Skipped |\n")
        continue

    dir_name = f"day{day:02d}"
    hyperfine_command = [
        "hyperfine",
        "--warmup", "2",
        f"--export-json=bench/{day}.json",
        f"--command-name={year} Day {day}",
        "--time-unit", "millisecond",
        "--input", f"{dir_name}/resources/input",
        f"target/release/{dir_name}"
    ]
    try:
        subprocess.run(hyperfine_command, check=True, timeout=10)
    except subprocess.TimeoutExpired:
        print(f"Timeout {year} {day}")
        results_file.write_text(results_file.read_text() + f"| `{year} Day {day}` | Timed out |\n")
        continue

    # Read the mean time from the JSON file and convert it to milliseconds
    with open(f"bench/{day}.json", "r") as json_file:
        data = json.load(json_file)
        mean_time = data['results'][0]['mean'] * 1000  # Convert to milliseconds
        results_file.write_text(results_file.read_text() + f"| `{year} Day {day}` | {mean_time:.2f} ms |\n")

for file in bench_dir.glob("*"):
    file.unlink()
bench_dir.rmdir()
