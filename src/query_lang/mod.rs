use std::collections::HashMap;

use crate::schemas::Schema;

pub struct Match {
    nodes: Vec<String>,
    vertices: Vec<String>
}

#[derive(Debug)]
pub struct QueryNode<'a> {
    schema: Schema<'a>,
    properties: HashMap<String,String>
}

