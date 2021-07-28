CREATE TABLE tasks (
    id INTEGER PRIMARY KEY,
    image_id INTEGER NOT NULL,
    module TEXT NOT NULL,
    priority INTEGER NOT NULL,
    FOREIGN KEY (image_id) REFERENCES images (id) ON DELETE CASCADE
);
