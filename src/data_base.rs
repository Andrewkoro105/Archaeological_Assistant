use crate::settings::PrintSettings;
use std::path::Path;
use umya_spreadsheet::*;

pub struct DataBase {
    book: Spreadsheet,
}

impl DataBase {
    fn get_sheet(&self) -> &Worksheet {
        self.book.get_sheet(&0).expect("Can not get sheet")
    }

    fn get_sheet_mut(&mut self) -> &mut Worksheet {
        self.book.get_sheet_mut(&0).expect("Can not get sheet")
    }

    fn get_end_line(&self) -> Vec<&Cell> {
        let sheet = self.get_sheet();

        sheet.get_collection_by_row(&Self::get_end_line_index(&sheet))
    }

    fn get_end_line_index(sheet: &Worksheet) -> u32 {
        let mut result = 0;
        for row in sheet.get_row_dimensions() {
            let end_row = sheet.get_collection_by_row(row.get_row_num());
            let cell_value = end_row[0].get_cell_value();
            if !cell_value.is_empty() && !cell_value.get_value().to_string().is_empty() && result < *row.get_row_num() {
                result = *row.get_row_num()
            }
        }
        result
    }

    fn sort_cells<T, F>(cells: Vec<&Cell>, field_size: usize, offset: usize, map_fn: F) -> Vec<T>
    where
        T: Default + Clone,
        F: Fn(&Cell) -> (usize, T),
    {
        let size = field_size + 1;
        let mut result = vec![T::default(); size];
        let cells_iter = cells.into_iter().map(map_fn);

        for (mut i, cell) in cells_iter {

            if i >= offset {
                i -= offset;
                if i < size {
                    result[i] = cell;
                }
            }
        }

        result
    }

    pub fn get_end_data(path: &Path, field_size: usize, offset: usize) -> Vec<String> {
        let db = DataBase::from(path);
        Self::sort_cells(db.get_end_line(), field_size, offset, |cell| {
            (
                *cell.get_coordinate().get_col_num() as usize,
                cell.get_value().to_string(),
            )
        })
    }

    pub fn create_record(
        path: &Path,
        print_settings: &PrintSettings,
        quantity: usize,
        data: Vec<String>,
    ) {
        let mut db = DataBase::from(path);
        let sheet = db.get_sheet_mut();
        let end_line_index = Self::get_end_line_index(&sheet);
        for (str_field, i) in data.iter().zip(2..data.len() as u32 + 2) {
            sheet.get_cell_mut((i, end_line_index + 1)).set_value(str_field);
        }
        
        let end_index = sheet.get_value_number((1, end_line_index)).unwrap_or(0.0);
        sheet.get_cell_mut((1, end_line_index + 1)).set_value_number(end_index + 1.0);
        db.save(path);
    }

    pub fn save(&self, path: &Path) {
        writer::xlsx::write(&self.book, path).unwrap()
    }
}

impl From<&Path> for DataBase {
    fn from(path: &Path) -> Self {
        Self {
            book: reader::xlsx::read(path)
                .expect(format!("can not read ({})", path.to_string_lossy()).as_str()),
        }
    }
}
