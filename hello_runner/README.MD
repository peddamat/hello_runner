Must do the following to execute binary directly (not using cargo run):
- cp target/debug/deps/libhello.dylib target/debug/
- `export DYLD_LIBRARY_PATH=.`