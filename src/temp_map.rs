extern crate noise;
extern crate image;

pub mod temp_map {
    use crate::{height_map::height_map::HeightMap, MapParams};

    pub type TemperatureMap = Vec<Vec<f64>>;

    pub fn generate_temperate_temperature_map(map_params: &MapParams, height_map: &HeightMap) -> TemperatureMap {
        let width = map_params.width;
        let height = map_params.height;

        let mut temp_map: TemperatureMap = vec![vec![0.0; width]; height];
        // Simulating a northern temperate zone, adjust base_temp as needed
        let base_temp = 10.0; // Average temperature in a temperate zone
        let temp_variation = 5.0; // Variation due to altitude and other factors

        for y in 0..height {
            for x in 0..width {
                let altitude_effect = (1.0 - height_map[y][x]) * temp_variation; // Higher altitude = cooler
                temp_map[y][x] = base_temp - altitude_effect;
            }
        }

        temp_map
    }
}
