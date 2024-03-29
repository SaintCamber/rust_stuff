use std::default::Default;
pub mod models{
    #[derive(PartialEq)]
    #[derive(Debug)]
    pub struct Table {
        name: String,
        columns: Vec<Column>,
    }
    #[derive(PartialEq)]
    #[derive(Debug)]
    pub struct Column {
        name: String,
        data_type: DataType,
        constraints: Vec<String>, 
        primary_key: Option<String>,
        foreign_key: Option<String>,
    }
    #[derive(PartialEq)]
    #[derive(Debug)]
    pub enum DataType {
        String,
        Char,
        Integer,
        Float,
        Boolean,
        Tuple,
        Vec,
        // additional types go here
    }
    
    impl Table {
        pub fn new(name: &str) -> TableBuilder {
            TableBuilder::default().name(name)
        }
    }
    #[derive(PartialEq)]
    #[derive(Debug)]
    pub struct TableBuilder {
        table: Table,
    }
    
    impl Default for TableBuilder {
        fn default() -> Self {
            TableBuilder {
                table: Table {
                    name: String::new(),
                    columns: Vec::new(),
                }
            }
        }
}

impl TableBuilder {
    pub fn name(mut self, name: &str) -> Self {
        self.table.name = name.to_string();
        self
    }
    
    pub fn add_column(mut self, name: &str, data_type: DataType) -> Self {
        self.table.columns.push(Column {
            name: name.to_string(),
            data_type,
            constraints: Vec::new(),
            primary_key: None,
            foreign_key: None,
        });
        self
    }
    
    pub fn build(self) -> Table {
        self.table
    }
}



#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works() {
        let users_table = Table::new("users")
            .add_column("id", DataType::Integer)
            .add_column("name", DataType::String)
            .add_column("email", DataType::String)
            .build();

        assert_eq!(
            users_table,
            Table {
                name: "users".to_string(),
                columns: vec![
                    Column {
                        name: "id".to_string(),
                        data_type: DataType::Integer,
                        primary_key:None,
                        foreign_key:None,
                        constraints:Vec::new(),
                    },
                    Column {
                        name: "name".to_string(),
                        data_type: DataType::String,
                        primary_key:None,
                        foreign_key:None,
                        constraints:Vec::new(),
                    },
                    Column {
                        name: "email".to_string(),
                        data_type: DataType::String,
                        primary_key:None,
                        foreign_key:None,
                        constraints:Vec::new(),
                    }
                ]
            }
        );
    }
}

}