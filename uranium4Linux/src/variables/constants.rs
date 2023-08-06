use std::sync::RwLock;

pub const EXTENSION: &str = ".mrpack";

pub const HELP_MSG: &str = "
-D | --download         Download the specified modpack in the specified route
-U | --update           Upgrade the specified modpack
-M | --make             Make a modpack file from specified route
-R | --request <TYPE>   Make a requests
-I | --instanciate
   | --list-instances   Print all minecraft versions
-h | --help             Print this message

-f                      Specify the file/dir for [download/make modpack]
-m                      Specify the minecraft root dir
-t                      Specify the numbers of threads for searching/downloading mods
-c                      If enable it means the modpack is a curse pack
-r                      If enable it means the modpack is a rinth pack

REQUESTS TYPES
---------------------------

--query <string>
--for <limit> <offset>
--version <id>
--versions <id>
--mod <id>
--project <id>
--resourcepacks <limit> <offset>

";

pub const TEMP_DIR: &str = "./temp_dir/";
pub const DEFAULT_NTHREADS: usize = 32;
pub const RINTH_JSON: &str = "modrinth.index.json";
pub const CURSE_JSON: &str = "manifest.json";
pub const CONFIG_DIR: &str = "config/";
pub const OVERRIDES_FOLDER: &str = "overrides/";

pub static NTHREADS: RwLock<usize> = RwLock::new(32);

// SHORT ACTIONS FLAGS
pub const SHORT_DOWNLOAD: &str = "-D";
pub const SHORT_MAKE: &str = "-M";
pub const SHORT_REQUEST: &str = "-R";
pub const SHORT_INSTACIATE: &str = "-I";
pub const SHORT_HELP: &str = "-h";
pub const SHORT_UPDATE: &str = "-U";

// LONG ACTIONS FLAGS
pub const LONG_DOWNLOAD: &str = "--download";
pub const LONG_MAKE: &str = "--make";
pub const LONG_UPDATE: &str = "--update";
pub const LONG_REQUEST: &str = "--request";
pub const LONG_HELP: &str = "--help";
pub const LONG_INSTACIATE: &str = "--instanciate";
pub const LIST_INSTANCES: &str = "--list-instances";

// OPTIONS FLAGS
pub const THREADS_FLAG: &str = "-t";
pub const RINTH_FLAG: &str = "-r";
pub const CURSE_FLAG: &str = "-c";
pub const FILE_FLAG: &str = "-f";
pub const ROOT_FLAG: &str = "-m";

// REQUESTS TYPES
pub const QUERY: &str = "--query";
pub const FOR: &str = "--for";
pub const VERSION: &str = "--version";
pub const VERSIONS: &str = "--versions";
pub const MOD: &str = "--mod";
pub const PROJECT: &str = "--project";
pub const RESOURCEPACKS: &str = "--resourcepacks";
pub const MODPACKS: &str = "--modpacks";

// ERROR MESSAGES
pub const DOWNLOAD_ERROR_MSG: &str = "Error with the download request";
pub const CANT_CREATE_DIR: &str = "Cant create the directory";
