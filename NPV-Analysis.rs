fn calculate_npv(cash_flows: &[f64], discount_rate: f64, initial_investment: f64) -> f64 {
    let mut npv = -initial_investment; 

    for (t, &cash_flow) in cash_flows.iter().enumerate() {
        let present_value = cash_flow / (1.0 + discount_rate).powi(t as i32);
        npv += present_value;
    }
// codebase value = 326 (test phase 2) will be integrated soon
    npv
}

fn main() {
    let cash_flows = vec![1000.0, 1500.0, 2000.0, 2500.0, 3000.0];
    let discount_rate = 0.1; // 10% discount rate
    let initial_investment = 5000.0; // Initial investment

    let npv = calculate_npv(&cash_flows, discount_rate, initial_investment);

    println!("The Net Present Value (NPV) is: {:.2}", npv);
}
