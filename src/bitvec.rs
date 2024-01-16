use std::fmt;
use std::slice::Iter;
use std::slice::IterMut;

#[derive(Clone)]
pub struct Bitvec(Vec<u8>);

impl Bitvec {
    pub fn new(capacity: usize) -> Self {
        let n_bytes: usize = (capacity + 7) / 8;
        Bitvec(vec![0; n_bytes])
    }

    pub fn from_vector(values: &[usize], capacity: usize) -> Bitvec {
        let mut bitvec = Bitvec::new(capacity);
        for &index in values {
            bitvec.insert(index);
        }
        bitvec
    }

    pub fn iter(&self) -> Iter<'_, u8> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, u8> {
        self.0.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn n_elements(&self) -> usize {
        self.iter().map(|&byte| byte.count_ones() as usize).sum()
    }
    pub fn is_empty(&self) -> bool {
        self.n_elements() == 0
    }
    pub fn insert(&mut self, index: usize) {
        let byte_position = index / 8;
        let bit_position = index % 8;
        if byte_position < self.len() {
            // Set the bit to 1
            self.0[byte_position] |= 1 << (7 - bit_position);
        } else {
            // Handle out-of-bounds index or resize the vector if needed
            panic!("Index out of bounds");
        }
    }
    pub fn insertion(&self, index: usize) -> Bitvec {
        let mut away = self.clone();
        away.insert(index);
        away
    }
    pub fn remove(&mut self, index: usize) {
        let byte_position = index / 8;
        let bit_position = index % 8;
        if byte_position < self.len() {
            // Set the bit to 1
            self.0[byte_position] &= !(1 << (7 - bit_position));
        } else {
            // Handle out-of-bounds index or resize the vector if needed
            panic!("Index out of bounds");
        }
    }

    pub fn removal(&self, index: usize) -> Bitvec {
        let mut away = self.clone();
        away.remove(index);
        away
    }

    pub fn contains(&self, index: usize) -> bool {
        let byte_position = index / 8;
        let bit_position = index % 8;

        if byte_position < self.len() {
            // Check if the bit is set
            (self.0[byte_position] & (1 << (7 - bit_position))) != 0
        } else {
            // Out-of-bounds index is considered not contained
            false
        }
    }

    pub fn contains_all(&self, elements: &[usize]) -> bool {
        //more efficient, stops at first encounter
        for index in elements.iter() {
            if !self.contains(*index) {
                return false;
            }
        }
        return true;
    }

    pub fn intersection(&self, other: &Bitvec) -> Bitvec {
        // Ensure both Nimbus instances have the same length
        assert_eq!(self.len(), other.len(), "Vectors must have the same length");

        let result_bytes: Vec<u8> = self
            .iter()
            .zip(&other.0)
            .map(|(&byte_self, &byte_other)| byte_self & byte_other)
            .collect();

        Bitvec(result_bytes)
    }
    pub fn union(&self, other: &Bitvec) -> Bitvec {
        // Ensure both Nimbus instances have the same length
        assert_eq!(self.len(), other.len(), "Vectors must have the same length");

        let result_bytes: Vec<u8> = self
            .iter()
            .zip(&other.0)
            .map(|(&byte_self, &byte_other)| byte_self | byte_other)
            .collect();

        Bitvec(result_bytes)
    }

    pub fn difference(&self, other: &Bitvec) -> Bitvec {
        // Ensure both Nimbus instances have the same length
        assert_eq!(
            self.len(),
            other.len(),
            "Vectors instances must have the same length"
        );
        let mut result = self.clone();
        for index in other.elements().iter() {
            result.remove(*index);
        }
        result
    }

    pub fn intersection_with(&mut self, other: &Bitvec) {
        // Ensure both Nimbus instances have the same length
        assert_eq!(
            self.0.len(),
            other.0.len(),
            "Vectors must have the same length"
        );

        for (byte_self, &byte_other) in self.iter_mut().zip(&other.0) {
            *byte_self &= byte_other;
        }
    }
    pub fn union_with(&mut self, other: &Bitvec) {
        // Ensure both Nimbus instances have the same length
        assert_eq!(
            self.0.len(),
            other.0.len(),
            "Vectors must have the same length"
        );

        for (byte_self, &byte_other) in self.iter_mut().zip(&other.0) {
            *byte_self |= byte_other;
        }
    }

    pub fn elements(&self) -> Vec<usize> {
        let mut positions = Vec::new();
        let mut offset = 0;

        for byte in self.iter() {
            let byte_positions = find_set_bits_positions_in_byte(*byte);
            positions.extend(byte_positions.iter().map(|pos| pos + offset));
            offset += 8; // Move the offset to the next byte position
        }

        positions
    }

    pub fn elements_from(&self, start_index: usize) -> Vec<usize> {
        let mut positions = Vec::new();
        let mut offset = start_index / 8;

        for byte in self.iter().skip(start_index / 8) {
            let byte_positions = find_set_bits_positions_in_byte(*byte);

            positions.extend(
                byte_positions
                    .iter()
                    .map(|pos| pos + offset)
                    .filter(|&pos| pos > start_index),
            );

            offset += 8; // Move the offset to the next byte position
        }

        positions
    }

