use umya_spreadsheet::{Cell, Spreadsheet, Worksheet};

pub trait Table {
    fn new(book: &Spreadsheet) -> &Self;

    fn new_mut(book: &mut Spreadsheet) -> &mut Self;

    fn get_row<'row, 'sheet: 'row>(&'sheet self, row_num: u32) -> Vec<&'row Cell>;

    fn get_row_mut<'row, 'sheet: 'row>(&'sheet mut self, row_num: u32) -> Vec<&'row mut Cell>;

    fn get_matrix<'row, 'sheet: 'row>(&'sheet self) -> Vec<Vec<&'row Cell>>;

    fn get_string_matrix<'row, 'sheet: 'row>(&'sheet self) -> Vec<Vec<String>>;

    fn get_row_index_from_index(&self, row_num: u32) -> Option<u32>;

    fn skips(&self) -> Vec<u32>;

    fn get_end_index(&self) -> f64;

    fn get_end_line(&self) -> Vec<&Cell>;

    fn get_end_line_index(&self) -> u32;

    fn get_start_index(&self) -> f64;

    fn get_start_line(&self) -> Vec<&Cell>;

    fn get_start_line_index(&self) -> u32;

    fn get_begin_cell_in_row<'cell, 'sheet: 'cell>(
        &'sheet self,
        row_num: u32,
    ) -> Option<&'cell Cell>;

    fn get_end_cell_in_row<'cell, 'sheet: 'cell>(&'sheet self, row_num: u32)
        -> Option<&'cell Cell>;

    fn insert_row(&mut self, row_num: u32, cells: Vec<String>);

    fn set_row(&mut self, row_num: u32, cells: Vec<String>);

    fn row_is_empty(&self, row_num: u32) -> bool;

    fn get_count_empty_rows(&self, line_index: u32, top: bool) -> u32;
}

impl Table for Worksheet {
    fn new(book: &Spreadsheet) -> &Self {
        book.get_sheet(&0).expect("Can not get sheet")
    }

    fn new_mut(book: &mut Spreadsheet) -> &mut Self {
        book.get_sheet_mut(&0).expect("Can not get sheet")
    }

