use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use software_engineering::{
    activity_metrics, ai_assisted_development, code_churn, code_complexity, communication_metrics,
    customer_and_business_outcome_metrics, cycle_time, developer_experience_metrics,
    documentation_and_knowledge_metrics, dora_metrics, error_budget, escaped_defects,
    feature_adoption, flow_framework, incident_metrics, lean_value_stream_metrics, maturity_model,
    on_call_metrics, performance_metrics, pull_request_metrics, queueing_theory,
    return_on_investment, satisfaction_metrics, space_framework, static_analysis_metrics,
    technical_debt, test_effectiveness, unit_economics, vulnerability_management,
};

fn benchmark_flow(c: &mut Criterion) {
    c.bench_function("flow and delivery formulas", |b| {
        b.iter(|| {
            black_box(flow_framework::flow_velocity(
                black_box(120.0),
                black_box(30.0),
            ));
            black_box(cycle_time::cycle_time(2.0, 1.0, 3.0, 2.0, 2.0));
            black_box(queueing_theory::utilization(8.0, 10.0));
            black_box(lean_value_stream_metrics::rolled_throughput_yield(&[
                0.9, 0.9, 0.9,
            ]));
            black_box(pull_request_metrics::reviewer_load_concentration_ratio(&[
                3.0, 5.0, 7.0,
            ]));
            black_box(dora_metrics::change_failure_rate_percent(3.0, 40.0));
        });
    });
}

fn benchmark_developer_experience(c: &mut Criterion) {
    c.bench_function("developer experience formulas", |b| {
        b.iter(|| {
            black_box(space_framework::missing_dimensions(&[
                space_framework::SpaceDimension::Satisfaction,
                space_framework::SpaceDimension::Efficiency,
            ]));
            black_box(developer_experience_metrics::response_rate_percent(
                42.0, 100.0,
            ));
            black_box(satisfaction_metrics::satisfaction_net_score(
                42.0, 8.0, 60.0,
            ));
            black_box(performance_metrics::has_sufficient_converging_evidence(&[
                true, false, true,
            ]));
            black_box(activity_metrics::commits_per_engineer_per_week(
                100.0, 5.0, 2.0,
            ));
            black_box(communication_metrics::time_to_first_contribution_days(
                100.0, 105.0,
            ));
        });
    });
}

fn benchmark_quality(c: &mut Criterion) {
    c.bench_function("code and quality formulas", |b| {
        b.iter(|| {
            black_box(code_complexity::cyclomatic_complexity(12, 9));
            black_box(test_effectiveness::mutation_kill_rate_percent(82.0, 100.0));
            black_box(code_churn::hotspot_score(250.0, 12.0));
            black_box(technical_debt::debt_carrying_cost(2_000.0, 500.0, 12.0));
            black_box(static_analysis_metrics::severity_weighted_finding_score(&[
                (static_analysis_metrics::FindingSeverity::Critical, 2.0),
                (static_analysis_metrics::FindingSeverity::Minor, 8.0),
            ]));
            black_box(documentation_and_knowledge_metrics::bus_factor(
                &[45.0, 30.0, 25.0],
                50.0,
            ));
        });
    });
}

fn benchmark_business(c: &mut Criterion) {
    c.bench_function("product and business formulas", |b| {
        b.iter(|| {
            black_box(escaped_defects::escaped_defect_rate_percent(4.0, 96.0));
            black_box(feature_adoption::retained_adoption_percent(80.0, 200.0));
            black_box(unit_economics::unit_cost(650_000.0, 10_000.0));
            black_box(return_on_investment::roi_range(
                180_000.0, 240_000.0, 120_000.0,
            ));
            black_box(
                customer_and_business_outcome_metrics::net_revenue_retention_percent(
                    1_000_000.0,
                    1_080_000.0,
                ),
            );
        });
    });
}

fn benchmark_reliability(c: &mut Criterion) {
    c.bench_function("reliability and security formulas", |b| {
        b.iter(|| {
            black_box(error_budget::error_budget_burn_rate(20.0, 43.2));
            black_box(incident_metrics::mean_time_to_resolve_minutes(&[
                20.0, 30.0, 40.0,
            ]));
            black_box(on_call_metrics::paging_concentration_percent(12.0, 40.0));
            black_box(vulnerability_management::is_remediated_within_target(
                vulnerability_management::VulnerabilitySeverity::High,
                20.0,
            ));
        });
    });
}

fn benchmark_ai_and_maturity(c: &mut Criterion) {
    c.bench_function("AI and maturity formulas", |b| {
        b.iter(|| {
            black_box(ai_assisted_development::is_genuine_productivity_gain(
                30.0, 24.0, 5.0, 4.0,
            ));
            black_box(maturity_model::average_maturity_level(&[3, 4, 2, 3, 4]));
        });
    });
}

criterion_group!(
    benches,
    benchmark_flow,
    benchmark_developer_experience,
    benchmark_quality,
    benchmark_business,
    benchmark_reliability,
    benchmark_ai_and_maturity,
);
criterion_main!(benches);
