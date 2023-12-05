use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use axum::http::{HeaderValue, Method};
use axum::{Router};
use axum::http::header::CONTENT_TYPE;

use crate::api::app_state::AppState;
use crate::api::containers::build_containers_router;
use crate::api::probes::build_probes_router;
use crate::api::users::build_users_router;

pub struct Server {
    main_router: Router,
    addr: SocketAddr,
}

impl Server {
    pub fn new(addr: SocketAddr, state: AppState) -> Server {
        let api_router: Router = Router::new()
            .nest("/users", build_users_router())
            .nest("/containers", build_containers_router())
            .nest("/probes", build_probes_router())
            .with_state(state);
        let main_router = Router::new()
            .nest("/api", api_router)
            .layer(
                CorsLayer::new()
                    .allow_origin("http://localhost:8080".parse::<HeaderValue>().unwrap())
                    .allow_headers([CONTENT_TYPE])
                    .allow_methods([Method::GET, Method::POST, Method::PATCH]),
            );
        Server::new_with_router(addr, main_router)
    }

    pub fn new_with_router(addr: SocketAddr, main_router: Router) -> Server {
        Server { main_router, addr }
    }

    pub async fn start(&mut self) {
        println!("listening on {}", self.addr);
        axum::Server::bind(&self.addr)
            .serve(self.main_router.clone().into_make_service())
            .await
            .unwrap()
    }
}
