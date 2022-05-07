use hex;
use sha1::Digest;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
//sha1 cracker
const SHA1_HEX_STRING_LENGHTH: usize = 40;
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Incorrect number of parameters");
        println!("Usage:");
        println!("sha1_cracker <wordlist> <hash>");
        return Err("Invalid arguments".into());
    }

    let hash_to_crack = args[2].trim();

    if hash_to_crack.len() != SHA1_HEX_STRING_LENGHTH {
        return Err("Sha1 hash invalid".into());
    }

    let wordlist = File::open(&args[1])?;
    let buffer = BufReader::new(&wordlist);

    for line in buffer.lines() {
        let line = line?;
        let password_to_try = line.trim();
        if hash_to_crack == hex::encode(sha1::Sha1::digest(password_to_try.as_bytes())) {
            println!("Password cracked: {}", password_to_try);
            return Ok(());
        }
    }

    println!("Password not found in wordlist");
    Ok(())
}
