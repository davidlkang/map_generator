extern crate rand;

pub mod water_map {
    use rand::seq::SliceRandom;

    use crate::{height_map::height_map::HeightMap, MapParams};

    pub type WaterMap = Vec<Vec<bool>>;
    type Point = (usize, usize); // Represents a point on the map

    struct RiverPath {
        points: Vec<Point>,  // The path the river takes
        volume: usize,       // The volume of water in the river
    }

    pub fn generate_water_map(map_params: &MapParams, height_map: &HeightMap) -> WaterMap {
        let width = map_params.width;
        let height = map_params.height;
        let river_elevation_threshold: f64 = map_params.river_elevation_threshold;
        let slope_threshold: f64 = map_params.slope_threshold;
        let lake_formation_threshold = map_params.lake_formation_threshold;

        let mut water_map: WaterMap = vec![vec![false; width]; height];

        // Find basins and update the water map
        let basins = find_basins(&height_map);
        for (x, y) in &basins {
            water_map[*y][*x] = true; // Consider expanding this for actual basin size
        }

        // Determine river sources (you need to define how you choose these)
        let start_points = determine_river_sources(&height_map, river_elevation_threshold, slope_threshold);
        // Simulate water flow and update the water map
        let mut river_paths = simulate_water_flow(map_params, start_points, &height_map, &basins);

        for path in &mut river_paths {
            if path.volume > lake_formation_threshold { // Define this threshold based on your simulation's scale
                let lake_center = path.points.last().unwrap(); // Assuming lakes form at the end of paths
                // Expand around the lake_center to form a lake, adjusting the size based on volume
                form_lake(&mut water_map, lake_center, path.volume);
            }
        }

        water_map
    }

    fn find_basins(height_map: &HeightMap) -> Vec<(usize, usize)> {
        let mut basins = Vec::new();
        let width = height_map[0].len();
        let height = height_map.len();

        for y in 1..height-1 {
            for x in 1..width-1 {
                let current_height = height_map[y][x];
                if is_low_point(x, y, current_height, height_map) {
                    basins.push((x, y));
                }
            }
        }

        basins
    }

    fn is_low_point(x: usize, y: usize, current_height: f64, height_map: &HeightMap) -> bool {
        let neighbors = [
            (x - 1, y), (x + 1, y),
            (x, y - 1), (x, y + 1),
            (x - 1, y - 1), (x + 1, y + 1),
            (x - 1, y + 1), (x + 1, y - 1),
        ];

        neighbors.iter().all(|&(nx, ny)| {
            current_height < height_map[ny][nx]
        })
    }

