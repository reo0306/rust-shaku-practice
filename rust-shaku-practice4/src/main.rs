use shaku::{module, Interface, Component, HasComponent};
use std::sync::Arc;

trait Inventory: Interface {
    fn is_in_stock(&self, item_id: &str) -> bool;
}

#[derive(Component)]
#[shaku(interface = Inventory)]
struct StaticInventory;

impl Inventory for StaticInventory {
    fn is_in_stock(&self, item_id: &str) -> bool {
        if item_id == "item123" {
            true
        } else {
            false
        }
    }
}

trait Notifier: Interface {
    fn notify(&self, user: &str, msg: &str);
}

#[derive(Component)]
#[shaku(interface = Notifier)]
struct ConsoleNotifier;

impl Notifier for ConsoleNotifier {
    fn notify(&self, user: &str, msg: &str) {
        println!("user: {}, msg: {}", user, msg);
    }
}

trait OrderService: Interface {
    fn order(&self, user: &str, item_id: &str);
}

#[derive(Component)]
#[shaku(interface = OrderService)]
struct OrderServiceImpl {
    #[shaku(inject)]
    inventory: Arc<dyn Inventory>,
    #[shaku(inject)]
    notification: Arc<dyn Notifier>,    
}

impl OrderService for OrderServiceImpl {
    fn order(&self, user: &str, item_id: &str) {
        let stock = self.inventory.is_in_stock(item_id);
        if stock {
            self.notification.notify(user, "is_in_stock: OK");
        }
    }
}

module! {
    MyModule {
        components = [StaticInventory, ConsoleNotifier, OrderServiceImpl],
        providers = []
    }
}

fn main() {
    let module = MyModule::builder().build();

    let service: &dyn OrderService = module.resolve_ref();
    service.order("Taro Tanaka", "item123");
}

#[cfg(test)]
mod order_service_test {
    use super::*;
    use mockall::{mock, predicate::*};
    use std::sync::Arc;

    mock! {
        pub Inventory {}
        impl Inventory for Inventory {
            fn is_in_stock(&self, item_id: &str) -> bool;
        }
    }
    mock! {
        pub Notifier {}
        impl Notifier for Notifier {
            fn notify(&self, user: &str, msg: &str);
        }
    }

    #[test]
    fn test_order_service() {
        let mut mock_inventory = MockInventory::new();
        mock_inventory
            .expect_is_in_stock()
            .with(eq("item123"))
            .times(1)
            .return_const(true);

        let mut mock_notifier = MockNotifier::new();
        mock_notifier
            .expect_notify()
            .with(eq("Hanako Sato"), eq("is_in_stock: OK"))
            .times(1)
            .return_const(());

        let order_service = OrderServiceImpl {
            inventory: Arc::new(mock_inventory),
            notification: Arc::new(mock_notifier)
        };

        order_service.order("Hanako Sato", "item123");
    }
}