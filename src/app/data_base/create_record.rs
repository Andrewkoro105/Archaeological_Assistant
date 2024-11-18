use std::cmp::min;
use crate::app::data_base::table::Table;
use umya_spreadsheet::Worksheet;

pub trait CreateRecord {
    fn create_start_record(&mut self, quantity: u32, data: Vec<String>);

    fn create_end_record(&mut self, quantity: u32, data: Vec<String>);

    fn create_record_from_index(&mut self, index: u32, data: Vec<String>);
}

impl CreateRecord for Worksheet {
    fn create_start_record(&mut self, quantity: u32, data: Vec<String>) {
        let start_index = self.get_start_index() as u32;
        let mut line_index = self.get_start_line_index();
        for index in ((start_index - quantity)..start_index).rev() {
            let mut new_data = data.clone();
            new_data.insert(0, index.to_string());
            let insert = {
                if line_index == 1 {
                    true
                } else {
                    !self.row_is_empty(line_index - 1)
                }
            };
            if insert {
                self.insert_new_row(&line_index, &1);
            } else {
                line_index -= 1;
            }
            self.set_row(line_index, new_data);
        }
    }

    fn create_end_record(&mut self, quantity: u32, data: Vec<String>) {
        let start_end_line_index = self.get_end_line_index();
        let start_end_index = self.get_end_index() as u32;
        for (end_line_index, end_index) in ((start_end_line_index + 1)
            ..=(start_end_line_index + quantity))
            .zip((start_end_index + 1)..=(start_end_index + quantity))
        {
            let mut new_data = data.clone();
            new_data.insert(0, end_index.to_string());
            self.set_row(end_line_index, new_data);
        }
    }

