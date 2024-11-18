mod create_record;
pub(super) mod sort_cells;
pub mod table;
use create_record::CreateRecord;

use crate::app::settings::insert_methods::{InsertMethods, InsertMethodsData, StartEnd};
use crate::app::settings::print_settings::PrintSettings;
use std::path::Path;
use table::Table;
use umya_spreadsheet::*;

pub struct DataBase {
    book: Spreadsheet,
}

impl DataBase {
    pub fn get_sheet(&self) -> &Worksheet {
        Worksheet::new(&self.book)
    }

    pub fn get_sheet_mut(&mut self) -> &mut Worksheet {
        Worksheet::new_mut(&mut self.book)
    }

    pub fn create_record(
        path: &Path,
        print_settings: &PrintSettings,
        quantity: u32,
        data: Vec<String>,
        insert_methods_data: &InsertMethodsData,
    ) {
        let mut data_base = DataBase::from(path);
        let sheet = data_base.get_sheet_mut();
        match insert_methods_data.insert_methods.clone() {
            InsertMethods::StartEnd => match insert_methods_data.start_end {
                StartEnd::Start => sheet.create_start_record(quantity, data),
                StartEnd::End => sheet.create_end_record(quantity, data),
            },
            InsertMethods::Input => sheet.create_record_from_index(insert_methods_data.input, data),
            InsertMethods::AutoInsert => {
                sheet.create_record_from_index(insert_methods_data.auto_insert, data)
            }
        };
        
        data_base.save(path)
    }

    pub fn save(&self, path: &Path) {
        writer::xlsx::write(&self.book, path).unwrap()
    }
}

impl From<&Path> for DataBase {
    fn from(path: &Path) -> Self {
        Self {
            book: {
                if path.exists() {
                    reader::xlsx::read(path)
                        .expect(format!("can not read ({})", path.to_string_lossy()).as_str())
                } else {
                    new_file()
                }
            },
        }
    }
}
