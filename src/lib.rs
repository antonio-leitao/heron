mod bitmatrix;
mod bitvec;
mod homology;
mod linalg;
use bitmatrix::{AllCliques, Bitmatrix, NextCliques};
use bitvec::Bitvec;
use pyo3::prelude::*;
use std::time::Instant;

#[pyfunction]
fn betti_numbers(adjacency_matrix: Vec<Vec<usize>>) -> PyResult<Vec<usize>> {
    let n_nodes = adjacency_matrix.len();
    let graph = Bitmatrix::new(adjacency_matrix, n_nodes);
    Ok(homology::betti_numbers(graph))
}

#[pyfunction]
fn find_cliques(adjacency_matrix: Vec<Vec<usize>>) -> PyResult<(f64, u32)> {
    let n_nodes = adjacency_matrix.len();
    let graph = Bitmatrix::new(adjacency_matrix, n_nodes);
    let start_time = Instant::now();
    let count = graph.all_cliques();
    let elapsed = start_time.elapsed();
    Ok((elapsed.as_secs() as f64, count as u32))
}

#[pyfunction]
fn cliques_up_to(adjacency_matrix: Vec<Vec<usize>>) -> PyResult<(f64, u32)> {
    let n_nodes = adjacency_matrix.len();
    let mut cliques = Vec::new();
    let mut count = 0;
    let graph = Bitmatrix::new(adjacency_matrix, n_nodes);
    for i in 0..n_nodes {
        cliques.push(Bitvec::from_vector(&[i], n_nodes))
    }
    let start_time = Instant::now();
    while cliques.len() > 0 {
        count += cliques.len();
        cliques = graph.get_next_cliques(&cliques);
    }
    let elapsed = start_time.elapsed();
    Ok((elapsed.as_secs() as f64, count as u32))
}

/// A Python module implemented in Rust.
#[pymodule]
fn heron(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(find_cliques, m)?)?;
    m.add_function(wrap_pyfunction!(cliques_up_to, m)?)?;
    m.add_function(wrap_pyfunction!(betti_numbers, m)?)?;
    Ok(())
}
