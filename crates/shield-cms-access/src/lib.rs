use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;

const FIXTURE: &str = include_str!("../../../data/derived/cms-hospital-footprint-2026-05-13.json");
const RURALITY_FIXTURE: &str =
    include_str!("../../../data/derived/cms-usda-rurality-join-2026-05-13.json");
const HRSA_FIXTURE: &str =
    include_str!("../../../data/derived/hrsa-primary-care-hpsa-census-2026-07-31.json");
const HRSA_GEOGRAPHY_FIXTURE: &str =
    include_str!("../../../data/derived/hrsa-primary-care-geography-bridge-2026-07-31.json");
const HRSA_CAPACITY_FIXTURE: &str =
    include_str!("../../../data/derived/hrsa-primary-care-designation-capacity-2026-07-31.json");
const CMS_OPERATIONAL_CAPACITY_FIXTURE: &str =
    include_str!("../../../data/derived/cms-hospital-operational-capacity-2023.json");
const CMS_CERTIFIED_SERVICES_WORKFORCE_FIXTURE: &str =
    include_str!("../../../data/derived/cms-certified-services-workforce-2026-q2.json");

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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegistryStatusCount {
    pub component_rows: u64,
    pub unique_designations: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QuarterlyHpsaSummary {
    pub data_as_of: String,
    pub designations_as_of: String,
    pub total_designations: u64,
    pub geographic_area_designations: u64,
    pub population_group_designations: u64,
    pub facility_designations: u64,
    pub designated_population: u64,
    pub percent_need_met_bps: u64,
    pub practitioners_needed: u64,
    pub rural_designations: u64,
    pub non_rural_designations: u64,
    pub partially_rural_designations: u64,
    pub unknown_rurality_designations: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HrsaPrimaryCareCensus {
    pub csv_download_url: String,
    pub csv_created: String,
    pub csv_bytes: u64,
    pub csv_sha256: String,
    pub csv_component_rows: u64,
    pub status_counts: BTreeMap<String, RegistryStatusCount>,
    pub designated_type_counts: BTreeMap<String, u64>,
    pub designated_single_rural_status_counts: BTreeMap<String, u64>,
    pub designated_multi_rural_status_ids: u64,
    pub designated_single_component_ids: u64,
    pub designated_multi_component_ids: u64,
    pub maximum_component_rows_per_designation: u64,
    pub designated_component_county_fips: u64,
    pub quarterly_report_url: String,
    pub quarterly_report_bytes: u64,
    pub quarterly_report_sha256: String,
    pub quarterly_summary: QuarterlyHpsaSummary,
    pub cross_vintage_reconciliation_ready: bool,
}

impl HrsaPrimaryCareCensus {
    pub fn validate(&self) -> Result<(), AccessError> {
        let status_rows: u64 = self
            .status_counts
            .values()
            .map(|value| value.component_rows)
            .sum();
        if status_rows != self.csv_component_rows {
            return Err(AccessError::Invariant(format!(
                "HPSA status rows are {status_rows}, expected {}",
                self.csv_component_rows
            )));
        }
        let designated = self.status_counts["Designated"].unique_designations;
        let type_total: u64 = self.designated_type_counts.values().sum();
        if type_total != designated {
            return Err(AccessError::Invariant(format!(
                "designated type total is {type_total}, expected {designated}"
            )));
        }
        let rural_total: u64 = self
            .designated_single_rural_status_counts
            .values()
            .sum::<u64>()
            + self.designated_multi_rural_status_ids;
        if rural_total != designated {
            return Err(AccessError::Invariant(format!(
                "designated rural-status total is {rural_total}, expected {designated}"
            )));
        }
        let component_total =
            self.designated_single_component_ids + self.designated_multi_component_ids;
        if component_total != designated {
            return Err(AccessError::Invariant(format!(
                "designation component total is {component_total}, expected {designated}"
            )));
        }
        let quarterly = &self.quarterly_summary;
        let quarterly_type_total = quarterly.geographic_area_designations
            + quarterly.population_group_designations
            + quarterly.facility_designations;
        if quarterly_type_total != quarterly.total_designations {
            return Err(AccessError::Invariant(format!(
                "quarterly designation-type total is {quarterly_type_total}, expected {}",
                quarterly.total_designations
            )));
        }
        let quarterly_rural_total = quarterly.rural_designations
            + quarterly.non_rural_designations
            + quarterly.partially_rural_designations
            + quarterly.unknown_rurality_designations;
        if quarterly_rural_total != quarterly.total_designations {
            return Err(AccessError::Invariant(format!(
                "quarterly rurality total is {quarterly_rural_total}, expected {}",
                quarterly.total_designations
            )));
        }
        if self.cross_vintage_reconciliation_ready {
            return Err(AccessError::Invariant(
                "cross-vintage reconciliation must remain held".to_string(),
            ));
        }
        Ok(())
    }
}

pub fn load_hrsa_primary_care_fixture() -> Result<HrsaPrimaryCareCensus, AccessError> {
    let result: HrsaPrimaryCareCensus = serde_json::from_str(HRSA_FIXTURE)?;
    result.validate()?;
    Ok(result)
}

pub fn hrsa_primary_care_baseline_json() -> Result<String, AccessError> {
    let result = load_hrsa_primary_care_fixture()?;
    Ok(serde_json::to_string_pretty(&json!({
        "schema":"shield.hrsa-primary-care-hpsa-census.v1",
        "current_registry":{
            "source_url":result.csv_download_url,"created":result.csv_created,
            "bytes":result.csv_bytes,"sha256":result.csv_sha256,
            "component_rows":result.csv_component_rows,"status_counts":result.status_counts,
            "designated_type_counts":result.designated_type_counts,
            "designated_single_rural_status_counts":result.designated_single_rural_status_counts,
            "designated_multi_rural_status_ids":result.designated_multi_rural_status_ids,
            "designated_single_component_ids":result.designated_single_component_ids,
            "designated_multi_component_ids":result.designated_multi_component_ids,
            "maximum_component_rows_per_designation":result.maximum_component_rows_per_designation,
            "designated_component_county_fips":result.designated_component_county_fips
        },
        "quarterly_context":{
            "source_url":result.quarterly_report_url,"bytes":result.quarterly_report_bytes,
            "sha256":result.quarterly_report_sha256,"summary":result.quarterly_summary,
            "cross_vintage_reconciliation_ready":result.cross_vintage_reconciliation_ready
        },
        "interpretation":{
            "allowed":"current registry grain, status, designation type, rural-status multiplicity, component expansion, and separately vintaged quarterly totals",
            "held":"hospital-level HPSA assignment, whole-county shortage, unique affected people, patient access, provider supply, staffed capacity, facility need met, adequacy, effects, costs, and savings",
            "boundary":"CSV rows are designation components; HPSA IDs are designations; populations may overlap; daily and quarterly vintages are not merged."
        }
    }))?)
}

pub fn hrsa_primary_care_held_pack_json() -> Result<String, AccessError> {
    let result = load_hrsa_primary_care_fixture()?;
    let designated = &result.status_counts["Designated"];
    let proposed = &result.status_counts["Proposed For Withdrawal"];
    let withdrawn = &result.status_counts["Withdrawn"];
    Ok(serde_json::to_string(&json!({
        "schema":"taxlane.lane-evidence-pack-candidate.v1",
        "identity":{"pack_id":"shield:hrsa-primary-care-hpsa-census-2026-07-31:v1","track":"HLT","domain_repository":"SHIELD","candidate_id":null,"candidate_name":null,"fiscal_owner":"TAXLANE"},
        "scope":{"geography":"HRSA primary-care HPSA registry components and national quarterly summary","population_or_network":"geographic, population-group, and facility shortage designations","ownership":"HRSA Bureau of Health Workforce designation system","time_basis":"daily CSV created 2026-07-31; quarterly designations as of 2026-06-30","unit_basis":"component rows, unique HPSA IDs, and separately vintaged summary quantities","included":"registry status, designation type, rural-status multiplicity, component expansion, and national summary","excluded":"hospital assignment, whole-county shortage, deduplicated people, access, capacity, adequacy, candidates, costs, and effects"},
        "source_custody":{"source_id":"HRSA-BCD_HPSA_FCT_DET_PC-2026-07-31 + HRSA-HPSA-QUARTERLY-2026-Q2","publisher":"Health Resources and Services Administration","source_path_or_url":result.csv_download_url,"vintage":"2026-07-31 daily CSV / 2026-06-30 quarterly designations","capture_status":"derived aggregate with status, identity, type, rurality, and component invariants","checksum_or_null":result.csv_sha256,"quarterly_checksum":result.quarterly_report_sha256},
        "problem":{"baseline_metric":"primary-care shortage designation registry","baseline_value_or_null":designated.unique_designations,"affected_population_or_exposure_or_null":null,"problem_boundary":"designations and components are not unique counties, hospitals, or people","source_component_rows":result.csv_component_rows,"designated_hpsa_ids":designated.unique_designations,"proposed_for_withdrawal_hpsa_ids":proposed.unique_designations,"withdrawn_hpsa_ids":withdrawn.unique_designations,"multi_component_designated_ids":result.designated_multi_component_ids,"multi_rural_status_designated_ids":result.designated_multi_rural_status_ids,"quarterly_total_designations":result.quarterly_summary.total_designations,"cross_vintage_reconciliation_ready":result.cross_vintage_reconciliation_ready},
        "intervention":{"mechanism":null,"implementing_owner":null,"eligibility_rule":null,"exclusions":"no HPSA designation, withdrawal, funding, workforce, facility, payment, or capital decision","existing_treatment_or_programmed_work":null},
        "outcomes":{"bounded_marginal_effect_or_null":null,"effect_population":null,"horizon":null,"uncertainty":"designation components overlap and daily and quarterly totals differ by vintage/status surface","transferability_boundary":"shortage designation does not establish a hospital's staffed services, patient access, quality, or outcome"},
        "service_floors":{"access":null,"quality_safety":null,"equity_distribution":null,"adequacy_resilience":null,"delivery_feasibility":null,"staffed_capacity":null,"affordability":null,"hospital_level_shortage":null,"do_no_harm_pass":null},
        "costs":{"price_year_or_null":null,"gross_cost_or_null":null,"implementation_cost_or_null":null,"maintenance_cost_or_null":null,"offsets_or_null":null,"dedicated_receipts_or_null":null,"state_local_private_shift_or_null":null,"net_cost_or_null":null,"public_savings":null},
        "fiscal_bridge":{"gross_public_funding_need_or_null":null,"delivery_efficiency_public_savings_or_null":null,"external_economic_benefit_or_null":null,"operator_or_private_revenue_or_null":null,"legally_dedicated_public_receipts_or_null":null,"collection_and_financing_cost_or_null":null,"net_public_fiscal_pressure_or_null":null,"revenue_authority":"none","demand_and_incidence_basis":"not established","netting_rule":"designation counts and reported shortage cannot enter Taxlane fiscal arithmetic"},
        "adaptive_pathways":{"pathway_classes":"registry baseline only","peer_goal_basis":null,"evaluation_horizons":"daily CSV and quarterly summary refresh","realization_owner_or_null":null,"transition_and_implementation_cost_or_null":null,"uncertainty_and_downside":"overlap, component grain, changing status, provider supply, access, facility mapping, and population deduplication remain unresolved","service_floor_and_distribution_result":"held","overlap_and_non_additivity":"do not add components, overlapping designation populations, status classes, or vintages","observation_cadence":"daily and quarterly source refresh","reopen_triggers":"same-vintage component-to-designation-to-geography bridge plus bounded candidate, access, capacity, outcome, cost, incidence, and delivery evidence","current_disposition":"held"},
        "delivery":{"capacity":null,"schedule":null,"milestones":null,"useful_life":null,"sunset_or_review":"refresh on HRSA registry or quarterly release"},
        "overlap":{"shared_projects":null,"shared_cost_allocation":null,"other_lane_interactions":"RUR ISF VET DEF","non_additivity_rule":"HPSA designations are shared eligibility/need context, not additive programs or spending claims"},
        "readiness":{"domain_evidence_ready":true,"candidate_bounded":false,"outcome_ready":false,"cost_ready":false,"floors_ready":false,"delivery_ready":false,"overlap_ready":false,"taxlane_admission_ready":false},
        "claim_boundaries":{"domain_finding_allowed":true,"candidate_recommendation_allowed":false,"designation_workforce_or_facility_decision_allowed":false,"savings_allowed":false,"allocation_allowed":false,"rate_change_allowed":false,"public_release_allowed":false}
    }))?)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GeographyComponentClass {
    pub code: String,
    pub component_rows: u64,
    pub designation_ids: u64,
    pub single_component_ids: u64,
    pub multi_component_ids: u64,
    pub single_county_ids: u64,
    pub multi_county_ids: u64,
    pub maximum_components_per_designation: u64,
    pub distinct_valid_county_fips: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HrsaPrimaryCareGeographyBridge {
    pub csv_download_url: String,
    pub csv_created: String,
    pub csv_bytes: u64,
    pub csv_sha256: String,
    pub designated_component_rows: u64,
    pub designated_hpsa_ids: u64,
    pub area_designation_rows: u64,
    pub area_designation_ids: u64,
    pub facility_designation_rows: u64,
    pub facility_designation_ids: u64,
    pub area_single_component_ids: u64,
    pub area_multi_component_ids: u64,
    pub area_single_county_ids: u64,
    pub area_multi_county_ids: u64,
    pub area_distinct_valid_county_fips: u64,
    pub component_classes: BTreeMap<String, GeographyComponentClass>,
    pub valid_common_county_rows: u64,
    pub valid_common_county_designation_ids: u64,
    pub distinct_valid_common_county_fips: u64,
    pub facility_valid_common_county_rows: u64,
    pub facility_valid_common_county_designation_ids: u64,
    pub facility_distinct_valid_common_county_fips: u64,
    pub geography_residual_rows: u64,
    pub geography_residual_designation_ids: u64,
    pub placeholder_common_county_rows: u64,
    pub state_prefix_inconsistent_rows: u64,
    pub area_geography_residual_rows: u64,
    pub facility_geography_residual_rows: u64,
    pub designation_component_type_mixed_ids: u64,
    pub cms_facility_join_ready: bool,
    pub patient_access_ready: bool,
}

impl HrsaPrimaryCareGeographyBridge {
    pub fn validate(&self) -> Result<(), AccessError> {
        let census = load_hrsa_primary_care_fixture()?;
        let designated = &census.status_counts["Designated"];
        if self.designated_component_rows != designated.component_rows
            || self.designated_hpsa_ids != designated.unique_designations
            || self.csv_sha256 != census.csv_sha256
        {
            return Err(AccessError::Invariant(
                "geography bridge does not match the same-vintage registry census".to_string(),
            ));
        }
        if self.area_designation_rows + self.facility_designation_rows
            != self.designated_component_rows
            || self.area_designation_ids + self.facility_designation_ids != self.designated_hpsa_ids
        {
            return Err(AccessError::Invariant(
                "area and facility geography partitions do not reconcile".to_string(),
            ));
        }
        let class_rows: u64 = self
            .component_classes
            .values()
            .map(|class| class.component_rows)
            .sum();
        let class_ids: u64 = self
            .component_classes
            .values()
            .map(|class| class.designation_ids)
            .sum();
        if class_rows != self.area_designation_rows || class_ids != self.area_designation_ids {
            return Err(AccessError::Invariant(
                "area component classes do not reconcile".to_string(),
            ));
        }
        for (name, class) in &self.component_classes {
            if class.single_component_ids + class.multi_component_ids != class.designation_ids
                || class.single_county_ids + class.multi_county_ids != class.designation_ids
            {
                return Err(AccessError::Invariant(format!(
                    "{name} designation partitions do not reconcile"
                )));
            }
        }
        if self.area_single_component_ids + self.area_multi_component_ids
            != self.area_designation_ids
            || self.area_single_county_ids + self.area_multi_county_ids != self.area_designation_ids
        {
            return Err(AccessError::Invariant(
                "area component or county partitions do not reconcile".to_string(),
            ));
        }
        if self.valid_common_county_rows + self.geography_residual_rows
            != self.designated_component_rows
            || self.valid_common_county_designation_ids + self.geography_residual_designation_ids
                != self.designated_hpsa_ids
            || self.placeholder_common_county_rows + self.state_prefix_inconsistent_rows
                != self.geography_residual_rows
            || self.area_geography_residual_rows + self.facility_geography_residual_rows
                != self.geography_residual_rows
        {
            return Err(AccessError::Invariant(
                "valid geography and residual partitions do not reconcile".to_string(),
            ));
        }
        if self.facility_valid_common_county_rows + self.facility_geography_residual_rows
            != self.facility_designation_rows
            || self.facility_valid_common_county_designation_ids
                + self.geography_residual_designation_ids
                != self.facility_designation_ids
        {
            return Err(AccessError::Invariant(
                "facility geography partition does not reconcile".to_string(),
            ));
        }
        if self.designation_component_type_mixed_ids != 0
            || self.cms_facility_join_ready
            || self.patient_access_ready
        {
            return Err(AccessError::Invariant(
                "geography bridge must not imply a facility join or access result".to_string(),
            ));
        }
        Ok(())
    }
}

pub fn load_hrsa_geography_fixture() -> Result<HrsaPrimaryCareGeographyBridge, AccessError> {
    let result: HrsaPrimaryCareGeographyBridge = serde_json::from_str(HRSA_GEOGRAPHY_FIXTURE)?;
    result.validate()?;
    Ok(result)
}

pub fn hrsa_geography_baseline_json() -> Result<String, AccessError> {
    let result = load_hrsa_geography_fixture()?;
    Ok(serde_json::to_string_pretty(&json!({
        "schema":"shield.hrsa-primary-care-geography-bridge.v1",
        "source":{"url":result.csv_download_url,"created":result.csv_created,"bytes":result.csv_bytes,"sha256":result.csv_sha256},
        "registry":{"designated_component_rows":result.designated_component_rows,"designated_hpsa_ids":result.designated_hpsa_ids},
        "area_designations":{"component_rows":result.area_designation_rows,"designation_ids":result.area_designation_ids,"single_component_ids":result.area_single_component_ids,"multi_component_ids":result.area_multi_component_ids,"single_county_ids":result.area_single_county_ids,"multi_county_ids":result.area_multi_county_ids,"distinct_valid_county_fips":result.area_distinct_valid_county_fips,"component_classes":result.component_classes},
        "facility_designations":{"component_rows":result.facility_designation_rows,"designation_ids":result.facility_designation_ids,"valid_common_county_ids":result.facility_valid_common_county_designation_ids,"distinct_valid_common_county_fips":result.facility_distinct_valid_common_county_fips},
        "county_key":{"valid_rows":result.valid_common_county_rows,"valid_designation_ids":result.valid_common_county_designation_ids,"distinct_valid_fips":result.distinct_valid_common_county_fips,"residual_rows":result.geography_residual_rows,"residual_designation_ids":result.geography_residual_designation_ids,"placeholder_rows":result.placeholder_common_county_rows,"state_prefix_inconsistent_rows":result.state_prefix_inconsistent_rows},
        "interpretation":{"allowed":"same-vintage designation-to-component-to-county-key structure and explicit geography residual","boundary":"Single County, Census Tract, County Subdivision, and facility/unknown component classes remain distinct; a county key is context, not whole-county coverage.","held":"CMS hospital join, facility equivalence, whole-county shortage, unique population, patient access, staffed capacity, adequacy, effects, costs, and savings"}
    }))?)
}

pub fn hrsa_geography_held_pack_json() -> Result<String, AccessError> {
    let result = load_hrsa_geography_fixture()?;
    Ok(serde_json::to_string(&json!({
        "schema":"taxlane.lane-evidence-pack-candidate.v1",
        "identity":{"pack_id":"shield:hrsa-primary-care-geography-bridge-2026-07-31:v1","track":"HLT","domain_repository":"SHIELD","candidate_id":null,"candidate_name":null,"fiscal_owner":"TAXLANE"},
        "scope":{"geography":"HRSA primary-care HPSA designation components and common county keys","population_or_network":"current geographic, population-group, and facility shortage designations","ownership":"HRSA Bureau of Health Workforce designation system","time_basis":"daily CSV created 2026-07-31","unit_basis":"component rows, unique HPSA IDs, component classes, and validated five-character common county FIPS keys","included":"same-vintage designation-component geography structure and residual","excluded":"CMS hospital join, whole-county shortage, unique people, access, capacity, adequacy, candidates, costs, and effects"},
        "source_custody":{"source_id":"HRSA-BCD_HPSA_FCT_DET_PC-2026-07-31","publisher":"Health Resources and Services Administration","source_path_or_url":result.csv_download_url,"vintage":result.csv_created,"capture_status":"derived aggregate with same-vintage component-class, county-key, and residual invariants","checksum_or_null":result.csv_sha256},
        "problem":{"baseline_metric":"current primary-care HPSA geography structure","baseline_value_or_null":result.designated_hpsa_ids,"affected_population_or_exposure_or_null":null,"problem_boundary":"county keys locate components but do not make tract, subdivision, population-group, or facility designations whole-county findings","area_designation_ids":result.area_designation_ids,"facility_designation_ids":result.facility_designation_ids,"multi_component_area_ids":result.area_multi_component_ids,"multi_county_area_ids":result.area_multi_county_ids,"valid_county_key_ids":result.valid_common_county_designation_ids,"geography_residual_ids":result.geography_residual_designation_ids},
        "intervention":{"mechanism":null,"implementing_owner":null,"eligibility_rule":null,"exclusions":"no HPSA, workforce, facility, payment, funding, or capital decision","existing_treatment_or_programmed_work":null},
        "outcomes":{"bounded_marginal_effect_or_null":null,"effect_population":null,"horizon":null,"uncertainty":"partial-area components, multi-county designations, facility identity, population overlap, and 18 geography residuals remain material","transferability_boundary":"a designation-component county key does not establish hospital access, staffed services, quality, or outcomes"},
        "service_floors":{"access":null,"quality_safety":null,"equity_distribution":null,"adequacy_resilience":null,"delivery_feasibility":null,"staffed_capacity":null,"affordability":null,"hospital_level_shortage":null,"do_no_harm_pass":null},
        "costs":{"price_year_or_null":null,"gross_cost_or_null":null,"implementation_cost_or_null":null,"maintenance_cost_or_null":null,"offsets_or_null":null,"dedicated_receipts_or_null":null,"state_local_private_shift_or_null":null,"net_cost_or_null":null,"public_savings":null},
        "fiscal_bridge":{"gross_public_funding_need_or_null":null,"delivery_efficiency_public_savings_or_null":null,"external_economic_benefit_or_null":null,"operator_or_private_revenue_or_null":null,"legally_dedicated_public_receipts_or_null":null,"collection_and_financing_cost_or_null":null,"net_public_fiscal_pressure_or_null":null,"revenue_authority":"none","demand_and_incidence_basis":"not established","netting_rule":"geography and designation counts cannot enter Taxlane fiscal arithmetic"},
        "adaptive_pathways":{"pathway_classes":"same-vintage geography bridge only","peer_goal_basis":null,"evaluation_horizons":"daily source refresh","realization_owner_or_null":null,"transition_and_implementation_cost_or_null":null,"uncertainty_and_downside":"geography is contextual until component coverage, compatible facility identity, access, capacity, and population overlap are resolved","service_floor_and_distribution_result":"held","overlap_and_non_additivity":"do not add components, counties, designations, or overlapping populations","observation_cadence":"daily source refresh","reopen_triggers":"compatible facility or staffed-capacity bridge plus bounded candidate, access, outcome, cost, incidence, and delivery evidence","current_disposition":"held"},
        "delivery":{"capacity":null,"schedule":null,"milestones":null,"useful_life":null,"sunset_or_review":"refresh on HRSA registry release"},
        "overlap":{"shared_projects":null,"shared_cost_allocation":null,"other_lane_interactions":"RUR ISF VET DEF","non_additivity_rule":"shared geography context is not additive program need or spending"},
        "readiness":{"domain_evidence_ready":true,"geography_bridge_ready":true,"cms_facility_join_ready":result.cms_facility_join_ready,"candidate_bounded":false,"outcome_ready":false,"cost_ready":false,"floors_ready":false,"delivery_ready":false,"overlap_ready":false,"taxlane_admission_ready":false},
        "claim_boundaries":{"domain_finding_allowed":true,"candidate_recommendation_allowed":false,"whole_county_or_hospital_shortage_allowed":false,"savings_allowed":false,"allocation_allowed":false,"rate_change_allowed":false,"public_release_allowed":false}
    }))?)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HrsaPrimaryCareDesignationCapacity {
    pub csv_download_url: String,
    pub csv_created: String,
    pub csv_bytes: u64,
    pub csv_sha256: String,
    pub metadata_url: String,
    pub metadata_bytes: u64,
    pub metadata_sha256: String,
    pub designated_hpsa_ids: u64,
    pub capacity_bearing_designation_ids: u64,
    pub area_capacity_designation_ids: u64,
    pub correctional_capacity_designation_ids: u64,
    pub capacity_excluded_designation_ids: u64,
    pub capacity_excluded_type_counts: BTreeMap<String, u64>,
    pub fte_present_designation_ids: u64,
    pub fte_zero_designation_ids: u64,
    pub fte_positive_designation_ids: u64,
    pub shortage_present_designation_ids: u64,
    pub shortage_zero_designation_ids: u64,
    pub shortage_positive_designation_ids: u64,
    pub designation_recorded_fte_e4: u64,
    pub designation_recorded_shortage_e4: u64,
    pub designation_recorded_need_met_bps: u64,
    pub area_recorded_fte_e4: u64,
    pub area_recorded_shortage_e4: u64,
    pub area_need_met_bps: u64,
    pub area_designation_population: u64,
    pub area_estimated_served_population: u64,
    pub area_estimated_underserved_population: u64,
    pub area_population_identity_exact_ids: u64,
    pub area_population_identity_residual_ids: u64,
    pub area_population_identity_residual_people: i64,
    pub area_served_formula_within_half_person_ids: u64,
    pub area_shortage_formula_within_one_hundredth_fte_ids: u64,
    pub correctional_recorded_fte_e4: u64,
    pub correctional_recorded_shortage_e4: u64,
    pub correctional_need_met_bps: u64,
    pub correctional_designation_population: u64,
    pub provider_ratio_goal_counts: BTreeMap<String, u64>,
    pub capacity_excluded_population_present_ids: u64,
    pub capacity_excluded_population_missing_ids: u64,
    pub unique_physician_supply_ready: bool,
    pub nurse_practitioner_and_physician_assistant_supply_included: bool,
    pub cms_facility_capacity_ready: bool,
    pub patient_access_ready: bool,
}

fn rounded_need_met_bps(fte_e4: u64, shortage_e4: u64) -> u64 {
    let denominator = fte_e4 + shortage_e4;
    (fte_e4 * 10_000 + denominator / 2) / denominator
}

impl HrsaPrimaryCareDesignationCapacity {
    pub fn validate(&self) -> Result<(), AccessError> {
        let census = load_hrsa_primary_care_fixture()?;
        if self.designated_hpsa_ids != census.status_counts["Designated"].unique_designations
            || self.csv_sha256 != census.csv_sha256
        {
            return Err(AccessError::Invariant(
                "capacity baseline does not match the same-vintage registry census".to_string(),
            ));
        }
        if self.capacity_bearing_designation_ids + self.capacity_excluded_designation_ids
            != self.designated_hpsa_ids
            || self.area_capacity_designation_ids + self.correctional_capacity_designation_ids
                != self.capacity_bearing_designation_ids
            || self.capacity_excluded_type_counts.values().sum::<u64>()
                != self.capacity_excluded_designation_ids
        {
            return Err(AccessError::Invariant(
                "capacity-bearing and excluded partitions do not reconcile".to_string(),
            ));
        }
        if self.fte_present_designation_ids != self.capacity_bearing_designation_ids
            || self.fte_zero_designation_ids + self.fte_positive_designation_ids
                != self.capacity_bearing_designation_ids
            || self.shortage_present_designation_ids != self.capacity_bearing_designation_ids
            || self.shortage_zero_designation_ids + self.shortage_positive_designation_ids
                != self.capacity_bearing_designation_ids
            || self.provider_ratio_goal_counts.values().sum::<u64>()
                != self.capacity_bearing_designation_ids
        {
            return Err(AccessError::Invariant(
                "capacity field-presence partitions do not reconcile".to_string(),
            ));
        }
        if self.area_recorded_fte_e4 + self.correctional_recorded_fte_e4
            != self.designation_recorded_fte_e4
            || self.area_recorded_shortage_e4 + self.correctional_recorded_shortage_e4
                != self.designation_recorded_shortage_e4
        {
            return Err(AccessError::Invariant(
                "area and correctional capacity quantities do not reconcile".to_string(),
            ));
        }
        for (label, actual, expected) in [
            (
                "all designation-recorded",
                self.designation_recorded_need_met_bps,
                rounded_need_met_bps(
                    self.designation_recorded_fte_e4,
                    self.designation_recorded_shortage_e4,
                ),
            ),
            (
                "area",
                self.area_need_met_bps,
                rounded_need_met_bps(self.area_recorded_fte_e4, self.area_recorded_shortage_e4),
            ),
            (
                "correctional",
                self.correctional_need_met_bps,
                rounded_need_met_bps(
                    self.correctional_recorded_fte_e4,
                    self.correctional_recorded_shortage_e4,
                ),
            ),
        ] {
            if actual != expected {
                return Err(AccessError::Invariant(format!(
                    "{label} need-met basis points are {actual}, expected {expected}"
                )));
            }
        }
        if self.area_population_identity_exact_ids + self.area_population_identity_residual_ids
            != self.area_capacity_designation_ids
            || self.area_designation_population as i64
                - self.area_estimated_served_population as i64
                - self.area_estimated_underserved_population as i64
                != self.area_population_identity_residual_people
            || self.area_served_formula_within_half_person_ids != self.area_capacity_designation_ids
            || self.area_shortage_formula_within_one_hundredth_fte_ids
                != self.area_capacity_designation_ids
        {
            return Err(AccessError::Invariant(
                "area population or formula checks do not reconcile".to_string(),
            ));
        }
        if self.capacity_excluded_population_present_ids
            + self.capacity_excluded_population_missing_ids
            != self.capacity_excluded_designation_ids
            || self.unique_physician_supply_ready
            || self.nurse_practitioner_and_physician_assistant_supply_included
            || self.cms_facility_capacity_ready
            || self.patient_access_ready
        {
            return Err(AccessError::Invariant(
                "capacity baseline must retain workforce and access boundaries".to_string(),
            ));
        }
        Ok(())
    }
}

pub fn load_hrsa_capacity_fixture() -> Result<HrsaPrimaryCareDesignationCapacity, AccessError> {
    let result: HrsaPrimaryCareDesignationCapacity = serde_json::from_str(HRSA_CAPACITY_FIXTURE)?;
    result.validate()?;
    Ok(result)
}

pub fn hrsa_capacity_baseline_json() -> Result<String, AccessError> {
    let result = load_hrsa_capacity_fixture()?;
    Ok(serde_json::to_string_pretty(&json!({
        "schema":"shield.hrsa-primary-care-designation-capacity.v1",
        "source":{"url":result.csv_download_url,"created":result.csv_created,"bytes":result.csv_bytes,"sha256":result.csv_sha256,"metadata_url":result.metadata_url,"metadata_bytes":result.metadata_bytes,"metadata_sha256":result.metadata_sha256},
        "coverage":{"designated_hpsa_ids":result.designated_hpsa_ids,"capacity_bearing_ids":result.capacity_bearing_designation_ids,"area_ids":result.area_capacity_designation_ids,"correctional_ids":result.correctional_capacity_designation_ids,"capacity_excluded_ids":result.capacity_excluded_designation_ids,"capacity_excluded_type_counts":result.capacity_excluded_type_counts},
        "designation_recorded_capacity":{"fte_e4":result.designation_recorded_fte_e4,"shortage_e4":result.designation_recorded_shortage_e4,"derived_need_met_bps":result.designation_recorded_need_met_bps,"fte_zero_ids":result.fte_zero_designation_ids,"fte_positive_ids":result.fte_positive_designation_ids,"shortage_zero_ids":result.shortage_zero_designation_ids,"shortage_positive_ids":result.shortage_positive_designation_ids,"provider_ratio_goal_counts":result.provider_ratio_goal_counts},
        "area_formula":{"fte_e4":result.area_recorded_fte_e4,"shortage_e4":result.area_recorded_shortage_e4,"derived_need_met_bps":result.area_need_met_bps,"designation_population":result.area_designation_population,"estimated_served_population":result.area_estimated_served_population,"estimated_underserved_population":result.area_estimated_underserved_population,"population_identity_exact_ids":result.area_population_identity_exact_ids,"population_identity_residual_ids":result.area_population_identity_residual_ids,"population_identity_residual_people":result.area_population_identity_residual_people},
        "correctional_formula":{"fte_e4":result.correctional_recorded_fte_e4,"shortage_e4":result.correctional_recorded_shortage_e4,"derived_need_met_bps":result.correctional_need_met_bps,"designation_population":result.correctional_designation_population},
        "interpretation":{"allowed":"designation-recorded primary-care physician FTE, shortage, provider-ratio, population-formula coverage, and explicit policy exclusions at current HPSA-ID grain","boundary":"quantities are designation-formula values, not deduplicated physicians, people, counties, facilities, or service-line capacity; nurse practitioners and physician assistants are excluded.","held":"unique workforce supply or need, CMS hospital capacity, patient access, adequacy, candidate effects, costs, and savings"}
    }))?)
}

pub fn hrsa_capacity_held_pack_json() -> Result<String, AccessError> {
    let result = load_hrsa_capacity_fixture()?;
    Ok(serde_json::to_string(&json!({
        "schema":"taxlane.lane-evidence-pack-candidate.v1",
        "identity":{"pack_id":"shield:hrsa-primary-care-designation-capacity-2026-07-31:v1","track":"HLT","domain_repository":"SHIELD","candidate_id":null,"candidate_name":null,"fiscal_owner":"TAXLANE"},
        "scope":{"geography":"current HRSA primary-care HPSA designations","population_or_network":"area and correctional designations with recorded primary-care physician FTE/shortage formulas","ownership":"HRSA Bureau of Health Workforce designation system","time_basis":"daily CSV created 2026-07-31","unit_basis":"unique HPSA IDs and designation-recorded physician FTE ten-thousandths","included":"formula coverage, recorded FTE and shortage, provider-ratio goals, population identities, and exclusions","excluded":"deduplicated workforce or population, NP/PA supply, CMS hospital capacity, access, adequacy, candidates, costs, and effects"},
        "source_custody":{"source_id":"HRSA-BCD_HPSA_FCT_DET_PC-2026-07-31 + HPSA-DATAMART-METADATA","publisher":"Health Resources and Services Administration","source_path_or_url":result.csv_download_url,"vintage":result.csv_created,"capture_status":"derived aggregate with same-vintage identity, field-presence, formula, and exclusion invariants","checksum_or_null":result.csv_sha256,"metadata_checksum":result.metadata_sha256},
        "problem":{"baseline_metric":"designation-recorded primary-care physician capacity and shortage formula","baseline_value_or_null":result.designation_recorded_shortage_e4,"affected_population_or_exposure_or_null":null,"problem_boundary":"FTE and shortage sums are designation-recorded quantities and may overlap; they are not unique national physicians or people","capacity_bearing_ids":result.capacity_bearing_designation_ids,"capacity_excluded_ids":result.capacity_excluded_designation_ids,"recorded_fte_e4":result.designation_recorded_fte_e4,"recorded_shortage_e4":result.designation_recorded_shortage_e4,"derived_need_met_bps":result.designation_recorded_need_met_bps,"area_population_rounding_residual_people":result.area_population_identity_residual_people},
        "intervention":{"mechanism":null,"implementing_owner":null,"eligibility_rule":null,"exclusions":"no HPSA, workforce, facility, payment, funding, or capital decision","existing_treatment_or_programmed_work":null},
        "outcomes":{"bounded_marginal_effect_or_null":null,"effect_population":null,"horizon":null,"uncertainty":"overlapping designations, excluded automatic facilities, NP/PA omission, facility identity, service breadth, and patient access remain unresolved","transferability_boundary":"designation-formula physician FTE does not establish staffed hospital services, appointment availability, quality, or outcomes"},
        "service_floors":{"access":null,"quality_safety":null,"equity_distribution":null,"adequacy_resilience":null,"delivery_feasibility":null,"staffed_capacity":null,"affordability":null,"hospital_level_shortage":null,"do_no_harm_pass":null},
        "costs":{"price_year_or_null":null,"gross_cost_or_null":null,"implementation_cost_or_null":null,"maintenance_cost_or_null":null,"offsets_or_null":null,"dedicated_receipts_or_null":null,"state_local_private_shift_or_null":null,"net_cost_or_null":null,"public_savings":null},
        "fiscal_bridge":{"gross_public_funding_need_or_null":null,"delivery_efficiency_public_savings_or_null":null,"external_economic_benefit_or_null":null,"operator_or_private_revenue_or_null":null,"legally_dedicated_public_receipts_or_null":null,"collection_and_financing_cost_or_null":null,"net_public_fiscal_pressure_or_null":null,"revenue_authority":"none","demand_and_incidence_basis":"not established","netting_rule":"designation-recorded FTE and shortage cannot enter Taxlane fiscal arithmetic"},
        "adaptive_pathways":{"pathway_classes":"designation-formula capacity baseline only","peer_goal_basis":null,"evaluation_horizons":"daily source refresh","realization_owner_or_null":null,"transition_and_implementation_cost_or_null":null,"uncertainty_and_downside":"formula coverage is incomplete and non-deduplicated and excludes NP/PA services","service_floor_and_distribution_result":"held","overlap_and_non_additivity":"do not add overlapping designation populations or treat designation-recorded FTE as unique workforce","observation_cadence":"daily source refresh","reopen_triggers":"compatible provider/site identity or staffed-service source plus bounded candidate, access, outcome, cost, incidence, and delivery evidence","current_disposition":"held"},
        "delivery":{"capacity":null,"schedule":null,"milestones":null,"useful_life":null,"sunset_or_review":"refresh on HRSA registry release"},
        "overlap":{"shared_projects":null,"shared_cost_allocation":null,"other_lane_interactions":"RUR ISF VET DEF","non_additivity_rule":"designation-formula capacity is shared context, not additive program need or spending"},
        "readiness":{"domain_evidence_ready":true,"provider_capacity_formula_ready":true,"unique_physician_supply_ready":result.unique_physician_supply_ready,"cms_facility_capacity_ready":result.cms_facility_capacity_ready,"candidate_bounded":false,"outcome_ready":false,"cost_ready":false,"floors_ready":false,"delivery_ready":false,"overlap_ready":false,"taxlane_admission_ready":false},
        "claim_boundaries":{"domain_finding_allowed":true,"candidate_recommendation_allowed":false,"unique_workforce_or_hospital_capacity_allowed":false,"savings_allowed":false,"allocation_allowed":false,"rate_change_allowed":false,"public_release_allowed":false}
    }))?)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CmsOperationalCapacitySource {
    pub publisher: String,
    pub dataset_title: String,
    pub dataset_type_id: String,
    pub dataset_version_id: String,
    pub landing_page: String,
    pub download_url: String,
    pub data_year: String,
    pub released: String,
    pub captured: String,
    pub csv_bytes: u64,
    pub csv_sha256: String,
    pub dictionary_url: String,
    pub dictionary_bytes: u64,
    pub dictionary_sha256: String,
    pub methodology_url: String,
    pub methodology_bytes: u64,
    pub methodology_sha256: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CmsHospitalOperationalCapacity {
    pub source: CmsOperationalCapacitySource,
    pub source_report_rows: u64,
    pub unique_report_ids: u64,
    pub unique_provider_ccns: u64,
    pub duplicate_ccn_groups: u64,
    pub adjacent_duplicate_period_pairs: u64,
    pub overlapping_duplicate_period_pairs: u64,
    pub available_bed_rows: u64,
    pub available_bed_day_rows: u64,
    pub inpatient_day_rows: u64,
    pub complete_operational_rows: u64,
    pub usable_operational_rows: u64,
    pub usable_operational_ccns: u64,
    pub missing_operational_rows: u64,
    pub invalid_operational_rows: u64,
    pub usable_bed_days: u64,
    pub usable_inpatient_days: u64,
    pub weighted_inpatient_use_bps: u64,
    pub standard_period_rows: u64,
    pub short_period_rows: u64,
    pub long_period_rows: u64,
    pub current_hgi_facilities: u64,
    pub current_hgi_matched_ccns: u64,
    pub current_hgi_only_facilities: u64,
    pub cost_report_only_ccns: u64,
    pub current_hgi_match_bps: u64,
    pub current_hgi_usable_operational_rows: u64,
    pub current_hgi_usable_operational_ccns: u64,
    pub current_hgi_usable_bed_days: u64,
    pub current_hgi_usable_inpatient_days: u64,
    pub current_hgi_weighted_inpatient_use_bps: u64,
    pub current_hgi_operational_coverage_bps: u64,
    pub noncurrent_hgi_usable_operational_rows: u64,
    pub noncurrent_hgi_usable_operational_ccns: u64,
    pub noncurrent_hgi_usable_bed_days: u64,
    pub noncurrent_hgi_usable_inpatient_days: u64,
    pub hgi_source_sha256: String,
    pub available_beds_are_staffed_beds: bool,
    pub point_in_time_beds_additive_across_reports: bool,
    pub facility_adequacy_ready: bool,
    pub candidate_ready: bool,
    pub taxlane_admission_ready: bool,
}

impl CmsHospitalOperationalCapacity {
    pub fn validate(&self) -> Result<(), AccessError> {
        if self.source_report_rows != self.unique_report_ids
            || self.standard_period_rows + self.short_period_rows + self.long_period_rows
                != self.source_report_rows
        {
            return Err(AccessError::Invariant(
                "cost-report identity or period partitions do not reconcile".to_string(),
            ));
        }
        if self.complete_operational_rows
            != self.usable_operational_rows + self.invalid_operational_rows
            || self.source_report_rows
                != self.complete_operational_rows + self.missing_operational_rows
        {
            return Err(AccessError::Invariant(
                "operational-field coverage does not reconcile".to_string(),
            ));
        }
        if self.current_hgi_matched_ccns + self.current_hgi_only_facilities
            != self.current_hgi_facilities
            || self.current_hgi_matched_ccns + self.cost_report_only_ccns
                != self.unique_provider_ccns
        {
            return Err(AccessError::Invariant(
                "current-footprint CCN join does not reconcile".to_string(),
            ));
        }
        if self.current_hgi_usable_operational_rows + self.noncurrent_hgi_usable_operational_rows
            != self.usable_operational_rows
            || self.current_hgi_usable_operational_ccns
                + self.noncurrent_hgi_usable_operational_ccns
                != self.usable_operational_ccns
            || self.current_hgi_usable_bed_days + self.noncurrent_hgi_usable_bed_days
                != self.usable_bed_days
            || self.current_hgi_usable_inpatient_days + self.noncurrent_hgi_usable_inpatient_days
                != self.usable_inpatient_days
        {
            return Err(AccessError::Invariant(
                "usable current-footprint partition does not reconcile".to_string(),
            ));
        }
        let rounded_bps =
            |numerator: u64, denominator: u64| (numerator * 10_000 + denominator / 2) / denominator;
        if self.weighted_inpatient_use_bps
            != rounded_bps(self.usable_inpatient_days, self.usable_bed_days)
            || self.current_hgi_weighted_inpatient_use_bps
                != rounded_bps(
                    self.current_hgi_usable_inpatient_days,
                    self.current_hgi_usable_bed_days,
                )
            || self.current_hgi_match_bps
                != rounded_bps(self.current_hgi_matched_ccns, self.current_hgi_facilities)
            || self.current_hgi_operational_coverage_bps
                != rounded_bps(
                    self.current_hgi_usable_operational_ccns,
                    self.current_hgi_facilities,
                )
        {
            return Err(AccessError::Invariant(
                "derived operational ratios do not reproduce".to_string(),
            ));
        }
        if self.usable_inpatient_days > self.usable_bed_days
            || self.current_hgi_usable_inpatient_days > self.current_hgi_usable_bed_days
            || self.overlapping_duplicate_period_pairs != 0
            || self.available_beds_are_staffed_beds
            || self.point_in_time_beds_additive_across_reports
            || self.facility_adequacy_ready
            || self.candidate_ready
            || self.taxlane_admission_ready
        {
            return Err(AccessError::Invariant(
                "operational-capacity boundaries are not preserved".to_string(),
            ));
        }
        Ok(())
    }
}

pub fn load_cms_operational_capacity_fixture() -> Result<CmsHospitalOperationalCapacity, AccessError>
{
    let result: CmsHospitalOperationalCapacity =
        serde_json::from_str(CMS_OPERATIONAL_CAPACITY_FIXTURE)?;
    result.validate()?;
    Ok(result)
}

pub fn cms_operational_capacity_baseline_json() -> Result<String, AccessError> {
    let result = load_cms_operational_capacity_fixture()?;
    Ok(serde_json::to_string(&json!({
        "schema":"shield.cms-hospital-operational-capacity.v1",
        "source":result.source,
        "report_identity":{"report_rows":result.source_report_rows,"unique_report_ids":result.unique_report_ids,"unique_provider_ccns":result.unique_provider_ccns,"duplicate_ccn_groups":result.duplicate_ccn_groups,"adjacent_duplicate_period_pairs":result.adjacent_duplicate_period_pairs,"overlapping_duplicate_period_pairs":result.overlapping_duplicate_period_pairs},
        "operational_coverage":{"available_bed_rows":result.available_bed_rows,"available_bed_day_rows":result.available_bed_day_rows,"inpatient_day_rows":result.inpatient_day_rows,"complete_rows":result.complete_operational_rows,"usable_rows":result.usable_operational_rows,"usable_ccns":result.usable_operational_ccns,"missing_rows":result.missing_operational_rows,"invalid_rows":result.invalid_operational_rows},
        "report_period_observation":{"usable_bed_days":result.usable_bed_days,"usable_inpatient_days":result.usable_inpatient_days,"weighted_inpatient_use_bps":result.weighted_inpatient_use_bps,"standard_period_rows":result.standard_period_rows,"short_period_rows":result.short_period_rows,"long_period_rows":result.long_period_rows},
        "current_footprint_join":{"current_hgi_facilities":result.current_hgi_facilities,"matched_ccns":result.current_hgi_matched_ccns,"current_hgi_only_facilities":result.current_hgi_only_facilities,"cost_report_only_ccns":result.cost_report_only_ccns,"match_bps":result.current_hgi_match_bps,"usable_operational_ccns":result.current_hgi_usable_operational_ccns,"operational_coverage_bps":result.current_hgi_operational_coverage_bps,"usable_bed_days":result.current_hgi_usable_bed_days,"usable_inpatient_days":result.current_hgi_usable_inpatient_days,"weighted_inpatient_use_bps":result.current_hgi_weighted_inpatient_use_bps},
        "interpretation":{"allowed":"CMS annual hospital report identity, adult-and-pediatric beds available for patient use, report-period available bed-days, inpatient days, bounded weighted use, and exact CCN overlap with the current CMS hospital footprint","boundary":"Available beds are not staffed beds; weighted inpatient use is not service-line availability, surge capacity, wait time, access, quality, need, or adequacy. Point-in-time bed values are not summed across repeated reports.","held":"staffed-bed capacity, service-line and workforce capacity, patient access, adequacy, candidate effects, costs, savings, allocation, and rates"}
    }))?)
}

pub fn cms_operational_capacity_held_pack_json() -> Result<String, AccessError> {
    let result = load_cms_operational_capacity_fixture()?;
    Ok(serde_json::to_string(&json!({
        "schema":"taxlane.lane-evidence-pack-candidate.v1",
        "identity":{"pack_id":"shield:cms-hospital-operational-capacity-2023:v1","track":"HLT","domain_repository":"SHIELD","candidate_id":null,"candidate_name":null,"fiscal_owner":"TAXLANE"},
        "scope":{"geography":"United States hospitals represented in the CMS 2023 Hospital Provider Cost Report","population_or_network":"Medicare-certified hospital cost-report records and exact CCN overlap with the May 13, 2026 CMS hospital footprint","ownership":"mixed hospital ownership","time_basis":"CMS 2023 annual cost-report PUF released 2026-01-08; current-footprint join released 2026-05-13","unit_basis":"cost-report records, unique CCNs, available bed-days, inpatient days, and basis points","included":"report identity, adult-and-pediatric available beds and bed-days, inpatient days, weighted report-period use, completeness, invalid residual, and exact CCN overlap","excluded":"staffed beds, service-line staffing, patients, travel, access, need, quality, outcomes, adequacy, candidates, costs, and effects"},
        "source_custody":{"source_id":result.source.dataset_version_id,"publisher":result.source.publisher,"source_path_or_url":result.source.download_url,"vintage":result.source.data_year,"capture_status":"derived aggregate with report-identity, field-coverage, period, utilization, and exact-CCN-join invariants","checksum_or_null":result.source.csv_sha256,"current_hgi_checksum":result.hgi_source_sha256},
        "problem":{"baseline_metric":"hospital report-period available-bed use","baseline_value_or_null":result.current_hgi_weighted_inpatient_use_bps,"affected_population_or_exposure_or_null":null,"problem_boundary":"weighted use across valid reports is context, not proof of local capacity, staffed availability, need, or adequacy","usable_report_rows":result.usable_operational_rows,"usable_report_ccns":result.usable_operational_ccns,"current_hgi_usable_ccns":result.current_hgi_usable_operational_ccns,"current_hgi_operational_coverage_bps":result.current_hgi_operational_coverage_bps,"current_hgi_weighted_inpatient_use_bps":result.current_hgi_weighted_inpatient_use_bps,"missing_rows":result.missing_operational_rows,"invalid_rows":result.invalid_operational_rows},
        "intervention":{"mechanism":null,"implementing_owner":null,"eligibility_rule":null,"exclusions":"no facility, staffing, payment, funding, service-line, or capital decision","existing_treatment_or_programmed_work":null},
        "outcomes":{"bounded_marginal_effect_or_null":null,"effect_population":null,"horizon":null,"uncertainty":"report-year/current-footprint vintage difference, missing and invalid reports, non-staffed bed definition, service-line mix, transfers, seasonality, and local demand remain unresolved","transferability_boundary":"aggregate report-period inpatient use cannot establish local access, surge readiness, quality, or outcomes"},
        "service_floors":{"access":null,"quality_safety":null,"equity_distribution":null,"adequacy_resilience":null,"delivery_feasibility":null,"staffed_capacity":null,"affordability":null,"service_line_capacity":null,"do_no_harm_pass":null},
        "costs":{"price_year_or_null":null,"gross_cost_or_null":null,"implementation_cost_or_null":null,"maintenance_cost_or_null":null,"offsets_or_null":null,"dedicated_receipts_or_null":null,"state_local_private_shift_or_null":null,"net_cost_or_null":null,"public_savings":null},
        "fiscal_bridge":{"gross_public_funding_need_or_null":null,"delivery_efficiency_public_savings_or_null":null,"external_economic_benefit_or_null":null,"operator_or_private_revenue_or_null":null,"legally_dedicated_public_receipts_or_null":null,"collection_and_financing_cost_or_null":null,"net_public_fiscal_pressure_or_null":null,"revenue_authority":"none","demand_and_incidence_basis":"not established","netting_rule":"available-bed utilization cannot enter Taxlane fiscal arithmetic"},
        "adaptive_pathways":{"pathway_classes":"operational baseline only","peer_goal_basis":null,"evaluation_horizons":"annual cost-report and current-footprint refresh","realization_owner_or_null":null,"transition_and_implementation_cost_or_null":null,"uncertainty_and_downside":"available beds can be unstaffed and aggregate use can conceal service-line or geographic shortages","service_floor_and_distribution_result":"held","overlap_and_non_additivity":"do not add point-in-time bed counts across multiple reports; report-period bed-days and inpatient days use only valid non-overlapping records","observation_cadence":"annual source refresh","reopen_triggers":"compatible staffed service-line, workforce, access, need, outcome, cost, incidence, and delivery evidence for a bounded candidate","current_disposition":"held"},
        "delivery":{"capacity":null,"schedule":null,"milestones":null,"useful_life":null,"sunset_or_review":"refresh on CMS cost-report or hospital-footprint release"},
        "overlap":{"shared_projects":null,"shared_cost_allocation":null,"other_lane_interactions":"RUR ISF VET DEF","non_additivity_rule":"operational observations are shared HLT context, not additive program need or spending"},
        "readiness":{"domain_evidence_ready":true,"available_bed_use_ready":true,"current_footprint_identity_ready":true,"staffed_capacity_ready":false,"service_line_capacity_ready":false,"facility_adequacy_ready":result.facility_adequacy_ready,"candidate_bounded":result.candidate_ready,"outcome_ready":false,"cost_ready":false,"floors_ready":false,"delivery_ready":false,"overlap_ready":false,"taxlane_admission_ready":result.taxlane_admission_ready},
        "claim_boundaries":{"domain_finding_allowed":true,"candidate_recommendation_allowed":false,"staffed_capacity_or_adequacy_allowed":false,"savings_allowed":false,"allocation_allowed":false,"rate_change_allowed":false,"public_release_allowed":false}
    }))?)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CmsCertifiedServicesSource {
    pub publisher: String,
    pub dataset_title: String,
    pub dataset_type_id: String,
    pub dataset_version_id: String,
    pub landing_page: String,
    pub download_url: String,
    pub vintage: String,
    pub released: String,
    pub captured: String,
    pub csv_bytes: u64,
    pub csv_sha256: String,
    pub dictionary_url: String,
    pub dictionary_bytes: u64,
    pub dictionary_sha256: String,
    pub methodology_url: String,
    pub methodology_bytes: u64,
    pub methodology_sha256: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServiceDeliveryCoverage {
    pub not_provided: u64,
    pub staff_only: u64,
    pub arrangement_only: u64,
    pub staff_and_arrangement: u64,
    pub missing: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WorkforceFieldCoverage {
    pub present: u64,
    pub missing: u64,
    pub source_recorded_zero: u64,
    pub positive: u64,
    pub negative: u64,
    pub recorded_fte_e2: u64,
    pub maximum_recorded_fte_e2: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CmsCertifiedServicesWorkforce {
    pub source: CmsCertifiedServicesSource,
    pub source_rows: u64,
    pub source_columns: u64,
    pub hospital_rows: u64,
    pub hospital_unique_ccns: u64,
    pub duplicate_hospital_ccns: u64,
    pub current_hgi_facilities: u64,
    pub current_hgi_matched_ccns: u64,
    pub current_hgi_only_facilities: u64,
    pub pos_hospital_only_ccns: u64,
    pub current_hgi_match_bps: u64,
    pub service_complete_current_ccns: u64,
    pub service_missing_current_ccns: u64,
    pub service_complete_current_bps: u64,
    pub service_missing_type_counts: BTreeMap<String, u64>,
    pub service_lines: BTreeMap<String, ServiceDeliveryCoverage>,
    pub workforce_fields: BTreeMap<String, WorkforceFieldCoverage>,
    pub service_codes_establish_current_schedule_coverage: bool,
    pub workforce_fte_are_unique_people: bool,
    pub staffed_service_capacity_ready: bool,
    pub facility_adequacy_ready: bool,
    pub candidate_ready: bool,
    pub taxlane_admission_ready: bool,
    pub hgi_source_sha256: String,
}

impl CmsCertifiedServicesWorkforce {
    pub fn validate(&self) -> Result<(), AccessError> {
        if self.hospital_rows != self.hospital_unique_ccns || self.duplicate_hospital_ccns != 0 {
            return Err(AccessError::Invariant(
                "POS hospital identity does not reconcile".to_string(),
            ));
        }
        if self.current_hgi_matched_ccns + self.current_hgi_only_facilities
            != self.current_hgi_facilities
            || self.current_hgi_matched_ccns + self.pos_hospital_only_ccns
                != self.hospital_unique_ccns
            || self.service_complete_current_ccns + self.service_missing_current_ccns
                != self.current_hgi_matched_ccns
            || self.service_missing_type_counts.values().sum::<u64>()
                != self.service_missing_current_ccns
        {
            return Err(AccessError::Invariant(
                "POS current-footprint join does not reconcile".to_string(),
            ));
        }
        for (name, line) in &self.service_lines {
            if line.not_provided
                + line.staff_only
                + line.arrangement_only
                + line.staff_and_arrangement
                + line.missing
                != self.current_hgi_matched_ccns
                || line.missing != self.service_missing_current_ccns
            {
                return Err(AccessError::Invariant(format!(
                    "service delivery partition does not reconcile: {name}"
                )));
            }
        }
        for (name, field) in &self.workforce_fields {
            if field.present + field.missing != self.current_hgi_matched_ccns
                || field.source_recorded_zero + field.positive != field.present
                || field.negative != 0
                || field.recorded_fte_e2 < field.maximum_recorded_fte_e2
            {
                return Err(AccessError::Invariant(format!(
                    "workforce field partition does not reconcile: {name}"
                )));
            }
        }
        let rounded_bps =
            |numerator: u64, denominator: u64| (numerator * 10_000 + denominator / 2) / denominator;
        if self.current_hgi_match_bps
            != rounded_bps(self.current_hgi_matched_ccns, self.current_hgi_facilities)
            || self.service_complete_current_bps
                != rounded_bps(
                    self.service_complete_current_ccns,
                    self.current_hgi_facilities,
                )
            || self.service_codes_establish_current_schedule_coverage
            || self.workforce_fte_are_unique_people
            || self.staffed_service_capacity_ready
            || self.facility_adequacy_ready
            || self.candidate_ready
            || self.taxlane_admission_ready
        {
            return Err(AccessError::Invariant(
                "certified-service and workforce claim boundaries are not preserved".to_string(),
            ));
        }
        Ok(())
    }
}

pub fn load_cms_certified_services_workforce_fixture(
) -> Result<CmsCertifiedServicesWorkforce, AccessError> {
    let result: CmsCertifiedServicesWorkforce =
        serde_json::from_str(CMS_CERTIFIED_SERVICES_WORKFORCE_FIXTURE)?;
    result.validate()?;
    Ok(result)
}

pub fn cms_certified_services_workforce_baseline_json() -> Result<String, AccessError> {
    let result = load_cms_certified_services_workforce_fixture()?;
    Ok(serde_json::to_string(&json!({
        "schema":"shield.cms-certified-services-workforce.v1",
        "source":result.source,
        "identity":{"source_rows":result.source_rows,"source_columns":result.source_columns,"hospital_rows":result.hospital_rows,"hospital_unique_ccns":result.hospital_unique_ccns,"duplicate_hospital_ccns":result.duplicate_hospital_ccns},
        "current_footprint_join":{"current_hgi_facilities":result.current_hgi_facilities,"matched_ccns":result.current_hgi_matched_ccns,"current_hgi_only_facilities":result.current_hgi_only_facilities,"pos_hospital_only_ccns":result.pos_hospital_only_ccns,"match_bps":result.current_hgi_match_bps,"service_complete_ccns":result.service_complete_current_ccns,"service_missing_ccns":result.service_missing_current_ccns,"service_complete_bps":result.service_complete_current_bps,"service_missing_type_counts":result.service_missing_type_counts},
        "service_code_legend":{"0":"not provided","1":"provided by staff","2":"provided under arrangement","3":"provided by staff and under arrangement"},
        "service_lines":result.service_lines,
        "workforce_fields":result.workforce_fields,
        "interpretation":{"allowed":"Q2 2026 CMS certification-record service delivery modes and employed clinical FTE fields at exact current-footprint CCN grain","boundary":"Certification records do not prove current shift coverage, hours, throughput, appointment supply, response time, surge readiness, unique people, or local adequacy. Source-recorded zeros remain source values, not independently verified absence.","held":"staffed service capacity, patient access, need, quality, outcomes, adequacy, candidate effects, costs, savings, allocation, and rates"}
    }))?)
}

pub fn cms_certified_services_workforce_held_pack_json() -> Result<String, AccessError> {
    let result = load_cms_certified_services_workforce_fixture()?;
    Ok(serde_json::to_string(&json!({
        "schema":"taxlane.lane-evidence-pack-candidate.v1",
        "identity":{"pack_id":"shield:cms-certified-services-workforce-2026-q2:v1","track":"HLT","domain_repository":"SHIELD","candidate_id":null,"candidate_name":null,"fiscal_owner":"TAXLANE"},
        "scope":{"geography":"current CMS hospital footprint with exact Q2 2026 QIES POS CCN matches","population_or_network":"Medicare-registered hospitals and CMS certification records","ownership":"mixed hospital ownership","time_basis":"Q2 2026 POS file released 2026-07-16 joined to Hospital General Information released 2026-05-13","unit_basis":"facility CCNs, certification service codes, and source-recorded employed FTE hundredths","included":"exact identity, fourteen service delivery modes, seven clinical workforce fields, completeness, and source residuals","excluded":"shift schedules, hours, throughput, appointment supply, travel, access, need, quality, outcomes, adequacy, candidates, costs, and effects"},
        "source_custody":{"source_id":result.source.dataset_version_id,"publisher":result.source.publisher,"source_path_or_url":result.source.download_url,"vintage":result.source.vintage,"capture_status":"derived aggregate with exact CCN, service-code, workforce-field, and residual invariants","checksum_or_null":result.source.csv_sha256,"dictionary_checksum":result.source.dictionary_sha256,"current_hgi_checksum":result.hgi_source_sha256},
        "problem":{"baseline_metric":"certification-record service delivery and employed clinical workforce spine","baseline_value_or_null":result.current_hgi_matched_ccns,"affected_population_or_exposure_or_null":null,"problem_boundary":"certification modes and employed FTE do not establish current staffed capacity or local adequacy","current_hgi_facilities":result.current_hgi_facilities,"matched_ccns":result.current_hgi_matched_ccns,"match_bps":result.current_hgi_match_bps,"service_complete_ccns":result.service_complete_current_ccns,"service_missing_ccns":result.service_missing_current_ccns,"service_complete_bps":result.service_complete_current_bps},
        "intervention":{"mechanism":null,"implementing_owner":null,"eligibility_rule":null,"exclusions":"no certification, staffing, facility, payment, funding, service-line, or capital decision","existing_treatment_or_programmed_work":null},
        "outcomes":{"bounded_marginal_effect_or_null":null,"effect_population":null,"horizon":null,"uncertainty":"certification timing, source-recorded zeros, employee-only workforce scope, arrangements, shared clinicians, scheduling, throughput, catchments, and demand remain unresolved","transferability_boundary":"service codes and employed FTE do not establish patient access, service quality, surge readiness, or outcomes"},
        "service_floors":{"access":null,"quality_safety":null,"equity_distribution":null,"adequacy_resilience":null,"delivery_feasibility":null,"staffed_capacity":null,"affordability":null,"service_line_capacity":null,"do_no_harm_pass":null},
        "costs":{"price_year_or_null":null,"gross_cost_or_null":null,"implementation_cost_or_null":null,"maintenance_cost_or_null":null,"offsets_or_null":null,"dedicated_receipts_or_null":null,"state_local_private_shift_or_null":null,"net_cost_or_null":null,"public_savings":null},
        "fiscal_bridge":{"gross_public_funding_need_or_null":null,"delivery_efficiency_public_savings_or_null":null,"external_economic_benefit_or_null":null,"operator_or_private_revenue_or_null":null,"legally_dedicated_public_receipts_or_null":null,"collection_and_financing_cost_or_null":null,"net_public_fiscal_pressure_or_null":null,"revenue_authority":"none","demand_and_incidence_basis":"not established","netting_rule":"certification service codes and employed FTE cannot enter Taxlane fiscal arithmetic"},
        "adaptive_pathways":{"pathway_classes":"certified service and workforce registry only","peer_goal_basis":null,"evaluation_horizons":"quarterly POS and current-footprint refresh","realization_owner_or_null":null,"transition_and_implementation_cost_or_null":null,"uncertainty_and_downside":"registry presence can conceal unavailable shifts, insufficient throughput, shared clinicians, or geographic mismatch","service_floor_and_distribution_result":"held","overlap_and_non_additivity":"do not add FTE fields as unique people or service codes as capacity units","observation_cadence":"quarterly source refresh","reopen_triggers":"compatible schedules, throughput, access, need, outcome, cost, incidence, and delivery evidence for a bounded candidate","current_disposition":"held"},
        "delivery":{"capacity":null,"schedule":null,"milestones":null,"useful_life":null,"sunset_or_review":"refresh on CMS POS or hospital-footprint release"},
        "overlap":{"shared_projects":null,"shared_cost_allocation":null,"other_lane_interactions":"RUR ISF VET DEF","non_additivity_rule":"certification and workforce observations are shared HLT context, not additive program need or spending"},
        "readiness":{"domain_evidence_ready":true,"exact_ccn_identity_ready":true,"certified_service_delivery_ready":true,"employed_workforce_fields_ready":true,"current_schedule_coverage_ready":result.service_codes_establish_current_schedule_coverage,"unique_workforce_ready":result.workforce_fte_are_unique_people,"staffed_service_capacity_ready":result.staffed_service_capacity_ready,"facility_adequacy_ready":result.facility_adequacy_ready,"candidate_bounded":result.candidate_ready,"outcome_ready":false,"cost_ready":false,"floors_ready":false,"delivery_ready":false,"overlap_ready":false,"taxlane_admission_ready":result.taxlane_admission_ready},
        "claim_boundaries":{"domain_finding_allowed":true,"candidate_recommendation_allowed":false,"current_staffed_capacity_or_adequacy_allowed":false,"savings_allowed":false,"allocation_allowed":false,"rate_change_allowed":false,"public_release_allowed":false}
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

    #[test]
    fn hrsa_registry_reconciles_rows_ids_types_and_components() {
        let result = load_hrsa_primary_care_fixture().unwrap();
        assert_eq!(result.csv_component_rows, 79_150);
        assert_eq!(
            result.status_counts["Designated"].unique_designations,
            7_682
        );
        assert_eq!(result.designated_multi_component_ids, 762);
        assert_eq!(result.designated_multi_rural_status_ids, 282);
    }

    #[test]
    fn hrsa_quarterly_snapshot_stays_separate_and_reconciles() {
        let result = load_hrsa_primary_care_fixture().unwrap();
        assert_eq!(result.quarterly_summary.total_designations, 9_003);
        assert_eq!(result.quarterly_summary.practitioners_needed, 18_541);
        assert!(!result.cross_vintage_reconciliation_ready);
    }

    #[test]
    fn hrsa_baseline_states_component_and_overlap_boundaries() {
        let output = hrsa_primary_care_baseline_json().unwrap();
        assert!(output.contains("CSV rows are designation components"));
        assert!(output.contains("populations may overlap"));
    }

    #[test]
    fn hrsa_pack_does_not_assign_shortage_or_fiscal_authority() {
        let output: serde_json::Value =
            serde_json::from_str(&hrsa_primary_care_held_pack_json().unwrap()).unwrap();
        assert_eq!(output["identity"]["track"], "HLT");
        assert_eq!(output["problem"]["designated_hpsa_ids"], 7_682);
        assert_eq!(
            output["problem"]["cross_vintage_reconciliation_ready"],
            false
        );
        assert_eq!(
            output["service_floors"]["hospital_level_shortage"],
            serde_json::Value::Null
        );
        assert_eq!(output["costs"]["public_savings"], serde_json::Value::Null);
        assert_eq!(output["readiness"]["taxlane_admission_ready"], false);
    }

    #[test]
    fn hrsa_geography_bridge_reconciles_area_and_facility_grains() {
        let result = load_hrsa_geography_fixture().unwrap();
        assert_eq!(result.designated_hpsa_ids, 7_682);
        assert_eq!(result.area_designation_ids, 2_838);
        assert_eq!(result.facility_designation_ids, 4_844);
        assert_eq!(result.area_multi_component_ids, 762);
        assert_eq!(result.area_multi_county_ids, 155);
    }

    #[test]
    fn hrsa_geography_bridge_preserves_component_classes() {
        let result = load_hrsa_geography_fixture().unwrap();
        assert_eq!(
            result.component_classes["Single County"].designation_ids,
            2_088
        );
        assert_eq!(
            result.component_classes["Census Tract"].component_rows,
            11_697
        );
        assert_eq!(
            result.component_classes["County Subdivision"].designation_ids,
            164
        );
        assert_eq!(result.designation_component_type_mixed_ids, 0);
    }

    #[test]
    fn hrsa_geography_bridge_keeps_invalid_keys_visible() {
        let result = load_hrsa_geography_fixture().unwrap();
        assert_eq!(result.valid_common_county_designation_ids, 7_664);
        assert_eq!(result.geography_residual_designation_ids, 18);
        assert_eq!(result.placeholder_common_county_rows, 17);
        assert_eq!(result.state_prefix_inconsistent_rows, 1);
    }

    #[test]
    fn hrsa_geography_pack_does_not_promote_context_to_access_or_savings() {
        let output: serde_json::Value =
            serde_json::from_str(&hrsa_geography_held_pack_json().unwrap()).unwrap();
        assert_eq!(output["identity"]["track"], "HLT");
        assert_eq!(output["readiness"]["geography_bridge_ready"], true);
        assert_eq!(output["readiness"]["cms_facility_join_ready"], false);
        assert_eq!(output["service_floors"]["access"], serde_json::Value::Null);
        assert_eq!(output["costs"]["public_savings"], serde_json::Value::Null);
        assert_eq!(output["readiness"]["taxlane_admission_ready"], false);
    }

    #[test]
    fn hrsa_capacity_partitions_formula_coverage_and_exclusions() {
        let result = load_hrsa_capacity_fixture().unwrap();
        assert_eq!(result.capacity_bearing_designation_ids, 3_388);
        assert_eq!(result.area_capacity_designation_ids, 2_838);
        assert_eq!(result.correctional_capacity_designation_ids, 550);
        assert_eq!(result.capacity_excluded_designation_ids, 4_294);
    }

    #[test]
    fn hrsa_capacity_reconciles_fte_shortage_and_need_met() {
        let result = load_hrsa_capacity_fixture().unwrap();
        assert_eq!(result.designation_recorded_fte_e4, 106_354_884);
        assert_eq!(result.designation_recorded_shortage_e4, 122_670_916);
        assert_eq!(result.designation_recorded_need_met_bps, 4_644);
        assert_eq!(result.fte_zero_designation_ids, 805);
    }

    #[test]
    fn hrsa_area_capacity_formula_keeps_rounding_residual_visible() {
        let result = load_hrsa_capacity_fixture().unwrap();
        assert_eq!(result.area_population_identity_exact_ids, 2_837);
        assert_eq!(result.area_population_identity_residual_ids, 1);
        assert_eq!(result.area_population_identity_residual_people, -1);
        assert_eq!(
            result.area_shortage_formula_within_one_hundredth_fte_ids,
            2_838
        );
    }

    #[test]
    fn hrsa_capacity_pack_does_not_claim_unique_workforce_or_savings() {
        let output: serde_json::Value =
            serde_json::from_str(&hrsa_capacity_held_pack_json().unwrap()).unwrap();
        assert_eq!(output["identity"]["track"], "HLT");
        assert_eq!(output["readiness"]["provider_capacity_formula_ready"], true);
        assert_eq!(output["readiness"]["unique_physician_supply_ready"], false);
        assert_eq!(
            output["service_floors"]["staffed_capacity"],
            serde_json::Value::Null
        );
        assert_eq!(output["costs"]["public_savings"], serde_json::Value::Null);
        assert_eq!(output["readiness"]["taxlane_admission_ready"], false);
    }

    #[test]
    fn cms_operational_capacity_reconciles_report_identity_and_periods() {
        let result = load_cms_operational_capacity_fixture().unwrap();
        assert_eq!(result.source_report_rows, 6_103);
        assert_eq!(result.unique_report_ids, 6_103);
        assert_eq!(result.unique_provider_ccns, 6_040);
        assert_eq!(result.duplicate_ccn_groups, 62);
        assert_eq!(result.adjacent_duplicate_period_pairs, 63);
        assert_eq!(result.overlapping_duplicate_period_pairs, 0);
    }

    #[test]
    fn cms_operational_capacity_preserves_missing_and_invalid_residuals() {
        let result = load_cms_operational_capacity_fixture().unwrap();
        assert_eq!(result.complete_operational_rows, 5_978);
        assert_eq!(result.usable_operational_rows, 5_953);
        assert_eq!(result.missing_operational_rows, 125);
        assert_eq!(result.invalid_operational_rows, 25);
        assert_eq!(result.weighted_inpatient_use_bps, 6_256);
    }

    #[test]
    fn cms_operational_capacity_joins_current_footprint_by_exact_ccn() {
        let result = load_cms_operational_capacity_fixture().unwrap();
        assert_eq!(result.current_hgi_facilities, 5_432);
        assert_eq!(result.current_hgi_matched_ccns, 5_144);
        assert_eq!(result.current_hgi_usable_operational_ccns, 5_032);
        assert_eq!(result.current_hgi_operational_coverage_bps, 9_264);
        assert_eq!(result.current_hgi_weighted_inpatient_use_bps, 6_233);
    }

    #[test]
    fn cms_operational_capacity_pack_holds_staffing_adequacy_and_savings() {
        let output: serde_json::Value =
            serde_json::from_str(&cms_operational_capacity_held_pack_json().unwrap()).unwrap();
        assert_eq!(output["identity"]["track"], "HLT");
        assert_eq!(output["readiness"]["available_bed_use_ready"], true);
        assert_eq!(output["readiness"]["staffed_capacity_ready"], false);
        assert_eq!(
            output["service_floors"]["staffed_capacity"],
            serde_json::Value::Null
        );
        assert_eq!(output["costs"]["public_savings"], serde_json::Value::Null);
        assert_eq!(output["readiness"]["taxlane_admission_ready"], false);
    }

    #[test]
    fn cms_certified_services_join_current_footprint_by_exact_ccn() {
        let result = load_cms_certified_services_workforce_fixture().unwrap();
        assert_eq!(result.hospital_rows, 13_566);
        assert_eq!(result.hospital_unique_ccns, 13_566);
        assert_eq!(result.current_hgi_facilities, 5_432);
        assert_eq!(result.current_hgi_matched_ccns, 5_422);
        assert_eq!(result.current_hgi_match_bps, 9_982);
    }

    #[test]
    fn cms_certified_service_modes_reconcile_with_federal_residual() {
        let result = load_cms_certified_services_workforce_fixture().unwrap();
        assert_eq!(result.service_complete_current_ccns, 5_286);
        assert_eq!(result.service_missing_current_ccns, 136);
        assert_eq!(
            result.service_missing_type_counts["Acute Care - Veterans Administration"],
            112
        );
        let emergency = &result.service_lines["dedicated_emergency_department"];
        assert_eq!(emergency.staff_only, 2_494);
        assert_eq!(emergency.arrangement_only, 209);
        assert_eq!(emergency.staff_and_arrangement, 1_651);
    }

    #[test]
    fn cms_workforce_fields_preserve_recorded_zero_and_outlier_boundaries() {
        let result = load_cms_certified_services_workforce_fixture().unwrap();
        let rn = &result.workforce_fields["registered_nurse"];
        assert_eq!(rn.present, 5_422);
        assert_eq!(rn.source_recorded_zero, 325);
        assert_eq!(rn.positive, 5_097);
        assert_eq!(rn.recorded_fte_e2, 122_496_228);
        assert_eq!(rn.maximum_recorded_fte_e2, 6_478_200);
        assert!(!result.workforce_fte_are_unique_people);
    }

    #[test]
    fn cms_certified_services_pack_holds_current_capacity_and_savings() {
        let output: serde_json::Value =
            serde_json::from_str(&cms_certified_services_workforce_held_pack_json().unwrap())
                .unwrap();
        assert_eq!(output["identity"]["track"], "HLT");
        assert_eq!(
            output["readiness"]["certified_service_delivery_ready"],
            true
        );
        assert_eq!(
            output["readiness"]["current_schedule_coverage_ready"],
            false
        );
        assert_eq!(output["readiness"]["staffed_service_capacity_ready"], false);
        assert_eq!(
            output["service_floors"]["staffed_capacity"],
            serde_json::Value::Null
        );
        assert_eq!(output["costs"]["public_savings"], serde_json::Value::Null);
        assert_eq!(output["readiness"]["taxlane_admission_ready"], false);
    }
}
