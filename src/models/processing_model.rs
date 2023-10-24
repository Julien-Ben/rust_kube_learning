use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Processing {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub timestamp: DateTime,
    pub base_image: ObjectId,
    pub processed_image: ObjectId,
    pub user: ObjectId,
}
