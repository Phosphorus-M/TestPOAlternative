use rbdc::{datetime::{FastDateTime}};
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Tests {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub author_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted:Option<u8>,
    pub created: Option<FastDateTime>
}
crud!(Tests {});
impl_select!(Tests{select_by_id(id:i64) -> Option => "`where id = #{id} limit 1`"});
impl_select_page!(Tests{select_page() => "`where deleted != 1 `"});