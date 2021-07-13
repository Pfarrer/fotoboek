CREATE TABLE tasks (
    id INTEGER PRIMARY KEY,
    image_id INTEGER,
    module TEXT NOT NULL,
    action TEXT NOT NULL,
    blocked_by_task_id INTEGER NULL,
    FOREIGN KEY (image_id) REFERENCES images (id),
    FOREIGN KEY (blocked_by_task_id) REFERENCES tasks (id)
);
