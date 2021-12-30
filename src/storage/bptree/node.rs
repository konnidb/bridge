
#[derive(Debug)]
pub struct Node<'a> {
    pub order: i32,
    pub keys: Option<&'a mut Vec<i32>>,
    pub stored_values: Option<&'a mut Vec<&'a str>>,
    pub left: Option<&'a mut Self>,
    pub right: Option<&'a mut Self>,
    pub is_leaf: bool,
}

impl<'a> Node<'a> {
    pub fn new(
        order: i32,
    ) -> Self {
        Self {
            order,
            keys: None,
            stored_values: None,
            left: None,
            right: None,
            is_leaf: true
        }
    }

    pub fn insert(&mut self, value: String) {
        
    }

    pub fn delete(&mut self, key: &i32) {

    }

    pub fn split(&mut self) {

    }

    pub fn index(&mut self, key: &i32) {
        for key in self.keys.as_ref().unwrap().iter() {
            println!("{}", &key);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new_Node() {
        let mut node = &mut Node::new(32);
        assert_eq!(node.order, 32);
    }

    fn test_new_with_data_Node() {}
}
