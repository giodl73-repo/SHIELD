//! Frozen public-aggregate source queries, without clinical/access inference.
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub const MAX_INPUT_BYTES: usize = 65_536;

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum View {
    Footprint,
    Rurality,
    Registry,
    Geography,
    Capacity,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Query {
    pub view: View,
    pub jurisdiction: Option<String>,
    pub status: Option<String>,
    pub geography_cohort: Option<String>,
    pub capacity_cohort: Option<String>,
}

pub fn evaluate(input: &str) -> Result<String, String> {
    if input.len() > MAX_INPUT_BYTES {
        return Err("Use at most 64 KiB of source-query JSON".into());
    }
    let query: Query = serde_json::from_str(input).map_err(|error| error.to_string())?;
    if query.view == View::Capacity {
        return capacity(query);
    }
    if query.capacity_cohort.is_some() {
        return Err("A capacity cohort is supported only in the capacity view".into());
    }
    if query.view == View::Geography {
        return geography(query);
    }
    if query.geography_cohort.is_some() {
        return Err("A geography cohort is supported only in the geography view".into());
    }
    if query.view == View::Registry {
        return registry(query);
    }
    if query.status.is_some() {
        return Err("A registry status filter is supported only in the registry view".into());
    }
    let footprint = shield_cms_access::load_fixture().map_err(|error| error.to_string())?;
    if footprint.total_facilities == 0 {
        return Err("The source denominator is not established".into());
    }
    if query.view == View::Rurality && query.jurisdiction.is_some() {
        return Err("The rurality view has a national county-class grain; no state cross-tab is established".into());
    }
    let selected = match &query.jurisdiction {
        Some(code) => *footprint
            .states_and_territories
            .get(code)
            .ok_or_else(|| "Unknown jurisdiction key in this frozen source".to_string())?,
        None => footprint.total_facilities,
    };
    let (baseline_canonical, held_canonical) = match query.view {
        View::Footprint => (
            shield_cms_access::baseline_json(),
            shield_cms_access::held_pack_json(),
        ),
        View::Rurality => (
            shield_cms_access::rurality_baseline_json(),
            shield_cms_access::rurality_held_pack_json(),
        ),
        View::Capacity => unreachable!("capacity queries return before CMS loading"),
        View::Geography => unreachable!("geography queries return before CMS loading"),
        View::Registry => unreachable!("registry queries return before CMS loading"),
    };
    let baseline_canonical = baseline_canonical.map_err(|error| error.to_string())?;
    let held_canonical = held_canonical.map_err(|error| error.to_string())?;
    let baseline: Value =
        serde_json::from_str(&baseline_canonical).map_err(|error| error.to_string())?;
    let held: Value = serde_json::from_str(&held_canonical).map_err(|error| error.to_string())?;
    let projection = if query.view == View::Footprint {
        json!({
            "jurisdiction":query.jurisdiction,"registered_facilities":selected,"national_denominator":footprint.total_facilities,
            "share_of_source_bps":(selected*10_000+footprint.total_facilities/2)/footprint.total_facilities,
            "state_by_type":null,"state_emergency_flags":null,"actual_access":null,
            "interpretation":"a source subset, not per-capita need or service capacity; type and emergency partitions below stay national"
        })
    } else {
        Value::Null
    };
    serde_json::to_string(&json!({
        "schema":"shield.frozen-source-explorer.v1","view":query.view,"source_period":"CMS release 2026-05-13; USDA RUCC 2023; frozen repository capture",
        "projection":projection,"native_baseline":baseline,"native_baseline_canonical":baseline_canonical,"held_pack":held,"jurisdiction_keys":footprint.states_and_territories.keys().collect::<Vec<_>>(),
        "authority":{"clinical":false,"facility_action":false,"payer":false,"savings":false,"allocation":false,"rate_change":false}
    })).map_err(|error|error.to_string())
}

fn registry(query: Query) -> Result<String, String> {
    if query.jurisdiction.is_some() {
        return Err("The frozen registry has a national status grain; no jurisdiction cross-tab is established".into());
    }
    let source =
        shield_cms_access::load_hrsa_primary_care_fixture().map_err(|error| error.to_string())?;
    let selected = query
        .status
        .as_ref()
        .map(|status| {
            source
                .status_counts
                .get(status)
                .ok_or_else(|| "Unknown status in this frozen registry".to_string())
        })
        .transpose()?;
    let baseline_canonical =
        shield_cms_access::hrsa_primary_care_baseline_json().map_err(|error| error.to_string())?;
    let held_canonical =
        shield_cms_access::hrsa_primary_care_held_pack_json().map_err(|error| error.to_string())?;
    let baseline: Value =
        serde_json::from_str(&baseline_canonical).map_err(|error| error.to_string())?;
    let held: Value = serde_json::from_str(&held_canonical).map_err(|error| error.to_string())?;
    serde_json::to_string(&json!({
        "schema":"shield.frozen-registry-explorer.v1","view":"registry",
        "source_period":"HRSA daily CSV created 2026-07-31; quarterly designations as of 2026-06-30, data as of 2026-07-01; frozen repository aggregate",
        "projection":{
            "status":query.status,
            "component_rows":selected.map_or(source.csv_component_rows,|count|count.component_rows),
            "unique_designations_within_status":selected.map(|count|count.unique_designations),
            "all_status_unique_designations":null,"registered_hospitals":null,"unique_affected_people":null,
            "actual_access":null,"staffed_capacity":null,"status_by_type":null,"status_by_rurality":null,
            "cross_vintage_comparison":null,
            "interpretation":"component rows are not designations; IDs are counted within a selected status, not summed across statuses; Designated partitions stay separate from the selected status"
        },
        "native_baseline":baseline,"native_baseline_canonical":baseline_canonical,"held_pack":held,
        "jurisdiction_keys":[],"registry_status_keys":source.status_counts.keys().collect::<Vec<_>>(),
        "portable_source_custody":{"aggregate_fixture":"registry-source.json","original_primary_bytes_in_bundle":false,"primary_hashes_reverified_by_this_work":false,"publisher_urls_may_return_newer_releases":true},
        "authority":{"clinical":false,"designation":false,"workforce":false,"facility_action":false,"payer":false,"funding":false,"savings":false,"allocation":false,"rate_change":false}
    }))
    .map_err(|error| error.to_string())
}

fn geography(query: Query) -> Result<String, String> {
    if query.jurisdiction.is_some() || query.status.is_some() {
        return Err("The frozen geography bridge is Designated-only national context; status and jurisdiction filters are unsupported".into());
    }
    let source =
        shield_cms_access::load_hrsa_geography_fixture().map_err(|error| error.to_string())?;
    let (rows, ids, keys, residual) = match query.geography_cohort.as_deref() {
        None => (
            source.designated_component_rows,
            source.designated_hpsa_ids,
            source.distinct_valid_common_county_fips,
            Some(source.geography_residual_rows),
        ),
        Some("Area designations") => (
            source.area_designation_rows,
            source.area_designation_ids,
            source.area_distinct_valid_county_fips,
            Some(source.area_geography_residual_rows),
        ),
        Some("Facility designations") => (
            source.facility_designation_rows,
            source.facility_designation_ids,
            source.facility_distinct_valid_common_county_fips,
            Some(source.facility_geography_residual_rows),
        ),
        Some(name) => {
            let class = source
                .component_classes
                .get(name)
                .ok_or_else(|| "Unknown cohort in this frozen geography bridge".to_string())?;
            (
                class.component_rows,
                class.designation_ids,
                class.distinct_valid_county_fips,
                None,
            )
        }
    };
    let baseline_canonical =
        shield_cms_access::hrsa_geography_baseline_json().map_err(|error| error.to_string())?;
    let held_canonical =
        shield_cms_access::hrsa_geography_held_pack_json().map_err(|error| error.to_string())?;
    let baseline: Value =
        serde_json::from_str(&baseline_canonical).map_err(|error| error.to_string())?;
    let held: Value = serde_json::from_str(&held_canonical).map_err(|error| error.to_string())?;
    let mut cohorts = vec!["Area designations", "Facility designations"];
    cohorts.extend(source.component_classes.keys().map(String::as_str));
    serde_json::to_string(&json!({
        "schema":"shield.frozen-geography-explorer.v1","view":"geography",
        "source_period":"HRSA daily CSV created 2026-07-31; Designated cohort only; frozen repository geography aggregate",
        "projection":{"geography_cohort":query.geography_cohort,"component_rows":rows,"designation_ids":ids,"distinct_valid_county_keys":keys,"county_key_residual_rows":residual,
            "whole_county_coverage":null,"registered_hospitals":null,"unique_affected_people":null,"actual_access":null,"staffed_capacity":null,"per_county_counts":null},
        "native_baseline":baseline,"native_baseline_canonical":baseline_canonical,"held_pack":held,
        "jurisdiction_keys":[],"geography_cohort_keys":cohorts,
        "residual_partition":{"area_rows":source.area_geography_residual_rows,"facility_rows":source.facility_geography_residual_rows},
        "portable_source_custody":{"aggregate_fixture":"geography-source.json","original_primary_bytes_in_bundle":false,"primary_hashes_reverified_by_this_work":false,"publisher_urls_may_return_newer_releases":true},
        "authority":{"clinical":false,"designation":false,"workforce":false,"facility_action":false,"payer":false,"funding":false,"savings":false,"allocation":false,"rate_change":false}
    })).map_err(|error|error.to_string())
}

fn capacity_row(
    label: &str,
    ids: u64,
    fte: Option<u64>,
    shortage: Option<u64>,
    bps: Option<u64>,
) -> Value {
    let decimal = |value: u64| format!("{}.{:04}", value / 10_000, value % 10_000);
    json!({"cohort_label":label,"designation_ids":ids,"fte_e4":fte,"shortage_e4":shortage,"derived_need_met_bps":bps,
        "fte_display":fte.map(decimal),"shortage_display":shortage.map(decimal),
        "need_met_display":bps.map(|value|format!("{}.{:02}%",value/100,value%100))})
}
fn capacity(query: Query) -> Result<String, String> {
    if query.jurisdiction.is_some() || query.status.is_some() || query.geography_cohort.is_some() {
        return Err("The frozen capacity baseline is Designated-only national formula context; status, jurisdiction and geography filters are unsupported".into());
    }
    let source =
        shield_cms_access::load_hrsa_capacity_fixture().map_err(|error| error.to_string())?;
    let selected = match query.capacity_cohort.as_deref() {
        None => 0,
        Some("Area designations") => 1,
        Some("Correctional designations") => 2,
        Some("Policy-excluded facilities") => 3,
        Some(_) => return Err("Unknown cohort in this frozen designation-formula baseline".into()),
    };
    let rows = vec![
        capacity_row(
            "All formula-bearing designations",
            source.capacity_bearing_designation_ids,
            Some(source.designation_recorded_fte_e4),
            Some(source.designation_recorded_shortage_e4),
            Some(source.designation_recorded_need_met_bps),
        ),
        capacity_row(
            "Area designations",
            source.area_capacity_designation_ids,
            Some(source.area_recorded_fte_e4),
            Some(source.area_recorded_shortage_e4),
            Some(source.area_need_met_bps),
        ),
        capacity_row(
            "Correctional designations",
            source.correctional_capacity_designation_ids,
            Some(source.correctional_recorded_fte_e4),
            Some(source.correctional_recorded_shortage_e4),
            Some(source.correctional_need_met_bps),
        ),
        capacity_row(
            "Policy-excluded facilities",
            source.capacity_excluded_designation_ids,
            None,
            None,
            None,
        ),
    ];
    let mut projection = rows[selected].clone();
    projection["capacity_cohort"] = json!(query.capacity_cohort);
    for key in [
        "unique_physician_supply",
        "unique_affected_people",
        "cms_hospital_staffing",
        "staffed_service_lines",
        "actual_access",
        "adequacy",
        "fiscal_savings",
    ] {
        projection[key] = Value::Null;
    }
    let baseline_canonical =
        shield_cms_access::hrsa_capacity_baseline_json().map_err(|error| error.to_string())?;
    let held_canonical =
        shield_cms_access::hrsa_capacity_held_pack_json().map_err(|error| error.to_string())?;
    let baseline: Value =
        serde_json::from_str(&baseline_canonical).map_err(|error| error.to_string())?;
    let held: Value = serde_json::from_str(&held_canonical).map_err(|error| error.to_string())?;
    serde_json::to_string(&json!({
        "schema":"shield.frozen-capacity-explorer.v1","view":"capacity",
        "source_period":"HRSA daily CSV created 2026-07-31; Designated HPSA-ID formula/exclusion cohorts; frozen repository aggregate",
        "projection":projection,"fixed_formula_rows":rows,
        "native_baseline":baseline,"native_baseline_canonical":baseline_canonical,"held_pack":held,
        "jurisdiction_keys":[],"capacity_cohort_keys":["Area designations","Correctional designations","Policy-excluded facilities"],
        "area_formula_checks":{"served_within_half_person_ids":source.area_served_formula_within_half_person_ids,"shortage_within_one_hundredth_fte_ids":source.area_shortage_formula_within_one_hundredth_fte_ids},
        "field_coverage":{"fte_present_ids":source.fte_present_designation_ids,"shortage_present_ids":source.shortage_present_designation_ids,"excluded_population_present_ids":source.capacity_excluded_population_present_ids,"excluded_population_missing_ids":source.capacity_excluded_population_missing_ids},
        "supply_boundaries":{"unique_physician_supply_ready":source.unique_physician_supply_ready,"np_pa_supply_included":source.nurse_practitioner_and_physician_assistant_supply_included,"cms_facility_capacity_ready":source.cms_facility_capacity_ready,"patient_access_ready":source.patient_access_ready},
        "portable_source_custody":{"aggregate_fixture":"capacity-source.json","original_primary_bytes_in_bundle":false,"primary_hashes_reverified_by_this_work":false,"publisher_urls_may_return_newer_releases":true},
        "authority":{"clinical":false,"designation":false,"workforce":false,"facility_action":false,"payer":false,"funding":false,"savings":false,"allocation":false,"rate_change":false}
    })).map_err(|error|error.to_string())
}

pub fn response(input: &str) -> String {
    match evaluate(input){Ok(canonical)=>json!({"ok":true,"canonical":canonical,"result":serde_json::from_str::<Value>(&canonical).expect("serialized result")}).to_string(),Err(error)=>json!({"ok":false,"error":error}).to_string()}
}
#[cfg(target_arch = "wasm32")]
mod abi;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn capacity_cohorts_keep_exact_values_and_exclusions_null() {
        for (cohort, ids, fte, shortage, bps, display) in [
            (
                None,
                3388,
                Some(106354884),
                Some(122670916),
                Some(4644),
                Some("10635.4884"),
            ),
            (
                Some("Area designations"),
                2838,
                Some(103273034),
                Some(114984866),
                Some(4732),
                Some("10327.3034"),
            ),
            (
                Some("Correctional designations"),
                550,
                Some(3081850),
                Some(7686050),
                Some(2862),
                Some("308.1850"),
            ),
            (
                Some("Policy-excluded facilities"),
                4294,
                None,
                None,
                None,
                None,
            ),
        ] {
            let result: Value = serde_json::from_str(
                &evaluate(&json!({"view":"capacity","capacity_cohort":cohort}).to_string())
                    .unwrap(),
            )
            .unwrap();
            assert_eq!(result["projection"]["designation_ids"], ids);
            assert_eq!(result["projection"]["fte_e4"], json!(fte));
            assert_eq!(result["projection"]["shortage_e4"], json!(shortage));
            assert_eq!(result["projection"]["derived_need_met_bps"], json!(bps));
            assert_eq!(result["projection"]["fte_display"], json!(display));
            assert_eq!(
                result["native_baseline_canonical"],
                shield_cms_access::hrsa_capacity_baseline_json().unwrap()
            );
            assert_eq!(
                result["native_baseline"]["area_formula"]["population_identity_residual_people"],
                -1
            );
            assert_eq!(result["fixed_formula_rows"].as_array().unwrap().len(), 4);
        }
    }
    #[test]
    fn capacity_formulas_do_not_establish_unique_workforce_or_staffed_care() {
        let result: Value =
            serde_json::from_str(&evaluate(r#"{"view":"capacity"}"#).unwrap()).unwrap();
        for key in [
            "unique_physician_supply",
            "unique_affected_people",
            "cms_hospital_staffing",
            "staffed_service_lines",
            "actual_access",
            "adequacy",
            "fiscal_savings",
        ] {
            assert!(result["projection"][key].is_null());
        }
        assert!(result["supply_boundaries"]
            .as_object()
            .unwrap()
            .values()
            .all(|value| value == false));
        assert!(result["authority"]
            .as_object()
            .unwrap()
            .values()
            .all(|value| value == false));
        assert!(result["held_pack"]["service_floors"]
            .as_object()
            .unwrap()
            .values()
            .all(Value::is_null));
        assert_eq!(
            result["held_pack"]["readiness"]["taxlane_admission_ready"],
            false
        );
        assert_eq!(
            result["native_baseline"]["designation_recorded_capacity"]["fte_zero_ids"],
            805
        );
        assert_eq!(
            result["native_baseline"]["designation_recorded_capacity"]["shortage_zero_ids"],
            1
        );
        assert_eq!(
            result["area_formula_checks"]["served_within_half_person_ids"],
            2838
        );
        assert_eq!(
            result["field_coverage"]["excluded_population_missing_ids"],
            10
        );
    }
    #[test]
    fn capacity_rejects_cross_view_filters_and_unsupported_grains() {
        for input in [
            r#"{"view":"capacity","status":"Designated"}"#,
            r#"{"view":"capacity","jurisdiction":"CA"}"#,
            r#"{"view":"capacity","geography_cohort":"Area designations"}"#,
            r#"{"view":"capacity","capacity_cohort":""}"#,
            r#"{"view":"capacity","capacity_cohort":"all"}"#,
            r#"{"view":"capacity","provider_id":"example"}"#,
            r#"{"view":"footprint","capacity_cohort":"Area designations"}"#,
            r#"{"view":"rurality","capacity_cohort":"Area designations"}"#,
            r#"{"view":"registry","capacity_cohort":"Area designations"}"#,
            r#"{"view":"geography","capacity_cohort":"Area designations"}"#,
        ] {
            assert!(evaluate(input).is_err(), "{input}");
        }
    }

    #[test]
    fn geography_cohorts_preserve_rows_ids_and_non_additive_keys() {
        for (cohort, rows, ids, keys, residual) in [
            (None, 20368, 7682, 2932, Some(18)),
            (Some("Area designations"), 15524, 2838, 2616, Some(0)),
            (Some("Facility designations"), 4844, 4844, 1879, Some(18)),
            (Some("Census Tract"), 11697, 586, 236, None),
            (Some("County Subdivision"), 1579, 164, 149, None),
            (Some("Single County"), 2248, 2088, 2248, None),
        ] {
            let result: Value = serde_json::from_str(
                &evaluate(&json!({"view":"geography","geography_cohort":cohort}).to_string())
                    .unwrap(),
            )
            .unwrap();
            assert_eq!(result["projection"]["component_rows"], rows);
            assert_eq!(result["projection"]["designation_ids"], ids);
            assert_eq!(result["projection"]["distinct_valid_county_keys"], keys);
            assert_eq!(
                result["projection"]["county_key_residual_rows"],
                json!(residual)
            );
            assert_eq!(
                result["native_baseline_canonical"],
                shield_cms_access::hrsa_geography_baseline_json().unwrap()
            );
            assert_eq!(result["native_baseline"]["county_key"]["residual_rows"], 18);
            assert_eq!(
                result["native_baseline"]["area_designations"]["multi_county_ids"],
                155
            );
            assert_eq!(
                result["native_baseline"]["area_designations"]["component_classes"]
                    ["Single County"]["multi_county_ids"],
                89
            );
        }
    }
    #[test]
    fn geography_context_does_not_promote_hospital_identity_or_service_authority() {
        let result: Value =
            serde_json::from_str(&evaluate(r#"{"view":"geography"}"#).unwrap()).unwrap();
        for key in [
            "whole_county_coverage",
            "registered_hospitals",
            "unique_affected_people",
            "actual_access",
            "staffed_capacity",
            "per_county_counts",
        ] {
            assert!(result["projection"][key].is_null());
        }
        assert!(result["held_pack"]["service_floors"]
            .as_object()
            .unwrap()
            .values()
            .all(Value::is_null));
        assert!(result["authority"]
            .as_object()
            .unwrap()
            .values()
            .all(|value| value == false));
        assert_eq!(
            result["held_pack"]["readiness"]["cms_facility_join_ready"],
            false
        );
        assert_eq!(
            result["held_pack"]["readiness"]["taxlane_admission_ready"],
            false
        );
        assert_eq!(
            result["portable_source_custody"]["original_primary_bytes_in_bundle"],
            false
        );
    }
    #[test]
    fn geography_rejects_unsupported_grains_and_cross_view_cohorts() {
        for input in [
            r#"{"view":"geography","status":"Designated"}"#,
            r#"{"view":"geography","jurisdiction":"CA"}"#,
            r#"{"view":"geography","geography_cohort":"UNK"}"#,
            r#"{"view":"geography","geography_cohort":""}"#,
            r#"{"view":"geography","county_fips":"06001"}"#,
            r#"{"view":"footprint","geography_cohort":"Census Tract"}"#,
            r#"{"view":"rurality","geography_cohort":"Census Tract"}"#,
            r#"{"view":"registry","geography_cohort":"Census Tract"}"#,
        ] {
            assert!(evaluate(input).is_err(), "{input}");
        }
    }

    #[test]
    fn registry_rows_are_not_an_invented_all_status_id_union() {
        let result: Value =
            serde_json::from_str(&evaluate(r#"{"view":"registry","jurisdiction":null}"#).unwrap())
                .unwrap();
        assert_eq!(result["projection"]["component_rows"], 79150);
        assert!(result["projection"]["unique_designations_within_status"].is_null());
        assert!(result["projection"]["all_status_unique_designations"].is_null());
        assert_eq!(
            result["native_baseline_canonical"],
            shield_cms_access::hrsa_primary_care_baseline_json().unwrap()
        );
        assert_eq!(
            result["native_baseline"]["quarterly_context"]["summary"]["total_designations"],
            9003
        );
        assert_eq!(
            result["native_baseline"]["quarterly_context"]["cross_vintage_reconciliation_ready"],
            false
        );
        assert!(result["projection"]["cross_vintage_comparison"].is_null());
    }

    #[test]
    fn each_registry_status_preserves_fixed_designated_cohorts_and_holds() {
        for (status, rows, ids) in [
            ("Designated", 20368, 7682),
            ("Proposed For Withdrawal", 9962, 1014),
            ("Withdrawn", 48820, 8999),
        ] {
            let result: Value = serde_json::from_str(
                &evaluate(
                    &json!({"view":"registry","jurisdiction":null,"status":status}).to_string(),
                )
                .unwrap(),
            )
            .unwrap();
            assert_eq!(result["projection"]["component_rows"], rows);
            assert_eq!(
                result["projection"]["unique_designations_within_status"],
                ids
            );
            assert_eq!(
                result["native_baseline"]["current_registry"]["designated_multi_component_ids"],
                762
            );
            assert_eq!(
                result["native_baseline"]["current_registry"]["designated_multi_rural_status_ids"],
                282
            );
            assert!(result["projection"]["status_by_type"].is_null());
            assert!(result["projection"]["status_by_rurality"].is_null());
            assert!(result["projection"]["registered_hospitals"].is_null());
            assert!(result["held_pack"]["service_floors"]
                .as_object()
                .unwrap()
                .values()
                .all(Value::is_null));
            assert_eq!(
                result["held_pack"]["readiness"]["taxlane_admission_ready"],
                false
            );
            assert!(result["authority"]
                .as_object()
                .unwrap()
                .values()
                .all(|value| value == false));
        }
    }

    #[test]
    fn registry_status_and_jurisdiction_filters_do_not_cross_source_grains() {
        for input in [
            r#"{"view":"registry","jurisdiction":"CA"}"#,
            r#"{"view":"registry","jurisdiction":null,"status":"Unknown"}"#,
            r#"{"view":"registry","jurisdiction":null,"status":""}"#,
            r#"{"view":"footprint","jurisdiction":null,"status":"Designated"}"#,
            r#"{"view":"rurality","jurisdiction":null,"status":"Withdrawn"}"#,
            r#"{"view":"registry","jurisdiction":null,"facility_id":"example"}"#,
        ] {
            assert!(evaluate(input).is_err(), "{input}");
        }
    }
    #[test]
    fn preserves_native_baseline_and_held_pack() {
        let result: Value =
            serde_json::from_str(&evaluate(r#"{"view":"footprint","jurisdiction":null}"#).unwrap())
                .unwrap();
        assert_eq!(
            result["native_baseline_canonical"],
            shield_cms_access::baseline_json().unwrap()
        );
        assert_eq!(result["projection"]["registered_facilities"], 5432);
        assert!(result["projection"]["actual_access"].is_null());
        assert_eq!(
            result["held_pack"]["claim_boundaries"]["savings_allowed"],
            false
        );
    }
    #[test]
    fn selected_jurisdiction_is_a_source_subset_not_an_invented_crosstab() {
        let result: Value =
            serde_json::from_str(&evaluate(r#"{"view":"footprint","jurisdiction":"CA"}"#).unwrap())
                .unwrap();
        let source = shield_cms_access::load_fixture().unwrap();
        assert_eq!(
            result["projection"]["registered_facilities"],
            source.states_and_territories["CA"]
        );
        assert!(result["projection"]["state_by_type"].is_null());
    }
    #[test]
    fn rurality_keeps_the_unmatched_residual() {
        let result: Value =
            serde_json::from_str(&evaluate(r#"{"view":"rurality","jurisdiction":null}"#).unwrap())
                .unwrap();
        assert_eq!(
            result["native_baseline_canonical"],
            shield_cms_access::rurality_baseline_json().unwrap()
        );
        assert_eq!(
            result["native_baseline"]["join_result"]["unmatched_facilities"],
            72
        );
        assert!(result["held_pack"]["service_floors"]["access"].is_null());
    }
    #[test]
    fn rejects_unknown_queries_and_unsupported_grains() {
        for input in [
            r#"{"view":"footprint","jurisdiction":"XX"}"#,
            r#"{"view":"rurality","jurisdiction":"CA"}"#,
            r#"{"view":"footprint","person_name":"example"}"#,
        ] {
            assert!(evaluate(input).is_err());
        }
        assert!(evaluate(&"x".repeat(65537)).is_err());
    }
}
