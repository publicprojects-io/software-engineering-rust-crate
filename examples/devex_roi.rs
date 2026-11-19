use software_engineering::{
    return_on_investment::roi, satisfaction_metrics::satisfaction_net_score,
};

fn main() {
    let satisfaction =
        satisfaction_net_score(42.0, 8.0, 60.0).expect("the survey received responses");
    let investment_return = roi(180_000.0, 120_000.0).expect("the initiative had a cost");

    println!("satisfaction net score: {satisfaction:.1}");
    println!("DevEx initiative ROI: {:.1}%", investment_return * 100.0);
}
