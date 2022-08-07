pub const HELP: &str = 
"
-d Download the specified modpack in the specified route
-u Upgrade the specified modpack
-m Make a modpack file from specified route
-h Print this message

-f Specify the file/dir for [download/make modpack]
-r Specify the minecraft root dir
-t Specify the numbers of threads for searching/downloading mods
-c If enable it means the modpack is a curse pack
";

pub const EXTENSION: &str = ".zip";
pub const TEMP_DIR: &str = "./temp_dir/";

// TODO! Look the possibilitie to add this constant so pass the number
// of threads to every function wont be necesary anymore
// MORE DEVELOPMENT IS NEEDED !
pub static mut N_THREADS: usize = 32;
