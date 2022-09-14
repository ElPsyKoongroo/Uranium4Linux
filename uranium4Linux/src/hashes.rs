use hex::ToHex;
use sha1::{Digest, Sha1};
use std::fs;
use std::io::Read;
use murmurhash32::murmurhash2;

fn get_sha1_from_file(file_path: &String) -> String {
    let mut hasher = Sha1::new();
    let mut file = fs::File::open(file_path).unwrap();
    let metadata = fs::metadata(file_path).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    file.read_to_end(&mut buffer).expect("buffer overflow");

    hasher.update(buffer);
    let temp = hasher.finalize().to_vec();
    temp.encode_hex::<String>()
}

pub fn rinth_hash(path: &String) -> String {
    get_sha1_from_file(path)
}

pub fn curse_hash(path: &String) -> String {
    let mut file = std::fs::File::open(path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    buffer.retain(|&x| (x != 9 && x != 10 && x != 13 && x != 32));
    murmurhash2(&buffer).to_string()
}
