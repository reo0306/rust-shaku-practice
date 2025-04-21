use axum::{
    extract::{Extension,Path, Json},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Router,
};
use shaku::{module, Component, Interface, HasComponent};
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use serde::{Deserialize};
use serde_json::json;

trait Logger: Interface {
    fn log(&self, name: &str);
}

#[derive(Component)]
#[shaku(interface = Logger)]
struct LoggerImpl;

impl Logger for LoggerImpl {
    fn log(&self, name: &str) {
        println!("Saved user: {name}");
    }
}

trait UserRepository: Interface {
    fn save(&self, name: &str);
}

#[derive(Component)]
#[shaku(interface = UserRepository)]
struct InMemoryUserRepository;

impl UserRepository for InMemoryUserRepository {
    fn save(&self, name: &str) {
        println!("Save user: {name}");
    }
}

trait AppService: Interface {
    fn execute(&self, name: &str);
}

#[derive(Component)]
#[shaku(interface = AppService)]
struct AppServiceImpl {
    #[shaku(inject)]
    logger: Arc<dyn Logger>,
    #[shaku(inject)]
    user_repository: Arc<dyn UserRepository>,
}

impl AppService for AppServiceImpl {
    fn execute(&self, name: &str) {
        self.user_repository.save(name);
        self.logger.log(name);
    }
}

module! {
    MyModule {
        components = [LoggerImpl, InMemoryUserRepository, AppServiceImpl],
        providers = [],
    }
}

#[derive(Deserialize)]
struct User {
    name: String
}

async fn handler(
    Extension(modules): Extension<Arc<MyModule>>,
    user: Json<User>,
) -> Result<impl IntoResponse, StatusCode> {
    let service: &dyn AppService = modules.resolve_ref();
    service.execute(&user.name);
    Ok(())
}

#[tokio::main]
async fn main() {
   let module = MyModule::builder().build();
   let app = Router::new()
        .route("/save", post(handler)) 
        .layer(Extension(Arc::new(module)));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    let listener = TcpListener::bind(addr)
        .await
        .unwrap_or_else(|e| panic!("failed to listen on {addr}: {e}"));

    axum::serve(listener, app)
    .await
    .unwrap_or_else(|e| panic!("failed to run `auxm::serve`: {e}"));

}
