//! Frozen public-aggregate source queries, without clinical/access inference.
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub const MAX_INPUT_BYTES: usize = 65_536;

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum View {
    Footprint,
    Rurality,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Query {
    pub view: View,
    pub jurisdiction: Option<String>,
}

pub fn evaluate(input: &str) -> Result<String, String> {
    if input.len() > MAX_INPUT_BYTES {
        return Err("Use at most 64 KiB of source-query JSON".into());
    }
    let query: Query = serde_json::from_str(input).map_err(|error| error.to_string())?;
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
pub fn response(input: &str) -> String {
    match evaluate(input){Ok(canonical)=>json!({"ok":true,"canonical":canonical,"result":serde_json::from_str::<Value>(&canonical).expect("serialized result")}).to_string(),Err(error)=>json!({"ok":false,"error":error}).to_string()}
}
#[cfg(target_arch = "wasm32")]
mod abi;

#[cfg(test)]
mod tests {
    use super::*;
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
