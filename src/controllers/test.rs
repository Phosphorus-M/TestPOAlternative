use chrono::Utc;
use graphul::{
    async_trait,
    http::{resource::Resource, response::Response, StatusCode},
    Context, IntoResponse, extract::Json,
};

use sea_orm::{EntityTrait, QuerySelect, ActiveValue};
use serde_json::json;

use crate::{middlewares::result_response::ErrorApps, AppState, utils::adapters::page::PageAdapter, entities::tests::{Model, self}};
use crate::entities::prelude::Tests;

pub struct TestController;

#[async_trait]
impl Resource<AppState> for TestController {

    async fn get(_ctx: Context<AppState>) -> Response {
        let page = _ctx.query("page");
        if page.is_empty() {
            let tests = Tests::find().all(&_ctx.state().db).await;
            let Ok(tests) = tests else {
                return ErrorApps::Unknown.into();
            };
            (StatusCode::OK, Json(tests)).into_response()
        } else {
            let Ok(mut page) = page.parse::<u64>() else {
                return ErrorApps::ParseIntError.into_response();
            };
            let tests = if page == 0 || page == 1 {
                page = 1;
                Tests::find().limit(10).all(&_ctx.state().db).await
            }else {
                Tests::find().limit(10).offset(page * 10).all(&_ctx.state().db).await
            };
            let Ok(tests) = tests else {
                return ErrorApps::Unknown.into();
            };
            let page_test = PageAdapter {
                items: tests,
                page,
                page_size: 10
            };
            (StatusCode::OK, Json(page_test)).into_response()
        }

    }

    async fn post(_ctx: Context<AppState>) -> Response {
        let mut value: Json<Model> = match _ctx.payload().await {
            Ok(data) => data,
            Err(_) => return ErrorApps::JsonRejection.into_response()
        };
        value.0.created = Some(Utc::now().date_naive());
        let test : tests::ActiveModel = value.0.clone().into();
        let insert_result = match Tests::insert(test).exec(&_ctx.state().db).await {
            Ok(data) => data,
            Err(_) => return ErrorApps::Unknown.into_response()
        };
        value.0.id = insert_result.last_insert_id;
        (StatusCode::CREATED, value).into_response()
    }
}

impl TestController {
    pub async fn get_by_id(_ctx: Context<AppState>) -> Response {
        let Ok(id) = _ctx.params("id").parse::<i32>() else {
            return ErrorApps::IdIsRequired.into_response();
        };
        let Ok(data) = Tests::find_by_id(id).one(&_ctx.state().db).await else {
            return (StatusCode::INTERNAL_SERVER_ERROR, "An unexpected error occurred. Try again.").into_response();
        };
        let Some(test) = data else {
            return (StatusCode::NOT_FOUND, "Test not found").into_response();
        };
        (StatusCode::OK, Json(json!(test))).into_response()
    }

    pub async fn patch(_ctx: Context<AppState>) -> Response {
        let Ok(test_id) = _ctx.params("id").parse::<i32>() else {
            return ErrorApps::IdIsRequired.into_response();
        };
        let value: Json<Model> = match _ctx.payload().await {
            Ok(data) => data,
            Err(_) => return ErrorApps::JsonRejection.into_response()
        };
        if value.created.is_some() || value.author_id.is_some() {
            return ErrorApps::FieldsAreNotAvailableToUpdate.into_response();
        }
        let test_to_update = tests::ActiveModel{
            id: ActiveValue::Unchanged(test_id),
            description: ActiveValue::Set(value.0.description),
            title: ActiveValue::Set(value.0.title),
            ..Default::default()
        };
        match Tests::update(test_to_update).exec(&_ctx.state().db).await {
            Ok(_) => (StatusCode::NO_CONTENT, "").into_response(),
            Err(_) => ErrorApps::Unknown.into_response()
        }
    }

    pub async fn delete(_ctx: Context<AppState>) -> Response {
        let Ok(test_id) = _ctx.params("id").parse::<i32>() else {
            return ErrorApps::IdIsRequired.into_response();
        };
        let test_to_delete : tests::ActiveModel = tests::ActiveModel {
            id: ActiveValue::Set(test_id),
            deleted: ActiveValue::Set(Some(1)),
            ..Default:: default()
        };
        match Tests::update(test_to_delete).exec(&_ctx.state().db).await {
            Ok(_) => (StatusCode::NO_CONTENT, "").into_response(),
            Err(_) => ErrorApps::Unknown.into_response()
        }
    }
}