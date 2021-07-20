CREATE TABLE tasks (
    id INTEGER PRIMARY KEY,
    image_id INTEGER NOT NULL,
    module TEXT NOT NULL,
    action TEXT NOT NULL,
    priority INTEGER NOT NULL,
    blocked_by_task_id INTEGER NULL,
    FOREIGN KEY (image_id) REFERENCES images (id) ON DELETE CASCADE,
    FOREIGN KEY (blocked_by_task_id) REFERENCES tasks (id) ON DELETE SET NULL
);