    fn get_row<'row, 'sheet: 'row>(&'sheet self, row_num: u32) -> Vec<&'row Cell> {
        let mut result = self.get_collection_by_row(&row_num);
        result.sort_by(|cell1, cell2| {
            cell1
                .get_coordinate()
                .get_col_num()
                .cmp(&cell2.get_coordinate().get_col_num())
        });
        result
    }

    fn get_row_mut<'row, 'sheet: 'row>(&'sheet mut self, row_num: u32) -> Vec<&'row mut Cell> {
        let mut result = self
            .get_cell_collection_mut()
            .into_iter()
            .filter(|cell| *cell.get_coordinate().get_row_num() == row_num)
            .collect::<Vec<_>>();
        result.sort_by(|cell1, cell2| {
            cell1
                .get_coordinate()
                .get_col_num()
                .cmp(&cell2.get_coordinate().get_col_num())
        });
        result
    }

    fn get_matrix<'row, 'sheet: 'row>(&'sheet self) -> Vec<Vec<&'row Cell>> {
        let nums = self
            .get_row_dimensions()
            .into_iter()
            .map(|cell| *cell.get_row_num());
        (nums.clone().min().unwrap()..=nums.max().unwrap()).map(|num| self.get_row(num)).collect()
    }

    fn get_string_matrix<'row, 'sheet: 'row>(&'sheet self) -> Vec<Vec<String>> {
        self.get_matrix()
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| cell.get_value().to_string())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }

    fn get_row_index_from_index(&self, index: u32) -> Option<u32> {
        self.get_collection_by_column(&1)
            .iter()
            .find(|cell| cell.get_value_number().unwrap_or(0.) == index as f64)
            .map(|cell| *cell.get_coordinate().get_row_num())
    }

    fn skips(&self) -> Vec<u32> {
        let nums = self
            .get_row_dimensions()
            .iter()
            .filter_map(|row| {
                self.get_begin_cell_in_row(*row.get_row_num()).map(|cell| {
                    cell.get_value_number()
                        .map(|num| num as u32)
                        .unwrap_or(0)
                })
            })
            .collect::<Vec<_>>();

        (*nums.iter().min().unwrap_or(&0)..*nums.iter().max().unwrap_or(&0))
            .filter(|&i| nums.iter().find(|&&num| num == i).is_none())
            .collect()
    }

    fn get_end_index(&self) -> f64 {
        self.get_value_number((1, self.get_end_line_index()))
            .unwrap_or(0.0)
    }

    fn get_end_line(&self) -> Vec<&Cell> {
        self.get_row(self.get_end_line_index())
    }

    fn get_end_line_index(&self) -> u32 {
        let mut result = 0;
        for row in self.get_row_dimensions() {
            let cell_value = self.get_collection_by_row(row.get_row_num());
            let cell_value = cell_value
                .iter()
                .find_map(|cell| {
                    (*cell.get_coordinate().get_col_num() == 1)
                        .then_some(cell.get_value().to_string())
                })
                .unwrap_or("".to_string());
            if !cell_value.is_empty() && result < *row.get_row_num() {
                result = *row.get_row_num()
            }
        }
        result
    }

    fn get_start_index(&self) -> f64 {
        self.get_value_number((1, self.get_start_line_index()))
            .unwrap_or(0.0)
    }

    fn get_start_line(&self) -> Vec<&Cell> {
        self.get_row(self.get_start_line_index())
    }

    fn get_start_line_index(&self) -> u32 {
        let mut result = None;
        for num_row in self
            .get_row_dimensions()
            .iter()
            .map(|row| *row.get_row_num())
        {
            if !self.get_collection_by_row(&num_row).is_empty()
                && self.get_begin_cell_in_row(num_row).unwrap().get_value_number().is_some()
                && (result.is_none() || num_row < result.unwrap())
            {
                result = Some(num_row);
            }
        }

        result.unwrap_or_else(|| self.get_end_line_index())
    }

    fn get_begin_cell_in_row<'cell, 'sheet: 'cell>(
        &'sheet self,
        row_num: u32,
    ) -> Option<&'cell Cell> {
        self.get_collection_by_row(&row_num)
            .into_iter()
            .min_by(|cell1, cell2| {
                cell1
                    .get_coordinate()
                    .get_col_num()
                    .cmp(&cell2.get_coordinate().get_col_num())
            })
    }

    fn get_end_cell_in_row<'cell, 'sheet: 'cell>(
        &'sheet self,
        row_num: u32,
    ) -> Option<&'cell Cell> {
        self.get_collection_by_row(&row_num)
            .into_iter()
            .max_by(|cell1, cell2| {
                cell1
                    .get_coordinate()
                    .get_col_num()
                    .cmp(&cell2.get_coordinate().get_col_num())
            })
    }

    fn insert_row(&mut self, row_num: u32, cells: Vec<String>) {
        self.insert_new_row(&row_num, &1);
        self.set_row(row_num, cells);
    }

    fn set_row(&mut self, row_num: u32, cells: Vec<String>) {
        for (str_field, i) in cells.iter().zip(1..=cells.len() as u32) {
            self.get_cell_mut((i, row_num)).set_value(str_field);
        }
    }

    fn row_is_empty(&self, row_num: u32) -> bool {
        self.get_begin_cell_in_row(row_num).map(|cell| cell.get_value().is_empty()).unwrap_or(true)
    }

    fn get_count_empty_rows(&self, line_index: u32, top: bool) -> u32 {
        let mut result = 0;
        let line_index_iter: Box<dyn Iterator<Item = u32>> = if top { 
            Box::new((line_index + 1)..self.get_end_line_index())
        } else {
            Box::new((1..line_index).rev())
        };

        for new_line_index in line_index_iter {
            if self.row_is_empty(new_line_index) {
                result += 1;
            } else { 
                break;
            }
        }
        result
    }
}

