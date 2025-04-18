use shaku::{module, Component, Interface, HasComponent};
use std::sync::Arc;
use std::env;

trait Logger: Interface {
    fn log(&self, message: &str);
}

#[derive(Component)]
#[shaku(interface = Logger)]
struct LoggerImpl;

impl Logger for LoggerImpl {
    fn log(&self, message: &str) {
        println!("[LOG] Laded config: ENV={}", message);
    }
}

trait ConfigLoader: Interface {
    fn load_config(&self) -> String;
}

#[derive(Component)]
#[shaku(interface = ConfigLoader)]
struct ConfigLoaderImpl;

impl ConfigLoader for ConfigLoaderImpl {
    fn load_config(&self) -> String {
        env::var("ENVIROMENT").unwrap_or_else(|_| "development".to_string())
    }
}

trait AppService: Interface {
    fn run(&self);
}

#[derive(Component)]
#[shaku(interface = AppService)]
struct AppServiceImpl{
    #[shaku(inject)]
    logger: Arc<dyn Logger>,
    #[shaku(inject)]
    config_loader: Arc<dyn ConfigLoader>,
}

impl AppService for AppServiceImpl {
    fn run(&self) {
        let config = self.config_loader.load_config();
        self.logger.log(&config);
    }
}

module! {
    MyModule {
        components = [LoggerImpl, ConfigLoaderImpl, AppServiceImpl],
        providers = [],
    }
}

fn main() {
    let module = MyModule::builder().build();

    let service: &dyn AppService = module.resolve_ref();

    service.run();
}
