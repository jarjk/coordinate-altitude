use coordinate_altitude::Coord;

fn quit_help() -> ! {
    eprintln!("usage: coordinate-altitude <COORDINATE>\n<COORDINATE>: <LATITUDE> <LONGITUDE> || \"<LATITUDE>,<LONGITUDE>\"");
    std::process::exit(1);
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let (lat, lon) = if args.len() == 2 {
        let mut x = args[1].split(',');
        (
            x.next().unwrap_or_else(|| quit_help()),
            x.next().unwrap_or_else(|| quit_help()),
        )
    } else if args.len() == 3 {
        (args[1].as_str(), args[2].as_str())
    } else {
        quit_help()
    };
    let coord: (f64, f64) = (
        lat.parse().unwrap_or_else(|_| quit_help()),
        lon.parse().unwrap_or_else(|_| quit_help()),
    );
    let mut coord: Coord = coord.into();
    coord.add_altitude().unwrap_or_else(|_| quit_help());

    println!(
        "altitude for ({};{}) is {}m",
        coord.latitude, coord.longitude, coord.altitude
    );
}
