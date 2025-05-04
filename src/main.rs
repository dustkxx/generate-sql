/// Reads an Excel file and extracts table base information and create table column details.
///
/// # Arguments
///
/// * `file_path` - A `PathBuf` representing the path to the Excel file.
///
/// # Returns
///
/// A tuple containing:
/// - `Vec<TableBaseInfo>`: A vector of `TableBaseInfo` structs containing basic table information.
/// - `Vec<CreateTableColumns>`: A vector of `CreateTableColumns` structs containing column details.
///
/// # Panics
///
/// This function will panic if:
/// - The Excel file cannot be opened.
/// - The required worksheets ("基础信息" or "建表") are missing.
///
/// # Excel File Structure
///
/// The Excel file is expected to have the following worksheets:
/// - **基础信息**: Contains basic table information with the following columns:
///   1. `table_name` (String)
///   2. `table_comment` (String)
///   3. `ddl_type` (String)
///   4. `table_space` (String)
///   5. `createdby` (String)
/// - **建表**: Contains column details with the following columns:
///   1. `column_name` (String)
///   2. `column_type` (String)
///   3. `not_null` (String)
///   4. `default_value` (Optional<String>)
///   5. `column_comment` (String)
///
/// Rows in both worksheets are expected to have a header row, which will be skipped during processing.
///
/// # Example
///
/// ```rust
/// use std::path::PathBuf;
/// let file_path = PathBuf::from("path/to/excel/file.xlsx");
/// let (table_base_info, create_table_columns) = read_excel(file_path);
/// println!("{:?}", table_base_info);
/// println!("{:?}", create_table_columns);
/// ```
use generate_sql::create_table_columns::{CreateTableColumns, TableBaseInfo};
use generate_sql::helper::gen_mysql_helper as gmh;
use calamine::{open_workbook, Reader, Xlsx};
use std::{env, path::PathBuf};


fn read_excel(file_path: PathBuf) -> (Vec<TableBaseInfo>, Vec<CreateTableColumns>) {
let mut workbook: Xlsx<_> = open_workbook(file_path).expect("找不到模版文件");
    let mut table_base_info = Vec::new();
    let mut create_table_columns: Vec<CreateTableColumns> = Vec::new();

    if let Some(Ok(range)) = workbook.worksheet_range("基础信息") {
        for row in range.rows().skip(1) { // Skip header row
            table_base_info.push(TableBaseInfo {
                table_name: row[0].to_string(),
                table_comment: row[1].to_string(),
                ddl_type: row[2].to_string(),
                table_space: row[3].to_string(),
                createdby: row[4].to_string()
            });
        }
    }

    if let Some(Ok(range)) = workbook.worksheet_range("建表") {
        for row in range.rows().skip(1) { // Skip header row
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

fn main() {
    match env::current_dir() {
        Ok(path) => println!("当前工作路径是: {}", path.display()),
        Err(e) => println!("获取当前工作路径失败: {}", e),
    }

    // 获取项目根目录路径
    let project_root = std::env::current_dir().expect("获取当前工作路径失败");
    // 构建文件路径
    let file_path = project_root.join("data").join("exceltmp").join("gentableddl.xlsx");

    let (table_base_info, create_table_columns) = 
    read_excel(file_path);

    println!("TableBaseInfo: {:?}", table_base_info);
    println!("CreateTableColumns: {:?}", create_table_columns);
    let base_info = &table_base_info[0];
    gmh::gen_mysql_sql(
        base_info,
        &create_table_columns,
        &project_root.join("output").join("mysql.sql").to_string_lossy().to_string(),
    );

}
