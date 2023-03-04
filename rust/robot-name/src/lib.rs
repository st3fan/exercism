#[macro_use]
extern crate lazy_static;

use std::{collections::HashSet, sync::Mutex};

lazy_static! {
    static ref ALLOCATED_NAMES: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

fn generate_robot_name() -> String {
    let n = fastrand::u16(0..=999);
    format!("{}{}{:03}", fastrand::uppercase(), fastrand::uppercase(), n)
}

fn random_robot_name() -> String {
    return loop {
        let new_name = generate_robot_name();
        if ALLOCATED_NAMES.lock().unwrap().insert(new_name.clone()) {
            break new_name;
        }
    };
}

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        Self {
            name: random_robot_name(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = random_robot_name();
    }
}
