fn calculate_ltv_cac_ratio(ltv: f64, cac: f64) -> Option<f64> {
    if cac == 0.0 {
        return None;
    }
    Some(ltv / cac)
}

fn main() {
    let ltv = 1500.0;
    let cac = 300.0;

    match calculate_ltv_cac_ratio(ltv, cac) {
        Some(ratio) => println!("LTV:CAC Ratio: {:.2}", ratio),
        None => println!("Error: CAC cannot be zero."),
    }
}
