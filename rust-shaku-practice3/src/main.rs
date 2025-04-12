use shaku::{module, Component, Interface, HasComponent};
use std::sync::Arc;

trait Config: Interface {
    fn get(&self, key: &str) -> Option<String>;
}

#[derive(Component)]
#[shaku(interface = Config)]
struct StaticConfig;

impl Config for StaticConfig {
    fn get(&self, key: &str) -> Option<String> {
        if key == "from" {
            Some("support@example.com".to_string())
        } else {
            None
        }
    }
}

trait Logger: Interface {
    fn log(&self, msg: &str);
}

#[derive(Component)]
#[shaku(interface = Logger)]
struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&self, msg: &str) {
        println!("[LOG] {}", msg);
    }
}

trait Mailer: Interface {
    fn send_email(&self, to: &str, message: &str);
}

#[derive(Component)]
#[shaku(interface = Mailer)]
struct EmailService {
    #[shaku(inject)]
    logger: Arc<dyn Logger>,
    #[shaku(inject)]
    config: Arc<dyn Config>,
}

impl Mailer for EmailService{
    fn send_email(&self, to: &str, message: &str) {
        let from_address = self.config.get("from").unwrap();
        self.logger.log(&format!("from: {}, to: {}, message: {}", from_address, to, message));
    }
}

module! {
    MyModule {
        components = [StaticConfig, ConsoleLogger, EmailService],
        providers = [],
    }
}

fn main() {
    let module = MyModule::builder().build();

    let email_service: &dyn Mailer = module.resolve_ref();
    email_service.send_email("practice@example.com", "Hello, World! Shaku");
}
