/// Generates a MySQL SQL script based on the provided table information and columns.
///
/// # Arguments
///
/// * `base_info` - A reference to the base information of the table, including its name and other metadata.
/// * `table_columns` - A vector of references to `CreateTableColumns` which define the structure of the table's columns.
/// * `output_path` - A string reference specifying the file path where the generated SQL script will be saved.
///
/// # Example
///
/// ```rust
/// use crate::create_table_columns::{CreateTableColumns, TableBaseInfo};
/// use crate::helper::gen_mysql_helper::gen_mysql_sql;
///
/// let base_info = TableBaseInfo::new("example_table", "Example description");
/// let columns = vec![
///     &CreateTableColumns::new("id", "INT", true, Some("Primary key")),
///     &CreateTableColumns::new("name", "VARCHAR(255)", false, None),
/// ];
/// let output_path = String::from("/path/to/output.sql");
///
/// gen_mysql_sql(&base_info, columns, &output_path);
/// ```
///
/// # Notes
///
/// This function does not return any value. It writes the generated SQL script
/// to the specified output file path.
use crate::create_table_columns::{CreateTableColumns, TableBaseInfo};
use std::{
    fs::{File, read_to_string},
    io::Write,
};

pub fn gen_mysql_sql(
    base_info: &TableBaseInfo,
    table_columns: Vec<&CreateTableColumns>,
    output_path: &String,
) {
    let mut column_sql = String::new();
    for column in table_columns {
        let mut column_str = String::new();
        column_str.push_str(&format!("`{}` ", column.column_name()));
        column_str.push_str(&format!("{} ", deal_column_type(&column.column_type())));
        column_str.push_str(&format!("{} ", deal_default_value(&column.default_value())));
        column_str.push_str(&format!("{} ", deal_not_null(&column.not_null())));
        column_str.push_str(&format!("COMMENT ''{}'' ,\n", column.column_comment()));
        column_sql.push_str(&column_str);
    }

    let mysql_template =
        read_to_string("data/sqltmp/mysqlTmp.sql").expect("获取 gentableddl.xlsx 文件失败");
    let mysql_sql = mysql_template
        .replace("@tableName@", base_info.table_name())
        .replace("@tableComment@", base_info.table_comment())
        .replace("@tableColumn@", &column_sql);
    File::create(output_path).expect("创建文件失败");
    let mut file = File::open(output_path).expect("打开文件失败");
    file.write_all(mysql_sql.as_bytes()).expect("写入文件失败");
}

fn deal_column_type(column_type: &String) -> String {
    // 处理列类型
    if column_type.to_uppercase() == "DATE" {
        return String::from("datetime");
    }
    if column_type.to_uppercase().contains("NUMBER") {
        return column_type.to_string().replace("NUMBER", "decimal");
    }
    if column_type.to_uppercase().contains("VARCHAR2") {
        let mut x = column_type.to_string().replace("VARCHAR2", "varchar");
        x.push_str("collate utf8_bin");
        return x;
    }

    return column_type.to_string();
}

fn deal_not_null(not_null: &String) -> String {
    // 处理非空
    if not_null == "是" || not_null == "N" {
        return String::from("NOT NULL");
    }
    String::from("")
}

fn deal_default_value(default_value: &Option<String>) -> String {
    match default_value {
        Some(value) => {
            if value.to_uppercase() == "SYSDATE" {
                return String::from("current_timestamp");
            } else if value.to_uppercase().contains("VARCHAER2")
                || value.to_uppercase().contains("CHAR")
            {
                return "default  '' ".to_string() + value + "''";
            }
            "default ".to_string() + value
        }
        None => String::from(""),
    }
}
