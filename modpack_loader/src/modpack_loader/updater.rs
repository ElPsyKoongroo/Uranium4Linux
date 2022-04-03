#![allow(dead_code)]
use std::ops::Index;
use std::{fs, time::Duration};

use reqwest::Response;
use serde_json::{Error};
use tokio::task::{self, JoinHandle};
use tokio::time;
use serde::{Deserialize, Serialize};

use super::modpack_struct::*;


fn updater(modpack_path: Path){
    println!("Updater is not implemented yet");
    let mut pack: Modpack = ModPack::new();
    pack.load_pack(modpack_path);    
    println!("{}", pack);
    
}

/*


let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
let text = "2012-03-14, 2013-01-01 and 2014-07-05";
for cap in re.captures_iter(text) {
    println!("Month: {} Day: {} Year: {}", &cap[2], &cap[3], &cap[1]);
}
// Output:
// Month: 03 Day: 14 Year: 2012
// Month: 01 Day: 01 Year: 2013
// Month: 07 Day: 05 Year: 2014


*/