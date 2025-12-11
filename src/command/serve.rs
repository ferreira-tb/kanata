use super::Command;
use crate::item::Item;
use anyhow::Result;
use axum::Router;
use axum::extract::Path;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use clap::Args;
use http::StatusCode;
use local_ip_address::local_ip;
use maud::{DOCTYPE, html};
use std::net::{IpAddr, Ipv4Addr, SocketAddrV4};
use tokio::fs;
use tokio::net::TcpListener;

#[derive(Args, Debug)]
pub struct Serve {
  #[arg(long)]
  port: Option<u16>,
}

impl Command for Serve {
  async fn execute(self) -> Result<()> {
    let router = Router::new()
      .route("/", get(home))
      .route("/download/{name}", get(download));

    let ip = Ipv4Addr::new(0, 0, 0, 0);
    let port = self.port.unwrap_or(63000);
    let addr = SocketAddrV4::new(ip, port);

    let IpAddr::V4(local) = local_ip()? else { unreachable!() };
    println!("Listening on: {local}:{port}");

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, router.into_make_service()).await?;

    Ok(())
  }
}

async fn home() -> Response {
  let Ok(items) = Item::read_dir().await else {
    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
  };

  let html = html! {
    (DOCTYPE)
    html lang="en" {
      head {
        meta charset="utf-8";
        meta name="viewport" content="width=device-width, initial-scale=1.0";
        style { (include_str!("../../style/main.css")) }
        title { "Kanata" }
      }
      body {
        ol {
          @for item in &items {
            ul {
              @let url = format!("/download/{}", item.encode());
              a href=(url) { (item.name()) }
            }
          }
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
