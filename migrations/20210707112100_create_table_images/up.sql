CREATE TABLE images (
    id INTEGER PRIMARY KEY,
    filename TEXT NOT NULL,
    abs_path TEXT NOT NULL UNIQUE
);