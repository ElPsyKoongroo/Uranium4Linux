
# GitHub
- [ ] Add videos showing how to use Uranium4Linux


# Uranium4Linux


## General
- [ ] Try to change argument parsing to CLAP
- [ ] Add a `RinthMaker` struct in `lib.rs`
- [x] If config folder or mods folder don't exit make them


## update.rs
- [ ] Fix `update_modpack` unwrap
- [ ] `update_modpack` should return a `Result<(), Error>`



## maker.rs
- [x] `make_modpack` should return a `Result<(), Error>`
- [ ] Refactor `serach_mods_for_modpack` and `search_mod`
- [x] Change `path` type to `PathBuf`
- [x] Make a type alias for `&[(String, String)]`
- [ ] Change `&'str` to `&Path` to everything that uses a path.
- [x] Refactor `ModpackMaker` into a struct with chunk method
- [x] Make docs for pub functions
- [ ] Make docs for non-pub functions



## pack_zipper.rs
- [ ] Change paths/names types to `PathBuf` or `Path`
- [ ] Fix unwraps in `compress_pack`
- [ ] Change `&str` to `Path`  in `search_files`


## rinth_downloader.rs
- [x] Make docs for pub functions
- [ ] Make docs for non-pub functions
