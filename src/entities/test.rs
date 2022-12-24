use rbdc::datetime::FastDateTime;
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tests {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub author_id: i64,
    pub deleted:bool,
    pub created: FastDateTime
}
crud!(Tests {});
impl_select!(Tests{select_by_id(id:i64) -> Option => "`where id = #{id} limit 1`"});
impl_select_page!(Tests{select_page() => "`where deleted = false `"});