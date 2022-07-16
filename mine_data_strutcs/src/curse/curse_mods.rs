#![allow(dead_code)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)] 
/// This struct only contains data about the mod logo.
struct Logo {
    id: usize,
    modId: usize,
    thumbnailUrl: String,
    url: String
}


#[derive(Serialize, Deserialize, Clone, Debug)]
/// This struct contains the data about the specific file of a mod
pub struct CurseFile {
    id: usize,
    gameId: usize,
    modId: usize,
    displayName: String,
    fileName: String,
    downloadUrl: Option<String>,
    fileLength: usize,
    gameVersions: Vec<String>
}

impl CurseFile {
    
    pub fn get_fileName(&self) -> String {
        self.fileName.clone()
    }

    pub fn get_downloadUrl(&self) -> String {
        self.downloadUrl.clone().unwrap_or_default()
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
/// This struct contains the data about the request of a fingerprint
/// Fingerprint requets are like
///:"data": {
///:    exactMatches: \[
///:        CurseFile 
///:    \]
///:}
pub struct CurseFingerPrint {
    pub exactMatches: Vec<CurseFile>
}


#[derive(Serialize, Deserialize, Clone, Debug)]
/// This struct contains the data about a single version of a mod
pub struct CurseVersion{
    id: usize,
    gameId: usize,
    name: String,         
    slug: String,         
    downloadCount: usize,
    latestFiles: Vec<CurseFile>,
}


#[derive(Serialize, Deserialize, Clone, Debug)] 
/// This struct contains the data about the multiple versions of a mod
pub struct CurseVersions{
    data: Vec<CurseVersion>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
/// Because the standar response from Curse API is:
/// "data": {
///     * fields of other struct *
/// }
/// We need this struct.
pub struct CurseResponse<T: Serialize>{
    pub data: T
}

/*                          
"data": {
        "id": 238222,
        "gameId": 432,
        "name": "Just Enough Items (JEI)",
        "slug": "jei",
        "links": {
            "websiteUrl": "https://www.curseforge.com/minecraft/mc-mods/jei",
            "wikiUrl": "",
            "issuesUrl": "https://github.com/mezz/JustEnoughItems/issues?q=is%3Aissue",
            "sourceUrl": "https://github.com/mezz/JustEnoughItems"
        },
        "summary": "View Items and Recipes",
        "status": 4,
        "downloadCount": 176901586,
        "isFeatured": false,
        "primaryCategoryId": 423,
        "categories": [{
            "id": 421,
            "gameId": 432,
            "name": "API and Library",
            "slug": "library-api",
            "url": "https://www.curseforge.com/minecraft/mc-mods/library-api",
            "iconUrl": "https://media.forgecdn.net/avatars/6/36/635351496947765531.png",
            "dateModified": "2014-05-23T03:21:44.06Z",
            "isClass": false,
            "classId": 6,
            "parentCategoryId": 6
        }, {
            "id": 423,
            "gameId": 432,
            "name": "Map and Information",
            "slug": "map-information",
            "url": "https://www.curseforge.com/minecraft/mc-mods/map-information",
            "iconUrl": "https://media.forgecdn.net/avatars/6/38/635351497437388438.png",
            "dateModified": "2014-05-08T17:42:23.74Z",
            "isClass": false,
            "classId": 6,
            "parentCategoryId": 6
        }],
        "classId": 6,
        "authors": [{
            "id": 32358,
            "name": "mezz",
            "url": "https://www.curseforge.com/members/17072262-mezz?username=mezz"
        }],
        "logo": {
            "id": 29069,
            "modId": 238222,
            "title": "635838945588716414.jpeg",
            "description": "",
            "thumbnailUrl": "https://media.forgecdn.net/avatars/thumbnails/29/69/256/256/635838945588716414.jpeg",
            "url": "https://media.forgecdn.net/avatars/29/69/635838945588716414.jpeg"
        },
        "screenshots": [{
            "id": 31417,
            "modId": 238222,
            "title": "Recipe Completion",
            "description": "",
            "thumbnailUrl": "https://media.forgecdn.net/attachments/thumbnails/31/417/310/172/thzzdin.png",
            "url": "https://media.forgecdn.net/attachments/31/417/thzzdin.png"
        }, {
            "id": 31419,
            "modId": 238222,
            "title": "Potions",
            "description": "",
            "thumbnailUrl": "https://media.forgecdn.net/attachments/thumbnails/31/419/310/172/t7f7jh6.png",
            "url": "https://media.forgecdn.net/attachments/31/419/t7f7jh6.png"
        }, {
            "id": 31420,
            "modId": 238222,
            "title": "Itemlist Edit Mode",
            "description": "",
            "thumbnailUrl": "https://media.forgecdn.net/attachments/thumbnails/31/420/310/172/tgafkma.png",
            "url": "https://media.forgecdn.net/attachments/31/420/tgafkma.png"
        }, {
            "id": 31418,
            "modId": 238222,
            "title": "Big Screen Support",
            "description": "",
            "thumbnailUrl": "https://media.forgecdn.net/attachments/thumbnails/31/418/310/172/9lngh5f.png",
            "url": "https://media.forgecdn.net/attachments/31/418/9lngh5f.png"
        }],
        "mainFileId": 3847103,
        "latestFiles": [{
            "id": 3040523,
            "gameId": 432,
            "modId": 238222,
            "isAvailable": true,
            "displayName": "jei_1.12.2-4.16.1.301.jar",
            "fileName": "jei_1.12.2-4.16.1.301.jar",
            "releaseType": 1,
            "fileStatus": 4,
             "hashes": [{
                "value": "3045e8440ea44071d8b83c4e7b3c190348fdc527",
                "algo": 1
            }, {
                "value": "1dee4be93d666e2228039c551e927b35",
                "algo": 2
            }],
            "fileDate": "2020-08-24T01:01:39.123Z",
            "fileLength": 653211,
            "downloadCount": 11752168,
            "downloadUrl": "https://edge.forgecdn.net/files/3040/523/jei_1.12.2-4.16.1.301.jar",
            "gameVersions": ["1.12.2"],
            "sortableGameVersions": [{
                "gameVersionName": "1.12.2",
                "gameVersionPadded": "0000000001.0000000012.0000000002",
                "gameVersion": "1.12.2",
                "gameVersionReleaseDate": "2017-09-18T05:00:00Z",
                "gameVersionTypeId": 628
            }],
            "dependencies": [],
            "alternateFileId": 0,
            "isServerPack": false,
            "fileFingerprint": 3089143260,
            "modules": [{
                "name": "META-INF",
                "fingerprint": 2236405288
            }, {
                "name": "mezz",
                "fingerprint": 2222830911
            }, {
                "name": "pack.mcmeta",
                "fingerprint": 1488642189
            }, {
                "name": "mcmod.info",
                "fingerprint": 3528499262
            }, {
                "name": "assets",
                "fingerprint": 9943101
            }]
        },
 
 * */
