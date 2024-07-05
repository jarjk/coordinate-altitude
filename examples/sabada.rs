use coordinate_altitude::{fetch_altitude, Coord, Res};

fn main() {
    // coordinate as a tuple
    let coord: (f64, f64) = (34.324, 1.88832);
    // coordinate as a `Coord`
    let coord: Coord = coord.into();
    // and finally fetch altitude for `coord`
    let coord = coord.fetch_altitude();
    println!("coordinate: {coord:?}");

    let mut kuzminec = Coord::new(46.1650119, 15.9725117);
    // NOTE: it will become: `Coord::new(46.165012, 15.972512)`
    // NOTE: every latitude, longitude data will be rounded to 6 decimal places accuracy by https://open-elevation.com api
    println!("kuzminec, no elevation: {:#?}", kuzminec);
    kuzminec.add_altitude().unwrap();
    println!("kuzminec, w/ elevation: {:#?}", kuzminec);

    let jysk_vp = Coord::new(47.086038, 17.925097).fetch_altitude();
    println!("jysk_vp: {jysk_vp:?}");

    let coords: Vec<Coord> = vec![
        (58.2926289, 134.3025286).into(),   // Sheep Mountain
        (7.4894883, 80.8144869).into(),     // Sri Lanka
        Coord::new(47.0745464, 12.6938825), // Großglockner
    ];
    let coords: Res<Vec<Coord>> = fetch_altitude(&coords);
    println!("coordinates: {coords:#?}");
}
