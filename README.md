## Compile
See Opencv prerequisites: https://github.com/twistedfall/opencv-rust

### MacOS
```
brew install opencv
export DYLD_FALLBACK_LIBRARY_PATH="$(xcode-select --print-path)/usr/lib/"
```

## Initialize new Family Album
```
cargo install diesel_cli
diesel setup
diesel migration run
```