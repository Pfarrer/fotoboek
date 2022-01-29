table! {
    file_metadata (file_id) {
        file_id -> Nullable<Integer>,
        file_size_bytes -> Integer,
        file_hash -> Text,
        file_date -> Timestamp,
        resolution_x -> Integer,
        resolution_y -> Integer,
        exif_date -> Nullable<Timestamp>,
        exif_aperture -> Nullable<Text>,
        exif_exposure_time -> Nullable<Text>,
        exif_iso -> Nullable<Text>,
        exif_camera_manufacturer -> Nullable<Text>,
        exif_camera_model -> Nullable<Text>,
        exif_gps_lat -> Nullable<Float>,
        exif_gps_lon -> Nullable<Float>,
        effective_date -> Timestamp,
    }
}

table! {
    files (id) {
        id -> Nullable<Integer>,
        rel_path -> Text,
        file_type -> Text,
        file_name -> Text,
    }
}

table! {
    tasks (id) {
        id -> Nullable<Integer>,
        file_id -> Integer,
        module -> Text,
        priority -> Integer,
        work_started_at -> Timestamp,
        max_worker_id -> Integer,
    }
}

joinable!(file_metadata -> files (file_id));
joinable!(tasks -> files (file_id));

allow_tables_to_appear_in_same_query!(
    file_metadata,
    files,
    tasks,
);
