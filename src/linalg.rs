use crate::bitvec::Bitvec;

fn xor_rows(dest: &mut Bitvec, src: &Bitvec, col: usize) {
    let lower_triag = col / 8;
    for (dest_byte, src_byte) in dest
        .iter_mut()
        .skip(lower_triag)
        .zip(src.iter().skip(lower_triag))
    {
        *dest_byte ^= *src_byte;
    }
}

pub fn gaussian_elimination(matrix: &mut Vec<Bitvec>, num_cols: usize) {
    let num_rows = matrix.len();

    let mut pivot_row = 0;
    for col in 0..num_cols {
        // Find all rows with ones in the pivot column
        let mut found_rows = Vec::new();
        for row in pivot_row..num_rows {
            if !matrix[row].contains(col) {
                continue;
            }
            found_rows.push(row);
        }
        match found_rows.get(0) {
            Some(row) => {
                matrix.swap(pivot_row, *row);
            }
            None => continue,
        }
        let pivot_row_slice = &matrix[pivot_row].clone();
        for row in found_rows.into_iter().skip(1) {
            xor_rows(&mut matrix[row], &pivot_row_slice, col);
        }
        pivot_row += 1;
    }
}

pub fn rank(matrix: &mut Vec<Bitvec>, num_cols: usize) -> usize {
    let num_rows = matrix.len();
    gaussian_elimination(matrix, num_cols);
    let mut count = 0;
    for row in matrix.iter().rev() {
        if row.is_empty() {
            count += 1;
        } else {
            return num_rows - count;
        }
    }
    num_rows - count
}
