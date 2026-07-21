//! Gap analysis for the SHIELD project.
//!
//! This crate locates "gap regions" — dimensions of a market scale whose
//! provisional mean or under-served tail score falls below the adequacy
//! threshold of `5.0` — and correlates them with tier SLA gaps for the same
//! entries.
//!
//! # Cross-scale analysis
//!
//! By default [`find_gaps`] restricts the corpus to entries whose
//! [`shield_corpus::Scale`] equals the requested scale. Cross-scale analysis,
//! which considers entries of every scale, requires passing `cross_scale =
//! true`. With `cross_scale = false` only entries matching the requested scale
//! are selected.

use serde::{Deserialize, Serialize};
use shield_score::DimensionScorer;

/// Adequacy threshold; dimensions with a mean score below this are gaps.
const ADEQUACY_THRESHOLD: f64 = 5.0;

/// Share of scored entries below threshold at or above which a dispersion gap is
/// reclassified from a concentrated *tail* to a *systemic* deficit.
const SYSTEMIC_SHARE: f64 = 0.5;

/// A single dimension/scale pairing whose provisional mean score is inadequate.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GapRegion {
    pub dimension: shield_score::Dimension,
    pub scale: shield_corpus::Scale,
    pub mean_score: f64,
    pub member_ids: Vec<String>,
    pub label: shield_corpus::EvidenceLabel,
}

/// A dimension whose under-served tail falls below the adequacy threshold even
/// if the corpus mean does not.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TailGapRegion {
    pub dimension: shield_score::Dimension,
    pub scale: shield_corpus::Scale,
    pub tail_mean: f64,
    pub tail_member_ids: Vec<String>,
    /// Fraction of scored entries below threshold. A small share is a genuine
    /// tail (act on `tail_member_ids`); a large share is a systemic deficit.
    pub share_below_threshold: f64,
    /// True when the share crosses [`SYSTEMIC_SHARE`]: the deficit is the
    /// majority, so "tail" understates it and the whole class needs the upgrade.
    pub systemic: bool,
    pub label: shield_corpus::EvidenceLabel,
}

/// The complete result of a gap analysis over a corpus at a given scale.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GapAnalysis {
    pub scale: shield_corpus::Scale,
    pub regions: Vec<GapRegion>,
    pub tail_regions: Vec<TailGapRegion>,
    pub tier_sla_gaps: Vec<shield_tier::TierSlaGap>,
    pub null_result: bool,
}

