use std::{collections::HashMap};

#[derive(Debug)]
pub struct Schema<'a, 'b> {
    schema_name: String,
    properties: HashMap<&'a String, &'a ValueMetaData<'b>>,
}

impl<'a, 'b> Schema<'a, 'b> {
    pub fn new(
        schema_name: String,
        properties: HashMap<&'a String, &'a ValueMetaData<'b>>,
    ) -> Self {
        Self {
            schema_name,
            properties,
        }
    }

    pub fn add_property(&mut self, prop_name: &'a String, meta_data: &'a ValueMetaData<'b>) {
        self.properties.insert(prop_name, &meta_data);
    }

    fn can_remove_property(&self, prop_name: &'a String) -> bool {
        let prop_meta_data = self.properties.get(prop_name);
        match prop_meta_data {
            Some(x) => {
                
            },
            None => {

            }
        }
        false
    }
}

#[derive(Debug)]
pub struct ValueMetaData<'a> {
    value_type: &'a ValueType,
    name: String,
    allow_null: bool,
    is_index: bool,
    is_unique: bool
}

#[derive(Debug)]
pub enum ValueType {
    INT,  // 32 BIT
    LONG, // 64 BIT
    FLOAT,
    BOOLEAN,
    TEXT,
}
