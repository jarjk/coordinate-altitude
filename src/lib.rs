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
        let latitude: f64 = latitude.into();
        let longitude: f64 = longitude.into();

        assert!(latitude <= 90. && longitude <= 90.);

        Self {
            latitude,
            longitude,
            altitude: 0.,
        }
    }
    pub fn with_altitude<F: Into<f64>>(&self, altitude: F) -> Self {
        Self {
            altitude: altitude.into(),
            ..*self
        }
    }

    fn get_form(&self) -> String {
        format!("{},{}", self.latitude, self.longitude)
    }

    pub fn fetch_altitude(&self) -> Self {
        *fetch_altitude(&[*self]).first().unwrap()
    }
}

pub fn fetch_altitude(coords: &[Coord]) -> Vec<Coord> {
    let resp = if let Some(got_resp) = fetch_altitude_get(coords) {
        got_resp
    } else {
        fetch_altitude_post(coords)
    };
    // leading: ""results": {"
    let resp = &resp[11..];
    // trailing: "}"
    let resp = &resp[0..resp.len() - 1];

    serde_json::from_str(resp)
        .inspect_err(|e| eprintln!("parse error: {e:#?}"))
        .unwrap()
}

fn fetch_altitude_get(coords: &[Coord]) -> Option<String> {
    let mut form = coords
        .iter()
        .fold(String::new(), |sum, cnt| sum + &cnt.get_form() + "|");
    // trailing |
    form.pop();
    if form.as_bytes().len() < 1024 {
        return None;
    }
    // eprintln!("sending: {form:?}");
    ureq::get("https://api.open-elevation.com/api/v1/lookup")
        .query("locations", &form)
        .call()
        .inspect_err(|e| eprintln!("fetch error: {e:#?}"))
        .unwrap()
        .into_string()
        .ok()
}

fn fetch_altitude_post(coords: &[Coord]) -> String {
    let data = serde_json::to_string(coords)
        .inspect_err(|e| eprintln!("serialization error: {e:#?}"))
        .unwrap();
    let data = format!("{{\"locations\":{data}}}");
    // eprintln!("sending: {data:?}");

    ureq::post("https://api.open-elevation.com/api/v1/lookup")
        .set("Accept", "application/json")
        .set("Content-Type", "application/json")
        .send_bytes(data.as_bytes())
        .inspect_err(|e| eprintln!("fetch error: {e:#?}"))
        .unwrap()
        .into_string()
        .unwrap()
}

impl<F1: Into<f64>, F2: Into<f64>> From<(F1, F2)> for Coord {
    fn from(val: (F1, F2)) -> Self {
        Self::new(val.0, val.1)
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

    #[test]
    fn locations_deser() {
        let json = r#"
	[
		{
			"latitude": 10,
			"longitude": -10,
			"altitude": 21
		},
		{
			"latitude":20.3453,
			"longitude": 28,
			"elevation": 32
		},
		{
			"latitude":41.161758,
			"longitude":-8.583933,
			"altitude":3798
		}
	]"#;
        let result = serde_json::from_str::<Vec<Coord>>(json).unwrap();
        assert_eq!(
            result,
            vec![
                Coord::new(10, -10).with_altitude(21),
                Coord::new(20.3453, 28).with_altitude(32),
                Coord::new(41.161758, -8.583933).with_altitude(3798),
            ] // .into()
        );
    }
    #[test]
    fn locations_ser() {
        let json = r#"[{"latitude":10.0,"longitude":-10.0},{"latitude":20.3453,"longitude":28.0},{"latitude":41.161758,"longitude":-8.583933}]"#;
        let locations = vec![
            Coord::new(10, -10).with_altitude(21),
            Coord::new(20.3453, 28).with_altitude(32),
            Coord::new(41.161758, -8.583933),
        ];

        let result = serde_json::to_string(&locations).unwrap();
        assert_eq!(result, json);
    }
}
