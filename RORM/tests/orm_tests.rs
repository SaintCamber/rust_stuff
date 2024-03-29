#[cfg(test)]
mod tests {
    use super::*;
    use RORM::Table; // Import the Table struct from your module

    #[test]
    fn it_works() {
        let users_table = Table::new("users")
            .add_column("id", DataType::Integer)
            .add_column("name", DataType::Text)
            .add_column("email", DataType::Text)
            .build();

        assert_eq!(
            users_table,
            Table {
                name: "users".to_string(),
                columns: vec![
                    Column {
                        name: "id".to_string(),
                        data_type: DataType::Integer
                    },
                    Column {
                        name: "name".to_string(),
                        data_type: DataType::Text
                    },
                    Column {
                        name: "email".to_string(),
                        data_type: DataType::Text
                    }
                ]
            }
        );
    }
}
