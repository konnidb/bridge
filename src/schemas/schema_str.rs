use std::collections::HashMap;

use super::{ValueMetaData, ValueType, is_numeric};


#[derive(Debug)]
pub struct Schema<'a> {
    schema_name: String,
    properties: HashMap<&'a String, &'a ValueMetaData<'a>>,
    index_properties: HashMap<&'a String, &'a ValueMetaData<'a>>,
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
            index_properties,
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

    pub fn add_property(
        &mut self,
        prop_name: &'a String,
        meta_data: &'a ValueMetaData<'a>,
    ) -> Result<bool, String> {
        if self.properties.contains_key(prop_name) {
            return Err(std::fmt::format(format_args!(
                "Already existing {} property exists",
                &prop_name
            )));
        }
        if meta_data.is_index {
            self.index_properties.insert(prop_name, meta_data);
        }
        self.properties.insert(prop_name, meta_data);
        Ok(true)
    }

    pub fn alter_property(
        &mut self,
        prop_name: &'a String,
        meta_data: &'a ValueMetaData<'a>,
    ) -> Result<bool, String> {
        let mut is_idx = false;
        if self.index_properties.contains_key(prop_name) {
            if meta_data.is_index && meta_data.allow_null {
                return Err("Cannot accept null on index properties".to_string());
            }
            if !meta_data.is_index {
                is_idx = true;
            }
        }

        let prop = self.properties[prop_name];

        if is_numeric(meta_data.value_type) {
            if *prop.value_type == ValueType::TEXT {
                return Err("Cannot convert Text property to numeric.".to_string());
            }
            if *prop.value_type == ValueType::BOOLEAN {
                // TODO
                todo!("Implement migration of elements from numeric to boolean (convert non zero values to True and Zero values to lower)")
            }
            if is_numeric(prop.value_type) {
                if *prop.value_type == ValueType::FLOAT {}
                if is_idx {}
            }
        }
        if is_idx {
            self.index_properties.remove(prop_name);
        }

        return Ok(true);
    }
}
