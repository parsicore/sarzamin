// ~/tests/geo_tests.rs
//
//  * Copyright ParsiCore 2024-2025 <parsicore.dev@gmail.com>
//  * Package : sarzamin
//  * License : Apache-2.0
//  * Version : 1.0.0
//  * URL     : https://github.com/parsicore/sarzamin
//  * Sign: sarzamin-20250611-bb54068aea85-106fe4c17097e8285daf4519f5be7327
//
// Tests for the sarzamin library
#[allow(unused_imports)]
use sarzamin::{self, Province};

#[test]
fn test_provinces_are_loaded() {
    let provinces = sarzamin::get_all_provinces();
    // Assuming your provinces.json has 31 entries
    assert_eq!(provinces.len(), 31);
}

#[test]
fn test_find_tehran_province() {
    let tehran = sarzamin::get_all_provinces()
        .iter()
        .find(|p| p.name == "تهران");
    assert!(tehran.is_some());
    assert_eq!(tehran.unwrap().id, 123);
}

#[test]
fn test_find_province_of_shiraz_city() {
    let province = sarzamin::find_province_by_city_name("شیراز");
    assert!(province.is_some());
    let province = province.unwrap();
    assert_eq!(province.name, "فارس");
    assert_eq!(province.id, 107);
}

#[test]
fn test_get_cities_of_fars_province() {
    // Fars province has id 107
    let fars_cities = sarzamin::get_cities_of_province(107);
    assert!(!fars_cities.is_empty());
    // Check if Shiraz is in the list
    assert!(fars_cities.iter().any(|c| c.name == "شیراز"));
}
