extern crate noise;
extern crate image;

pub mod combined_map;
pub mod height_map;
pub mod temp_map;
pub mod water_map;

use combined_map::Map;

pub struct MapParams {
    width: usize,
    height: usize,
    seed: u32,
    river_elevation_threshold: f64,
    slope_threshold: f64,
    lake_formation_threshold: usize,
    initial_volume: usize,
}

fn main() {
    let map_params = MapParams {
        width: 1000,
        height: 1000,
        seed: 43622,
        river_elevation_threshold: 0.9,
        slope_threshold: 0.026,
        lake_formation_threshold: 1,
        initial_volume: 10,
    };

    // seed: (11134, 141421, 43622())

    let map: Map = Map::new(map_params);

    map.generate_water_map_image();
    map.generate_height_map_image();
    map.generate_temperature_map_image();
}
