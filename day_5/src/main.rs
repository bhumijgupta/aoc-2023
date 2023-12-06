use rayon::prelude::*;
use std::cmp::min;
use std::time::Instant;
use std::{fs, str::Lines};

use indicatif::{ProgressBar, ProgressStyle};

const PART_ONE: bool = false;
const USE_RAYON: bool = false;

#[derive(Debug)]
struct SourceDestData {
    source_range_start: u64,
    dest_range_start: u64,
    range_len: u64,
}

#[derive(Debug)]
struct ProcessedInputResp {
    seeds: Vec<u64>,
    seeds_to_soil_map: SourceDestCollection,
    soil_to_fertilizer_map: SourceDestCollection,
    fertilizer_to_water_map: SourceDestCollection,
    water_to_light_map: SourceDestCollection,
    light_to_temp_map: SourceDestCollection,
    temp_to_humi_map: SourceDestCollection,
    humi_to_loc_map: SourceDestCollection,
}

#[derive(Debug)]
struct SourceDestCollection {
    sorted: bool,
    data: Vec<SourceDestData>,
}

impl SourceDestCollection {
    fn get_dest(&self, source_num: &u64) -> u64 {
        let mut dest = 0;

        if *source_num < self.data[0].source_range_start {
            dest = *source_num;
        }

        let mut l = 0;
        let mut r = self.data.len() - 1;
        let mut mid_idx = 0;
        while l <= r {
            mid_idx = (l + r) / 2;
            let mid = &self.data.get(mid_idx).unwrap();
            match source_num.cmp(&mid.source_range_start) {
                std::cmp::Ordering::Less => {
                    if mid_idx > 0 {
                        r = mid_idx - 1;
                    } else {
                        break;
                    }
                }
                std::cmp::Ordering::Equal => {
                    dest = mid.dest_range_start;
                    break;
                }
                std::cmp::Ordering::Greater => {
                    dest = mid.dest_range_start;
                    l = mid_idx + 1;
                    if *source_num < mid.source_range_start + mid.range_len {
                        dest = mid.dest_range_start + (*source_num - mid.source_range_start);
                        break;
                    } else {
                        dest = *source_num;
                    }
                }
            }
        }

        // dest = *source_num;
        // for sp in &self.data {
        //     if *source_num >= sp.source_range_start
        //         && *source_num < sp.source_range_start + sp.range_len
        //     {
        //         dest = sp.dest_range_start + (*source_num - sp.source_range_start);
        //     }
        // }

        return dest;
    }
    fn sort(&mut self) {
        if self.sorted {
            return;
        }
        self.data
            .sort_by(|a, b| a.source_range_start.cmp(&b.source_range_start));
        self.sorted = true;
    }
    fn insert(&mut self, data_map: SourceDestData) {
        self.data.push(data_map);
    }
}

fn new_source_dest_collection() -> SourceDestCollection {
    return SourceDestCollection {
        sorted: false,
        data: vec![],
    };
}

fn process_input(input: String) -> ProcessedInputResp {
    let mut resp = ProcessedInputResp {
        seeds: vec![],
        seeds_to_soil_map: new_source_dest_collection(),
        soil_to_fertilizer_map: new_source_dest_collection(),
        fertilizer_to_water_map: new_source_dest_collection(),
        water_to_light_map: new_source_dest_collection(),
        light_to_temp_map: new_source_dest_collection(),
        temp_to_humi_map: new_source_dest_collection(),
        humi_to_loc_map: new_source_dest_collection(),
    };

    println!("Processing seeds");

    let mut line = input.lines();

    let seeds: Vec<&str> = line
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .collect();
    seeds.iter().for_each(|seed| {
        resp.seeds.push(seed.parse::<u64>().unwrap());
    });

    if !PART_ONE {
        let bar = ProgressBar::new(seeds.len().try_into().unwrap());

        resp.seeds = vec![];

        let mut is_seed_range = false;
        let mut seed_start = 0;
        for seed in seeds {
            bar.inc(1);
            if is_seed_range {
                let range = seed.parse::<u64>().unwrap();
                resp.seeds.extend(seed_start..seed_start + range);
            } else {
                seed_start = seed.parse::<u64>().unwrap();
            }
            is_seed_range = !is_seed_range;
        }
        bar.finish_and_clear();
    }

    // println!("Seeds processed: {:?}", resp.seeds);

    process_section(&mut line, &mut resp.seeds_to_soil_map);
    process_section(&mut line, &mut resp.soil_to_fertilizer_map);
    process_section(&mut line, &mut resp.fertilizer_to_water_map);
    process_section(&mut line, &mut resp.water_to_light_map);
    process_section(&mut line, &mut resp.light_to_temp_map);
    process_section(&mut line, &mut resp.temp_to_humi_map);
    process_section(&mut line, &mut resp.humi_to_loc_map);

    return resp;
}

