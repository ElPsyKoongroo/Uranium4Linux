#![allow(dead_code)]


const BASE_CUR_URL: &str = "https://api.curseforge.com";
const BASE_MRN_URL: &str =  "https://api.modrinth.com/api/v1/mod?";


pub struct ModRinth{

}


impl ModRinth {
    
    pub fn search() -> String{
        format!("{}", BASE_MRN_URL)
    }
    
    pub fn search_for(limit: u32, offset: u32) -> String{
        format!("{}limit={}&offset={}", BASE_MRN_URL, limit, offset)
    }
    
}
pub struct Curse{

}

impl Curse{

    pub fn search_for() -> String{ 
        format!("{}/v1/mods/search?gameid=432&pageSize=20&classId=6", BASE_CUR_URL)
    }
}



/*
pub fn curse_games() -> String{
    format!("{}/v1/categories", BASE_CUR_URL)
}
*/