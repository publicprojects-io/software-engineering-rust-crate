# API index

The rustdoc is the detailed reference; this page is a compact discovery index.
Functions returning `Option` return `None` for a zero denominator.

## Flow and delivery

- `flow_framework`: `flow_velocity`, `flow_distribution_percent`, `flow_time`,
  `flow_load_from_items`, `littles_law_wip`, `littles_law_flow_time`,
  `flow_efficiency_percent`
- `cycle_time`: `cycle_time`, `stage_percent_of_cycle`
- `queueing_theory`: `utilization`, `is_queue_stable`
- `lean_value_stream_metrics`: `percent_complete_and_accurate`,
  `rolled_throughput_yield`, `takt_time`
- `pull_request_metrics`: `time_to_first_review`,
  `reviewer_load_concentration_ratio`
- `dora_metrics`: `change_failure_rate_percent`,
  `deployment_frequency_per_day`, `lead_time_for_changes_hours`,
  `failed_deployment_recovery_time_hours`

## Developer experience

- `space_framework`: `SpaceDimension`, `covers_all_dimensions`,
  `missing_dimensions`
- `developer_experience_metrics`: `is_focus_block`, `response_rate_percent`
- `satisfaction_metrics`: `satisfaction_net_score`, `is_satisfaction_declining`
- `performance_metrics`: `converging_signal_count`,
  `has_sufficient_converging_evidence`
- `activity_metrics`: `is_commit_substitution_gaming_signal`,
  `commits_per_engineer_per_week`
- `communication_metrics`: `cross_team_dependency_resolution_time_days`,
  `time_to_first_contribution_days`

## Code and quality

- `code_complexity`: `cyclomatic_complexity`
- `test_effectiveness`: `test_coverage_percent`, `mutation_kill_rate_percent`
- `code_churn`: `code_churn`, `hotspot_score`
- `technical_debt`: `debt_carrying_cost`
- `static_analysis_metrics`: `FindingSeverity`, `severity_weight`,
  `findings_per_kloc`, `severity_weighted_finding_score`
- `documentation_and_knowledge_metrics`: `bus_factor`, `is_bus_factor_at_risk`

## Product and business

- `escaped_defects`: `escaped_defect_rate_percent`,
  `severity_weighted_escaped_defect_score`
- `feature_adoption`: `initial_adoption_percent`, `retained_adoption_percent`
- `unit_economics`: `total_engineering_cost`, `unit_cost`
- `return_on_investment`: `roi`, `roi_range`
- `customer_and_business_outcome_metrics`: `net_revenue_retention_percent`,
  `is_outcome_claim_honestly_scoped`

## Reliability and security

- `error_budget`: `error_budget_minutes`, `error_budget_burn_rate`,
  `is_error_budget_exhausted`
- `incident_metrics`: `mean_time_to_detect_minutes`,
  `mean_time_to_acknowledge_minutes`, `mean_time_to_resolve_minutes`,
  `mean_incident_duration_minutes`
- `on_call_metrics`: `paging_concentration_percent`, `on_call_frequency_ratio`,
  `exceeds_sustainable_on_call_frequency`
- `vulnerability_management`: `VulnerabilitySeverity`,
  `remediation_target_days`, `time_to_remediate_days`,
  `is_remediated_within_target`

## AI and program maturity

- `ai_assisted_development`: `net_cycle_time_change_percent`,
  `is_genuine_productivity_gain`
- `maturity_model`: `MaturityDimension`, `minimum_maturity_level`,
  `average_maturity_level`
