
_default:
    @just --list --unsorted --justfile {{justfile()}}

year := `date +%Y`
day := `nu -c 'date now | format date "%_d" | str trim'`

# Run a specific part of today's problem
p P in="":
    cargo run --release -- solve {{year}}:{{day}}:{{P}} {{in}}

# Run a specific day and part of this year
dp DP in="":
    cargo run --release -- solve {{year}}:{{DP}} {{in}}

# Run a specific year's day's part
dyp DYP in="":
    cargo run --release -- solve {{DYP}} {{in}}

# Create a new year crate
prep:
    cargo run --release -- new {{year}}

# Test today's solution against examples
test:
    cargo test -p y_{{year}} --release day_{{day}}_part_

# Test all of this year's solutions against examples
test-all:
    cargo test -p y_{{year}} --release

# Open VSCodium to today's file
start:
    codium . --goto years/{{year}}/src/day_{{day}}.rs
