pub Struct Table{
    name: String;
    columns: Vec<Column>,
}

pub Struct Column{
    name:String,
    data_type: DataType,
    constraints: Vec,
    primary_key: Option,
    foreign_key: Option,

}

pub enum DataType {
    String,
    char,
    Integer,
    Float,
    Boolean,
    Tuple,
    Vec,
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

