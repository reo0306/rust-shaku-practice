use shaku::{module, Component, Interface, HasComponent};
use std::sync::Arc;

trait MessageService: Interface {
    fn send(&self, msg: &str) -> String;
}

#[derive(Component)]
#[shaku(interface = MessageService)]
struct ConsoleMessageService;

impl MessageService for ConsoleMessageService {
    fn send(&self, msg: &str) -> String {
        format!("Sent: {}", msg)
    }
}

module! {
    MyModule {
        components = [ConsoleMessageService],
        providers = []
    }
}

fn main() {
    let module = MyModule::builder().build();
    let service: &dyn MessageService = module.resolve_ref();

    println!("{}", service.send("Hello"));
}
