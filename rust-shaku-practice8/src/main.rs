use axum::{
    extract::{Extension,Path},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use shaku::{module, Component, Interface, HasComponent};
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;

trait Logger: Interface {
    fn log(&self, msg: &str);
}

#[derive(Component)]
#[shaku(interface = Logger)]
struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&self, msg: &str) {
        println!("{}", msg);
    }
}

trait AppService: Interface {
    fn hello(&self, name: &str);
}

#[derive(Component)]
#[shaku(interface = AppService)]
struct AppServiceImpl {
    #[shaku(inject)]
    logger: Arc<dyn Logger>,
}

impl AppService for AppServiceImpl {
    fn hello(&self, name: &str) {
        self.logger.log(&format!("Hello {name}!"));
    }
}

module! {
    MyModule {
        components = [ConsoleLogger, AppServiceImpl],
        providers = [],
    }
}

async fn handler(
    Extension(modules): Extension<Arc<MyModule>>,
    Path(name): Path<String>,
    ) -> Result<impl IntoResponse, StatusCode> {
        let service: &dyn AppService = modules.resolve_ref();
        service.hello(&name);
        Ok(())
}

#[tokio::main]
async fn main() {
    let module = MyModule::builder().build();
    let app = Router::new()
        .route("/hello", get(handler))
        .route("/hello/{name}", get(handler))
        .layer(Extension(Arc::new(module)));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    let listener = TcpListener::bind(addr)
        .await
        .unwrap_or_else(|e| panic!("failed to listen on {addr}: {e}"));

    axum::serve(listener, app)
    .await
    .unwrap_or_else(|e| panic!("failed to run `auxm::serve`: {e}"));
}