#[cfg(test)]
pub(in crate::app::data_base) mod test {
    use super::Table;
    use sugar::hashmap;
    use umya_spreadsheet::{new_file, Spreadsheet, Worksheet};

    pub trait FromMatrix {
        fn matrix_to_sheet(&mut self, matrix: Vec<Vec<&str>>) -> &mut Worksheet;
    }

    impl FromMatrix for Spreadsheet {
        fn matrix_to_sheet(&mut self, matrix: Vec<Vec<&str>>) -> &mut Worksheet {
            let result = self.new_sheet("test").unwrap();
            for (row, y) in matrix.iter().zip(1..=matrix.len() as u32) {
                for (value, x) in row.iter().zip(1..=row.len() as u32) {
                    result.get_cell_mut((x, y)).set_value(value.to_string());
                }
            }
            result
        }
    }
    
    #[test]
    fn get_string_matrix() {
        assert_eq!(
            vec![
                vec!["", "", ""],
                vec![],
                vec!["", "", ""],
                vec!["2", "b", "c"],
                vec!["", "b", "c"],
                vec!["", "b", "c"],
            ],
            new_file()
                .matrix_to_sheet(vec![
                    vec!["", "", ""],
                    vec![],
                    vec!["", "", ""],
                    vec!["2", "b", "c"],
                    vec!["", "b", "c"],
                    vec!["", "b", "c"],
                ])
                .get_string_matrix()
        );
    }

    #[test]
    fn get_end_line_index() {
        assert_eq!(
            2,
            new_file()
                .matrix_to_sheet(vec![
                    vec!["1", "b", "c"],
                    vec!["2", "b", "c"],
                    vec!["", "b", "c"],
                    vec!["", "b", "c"],
                ])
                .get_end_line_index()
        );
        assert_eq!(
            4,
            new_file()
                .matrix_to_sheet(vec![
                    vec!["", "b", "c"],
                    vec!["", "b", "c"],
                    vec!["1", "b", "c"],
                    vec!["2", "b", "c"],
                ])
                .get_end_line_index()
        );
        assert_eq!(
            3,
            new_file()
                .matrix_to_sheet(vec![
                    vec!["", "b", "c"],
                    vec!["2", "b", "c"],
                    vec!["1", "b", "c"],
                    vec!["", "", ""],
                ])
                .get_end_line_index()
        );
        assert_eq!(
            2,
            new_file()
                .matrix_to_sheet(vec![
                    vec!["", "", ""],
                    vec!["index", "name", "info"],
                    vec!["", "", ""],
                    vec!["", "", ""],
                ])
                .get_end_line_index()
        );
    }

