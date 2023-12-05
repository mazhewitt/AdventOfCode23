use std::time::Instant;
use rayon::prelude::*;
#[derive(Clone, PartialEq, Debug)]
struct SeedRange {
    dest_start: i64,
    src_start: i64,
    length: i64,
}
impl SeedRange {
    fn contains(&self, value: i64) -> bool {
        value >= self.src_start && value < self.src_start + self.length
    }
}

struct RangeMapper {
    range_map: Vec<SeedRange>,
}

impl RangeMapper {
    // Initializes a new RangeMapper with an empty map
    fn new() -> Self {
        RangeMapper { range_map: Vec::new() }
    }

    // Loads a range mapping into the RangeMapper and merges the ranges
    fn load_map(&mut self, map_data: Vec<(i64, i64, i64)>) {
        let mut initial_ranges: Vec<SeedRange> = map_data.into_iter()
            .map(|(dest_start, src_start, length)| SeedRange { dest_start, src_start, length })
            .collect();

        // Sort the initial range data
        initial_ranges.sort_by_key(|range| range.src_start);

        // Now merge the sorted ranges
        self.range_map = self.merge_ranges(initial_ranges);
    }

    // Merges overlapping or adjacent ranges
    fn merge_ranges(&self, ranges: Vec<SeedRange>) -> Vec<SeedRange> {
        let mut merged = vec![];
        if ranges.is_empty() {
            return merged;
        }

        let mut curr = ranges[0].clone();

        for range in ranges.iter().skip(1) {
            if curr.src_start + curr.length > range.src_start {
                // Extend the current range only if there is an overlap
                curr.length = std::cmp::max(curr.src_start + curr.length, range.src_start + range.length) - curr.src_start;
            } else {
                // Push the current range and start a new one
                merged.push(curr);
                curr = range.clone();
            }
        }
        merged.push(curr);
        merged
    }


    // Maps a number based on the loaded range map
    fn map_number(&self, source: i64) -> i64 {
        for range in &self.range_map {
            if range.contains(source) {
                return range.dest_start + (source - range.src_start);
            }
        }
        source // Default to source if not in any range
    }

    fn from_vec(map_data: Vec<(i64, i64, i64)>) -> Self {
        // use load_map
        let mut mapper = RangeMapper::new();
        mapper.load_map(map_data);
        mapper
    }
    fn from_string(data: &str) -> Self{
        let range_map = data.lines()
            .filter_map(|line| {
                let parts: Vec<_> = line.split_whitespace().collect();
                if parts.len() == 3 {
                    Some((
                        parts[0].parse().unwrap_or(0),
                        parts[1].parse().unwrap_or(0),
                        parts[2].parse().unwrap_or(0),
                    ))
                } else {
                    None
                }
            })
            .collect();
        RangeMapper::from_vec(range_map)
    }
}

struct GardenMapper {
    seed_to_soil: RangeMapper,
    soil_to_fertilizer: RangeMapper,
    fertilizer_to_water: RangeMapper,
    water_to_light: RangeMapper,
    light_to_temperature: RangeMapper,
    temperature_to_humidity: RangeMapper,
    humidity_to_location: RangeMapper,
}

impl GardenMapper {
    fn new(
        seed_to_soil: RangeMapper,
        soil_to_fertilizer: RangeMapper,
        fertilizer_to_water: RangeMapper,
        water_to_light: RangeMapper,
        light_to_temperature: RangeMapper,
        temperature_to_humidity: RangeMapper,
        humidity_to_location: RangeMapper,
    ) -> Self {
        GardenMapper {
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        }
    }

    fn find_smallest_location(&self, seeds: &[i64]) -> i64 {
        seeds.iter().map(|&seed| {
            self.find_location_for_seed(seed)
        }).min().unwrap_or(i64::MAX) // Returns the smallest location or i64::MAX if no seeds are provided
    }

