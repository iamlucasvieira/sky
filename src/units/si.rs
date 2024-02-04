//! # SI Units
//!
//! This module contains the units of measure used in the library.

/// Trait for units with a value and a symbol
pub trait Unit {
    fn value(&self) -> f64;
    fn symbol(&self) -> &'static str;
}
