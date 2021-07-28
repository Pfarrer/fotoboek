CREATE TABLE metadata (
    image_id INTEGER PRIMARY KEY NOT NULL,
    file_size_bytes INTEGER NOT NULL,
    file_date TIMESTAMP NOT NULL,
    resolution_x INTEGER NOT NULL,
    resolution_y INTEGER NOT NULL,
    exif_date TIMESTAMP NULL,
    exif_aperture TEXT NULL,
    exif_exposure_time TEXT NULL,
    exif_iso TEXT NULL,
    exif_camera_manufacturer TEXT NULL,
    exif_camera_model TEXT NULL,
    exif_gps_lat FLOAT NULL,
    exif_gps_lon FLOAT NULL,
    FOREIGN KEY (image_id) REFERENCES images (id) ON DELETE CASCADE
);
