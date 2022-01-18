use std::collections::HashMap;

use crate::schemas::Schema;

#[derive(Debug)]
pub struct Match {
    nodes: Vec<String>,
    vertices: Vec<String>,
}

#[derive(Debug)]
pub struct QueryNode<'a> {
    schema: Schema<'a>,
    properties: HashMap<String, String>,
}

impl Match {
    pub fn new(nodes: Vec<String>, vertices: Vec<String>) -> Self {
        Self { nodes, vertices }
    }

    
}

#[derive(Debug)]
pub struct Query<'a> {
    pub query: &'a String
}

impl<'a> Query<'a> {
    pub fn new(query: &'a String) -> Self {
        Self {
            query
        }
    }

    pub fn get_nodes(self) {
        let mut tokens = self.query.split(" ");
        let mut sentence_map: HashMap<&str, bool> = HashMap::new();

        for token in tokens {
            match token {
                "match" => {},
                "create" => {},
                "delete" => {},
                "update" => {},
                "with" => {},
                "|" => {},
                _ => {},
            }
        }
    }
}

