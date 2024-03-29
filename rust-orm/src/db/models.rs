pub Struct Table{
    name: String;
    columns: Vec<Column>,
}

pub Struct Column{
    name:String,
    data_type: DataType,
}

pub enum DataType {
    String,
    Integer,
    Float,
    Boolean,
    Date,
    Time,
    DateTime,
    // additional types go here
}

impl Table {
    pub fn new(name: &str) -> TableBuilder{
        TableBuilder::default(),name(name);
    }
}

pub Struct TableBuilder {
    table: Table,
}

impl TableBuilder {
    pub fn name(mut self,name: $str)->Self {
        self.table.name = name.to_string();
        self
    }

    pub fn add_column(mut self,name: &str, data_type: DataType)-> Self{
        self.table.columns.push(Column {
            name: name.to_string(),
            data_type,
        });
        self
    }

    pub fn build(self)-> Table {
        self.table
    }
    }

    #[cfg(test)]
        #[tests]
        fn it_works(){
            let users_table = Table::new("users")
    .add_column("id", DataType::Integer)
    .add_column("name", DataType::Text)
    .add_column("email", DataType::Text)
    .build();

    assert_eq!(users_table, Table{
        name: "users",
        columns: [Column{name:"id",data_type: DataType::Integer},Column{name:"name",data_type = DataType::Text},Column{name:"email",data_type = DataType::Text}]
    })
        }