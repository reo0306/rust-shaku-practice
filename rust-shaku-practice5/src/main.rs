use shaku::{module, Interface, Component, HasComponent};
use std::sync::Arc;
use uuid::Uuid;
//use std::error::Error;

trait Logger: Interface {
    fn log(&self, msg: &str, id: &str);
}

trait UserService: Interface {
    fn register_name(&self, name: &str);
}

#[derive(Component)]
#[shaku(interface = Logger)]
struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&self, name: &str, id: &str) {
        println!("[LOG] Registered user:{} with ID: {}", name, id);
    }
}

/*struct UuidProvider;

impl<M: shaku::Module> Provider<M> for UuidProvider {
    type Interface = String;
    fn provide(_module: &M) -> Result<Box<String>, Box<dyn Error + 'static>> {
        Ok(Box::new(Uuid::new_v4().to_string()))
    }
}*/

#[derive(Component)]
#[shaku(interface = UserService)]
struct UserServiceImpl {
    #[shaku(inject)]
    logger: Arc<dyn Logger>,
    id: String,
}

impl UserService for UserServiceImpl {
    fn register_name(&self, name: &str) {
        self.logger.log(name, &self.id);
    }
}

module! {
    MyModule {
        components = [ConsoleLogger, UserServiceImpl],
        providers = [],
    }
}

fn main() {
    let uuid = Uuid::new_v4().to_string();

    let module = MyModule::builder()
    .with_component_parameters::<UserServiceImpl>(UserServiceImplParameters {
        id: uuid,
    })
    .build();

    let service: &dyn UserService = module.resolve_ref();

    service.register_name("Alice");
}
