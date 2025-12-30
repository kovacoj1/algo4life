use rand::seq::SliceRandom;
use rand::thread_rng;
use rayon::prelude::*;

fn run_single_trial(n: usize, threshold: f64) -> usize {
    let mut rng = thread_rng();
    let mut candidates: Vec<usize> = (0..n).collect();
    candidates.shuffle(&mut rng);

    let split = (threshold * n as f64) as usize;
    let benchmark = if split > 0 {
        *candidates[0..split].iter().max().unwrap()
    } else {
        0
    };

    for i in split..n {
        if candidates[i] > benchmark {
            return if candidates[i] == n - 1 { 1 } else { 0 };
        }
    }
    0
}

fn main() {
    let n = 1_000;
    let k = 1_000_000; 
    let threshold = 1.0 / std::f64::consts::E;

    println!("Starting simulation on {} threads...", rayon::current_num_threads());

    let total_successes: usize = (0..k)
        .into_par_iter()
        .map(|_| run_single_trial(n, threshold))
        .sum();

    let success_rate = total_successes as f64 / k as f64;
    println!("Results: {:.6} (Target: {:.6})", success_rate, threshold);
}
