/*
        Resources used to create this tool:
Shadow:	-	https://tecadmin.net/etc-shadow-file-in-linux/

Clap:	-	https://docs.rs/clap/latest/clap/_tutorial/index.html
	    -	https://github.com/clap-rs/clap

Color:	-	https://docs.rs/colored/latest/colored/

Hashes: -	https://crates.io/crates/md-5
        -	https://docs.rs/md-5/latest/md5/
	    -	https://docs.rs/rust-crypto/latest/crypto/sha2/index.html
*/
use colored::Colorize;
use clap::Parser;
use hex::encode;
use md5;
use sha2::{Digest, Sha256, Sha512};
use std::fs;
use std::io::{self, BufRead};

#[derive(Parser, Default, Debug)]
#[clap(author = "Superior404", version, about)]
/// Cipher Cracking Tool
struct Args {
    #[clap(short, long)]
    /// Provide a wordlist
    wordlist: String,

    #[clap(short, long)]
    /// Provide the file with the hashes or passwords
    pwd_file: String,
}

fn main() {
    let args = Args::parse();

    // Read the wordlist and password file
    let wordlist = read_wordlist_hash(&args.wordlist);
    let pwd_list = read_pwd_file(&args.pwd_file);

    // Compare hashes
    hash_compare(wordlist, pwd_list);
}

fn read_wordlist_hash(wordlist_file: &str) -> Vec<String> {
    // Read the wordlist file and return a list of words
    match fs::read_to_string(wordlist_file) {
        Ok(contents) => {
            let wordlist: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
            wordlist
        }
        Err(err) => {
            eprintln!("Error reading wordlist file: {}", err);
            std::process::exit(1);
        }
    }
}

fn read_pwd_file(pwd_file: &str) -> Vec<String> {
    // Read the password file and return a list of password entries
    match fs::File::open(pwd_file) {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let pwd_list: Vec<String> = lines
                .filter_map(|line| line.ok())
                .filter(|line| !line.is_empty())
                .collect();
            pwd_list
        }
        Err(err) => {
            eprintln!("Error reading password file: {}", err);
            std::process::exit(1);
        }
    }
}

fn hash_compare(wordlist: Vec<String>, pwd_list: Vec<String>) {
    for pwd_entry in pwd_list {
        // Split the password entry on ':'
        let parts: Vec<&str> = pwd_entry.splitn(3, ':').collect();

        if parts.len() != 3 {
            eprintln!("Invalid password entry: {}", pwd_entry);
            continue;
        }

        // Extract name and salt+password
        let name = parts[0];
        let salt_password_combined = parts[1];

        // Split salt and password from the combined string
        let salt_password_parts: Vec<&str> = salt_password_combined.splitn(3, '$').collect();

        if salt_password_parts.len() != 3 {
            eprintln!("Invalid salt+password entry: {}", salt_password_combined);
            continue;
        }

        let salt = salt_password_parts[1];
        let password = salt_password_parts[2];

        // Calculate hash using provided hash method and compare with wordlist
        for word in &wordlist {
            // Calculate hash using the provided method (you need to implement this)
            let calculated_hash = hash_method(word, salt);

            // Compare the calculated hash with the stored password hash
            if calculated_hash == password {
                println!("{}: {} {}: {}", "Password found for the user".green(), name.yellow(), "Password".green(), word.yellow());
                break;
            }
        }
    }
}

fn hash_method(word: &str, salt: &str) -> String {
    let word_as_byte = word.as_bytes();
    match salt {
        // MD5
        "1" => {
            let hash_string = md5::compute(word_as_byte)
                .iter()
                .map(|byte| format!("{:02x}", byte))
                .collect::<String>();
            return hash_string;
        }
        // Blowfish - not supported
        "2a" | "2y" => {}
        // SHA-256
        "5" => {
            let mut hasher = Sha256::new();
            hasher.update(word_as_byte);
            return encode(hasher.finalize());
        }
        // SHA-512
        "6" => {
            let mut hasher = Sha512::new();
            hasher.update(word_as_byte);
            return encode(hasher.finalize());
        }
        // yescrypt - not supported here
        "y" => {}
        _ => {}
    }
    format!("HASH({}:{})", word, salt)
}
