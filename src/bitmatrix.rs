use crate::bitvec::Bitvec;
use hashbrown::HashMap;

#[derive(Clone)]
pub struct Bitmatrix {
    pub matrix: Vec<Bitvec>,
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

pub trait AllCliques {
    // Define the methods or associated types here
    fn get_max_degree(&self, pool: &Bitvec) -> Option<usize>;
    fn all_cliques(&self) -> usize;
}

impl AllCliques for Bitmatrix {
    fn get_max_degree(&self, pool: &Bitvec) -> Option<usize> {
        pool.elements()
            .iter()
            .max_by_key(|u| self.matrix[**u].n_elements())
            .cloned()
    }

    fn all_cliques(&self) -> usize {
        let mut Q: Vec<Option<usize>> = Vec::new();
        let mut count = 0;
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
                        // println!("{:?}", Q)
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

/// Pretty efficient algoritghm for getting cliques, if I do say so myself.
/// receives a list of N dimensional cliques and returns all N+1 dimensional cliques.
/// It just duplicates and avoids lower degree nodes etc cannot think of better optimizations.
pub trait NextCliques {
    fn get_next_cliques(&self, cliques: &[Bitvec]) -> Vec<Bitvec>;
}

impl NextCliques for Bitmatrix {
    fn get_next_cliques(&self, cliques: &[Bitvec]) -> Vec<Bitvec> {
        let mut new_cliques: Vec<Bitvec> = Vec::new();
        let degrees: Vec<usize> = self.matrix.iter().map(|row| row.n_elements()).collect();
        for clique in cliques.iter() {
            let clique_size = clique.n_elements();
            let vertii = clique.elements();
            //get common neighbours of cliques
            let mut common_neighbours: Bitvec;
            match vertii.first() {
                Some(first) => {
                    common_neighbours = self.matrix[*first].clone();
                    for vertex in vertii.iter().skip(1) {
                        common_neighbours.intersection_with(&self.matrix[*vertex]);
                    }
                }
                None => continue,
            }
            match vertii.last() {
                Some(vertex) => {
                    for neighbour in common_neighbours.elements_from(*vertex).into_iter() {
                        if degrees[neighbour] < clique_size {
                            continue;
                        }
                        if self.matrix[neighbour].contains_all(&vertii) {
                            let mut new_clique = Bitvec::from_vector(&vertii, self.capacity);
                            new_clique.insert(neighbour);
                            new_cliques.push(new_clique);
                        }
                    }
                }
                None => continue,
            }
        }
        new_cliques
    }
}

/// Creates the boundary matrix given cliques of size N to size N+1
/// It is the less efficent version of the next clique algorithm since it needs
/// to account for all N cliques that generate the N+1 one. Also has to use a hashmap
/// to store these repetitions with takes away from the efficiency of Nimbus.
/// A bit of a defeat nonetheless.
pub trait BoundaryMatrix {
    fn boundary_matrix(&self, cliques: &[Bitvec]) -> (Vec<Bitvec>, Vec<Bitvec>);
}

impl BoundaryMatrix for Bitmatrix {
    fn boundary_matrix(&self, cliques: &[Bitvec]) -> (Vec<Bitvec>, Vec<Bitvec>) {
        let mut clique_map: HashMap<Vec<usize>, Vec<usize>> = HashMap::new();
        let degrees: Vec<usize> = self.matrix.iter().map(|row| row.n_elements()).collect();
        for (index, clique) in cliques.iter().enumerate() {
            let clique_size = clique.n_elements();
            let vertii = clique.elements();
            //get common neighbours of cliques
            let mut common_neighbours: Bitvec;
            match vertii.first() {
                Some(first) => {
                    common_neighbours = self.matrix[*first].clone();
                    for vertex in vertii.iter().skip(1) {
                        common_neighbours.intersection_with(&self.matrix[*vertex]);
                    }
                }
                None => continue,
            }
            for neighbour in common_neighbours.elements().into_iter() {
                if degrees[neighbour] < clique_size {
                    continue;
                }
                if self.matrix[neighbour].contains_all(&vertii) {
                    let mut element = vertii.clone();
                    element.push(neighbour);
                    element.sort();
                    let entry = clique_map.entry(element).or_insert(Vec::new());
                    entry.push(index);
                }
            }
        }
        let mut new_cliques = Vec::new();
        let mut matrix = Vec::new();
        for (key, value) in clique_map.into_iter() {
            //add the n+1 cliques
            new_cliques.push(Bitvec::from_vector(&key, self.capacity));
            //add the matrix
            matrix.push(Bitvec::from_vector(&value, cliques.len()));
        }
        (new_cliques, matrix)
    }
}
