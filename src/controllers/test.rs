use graphul::{
    async_trait,
    http::{resource::Resource, response::Response, StatusCode},
    Context, IntoResponse, extract::Json,
};
use rbatis::sql::PageRequest;
use rbdc::{datetime::{FastDateTime}};
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
            (StatusCode::OK, Json(tests)).into_response()
        } else {
            let Ok(page) = page.parse() else {
                return ErrorApps::ParseIntError.into_response();
            };
            let tests = Tests::select_page(&mut RB.clone(),&PageRequest::new(page, 10)).await;
            let Ok(tests) = tests else {
                return ErrorApps::Unknown.into();
            };
            (StatusCode::OK, Json(tests)).into_response()
        }

    }

    async fn post(_ctx: Context) -> Response {
        let mut value: Json<Tests> = match _ctx.payload().await {
            Ok(data) => data,
            Err(_) => return ErrorApps::JsonRejection.into_response()
        };
        value.0.created = Some(FastDateTime::now());
        let insert_result = match Tests::insert(&mut RB.clone(), &value.0).await {
            Ok(data) => data,
            Err(_) => return ErrorApps::Unknown.into_response()
        };
        value.0.id = insert_result.last_insert_id.as_i64();
        (StatusCode::CREATED, value).into_response()
    }
}

impl TestController {
    pub async fn get_by_id(_ctx: Context) -> Response {
        let Ok(id) = _ctx.params("id").parse::<i64>() else {
            return ErrorApps::IdIsRequired.into_response();
        };
        let Ok(data) = Tests::select_by_id(&mut RB.clone(), id).await else {
            return (StatusCode::INTERNAL_SERVER_ERROR, "An unexpected error occurred. Try again.").into_response();
        };
        let Some(test) = data else {
            return (StatusCode::NOT_FOUND, "Test not found").into_response();
        };
        (StatusCode::OK, Json(json!(test))).into_response()
    }

    pub async fn patch(_ctx: Context) -> Response {
        let Ok(test_id) = _ctx.params("id").parse::<i64>() else {
            return ErrorApps::IdIsRequired.into_response();
        };
        let mut value: Json<Tests> = match _ctx.payload().await {
            Ok(data) => data,
            Err(_) => return ErrorApps::JsonRejection.into_response()
        };
        if value.created.is_some() || value.id.is_some() || value.author_id.is_some() {
            return ErrorApps::FieldsAreNotAvailableToUpdate.into_response();
        }
        value.0.id = Some(test_id);
        let insert_result = match Tests::update_by_column(&mut RB.clone(), &value.0, "id").await {
            Ok(data) => data,
            Err(_) => return ErrorApps::Unknown.into_response()
        };
        value.0.id = insert_result.last_insert_id.as_i64();
        (StatusCode::NO_CONTENT, value).into_response()
    }
}