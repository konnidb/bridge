use std::{collections::HashMap};

#[derive(Debug)]
pub struct Schema<'a> {
    schema_name: String,
    properties: HashMap<&'a String, &'a ValueMetaData<'a>>,
}

impl<'a> Schema<'a> {
    pub fn new(
        schema_name: String,
        properties: HashMap<&'a String, &'a ValueMetaData<'a>>,
    ) -> Self {
        Self {
            schema_name,
            properties,
        }
    }

    pub fn add_property(&mut self, prop_name: &'a String, meta_data: &'a ValueMetaData<'a>) {
        self.properties.insert(prop_name, meta_data);
    }

    fn can_remove_property(&self, prop_name: &'a String) -> bool {
        let prop_meta_data = self.properties.get(prop_name);
        if let Some(x) = prop_meta_data {
            
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

impl<'a> ValueMetaData<'a> {
    pub fn new(value_type: &'a ValueType, name: String, allow_null: bool, is_index: bool, is_unique: bool) 
        -> Self { Self { value_type, name, allow_null, is_index, is_unique } }
}

#[derive(Debug, PartialEq)]
pub enum ValueType {
    INT,  // 32 BIT
    LONG, // 64 BIT
    FLOAT,
    BOOLEAN,
    TEXT,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ValueMetaData_new() {
        let meta_data = ValueMetaData::new(
            &ValueType::INT,
            "HelloTest".to_string(),
            false,
            false,
            false
        );
        let value_type = meta_data.value_type;
        assert_eq!(*value_type, ValueType::INT);
    }
}