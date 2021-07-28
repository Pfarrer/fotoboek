CREATE TABLE previews (
    id INTEGER PRIMARY KEY,
    image_id INTEGER NOT NULL,
    size TEXT NOT NULL,
    data BLOB NOT NULL,
    FOREIGN KEY (image_id) REFERENCES images (id) ON DELETE CASCADE,
    UNIQUE (image_id, size)
);
