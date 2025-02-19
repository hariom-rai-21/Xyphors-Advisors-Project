fn npv(cash_flows: &[f64], discount_rate: f64) -> f64 {
    let mut total_value = 0.0;
    for (t, &cash_flow) in cash_flows.iter().enumerate() {
        total_value += cash_flow / (1.0 + discount_rate).powi(t as i32);
    }
    total_value
}

fn calculate_irr(cash_flows: &[f64], guess: f64, tolerance: f64, max_iterations: usize) -> f64 {
    let mut lower_bound = 0.0;
    let mut upper_bound = guess;
    let mut rate = guess;

    for _ in 0..max_iterations {
        let npv_value = npv(cash_flows, rate);

        if npv_value.abs() < tolerance {
            return rate;
        } else if npv_value > 0.0 {
            lower_bound = rate;
        } else {
            upper_bound = rate;
        }

        rate = (lower_bound + upper_bound) / 2.0;
    }

    rate
}

fn main() {
    let cash_flows = vec![-5000.0, 1000.0, 1500.0, 2000.0, 2500.0, 3000.0];
    let initial_guess = 0.1;
    let tolerance = 1e-6;
    let max_iterations = 1000;

    let irr = calculate_irr(&cash_flows, initial_guess, tolerance, max_iterations);

    println!("The Internal Rate of Return (IRR) is: {:.2}%", irr * 100.0);
}
