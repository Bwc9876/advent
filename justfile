
_default:
    cargo run --release -- solve *

year := `date +%Y`
day := `date +%d`

p P in="":
    cargo run --release -- solve {{year}}:{{day}}:{{P}} {{in}}

dp DP in="":
    cargo run --release -- solve {{year}}:{{DP}} {{in}}

dyp DYP in="":
    cargo run --release -- solve {{DYP}} {{in}}

prep:
    cargo run --release -- new {{year}}

start:
    codium . --goto years/{{year}}/src/day_{{day}}.rs