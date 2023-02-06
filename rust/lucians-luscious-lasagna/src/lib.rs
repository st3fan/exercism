// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use core::num;

const EXPECTED_TIME_IN_OVEN: i32 = 40;
const PREP_TIME_PER_LAYER: i32 = 2;

pub fn expected_minutes_in_oven() -> i32 {
    EXPECTED_TIME_IN_OVEN
}

pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
    return EXPECTED_TIME_IN_OVEN - actual_minutes_in_oven;
}

pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
    number_of_layers * PREP_TIME_PER_LAYER
}

pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
    actual_minutes_in_oven + preparation_time_in_minutes(number_of_layers)
}
