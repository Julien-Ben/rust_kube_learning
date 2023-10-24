use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub filename: String,
    pub uri: String,
    pub timestamp: DateTime,
    pub user: ObjectId,
}
