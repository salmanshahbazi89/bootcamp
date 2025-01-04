use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    // TODO: add fields (make sure the fields are public)
    Open,
    InProgress,
    Resolved,
    Closed
}

#[derive(Debug)]
pub struct Epic {
    // TODO: add fields (make sure the fields are public)
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<i32>,

}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        // by default the status should be set to open and the stories should be an empty vector
        Epic {
            name, 
            description,
            status: Status::Open,
            stories: vec!()
        }
    }
}

#[derive(Debug)]
pub struct Story {
    // TODO: add fields (make sure the fields are public)
    pub name: String,
    pub description: String, 
    pub status: Status
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        // by default the status should be set to open
        Story {
            name,
            description,
            status: Status::Open
        }
    }
}

#[derive(Debug)]
pub struct DBState {
    // This struct represents the entire db state which includes the last_item_id, epics, and stories
    // TODO: add fields (make sure the fields are public)
    pub last_item_id: i32,
    pub epics: HashMap<i32, Epic>,
    pub stories: HashMap<i32, Story>
}