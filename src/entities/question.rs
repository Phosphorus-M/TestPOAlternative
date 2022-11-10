


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Questions {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub author: String,
    pub deleted:bool,
    pub created: FastDateTime
}