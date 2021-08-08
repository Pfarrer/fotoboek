table! {
    image_paths (abs_dir_path, image_id) {
        abs_dir_path -> Text,
        image_id -> Integer,
        distance -> Integer,
        parent_dir_path -> Nullable<Text>,
    }
}

table! {
    images (id) {
        id -> Nullable<Integer>,
        filename -> Text,
        abs_path -> Text,
    }
}

table! {
    metadata (image_id) {
        image_id -> Integer,
        file_size_bytes -> Integer,
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
    }
}

table! {
    previews (id) {
        id -> Nullable<Integer>,
        image_id -> Integer,
        size -> Text,
        data -> Binary,
    }
}

table! {
    tasks (id) {
        id -> Nullable<Integer>,
        image_id -> Integer,
        module -> Text,
        priority -> Integer,
        work_started_at -> Timestamp,
    }
}

joinable!(image_paths -> images (image_id));
joinable!(metadata -> images (image_id));
joinable!(previews -> images (image_id));
joinable!(tasks -> images (image_id));

allow_tables_to_appear_in_same_query!(
    image_paths,
    images,
    metadata,
    previews,
    tasks,
);
