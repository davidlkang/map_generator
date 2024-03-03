extern crate noise;

pub mod height_map {
    use noise::{NoiseFn, Perlin, Seedable};

    use crate::MapParams;

    pub type HeightMap = Vec<Vec<f64>>;

    pub fn generate_height_map(map_params: &MapParams) -> HeightMap {
        let width = map_params.width;
        let height = map_params.height;
        let seed = map_params.seed;

        let perlin = Perlin::new().set_seed(seed);
        let mut map = vec![vec![0.0; width]; height];

        for y in 0..height {
            for x in 0..width {
                let nx: f64 = x as f64;
                let ny: f64 = y as f64;
                let noise_value: f64 = adjusted_layered_noise(nx, ny, &perlin);
                map[y][x] = noise_value;  // Directly use the adjusted noise value for more gradient
            }
        }

        map
    }

    // Introduce smoothstep function for smoother transitions
    fn smoothstep(edge0: f64, edge1: f64, x: f64) -> f64 {
        let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
        t * t * (3.0 - 2.0 * t)
    }

    // Adjusted layered noise function with non-linear blending and more variation
    fn adjusted_layered_noise(x: f64, y: f64, perlin: &Perlin) -> f64 {
        let large_scale: f64 = perlin.get([x * 0.001, y * 0.001]) * 5.5;
        let medium_scale: f64 = perlin.get([x * 0.01, y * 0.01]) * 0.6;
        let fine_scale: f64 = perlin.get([x * 0.05, y * 0.05]) * 0.35;
        let detail: f64 = perlin.get([x * 0.1, y * 0.1]) * 0.1;

        let combined: f64 = large_scale + medium_scale + smoothstep(0.2, 0.8, fine_scale) + detail;
        (combined + 1.0) / 2.0  // Normalize to 0..1
    }
}
