use graphul::{
    async_trait,
    http::{resource::Resource, response::Response, StatusCode},
    Context, IntoResponse, extract::Json,
};
use rbatis::sql::PageRequest;
use serde_json::json;

use crate::{entities::test::Tests, RB, middlewares::result_response::ErrorApps};

pub struct TestController;

#[async_trait]
impl Resource for TestController {
    async fn get(_ctx: Context) -> Response {
        let page = _ctx.query("page");
        if page.is_empty() {
            let tests = Tests::select_all(&mut RB.clone()).await;
            let Ok(tests) = tests else {
                return ErrorApps::Unknown.into();
            };
            (StatusCode::OK, Json(json!(tests))).into_response()
        } else {
            let Ok(page) = page.parse() else {
                return ErrorApps::ParseIntError.into_response();
            };
            let tests = Tests::select_page(&mut RB.clone(),&PageRequest::new(page, 10)).await;
            let Ok(tests) = tests else {
                return ErrorApps::Unknown.into();
            };
            (StatusCode::OK, Json(json!(tests))).into_response()
        }

    }
}

impl TestController {
    pub async fn get_by_id(_ctx: Context) -> Response {
        let Ok(id) = _ctx.params("id").parse::<i64>() else {
            return (StatusCode::BAD_REQUEST, "Id is required").into_response();
        };
        let Ok(data) = Tests::select_by_id(&mut RB.clone(), id).await else {
            return (StatusCode::INTERNAL_SERVER_ERROR, "An unexpected error occurred. Try again.").into_response();
        };
        let Some(test) = data else {
            return (StatusCode::NOT_FOUND, "Test not found").into_response();
        };
        (StatusCode::OK, Json(json!(test))).into_response()
    }
}