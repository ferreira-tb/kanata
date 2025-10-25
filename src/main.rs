use anyhow::Result;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<()> {
  use axum::Router;
  use kanata::app::*;
  use leptos::logging::log;
  use leptos::prelude::*;
  use leptos_axum::{LeptosRoutes, generate_route_list};

  let conf = get_configuration(None)?;
  let addr = conf.leptos_options.site_addr;
  let leptos_options = conf.leptos_options;
  let routes = generate_route_list(App);

  let app = Router::new()
    .leptos_routes(&leptos_options, routes, {
      let leptos_options = leptos_options.clone();
      move || shell(leptos_options.clone())
    })
    .fallback(leptos_axum::file_and_error_handler(shell))
    .with_state(leptos_options);

  log!("listening on http://{}", &addr);
  let listener = tokio::net::TcpListener::bind(&addr).await?;
  axum::serve(listener, app.into_make_service()).await?;

  Ok(())
}

#[cfg(not(feature = "ssr"))]
fn main() {}
