mod comparisons;
use std::{collections::HashMap, str::Bytes, vec};

use crate::schemas::Schema;

pub use comparisons::*;

// TODO Rethink lifecycles in this section

#[derive(Debug)]
pub struct Graph<'a> {
    is_directed: bool,
    nodes: Vec<Node<'a>>,
    edges: Vec<Edge<'a>>,
}

impl Default for Graph<'static> {
    fn default() -> Self {
        Self {
            is_directed: true,
            nodes: vec![],
            edges: vec![],
        }
    }
}

impl<'a> Graph<'a> {
    
}

#[derive(Debug)]
pub struct Node<'a> {
    schema: Schema<'a>,
    data: HashMap<String, String>,
}

#[derive(Debug)]
pub struct Edge<'a> {
    source: &'a Node<'a>,
    dest: &'a Node<'a>,
}
