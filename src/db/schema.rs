table! {
    images (id) {
        id -> Nullable<Integer>,
        filename -> Text,
        abs_path -> Text,
    }
}

table! {
    tasks (id) {
        id -> Nullable<Integer>,
        image_id -> Nullable<Integer>,
        module -> Text,
        action -> Text,
        blocked_by_task_id -> Nullable<Integer>,
    }
}

joinable!(tasks -> images (image_id));

allow_tables_to_appear_in_same_query!(
    images,
    tasks,
);
