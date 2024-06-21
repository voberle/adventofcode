#!/usr/bin/python3

import os
import subprocess
from pathlib import Path
import json

# Function to benchmark a single year
def benchmark_year(year_path):
    year = year_path.name

    bench_dir = year_path / "bench"
    bench_dir.mkdir(exist_ok=True)
    for file in bench_dir.glob("*"):
        file.unlink()

    subprocess.run(["cargo", "build", "--release"], check=True, cwd=year_path)

    year_results = f"## {year}\n| Day | Mean [ms] |\n|:---|---:|\n"

    for day in range(1, 26):
        dir_name = f"day{day:02d}"
        hyperfine_command = [
            "hyperfine",
            "--warmup", "2",
            f"--export-json={bench_dir}/{day}.json",
            f"--command-name={year} Day {day}",
            "--time-unit", "millisecond",
            "--input", f"{year_path}/{dir_name}/resources/input",
            f"{year_path}/target/release/{dir_name}"
        ]
        try:
            subprocess.run(hyperfine_command, check=True, timeout=10, cwd=year_path)
        except subprocess.TimeoutExpired:
            print(f"Timeout {year} {day}")
            year_results += f"| `{year} Day {day}` | Timed out |\n"
            continue
        except subprocess.CalledProcessError as e:
            print(f"Error {year} {day}: {e}")
            year_results += f"| `{year} Day {day}` | Error |\n"
            continue

        # Read the mean time from the JSON file and convert it to milliseconds
        with open(bench_dir / f"{day}.json", "r") as json_file:
            data = json.load(json_file)
            mean_time = data['results'][0]['mean'] * 1000  # Convert to milliseconds
            year_results += f"| `{year} Day {day}` | {mean_time:.2f} |\n"

    for file in bench_dir.glob("*"):
        file.unlink()
    bench_dir.rmdir()

    return year_results

# Check if the script is run from a year directory or a parent directory
current_path = Path.cwd()
years = [current_path] if current_path.name.isdigit() else [d for d in current_path.iterdir() if d.is_dir() and d.name.isdigit()]

results_file = Path("benchmarks.md")
results_content = ""

for year_path in years:
    results_content += benchmark_year(year_path)

results_file.write_text(results_content)
