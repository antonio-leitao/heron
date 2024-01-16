use crate::bitmatrix::Bitmatrix;
use crate::bitvec::Bitvec;
use crate::bitmatrix::BoundaryMatrix;
use crate::linalg::rank;

pub fn betti_numbers(adjacency_matrix:Bitmatrix) -> Vec<usize>{
  //start variables
  let mut betti_numbers = Vec::new();
  let mut n_cliques_k = adjacency_matrix.matrix.len();
  //you have to hold 2 values
  let mut cliques_k:Vec<Bitvec> = Vec::new();
  for node in 0..n_cliques_k{
    cliques_k.push(Bitvec::from_vector(&[node], n_cliques_k));
  }
  let mut rk = 0;
  //loop for k+1
  loop {
    // get k1 info
    let (cliques_k1,mut delta) = adjacency_matrix.boundary_matrix(&cliques_k);
    let n_cliques_k1 = cliques_k1.len();
    if n_cliques_k1==0{break}
    let rk1 = rank(&mut delta,n_cliques_k);
    let bk = n_cliques_k - (rk + rk1 );
    betti_numbers.push(bk);
    //update k+1 -> k
    cliques_k = cliques_k1;
    n_cliques_k = n_cliques_k1;
    rk = rk1;
  };
  betti_numbers
}
