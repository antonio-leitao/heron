use pyo3::prelude::*;
mod bitmatrix;
mod bitvec;
use bitmatrix::{Bitmatrix, FindCliques};
use std::time::Instant;

#[pyfunction]
fn find_cliques(adjacency_matrix: Vec<Vec<usize>>) -> PyResult<(f64, u32)> {
    let n_nodes = adjacency_matrix.len();
    let graph = Bitmatrix::new(adjacency_matrix, n_nodes);
    let start_time = Instant::now();
    let cliques = graph.find_cliques();
    let count = cliques.len();
    let elapsed = start_time.elapsed();
    Ok((elapsed.as_secs() as f64, count))
}

/// A Python module implemented in Rust.
#[pymodule]
fn heron(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(find_cliques, m)?)?;
    Ok(())
}