    fn determine_river_sources(height_map: &HeightMap, elevation_threshold: f64, slope_threshold: f64) -> Vec<Point> {
        let mut sources = Vec::new();
        let width = height_map[0].len();
        let height = height_map.len();

        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let current_height = height_map[y][x];
                if current_height > elevation_threshold {
                    let neighbors = [
                        (x.wrapping_sub(1), y),
                        (x + 1, y),
                        (x, y.wrapping_sub(1)),
                        (x, y + 1),
                    ];
                    let mut max_slope = 0.0;
                    for &(nx, ny) in &neighbors {
                        let slope = current_height - height_map[ny][nx];
                        if slope > max_slope {
                            max_slope = slope;
                        }
                    }
                    if max_slope > slope_threshold {
                        sources.push((x, y));
                    }
                }
            }
        }
        sources
    }

    fn _is_high_point(x: usize, y: usize, height_map: &HeightMap) -> bool {
        let current_height = height_map[y][x];
        let neighbors = [
            (x - 1, y), (x + 1, y),
            (x, y - 1), (x, y + 1),
            (x - 1, y - 1), (x + 1, y + 1),
            (x - 1, y + 1), (x + 1, y - 1),
        ];

        neighbors.iter().all(|&(nx, ny)| {
            // Ensure valid indices
            nx < height_map[0].len() && ny < height_map.len() && current_height >= height_map[ny][nx]
        })
    }

    fn _is_basin(point: &Point, basins: &Vec<Point>) -> bool {
        basins.contains(point)
    }

    fn simulate_water_flow(map_params: &MapParams, start_points: Vec<Point>, height_map: &HeightMap, _basins: &Vec<Point>) -> Vec<RiverPath> {
        let width = map_params.width;
        let height = map_params.height;
        let initial_volume = map_params.initial_volume;

        let mut river_paths: Vec<RiverPath> = Vec::new();
        let mut visited: Vec<Vec<bool>> = vec![vec![false; width]; height];

        // Assuming each source starts with a volume of 1
        // Adjust based on your simulation's scale and realism factors
        for start_point in start_points {
            let mut current_path = RiverPath { points: vec![start_point], volume: initial_volume };
            let mut current_point = start_point;

            while let Some(next_point) = find_next_flow_point(&current_point, height_map, &mut visited) {
                // Check if this new point connects with an existing river
                if let Some(existing_path) = find_connecting_path(&next_point, &mut river_paths) {
                    existing_path.points.extend_from_slice(&current_path.points);
                    existing_path.volume += current_path.volume;
                    break;  // Exit the loop as the current river has merged into an existing one
                } else {
                    current_path.points.push(next_point);
                    current_point = next_point;
                }
            }

            if current_path.points.len() > 1 { // Ignore trivial paths
                river_paths.push(current_path);
            }
        }

        river_paths
    }

    fn _find_lowest_adjacent(point: Point, height_map: &HeightMap) -> Point {
        let (x, y) = point;
        let mut lowest_point = point;
        let mut lowest_height = height_map[y][x];

        let adjacent_points = [
            (x.wrapping_sub(1), y), (x + 1, y),
            (x, y.wrapping_sub(1)), (x, y + 1),
            (x.wrapping_sub(1), y.wrapping_sub(1)), (x + 1, y + 1),
            (x.wrapping_sub(1), y + 1), (x + 1, y.wrapping_sub(1)),
        ];

        for &(adj_x, adj_y) in &adjacent_points {
            if adj_x >= height_map[0].len() || adj_y >= height_map.len() {
                continue; // Skip if out of bounds
            }
            let adj_height = height_map[adj_y][adj_x];
            if adj_height < lowest_height {
                lowest_height = adj_height;
                lowest_point = (adj_x, adj_y);
            }
        }

        lowest_point
    }

    fn find_connecting_path<'a>(new_point: &Point, river_paths: &'a mut [RiverPath]) -> Option<&'a mut RiverPath> {
        for path in river_paths.iter_mut() {
            if path.points.last() == Some(new_point) {
                return Some(path);
            }
        }
        None
    }

    fn find_next_flow_point(current: &Point, height_map: &HeightMap, visited: &mut Vec<Vec<bool>>) -> Option<Point> {
        let (x, y) = *current;
        let mut adjacent_points = vec![
            (x.wrapping_sub(1), y), // left
            (x + 1, y),            // right
            (x, y.wrapping_sub(1)), // up
            (x, y + 1),            // down
            // Remove diagonals if you want more natural river flow
        ];

        // Shuffle or rotate adjacent_points to avoid bias in direction
        adjacent_points.shuffle(&mut rand::thread_rng());  // Make sure to use the rand crate

        let mut lowest_point: Option<Point> = None;
        let mut lowest_height = f64::INFINITY;

        for &(adj_x, adj_y) in &adjacent_points {
            if adj_x < height_map[0].len() && adj_y < height_map.len() && !visited[adj_y][adj_x] {
                let adj_height = height_map[adj_y][adj_x];
                if adj_height < lowest_height {
                    lowest_height = adj_height;
                    lowest_point = Some((adj_x, adj_y));
                }
            }
        }

        // If all adjacent points are at the same height (flat terrain),
        // choose an unvisited one to prevent being stuck
        if lowest_height >= height_map[y][x] {
            for &(adj_x, adj_y) in &adjacent_points {
                if adj_x < height_map[0].len() && adj_y < height_map.len() && !visited[adj_y][adj_x] {
                    return Some((adj_x, adj_y));  // Return the first unvisited adjacent point
                }
            }
        }

        if let Some(point) = lowest_point {
            visited[point.1][point.0] = true;  // Mark this point as visited
        }

        lowest_point
    }

    fn form_lake(water_map: &mut WaterMap, center: &Point, volume: usize) {
        // Example: Form a square lake for simplicity
        let side_length = (volume as f64).sqrt().round() as usize; // Adjust based on desired lake size
        let (cx, cy) = *center;
        for y in cy.saturating_sub(side_length)..=cy + side_length {
            for x in cx.saturating_sub(side_length)..=cx + side_length {
                if y < water_map.len() && x < water_map[0].len() {
                    water_map[y][x] = true;
                }
            }
        }
    }
}
