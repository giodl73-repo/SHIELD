use serde::{Deserialize, Serialize};

/// Service tier classification, T1 most demanding through T4 baseline.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tier {
    T1,
    T2,
    T3,
    T4,
}

impl Tier {
    pub fn label(&self) -> &'static str {
        match self {
            Tier::T1 => "T1",
            Tier::T2 => "T2",
            Tier::T3 => "T3",
            Tier::T4 => "T4",
        }
    }
}

/// Provisional service-level agreement targets for a tier.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Sla {
    pub access_time: f64,
    pub capacity_beds: f64,
    pub service_breadth: f64,
    pub outcomes: f64,
}

/// Hardcoded provisional SLA table; T1 is most demanding and T4 baseline.
pub fn provisional_sla(tier: Tier) -> Sla {
    match tier {
        Tier::T1 => Sla {
            access_time: 10.0,
            capacity_beds: 400.0,
            service_breadth: 1.0,
            outcomes: 0.95,
        },
        Tier::T2 => Sla {
            access_time: 20.0,
            capacity_beds: 200.0,
            service_breadth: 0.8,
            outcomes: 0.90,
        },
        Tier::T3 => Sla {
            access_time: 40.0,
            capacity_beds: 100.0,
            service_breadth: 0.6,
            outcomes: 0.85,
        },
        Tier::T4 => Sla {
            access_time: 60.0,
            capacity_beds: 50.0,
            service_breadth: 0.4,
            outcomes: 0.80,
        },
    }
}

/// Classify an entry by its declared tier string, defaulting to T4.
pub fn classify(entry: &shield_corpus::CorpusEntry) -> Tier {
    match entry.tier.as_deref() {
        Some("T1") => Tier::T1,
        Some("T2") => Tier::T2,
        Some("T3") => Tier::T3,
        Some("T4") => Tier::T4,
        _ => Tier::T4,
    }
}

/// Dimension 13: tier/SLA conformance assessment for a corpus entry.
#[derive(Clone, Copy, Debug)]
pub struct Dim13 {
    pub score: shield_score::Score,
    pub basis: shield_corpus::DemandBasis,
    pub redundancy: bool,
}

fn observed_beds(entry: &shield_corpus::CorpusEntry) -> f64 {
    entry
        .quantities
        .iter()
        .find(|q| q.unit.to_lowercase().contains("beds"))
        .map_or(0.0, |q| q.value)
}

/// Compute the tier/SLA conformance dimension for an entry within a network.
pub fn conformance(entry: &shield_corpus::CorpusEntry, network: &shield_network::Network) -> Dim13 {
    let required = provisional_sla(classify(entry));
    let observed = observed_beds(entry);
    let redundancy = matches!(network.degree(&entry.id), Some(d) if d >= 2);

    let mut result = (observed / required.capacity_beds).min(1.0) * 10.0;
    if !redundancy {
        result -= 2.0;
    }

    Dim13 {
        score: shield_score::Score::clamped(result),
        basis: shield_corpus::DemandBasis::Surge,
        redundancy,
    }
}

/// Reported gap between a tier's required bed capacity and observed beds.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TierSlaGap {
    pub entry_id: String,
    pub tier: Tier,
    pub required_beds: f64,
    pub observed_beds: f64,
    pub label: shield_corpus::EvidenceLabel,
}

