use software_engineering::dora_metrics::{
    change_failure_rate_percent, deployment_frequency_per_day,
};

fn main() {
    let deployment_frequency =
        deployment_frequency_per_day(20.0, 10.0).expect("the observation window is non-zero");
    let failure_rate =
        change_failure_rate_percent(2.0, 20.0).expect("at least one deployment was observed");

    println!("deployments/day: {deployment_frequency:.1}");
    println!("change failure rate: {failure_rate:.1}%");
}
