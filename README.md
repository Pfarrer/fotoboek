# Fotoboek
Fotoboek is a small service that indexes your images. It will prepare a thumbnail and preview for each image, extract metadata (EXIF mostly) and image paths. With this, you can browse your photos by (recursive) image path, date or, geographic location.

## Core Features
- Show images in chronological order (not filename based)
- Allow recursive image galleries spanning any number of sub-folders
- Pre-calculate preview images as early as possible
- Images are never changes or moved

## Open/Finished Tasks
- JPG Image Indexing
  - [x] Index images recursively
  - [x] Trigger index by `GET /api/scan`
  - [ ] Trigger index on startup
  - [ ] Trigger index on filesystem events (inotify)
  - [ ] Detect removed images
- Video Indexing
  - [ ] Index videos recursively
- User Interface & Features
  - [x] Basic Gallery
    - [ ] Preview image for folders
  - [x] Recursive view in Gallery
  - [x] Basic Flashback
  - [ ] Make it pretty
- Worker Framework
  - [x] Create image "jobs" when new image is found
  - [x] Lock jobs when worker started working on it
  - [x] Generic worker process to handle image jobs
  - [x] Concurrent worker processes
  - [x] Worker process sleep when no jobs available
  - [ ] Notify workers on new jobs
- Image Metadata 
  - [x] Extract EXIF data from images
  - [x] Parse image path and allow recursive image gallery
  - [ ] Allow manual override of image date/order
- Image Preview
  - [x] Generate thumbnail and preview images for JPGs

## Compile
See Opencv prerequisites: https://github.com/twistedfall/opencv-rust

### MacOS
```
brew install opencv
export DYLD_FALLBACK_LIBRARY_PATH="$(xcode-select --print-path)/usr/lib/"
```