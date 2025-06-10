// ~/src/lib.rs
//
//  * Copyright ParsiCore 2024-2025 <jalalvandi.sina@gmail.com>
//  * Package : sarzamin
//  * License : Apache-2.0
//  * Version : 1.0.0
//  * URL     : https://github.com/parsicore/sarzamin
//  * Sign: sarzamin-20250611-bb54068aea85-106fe4c17097e8285daf4519f5be7327
//
// Main entry point for the sarzamin library

//! `sarzamin` is a comprehensive Rust library for accessing geographical data of Iran,
//! including provinces, counties, cities, districts, and rural districts.
//!
//! Data is loaded from JSON files at compile-time, ensuring zero runtime dependencies
//! and high performance after the initial lazy-load.
//!
//! # Examples
//!
//! ```
//! use sarzamin::{get_all_provinces, find_province_by_city_name};
//!
//! // Get all provinces
//! let provinces = get_all_provinces();
//! assert_eq!(provinces.len(), 31); // Based on the full dataset
//! println!("Province of Tehran: {:?}", provinces.iter().find(|p| p.name == "تهران"));
//!
//! // Find the province of a city
//! if let Some(province) = find_province_by_city_name("شیراز") {
//!     assert_eq!(province.name, "فارس");
//!     println!("'شیراز' is in '{}' province.", province.name);
//! }
//! ```

mod data;
pub mod models;

// Re-export all models for easy access
pub use models::{City, County, District, Province, RuralDistrict};

// --- Province Functions ---

/// Returns a slice of all provinces.
///
/// Data is lazily-loaded and cached on the first call.
pub fn get_all_provinces() -> &'static [Province] {
    &data::PROVINCES
}

/// Finds a province by its ID.
pub fn find_province_by_id(id: u32) -> Option<&'static Province> {
    data::PROVINCES.iter().find(|p| p.id == id)
}

// --- County Functions ---

/// Returns a slice of all counties.
pub fn get_all_counties() -> &'static [County] {
    &data::COUNTIES
}

/// Returns a vector of counties belonging to a specific province.
pub fn get_counties_of_province(province_id: u32) -> Vec<&'static County> {
    data::COUNTIES
        .iter()
        .filter(|c| c.province_id == province_id)
        .collect()
}

// --- City Functions ---

/// Returns a slice of all cities.
pub fn get_all_cities() -> &'static [City] {
    &data::CITIES
}

/// Returns a vector of cities belonging to a specific province.
pub fn get_cities_of_province(province_id: u32) -> Vec<&'static City> {
    data::CITIES
        .iter()
        .filter(|city| city.province_id == province_id)
        .collect()
}

/// Finds the province of a city by its name.
/// This function performs a case-sensitive search.
pub fn find_province_by_city_name(city_name: &str) -> Option<&'static Province> {
    let city = data::CITIES.iter().find(|c| c.name == city_name)?;
    find_province_by_id(city.province_id)
}