    pub fn find_location_for_seed(&self, seed: i64) -> i64 {
        let soil = self.seed_to_soil.map_number(seed);
        let fertilizer = self.soil_to_fertilizer.map_number(soil);
        let water = self.fertilizer_to_water.map_number(fertilizer);
        let light = self.water_to_light.map_number(water);
        let temperature = self.light_to_temperature.map_number(light);
        let humidity = self.temperature_to_humidity.map_number(temperature);
        self.humidity_to_location.map_number(humidity)
    }
}
fn parse_file_contents(file_contents: &str) -> (Vec<i64>, GardenMapper) {
    let sections: Vec<_> = file_contents.split("\n\n").collect();

    let seeds = sections[0]
        .split(':')
        .nth(1)
        .unwrap_or("")
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    let garden_mapper = GardenMapper::new(
        RangeMapper::from_string(sections[1]),
        RangeMapper::from_string(sections[2]),
        RangeMapper::from_string(sections[3]),
        RangeMapper::from_string(sections[4]),
        RangeMapper::from_string(sections[5]),
        RangeMapper::from_string(sections[6]),
        RangeMapper::from_string(sections[7]),
    );


    // ... Load other mappings similarly

    (seeds, garden_mapper)
}

fn expand_seed_ranges(ranges:&Vec<(i64, i64)>) -> Vec<i64> {
    let mut seeds = Vec::new();
    for (start, length) in ranges {
        seeds.extend(*start..*start + *length);
    }
    seeds
}

fn process_seed_ranges(seed_ranges: Vec<(i64, i64)>, garden_mapper: &GardenMapper) -> i64 {
    seed_ranges.into_par_iter()
        .flat_map(|(start, length)| (start..start + length))
        .map(|seed| {
            // Process each seed to find its corresponding location
            garden_mapper.find_location_for_seed(seed)
        })
        .min()
        .unwrap_or(i64::MAX)  // Or handle the case when there are no seeds differently
}

