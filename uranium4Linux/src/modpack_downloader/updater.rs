use crate::code_functions::N_THREADS;
use crate::hashes::rinth_hash;
use mine_data_strutcs::rinth::rinth_mods::{Attributes, RinthVersion, RinthVersions};
use mine_data_strutcs::url_maker::maker::ModRinth;
use requester::async_pool::AsyncPool;
use requester::mod_searcher::{search_by_url, search_version_by_id};
use requester::requester::request_maker::RinthRequester;


pub async fn update_modpack(minecraft_path: &str) {
    let mods_path = minecraft_path.to_owned() + "mods/";
    let mods_names = std::fs::read_dir(&mods_path).unwrap();
    let mods_hashes = mods_names
        .map(|f| rinth_hash(f.unwrap().path().to_str().unwrap()))
        .collect::<Vec<String>>();



    println!("Getting identifiers");
    let mods_ids = get_identifiers_from_hashes(&mods_hashes).await;
    get_updated_mods(&mods_ids).await;

    /* TODO
     *
     * 1. Get mods_lastests_version for the current minecraft version
     * 2. Compare if the installed one is the new one
     * 3. Update
     */

    /*
    unzip_temp_pack(modpack_path);

    let json_name = TEMP_DIR.to_owned() + &modpack_path.replace(EXTENSION, ".json");

    let old_modpack: UraniumPack = load_pack(&json_name).unwrap();
    let identifiers = get_project_identifiers(&old_modpack);

    let mods_to_update: VecDeque<Mods> = get_updates(&identifiers).await;

    let mut updated_modpack = UraniumPack::new();
    make_updates(mods_to_update, &mut updated_modpack);

    updated_modpack.set_name(old_modpack.get_name());
    updated_modpack.set_version(old_modpack.get_version());
    updated_modpack.write_mod_pack();
    */
}

async fn get_identifiers_from_hashes(mods_hashes: &[String]) -> Vec<String> {
    let client = reqwest::Client::new();
    let mut ids: Vec<String> = Vec::with_capacity(mods_hashes.len());
    for chunk in mods_hashes.chunks(N_THREADS()) {
        let mut pool = AsyncPool::new();
        let mut tasks = Vec::with_capacity(N_THREADS());



        chunk
            .into_iter()
            .for_each(|hash| tasks.push(search_by_url(&client, &ModRinth::hash(&hash))));

        pool.push_request_vec(tasks);
        pool.start().await;

        let responses: Vec<reqwest::Response> =
            pool.get_done_request().into_iter().flatten().collect();
        let rinth_versions = get_parsed_response::<RinthVersion>(responses).await;


        let mut rinth_ids = rinth_versions
            .iter()
            .map(RinthVersion::get_project_id)
            .collect::<Vec<String>>();
        ids.append(&mut rinth_ids);
    }
    ids
}


async fn get_updated_mods(ids: &[String]) {
    //let mut lastests_versions = Vec::with_capacity(ids.len());
    // let cliente = RinthRequester::new();
    let cliente = reqwest::Client::new();
    println!("Getting updates");
    for id_chunk in ids.chunks(N_THREADS()) {
        let mut pool = AsyncPool::new();
        let mut tasks = Vec::with_capacity(id_chunk.len());


        id_chunk
            .into_iter()
            .for_each(|id| tasks.push(search_by_url(&cliente, &ModRinth::mod_versions_by_id(id))));

        pool.push_request_vec(tasks);
        pool.start().await;

        let responses: Vec<reqwest::Response> =
            pool.get_done_request().into_iter().flatten().collect();


        println!("Parsing Responses");
        let rinth_versions: Vec<RinthVersions> = get_parsed_response::<RinthVersions>(responses).await;


        println!("{}", rinth_versions.len());
        //let lastests_versions = get_lastest_versions(&rinth_versions, 1192);
    }
}

fn get_lastest_versions(rinth_versions: &[RinthVersions], minecraft_version: usize) {
    let fabric_versions: Vec<Vec<RinthVersion>> = rinth_versions
        .iter()
        .map(|v| v.filter_by(Attributes::Loader, "fabric"))
        .collect();
    
    let mut updated_versions = Vec::with_capacity(rinth_versions.len());

    for mod_versions in fabric_versions {
        if mod_versions.iter().any(|mod_v| {
            mod_v
                .get_versions_usize()
                .iter()
                .any(|v| *v == minecraft_version)
        }) {
            let updated_version = select_version(&mod_versions, minecraft_version);

                
            // If there is a new version of a mod push it to the vector
            if let Some(a) = updated_version {
                println!("{:?}", a);
                updated_versions.push(a);
            }
        }
    }
}

