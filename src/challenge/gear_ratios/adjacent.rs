pub struct Adjacent<'a, T> {
    matrix: &'a Vec<Vec<T>>,
    offset_cells: Vec<(usize, usize)>,
    offset_index: usize,
}

impl<'a, T> Adjacent<'a, T> {
    pub fn new(matrix: &'a Vec<Vec<T>>, cell: (usize, usize)) -> Self {
        let (start_row, start_col) = cell;
        let offsets: Vec<(isize, isize)> = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        let offset_cells: Vec<(usize, usize)> = offsets
            .into_iter()
            .map(|(offset_row, offset_col)| {
                (
                    start_row as isize + offset_row,
                    start_col as isize + offset_col,
                )
            })
            .filter(|&(offset_row, offset_col)| offset_row >= 0 && offset_col >= 0)
            .map(|(a, b)| (a as usize, b as usize))
            .collect();

        Adjacent {
            matrix,
            offset_cells,
            offset_index: 0,
        }
    }
}

impl<'a, T: Copy> Iterator for Adjacent<'a, T> {
    type Item = (T, usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        while self.offset_index < self.offset_cells.len() {
            let (row, col) = self.offset_cells[self.offset_index];
            self.offset_index += 1;

            if let Some(matrix_row) = self.matrix.get(row) {
                if let Some(matrix_cell) = matrix_row.get(col) {
                    return Some((matrix_cell.clone(), row, col));
                }
            }
        }
        return None;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_matrix() -> Vec<Vec<u8>> {
        vec![
            vec![1, 2, 3, 4, 5],
            vec![1, 2, 3, 4, 5],
            vec![1, 2, 3, 4, 5],
            vec![1, 2, 3, 4, 5],
        ]
    }

    #[test]
    fn ch03_adjacent_all_values() {
        let matrix = get_matrix();

        let adjacent = Adjacent::new(&matrix, (2, 2));
        let collected_iter: Vec<u8> = adjacent.map(|(ch, _, _)| ch).collect();

        let expected = vec![2, 3, 4, 2, 4, 2, 3, 4];
        assert_eq!(collected_iter, expected);
    }

    #[test]
    fn ch03_adjacent_edge() {
        let matrix = get_matrix();
        let adjacent = Adjacent::new(&matrix, (0, 1));
        let collected_iter: Vec<u8> = adjacent.map(|(ch, _, _)| ch).collect();

        let expected = vec![1, 3, 1, 2, 3];
        assert_eq!(collected_iter, expected);
    }
}
