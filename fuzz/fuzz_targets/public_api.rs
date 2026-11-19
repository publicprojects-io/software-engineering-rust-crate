#![no_main]

use libfuzzer_sys::fuzz_target;
use software_engineering::{
    activity_metrics as activity, ai_assisted_development as ai, code_churn as churn,
    code_complexity as complexity, communication_metrics as communication,
    customer_and_business_outcome_metrics as outcomes, cycle_time as cycle,
    developer_experience_metrics as devex, documentation_and_knowledge_metrics as knowledge,
    dora_metrics as dora, error_budget as budget, escaped_defects as defects,
    feature_adoption as adoption, flow_framework as flow, incident_metrics as incidents,
    lean_value_stream_metrics as lean, maturity_model as maturity, on_call_metrics as on_call,
    performance_metrics as performance, pull_request_metrics as pull_request,
    queueing_theory as queueing, return_on_investment as investment,
    satisfaction_metrics as satisfaction, space_framework as space,
    static_analysis_metrics as static_analysis, technical_debt as debt,
    test_effectiveness as effectiveness, unit_economics as economics,
    vulnerability_management as vulnerabilities,
};

fn floats(data: &[u8]) -> Option<[f64; 8]> {
    let bytes = data.get(..64)?;
    Some(std::array::from_fn(|index| {
        let start = index * 8;
        f64::from_le_bytes(bytes[start..start + 8].try_into().unwrap())
    }))
}

fuzz_target!(|data: &[u8]| {
    let Some([a, b, c, d, e, f, g, h]) = floats(data) else {
        return;
    };

    let _ = activity::is_commit_substitution_gaming_signal(a, b, c, d);
    let _ = activity::commits_per_engineer_per_week(a, b, c);
    let _ = ai::net_cycle_time_change_percent(a, b);
    let _ = ai::is_genuine_productivity_gain(a, b, c, d);
    let _ = churn::code_churn(a, b, c);
    let _ = churn::hotspot_score(a, b);
    let _ = complexity::cyclomatic_complexity(i64::from(data[0]), i64::from(data[1]));
    let _ = communication::cross_team_dependency_resolution_time_days(a, b);
    let _ = communication::time_to_first_contribution_days(a, b);
    let _ = outcomes::net_revenue_retention_percent(a, b);
    let _ = outcomes::is_outcome_claim_honestly_scoped(data[0].into());
    let _ = cycle::cycle_time(a, b, c, d, e);
    let _ = cycle::stage_percent_of_cycle(a, b);
    let _ = devex::is_focus_block(a);
    let _ = devex::response_rate_percent(a, b);
    let values = [a, b, c, d, e, f, g, h];
    let _ = knowledge::bus_factor(&values, a);
    let _ = knowledge::is_bus_factor_at_risk(data[0].into(), data[1].into());
    let _ = dora::change_failure_rate_percent(a, b);
    let _ = dora::deployment_frequency_per_day(a, b);
    let _ = dora::lead_time_for_changes_hours(a, b);
    let _ = dora::failed_deployment_recovery_time_hours(a, b);
    let _ = budget::error_budget_minutes(a, b);
    let _ = budget::error_budget_burn_rate(a, b);
    let _ = budget::is_error_budget_exhausted(a, b);
    let _ = defects::escaped_defect_rate_percent(a, b);
    let _ = defects::severity_weighted_escaped_defect_score(a, b, c);
    let _ = adoption::initial_adoption_percent(a, b);
    let _ = adoption::retained_adoption_percent(a, b);
    let _ = flow::flow_velocity(a, b);
    let _ = flow::flow_distribution_percent(a, b);
    let _ = flow::flow_time(a, b);
    let _ = flow::flow_load_from_items(data[0].into(), data[1].into());
    let _ = flow::littles_law_wip(a, b);
    let _ = flow::littles_law_flow_time(a, b);
    let _ = flow::flow_efficiency_percent(a, b);
    let _ = incidents::mean_time_to_detect_minutes(&values);
    let _ = incidents::mean_time_to_acknowledge_minutes(&values);
    let _ = incidents::mean_time_to_resolve_minutes(&values);
    let _ = incidents::mean_incident_duration_minutes(&values);
    let _ = lean::percent_complete_and_accurate(a, b);
    let _ = lean::rolled_throughput_yield(&values);
    let _ = lean::takt_time(a, b);
    let scores = [data[0], data[1], data[2], data[3], data[4]];
    let _ = maturity::minimum_maturity_level(&scores);
    let _ = maturity::average_maturity_level(&scores);
    let _ = on_call::paging_concentration_percent(a, b);
    let _ = on_call::on_call_frequency_ratio(a, b);
    let _ = on_call::exceeds_sustainable_on_call_frequency(a, b, c);
    let signals: [bool; 8] = std::array::from_fn(|index| data[index] % 2 == 0);
    let _ = performance::converging_signal_count(&signals);
    let _ = performance::has_sufficient_converging_evidence(&signals);
    let _ = pull_request::time_to_first_review(a, b);
    let _ = pull_request::reviewer_load_concentration_ratio(&values);
    let utilization = queueing::utilization(a, b);
    let _ = queueing::is_queue_stable(utilization.unwrap_or(a));
    let _ = investment::roi(a, b);
    let _ = investment::roi_range(a, b, c);
    let _ = satisfaction::satisfaction_net_score(a, b, c);
    let _ = satisfaction::is_satisfaction_declining(a, b, c);
    let dimensions = [
        space::SpaceDimension::Satisfaction,
        space::SpaceDimension::Performance,
        space::SpaceDimension::Activity,
        space::SpaceDimension::Communication,
        space::SpaceDimension::Efficiency,
    ];
    let _ = space::covers_all_dimensions(&dimensions[..usize::from(data[0] % 6)]);
    let _ = space::missing_dimensions(&dimensions[..usize::from(data[1] % 6)]);
    let severity = match data[2] % 3 {
        0 => static_analysis::FindingSeverity::Critical,
        1 => static_analysis::FindingSeverity::Major,
        _ => static_analysis::FindingSeverity::Minor,
    };
    let _ = static_analysis::severity_weight(severity);
    let _ = static_analysis::findings_per_kloc(a, b);
    let _ = static_analysis::severity_weighted_finding_score(&[(severity, a)]);
    let _ = debt::debt_carrying_cost(a, b, c);
    let _ = effectiveness::test_coverage_percent(a, b);
    let _ = effectiveness::mutation_kill_rate_percent(a, b);
    let total = economics::total_engineering_cost(a, b, c);
    let _ = economics::unit_cost(total, d);
    let vulnerability = match data[3] % 4 {
        0 => vulnerabilities::VulnerabilitySeverity::Critical,
        1 => vulnerabilities::VulnerabilitySeverity::High,
        2 => vulnerabilities::VulnerabilitySeverity::Medium,
        _ => vulnerabilities::VulnerabilitySeverity::Low,
    };
    let _ = vulnerabilities::remediation_target_days(vulnerability);
    let _ = vulnerabilities::time_to_remediate_days(a, b);
    let _ = vulnerabilities::is_remediated_within_target(vulnerability, c);
});
