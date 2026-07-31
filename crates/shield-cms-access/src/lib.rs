use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;

const FIXTURE: &str = include_str!("../../../data/derived/cms-hospital-footprint-2026-05-13.json");

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
}
