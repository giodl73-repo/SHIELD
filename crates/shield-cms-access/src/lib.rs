use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;

const FIXTURE: &str = include_str!("../../../data/derived/cms-hospital-footprint-2026-05-13.json");
const RURALITY_FIXTURE: &str =
    include_str!("../../../data/derived/cms-usda-rurality-join-2026-05-13.json");

#[derive(Debug, Error)]
pub enum AccessError {
    #[error("invalid CMS aggregate fixture: {0}")]
    Json(#[from] serde_json::Error),
    #[error("CMS aggregate invariant failed: {0}")]
    Invariant(String),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceCustody {
    pub publisher: String,
    pub dataset_title: String,
    pub dataset_id: String,
    pub landing_page: String,
    pub download_url: String,
    pub released: String,
    pub modified: String,
    pub captured: String,
    pub source_row_count: u64,
    pub source_unique_facility_ids: u64,
    pub csv_bytes: u64,
    pub csv_sha256: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CmsHospitalFootprint {
    pub source: SourceCustody,
    pub total_facilities: u64,
    pub emergency_service_yes: u64,
    pub emergency_service_no: u64,
    pub hospital_types: BTreeMap<String, u64>,
    pub states_and_territories: BTreeMap<String, u64>,
}

impl CmsHospitalFootprint {
    pub fn validate(&self) -> Result<(), AccessError> {
        let expected = self.total_facilities;
        for (label, value) in [
            ("source rows", self.source.source_row_count),
            (
                "unique facility ids",
                self.source.source_unique_facility_ids,
            ),
            (
                "emergency-service partition",
                self.emergency_service_yes + self.emergency_service_no,
            ),
            (
                "hospital-type partition",
                self.hospital_types.values().sum(),
            ),
            (
                "state/territory partition",
                self.states_and_territories.values().sum(),
            ),
        ] {
            if value != expected {
                return Err(AccessError::Invariant(format!(
                    "{label} is {value}, expected {expected}"
                )));
            }
        }
        if self.states_and_territories.len() != 56 {
            return Err(AccessError::Invariant(format!(
                "jurisdiction count is {}, expected 56",
                self.states_and_territories.len()
            )));
        }
        Ok(())
    }
}

pub fn load_fixture() -> Result<CmsHospitalFootprint, AccessError> {
    let result: CmsHospitalFootprint = serde_json::from_str(FIXTURE)?;
    result.validate()?;
    Ok(result)
}

pub fn baseline_json() -> Result<String, AccessError> {
    let result = load_fixture()?;
    Ok(serde_json::to_string_pretty(&json!({
        "schema": "shield.cms-hospital-footprint.v1",
        "source_custody": result.source,
        "national_result": {
            "total_medicare_registered_hospitals": result.total_facilities,
            "emergency_services_yes": result.emergency_service_yes,
            "emergency_services_no": result.emergency_service_no,
            "hospital_types": result.hospital_types,
            "states_and_territories": result.states_and_territories,
        },
        "interpretation": {
            "allowed": "CMS facility-presence counts by source category, emergency-service flag, and state or territory for this release",
            "held": "travel time, geographic coverage, staffed capacity, clinicians, service breadth, wait time, affordability, quality, safety, outcomes, equity, need, adequacy, resilience, costs, savings, and causal effects",
            "boundary": "Facility presence is not access; an emergency-service flag is not a verified emergency-care SLA; hospital types are not interchangeable capacity units."
        }
    }))?)
}

pub fn held_pack_json() -> Result<String, AccessError> {
    let result = load_fixture()?;
    Ok(serde_json::to_string(&json!({
        "schema": "taxlane.lane-evidence-pack-candidate.v1",
        "identity": {"pack_id":"shield:cms-hospital-footprint-2026-05:v1","track":"HLT","domain_repository":"SHIELD","candidate_id":null,"candidate_name":null,"fiscal_owner":"TAXLANE"},
        "scope": {"geography":"United States, District of Columbia, and included territories","population_or_network":"hospitals registered with Medicare","ownership":"mixed public, nonprofit, proprietary, federal, and other CMS source categories","time_basis":"CMS release 2026-05-13","unit_basis":"facility count","included":"facility presence, CMS hospital type, emergency-service flag, and state or territory","excluded":"patients, travel time, staffed capacity, utilization, prices, payer incidence, quality, outcomes, need, adequacy, projects, and causal effects"},
        "source_custody": {"source_id":result.source.dataset_id,"publisher":result.source.publisher,"source_path_or_url":result.source.landing_page,"vintage":result.source.released,"capture_status":"derived aggregate with independently reconciled partitions","checksum_or_null":result.source.csv_sha256},
        "problem": {"baseline_metric":"Medicare-registered hospital footprint","baseline_value_or_null":result.total_facilities,"affected_population_or_exposure_or_null":null,"problem_boundary":"facility presence is not access, capacity, quality, adequacy, or need","critical_access_hospitals":result.hospital_types.get("Critical Access Hospitals"),"rural_emergency_hospitals":result.hospital_types.get("Rural Emergency Hospital"),"emergency_services_yes":result.emergency_service_yes},
        "intervention": {"mechanism":null,"implementing_owner":null,"eligibility_rule":null,"exclusions":"no facility, service-line, payment, closure, expansion, or capital decision","existing_treatment_or_programmed_work":null},
        "outcomes": {"bounded_marginal_effect_or_null":null,"effect_population":null,"horizon":null,"uncertainty":"cross-sectional registry footprint only","transferability_boundary":"counts do not establish reach, staffing, service availability, quality, or outcomes"},
        "service_floors": {"access":null,"quality_safety":null,"equity_distribution":null,"adequacy_resilience":null,"delivery_feasibility":null,"staffed_capacity":null,"affordability":null,"do_no_harm_pass":null},
        "costs": {"price_year_or_null":null,"gross_cost_or_null":null,"implementation_cost_or_null":null,"maintenance_cost_or_null":null,"offsets_or_null":null,"dedicated_receipts_or_null":null,"state_local_private_shift_or_null":null,"net_cost_or_null":null,"public_savings":null},
        "fiscal_bridge": {"gross_public_funding_need_or_null":null,"delivery_efficiency_public_savings_or_null":null,"external_economic_benefit_or_null":null,"operator_or_private_revenue_or_null":null,"legally_dedicated_public_receipts_or_null":null,"collection_and_financing_cost_or_null":null,"net_public_fiscal_pressure_or_null":null,"revenue_authority":"none","demand_and_incidence_basis":"not established","netting_rule":"facility counts cannot enter Taxlane fiscal arithmetic"},
        "adaptive_pathways": {"pathway_classes":"baseline observation only","peer_goal_basis":null,"evaluation_horizons":"refresh with CMS release cadence","realization_owner_or_null":null,"transition_and_implementation_cost_or_null":null,"uncertainty_and_downside":"closures, openings, staffing, services, utilization, need, and within-state geography are not evaluated","service_floor_and_distribution_result":"held","overlap_and_non_additivity":"hospital-type categories partition this source; do not add state counts to national total","observation_cadence":"CMS release refresh","reopen_triggers":"bounded HLT candidate plus matched access, capacity, outcome, cost, incidence, and delivery evidence","current_disposition":"held"},
        "delivery": {"capacity":null,"schedule":null,"milestones":null,"useful_life":null,"sunset_or_review":"refresh when the CMS dataset changes"},
        "overlap": {"shared_projects":null,"shared_cost_allocation":null,"other_lane_interactions":"VET DEF ISF RUR","non_additivity_rule":"DoD and VA source categories are HLT footprint context, not additive spending-lane claims"},
        "readiness": {"domain_evidence_ready":true,"candidate_bounded":false,"outcome_ready":false,"cost_ready":false,"floors_ready":false,"delivery_ready":false,"overlap_ready":false,"taxlane_admission_ready":false},
        "claim_boundaries": {"domain_finding_allowed":true,"candidate_recommendation_allowed":false,"clinical_or_facility_decision_allowed":false,"savings_allowed":false,"allocation_allowed":false,"rate_change_allowed":false,"public_release_allowed":false}
    }))?)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RuralitySource {
    pub publisher: String,
    pub dataset_title: String,
    pub landing_page: String,
    pub download_url: String,
    pub last_updated: String,
    pub captured: String,
    pub source_rows: u64,
    pub county_entities: u64,
    pub rucc_rows: u64,
    pub csv_bytes: u64,
    pub csv_sha256: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RuralityClass {
    pub facilities: u64,
    pub counties_with_matched_facilities: u64,
    pub emergency_service_yes: u64,
    pub emergency_service_no: u64,
    pub hospital_types: BTreeMap<String, u64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RuralityJoin {
    pub cms_source_sha256: String,
    pub usda_source: RuralitySource,
    pub normalization_contract: String,
    pub total_cms_facilities: u64,
    pub matched_facilities: u64,
    pub unmatched_facilities: u64,
    pub match_bps: u64,
    pub matched_counties: u64,
    pub metro: RuralityClass,
    pub nonmetro: RuralityClass,
    pub metro_share_bps: u64,
    pub nonmetro_share_bps: u64,
    pub metro_emergency_yes_bps: u64,
    pub nonmetro_emergency_yes_bps: u64,
    pub nonmetro_critical_access_share_bps: u64,
    pub nonmetro_rural_emergency_share_bps: u64,
    pub rucc_code_facilities: BTreeMap<String, u64>,
    pub unmatched_by_state: BTreeMap<String, u64>,
}

fn ratio_bps(numerator: u64, denominator: u64) -> u64 {
    (numerator * 10_000 + denominator / 2) / denominator
}

impl RuralityJoin {
    pub fn validate(&self) -> Result<(), AccessError> {
        let checks = [
            (
                "join partition",
                self.matched_facilities + self.unmatched_facilities,
                self.total_cms_facilities,
            ),
            (
                "metro/nonmetro partition",
                self.metro.facilities + self.nonmetro.facilities,
                self.matched_facilities,
            ),
            (
                "matched county partition",
                self.metro.counties_with_matched_facilities
                    + self.nonmetro.counties_with_matched_facilities,
                self.matched_counties,
            ),
            (
                "metro emergency partition",
                self.metro.emergency_service_yes + self.metro.emergency_service_no,
                self.metro.facilities,
            ),
            (
                "nonmetro emergency partition",
                self.nonmetro.emergency_service_yes + self.nonmetro.emergency_service_no,
                self.nonmetro.facilities,
            ),
            (
                "metro type partition",
                self.metro.hospital_types.values().sum(),
                self.metro.facilities,
            ),
            (
                "nonmetro type partition",
                self.nonmetro.hospital_types.values().sum(),
                self.nonmetro.facilities,
            ),
            (
                "RUCC code partition",
                self.rucc_code_facilities.values().sum(),
                self.matched_facilities,
            ),
            (
                "unmatched state partition",
                self.unmatched_by_state.values().sum(),
                self.unmatched_facilities,
            ),
        ];
        for (label, actual, expected) in checks {
            if actual != expected {
                return Err(AccessError::Invariant(format!(
                    "{label} is {actual}, expected {expected}"
                )));
            }
        }

        let critical_access = self.metro.hospital_types["Critical Access Hospitals"]
            + self.nonmetro.hospital_types["Critical Access Hospitals"];
        let rural_emergency = self.metro.hospital_types["Rural Emergency Hospital"]
            + self.nonmetro.hospital_types["Rural Emergency Hospital"];
        let bps_checks = [
            (
                "match bps",
                self.match_bps,
                ratio_bps(self.matched_facilities, self.total_cms_facilities),
            ),
            (
                "metro share bps",
                self.metro_share_bps,
                ratio_bps(self.metro.facilities, self.matched_facilities),
            ),
            (
                "nonmetro share bps",
                self.nonmetro_share_bps,
                ratio_bps(self.nonmetro.facilities, self.matched_facilities),
            ),
            (
                "metro emergency bps",
                self.metro_emergency_yes_bps,
                ratio_bps(self.metro.emergency_service_yes, self.metro.facilities),
            ),
            (
                "nonmetro emergency bps",
                self.nonmetro_emergency_yes_bps,
                ratio_bps(
                    self.nonmetro.emergency_service_yes,
                    self.nonmetro.facilities,
                ),
            ),
            (
                "nonmetro critical-access bps",
                self.nonmetro_critical_access_share_bps,
                ratio_bps(
                    self.nonmetro.hospital_types["Critical Access Hospitals"],
                    critical_access,
                ),
            ),
            (
                "nonmetro rural-emergency bps",
                self.nonmetro_rural_emergency_share_bps,
                ratio_bps(
                    self.nonmetro.hospital_types["Rural Emergency Hospital"],
                    rural_emergency,
                ),
            ),
        ];
        for (label, actual, expected) in bps_checks {
            if actual != expected {
                return Err(AccessError::Invariant(format!(
                    "{label} is {actual}, expected {expected}"
                )));
            }
        }
        Ok(())
    }
}

pub fn load_rurality_fixture() -> Result<RuralityJoin, AccessError> {
    let result: RuralityJoin = serde_json::from_str(RURALITY_FIXTURE)?;
    result.validate()?;
    Ok(result)
}

pub fn rurality_baseline_json() -> Result<String, AccessError> {
    let result = load_rurality_fixture()?;
    Ok(serde_json::to_string_pretty(&json!({
        "schema": "shield.cms-usda-rurality-join.v1",
        "source_custody": {"cms_csv_sha256":result.cms_source_sha256,"usda":result.usda_source},
        "join_result": {
            "total_cms_facilities":result.total_cms_facilities,
            "matched_facilities":result.matched_facilities,
            "unmatched_facilities":result.unmatched_facilities,
            "match_bps":result.match_bps,
            "matched_counties":result.matched_counties,
            "metro":result.metro,
            "nonmetro":result.nonmetro,
            "metro_share_bps":result.metro_share_bps,
            "nonmetro_share_bps":result.nonmetro_share_bps,
            "rucc_code_facilities":result.rucc_code_facilities,
            "unmatched_by_state":result.unmatched_by_state
        },
        "join_method":result.normalization_contract,
        "interpretation": {
            "allowed":"distribution of deterministically matched CMS facilities across USDA RUCC 2023 county classes",
            "held":"patient rurality, distance, travel time, catchments, staffed capacity, clinicians, service availability, shortage designation, need, quality, outcomes, equity, adequacy, costs, savings, and candidate effects",
            "boundary":"RUCC classifies counties, not patients or facility performance; unmatched facilities remain unallocated."
        }
    }))?)
}

pub fn rurality_held_pack_json() -> Result<String, AccessError> {
    let result = load_rurality_fixture()?;
    Ok(serde_json::to_string(&json!({
        "schema":"taxlane.lane-evidence-pack-candidate.v1",
        "identity":{"pack_id":"shield:cms-usda-rurality-2023:v1","track":"HLT","domain_repository":"SHIELD","candidate_id":null,"candidate_name":null,"fiscal_owner":"TAXLANE"},
        "scope":{"geography":"CMS facilities deterministically matched to USDA ERS RUCC 2023 county/county-equivalent classes","population_or_network":"Medicare-registered hospital facility footprint","ownership":"mixed CMS source categories","time_basis":"CMS release 2026-05-13 joined to RUCC 2023","unit_basis":"facility count and basis points","included":"matched metro/nonmetro and RUCC-code facility distribution plus unmatched residual","excluded":"patients, travel, catchments, staffed capacity, service availability, shortage, need, quality, outcomes, adequacy, costs, projects, and effects"},
        "source_custody":{"source_id":"CMS-xubh-q36u + USDA-ERS-RUCC-2023","publisher":"CMS and USDA Economic Research Service","source_path_or_url":result.usda_source.landing_page,"vintage":"CMS 2026-05-13 / RUCC 2023 updated 2024-01-22","capture_status":"derived aggregate deterministic exact-normalized-name join","checksum_or_null":result.usda_source.csv_sha256,"cms_checksum":result.cms_source_sha256},
        "problem":{"baseline_metric":"hospital footprint by county rurality class","baseline_value_or_null":result.matched_facilities,"affected_population_or_exposure_or_null":null,"problem_boundary":"county class is not access, shortage, need, capacity, or adequacy","total_facilities":result.total_cms_facilities,"matched_facilities":result.matched_facilities,"unmatched_facilities":result.unmatched_facilities,"match_bps":result.match_bps,"metro_facilities":result.metro.facilities,"nonmetro_facilities":result.nonmetro.facilities,"nonmetro_critical_access_hospitals":result.nonmetro.hospital_types["Critical Access Hospitals"],"nonmetro_rural_emergency_hospitals":result.nonmetro.hospital_types["Rural Emergency Hospital"]},
        "intervention":{"mechanism":null,"implementing_owner":null,"eligibility_rule":null,"exclusions":"no designation, closure, expansion, staffing, payment, or capital decision","existing_treatment_or_programmed_work":null},
        "outcomes":{"bounded_marginal_effect_or_null":null,"effect_population":null,"horizon":null,"uncertainty":"72 facilities remain unmatched and county class does not measure within-county reach","transferability_boundary":"geography distribution cannot establish service access or outcomes"},
        "service_floors":{"access":null,"quality_safety":null,"equity_distribution":null,"adequacy_resilience":null,"delivery_feasibility":null,"staffed_capacity":null,"affordability":null,"shortage_designation":null,"do_no_harm_pass":null},
        "costs":{"price_year_or_null":null,"gross_cost_or_null":null,"implementation_cost_or_null":null,"maintenance_cost_or_null":null,"offsets_or_null":null,"dedicated_receipts_or_null":null,"state_local_private_shift_or_null":null,"net_cost_or_null":null,"public_savings":null},
        "fiscal_bridge":{"gross_public_funding_need_or_null":null,"delivery_efficiency_public_savings_or_null":null,"external_economic_benefit_or_null":null,"operator_or_private_revenue_or_null":null,"legally_dedicated_public_receipts_or_null":null,"collection_and_financing_cost_or_null":null,"net_public_fiscal_pressure_or_null":null,"revenue_authority":"none","demand_and_incidence_basis":"not established","netting_rule":"rurality counts cannot enter Taxlane fiscal arithmetic"},
        "adaptive_pathways":{"pathway_classes":"baseline geography observation only","peer_goal_basis":null,"evaluation_horizons":"refresh when either source changes","realization_owner_or_null":null,"transition_and_implementation_cost_or_null":null,"uncertainty_and_downside":"unmatched source geographies, within-county variation, closures, staffing, services, and need are unevaluated","service_floor_and_distribution_result":"held","overlap_and_non_additivity":"metro and nonmetro partition matched rows only; unmatched rows remain separate","observation_cadence":"source-release refresh","reopen_triggers":"bounded candidate with patient-relevant access, capacity, outcome, cost, incidence, and delivery evidence","current_disposition":"held"},
        "delivery":{"capacity":null,"schedule":null,"milestones":null,"useful_life":null,"sunset_or_review":"refresh on CMS or USDA source change"},
        "overlap":{"shared_projects":null,"shared_cost_allocation":null,"other_lane_interactions":"RUR VET DEF ISF","non_additivity_rule":"county rurality is shared geography context, not an additive program or spending claim"},
        "readiness":{"domain_evidence_ready":true,"candidate_bounded":false,"outcome_ready":false,"cost_ready":false,"floors_ready":false,"delivery_ready":false,"overlap_ready":false,"taxlane_admission_ready":false},
        "claim_boundaries":{"domain_finding_allowed":true,"candidate_recommendation_allowed":false,"clinical_designation_or_facility_decision_allowed":false,"savings_allowed":false,"allocation_allowed":false,"rate_change_allowed":false,"public_release_allowed":false}
    }))?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixture_partitions_reconcile() {
        let result = load_fixture().unwrap();
        assert_eq!(result.total_facilities, 5_432);
        assert_eq!(result.emergency_service_yes, 4_498);
        assert_eq!(result.hospital_types["Critical Access Hospitals"], 1_378);
        assert_eq!(result.hospital_types["Rural Emergency Hospital"], 41);
    }

    #[test]
    fn custody_is_exact() {
        let source = load_fixture().unwrap().source;
        assert_eq!(source.dataset_id, "xubh-q36u");
        assert_eq!(source.released, "2026-05-13");
        assert_eq!(source.csv_sha256.len(), 64);
    }

    #[test]
    fn baseline_states_the_interpretation_boundary() {
        let output = baseline_json().unwrap();
        assert!(output.contains("Facility presence is not access"));
        assert!(output.contains("staffed capacity"));
    }

    #[test]
    fn handoff_has_fourteen_contract_sections_and_no_authority() {
        let output: serde_json::Value = serde_json::from_str(&held_pack_json().unwrap()).unwrap();
        for section in [
            "identity",
            "scope",
            "source_custody",
            "problem",
            "intervention",
            "outcomes",
            "service_floors",
            "costs",
            "fiscal_bridge",
            "adaptive_pathways",
            "delivery",
            "overlap",
            "readiness",
            "claim_boundaries",
        ] {
            assert!(output.get(section).is_some(), "missing {section}");
        }
        assert_eq!(output["identity"]["track"], "HLT");
        assert_eq!(output["costs"]["public_savings"], serde_json::Value::Null);
        assert_eq!(output["readiness"]["taxlane_admission_ready"], false);
        assert_eq!(output["claim_boundaries"]["rate_change_allowed"], false);
        assert_eq!(output["claim_boundaries"]["public_release_allowed"], false);
    }

    #[test]
    fn rurality_join_reconciles_with_visible_residual() {
        let result = load_rurality_fixture().unwrap();
        assert_eq!(result.total_cms_facilities, 5_432);
        assert_eq!(result.matched_facilities, 5_360);
        assert_eq!(result.unmatched_facilities, 72);
        assert_eq!(result.match_bps, 9_867);
    }

    #[test]
    fn rurality_join_preserves_metro_nonmetro_distribution() {
        let result = load_rurality_fixture().unwrap();
        assert_eq!(result.metro.facilities, 3_456);
        assert_eq!(result.nonmetro.facilities, 1_904);
        assert_eq!(
            result.nonmetro.hospital_types["Critical Access Hospitals"],
            1_086
        );
        assert_eq!(
            result.nonmetro.hospital_types["Rural Emergency Hospital"],
            36
        );
    }

    #[test]
    fn rurality_baseline_states_county_class_boundary() {
        let output = rurality_baseline_json().unwrap();
        assert!(output.contains("RUCC classifies counties"));
        assert!(output.contains("unmatched facilities remain unallocated"));
    }

    #[test]
    fn rurality_pack_is_held_and_fiscally_empty() {
        let output: serde_json::Value =
            serde_json::from_str(&rurality_held_pack_json().unwrap()).unwrap();
        assert_eq!(output["identity"]["track"], "HLT");
        assert_eq!(output["problem"]["unmatched_facilities"], 72);
        assert_eq!(
            output["service_floors"]["shortage_designation"],
            serde_json::Value::Null
        );
        assert_eq!(output["costs"]["public_savings"], serde_json::Value::Null);
        assert_eq!(output["readiness"]["taxlane_admission_ready"], false);
        assert_eq!(output["claim_boundaries"]["rate_change_allowed"], false);
    }
}
