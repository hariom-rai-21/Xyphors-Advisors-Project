use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug, Clone)]
struct Item {
    id: u32,
    name: String,
    category: String,
    quantity: u32,
    price: f64,
}

struct Inventory {
    items: HashMap<u32, Item>,
    next_id: u32,
}

impl Inventory {
    fn new() -> Self {
        Inventory {
            items: HashMap::new(),
            next_id: 1,
        }
    }

    fn add_item(&mut self, name: String, category: String, quantity: u32, price: f64) {
        let item = Item {
            id: self.next_id,
            name,
            category,
            quantity,
            price,
        };
        self.items.insert(self.next_id, item);
        self.next_id += 1;
        println!("Item added successfully!");
    }

    fn remove_item(&mut self, id: u32) {
        if self.items.remove(&id).is_some() {
            println!("Item removed successfully!");
        } else {
            println!("Item not found!");
        }
    }

    fn update_item(&mut self, id: u32, quantity: u32, price: f64) {
        if let Some(item) = self.items.get_mut(&id) {
            item.quantity = quantity;
            item.price = price;
            println!("Item updated successfully!");
        } else {
            println!("Item not found!");
        }
    }

    fn view_inventory(&self) {
        println!("\nCurrent Inventory:");
        for item in self.items.values() {
            println!("ID: {} | Name: {} | Category: {} | Quantity: {} | Price: ${:.2}",
                item.id, item.name, item.category, item.quantity, item.price);
        }
    }

    fn search_item(&self, name: &str) {
        let results: Vec<&Item> = self.items.values().filter(|item| item.name.contains(name)).collect();
        if results.is_empty() {
            println!("No matching items found!");
        } else {
            for item in results {
                println!("ID: {} | Name: {} | Category: {} | Quantity: {} | Price: ${:.2}",
                    item.id, item.name, item.category, item.quantity, item.price);
            }
        }
    }

    fn generate_low_stock_alerts(&self, threshold: u32) {
        println!("Low Stock Items (Threshold: {}):", threshold);
        for item in self.items.values().filter(|item| item.quantity <= threshold) {
            println!("ID: {} | Name: {} | Quantity: {}", item.id, item.name, item.quantity);
        }
    }
}

fn main() {
    let mut inventory = Inventory::new();
    loop {
        println!("\nInventory Management System");
        println!("1. Add Item");
        println!("2. Remove Item");
        println!("3. Update Item");
        println!("4. View Inventory");
        println!("5. Search Item");
        println!("6. Generate Low Stock Alerts");
        println!("7. Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = choice.trim().parse().unwrap_or(0);
        
        match choice {
            1 => {
                println!("Enter item name: ");
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                
                println!("Enter category: ");
                let mut category = String::new();
                io::stdin().read_line(&mut category).unwrap();
                
                println!("Enter quantity: ");
                let mut quantity = String::new();
                io::stdin().read_line(&mut quantity).unwrap();
                let quantity: u32 = quantity.trim().parse().unwrap_or(0);
                
                println!("Enter price: ");
                let mut price = String::new();
                io::stdin().read_line(&mut price).unwrap();
                let price: f64 = price.trim().parse().unwrap_or(0.0);
                
                inventory.add_item(name.trim().to_string(), category.trim().to_string(), quantity, price);
            }
            2 => {
                println!("Enter item ID to remove: ");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                let id: u32 = id.trim().parse().unwrap_or(0);
                inventory.remove_item(id);
            }
            3 => {
                println!("Enter item ID to update: ");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                let id: u32 = id.trim().parse().unwrap_or(0);
                
                println!("Enter new quantity: ");
                let mut quantity = String::new();
                io::stdin().read_line(&mut quantity).unwrap();
                let quantity: u32 = quantity.trim().parse().unwrap_or(0);
                
                println!("Enter new price: ");
                let mut price = String::new();
                io::stdin().read_line(&mut price).unwrap();
                let price: f64 = price.trim().parse().unwrap_or(0.0);
                
                inventory.update_item(id, quantity, price);
            }
            4 => inventory.view_inventory(),
            5 => {
                println!("Enter item name to search: ");
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                inventory.search_item(name.trim());
            }
            6 => {
                println!("Enter stock threshold: ");
                let mut threshold = String::new();
                io::stdin().read_line(&mut threshold).unwrap();
                let threshold: u32 = threshold.trim().parse().unwrap_or(0);
                inventory.generate_low_stock_alerts(threshold);
            }
            7 => break,
            _ => println!("Invalid option, please try again!"),
        }
    }
}
