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
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Query {
    pub view: View,
    pub jurisdiction: Option<String>,
    pub status: Option<String>,
}

pub fn evaluate(input: &str) -> Result<String, String> {
    if input.len() > MAX_INPUT_BYTES {
        return Err("Use at most 64 KiB of source-query JSON".into());
    }
    let query: Query = serde_json::from_str(input).map_err(|error| error.to_string())?;
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
pub fn response(input: &str) -> String {
    match evaluate(input){Ok(canonical)=>json!({"ok":true,"canonical":canonical,"result":serde_json::from_str::<Value>(&canonical).expect("serialized result")}).to_string(),Err(error)=>json!({"ok":false,"error":error}).to_string()}
}
#[cfg(target_arch = "wasm32")]
mod abi;

#[cfg(test)]
mod tests {
    use super::*;
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
