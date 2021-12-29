use std::{fs::File, string, collections::HashMap, io::Read, fmt::Error};

pub struct Index {
    id: u128,
    page: String,
    group_field: Option<u128>,
    grouper_content: Option<u128>
}

pub struct IndexHandler {
    path: String,
    index_map: Option<HashMap<String, String>>
}

impl IndexHandler {
    pub fn new(path: String) -> Self {
        IndexHandler {
            path,
            index_map: Some(HashMap::new())
        }
    }

    pub fn load_index(&self) -> std::result::Result<String, &str> {
        let mut file = File::open(&self.path);
        match file {
            Ok(mut data) => {
                let mut contents = String::new();
                data.read_to_string(&mut contents)
                    .expect("Cannot read the file!");
                println!("File contents:\n\n{}", &contents);
                
                Ok(contents)
            },
            Err(err) => {
                Err("Cannot open File")
            }
        }
    }

    pub fn store_index(&self) {

    }

    pub fn get_next_page_id(&self) {

    }
}

#[cfg(test)]
mod tests {
    use super::{Index, IndexHandler};
    
    #[test]
    fn test_new_IndexHandler() {
        let handler = IndexHandler::new("this_is_path".to_string());
        assert_eq!(handler.path, "this_is_path");
    }
}
