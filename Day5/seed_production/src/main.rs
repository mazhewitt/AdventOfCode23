struct RangeMapper {
    range_map: Vec<(usize, usize, usize)>,
}

impl RangeMapper {
    // Initializes a new RangeMapper with an empty map
    fn new() -> Self {
        RangeMapper { range_map: Vec::new() }
    }

    // Loads a range mapping into the RangeMapper
    fn load_map(&mut self, map_data: Vec<(usize, usize, usize)>) {
        self.range_map = map_data;
    }

    // Maps a number based on the loaded range map
    fn map_number(&self, source: usize) -> usize {
        for &(dest_start, src_start, length) in self.range_map.iter() {
            if source >= src_start && source < src_start + length {
                return dest_start + (source - src_start);
            }
        }
        source // Default to source if not in any range
    }

    fn map_through_categories(&self, start: usize, maps: &[&RangeMapper]) -> usize {
        let mut result = start;
        for map in maps {
            result = self.map_number(result);
        }
        result
    }
    fn from_vec(map_data: Vec<(usize, usize, usize)>) -> Self {
        RangeMapper { range_map: map_data }
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

    fn find_smallest_location(&self, seeds: &[usize]) -> usize {
        seeds.iter().map(|&seed| {
            let soil = self.seed_to_soil.map_number(seed);
            let fertilizer = self.soil_to_fertilizer.map_number(soil);
            let water = self.fertilizer_to_water.map_number(fertilizer);
            let light = self.water_to_light.map_number(water);
            let temperature = self.light_to_temperature.map_number(light);
            let humidity = self.temperature_to_humidity.map_number(temperature);
            self.humidity_to_location.map_number(humidity)
        }).min().unwrap_or(usize::MAX) // Returns the smallest location or usize::MAX if no seeds are provided
    }


}
fn parse_file_contents(file_contents: &str) -> (Vec<usize>, GardenMapper) {
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

fn main() {
    let file_contents = std::fs::read_to_string("input_file.txt").expect("Failed to read input_file.txt");
    let (seeds, garden_mapper) = parse_file_contents(&file_contents);
    let smallest_location = garden_mapper.find_smallest_location(&seeds);
    println!("Smallest location: {}", smallest_location);
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

        // Seeds to be tested
        let seeds = vec![79, 14, 55, 13];

        // Find the smallest location using the GardenMapper
        let smallest_location = garden_mapper.find_smallest_location(&seeds);

        // Assert that the smallest location is as expected
        assert_eq!(smallest_location, 35);
    }

}