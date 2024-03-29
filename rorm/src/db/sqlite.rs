

fn generate_create_table_sql(table:&Table) -> String {
    let mut columns = Vec::new();

    for column in &table.columns {
        let data_type = match column.data_type {
            DataType::String = "TEXT",
            DataTpe::char = "TEXT"
            DataType::Integer = "INTEGER",
            DataType::Boolean = "INTEGER",
            DataType::Tuple = "TEXT",
            DataType::Vec = "TEXT"
        }
        columns.push(format!("{} {}",column.name,data_type))
    }
}