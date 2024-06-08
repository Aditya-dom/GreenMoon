/* This constructs its own World and can run
individual systems against that world. It provides
convenience methods to get entities and read
components. */

use specs::prelude::*;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct Name {
    pub string: String,
}

impl Name {
    pub fn new(s: &str) -> Self {
        Self { string: s.to_string() }
    }
}