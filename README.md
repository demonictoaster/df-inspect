# df-inspect
Pretty print csv files in the command line. The goal of this small project was to build a simple CLI tool to learn more about Rust and GitHub actions.

# Installation
Precompiled binaries for Linux and macOS are archived and attached to each [release](https://github.com/demonictoaster/df-inspect/releases) during CI. These can be downloaded and should be directly usable on the target platfrom. 

# Use
`df-inspect <file.csv>`

The following optional arguments are can be used:
* `-n`, `--nrows`: number of rows to display (int, optional)
* `c`, `--cols`: subset of columns to be printed (string(s), optional)
* `--no-headers`: specificy whether the first rows contains the headers (bool, optional) 
* `-h`, `--help`: print help