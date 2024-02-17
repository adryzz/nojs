use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use megalodon::entities::{Account, Status};

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

pub fn try_get_host(mut str: &str) -> Option<&str> {
    if str.starts_with("https://") {
        str = &str[8..];
    } else if str.starts_with("http://") {
        str = &str[7..];
    }

    if let Some(i) = str.find('/') {
        str = &str[..i];
    }

    if let None = str.find('.') {
        return None;
    }

    return Some(str);
}

pub fn try_get_filter_host<'a>(str: &'a str, filter: &str) -> Option<&'a str> {
    let out = try_get_host(str);
    if let Some(s) = out {
        if s == filter {
            return None;
        }
    }

    return out;
}

pub fn get_retooter(toot: &Status) -> Option<Account> {
    if let Some(_retoot) = &toot.reblog {
        Some(toot.account.clone())
    } else {
        None
    }
}

pub fn remove_emotes(input: &str) -> String {
    let mut result = String::new();
    let mut in_sequence = false;
    
    for c in input.chars() {
        if c == ':' {
            in_sequence = !in_sequence;
        } else if !in_sequence {
            result.push(c);
        }
    }
    
    result
}