#!/usr/bin/python3

import os
import shutil
import subprocess
from pathlib import Path

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

for day in range(1, 26):
    if day in skip[year]:
        print(f"Skipping {year} {day}")
        f = Path(f"bench/{day}.md")
        f.write_text(f"| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |\n|:---|---:|---:|---:|---:|\n| `{year} Day {day}` | Skipped | | | |\n")
        continue

    dir_name = f"day{day:02d}"
    hyperfine_command = [
        "hyperfine",
        "--warmup", "2",
        f"--export-markdown=bench/{day}.md",
        f"--command-name={year} Day {day}",
        "--time-unit", "millisecond",
        "--input", f"{dir_name}/resources/input",
        f"target/release/{dir_name}"
    ]
    try:
        subprocess.run(hyperfine_command, check=True, timeout=10)
    except subprocess.TimeoutExpired:
        print(f"Timeouted {year} {day}")
        f = Path(f"bench/{day}.md")
        f.write_text(f"| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |\n|:---|---:|---:|---:|---:|\n| `{year} Day {day}` | Skipped | | | |\n")
        continue

results_file = Path("benchmarks.md")
results_file.write_text("| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |\n|:---|---:|---:|---:|---:|\n")

for day in range(1, 26):
    day_md_file = bench_dir / f"{day}.md"
    if day_md_file.exists():
        with open(day_md_file, "r") as day_file:
            lines = day_file.readlines()[2:]  # Skip the first two lines
            results_file.write_text(results_file.read_text() + ''.join(lines))

for file in bench_dir.glob("*"):
    file.unlink()
bench_dir.rmdir()
