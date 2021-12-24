use std::{fs::File, string, collections::HashMap, io::Read, fmt::Error};

pub struct Index {
    id: u128,
    page: String,
    group_field: Option<u128>,
    grouper_content: Option<u128>
}

pub struct IndexHandler<'a> {
    path: &'a String,
    index_map: Option<HashMap<String, String>>
}

impl<'a> IndexHandler<'a> {
    pub fn new(path: &'a String) -> Self {
        IndexHandler { path, index_map: Some(HashMap::new()) }
    }

    pub fn load_index(&self) -> std::result::Result<String, &str> {
        let mut file = File::open("info.txt");
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
