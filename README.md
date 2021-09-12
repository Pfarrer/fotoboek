# Fotoboek
Fotoboek is a small service that indexes your images. It will prepare a thumbnail and preview for each image, extract metadata (EXIF mostly) and image paths. With this, you can browse your photos by (recursive) image path, date or, geographic location.

## Core Features
- Show images in chronological order (not filename based)
- Allow recursive image galleries spanning any number of sub-folders
- Pre-calculate preview images as early as possible
- Images are never changes or moved

## Open/Finished Tasks
- [x] Index images recursively
  - [x] Triggered by `GET /api/scan`
  - [ ] On startup
  - [ ] Watch for filesystem events (inotify)
- [ ] Detect removed images
- [x] Create image "jobs" when new image is found
  - [x] Lock jobs when worker started working on it
- [x] Generic worker process to handle image jobs
  - [x] Concurrent worker processes
  - [x] Worker process sleep when no jobs available
  - [ ] Notify workers on new jobs
- [x] Extract EXIF data from images
- [x] Generate thumbnail and preview images
- [x] Parse image path and allow recursive image gallery
- [ ] Override image date/order

## Compile
See Opencv prerequisites: https://github.com/twistedfall/opencv-rust

### MacOS
```
brew install opencv
export DYLD_FALLBACK_LIBRARY_PATH="$(xcode-select --print-path)/usr/lib/"
```