fn select_version(mod_versions: &[RinthVersion], minecraft_version: usize) -> Option<RinthVersion> {
    let lastest_version_index = 0;

    for version in mod_versions {
        println!("{:?}", version.get_versions_usize());
        if version
            .get_versions_usize()
            .iter()
            .any(|v| *v == minecraft_version)
        {
            return Some(version.clone())
        }
    }
    None
}

async fn get_parsed_response<T>(responses: Vec<reqwest::Response>) -> Vec<T>
where
    T: serde::de::DeserializeOwned,
    T: Default
{
    let mut parsed_vec = Vec::with_capacity(responses.len());

    for response in responses.into_iter() {
        match response.text().await {
            Ok(version) => { 
                println!("{}\n\n", version);
                parsed_vec.push(serde_json::from_str(&version).unwrap_or_default()); 
            },
            Err(e) => {
                println!("{:?}", e);
            }
        } 

    }
    parsed_vec
}


/*
/// Update the old versions of the mods with the new ones. <br>
/// Consumes `mods_to_update`.
fn make_updates(mods_to_update: VecDeque<Mods>, updated_modpack: &mut UraniumPack) {
    mods_to_update
        .into_iter()
        .for_each(|m| updated_modpack.push_mod(m));
}

/// Sorts the modpack mods by their identifiers
fn sort_mods(mods: RinthVersions, identifiers: &[String]) -> RinthVersions {
    let mut sorted_mods: RinthVersions = RinthVersions::new();

    for identifier in identifiers {
        for mod_ in mods.mods() {
            if mod_.get_project_id() == *identifier {
                sorted_mods.push(mod_.clone());
            }
        }
    }
    sorted_mods
}

/// Compare every mod of the old modpack with the last version found.
async fn get_updates(identifiers: &[String]) -> VecDeque<Mods> {
    let mut mods_lastests_versions: RinthVersions = RinthVersions::new();
    let mut updated_mods: VecDeque<Mods> = VecDeque::new();

    get_new_versions(identifiers, &mut mods_lastests_versions).await;
    mods_lastests_versions = sort_mods(mods_lastests_versions, identifiers);

    resolve_dependencies(&mut mods_lastests_versions).await;

    for i in 0..mods_lastests_versions.len() {
        updated_mods.push_back(Mods::from_RinthVersion(mods_lastests_versions.mod_at(i)));
    }
    updated_mods
}

#[allow(dead_code)]
/// True if `old_mod` is not the lastest version of the mod
fn is_newest(old_mod: &Mods, new_mod: &Mods) -> bool {
    old_mod.get_file() != new_mod.get_file()
}

/// Get the latest versions of all the idetifiers
async fn get_new_versions(identifiers: &[String], mods_info: &mut RinthVersions) {
    let mut pool = AsyncPool::new();
    for id in identifiers.iter() {
        let task = search_mod_by_id(id);
        pool.push_request(task);
    }

    pool.start().await;
    let done_responses = pool.get_done_request();
    for response in done_responses {
        let value = response.text().await.unwrap();
        let versions: Result<Vec<RinthVersion>, serde_json::Error> =
            serde_json::from_str(value.as_str());
        match versions {
            Ok(t) => {
                // TODO: Check if the version is the lastest
                mods_info.push(t[0].clone());
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
}

fn get_project_identifiers(modpack: &UraniumPack) -> Vec<String> {
    let re = Regex::new("data/(.{8})").unwrap(); // MAGIC !!
    let mut identifiers = Vec::new();

    for minecraft_mod in modpack.mods() {
        for cap in re.captures_iter(minecraft_mod.get_file().as_str()) {
            identifiers.push(cap[1].to_owned());
        }
    }
    identifiers
}

async fn resolve_dependencies(mods: &mut RinthVersions) {
    let mut dep_vector = Vec::new();

    for mine_mod in mods.mods() {
        if !mine_mod.had_dependencies() {
            continue;
        }
        // For each dependency check if it is already in the pack, if not, add it
        for dependency in mine_mod.get_dependencies() {
            if !mods.has(dependency.get_project_id()) {
                let response = search_version_by_id(dependency.get_version_id())
                    .await
                    .unwrap();
                let version: RinthVersion = response.json().await.unwrap();

                #[cfg(debug_assertions)]
                println!(
                    "The following dependency was added: {} by {}",
                    version.get_name(),
                    mine_mod.get_name()
                );

                dep_vector.push(version);
            }
        }
    }

    dep_vector.into_iter().for_each(|dep| mods.push(dep));
}
*/
