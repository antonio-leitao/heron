use crate::bitvec::Bitvec;
use std::collections::{vec_deque, VecDeque};

#[derive(Clone)]
pub struct Bitmatrix {
    matrix: Vec<Bitvec>,
    capacity: usize,
}

impl Bitmatrix {
    pub fn new(input_matrix: Vec<Vec<usize>>, capacity: usize) -> Self {
        let mut matrix: Vec<Bitvec> = Vec::with_capacity(capacity);
        // Convert each inner vector to a Nimbus instance
        for row in input_matrix.into_iter() {
            matrix.push(Bitvec::from_vector(&row, capacity));
        }
        Bitmatrix { matrix, capacity }
    }
    pub fn get_neighbours(&self, u: usize) -> Vec<usize> {
        self.matrix[u].elements()
    }
}

pub struct BronKerboshGenerator<'a> {
    stack: VecDeque<(Bitvec, Bitvec, Bitvec)>,
    matrix: &'a Bitmatrix,
}

pub trait BronKerbosh {
    // Define the methods or associated types here
    fn choose_pivot_vertex(&self, pux: &Bitvec) -> usize;
    fn bron_kerbosh_pivot(&self) -> BronKerboshGenerator;
}

impl BronKerbosh for Bitmatrix {
    fn choose_pivot_vertex(&self, pux: &Bitvec) -> usize {
        let mut max_degree: u32 = 0;
        let mut pivot: usize = 0;
        for vertex in pux.elements().into_iter() {
            let degree = self.matrix[vertex].n_elements() as u32;
            if degree + 1 > max_degree {
                max_degree = degree + 1;
                pivot = vertex;
            };
        }
        pivot
    }
    fn bron_kerbosh_pivot(&self) -> BronKerboshGenerator {
        BronKerboshGenerator::new(self)
    }
}

impl<'a> BronKerboshGenerator<'a> {
    fn new(matrix: &'a Bitmatrix) -> Self {
        let indices = (0..matrix.capacity).collect::<Vec<_>>();
        let p = Bitvec::from_vector(&indices, matrix.capacity);
        let mut stack: VecDeque<(Bitvec, Bitvec, Bitvec)> = VecDeque::new();
        stack.push_front((
            Bitvec::new(matrix.capacity),
            p.clone(),
            Bitvec::new(matrix.capacity),
        ));
        BronKerboshGenerator { stack, matrix }
    }
}

impl<'a> Iterator for BronKerboshGenerator<'a> {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((r, p, x)) = self.stack.pop_front() {
            if p.is_empty() && x.is_empty() {
                return Some(r.elements());
            } else {
                let q = self.matrix.choose_pivot_vertex(&p.union(&x));
                let q_neighbors = self.matrix.matrix[q].clone();
                if let Some(v) = p.difference(&q_neighbors).first_index() {
                    self.stack
                        .push_front((r.clone(), p.removal(v), x.insertion(v)));
                    let v_neighbours = self.matrix.matrix[v].clone();
                    self.stack.push_front((
                        r.insertion(v),
                        p.intersection(&v_neighbours),
                        x.intersection(&v_neighbours),
                    ));
                }
            }
        }
        None
    }
}

// NEW VERSION
pub trait Networkx {
    // Define the methods or associated types here
    fn get_max_degree(&self, pool: &Bitvec) -> Option<usize>;
    fn tomita(&self) -> usize;
}

impl Networkx for Bitmatrix {
    fn get_max_degree(&self, pool: &Bitvec) -> Option<usize> {
        pool.elements()
            .iter()
            .max_by_key(|u| self.matrix[**u].n_elements())
            .cloned()
    }

    fn tomita(&self) -> usize {
        let mut count = 0;
        let mut Q: Vec<Option<usize>> = Vec::new();
        Q.push(None);
        let mut cand = Bitvec::new(self.capacity);
        for i in 0..self.capacity {
            cand.insert(i)
        }
        let mut subg = cand.clone();
        let mut stack = Vec::new();
        let mut u = match self.get_max_degree(&cand) {
            Some(elem) => elem,
            None => return count,
        };
        let mut ext_u = cand.difference(&self.matrix[u]);
        while !Q.is_empty() | !stack.is_empty() | !ext_u.is_empty() {
            match ext_u.pop() {
                Some(q) => {
                    cand.remove(q);
                    match Q.last_mut() {
                        Some(last) => *last = Some(q),
                        None => break,
                    };
                    let adj_q = &self.matrix[q];
                    let subg_q = subg.intersection(adj_q);
                    if subg_q.is_empty() {
                        count += 1;
                    } else {
                        let cand_q = cand.intersection(adj_q);
                        if !cand_q.is_empty() {
                            stack.push((subg, cand, ext_u));
                            Q.push(None); //???
                            subg = subg_q;
                            cand = cand_q;
                            match self.get_max_degree(&cand.intersection(&self.matrix[u])) {
                                Some(index) => {
                                    u = index;
                                    ext_u = cand.difference(&self.matrix[u]);
                                }
                                None => ext_u = cand.difference(&self.matrix[u]),
                            };
                        }
                    }
                }
                None => {
                    Q.pop();
                    if let Some((subg_prev, cand_prev, ext_u_prev)) = stack.pop() {
                        subg = subg_prev;
                        cand = cand_prev;
                        ext_u = ext_u_prev;
                    } else {
                        break; // Break the loop if the stack is empty
                    }
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bron_kerbosch_pivot() {
        let adjacency_matrix: Vec<Vec<usize>> = vec![
            vec![1, 4],
            vec![0, 2, 4],
            vec![1, 3],
            vec![2, 4, 5],
            vec![0, 1, 3],
            vec![3],
        ];
        let n_nodes = adjacency_matrix.len();
        let graph = Bitmatrix::new(adjacency_matrix, n_nodes);
        for maximal_clique in graph.bron_kerbosh_pivot() {
            println!("Maximal Clique: {:?}", maximal_clique);
        }
        let result: Vec<Vec<usize>> = graph.bron_kerbosh_pivot().collect();
        assert_eq!(
            result,
            vec![
                vec![1, 2],
                vec![0, 1, 4],
                vec![2, 3],
                vec![3, 4],
                vec![3, 5]
            ]
        )
    }
}
