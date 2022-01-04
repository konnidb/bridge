
mod schema_str;
mod value_meta_data;
mod value_type;

use std::collections::HashMap;

pub use value_meta_data::*;
pub use value_type::*;
pub use schema_str::*;

pub fn is_numeric(val_type: &ValueType) -> bool {
    match val_type {
        ValueType::INT |ValueType::LONG |ValueType::FLOAT |ValueType::BOOLEAN => {
            true
        },
        ValueType::TEXT => false
    }
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
            false,
        );
        let value_type = meta_data.value_type;
        assert_eq!(*value_type, ValueType::INT);
    }
}
