use std::{collections::{HashSet, HashMap}, marker::PhantomData, num::IntErrorKind};
use eyre::{Result, bail};
use chrono::NaiveDateTime;

#[derive(Debug)]
pub enum SortMode {
    Top,
    New,
    Hot,
}

#[derive(Debug)]
pub struct PostQueryParams {
    pub depth: i32,
    pub limit: i32,
    pub sort: SortMode,
    pub before: Option<NaiveDateTime>,
    pub after: Option<NaiveDateTime>,
    pub fields: HashSet<String>,
    pub expansions: HashSet<String>,
    pub tags: HashSet<String>,
    _private: PhantomData<()>, // Prevent this struct from manually being constructed
}

impl Default for PostQueryParams {
    fn default() -> Self {
        PostQueryParams {
            depth: 0,
            limit: 0,
            sort: SortMode::New,
            before: None,
            after: None,
            fields: HashSet::new(),
            expansions: HashSet::new(),
            tags: HashSet::new(),
            _private: PhantomData,
        }
    }
}

impl PostQueryParams {
    pub fn from_query_params(params: HashMap<String, String>) -> Result<PostQueryParams> {
        // Parsing helpers
        let parse_i32: fn(&str, &str) -> Result<i32> = |field, strval| {
            match strval.parse::<i32>() {
                Ok(i) => Ok(i),
                Err(e) => match *e.kind() {
                    IntErrorKind::Empty => bail!("value of '{field}' must be non-empty"),
                    IntErrorKind::InvalidDigit => bail!("value of '{field}' contains an invalid character"),
                    IntErrorKind::NegOverflow | IntErrorKind::PosOverflow => bail!("value of '{field}' is out of range (acceptable values are -2,147,483,648 to 2147483647, inclusive"),
                    IntErrorKind::Zero => bail!("This is an internal server error! Parser returned a zero-value error, but zero is a valid value!"),
                    _ => bail!("an unknown error occurred parsing '{field}'")
                }
            }
        };

        let parse_iso8601: fn(&str, &str) -> Result<NaiveDateTime> = |field, strval| {
            use chrono::format::ParseErrorKind::*;
            match NaiveDateTime::parse_from_str(strval, "%Y%m%dT%H%M%SZ") {
                Ok(t) => Ok(t),
                Err(e) => match e.kind() {
                    OutOfRange => bail!("value of '{field}' is out of range"),
                    Impossible => bail!("value of '{field}' is not a valid date and time"),
                    NotEnough | TooShort => bail!("value of '{field}' does not have enough data to produce a valid date and time"),
                    TooLong => bail!("value of '{field}' is too long"),
                    Invalid => bail!("value of '{field}' has invalid characters"),
                    BadFormat => bail!("value of '{field}' has incorrect format"),
                    _ => bail!("an unknown error occurred parsing '{field}'")
                }
            }
        };

        let parse_csl: fn(&str, &str) -> Result<HashSet<String>> = |_field, strval| {
            Ok(strval.split(',').map(|s| s.to_owned()).collect())
        };

        // Assemble values
        let depth = match params.get("depth") {
            Some(strval) => parse_i32("depth", strval)?,
            None => 0,
        };

        let limit = match params.get("limit") {
            Some(strval) => parse_i32("limit", strval)?,
            None => 0,
        };

        let sort = match params.get("sort").map(|s| s.as_str()) {
            None | Some("new") => SortMode::New,
            Some("hot") => SortMode::Hot,
            Some("top") => SortMode::Top,
            _ => bail!("value for 'sort' must be one of ['new', 'hot', 'top']"),
        };

        let before = match params.get("before") {
            None => None,
            // Parse the format '20230710T221006Z'
            Some(strval) => Some(parse_iso8601("before", strval)?),
        };

        let after = match params.get("after") {
            None => None,
            // Parse the format '20230710T221006Z'
            Some(strval) => Some(parse_iso8601("after", strval)?),
        };

        if after.is_some() && before.is_some() {
            let a = after.as_ref().unwrap();
            let b = before.as_ref().unwrap();
            
            // "after" should be chronologically later than "before"
            // if both are specified. If the reverse is true, or if
            // they are the same time, then this is an error.
            if a <= b {
                bail!("'after' must be chronologically later than 'before'");
            }
        }

        let fields = match params.get("fields") {
            None => HashSet::new(),
            Some(strval) => parse_csl("fields", strval)?,
        };

        let expansions = match params.get("expansions") {
            None => HashSet::new(),
            Some(strval) => parse_csl("expansions", strval)?,
        };

        let tags = match params.get("tags") {
            None => HashSet::new(),
            Some(strval) => parse_csl("tags", strval)?,
        };

        Ok(PostQueryParams {
            depth,
            limit,
            sort,
            before,
            after,
            fields,
            expansions,
            tags,
            ..Default::default()
        })
    }
}