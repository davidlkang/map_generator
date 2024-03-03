use image::{ImageBuffer, Luma, Rgb};

use crate::height_map::height_map::{generate_height_map, HeightMap};
use crate::temp_map::temp_map::{generate_temperate_temperature_map, TemperatureMap};
use crate::water_map::water_map::{generate_water_map, WaterMap};
use crate::MapParams;

pub struct Map {
    width: usize,
    height: usize,

    height_map: HeightMap,
    temp_map: TemperatureMap,
    water_map: Vec<Vec<bool>>,
    //humid: Vec<Vec<f64>>,

    //combined: Vec<Vec<f64>>,
}

impl Map {
    pub fn new(map_params: MapParams) -> Self {
        let width = map_params.width;
        let height = map_params.height;

        let height_map:HeightMap = generate_height_map(&map_params);
        let temp_map: TemperatureMap = generate_temperate_temperature_map(&map_params, &height_map);
        let water_map: WaterMap = generate_water_map(&map_params, &height_map);

        Self {
            width,
            height,
            height_map,
            temp_map,
            water_map,
        }
    }
}

impl Map {

    // Function to map temperature values to colors with a finer granularity
    fn temperature_to_color(&self, temp: f64) -> Rgb<u8> {
        match temp {
            t if t <= 2.0 => Rgb([0, 0, 128]),   // Cold (Dark Blue)
            t if t <= 4.0 => Rgb([0, 0, 255]),   // Chilly (Blue)
            t if t <= 6.0 => Rgb([64, 104, 255]), // Cooler (Light Blue)
            t if t <= 8.0 => Rgb([116, 208, 241]), // Coldish (Sky Blue)
            t if t <= 10.0 => Rgb([178, 255, 255]), // Cool (Cyan)
            t if t <= 12.0 => Rgb([144, 238, 144]), // Slightly Cool (Light Green)
            t if t <= 14.0 => Rgb([0, 255, 0]),   // Temperate (Green)
            t if t <= 16.0 => Rgb([255, 255, 100]), // Warmish (Light Yellow)
            t if t <= 18.0 => Rgb([255, 223, 0]),   // Warm (Yellow)
            t if t <= 20.0 => Rgb([255, 170, 0]),   // Warmer (Orange)
            t if t <= 22.0 => Rgb([255, 113, 0]),   // Toasty (Dark Orange)
            t if t <= 24.0 => Rgb([255, 56, 0]),    // Hot (Red-Orange)
            t if t <= 26.0 => Rgb([255, 0, 0]),     // Hotter (Red)
            t if t <= 28.0 => Rgb([200, 0, 0]),     // Very Hot (Dark Red)
            _ => Rgb([155, 0, 0]),                  // Scorching (Darker Red)
        }
    }

    pub fn generate_height_map_image(&self) {
        let imgx = self.width as u32;
        let imgy = self.height as u32;
        let mut imgbuf = ImageBuffer::new(imgx, imgy);

        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let val = (self.height_map[y as usize][x as usize] * 255.0) as u8;
            *pixel = Luma([val]);
        }

        imgbuf.save("target_maps/height_map.png").unwrap();
    }

    // Assuming you have your temperature map ready
    pub fn generate_temperature_map_image(&self) {
        let imgx = self.width as u32;
        let imgy = self.height as u32;
        let mut imgbuf = ImageBuffer::new(imgx, imgy);

        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let temp = self.temp_map[y as usize][x as usize];
            *pixel = self.temperature_to_color(temp);
        }

        imgbuf.save("target_maps/temperature_map.png").unwrap();
    }

    // Function to generate an image from the water map
    pub fn generate_water_map_image(&self) {
        let imgx = self.width as u32;
        let imgy = self.height as u32;
        let mut imgbuf: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(imgx, imgy);

        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            if self.water_map[y as usize][x as usize] {
                *pixel = Rgb([0, 0, 255]); // Blue for water
            } else {
                *pixel = Rgb([34, 139, 34]); // Green for land
            }
        }

        imgbuf.save("target_maps/water_map.png").unwrap();
    }
}
