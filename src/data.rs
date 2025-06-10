// ~/src/data.rs
//
//  * Copyright ParsiCore 2024-2025 <jalalvandi.sina@gmail.com>
//  * Package : sarzamin
//  * License : Apache-2.0
//  * Version : 1.0.0
//  * URL     : https://github.com/parsicore/sarzamin
//  * Sign: sarzamin-20250611-bb54068aea85-106fe4c17097e8285daf4519f5be7327
//
// This file contains the data loading logic for the Parsidate library.

use crate::models::{City, County, /* District*/ Province /*RuralDistrict*/};
use once_cell::sync::Lazy;
use serde::de::DeserializeOwned;

// A generic function to load and parse our JSON data.
// It takes the JSON string, includes it at compile time, and parses it.
fn load_json<T: DeserializeOwned>(json_str: &'static str) -> Vec<T> {
    serde_json::from_str(json_str).expect("Failed to parse JSON data")
}

// Use `include_str!` to embed the JSON file content directly into the binary.
// The path is relative to the current source file (data.rs).
// You need to place your JSON files in a `data` directory inside `src`.
static PROVINCES_JSON: &str = include_str!("data/provinces.json");
static COUNTIES_JSON: &str = include_str!("data/counties.json");
static CITIES_JSON: &str = include_str!("data/cities.json");
/*static DISTRICTS_JSON: &str = include_str!("data/districts.json");
static RURALS_JSON: &str = include_str!("data/rurals.json");*/

// Use `Lazy` from `once_cell` to ensure that parsing happens only once,
// the first time each dataset is accessed. The result is then cached.
pub static PROVINCES: Lazy<Vec<Province>> = Lazy::new(|| load_json(PROVINCES_JSON));
pub static COUNTIES: Lazy<Vec<County>> = Lazy::new(|| load_json(COUNTIES_JSON));
pub static CITIES: Lazy<Vec<City>> = Lazy::new(|| load_json(CITIES_JSON));
/*pub static DISTRICTS: Lazy<Vec<District>> = Lazy::new(|| load_json(DISTRICTS_JSON));
pub static RURAL_DISTRICTS: Lazy<Vec<RuralDistrict>> = Lazy::new(|| load_json(RURALS_JSON));*/
