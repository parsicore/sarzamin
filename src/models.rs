// ~/src/modeles.rs
//
//  * Copyright ParsiCore 2024-2025 <parsicore.dev@gmail.com>
//  * Package : sarzamin
//  * License : Apache-2.0
//  * Version : 1.0.0
//  * URL     : https://github.com/parsicore/sarzamin
//  * Sign: sarzamin-20250611-bb54068aea85-106fe4c17097e8285daf4519f5be7327
//
// This module defines the data models for the sarzamin library.

// Add this line to use serde's Deserialize feature
use serde::Deserialize;

// Tell serde how to map JSON fields to struct fields.
// The `rename_all = "snake_case"` is useful for fields like `province_id`.
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct Province {
    pub id: u32,
    pub name: String, // Use String because the data is now owned
    pub slug: String,
    pub tel_prefix: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct County {
    pub id: u32,
    pub name: String,
    pub slug: String,
    pub province_id: u32,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct City {
    pub id: u64, // The ID is large, so u64 is safer
    pub name: String,
    pub slug: String,
    pub province_id: u32,
    pub county_id: u32,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct District {
    pub id: u32,
    pub name: String,
    pub slug: String,
    pub province_id: u32,
    pub county_id: u32,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct RuralDistrict {
    pub id: u32,
    pub name: String,
    pub slug: String,
    pub province_id: u32,
    pub county_id: u32,
}
