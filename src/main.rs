mod map;
mod system;

fn main() {
    let v: Vec<map::Coord> = map::CoordRings::new().take(20).collect();
    let m: map::Map = map::Map::from_map_string(
        "42 39 49 30 25 33 36 32 48 20 45 37 43 22 50 41 44 31 0 21 24 0 27 38 0 29 26 14 34 47 0 35 23 0 19 40"
    );
    print!("{:?}", m);
}
