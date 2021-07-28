CREATE TABLE folders (
    abs_path TEXT NOT NULL,
    image_id INTEGER NOT NULL,
    distance INTEGER NOT NULL,
    PRIMARY KEY (abs_path, image_id)
    FOREIGN KEY (image_id) REFERENCES images (id) ON DELETE CASCADE
);
