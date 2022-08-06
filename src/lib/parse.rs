use super::*;
use chrono::NaiveDate;
use serde_json::{json, Map, Value};

/// Utilities for handling parsing and converting collected rows and columns from csv data

pub struct ParseService();

impl ParseService {
    pub fn parse_row(def: &Vec<ColumnDef>, row: Vec<String>) -> Value {
        let mut ob = Map::<String, Value>::new();
        for (i, n) in row.into_iter().enumerate() {
            // todo!
            let def = def.get(i).unwrap();
            ob.insert(def.col_name.clone(), Self::parse_value(def, &n));
        }
        json!(ob)
    }

    pub fn parse_value(def: &ColumnDef, value: &str) -> Value {
        if let ColType::Integer = def.col_type {
            let num = Self::parse_i32(value);
            if let Err(err) = num {
                info!("{} using default value...", err);
                return def.col_default.clone();
            } else {
                return json!(num.unwrap());
            }
        }

        if let ColType::Float = def.col_type {
            let num = Self::parse_f64(value);
            if let Err(err) = num {
                info!("{} using default value...", err);
                return def.col_default.clone();
            } else {
                return json!(num.unwrap());
            }
        }

        if let ColType::Bool = def.col_type {
            if value == def.col_true_string {
                return json!(true);
            } else if value == def.col_false_string {
                return json!(false);
            } else {
                return def.col_default.clone();
            }
        }

        if let ColType::Date = def.col_type {
            let date = Self::parse_date(value, &def.col_format);
            if let Err(err) = date {
                info!("{} continue with default", err);
                return def.col_default.clone();
            }
            return json!(date.unwrap());
        }

        json!(value)
    }

    pub fn parse_i32(n: &str) -> Result<i32> {
        let n = sanitise_number(n);
        n.parse::<i32>().map_err(|_| get_sanitise_err(&n, "i32"))
    }

    pub fn parse_f64(n: &str) -> Result<f64> {
        let n = sanitise_number(n);
        n.parse::<f64>().map_err(|_| get_sanitise_err(&n, "f64"))
    }

    pub fn parse_date(n: &str, format: &str) -> Result<i64> {
        let date = NaiveDate::parse_from_str(n, format);

        if let Err(err) = date.as_ref() {
            return Err(WhoopsBuilder::new()
                .err_type("parse-date-err")
                .context("whilst trying to parse date")
                .why(&format!(
                    "{} is not parsable with format {}: {}",
                    n, format, err
                ))
                .suggestion(&format!("format {} with given format: {}", n, format))
                .build());
        }

        let date = date.unwrap();

        let date = date.and_hms(0, 0, 0).timestamp();
        Ok(date)
    }
}

fn sanitise_number(n: &str) -> String {
    let mut new_str = String::new();
    new_str.reserve(n.len());
    for (i, c) in n.chars().enumerate() {
        if c.is_digit(10) || c == '.' {
            new_str.push(c);
        }
        if c == '-' && i == 0 {
            new_str.push(c);
        }
    }
    new_str
}

fn get_sanitise_err(input: &str, intended_type: &str) -> Whoops {
    Whoops {
        err_type: format!("sanitise-{}-error", intended_type),
        context: format!("whilst trying to sanitise {} str", intended_type),
        why: format!("{} is not parsable as a {}", input, intended_type),
        suggestion: format!(
            "Modify {}, so that it is parsable as a {}",
            input, intended_type
        ),
    }
}
