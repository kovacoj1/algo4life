use pyo3::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rayon::prelude::*;

#[pyfunction]
#[pyo3(signature = (threshold, n, k))]
fn run_simulation(threshold: f64, n: usize, k: usize) -> PyResult<f64> {
    // Parallel computation using Rayon
    let total_successes: usize = (0..k)
        .into_par_iter()
        .map(|_| {
            let mut rng = thread_rng();
            let mut candidates: Vec<usize> = (0..n).collect();
            candidates.shuffle(&mut rng);

            let split = (threshold * n as f64) as usize;
            let benchmark = if split > 0 {
                *candidates[0..split].iter().max().unwrap_or(&0)
            } else {
                0
            };

            for i in split..n {
                if candidates[i] > benchmark {
                    return if candidates[i] == n - 1 { 1 } else { 0 };
                }
            }
            0
        })
        .sum();

    Ok(total_successes as f64 / k as f64)
}

#[pymodule]
fn rust_sim(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_simulation, m)?)?;
    Ok(())
}
