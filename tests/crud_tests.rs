//! Run with
//!
//! ```not_rust
//! cargo test --features=sea-orm/sqlx-sqlite,sea-orm/runtime-tokio --test crud_tests
//! ```

pub mod common;

use axum::body::Body;
use axum::http::{self, Request, StatusCode};
use axum::Router;
use common::*;
use http_body_util::BodyExt;
use sea_orm::ActiveValue::NotSet;
use sea_orm::{IntoActiveModel, Set};
use sea_orm_crud::*;
use tower::ServiceExt;

use common::dto::user::*;

#[tokio::test]
async fn main() -> Result<(), anyhow::Error> {
  let ctx = TestContext::new("crud_tests").await;

  schema::create_tables(&ctx).await?;

  let app = router::new(&ctx);

  let _username = create_user(&app).await?;

  let resps = [
    app.clone().oneshot(Request::builder().uri("/users").body(Body::empty())?).await?,
    app.clone().oneshot(Request::builder().uri("/posts").body(Body::empty())?).await?,
    app.clone().oneshot(Request::builder().uri("/comments").body(Body::empty())?).await?,
  ];

  assert_eq!(resps[0].status(), StatusCode::OK);
  assert_eq!(resps[1].status(), StatusCode::OK);
  assert_eq!(resps[2].status(), StatusCode::OK);

  // let new_user = create_user_active_model();
  // schema::users::Entity::insert(new_user).exec(&ctx.db_conn).await?;

  // let user = schema::users::Entity::find_by_id("ahas")
  //   .into_model::<GetUser>()
  //   .one(&ctx.db_conn)
  //   .await
  //   .expect("could not find user");

  // assert!(user.is_some());

  ctx.delete().await;

  Ok(())
}

async fn create_user(app: &Router) -> Result<String, anyhow::Error> {
  let body = dto::user::CreateUser {
    username: "ahas".to_owned(),
    age: Nullable::Null,
    role: "admin".to_owned(),
    is_active: Optional(None),
  };

  assert_eq!(
    body.clone().into_active_model(),
    schema::users::ActiveModel {
      username: Set("ahas".to_owned()),
      age: Set(None),
      role: Set("admin".to_owned()),
      is_active: NotSet,
      ..Default::default()
    }
  );

  let req_body = Body::from(serde_json::to_vec(&body)?);
  let resp = app
    .clone()
    .oneshot(
      Request::builder()
        .method(http::Method::POST)
        .uri("/users")
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(req_body)?,
    )
    .await?;

  assert_eq!(resp.status(), StatusCode::CREATED);

  let resp_body_bytes = resp.into_body().collect().await?.to_bytes();
  let resp_body: UserCreated = serde_json::from_slice(&resp_body_bytes)?;

  assert_eq!(
    resp_body.clone(),
    UserCreated {
      username: "ahas".to_string()
    }
  );

  Ok(resp_body.username)
}
