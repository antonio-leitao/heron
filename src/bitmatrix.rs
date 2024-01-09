use crate::bitvec::Bitvec;

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
    pub fn transpose(&mut self) {
        let mut new_matrix: Vec<Bitvec> = Vec::with_capacity(self.capacity);
        for _ in 0..self.capacity {
            new_matrix.push(Bitvec::new(self.matrix.len()));
        }
        for (i, row) in self.matrix.iter().enumerate() {
            for column in row.elements() {
                new_matrix[column].insert(i);
            }
        }
        self.matrix = new_matrix;
    }
}
// NEW VERSION
pub trait FindCliques {
    // Define the methods or associated types here
    fn get_max_degree(&self, pool: &Bitvec) -> Option<usize>;
    fn find_cliques(&self) -> Vec<Vec<Option<usize>>>;
}

impl FindCliques for Bitmatrix {
    fn get_max_degree(&self, pool: &Bitvec) -> Option<usize> {
        pool.elements()
            .iter()
            .max_by_key(|u| self.matrix[**u].n_elements())
            .cloned()
    }

    fn find_cliques(&self) -> Vec<Vec<Option<usize>>> {
        let mut cliques: Vec<Vec<Option<usize>>>;
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
            None => return cliques,
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
                        cliques.push(Q.clone());
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
        cliques
    }
}

pub trait NextCliques {
    fn next_cliques(&self, cliques: &[Bitvec]);
}

impl NextCliques for Bitmatrix {
    fn next_cliques(&self, cliques: &[Bitvec]) {
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
                        if degrees[neighbour] < clique_size + 1 {
                            continue;
                        }

                        if self.matrix[neighbour].contains_all(&vertii) {
                            println!("{:?}+{}", vertii, neighbour);
                        }
                    }
                }
                None => continue,
            }
        }
    }
}

