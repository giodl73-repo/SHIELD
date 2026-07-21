use std::collections::BTreeMap;
use std::fmt;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Scale {
    International,
    National,
    Regional,
    Local,
}

impl Scale {
    pub fn parse(s: &str) -> Option<Scale> {
        match s {
            "international" => Some(Scale::International),
            "national" => Some(Scale::National),
            "regional" => Some(Scale::Regional),
            "local" => Some(Scale::Local),
            _ => None,
        }
    }
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Scale::International => "international",
            Scale::National => "national",
            Scale::Regional => "regional",
            Scale::Local => "local",
        };
        f.write_str(name)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceLabel {
    Estimated,
    Cited,
    Validated,
    Provisional,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DemandBasis {
    Surge,
    Baseline,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Quantity {
    pub value: f64,
    pub unit: String,
    pub label: EvidenceLabel,
    pub source_id: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CorpusEntry {
    pub id: String,
    pub kind: String,
    pub scale: Option<Scale>,
    pub jurisdiction: String,
    pub tier: Option<String>,
    pub sla: Option<String>,
    pub quantities: Vec<Quantity>,
    pub scores: BTreeMap<String, f64>,
}

#[derive(Debug, Error)]
pub enum CorpusError {
    #[error("corpus entry is missing a required id")]
    MissingId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HoldReason {
    UncitedQuantity(String),
    MissingScale,
}

impl CorpusEntry {
    pub fn validate(&self) -> Result<Vec<HoldReason>, CorpusError> {
        if self.id.is_empty() {
            return Err(CorpusError::MissingId);
        }

        let mut reasons = Vec::new();

        if self.scale.is_none() {
            reasons.push(HoldReason::MissingScale);
        }

        for quantity in &self.quantities {
            if quantity.source_id.is_none() {
                reasons.push(HoldReason::UncitedQuantity(quantity.unit.clone()));
            }
        }

        Ok(reasons)
    }

    pub fn from_markdown(input: &str) -> Result<CorpusEntry, CorpusError> {
        let mut id = String::new();
        let mut kind = String::new();
        let mut scale: Option<Scale> = None;
        let mut jurisdiction = String::new();
        let mut tier: Option<String> = None;

        let mut in_frontmatter = false;
        let mut seen_open = false;

        for line in input.lines() {
            let trimmed = line.trim();

            if trimmed == "---" {
                if !seen_open {
                    seen_open = true;
                    in_frontmatter = true;
                    continue;
                }
                break;
            }

            if !in_frontmatter {
                continue;
            }

            if let Some((raw_key, raw_value)) = trimmed.split_once(':') {
                let key = raw_key.trim();
                let value = raw_value.trim();

                match key {
                    "id" => id = value.to_string(),
                    "kind" | "type" => kind = value.to_string(),
                    "scale" => scale = Scale::parse(value),
                    "jurisdiction" => jurisdiction = value.to_string(),
                    "tier" => {
                        tier = if value.is_empty() {
                            None
                        } else {
                            Some(value.to_string())
                        }
                    }
                    _ => {}
                }
            }
        }

        if id.is_empty() {
            return Err(CorpusError::MissingId);
        }

        Ok(CorpusEntry {
            id,
            kind,
            scale,
            jurisdiction,
            tier,
            ..Default::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_id_rejected() {
        let entry = CorpusEntry::default();
        let result = entry.validate();
        assert!(matches!(result, Err(CorpusError::MissingId)));
    }

    #[test]
    fn missing_scale_held() {
        let entry = CorpusEntry {
            id: "e1".to_string(),
            ..Default::default()
        };
        let reasons = entry.validate().unwrap();
        assert!(reasons.contains(&HoldReason::MissingScale));
    }

    #[test]
    fn uncited_quantity_held() {
        let entry = CorpusEntry {
            id: "e1".to_string(),
            scale: Some(Scale::National),
            quantities: vec![Quantity {
                value: 10.0,
                unit: "beds".to_string(),
                label: EvidenceLabel::Estimated,
                source_id: None,
            }],
            ..Default::default()
        };
        let reasons = entry.validate().unwrap();
        assert!(reasons.contains(&HoldReason::UncitedQuantity("beds".to_string())));
    }

    #[test]
    fn evidence_label_unchanged_after_validate() {
        let entry = CorpusEntry {
            id: "e1".to_string(),
            scale: Some(Scale::National),
            quantities: vec![Quantity {
                value: 10.0,
                unit: "beds".to_string(),
                label: EvidenceLabel::Validated,
                source_id: None,
            }],
            ..Default::default()
        };
        let _ = entry.validate().unwrap();
        assert_eq!(entry.quantities[0].label, EvidenceLabel::Validated);
    }

    #[test]
    fn from_markdown_parses_frontmatter() {
        let input =
            "---\nid: e1\nkind: facility\nscale: national\njurisdiction: us\n---\nbody text\n";
        let entry = CorpusEntry::from_markdown(input).unwrap();
        assert_eq!(entry.id, "e1");
        assert_eq!(entry.scale, Some(Scale::National));
    }

    #[test]
    fn demand_basis_roundtrips() {
        let basis = DemandBasis::Surge;
        assert_eq!(basis, DemandBasis::Surge);
        assert_ne!(basis, DemandBasis::Baseline);
    }
}
