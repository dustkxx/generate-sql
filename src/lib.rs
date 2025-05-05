pub mod columnobj {
    pub mod create_table_columns;
}

pub mod helper {
    pub mod gen_mysql_helper;
    pub mod gen_oracle_helper;
}

use std::path::PathBuf;

use calamine::{Reader, Xlsx, open_workbook};
use chrono::Local;
pub use columnobj::create_table_columns;

use columnobj::create_table_columns::CreateTableColumns;
use columnobj::create_table_columns::TableBaseInfo;
pub use helper::gen_mysql_helper;
pub use helper::gen_oracle_helper;

pub fn run() {
    let xlsx_file_path: PathBuf = std::env::current_dir()
        .unwrap()
        .join("data/exceltmp/gentableddl.xlsx");
    // 从指定文件中读取表结构定义信息
    let (table_base_info, create_table_columns) = read_excel(xlsx_file_path);
    // 表结构基本信息
    let base_info = &table_base_info[0];
    // 脚本输出路径
    let output_path = std::env::current_dir()
        .unwrap()
        .join("output")
        .join(format!(
            "create_{}_{}_{}.sql",
            base_info.table_name(),
            base_info.createdby(),
            Local::now().format("%Y%m%d%H%M%S")
        ))
        .to_string_lossy()
        .to_string();
    // 生成脚本
    gen_mysql_helper::gen_mysql_sql(base_info, &create_table_columns, &output_path);
}

pub fn read_excel(file_path: PathBuf) -> (Vec<TableBaseInfo>, Vec<CreateTableColumns>) {
    let mut workbook: Xlsx<_> = open_workbook(file_path).expect("找不到模版文件");
    let mut table_base_info = Vec::new();
    let mut create_table_columns: Vec<CreateTableColumns> = Vec::new();

    if let Some(Ok(range)) = workbook.worksheet_range("基础信息") {
        for row in range.rows().skip(1) {
            // Skip header row
            table_base_info.push(TableBaseInfo {
                table_name: row[0].to_string(),
                table_comment: row[1].to_string(),
                ddl_type: row[2].to_string(),
                table_space: row[3].to_string(),
                createdby: row[4].to_string(),
            });
        }
    }

    if let Some(Ok(range)) = workbook.worksheet_range("建表") {
        for row in range.rows().skip(1) {
            // Skip header row
            create_table_columns.push(CreateTableColumns {
                column_name: row[0].to_string(),
                column_type: row[1].to_string(),
                not_null: row[2].to_string(),
                default_value: row.get(3).map(|v| v.to_string()),
                column_comment: row[4].to_string(),
            });
        }
    }

    (table_base_info, create_table_columns)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_excel() {
        let xlsx_file_path: PathBuf = std::env::current_dir()
            .unwrap()
            .join("data/exceltmp/gentableddl.xlsx");
        let (table_base_info, create_table_columns) = read_excel(xlsx_file_path);
        assert_eq!(table_base_info.len(), 1);
        assert!(create_table_columns.len() > 1);
    }
    #[test]
    fn test_run() {
        run();
    }
}
