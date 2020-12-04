use std::ops::Range;
use std::time::{Instant, Duration};
use rand::prelude::*;
use std::f64::consts::PI;
//use std::f64::consts::PI;

const PI_DIGITS: &str = "14159265358979323846264338327950288";

pub fn main() {
    // monte_carlo_serial(15, 10, 10);
    // polygon_range(3..100);
    polygon_exponential(20, 10, 10);
}

#[derive(Debug)]
struct PiResult {
    pub count: usize,
    pub duration: Duration,
    pub correct_digits: usize,
    pub approx_pi: f64,
}

impl PiResult {
    pub fn new(count: usize, duration: Duration, correct_digits: usize, approx_pi: f64) -> Self {
        Self {
            count,
            duration,
            correct_digits,
            approx_pi,
        }
    }
}

fn monte_carlo_serial(trial_count: usize, darts_from: usize, darts_mult: usize) {
    let mut rng = rand::thread_rng();
    let mut overall_results = vec![];
    let radius = 0.5;
    let origin = radius;
    let mut dart_count = darts_from;
    for _ in 0..trial_count {
        let start = Instant::now();
        let mut hit_count = 0usize;
        for _ in 0..dart_count {
            let x: f64 = rng.gen();
            let y: f64 = rng.gen();
            let x_dist = (x - origin).abs();
            let y_dist = (y - origin).abs();
            let dist_to_origin = ((x_dist * x_dist) + (y_dist * y_dist)).sqrt();
            if dist_to_origin <= radius {
                hit_count += 1;
            }
        }
        let hit_pct = hit_count as f64 / dart_count as f64;
        let approx_pi = 4.0 * hit_pct;
        let duration = Instant::now() - start;
        let correct_digits = count_correct_pi_digits(approx_pi);
        let result = PiResult::new(dart_count, duration, correct_digits, approx_pi);
        dbg!(&result);
        overall_results.push(result);
        dart_count *= darts_mult;
    }
    // bg!(&overall_results);
}

fn polygon_increment(sides_range: Range<usize>) {
    for side_count in sides_range {
        polygon_one(side_count);
    }
}

fn polygon_exponential(trial_count: usize, sides_from: usize, sides_mult: usize) {
    let mut side_count = sides_from;
    for _ in 0..trial_count {
        polygon_one(side_count);
        side_count *= sides_mult;
    }
}

fn polygon_one(side_count: usize) {
    let radius = 1.0;
    let start = Instant::now();
    let side_count_f64 = side_count as f64;
    let angle = (2.0 * PI) / (side_count_f64 * 2.0);
    let s = angle.sin();
    let approx_circumference = s * side_count_f64 * 2.0;
    let approx_pi = approx_circumference / 2.0;
    //bg!(side_count_f64, angle.to_degrees(), s, approx_circumference, approx_pi);
    let duration = Instant::now() - start;
    let correct_digits = count_correct_pi_digits(approx_pi);
    let result = PiResult::new(side_count, duration, correct_digits, approx_pi);
    dbg!(&result);
}

fn count_correct_pi_digits(approx_pi: f64) -> usize {
    count_correct_digits(PI_DIGITS, approx_pi)
}


fn count_correct_digits(reference_value: &str, approx_value: f64) -> usize {
    //bg!(approx_value);
    // debug_assert!(approx_value >= 0.0);
    // debug_assert!(approx_value <= 1.0);
    let reference_value_length = reference_value.len();
    // let approx_value_string = approx_value.to_string();
    let approx_value_string = &format!("{:.1$}", approx_value, reference_value_length)[2..];
    //bg!(reference_value, &approx_value_string);
    length_of_common_prefix(reference_value, approx_value_string)
}


fn length_of_common_prefix(a: &str, b: &str) -> usize {
    a.chars().zip(b.chars()).take_while(|(a,b)|a==b).map(|v|v.0).collect::<Vec<_>>().len()
}