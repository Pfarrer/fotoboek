# Fotoboek
Fotoboek is a service that indexes your image gallery optimized for viewing many images in nested folders. It will prepare a thumbnail and preview for each image, extract metadata (EXIF mostly) and image paths. With this, you can browse your photos by (recursive) image path, date or, geographic location.

# This project is discontinued!
... in favor of https://photoprism.app/ which is a newly identical project, far more major and advanced than this project.


## Run it yourself
The simplest method to run Fotoboek is by using Docker:
```yml
version: '3'
services:
  fotoboek:
    container_name: fotoboek
    image: pfarrer/fotoboek
    ports:
      - 1223:1223
    volumes:
      - {path-to-your-media-base-directory}:/opt/media-source
      - fotoboek-storage:/opt/fotoboek-storage
    restart: unless-stopped

volumes:
  fotoboek-storage:
```

Make sure to replace `{path-to-your-media-base-directory}` with your local path
to the base directory containing your media files.

After the container started, access `/api/admin/scan` to trigger a scan for any media files. The number of found and
added files will be returned once the scan is finished. Depending on the number of files, this might take a while.


## Core Features
- Show images in chronological order (not filename based)
- Allow recursive image galleries spanning any number of sub-folders
- Pre-calculate preview images as early as possible
- Images are never changed or moved

## Open/Finished Tasks
- JPG Image Indexing
  - [x] Index images recursively
  - [x] Trigger index by `POST /api/admin/scan`
  - [ ] Trigger index on startup
  - [ ] Trigger index on filesystem events (inotify)
  - [ ] Detect removed images
- Video Indexing
  - [x] Index videos recursively
  - [x] Generate preview images for videos
  - [x] Transcode videos for size and compatibility
- User Interface & Features
  - [x] Basic Gallery
    - [x] Preview image for folders
    - [x] Recursive view in Gallery
    - [ ] Edit all images in a folder (date, comments)
  - [x] Basic Flashback
  - [x] Timeline
    - [x] Infinite Scroll
    - [ ] Jump to any date (buggy at the moment)
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
  - [x] Optimize previews to reduce size


## Compile
See Opencv prerequisites: https://github.com/twistedfall/opencv-rust

### MacOS
```
brew install opencv
export DYLD_FALLBACK_LIBRARY_PATH="$(xcode-select --print-path)/usr/lib/"
```
