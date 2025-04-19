use axum::{
    extract::Extension,
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
    fn hello(&self);
}

#[derive(Component)]
#[shaku(interface = AppService)]
struct AppServiceImpl {
    #[shaku(inject)]
    logger: Arc<dyn Logger>,
}

impl AppService for AppServiceImpl {
    fn hello(&self) {
        self.logger.log("Hello from AppService!");
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
    ) -> Result<impl IntoResponse, StatusCode> {
        let service: &dyn AppService = modules.resolve_ref();
        service.hello();
        Ok(())
}

#[tokio::main]
async fn main() {
    let module = MyModule::builder().build();
    let service: &dyn AppService = module.resolve_ref();
    let app = Router::new()
        .route("/hello", get(handler))
        .layer(Extension(Arc::new(module)));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    let listener = TcpListener::bind(addr)
        .await
        .unwrap_or_else(|e| panic!("failed to listen on {addr}: {e}"));

    axum::serve(listener, app)
    .await
    .unwrap_or_else(|e| panic!("failed to run `auxm::serve`: {e}"));
}
