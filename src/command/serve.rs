use super::Command;
use crate::item::Item;
use anyhow::Result;
use axum::Router;
use axum::extract::Path;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use clap::Args;
use http::StatusCode;
use maud::html;
use tokio::fs;
use tokio::net::TcpListener;

#[derive(Args, Debug)]
pub struct Serve;

impl Command for Serve {
  async fn execute(self) -> Result<()> {
    let router = Router::new()
      .route("/", get(home))
      .route("/download/{name}", get(download));

    let listener = TcpListener::bind("0.0.0.0:63000").await?;
    axum::serve(listener, router.into_make_service()).await?;

    Ok(())
  }
}

async fn home() -> Response {
  let Ok(items) = Item::read_dir().await else {
    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
  };

  let html = html! {
    ol {
      @for item in &items {
        ul {
          @let url = format!("/download/{}", item.encode());
          a href=(url) { (item.name()) }
        }
      }
    }
  };

  html.into_response()
}

async fn download(Path(name): Path<String>) -> Response {
  if let Ok(Some(item)) = Item::decode(name.as_bytes())
    && let Ok(true) = fs::try_exists(item.path()).await
    && let Ok(bytes) = fs::read(item.path()).await
  {
    bytes.into_response()
  } else {
    StatusCode::INTERNAL_SERVER_ERROR.into_response()
  }
}
