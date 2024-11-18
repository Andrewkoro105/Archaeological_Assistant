use umya_spreadsheet::Cell;

pub fn sort_cells<T, F>(cells: Vec<&Cell>, field_size: usize, offset: usize, map_fn: F) -> Vec<T>
where
    T: Default + Clone,
    F: Fn(&Cell) -> (usize, T),
{
    let size = field_size;
    let mut result = vec![T::default(); size];

    for (mut i, cell) in cells.into_iter().map(map_fn) {
        if i >= offset {
            i -= offset;
            if i < size {
                result[i] = cell;
            }
        }
    }

    result
}