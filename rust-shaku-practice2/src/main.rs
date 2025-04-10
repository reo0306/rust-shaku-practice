use shaku::{module, Component, Interface, HasComponent};
use std::sync::Arc;

trait Notification: Interface {
    fn notify(&self) -> String;
}

#[derive(Component)]
#[shaku(interface = Notification)]
struct NotificationManager;

impl Notification for NotificationManager {
    fn notify(&self) -> String {
        format!("Rust is awesome!")
    }
}

trait Mailer: Interface {
    fn send_email(&self, to: &str, message: &str) -> String;
}

#[derive(Component)]
#[shaku(interface = Mailer)]
struct EmailService {
    #[shaku(default)]
    from_address: String,
}

impl Mailer for EmailService {
    fn send_email(&self, to: &str, message: &str) -> String {
        format!("[FROM: {}] To: {} - Message: {}", &self.from_address, to, message)
    }
}

module!{
    MyModule {
        components = [NotificationManager, EmailService],
        providers = []
    }
}

fn main() {
    let module = MyModule::builder()
    .with_component_parameters::<EmailService>(EmailServiceParameters{
        from_address: "noreply@example.com".to_string(),
    })
    .build();

    let notification_service: &dyn Notification = module.resolve_ref();
    println!("{}", notification_service.notify());

    let mailer_service: &dyn Mailer = module.resolve_ref();
    println!("{}", mailer_service.send_email("user@example.com", "Hello!"));
}
