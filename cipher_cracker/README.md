# Cipher Cracker by Superior404

## Overview
Cipher Cracker is a tool designed to identify the hashing method used for user passwords in an `/etc/shadow` file and attempt to crack the passwords using a given wordlist. This tool supports multiple hash methods, including MD5, SHA-256, and SHA-512. Blowfish and yescrypt are not supported at this time.

**Important Note:** This tool was developed for educational purposes only. Usage of this tool should comply with the laws in your respective country. Unauthorized use of this tool may be illegal and is not endorsed by the author. The author is not responsible for any misuse or damage caused by this tool.

## Features
- Identify hash methods from `/etc/shadow` files.
- Crack passwords using a specified wordlist.
- Supports MD5, SHA-256, and SHA-512 hashing methods.
- Excludes support for Blowfish and yescrypt.

## Requirements
- Rust programming language (latest stable version)

## Usage
### Command Line Flags
- `-w, --wordlist` : Path to the wordlist file.
- `-p, --pwd_file` : Path to the password file.

### Running the Tool
1. Clone the repository and navigate to the project directory.
2. Place your shadow file and wordlist in the `example_files` directory.
3. Run the tool using Cargo:
    ```sh
    cargo run -- -w example_files/wordlist.txt -p example_files/shadow.txt
    ```

### Help Command
For help and additional options, run:
```sh
cargo run -- -h
```

## Example Files
For testing purposes, use the files provided in the `example_files` directory:
- `wordlist.txt`: A sample wordlist file.
- `shadow.txt`: A sample shadow file.

## Dependencies
This project relies on the following crates:
- [Clap](https://docs.rs/clap/latest/clap/_tutorial/index.html) for command-line argument parsing.
- [Colored](https://docs.rs/colored/latest/colored/) for colored terminal output.
- [md-5](https://crates.io/crates/md-5) for MD5 hashing.
- [rust-crypto](https://docs.rs/rust-crypto/latest/crypto/sha2/index.html) for SHA-256 and SHA-512 hashing.

## Implementation Details
### Main Function
The `main` function parses command-line arguments, reads the wordlist and password file, and initiates the hash comparison process.

### Reading Wordlist and Password Files
- `read_wordlist_hash(wordlist_file: &str) -> Vec<String>`: Reads the wordlist file and returns a list of words.
- `read_pwd_file(pwd_file: &str) -> Vec<String>`: Reads the password file and returns a list of password entries.

### Hash Comparison
- `hash_compare(wordlist: Vec<String>, pwd_list: Vec<String>)`: Compares the hashes from the wordlist against the hashed passwords in the password file. If a match is found, it prints the username and the cracked password.

### Hash Method
- `hash_method(word: &str, salt: &str) -> String`: Computes the hash of a given word using the specified hash method and salt.

## Example Output
When a password is successfully cracked, the tool will output:
```
Password found for the user: username Password: password
```

## Legal Disclaimer
This tool is intended for educational purposes only. The usage of this tool should be in compliance with the laws of your country. Unauthorized or malicious use of this tool may be illegal and is not supported by the author. The author assumes no responsibility for any harm or damage caused by the use of this tool.