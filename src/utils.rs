use axum::http::StatusCode;
use chrono::{DateTime, Utc};

pub fn remove_protocol(mut input: String) -> String {
    if input.starts_with("https://") {
        input.replace_range(..8, "");
    } else if input.starts_with("http://") {
        input.replace_range(..7, "");
    }

    input
}

pub trait Err<T> {
    fn to_code(self) -> Result<T, StatusCode>;
}

impl<T> Err<T> for Result<T, megalodon::error::Error> {
    fn to_code(self) -> Result<T, StatusCode> {
        self.map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub fn print_datetime(date: &DateTime<Utc>) -> String {
    let now = Utc::now();
    let diff = now - date;
    let seconds = diff.num_seconds();

    if seconds >= 365 * 24 * 60 * 60 {
        return format!("{}y", seconds / (365 * 24 * 60 * 60));
    }
    if seconds >= 30 * 24 * 60 * 60 {
        return format!("{}mo", seconds / (30 * 24 * 60 * 60));
    }
    if seconds >= 7 * 24 * 60 * 60 {
        return format!("{}w", seconds / (7 * 24 * 60 * 60));
    }
    if seconds >= 24 * 60 * 60 {
        return format!("{}d", seconds / (24 * 60 * 60));
    }
    if seconds >= 60 * 60 {
        return format!("{}h", seconds / (60 * 60));
    }
    if seconds >= 60 {
        return format!("{}m", seconds / 60);
    }
    format!("{}s", seconds)
}