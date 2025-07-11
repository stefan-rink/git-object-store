use crate::objects::object_model::{ObjectModel, CreateObjectDto};

/**
 * Service for the object model
 */
#[derive(Clone)]
pub struct ObjectService;

impl ObjectService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn list_objects(&self) -> Vec<ObjectModel> {
        println!("Listing objects");
        return vec![];
    }

    pub fn get_object(&self, id: &String) -> Option<ObjectModel> {    
        println!("Getting object: {}", id);
        return None;
    }

    pub fn create_object(&self, object: &CreateObjectDto) -> Option<ObjectModel> {
        println!("Creating object: {}", object.name);
        return None;
    }
}