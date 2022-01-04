use super::value_type::ValueType;

#[derive(Debug)]
pub struct ValueMetaData<'a> {
    pub value_type: &'a ValueType,
    pub name: String,
    pub allow_null: bool,
    pub is_index: bool,
    pub is_unique: bool,
}

impl<'a> ValueMetaData<'a> {
    pub fn new(
        value_type: &'a ValueType,
        name: String,
        allow_null: bool,
        is_index: bool,
        is_unique: bool,
    ) -> Self {
        Self {
            value_type,
            name,
            allow_null,
            is_index,
            is_unique,
        }
    }
}