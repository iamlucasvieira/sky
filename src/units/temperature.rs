//! # Temperature
//!
//! This module provides temperature conversions.

use crate::units::Unit;

/// Trait for temperature conversions
///
/// Every temperature must implement a convertion to Kelvin and from Kelvin.
/// This allows to convert between different temperature scales.
pub trait Convertible {
    /// Method to covert from the temperature to Kelvin
    fn to_kelvin(&self) -> Kelvin;
    /// Method to convert from Kelvin to the temperature
    fn from_kelvin(k: Kelvin) -> Self;

    /// Method to convert to a different temperature scale
    fn to<T: Convertible>(&self) -> T {
        T::from_kelvin(self.to_kelvin())
    }
}

/// Celsius temperature scale
pub struct Celsius(f64);

/// Implement the Convertible trait for Celsius
impl Convertible for Celsius {
    fn to_kelvin(&self) -> Kelvin {
        Kelvin(self.0 + 273.15)
    }
    fn from_kelvin(k: Kelvin) -> Celsius {
        Celsius(k.0 - 273.15)
    }
}

/// Implement the Unit trait for Celsius
impl Unit for Celsius {
    fn symbol(&self) -> &'static str {
        "°C"
    }
    fn value(&self) -> f64 {
        self.0
    }
}

/// Fahrenheit temperature scale
pub struct Fahrenheit(f64);

/// Implement the Convertible trait for Fahrenheit
impl Convertible for Fahrenheit {
    fn to_kelvin(&self) -> Kelvin {
        Kelvin((self.0 - 32.0) * 5.0 / 9.0 + 273.15)
    }
    fn from_kelvin(k: Kelvin) -> Fahrenheit {
        Fahrenheit((k.0 - 273.15) * 9.0 / 5.0 + 32.0)
    }
}

/// Implement the Unit trait for Fahrenheit
impl Unit for Fahrenheit {
    fn symbol(&self) -> &'static str {
        "°F"
    }
    fn value(&self) -> f64 {
        self.0
    }
}

/// Kelvin temperature scale
#[derive(Clone, Copy)]
pub struct Kelvin(f64);

/// Implement the Unit trait for Kelvin
impl Unit for Kelvin {
    fn symbol(&self) -> &'static str {
        "K"
    }
    fn value(&self) -> f64 {
        self.0
    }
}

/// Implement the Convertible trait for Kelvin
impl Convertible for Kelvin {
    fn to_kelvin(&self) -> Kelvin {
        *self
    }
    fn from_kelvin(k: Kelvin) -> Kelvin {
        k
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celsius() {
        let c = Celsius(0.0);
        assert_eq!(c.value(), 0.0);
        assert_eq!(c.symbol(), "°C");
        assert_eq!(c.to_kelvin().value(), 273.15);
    }

    #[test]
    fn test_fahrenheit() {
        let f = Fahrenheit(32.0);
        assert_eq!(f.value(), 32.0);
        assert_eq!(f.symbol(), "°F");
        assert_eq!(f.to_kelvin().value(), 273.15);
    }

    #[test]
    fn test_kelvin() {
        let k = Kelvin(273.15);
        assert_eq!(k.value(), 273.15);
        assert_eq!(k.symbol(), "K");
    }

    #[test]
    fn test_convert() {
        let c = Celsius(0.0);
        let f = c.to::<Fahrenheit>();
        assert_eq!(f.value(), 32.0);
        assert_eq!(f.symbol(), "°F");
    }
}
