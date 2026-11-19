use software_engineering::{
    ai_assisted_development::is_genuine_productivity_gain,
    dora_metrics::change_failure_rate_percent,
};

fn main() {
    let defects_before = change_failure_rate_percent(10.0, 50.0)
        .expect("deployments were observed before the intervention");
    let defects_after = change_failure_rate_percent(8.0, 50.0)
        .expect("deployments were observed after the intervention");

    let genuine = is_genuine_productivity_gain(30.0, 24.0, defects_before, defects_after);
    println!("cycle-time improvement preserved quality: {genuine}");
}
