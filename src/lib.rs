/// a coordinate
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct Coord {
    /// y
    pub latitude: f64,
    /// x
    pub longitude: f64,
    /// elevation above sea-level
    pub altitude: f64,
}

impl From<(f32, f32)> for Coord {
    fn from(val: (f32, f32)) -> Self {
        Coord::new(val.0, val.1)
    }
}

impl Coord {
    pub fn new<F1: Into<f64>, F2: Into<f64>>(latitude: F1, longitude: F2) -> Self {
        Self {
            latitude: latitude.into(),
            longitude: longitude.into(),
            altitude: 0.,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
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
    fn default() {
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
}
