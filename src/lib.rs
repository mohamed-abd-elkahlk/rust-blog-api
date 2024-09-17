#[macro_export]
macro_rules! timestamp_to_datetime {
    ($row:expr) => {
        $row.created_at.map(|datetime| {
            let unix_timestamp_nanos = datetime.unix_timestamp_nanos();
            DateTime::<Utc>::from_timestamp_nanos(unix_timestamp_nanos as i64)
        })
    };
}