    pub fn first_index(&self) -> Option<usize> {
        for (byte_index, &byte) in self.iter().enumerate() {
            if byte != 0 {
                let bit_index = byte.leading_zeros() as usize;
                return Some(byte_index * 8 + bit_index);
            }
        }

        None
    }
    pub fn pop(&mut self) -> Option<usize> {
        match self.first_index() {
            Some(index) => {
                self.remove(index);
                Some(index)
            }
            None => None,
        }
    }
}

fn find_set_bits_positions_in_byte(byte: u8) -> Vec<usize> {
    let mut positions = Vec::new();

    let mut value = byte;
    while value != 0 {
        let index = value.leading_zeros();
        positions.push(index as usize);
        value ^= 1 << (7 - index);
    }

    positions
}

impl fmt::Display for Bitvec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let binary_strs: Vec<String> = self.iter().map(|&byte| format!("{:08b}", byte)).collect();
        write!(f, "{}", binary_strs.join(" "))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_remove() {
        let mut bitvec = Bitvec::new(16);
        assert_eq!(format!("{}", bitvec), "00000000 00000000");
        // Insert some bits
        bitvec.insert(5);
        bitvec.insert(10);
        bitvec.insert(15);
        assert_eq!(format!("{}", bitvec), "00000100 00100001");
        // Remove a bit
        bitvec.remove(10);
        assert_eq!(format!("{}", bitvec), "00000100 00000001");
    }
    #[test]
    fn test_insertion_removal() {
        let mut bitvec = Bitvec::new(16);
        assert_eq!(format!("{}", bitvec), "00000000 00000000");
        // Insert some bits
        bitvec.insert(5);
        let bitvec2 = bitvec.insertion(10);
        assert_eq!(format!("{}", bitvec), "00000100 00000000");
        assert_eq!(format!("{}", bitvec2), "00000100 00100000");
        bitvec.insert(10);
        bitvec.insert(15);
        // Remove a bit
        let bitvec3 = bitvec.removal(15);
        assert_eq!(format!("{}", bitvec3), "00000100 00100000");
        assert_eq!(format!("{}", bitvec), "00000100 00100001");
    }
    #[test]
    fn test_contains() {
        let mut bitvec = Bitvec::new(16);
        assert!(!bitvec.contains(5)); // Empty Nimbus, bit at index 5 should not be contained
        bitvec.insert(5);
        bitvec.insert(10);
        assert!(bitvec.contains(5)); // Bit at index 5 should be contained after insert
        assert!(!bitvec.contains(7)); // Bit at index 7 was not inserted and should not be contained
        assert!(bitvec.contains(10)); // Bit at index 10 should be contained after insert
        assert!(bitvec.contains_all(&[5, 10]))
    }
    #[test]
    fn test_union_and_intersection() {
        let mut bitvec1 = Bitvec::new(16);
        let mut bitvec2 = Bitvec::new(16);

        bitvec1.insert(5);
        bitvec1.insert(10);
        bitvec2.insert(10);
        bitvec2.insert(15);

        let union_bitvec = bitvec1.union(&bitvec2);
        assert_eq!(format!("{}", union_bitvec), "00000100 00100001");

        let union_bitvec = bitvec1.intersection(&bitvec2);
        assert_eq!(format!("{}", union_bitvec), "00000000 00100000");
    }
    #[test]
    fn test_union_and_intersection_inplace() {
        let mut bitvec1 = Bitvec::new(16);
        let mut bitvec2 = Bitvec::new(16);

        bitvec1.insert(5);
        bitvec1.insert(10);
        bitvec2.insert(10);
        bitvec2.insert(15);

        bitvec1.union_with(&bitvec2);
        assert_eq!(format!("{}", bitvec1), "00000100 00100001");

        bitvec1.intersection_with(&bitvec2);
        assert_eq!(format!("{}", bitvec1), "00000000 00100001");
    }
    #[test]
    fn test_indexes() {
        let mut bitvec = Bitvec::new(16);
        bitvec.insert(3);
        bitvec.insert(4);
        bitvec.insert(12);
        bitvec.insert(6);
        assert_eq!(bitvec.elements(), vec![3, 4, 6, 12]);
    }
    #[test]
    fn test_from_vector() {
        let bitvec = Bitvec::from_vector(&vec![3, 4, 12, 6], 16);
        assert_eq!(bitvec.elements(), vec![3, 4, 6, 12]);
    }
    #[test]
    fn test_n_elements() {
        let bitvec = Bitvec::from_vector(&vec![3, 4, 12, 6], 16);
        assert_eq!(bitvec.n_elements(), 4);
    }
    #[test]
    fn test_empty() {
        let bitvec = Bitvec::from_vector(&vec![3, 4, 12, 6], 16);
        assert!(!bitvec.is_empty());
        let bitvec = Bitvec::from_vector(&vec![], 16);
        assert!(bitvec.is_empty());
    }
    #[test]
    fn test_difference() {
        let bitvec1 = Bitvec::from_vector(&vec![3, 4, 12, 6], 16);
        let bitvec2 = Bitvec::from_vector(&vec![4, 5], 16);
        assert_eq!(bitvec1.difference(&bitvec2).elements(), vec![3, 6, 12]);
    }
    #[test]
    fn test_first_element() {
        let bitvec = Bitvec::from_vector(&vec![5, 4, 12, 6], 16);
        assert_eq!(bitvec.first_index(), Some(4));
    }
    #[test]
    fn test_elements_from() {
        let bitvec = Bitvec::from_vector(&vec![5, 4, 12, 6], 16);
        assert_eq!(bitvec.elements_from(5), vec![6, 12]);
    }
}