/// Report a provisional tier/SLA gap when observed beds fall short of the target.
pub fn tier_sla_gap(entry: &shield_corpus::CorpusEntry) -> Option<TierSlaGap> {
    let tier = classify(entry);
    let required = provisional_sla(tier);
    let observed = observed_beds(entry);

    if observed < required.capacity_beds {
        Some(TierSlaGap {
            entry_id: entry.id.clone(),
            tier,
            required_beds: required.capacity_beds,
            observed_beds: observed,
            label: shield_corpus::EvidenceLabel::Provisional,
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn score_value(score: &shield_score::Score) -> f64 {
        let rendered = format!("{:?}", score);
        let start = rendered.find('(').map_or(0, |i| i + 1);
        let end = rendered.rfind(')').unwrap_or(rendered.len());
        rendered[start..end].trim().parse::<f64>().unwrap()
    }

    fn build_network() -> shield_network::Network {
        let mut net = shield_network::Network::new();
        net.add_facility(shield_network::Facility {
            id: String::from("A"),
            name: String::from("A"),
            role: shield_network::FacilityRole::Hospital,
        })
        .unwrap();
        net.add_facility(shield_network::Facility {
            id: String::from("B"),
            name: String::from("B"),
            role: shield_network::FacilityRole::Clinic,
        })
        .unwrap();
        net.add_facility(shield_network::Facility {
            id: String::from("C"),
            name: String::from("C"),
            role: shield_network::FacilityRole::Clinic,
        })
        .unwrap();
        net.add_referral(
            "A",
            "B",
            shield_network::Referral {
                id: String::from("r1"),
                capacity_beds: 10.0,
                basis: shield_network::DemandBasis::Surge,
            },
        )
        .unwrap();
        net.add_referral(
            "B",
            "C",
            shield_network::Referral {
                id: String::from("r2"),
                capacity_beds: 5.0,
                basis: shield_network::DemandBasis::Surge,
            },
        )
        .unwrap();
        net.add_referral(
            "A",
            "C",
            shield_network::Referral {
                id: String::from("r3"),
                capacity_beds: 7.0,
                basis: shield_network::DemandBasis::Surge,
            },
        )
        .unwrap();
        net
    }

    fn entry_with(id: &str, tier: Option<&str>, beds: f64) -> shield_corpus::CorpusEntry {
        shield_corpus::CorpusEntry {
            id: String::from(id),
            tier: tier.map(String::from),
            quantities: vec![shield_corpus::Quantity {
                value: beds,
                unit: String::from("beds"),
                label: shield_corpus::EvidenceLabel::Cited,
                source_id: Some(String::from("s1")),
            }],
            ..Default::default()
        }
    }

    #[test]
    fn classify_maps_tiers_and_defaults_to_t4() {
        let mk = |t: Option<&str>| shield_corpus::CorpusEntry {
            tier: t.map(String::from),
            ..Default::default()
        };
        assert_eq!(classify(&mk(Some("T1"))), Tier::T1);
        assert_eq!(classify(&mk(Some("T2"))), Tier::T2);
        assert_eq!(classify(&mk(Some("T3"))), Tier::T3);
        assert_eq!(classify(&mk(Some("T4"))), Tier::T4);
        assert_eq!(classify(&mk(None)), Tier::T4);
    }

    #[test]
    fn conforming_element_has_no_gap_and_high_score() {
        let net = build_network();
        let entry = shield_corpus::CorpusEntry {
            id: String::from("A"),
            tier: Some(String::from("T1")),
            quantities: vec![shield_corpus::Quantity {
                value: 500.0,
                unit: String::from("beds"),
                label: shield_corpus::EvidenceLabel::Cited,
                source_id: Some(String::from("s1")),
            }],
            ..Default::default()
        };

        assert!(tier_sla_gap(&entry).is_none());

        let dim = conformance(&entry, &net);
        assert!(dim.redundancy);
        assert!(score_value(&dim.score) >= 9.0);
    }

    #[test]
    fn shortfall_yields_provisional_gap() {
        let entry = entry_with("A", Some("T1"), 100.0);
        let gap = tier_sla_gap(&entry).unwrap();
        assert_eq!(gap.tier, Tier::T1);
        assert_eq!(gap.entry_id, "A");
        assert!(matches!(
            gap.label,
            shield_corpus::EvidenceLabel::Provisional
        ));
    }

    #[test]
    fn isolated_facility_scores_lower_than_diverse_path() {
        let net = build_network();

        let diverse = entry_with("A", Some("T4"), 50.0);
        let isolated = entry_with("Z", Some("T4"), 50.0);

        let diverse_dim = conformance(&diverse, &net);
        let isolated_dim = conformance(&isolated, &net);

        assert!(diverse_dim.redundancy);
        assert!(!isolated_dim.redundancy);
        assert!(score_value(&isolated_dim.score) < score_value(&diverse_dim.score));
    }
}
