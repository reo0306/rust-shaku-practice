use shaku::{module, Component, Interface, HasComponent};
use std::sync::Arc;
use std::fs::OpenOptions;
use std::io::Write;

trait Logger: Interface {
    fn log(&self, msg: &str) -> Result<(), Box<dyn std::error::Error>>;
}

#[derive(Component)]
#[shaku(interface = Logger)]
struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&self, msg: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("[LOG] {}", msg);
        Ok(())
    }
}

#[derive(Component)]
#[shaku(interface = Logger)]
struct FileLogger {
    file_path: String,
}

impl FileLogger {
    fn new(file_path: &str) -> std::io::Result<Self> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)?;
        writeln!(file, "--- FileLogger Initialized ---")?;
        Ok(Self {
            file_path: file_path.to_string(),
        })
    }
}

impl Logger for FileLogger {
    fn log(&self, msg: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open("log.txt");
        writeln!(file?, "[FileLogger] {}", msg);
        Ok(())
    }
}

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

trait Mailer: Interface {
    fn send_email(&self, to: &str, message: &str) -> Result<(), Box<dyn std::error::Error>>;
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
    fn send_email(&self, to: &str, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let from_address = self.config.get("from").unwrap();
        self.logger.log(&format!("from: {}, to: {}, message: {}", from_address, to, message))?;
        Ok(())
    }
}

module! {
    MyModule {
        //components = [StaticConfig, ConsoleLogger, FileLogger, EmailService],
        //components = [StaticConfig, EmailService],
        components = [StaticConfig, ConsoleLogger, EmailService],
        providers = [],
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let use_file_logger = std::env::var("USE_FILE_LOGGER")
        .map(|v| v == "1")
        .unwrap_or(false);

    let mut builder = MyModule::builder();

    if use_file_logger {
        let file_logger = FileLogger::new("log.txt")?;
        builder = builder.with_component_override::<dyn Logger>(Box::new(file_logger));
    }

    let module = builder.build();

    let email_service: &dyn Mailer = module.resolve_ref();
    let _ = email_service.send_email("practice@example.com", "Hello, World! Shaku");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::{mock, predicate::*};
    use std::sync::Arc;

    mock! {
        pub Logger {}
        impl Logger for Logger {
            fn log(&self, msg: &str);
        }
    }
    mock! {
        pub Config {}
        impl Config for Config {
            fn get(&self, key: &str) -> Option<String>;
        }
    }

    #[test]
    fn test_send_email_logs_correct_message() {
        let mut mock_logger = MockLogger::new();
        mock_logger
            .expect_log()
            .with(eq("from: support@example.com, to: user@example.com, message: Hello!"))
            .times(1)
            .return_const(());

        let mut mock_config = MockConfig::new();
        mock_config
            .expect_get()
            .times(1)
            .return_const(Some("support@example.com".to_string()));

        let mailer = EmailService {
            logger: Arc::new(mock_logger),
            config: Arc::new(mock_config),
        };

        mailer.send_email("user@example.com", "Hello!");
    }
}