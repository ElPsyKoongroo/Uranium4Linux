#![allow(dead_code)]
use regex::Regex;
use super::modpack_struct::*;
use requester::requester::request_maker::Requester;
use minecraft_mod::url_maker;

pub fn update_modpack(modpack_path: String) {
    println!("Updater is not implemented yet");
    let pack: ModPack = load_pack(&modpack_path).unwrap();
    let re = Regex::new("data/(.{8})").unwrap();
    let mut identifiers = Vec::new();


    for mmod in pack.mods() {
        for cap in re.captures_iter(mmod.get_file().as_str()){
            identifiers.push(cap[1].to_string());
        }
    }
    get_updates(identifiers);
    
}


fn get_updates(identifiers: Vec<String>) {
    // let req = Requester::new();

    for id in identifiers{
        let url = url_maker::maker::ModRinth::mod_versions_by_id(&id);
        println!("{url}");
    }
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