    #[test]
    fn get_row() {
        let mut file = new_file();
        let sheet = file.matrix_to_sheet(vec![
            vec!["1", "b", "c"],
            vec!["2", "b", "c"],
            vec!["", "b", "c"],
            vec!["", "b", "c"],
        ]);

        assert_eq!(
            vec!["", "b", "c"],
            sheet
                .get_row(3)
                .iter()
                .map(|cell| cell.get_value().to_string())
                .collect::<Vec<_>>()
        );
        assert_eq!(
            vec!["1", "b", "c"],
            sheet
                .get_row(1)
                .iter()
                .map(|cell| cell.get_value().to_string())
                .collect::<Vec<_>>()
        );
        assert_eq!(
            Vec::<String>::new(),
            sheet
                .get_row(0)
                .iter()
                .map(|cell| cell.get_value().to_string())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn get_row_mut() {
        let mut file = new_file();
        let sheet = file.matrix_to_sheet(vec![
            vec!["1", "b", "c"],
            vec!["2", "b", "c"],
            vec!["", "b", "c"],
            vec!["", "b", "c"],
        ]);

        assert_eq!(
            vec!["", "b", "c"],
            sheet
                .get_row_mut(3)
                .iter()
                .map(|cell| cell.get_value().to_string())
                .collect::<Vec<_>>()
        );
        assert_eq!(
            vec!["1", "b", "c"],
            sheet
                .get_row_mut(1)
                .iter()
                .map(|cell| cell.get_value().to_string())
                .collect::<Vec<_>>()
        );
        assert_eq!(
            Vec::<String>::new(),
            sheet
                .get_row_mut(0)
                .iter()
                .map(|cell| cell.get_value().to_string())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn skips() {
        assert_eq!(
            vec![2, 4],
            new_file()
                .matrix_to_sheet(vec![
                    vec!["1", "b", "c"],
                    vec!["3", "b", "c"],
                    vec!["5", "b", "c"],
                ])
                .skips()
        );
        assert_eq!(
            vec![2, 4],
            new_file()
                .matrix_to_sheet(vec![
                    vec!["1", "b", "c"],
                    vec!["", "b", "c"],
                    vec!["3", "b", "c"],
                    vec!["5", "b", "c"],
                ])
                .skips()
        );
        assert_eq!(
            vec![2, 4],
            new_file()
                .matrix_to_sheet(vec![
                    vec!["", "b", "c"],
                    vec!["1", "b", "c"],
                    vec!["", "b", "c"],
                    vec!["5", "b", "c"],
                    vec!["3", "b", "c"],
                    vec!["", "b", "c"],
                ])
                .skips()
        );
    }

    #[test]
    fn get_start_line_index() {
        assert_eq!(
            1,
            new_file()
                .matrix_to_sheet(vec![
                    vec!["1.", "b", "c"],
                    vec!["3.", "b", "c"],
                    vec!["5.", "b", "c"],
                ])
                .get_start_line_index()
        );
        assert_eq!(
            2,
            new_file()
                .matrix_to_sheet(vec![
                    vec!["", "", ""],
                    vec!["3", "b", "c"],
                    vec!["5", "b", "c"],
                ])
                .get_start_line_index()
        );
        assert_eq!(
            3,
            new_file()
                .matrix_to_sheet(vec![
                    vec!["", "", ""],
                    vec!["index", "name", "info"],
                    vec!["1", "b", "c"],
                    vec!["2", "b", "c"],
                ])
                .get_start_line_index()
        );
    }
    
    #[test]
    fn row_is_empty() {
        let mut book = new_file();
        let sheet = book
            .matrix_to_sheet(vec![
                vec!["", "", ""],
                vec!["", "name", "info"],
                vec!["1", "b", "c"],
                vec!["2", "b", "c"],
            ]);
        assert!(sheet.row_is_empty(0));
        assert!(sheet.row_is_empty(1));
        assert!(sheet.row_is_empty(2));
        assert!(!sheet.row_is_empty(3));
    }

    #[test]
    fn get_count_empty_rows() {
        assert_eq!(
            2,
            new_file()
                .matrix_to_sheet(vec![
                    vec!["1", "b", "c"],
                    vec!["", "", ""],
                    vec!["", "", ""],
                    vec!["2", "b", "c"],
                ])
                .get_count_empty_rows(1, true)
        );
        assert_eq!(
            0,
            new_file()
                .matrix_to_sheet(vec![
                    vec!["1", "b", "c"],
                    vec!["", "", ""],
                    vec!["", "", ""],
                    vec!["2", "b", "c"],
                ])
                .get_count_empty_rows(1, false)
        );
        assert_eq!(
            0,
            new_file()
                .matrix_to_sheet(vec![
                    vec!["1", "b", "c"],
                    vec!["", "", ""],
                    vec!["", "", ""],
                    vec!["2", "b", "c"],
                ])
                .get_count_empty_rows(4, true)
        );
        assert_eq!(
            2,
            new_file()
                .matrix_to_sheet(vec![
                    vec!["1", "b", "c"],
                    vec!["", "", ""],
                    vec!["", "", ""],
                    vec!["2", "b", "c"],
                ])
                .get_count_empty_rows(4, false)
        );
    }
}
