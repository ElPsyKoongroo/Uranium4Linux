use std::sync::RwLock;

pub const HELP_MSG: &str = "
--download Download the specified modpack in the specified route
--update Upgrade the specified modpack
--make Make a modpack file from specified route
--request <TYPE> Make a requests
--instanciate
-h Print this message

-f Specify the file/dir for [download/make modpack]
-m Specify the minecraft root dir
-t Specify the numbers of threads for searching/downloading mods
-c If enable it means the modpack is a curse pack
-r If enable it means the modpack is a rinth pack

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

pub const EXTENSION: &str = ".zip";
pub const TEMP_DIR: &str = "./temp_dir/";
pub const DEFAULT_NTHREADS: usize = 32;
pub const RINTH_JSON: &str = "modrinth.index.json";

// TODO! Look the possibilitie to add this constant so pass the number
// of threads to every function wont be necesary anymore
// MORE DEVELOPMENT IS NEEDED !
pub static NTHREADS: RwLock<usize> = RwLock::new(32);

// SHORT ACTIONS FLAGS
pub const DOWNLOAD: &str = "-d";
pub const MAKE: &str = "--make";
pub const REQUEST: &str = "--request";
pub const INSTACIATE: &str = "-I";
pub const HELP: &str = "-h";

// LONG ACTIONS FLAGS
pub const LONG_DOWNLOAD: &str = "--download";
pub const LONG_MAKE: &str = "--make";
pub const LONG_UPDATE: &str = "--update";
pub const LONG_REQUEST: &str = "--request";
pub const LONG_HELP: &str = "--help";
pub const LONG_INSTACIATE: &str = "--instanciate";

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