    fn create_record_from_index(&mut self, index: u32, data: Vec<String>) {
        let mut new_data = data;
        new_data.insert(0, index.to_string());

        let mut offset = 0;
        let mut is_found = true;
        let begin_line_index = (1..=index)
            .rev()
            .find_map(|new_index| {
                offset = index - new_index;
                self.get_row_index_from_index(new_index) 
            })
            .unwrap_or_else(|| {
                is_found = false;
                self.get_start_line_index()
            });
        
        

        if offset == 0 && is_found{
            self.set_row(begin_line_index, new_data);
        } else { 
            let count_empty_row = self.get_count_empty_rows(begin_line_index, is_found);
            if count_empty_row == 0{
                self.insert_row((begin_line_index as i32 + if is_found{ 1 } else { 0 }) as u32, new_data);
            } else {
                let offset = min(offset, count_empty_row) as i32;
                self.set_row((begin_line_index as i32 + if is_found{ offset } else { -offset }) as u32, new_data);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::app::data_base::{
        create_record::CreateRecord,
        table::{test::FromMatrix, Table},
    };
    use umya_spreadsheet::new_file;

    #[test]
    fn test_create_end_record() {
        {
            let mut book = new_file();
            let test_sheet = book.matrix_to_sheet(vec![]);
            test_sheet.create_end_record(3, vec!["test".to_string(), "test2".to_string()]);
            assert_eq!(
                vec![
                    vec!["1", "test", "test2"],
                    vec!["2", "test", "test2"],
                    vec!["3", "test", "test2"],
                ],
                test_sheet.get_string_matrix()
            );
        }
        {
            let mut book = new_file();
            let test_sheet = book.matrix_to_sheet(vec![
                vec!["1", "b", "c"],
                vec!["3", "b", "c"],
                vec!["5", "b", "c"],
            ]);
            test_sheet.create_end_record(2, vec!["test".to_string(), "test2".to_string()]);
            assert_eq!(
                vec![
                    vec!["1", "b", "c"],
                    vec!["3", "b", "c"],
                    vec!["5", "b", "c"],
                    vec!["6", "test", "test2"],
                    vec!["7", "test", "test2"],
                ],
                test_sheet.get_string_matrix()
            );
        }
        {
            let mut book = new_file();
            let test_sheet = book.matrix_to_sheet(vec![
                vec!["", "", ""],
                vec!["1", "b", "c"],
                vec!["3", "b", "c"],
                vec!["", "", ""],
            ]);
            test_sheet.create_end_record(5, vec!["test".to_string(), "test2".to_string()]);
            assert_eq!(
                vec![
                    vec!["", "", ""],
                    vec!["1", "b", "c"],
                    vec!["3", "b", "c"],
                    vec!["4", "test", "test2"],
                    vec!["5", "test", "test2"],
                    vec!["6", "test", "test2"],
                    vec!["7", "test", "test2"],
                    vec!["8", "test", "test2"],
                ],
                test_sheet.get_string_matrix()
            );
        }
    }

    #[test]
    fn test_create_start_record() {
        {
            let mut book = new_file();
            let test_sheet = book.matrix_to_sheet(vec![
                vec!["3", "b", "c"],
                vec!["4", "b", "c"],
                vec!["5", "b", "c"],
            ]);
            test_sheet.create_start_record(2, vec!["test".to_string(), "test2".to_string()]);
            assert_eq!(
                vec![
                    vec!["1", "test", "test2"],
                    vec!["2", "test", "test2"],
                    vec!["3", "b", "c"],
                    vec!["4", "b", "c"],
                    vec!["5", "b", "c"],
                ],
                test_sheet.get_string_matrix()
            );
        }
        {
            let mut book = new_file();
            let test_sheet = book.matrix_to_sheet(vec![
                vec!["index", "name", "info"],
                vec!["", "", ""],
                vec!["4", "b", "c"],
                vec!["5", "b", "c"],
            ]);
            test_sheet.create_start_record(2, vec!["test".to_string(), "test2".to_string()]);
            assert_eq!(
                vec![
                    vec!["index", "name", "info"],
                    vec!["2", "test", "test2"],
                    vec!["3", "test", "test2"],
                    vec!["4", "b", "c"],
                    vec!["5", "b", "c"],
                ],
                test_sheet.get_string_matrix()
            );
        }
    }

    #[test]
    fn create_record_from_index() {
        {
            let mut book = new_file();
            let test_sheet = book.matrix_to_sheet(vec![]);
            test_sheet.create_record_from_index(4, vec!["test".to_string(), "test2".to_string()]);
            assert_eq!(
                vec![
                    vec!["4", "test", "test2"],
                ],
                test_sheet.get_string_matrix()
            );
        }
        {
            let mut book = new_file();
            let test_sheet = book.matrix_to_sheet(vec![
                vec!["3", "b", "c"],
                vec!["4", "b", "c"],
                vec!["5", "b", "c"],
            ]);
            test_sheet.create_record_from_index(4, vec!["test".to_string(), "test2".to_string()]);
            assert_eq!(
                vec![
                    vec!["3", "b", "c"],
                    vec!["4", "test", "test2"],
                    vec!["5", "b", "c"],
                ],
                test_sheet.get_string_matrix()
            );
        }
        {
            let mut book = new_file();
            let test_sheet = book.matrix_to_sheet(vec![
                vec!["2", "b", "c"],
                vec!["4", "b", "c"],
                vec!["5", "b", "c"],
            ]);
            test_sheet.create_record_from_index(3, vec!["test".to_string(), "test2".to_string()]);
            assert_eq!(
                vec![
                    vec!["2", "b", "c"],
                    vec!["3", "test", "test2"],
                    vec!["4", "b", "c"],
                    vec!["5", "b", "c"],
                ],
                test_sheet.get_string_matrix()
            );
        }
        {
            let mut book = new_file();
            let test_sheet = book.matrix_to_sheet(vec![
                vec!["2", "b", "c"],
                vec!["", "", ""],
                vec!["4", "b", "c"],
                vec!["5", "b", "c"],
            ]);
            test_sheet.create_record_from_index(3, vec!["test".to_string(), "test2".to_string()]);
            assert_eq!(
                vec![
                    vec!["2", "b", "c"],
                    vec!["3", "test", "test2"],
                    vec!["4", "b", "c"],
                    vec!["5", "b", "c"],
                ],
                test_sheet.get_string_matrix()
            );
        }
        {
            let mut book = new_file();
            let test_sheet = book.matrix_to_sheet(vec![
                vec!["2", "b", "c"],
                vec!["", "", ""],
                vec!["", "", ""],
                vec!["", "", ""],
                vec!["", "", ""],
                vec!["5", "b", "c"],
                vec!["6", "b", "c"],
            ]);
            test_sheet.create_record_from_index(4, vec!["test".to_string(), "test2".to_string()]);
            assert_eq!(
                vec![
                    vec!["2", "b", "c"],
                    vec!["", "", ""],
                    vec!["4", "test", "test2"],
                    vec!["", "", ""],
                    vec!["", "", ""],
                    vec!["5", "b", "c"],
                    vec!["6", "b", "c"],
                ],
                test_sheet.get_string_matrix()
            );
        }
        {
            let mut book = new_file();
            let test_sheet = book.matrix_to_sheet(vec![
                vec!["2", "b", "c"],
                vec!["", "", ""],
                vec!["", "", ""],
                vec!["", "", ""],
                vec!["", "", ""],
                vec!["11", "b", "c"],
                vec!["12", "b", "c"],
            ]);
            test_sheet.create_record_from_index(10, vec!["test".to_string(), "test2".to_string()]);
            assert_eq!(
                vec![
                    vec!["2", "b", "c"],
                    vec!["", "", ""],
                    vec!["", "", ""],
                    vec!["", "", ""],
                    vec!["10", "test", "test2"],
                    vec!["11", "b", "c"],
                    vec!["12", "b", "c"],
                ],
                test_sheet.get_string_matrix()
            );
        }
        {
            let mut book = new_file();
            let test_sheet = book.matrix_to_sheet(vec![
                vec!["2", "b", "c"],
            ]);
            test_sheet.create_record_from_index(4, vec!["test".to_string(), "test2".to_string()]);
            assert_eq!(
                vec![
                    vec!["2", "b", "c"],
                    vec!["4", "test", "test2"],
                ],
                test_sheet.get_string_matrix()
            );
        }
        {
            let mut book = new_file();
            let test_sheet = book.matrix_to_sheet(vec![
                vec!["2", "b", "c"],
            ]);
            test_sheet.create_record_from_index(1, vec!["test".to_string(), "test2".to_string()]);
            assert_eq!(
                vec![
                    vec!["1", "test", "test2"],
                    vec!["2", "b", "c"],
                ],
                test_sheet.get_string_matrix()
            );
        }
        {
            let mut book = new_file();
            let test_sheet = book.matrix_to_sheet(vec![
                vec!["", "", ""],
                vec!["", "", ""],
                vec!["", "", ""],
                vec!["5", "b", "c"],
            ]);
            test_sheet.create_record_from_index(3, vec!["test".to_string(), "test2".to_string()]);
            assert_eq!(
                vec![
                    vec!["", "", ""],
                    vec!["3", "test", "test2"],
                    vec!["", "", ""],
                    vec!["5", "b", "c"],
                ],
                test_sheet.get_string_matrix()
            );
        }
        {
            let mut book = new_file();
            let test_sheet = book.matrix_to_sheet(vec![
                vec!["", "", ""],
                vec!["", "", ""],
                vec!["", "", ""],
                vec!["10", "b", "c"],
            ]);
            test_sheet.create_record_from_index(4, vec!["test".to_string(), "test2".to_string()]);
            assert_eq!(
                vec![
                    vec!["4", "test", "test2"],
                    vec!["", "", ""],
                    vec!["", "", ""],
                    vec!["10", "b", "c"],
                ],
                test_sheet.get_string_matrix()
            );
        }
    }
}
