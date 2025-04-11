#!/bin/bash

# Downloads and builds the variants of chrono_tz needed for this puzzle.

# Location of all the files we download and build.
DIR=/tmp

counter=0
for version in 2018c 2018g 2021b 2023d; do
    ((counter++))
    echo "Setting up $version ($counter)"

    cd $DIR

    rm -fr chrono-tz-${version}
    git clone https://github.com/chronotope/chrono-tz.git
    mv chrono-tz chrono-tz-${version}

    cd chrono-tz-${version}

    cd chrono-tz
    cd tz
    wget https://www.iana.org/time-zones/repository/releases/tzdata${version}.tar.gz
    tar xzvf tzdata${version}.tar.gz
    cd ..
    cd ..

    # The version numbers must be changed to work around a Cargo limitation.
    # https://github.com/rust-lang/cargo/issues/10353

    DELIM='#'
    OLD_PATTERN='^version = "0.10.3"$'
    NEW="version = \"0.10.30${counter}\""
    sed -i '' "s${DELIM}${OLD_PATTERN}${DELIM}${NEW}${DELIM}" chrono-tz/Cargo.toml

    OLD_PATTERN='^version = "0.4.1"$'
    NEW="version = \"0.4.10${counter}\""
    sed -i '' "s${DELIM}${OLD_PATTERN}${DELIM}${NEW}${DELIM}" chrono-tz-build/Cargo.toml

    # Build
    cargo clean
    cargo b --release

done
