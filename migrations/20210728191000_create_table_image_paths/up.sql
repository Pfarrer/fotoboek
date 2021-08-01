CREATE TABLE image_paths (
    abs_dir_path TEXT NOT NULL,
    image_id INTEGER NOT NULL,
    distance INTEGER NOT NULL,
    parent_dir_path TEXT NULL,
    PRIMARY KEY (abs_dir_path, image_id)
    FOREIGN KEY (image_id) REFERENCES images (id) ON DELETE CASCADE
);
