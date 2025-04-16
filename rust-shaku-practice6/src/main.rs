use shaku::{module, Component, Interface, Provider};
use std::error::Error;
use chrono::Local;
use std::sync::Arc;

trait TimeSource: Interface {
    fn now(&self) -> String;
}

struct SystemTimeSource;

impl TimeSource for SystemTimeSource {
    fn now(&self) -> String {
        Local::now().to_rfc3339()
    }
}

trait Logger: Interface {
    fn log(&self, message: &str);
}

#[derive(Component)]
#[shaku(interface = Logger)]
struct LoggerImpl {
    #[shaku(provide)]
    time_source: Arc<dyn TimeSource>,
}

impl Logger for LoggerImpl {
    fn log(&self, message: &str) {
        let now = self.time_source.now();
        println!("[{}] {}", now, message);
    }
}

struct TimeSourceProvider;

impl<M: shaku::Module> Provider<M> for TimeSourceProvider {
    type Interface = dyn TimeSource;

    fn provide(_module: &M) -> Result<Box<dyn TimeSource>, Box<dyn Error>> {
        Ok(Box::new(SystemTimeSource))
    }
}

module! {
    MyModule {
        components = [LoggerImpl],
        providers = [TimeSourceProvider],
    }
}

fn main() {
    let module = MyModule::builder().build();

    let logger: &dyn Logger = module.resolve_ref();
    logger.log("This is a test message.");
}
