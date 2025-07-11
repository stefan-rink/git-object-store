use serde::{Deserialize, Serialize};
use serde_json::Value;

/**
 * Model for an object
 */
#[derive(Clone, Serialize, Deserialize)]
pub struct ObjectModel {
    /// The id of the object
    pub id: String,
    /// The name of the object
    pub name: String,
    /// The description of the object
    pub description: String,
    /// The json content of the object
    pub content: Value,
}


/**
 * DTO for creating an object
 */
#[derive(Deserialize)]
pub struct CreateObjectDto {
    /// The name of the object
    pub name: String,
    /// The description of the object
    pub description: String,
    /// The json content of the object
    pub content: Value,
}