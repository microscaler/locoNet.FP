use std::collections::HashMap;
use std::iter::FromIterator;
use std::num::ParseIntError;
use std::str::FromStr;
use regex::Regex;

pub trait Helpers {
    fn slice(&self, index_from: usize, index_to: usize) -> Result<Vec<Self> where Self: Sized, Vec<Self>> {
        if index_from > index_to {
            return Err("indexFrom is bigger than indexTo!".into());
        }
        Ok(self[index_from..index_to].to_vec())
    }

    fn with_max_length(&self, max_length: usize) -> String where Self: AsRef<str> {
        let text = self.as_ref();
        if text.len() <= max_length {
            text.to_string()
        } else {
            text[..max_length].to_string()
        }
    }

    fn wrap_at_length(&self, max_length: usize) -> Result<Vec<String>, &'static str> where Self: AsRef<str> {
        let text = self.as_ref();
        if max_length < 1 {
            return Err("'maxLength' must be greater than 0.");
        }
        let mut result = Vec::new();
        let mut start = 0;

        while start < text.len() {
            let end = std::cmp::min(start + max_length, text.len());
            result.push(text[start..end].to_string());
            start += max_length;
        }
        Ok(result)
    }

    fn merge_with(&mut self, new_options: Option<HashMap<String, String>>) 
    where Self: FromIterator<(String, String)> {
        if let Some(options) = new_options {
            for (key, value) in options {
                self.insert(key, value);
            }
        }
    }

    fn value_or_default(&self, key: &str, default_value: &str) -> String where Self: std::ops::Index<&str, Output = Option<String>> {
        self.get(key).unwrap_or_else(|| default_value.to_string())
    }

    fn if_null_or_empty(&self, fallback: &str) -> String where Self: AsRef<str> {
        let val = self.as_ref();
        if val.is_empty() {
            fallback.to_string()
        } else {
            val.to_string()
        }
    }

    fn if_null_or_empty_func<F>(&self, fallback: F) -> String 
    where F: FnOnce() -> String, Self: AsRef<str> {
        let val = self.as_ref();
        if val.is_empty() {
            fallback()
        } else {
            val.to_string()
        }
    }

    fn split(&self, chunk_sizes: &[usize]) -> Vec<String> where Self: AsRef<str> {
        let text = self.as_ref();
        let mut list_of_strings = Vec::new();
        let mut ix = 0;

        for &chunk_size in chunk_sizes {
            if ix + chunk_size > text.len() {
                break;
            }
            list_of_strings.push(text[ix..ix + chunk_size].to_string());
            ix += chunk_size;
        }
        list_of_strings
    }

    fn int_pow(&self, exp: u32) -> i32 where Self: Copy {
        let mut result = 1;
        let base = *self;
        for _ in 0..exp {
            result *= base;
        }
        result
    }

    fn parse_timeout(&self) -> Result<i32, ParseIntError> where Self: AsRef<str> {
        let timeout_regex = Regex::new(r"^(\d+)\s*?([ms]*)\s*?$").unwrap();
        let s = self.as_ref();
        if let Some(caps) = timeout_regex.captures(s) {
            if let Some(value_str) = caps.get(1) {
                let value = value_str.as_str().parse::<i32>()?;
                let suffix = caps.get(2).map_or("", |m| m.as_str());

                let result = match suffix {
                    "m" => value * 60_000,
                    "s" => value * 1_000,
                    "ms" | "" => value,
                    _ => 0,
                };
                return Ok(result);
            }
        }
        Ok(0)
    }
}

// Implement the Helpers trait for Vec<T>
impl<T> Helpers for Vec<T> {}

// Implement the Helpers trait for String
impl Helpers for String {}

// Implement the Helpers trait for str
impl Helpers for &str {}