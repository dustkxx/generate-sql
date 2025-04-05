use derive_builder::Builder;
use getset::{Getters, Setters};

#[derive(Builder, Debug, Getters, Setters)]
pub struct CreateTableColumns {
    // the column name
    #[getset(get = "pub")]
    column_name: String,
    // the column type
    #[getset(get = "pub")]
    column_type: String,
    // the column is not null
    #[getset(get = "pub")]
    not_null: String,
    // the column's default value
    #[getset(get = "pub")]
    default_value: Option<String>,
    // the column's comment
    #[getset(get = "pub")]
    column_comment: String,
}
#[derive(Builder, Debug, Getters, Setters)]
pub struct TableBaseInfo {
    // 表名
    #[getset(get = "pub")]
    table_name: String,
    // 表注释
    #[getset(get = "pub")]
    table_comment: String,
    // 语句类型 1-建表 2加字段 3 改字段
    #[getset(get = "pub")]
    ddl_type: String,
    // 表空间
    #[getset(get = "pub")]
    table_space: String,
    // 创建人
    #[getset(get = "pub")]
    createdby: String,
}
