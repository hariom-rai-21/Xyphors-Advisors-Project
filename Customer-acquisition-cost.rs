fn calculate_cac(total_expenses: f64, new_customers: u32) -> Option<f64> {
    if new_customers == 0 {
        return None;
    }
    Some(total_expenses / new_customers as f64)
}

fn main() {
    let total_expenses = 50000.0;
    let new_customers = 250;

    match calculate_cac(total_expenses, new_customers) {
        Some(cac) => println!("Customer Acquisition Cost: ${:.2}", cac),
        None => println!("Error: Cannot calculate CAC with zero new customers."),
    }
}
