use serde::{Deserialize, Serialize};

/// a coordinate
#[derive(Clone, Copy, PartialEq, Default, Debug, Serialize, Deserialize)]
pub struct Coord {
    /// y
    pub latitude: f64,
    /// x
    pub longitude: f64,
    /// elevation above sea-level
    #[serde(alias = "elevation", skip_serializing)]
    pub altitude: f64,
}
impl Coord {
    pub fn new<F1: Into<f64>, F2: Into<f64>>(latitude: F1, longitude: F2) -> Self {
        Self {
            latitude: latitude.into(),
            longitude: longitude.into(),
            altitude: 0.,
        }
    }
    pub fn with_altitude<F: Into<f64>>(&self, altitude: F) -> Self {
        Self {
            altitude: altitude.into(),
            ..*self
        }
    }
}

#[derive(Clone, PartialEq, Default, Debug, Serialize, Deserialize)]
struct Locations {
    locations: Vec<Coord>,
}
impl From<Locations> for Vec<Coord> {
    fn from(locations: Locations) -> Self {
        locations.locations
    }
}
impl From<Vec<Coord>> for Locations {
    fn from(locations: Vec<Coord>) -> Self {
        Self { locations }
    }
}

impl From<(f32, f32)> for Coord {
    fn from(val: (f32, f32)) -> Self {
        Coord::new(val.0, val.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coord_new() {
        let result = Coord::new(44., 2);
        assert_eq!(
            result,
            Coord {
                latitude: 44.,
                longitude: 2.,
                altitude: 0.
            }
        );
    }
    #[test]
    fn coord_default() {
        let result = Coord::default();
        assert_eq!(
            result,
            Coord {
                latitude: 0.,
                longitude: 0.,
                altitude: 0.
            }
        )
    }

    #[test]
    fn coord_deser() {
        let json = r#"{
            "latitude": 32.2643,
            "longitude": 20.333,
            "elevation": 354
        }"#;
        let result = serde_json::from_str::<Coord>(json).unwrap();
        assert_eq!(result, Coord::new(32.2643, 20.333).with_altitude(354));
    }
    #[test]
    fn coord_ser() {
        let coord = Coord::new(32.2643, 20.333).with_altitude(354);
        let result = serde_json::to_string(&coord).unwrap();
        // note that altitude isn't serialized
        let json = r#"{"latitude":32.2643,"longitude":20.333}"#;
        assert_eq!(result, json);
    }
}
