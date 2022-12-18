use std::fs;
use std::io::Read;

use hex::ToHex;
use murmurhash32::murmurhash2;
use sha1::{Digest, Sha1};

use crate::checker::*;

fn get_sha1_from_file(file_path: &str) -> String {
    let mut hasher = Sha1::new();
    let mut file = check_panic(
        fs::File::open(file_path),
        true,
        format!("hashes; Could not open {} for hashing", file_path),
    );
    let metadata = check_panic(
        fs::metadata(file_path),
        false,
        format!("hashes; Unable to get metadata from {}", file_path),
    );
    let mut buffer = Vec::with_capacity(metadata.len() as usize); //vec![0; metadata.len() as usize];
    check_panic(
        file.read_to_end(&mut buffer),
        true,
        format!("hashes; Unable to read {}", file_path),
    );

    hasher.update(buffer);
    let temp = hasher.finalize().to_vec();
    temp.encode_hex::<String>()
}

pub fn rinth_hash(path: &str) -> String {
    get_sha1_from_file(path)
}

// TODO! Remove curse
pub fn _curse_hash(path: &String) -> String {
    let mut file = std::fs::File::open(path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    buffer.retain(|&x| (x != 9 && x != 10 && x != 13 && x != 32));
    murmurhash2(&buffer).to_string()
}
