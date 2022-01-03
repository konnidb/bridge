use std::{collections::btree_map::Keys, ops::Deref};

#[derive(Debug)]
pub struct Node<'a> {
    pub order: i32,
    pub keys: Option<&'a mut Vec<&'a i32>>,
    pub stored_values: Option<&'a mut Vec<&'a str>>,
    pub left: Option<&'a mut Self>,
    pub right: Option<&'a mut Self>,
    pub is_leaf: bool,
}

impl<'a> Node<'a> {
    pub fn new(order: i32) -> Self {
        Self {
            order,
            keys: None,
            stored_values: None,
            left: None,
            right: None,
            is_leaf: true,
        }
    }

    pub fn insert(&mut self, key: &'a i32, value: &'a str) {
        // if let Some(keys) = self.keys.take() {
        //     for idx in 0..self.keys.unwrap().len() {

        //     }
        //     // for idx in 0..self.keys.unwrap().len() {
        //         // let item = self.keys.unwrap().get(idx);
        //     for (idx, item) in keys.iter_mut().enumerate() {
        //         if *key == **item {
        //             if let Some(values) = self.stored_values.take() {
        //                 values.push(value);
        //                 self.stored_values = Some(values);
        //                 keys.push(key);
        //             }
        //         }
        //         if *key < **item {
        //             self.insert_key_at_idx(key, idx);
        //         }
        //         if idx + 1 == keys.len() {

        //         }
        //     }
        // } else {
        todo!()
        // }
    }

    fn insert_key_at_idx(&mut self, key: &'a i32, idx: usize) {
        let keys = self.keys.take().unwrap();
        keys.insert(idx, key);
        self.keys = Some(keys);
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
}