/// Analyze `corpus` for gap regions at `scale`.
///
/// Selection: when `cross_scale` is `false` only entries whose scale equals
/// `Some(scale)` are considered; when `true` every entry is considered.
///
/// Each selected entry is scored on every [`shield_score::Dimension`] using
/// [`shield_score::ProvisionalScorer`]. For each dimension the mean score
/// across the selected entries is computed; when it falls below `5.0` a
/// [`GapRegion`] is emitted for that dimension and scale. Dimensions whose
/// bottom quartile is inadequate emit a [`TailGapRegion`]. Both are labelled
/// [`shield_corpus::EvidenceLabel::Provisional`]. The subset of `tier_gaps`
/// whose `entry_id` matches a selected entry is collected. `null_result` is
/// `true` when there are neither mean regions, tail regions, nor collected tier
/// gaps.
///
/// The `rubric` is retained for provenance.
///
/// Cross-scale analysis (considering entries of every scale) requires
/// `cross_scale = true`.
pub fn find_gaps(
    corpus: &[shield_corpus::CorpusEntry],
    rubric: &shield_score::Rubric,
    scale: shield_corpus::Scale,
    tier_gaps: &[shield_tier::TierSlaGap],
    cross_scale: bool,
) -> GapAnalysis {
    let _ = rubric;

    let selected: Vec<&shield_corpus::CorpusEntry> = corpus
        .iter()
        .filter(|entry| cross_scale || entry.scale == Some(scale))
        .collect();

    let member_ids: Vec<String> = selected.iter().map(|entry| entry.id.clone()).collect();

    let scorer = shield_score::ProvisionalScorer::default();
    let mut regions = Vec::new();
    let mut tail_regions = Vec::new();

    for dimension in shield_score::Dimension::all() {
        let count = selected.len();
        if count == 0 {
            break;
        }

        let mut scored: Vec<(&str, f64)> = selected
            .iter()
            .map(|entry| (entry.id.as_str(), scorer.score(entry, dimension).value()))
            .collect();
        let mean = scored.iter().map(|(_, value)| value).sum::<f64>() / count as f64;

        if mean < ADEQUACY_THRESHOLD {
            regions.push(GapRegion {
                dimension,
                scale,
                mean_score: mean,
                member_ids: member_ids.clone(),
                label: shield_corpus::EvidenceLabel::Provisional,
            });
        }

        let under: Vec<String> = scored
            .iter()
            .filter(|(_, value)| *value < ADEQUACY_THRESHOLD)
            .map(|(id, _)| (*id).to_string())
            .collect();
        if !under.is_empty() {
            scored.sort_by(|a, b| a.1.total_cmp(&b.1));
            let quartile = selected.len().div_ceil(4).max(1);
            let tail_mean = scored
                .iter()
                .take(quartile)
                .map(|(_, value)| value)
                .sum::<f64>()
                / quartile as f64;
            if tail_mean < ADEQUACY_THRESHOLD {
                let share = under.len() as f64 / count as f64;
                tail_regions.push(TailGapRegion {
                    dimension,
                    scale,
                    tail_mean,
                    tail_member_ids: under,
                    share_below_threshold: share,
                    systemic: share >= SYSTEMIC_SHARE,
                    label: shield_corpus::EvidenceLabel::Provisional,
                });
            }
        }
    }

    let tier_sla_gaps: Vec<shield_tier::TierSlaGap> = tier_gaps
        .iter()
        .filter(|gap| selected.iter().any(|entry| entry.id == gap.entry_id))
        .cloned()
        .collect();

    let null_result = regions.is_empty() && tail_regions.is_empty() && tier_sla_gaps.is_empty();

    GapAnalysis {
        scale,
        regions,
        tail_regions,
        tier_sla_gaps,
        null_result,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry_with(
        id: &str,
        scale: Option<shield_corpus::Scale>,
        score: f64,
    ) -> shield_corpus::CorpusEntry {
        let mut scores = std::collections::BTreeMap::new();
        for dim in shield_score::Dimension::all() {
            scores.insert(String::from(dim.code()), score);
        }
        shield_corpus::CorpusEntry {
            id: String::from(id),
            scale,
            scores,
            ..Default::default()
        }
    }

    #[test]
    fn low_scores_produce_gap_regions() {
        let rubric = shield_score::Rubric::v0();
        let corpus = vec![entry_with("A", Some(shield_corpus::Scale::National), 2.0)];

        let analysis = find_gaps(&corpus, &rubric, shield_corpus::Scale::National, &[], false);

        assert!(!analysis.regions.is_empty());
        assert!(!analysis.null_result);
        let region = analysis.regions.first().unwrap();
        assert_eq!(region.scale, shield_corpus::Scale::National);
        assert_eq!(region.member_ids, vec![String::from("A")]);
    }

    #[test]
    fn adequate_market_yields_null_result() {
        let rubric = shield_score::Rubric::v0();
        let corpus = vec![entry_with("A", Some(shield_corpus::Scale::National), 5.0)];

        let analysis = find_gaps(&corpus, &rubric, shield_corpus::Scale::National, &[], false);

        assert!(analysis.regions.is_empty());
        assert!(analysis.tail_regions.is_empty());
        assert!(analysis.tier_sla_gaps.is_empty());
        assert!(analysis.null_result);
    }

    #[test]
    fn split_corpus_flags_tail_gap_even_when_mean_clears_bar() {
        let rubric = shield_score::Rubric::v0();
        let corpus = vec![
            entry_with("low1", Some(shield_corpus::Scale::Regional), 1.0),
            entry_with("low2", Some(shield_corpus::Scale::Regional), 1.0),
            entry_with("high1", Some(shield_corpus::Scale::Regional), 9.0),
            entry_with("high2", Some(shield_corpus::Scale::Regional), 9.0),
        ];

        let analysis = find_gaps(&corpus, &rubric, shield_corpus::Scale::Regional, &[], false);

        assert!(
            analysis.regions.is_empty(),
            "mean is 5.0, not below the bar"
        );
        assert!(!analysis.tail_regions.is_empty(), "the tail is inadequate");
        assert!(!analysis.null_result);
        let tail = analysis.tail_regions.first().unwrap();
        assert!(tail.tail_mean < 5.0);
        assert!(tail.tail_member_ids.contains(&"low1".to_string()));
        assert!(!tail.tail_member_ids.contains(&"high1".to_string()));
    }

    #[test]
    fn adequate_market_has_no_tail_gap() {
        let rubric = shield_score::Rubric::v0();
        let corpus = vec![
            entry_with("e1", Some(shield_corpus::Scale::National), 7.0),
            entry_with("e2", Some(shield_corpus::Scale::National), 5.0),
        ];

        let analysis = find_gaps(&corpus, &rubric, shield_corpus::Scale::National, &[], false);

        assert!(analysis.regions.is_empty());
        assert!(analysis.tail_regions.is_empty());
        assert!(analysis.null_result);
    }

    #[test]
    fn tail_share_classifies_minority_vs_systemic() {
        // 1 under-served of 4 (25%) is a genuine tail; 3 of 4 (75%) is systemic.
        let minority = vec![
            entry_with("low1", Some(shield_corpus::Scale::Regional), 1.0),
            entry_with("hi1", Some(shield_corpus::Scale::Regional), 9.0),
            entry_with("hi2", Some(shield_corpus::Scale::Regional), 9.0),
            entry_with("hi3", Some(shield_corpus::Scale::Regional), 9.0),
        ];
        let rubric = shield_score::Rubric::v0();
        let a = find_gaps(
            &minority,
            &rubric,
            shield_corpus::Scale::Regional,
            &[],
            false,
        );
        let tail = a.tail_regions.first().expect("tail present");
        assert!((tail.share_below_threshold - 0.25).abs() < 1e-9);
        assert!(!tail.systemic);

        let majority = vec![
            entry_with("low1", Some(shield_corpus::Scale::Regional), 1.0),
            entry_with("low2", Some(shield_corpus::Scale::Regional), 1.0),
            entry_with("low3", Some(shield_corpus::Scale::Regional), 1.0),
            entry_with("hi1", Some(shield_corpus::Scale::Regional), 9.0),
        ];
        let b = find_gaps(
            &majority,
            &rubric,
            shield_corpus::Scale::Regional,
            &[],
            false,
        );
        let tail = b.tail_regions.first().expect("tail present");
        assert!((tail.share_below_threshold - 0.75).abs() < 1e-9);
        assert!(tail.systemic);
    }

    #[test]
    fn other_scale_entries_respect_cross_scale_flag() {
        let rubric = shield_score::Rubric::v0();
        let corpus = vec![entry_with("B", Some(shield_corpus::Scale::Regional), 2.0)];

        let excluded = find_gaps(&corpus, &rubric, shield_corpus::Scale::National, &[], false);
        assert!(excluded.regions.is_empty());
        assert!(excluded.null_result);

        let included = find_gaps(&corpus, &rubric, shield_corpus::Scale::National, &[], true);
        assert!(!included.regions.is_empty());
        assert_eq!(
            included.regions.first().unwrap().member_ids,
            vec![String::from("B")]
        );
    }
}
