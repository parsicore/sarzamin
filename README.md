# Sarzamin (سرزمین)

[![crates.io](https://img.shields.io/crates/v/sarzamin.svg)](https://crates.io/crates/sarzamin)
[![docs.rs (with version)](https://img.shields.io/docsrs/sarzamin/latest)](https://docs.rs/sarzamin/latest/sarzamin/)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/sarzamin)](https://crates.io/crates/sarzamin)
[![license](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](./LICENSE)
[![Tests](https://github.com/parsicore/sarzamin/actions/workflows/Tests.yml/badge.svg)](https://github.com/parsicore/sarzamin/actions/workflows/Tests.yml)
[![Lint](https://github.com/parsicore/sarzamin/actions/workflows/lint.yml/badge.svg)](https://github.com/parsicore/sarzamin/actions/workflows/lint.yml)
![Maintenance](https://img.shields.io/badge/maintained-actively-green)

**A comprehensive, fast, and ergonomic Rust library for Iran's geographical data.**

`Sarzamin` (سرزمین) is the Farsi word for "The Land" or "Homeland". This library provides a complete and easy-to-use API to access Iran's geographical divisions, including provinces, counties, districts, cities, and rural districts.

It is part of the **Parsicore** organization's mission to enrich the Rust ecosystem for Persian-speaking developers by providing high-quality, essential libraries for common local needs.

## ✨ Features

*   **Comprehensive Data:** Includes all official divisions of Iran:
    *   Provinces (استان‌ها)
    *   Counties (شهرستان‌ها)
    *   Districts (بخش‌ها)
    *   Cities (شهرها)
    *   Rural Districts (دهستان‌ها)
*   **Zero Runtime Dependencies:** All geographical data is embedded directly into your binary at compile-time using `include_str!`. No need to manage or distribute external JSON files.
*   **Lazy-Loaded & Performant:** Data is parsed from the embedded JSON only once on the first access, thanks to `once_cell`. Subsequent calls are virtually zero-cost.
*   **Ergonomic API:** Provides simple, idiomatic Rust functions to query the data (e.g., `get_cities_of_province`, `find_province_by_city_name`).
*   **Type-Safe:** Utilizes Rust's strong type system to represent geographical entities, preventing common errors.
*   **Built with Serde:** All data models derive `serde::Deserialize` for robust and efficient JSON parsing.

## 🚀 Installation

Add `sarzamin` to your `Cargo.toml` file:

```toml
[dependencies]
sarzamin = "0.1.0" # Replace with the latest version
```

Or use the command line:

```bash
cargo add sarzamin
```

## ⚡ Quick Start

Here's a quick example of how to find the province of a city:

```rust
use sarzamin::{find_province_by_city_name, get_cities_of_province};

fn main() {
    // Find the province for the city "Kerman"
    if let Some(province) = find_province_by_city_name("کرمان") {
        println!("'کرمان' is in '{}' province.", province.name);
        
        // Now, let's get all cities in that province
        let kerman_cities = get_cities_of_province(province.id);
        println!("Found {} cities in Kerman province.", kerman_cities.len());
        // println!("Cities: {:?}", kerman_cities);
    } else {
        println!("City not found!");
    }
}
```

## 📖 API Overview & Examples

### Provinces (استان‌ها)

You can get a list of all provinces or find a specific one by its ID.

```rust
use sarzamin::{get_all_provinces, find_province_by_id};

// Get a slice containing all 31 provinces
let all_provinces = get_all_provinces();
assert_eq!(all_provinces.len(), 31);

// Find the province of Tehran (ID 123)
if let Some(tehran) = find_province_by_id(123) {
    println!("Tehran's telephone prefix is: {}", tehran.tel_prefix);
}
```

### Counties (شهرستان‌ها)

Get all counties or filter them by province.

```rust
use sarzamin::get_counties_of_province;

// Get all counties in Fars province (ID 107)
let fars_counties = get_counties_of_province(107);
println!("Fars province has {} counties.", fars_counties.len());

// Find the county of "Shiraz" within Fars
if let Some(shiraz_county) = fars_counties.iter().find(|c| c.name == "شیراز") {
    println!("Found Shiraz county with ID: {}", shiraz_county.id);
}
```

### Cities (شهرها)

Find a city's details or get all cities within a province or county.

```rust
use sarzamin::{get_cities_of_province, find_province_by_city_name};

// Get all cities in Yazd province (ID 121)
let yazd_cities = get_cities_of_province(121);
assert!(yazd_cities.iter().any(|c| c.name == "یزد"));

// Find which province a city belongs to
let esfahan_province = find_province_by_city_name("اصفهان").unwrap();
assert_eq!(esfahan_province.name, "اصفهان");
```

*(API for Districts and Rural Districts follows a similar pattern)*

## 🏗️ Data Models

The library exposes the following data structures, which directly map to the geographical data:

*   `Province`
*   `County`
*   `City`
*   `District`
*   `RuralDistrict`

Here is an example of the `Province` struct:
```rust
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Province {
    pub id: u32,
    pub name: String,
    pub slug: String,
    pub tel_prefix: String,
}
```
Check the documentation or the `src/models.rs` file for the full details of each model.

## 💡 Design & Performance Philosophy

`sarzamin` is designed to be both easy to maintain and highly performant for the end-user.

1.  **Maintainability**: The source data is kept in simple JSON files. This makes it incredibly easy to update the data as official divisions change, without needing to modify Rust code.
2.  **Performance & Distribution**:
    *   At compile time, `include_str!` embeds the entire content of the JSON files into the final binary. This means the library has **no external file dependencies at runtime**. Users just add the crate, and it works.
    *   The `once_cell::sync::Lazy` static variables ensure that the JSON parsing logic is executed **only once** per data type, the very first time it's accessed. The resulting `Vec<T>` is then cached for the lifetime of the program, making all subsequent calls to functions like `get_all_provinces()` extremely fast.


## 📄 License

**Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

```
Version: 1.0.0
Sign: sarzamin-20250611-bb54068aea85-106fe4c17097e8285daf4519f5be7327
```
