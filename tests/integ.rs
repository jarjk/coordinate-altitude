use coordinate_altitude::Coord;

fn main() {
    let my_coord = MyWayOfStoringCoordinates { x: 8, y: 43, z: 0 };
    let mut coord_altitude: Coord = my_coord.into();
    coord_altitude.add_altitude().unwrap();
    println!(
        "altitude of ({};{}){}",
        coord_altitude.longitude, coord_altitude.latitude, coord_altitude.altitude
    );
}

struct MyWayOfStoringCoordinates {
    x: u8,
    y: u8,
    z: u8,
}

impl From<MyWayOfStoringCoordinates> for Coord {
    fn from(value: MyWayOfStoringCoordinates) -> Self {
        Self::new(value.y, value.x).with_altitude(value.z)
    }
}
