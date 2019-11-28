# log-loc-rs

Tag log statements with file name and line number

### Requirements

Rust nightly (tested on 2019-10-23).  
(Using the Unicode identifiers (feature `non_ascii_idents`) and the efficient hash map entry API (feature `hash_raw_entry`)).

### Installation

    cargo install --path . --force

### Usage example

    cd komodoDEX/
    log-loc --mode kmd --status --dry-run
