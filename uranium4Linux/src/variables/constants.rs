use std::{sync::RwLock, cell::Cell};

pub const HELP_MSG: &str = 
"
-d Download the specified modpack in the specified route
-u Upgrade the specified modpack
--make Make a modpack file from specified route
--request <TYPE> Make a requests
-h Print this message

-f Specify the file/dir for [download/make modpack]
-m Specify the minecraft root dir
-t Specify the numbers of threads for searching/downloading mods
-c If enable it means the modpack is a curse pack
-r If enable it means the modpack is a rinth pack

REQUESTS TYPES 
---------------------------

--querry <string>
--for <limit> <offset> 
--version <id>
--versions <id>
--mod <id>
--project <id>

";

pub const EXTENSION: &str = ".zip";
pub const TEMP_DIR: &str = "./temp_dir/";
pub const DEFAULT_NTHREADS: usize = 32;

// TODO! Look the possibilitie to add this constant so pass the number
// of threads to every function wont be necesary anymore
// MORE DEVELOPMENT IS NEEDED !
pub static NTHREADS: RwLock<usize> = RwLock::new(32);

// ACTIONS FLAGS
pub const DOWNLOAD: &str = "-d";
pub const MAKE: &str = "--make";
pub const REQUEST: &str = "--request";
pub const HELP: &str = "-h";

// OPTIONS FLAGS
pub const THREADS_FLAG: &str = "-t";
pub const RINTH_FLAG: &str = "-r";
pub const CURSE_FLAG: &str = "-c";
pub const FILE_FLAG: &str = "-f";
pub const ROOT_FLAG: &str = "-m";














