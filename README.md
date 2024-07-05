# coordinate-altitude

fetch altitude/elevation data for a coordinate of planet Earth

## example

after `cargo add coordinate_altitude`, do

```rust
use coordinate_altitude::*;

fn main() {
    // coordinate as a tuple
    let coord: (f64, f64) = (34.324, 1.88832);
    // coordinate as a `Coord`
    let coord: Coord = coord.into();
    // and finally fetch altitude for `coord`
    let coord: Option<Coord> = coord.fetch_altitude();
    println!("coordinate: {coord:?}");
}
```

see [examples](./examples) for more

## warning!

_open-elevation is only 6 decimal places accurate_
eg.: `32.324325435` will become `32.324325`

## dependencies

-   json (de)serialization: [serde](https://serde.rs), [serde_json](https://crates.io/crates/serde_json)
-   GET/POST requests: [ureq](https://crates.io/crates/ureq)
-   Elevation data source: [open-elevation](https://open-elevation.com/)
