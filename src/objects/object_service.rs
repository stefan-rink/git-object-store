use crate::objects::object_model::{CreateObjectDto, ObjectModel};

/**
 * Service for the object model
 */
#[derive(Clone, Copy)]
pub struct ObjectService;

impl ObjectService {
    pub const fn new() -> Self {
        Self {}
    }

    pub fn list_objects(&self) -> Vec<ObjectModel> {
        println!("Listing objects");
        vec![]
    }

    pub fn get_object(&self, id: &String) -> Option<ObjectModel> {
        println!("Getting object: {id}");
        None
    }

    pub fn create_object(&self, object: &CreateObjectDto) -> Option<ObjectModel> {
        println!("Creating object: {}", object.name);
        None
    }
}