fn process_section(line_iter: &mut Lines, collection: &mut SourceDestCollection) {
    println!("Processing section");

    let mut data = line_iter.next().unwrap();

    if data.len() == 0 {
        println!("  Skipping line: {:?}", data);
        data = line_iter.next().unwrap();
    }

    if data.ends_with("map:") {
        println!("  Skipping line: {:?}", data);
        data = line_iter.next().unwrap();
    }

    while data.len() != 0 {
        collection.insert(process_line(data.to_string()));
        data = match line_iter.next() {
            Some(line) => line,
            None => "",
        }
    }

    collection.sort();

    // println!("{:#?}", collection);
}

fn process_line(line: String) -> SourceDestData {
    // println!("  Processing line: {:?}", line);
    let line: Vec<&str> = line.split(" ").collect();
    let line: Vec<u64> = line
        .iter()
        .map(|x| x.parse::<u64>())
        .filter_map(Result::ok)
        .collect();
    return SourceDestData {
        source_range_start: line[1],
        dest_range_start: line[0],
        range_len: line[2],
    };
}

fn main() {
    let input = fs::read_to_string("./assets/input2.txt").unwrap();
    let processed_data = process_input(input);

    let bar = ProgressBar::new(processed_data.seeds.len().try_into().unwrap());
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{wide_bar} {pos}/{len} ETA: {eta} Ops/s: {per_sec}")
            .unwrap(),
    );

    let now = Instant::now();

    let mut min_loc = std::u64::MAX;

    if USE_RAYON {
        min_loc = {
            let seeds = &processed_data.seeds;
            let seeds_to_soil_map = &processed_data.seeds_to_soil_map;
            let soil_to_fertilizer_map = &processed_data.soil_to_fertilizer_map;
            let fertilizer_to_water_map = &processed_data.fertilizer_to_water_map;
            let water_to_light_map = &processed_data.water_to_light_map;
            let light_to_temp_map = &processed_data.light_to_temp_map;
            let temp_to_humi_map = &processed_data.temp_to_humi_map;
            let humi_to_loc_map = &processed_data.humi_to_loc_map;

            seeds
                .par_iter()
                .map(|seed| {
                    bar.inc(1);
                    let soil = seeds_to_soil_map.get_dest(seed);
                    let fert = soil_to_fertilizer_map.get_dest(&soil);
                    let water = fertilizer_to_water_map.get_dest(&fert);
                    let light = water_to_light_map.get_dest(&water);
                    let temp = light_to_temp_map.get_dest(&light);
                    let humi = temp_to_humi_map.get_dest(&temp);
                    humi_to_loc_map.get_dest(&humi)
                })
                .reduce(|| u64::MAX, |min_loc, loc| min(min_loc, loc))
        };
    } else {
        for seed in processed_data.seeds {
            bar.inc(1);
            let soil = processed_data.seeds_to_soil_map.get_dest(&seed);
            let fert = processed_data.soil_to_fertilizer_map.get_dest(&soil);
            let water = processed_data.fertilizer_to_water_map.get_dest(&fert);
            let light = processed_data.water_to_light_map.get_dest(&water);
            let temp = processed_data.light_to_temp_map.get_dest(&light);
            let humi = processed_data.temp_to_humi_map.get_dest(&temp);
            let loc = processed_data.humi_to_loc_map.get_dest(&humi);

            if loc < min_loc {
                min_loc = loc;
            }
        }
    }

    bar.finish();

    println!("Took {}s to compute", now.elapsed().as_secs());

    println!("{min_loc}")
}
