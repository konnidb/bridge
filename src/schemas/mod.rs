use std::{collections::HashMap, error::Error};

#[derive(Debug)]
pub struct Schema<'a> {
    schema_name: String,
    properties: HashMap<&'a String, &'a ValueMetaData<'a>>,
    index_properties: HashMap<&'a String, &'a ValueMetaData<'a>>
}

impl<'a> Schema<'a> {
    pub fn new(
        schema_name: String,
        properties: HashMap<&'a String, &'a ValueMetaData<'a>>,
        index_properties: HashMap<&'a String, &'a ValueMetaData<'a>>,
    ) -> Self {
        Self {
            schema_name,
            properties,
            index_properties
        }
    }

    fn can_remove_property(&self, prop_name: &'a String) -> bool {
        self.index_properties.contains_key(prop_name)
    }

    pub fn remove_property(&mut self, prop_name: &'a String) -> Result<bool, &'static str> {
        if !self.can_remove_property(prop_name) {
            return Err("Could not remove property");
        }
        self.properties.remove(prop_name);
        Ok(true)
    }

    pub fn add_property(&mut self, prop_name: &'a String, meta_data: &'a ValueMetaData<'a>) -> Result<bool, String> {
        if self.properties.contains_key(prop_name) {
            return Err(std::fmt::format(format_args!("Already existing {} property exists", &prop_name)));
        }
        if meta_data.is_index {
            self.index_properties.insert(prop_name, meta_data);
        }
        self.properties.insert(prop_name, meta_data);
        Ok(true)
    }

    pub fn alter_property(&mut self, prop_name: &'a String, meta_data: &'a ValueMetaData<'a>) -> Result<bool, String> {
        let mut remove_idx = false;
        if self.index_properties.contains_key(prop_name) {
            if meta_data.is_index && meta_data.allow_null {
                return Err("Cannot accept null on index properties".to_string());
            }
            if !meta_data.is_index {
                remove_idx = true;
            }
        }

        let prop = self.properties[prop_name];

        return Ok(true);
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