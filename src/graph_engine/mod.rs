use std::collections::HashMap;

use crate::schemas::Schema;

pub struct Graph<'a> {
    is_directed: bool,
    nodes: Vec<Node<'a>>,
    edges: Vec<Edge<'a>>
}

pub struct Node<'a> {
    schema: Schema<'a>,
    data: HashMap<String, String>
}

pub struct Edge<'a> {
    source: &'a Node<'a>,
    dest: &'a Node<'a>,
}
