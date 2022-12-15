use std::num::ParseIntError;

use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
struct SensorBeaconPair {
    sensor: (i64, i64),
    beacon: (i64, i64),
}

impl SensorBeaconPair {
    fn coverage_dist(&self) -> u64 {
        self.sensor.0.abs_diff(self.beacon.0) + self.sensor.1.abs_diff(self.beacon.1)
    }

    fn min_y(&self) -> i64 {
        self.sensor
            .1
            .min(self.beacon.1)
            .min(self.sensor.1 - self.coverage_dist() as i64)
    }
    fn min_x(&self) -> i64 {
        self.sensor
            .0
            .min(self.beacon.0)
            .min(self.sensor.0 - self.coverage_dist() as i64)
    }
    fn max_y(&self) -> i64 {
        self.sensor
            .1
            .max(self.beacon.1)
            .max(self.sensor.1 + self.coverage_dist() as i64)
    }
    fn max_x(&self) -> i64 {
        self.sensor
            .0
            .max(self.beacon.0)
            .max(self.sensor.0 + self.coverage_dist() as i64)
    }
}

impl TryFrom<&str> for SensorBeaconPair {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let re = Regex::new(r"at x=(-?\d+), y=(-?\d+):.*x=(-?\d+), y=(-?\d+)").unwrap();
        if let Some(groups) = re.captures(value) {
            let sensor_x = groups
                .get(1)
                .unwrap()
                .as_str()
                .parse()
                .map_err(|e: ParseIntError| e.to_string())?;
            let sensor_y = groups
                .get(2)
                .unwrap()
                .as_str()
                .parse()
                .map_err(|e: ParseIntError| e.to_string())?;
            let beacon_x = groups
                .get(3)
                .unwrap()
                .as_str()
                .parse()
                .map_err(|e: ParseIntError| e.to_string())?;
            let beacon_y = groups
                .get(4)
                .unwrap()
                .as_str()
                .parse()
                .map_err(|e: ParseIntError| e.to_string())?;

            Ok(SensorBeaconPair {
                sensor: (sensor_x, sensor_y),
                beacon: (beacon_x, beacon_y),
            })
        } else {
            Err("invalid input".to_string())
        }
    }
}

fn solve1(input: &[String]) -> impl std::fmt::Display {
    let mut covered_areas = Vec::new();

    const Y_TARGET: i64 = 2000000;

    let mut grid_min_x = i64::MAX;
    let mut grid_min_y = i64::MAX;
    let mut grid_max_x = i64::MIN;
    let mut grid_max_y = i64::MIN;

    for line in input {
        let sb: SensorBeaconPair = line.as_str().try_into().expect("invalid input");

        grid_min_x = grid_min_x.min(sb.min_x());
        grid_min_y = grid_min_y.min(sb.min_y());
        grid_max_x = grid_max_x.max(sb.max_x());
        grid_max_y = grid_max_y.max(sb.max_y());

        let cov = sb.coverage_dist();
        let dist_to_target = sb.sensor.1.abs_diff(Y_TARGET);

        if dist_to_target < cov {
            let delta = (cov - dist_to_target) as i64;
            let min = sb.sensor.0 - delta;
            let max = sb.sensor.0 + delta;

            covered_areas.push((min, max))
        }
    }

    covered_areas.sort();

    let mut prev = grid_min_x - 1;
    let mut uncovered = 0;

    covered_areas.push((grid_max_x + 1, grid_max_x + 1));
    for covered_area in covered_areas {
        if covered_area.0 > prev {
            uncovered += covered_area.0 - prev - 1;
        }

        prev = prev.max(covered_area.1);
    }

    grid_max_x - grid_min_x - uncovered
}

fn solve2(input: &[String]) -> impl std::fmt::Display {
    if !cfg!(debug_assertions) {
        let mut grid_min_x = i64::MAX;
        let mut grid_min_y = i64::MAX;
        let mut grid_max_x = i64::MIN;
        let mut grid_max_y = i64::MIN;

        let mut sbs = Vec::new();

        for line in input {
            let sb: SensorBeaconPair = line.as_str().try_into().expect("invalid input");

            grid_min_x = grid_min_x.min(sb.min_x());
            grid_min_y = grid_min_y.min(sb.min_y());
            grid_max_x = grid_max_x.max(sb.max_x());
            grid_max_y = grid_max_y.max(sb.max_y());

            sbs.push(sb);
        }

        for y in 0..=4000000 {
            let mut covered_areas = Vec::new();

            for sb in sbs.iter() {
                let cov = sb.coverage_dist();
                let dist_to_target = sb.sensor.1.abs_diff(y);
                if dist_to_target < cov {
                    let delta = (cov - dist_to_target) as i64;
                    let min = sb.sensor.0 - delta;
                    let max = sb.sensor.0 + delta;

                    covered_areas.push((min, max))
                }
            }

            covered_areas.sort();
            let mut covered_areas = covered_areas.into_iter();
            if let Some((_, mut prev)) = covered_areas.next() {
                for (min, max) in covered_areas {
                    if min > prev + 1 {
                        // found it
                        assert!(min == prev + 2);
                        let x = prev + 1;

                        return (x * 4000000 + y).to_string();
                    }

                    prev = prev.max(max)
                }
            }
        }

        "no solution found :O".to_string()
    } else {
        "disabled for debug build :(".to_string()
    }
}

impl_dayx!("15", solve1, solve2);
