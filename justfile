set positional-arguments
set shell := ["bash", "-cue"]
root_dir := justfile_directory()

buckify:
    cd ""{{root_dir}}"" && reindeer --third-party-dir external buckify
