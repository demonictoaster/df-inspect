# df-inspect
Pretty print csv files in the command line. The goal of this small project was to build a simple CLI tool to learn more about Rust and GitHub actions.

# Installation
1. Precompiled binaries for Linux and macOS are archived and attached to each [release](https://github.com/demonictoaster/df-inspect/releases) during CI. These can be downloaded and should be directly usable on the target platfrom. 
2. Add the binary to `${HOME}/bin`.
3. Make the binary executable: `chmod 755 ${HOME}/bin/df-inspect`
4. Add binary to your PATH. In your `.bashrc`, add the following: `export PATH="${HOME}/bin:${PATH}"`
5. Restart terminal / `source .bashrc`
6. On macOS, the OS might complain that the developer cannot be verified (its usually better to install binaries via a package manager like homebrew). If so, have a look at [this](https://zaiste.net/os/macos/howtos/resolve-macos-cannot-be-opened-because-the-developer-cannot-be-verified-error/).

# Use
`df-inspect <file.csv>`

The following optional arguments can be used:
* `-n`, `--nrows`: number of rows to display (int, optional)
* `-c`, `--cols`: subset of columns to be printed (string(s), optional)
* `--no-headers`: specificy whether the first rows contains the headers (bool, optional) 
* `-h`, `--help`: print help

# Example
`df-inspect network.csv -c duration protocol_type src_bytes -n 15`

![Alt text](example/screenshot.png?raw=true "Screenshot")
