use generate_sql::create_table_columns as ctc;
use generate_sql::helper::gen_mysql_helper as gmh;
use std::env;

fn main() {
    match env::current_dir() {
        Ok(path) => println!("当前工作路径是: {}", path.display()),
        Err(e) => println!("获取当前工作路径失败: {}", e),
    }
}
