CREATE TABLE files (
    id INTEGER PRIMARY KEY,
    rel_path TEXT NOT NULL UNIQUE,
    file_type TEXT NOT NULL,
    file_name TEXT NOT NULL
);

CREATE TABLE file_metadata (
    file_id INTEGER PRIMARY KEY,
    file_size_bytes INTEGER NOT NULL,
    file_hash TEXT NOT NULL,
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
    effective_date TIMESTAMP NOT NULL,
    FOREIGN KEY (file_id) REFERENCES files (id) ON DELETE CASCADE
);

CREATE INDEX file_metadata__effective_date
ON file_metadata(effective_date);

CREATE TABLE tasks (
    id INTEGER PRIMARY KEY,
    file_id INTEGER NOT NULL,
    module TEXT NOT NULL,
    priority INTEGER NOT NULL,
    work_started_at TIMESTAMP NOT NULL DEFAULT "1970-01-01 00:00:00",
    FOREIGN KEY (file_id) REFERENCES files (id) ON DELETE CASCADE
);