fn main() {
    let file_contents = std::fs::read_to_string("input_file.txt").expect("Failed to read input_file.txt");
    let (seeds, garden_mapper) = parse_file_contents(&file_contents);
    // time the next method call
    let start = Instant::now(); // Start timing
    let smallest_location = garden_mapper.find_smallest_location(&seeds);
    let duration = start.elapsed(); // Measure time elapsed since start
    println!("Time taken: {:?}", duration); // Print out the duration
    println!("Smallest location: {}", smallest_location);
    // convert seeds to list of tuples
    let seed_tuples:Vec<(i64,i64)> = seeds.chunks(2) // Split the vector into chunks of 2 elements each
        .filter_map(|chunk| {
            if chunk.len() == 2 {
                Some((chunk[0], chunk[1])) // Create a tuple from the chunk
            } else {
                None // Ignore the last chunk if it has less than 2 elements
            }
        })
        .collect(); // Collect the results into a Vec<(i64, i64)>

    let expanded_seeds = expand_seed_ranges(&seed_tuples);
    let start = Instant::now(); // Start timing
    println!("number of seeds: {}",expanded_seeds.len());
    let smallest_location_exp = process_seed_ranges(seed_tuples, &garden_mapper);
    let duration = start.elapsed(); // Measure time elapsed since start
    println!("Time taken: {:?}", duration); // Print out the duration
    println!("Smallest location of expanded seeds: {}", smallest_location_exp);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seed_to_soil_mapping() {
        let mut mapper = RangeMapper::new();
        mapper.load_map(vec![(50, 98, 2), (52, 50, 48)]);

        assert_eq!(mapper.map_number(79), 81);
        assert_eq!(mapper.map_number(14), 14);
        assert_eq!(mapper.map_number(55), 57);
        assert_eq!(mapper.map_number(13), 13);
    }

    #[test]
    fn test_mapping_through_categories() {

        // Seeds to be tested
        let seeds : Vec<i64> = vec![79, 14, 55, 13];
        let garden_mapper = prepare_test_garden();



        // Find the smallest location using the GardenMapper
        let smallest_location = garden_mapper.find_smallest_location(&seeds);

        // Assert that the smallest location is as expected
        assert_eq!(smallest_location, 35);
    }

    fn prepare_test_garden() -> GardenMapper {
// Instantiate each RangeMapper using the from_vec constructor
        let seed_to_soil_mapper = RangeMapper::from_vec(vec![(50, 98, 2), (52, 50, 48)]);
        let soil_to_fertilizer_mapper = RangeMapper::from_vec(vec![(0, 15, 37), (37, 52, 2), (39, 0, 15)]);
        let fertilizer_to_water_mapper = RangeMapper::from_vec(vec![(49, 53, 8), (0, 11, 42), (42, 0, 7), (57, 7, 4)]);
        let water_to_light_mapper = RangeMapper::from_vec(vec![(88, 18, 7), (18, 25, 70)]);
        let light_to_temperature_mapper = RangeMapper::from_vec(vec![(45, 77, 23), (81, 45, 19), (68, 64, 13)]);
        let temperature_to_humidity_mapper = RangeMapper::from_vec(vec![(0, 69, 1), (1, 0, 69)]);
        let humidity_to_location_mapper = RangeMapper::from_vec(vec![(60, 56, 37), (56, 93, 4)]);

        // Create a GardenMapper instance
        let garden_mapper = GardenMapper::new(
            seed_to_soil_mapper,
            soil_to_fertilizer_mapper,
            fertilizer_to_water_mapper,
            water_to_light_mapper,
            light_to_temperature_mapper,
            temperature_to_humidity_mapper,
            humidity_to_location_mapper,
        );
        garden_mapper
    }

    #[test]
    fn test_with_expanded_seeds(){
        // seeds = seeds: 79 14 55 13

        let expanded_seeds = expand_seed_ranges(&vec![(79,14), (55,13) ]);
        let garden_mapper = prepare_test_garden();



        // Find the smallest location using the GardenMapper
        let smallest_location = garden_mapper.find_smallest_location(&expanded_seeds);

        // Assert that the smallest location is as expected
        assert_eq!(smallest_location, 46);
    }

    #[test]
    fn test_seed_to_soil_mapping2() {
        let mut mapper = RangeMapper::new();
        mapper.load_map(vec![(50, 98, 2), (52, 50, 48)]);

        assert_eq!(mapper.map_number(79), 81);
        assert_eq!(mapper.map_number(14), 14);
        assert_eq!(mapper.map_number(55), 57);
        assert_eq!(mapper.map_number(13), 13);
    }


    #[test]
    fn test_load_map_with_overlapping_ranges() {
        let mut mapper = RangeMapper::new();
        // Providing overlapping ranges
        let map_data = vec![
            (10, 0, 5),  // dest: 10-14, src: 0-4
            (15, 3, 4),  // dest: 15-18, src: 3-6
            (20, 7, 2),  // dest: 20-21, src: 7-8
            (25, 10, 3), // dest: 25-27, src: 10-12
        ];

        mapper.load_map(map_data);

        // Expected merged ranges
        let expected_ranges = vec![
            SeedRange { dest_start: 10, src_start: 0, length: 7 }, // Merged 0-6
            SeedRange { dest_start: 20, src_start: 7, length: 2 }, // Separate 7-8
            SeedRange { dest_start: 25, src_start: 10, length: 3 }, // Separate 10-12
        ];

        assert_eq!(mapper.range_map, expected_ranges);
    }

    #[test]
    fn test_load_map_sorting() {
        let mut mapper = RangeMapper::new();
        // Providing unsorted range data
        let map_data = vec![
            (30, 10, 5),  // dest: 30-34, src: 10-14
            (10, 0, 5),   // dest: 10-14, src: 0-4
            (20, 5, 5),   // dest: 20-24, src: 5-9
        ];

        mapper.load_map(map_data);

        // Expected ranges after sorting (but before merging)
        let expected_sorted_ranges = vec![
            SeedRange { dest_start: 10, src_start: 0, length: 5 },
            SeedRange { dest_start: 20, src_start: 5, length: 5 },
            SeedRange { dest_start: 30, src_start: 10, length: 5 },
        ];

        // Check if the ranges are sorted correctly
        assert_eq!(mapper.range_map, expected_sorted_ranges);
    }

    #[test]
    fn test_expand_seed_ranges(){
        let seed_tuples:Vec<(i64,i64)> = vec![(79,14), (55,13) ];
        let expanded_seeds = expand_seed_ranges(&seed_tuples);
        assert_eq!(expanded_seeds.len(), 27);
        assert_eq!(expanded_seeds[0], 79);
        assert_eq!(expanded_seeds[26], 67);
    }
}