use clap::{Parser, Subcommand};
use shield_score::DimensionScorer;

#[derive(Parser)]
#[command(name = "shield")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Reproduce the bounded CMS national hospital-footprint baseline.
    CmsAccessBaseline,
    /// Emit the non-authoritative HLT evidence pack for Taxlane review.
    CmsAccessHeldPack,
    /// Reproduce the bounded CMS-USDA county rurality join.
    CmsRuralityBaseline,
    /// Emit the non-authoritative rurality HLT pack for Taxlane review.
    CmsRuralityHeldPack,
    /// Reproduce the bounded HRSA primary-care HPSA registry census.
    HrsaPrimaryCareBaseline,
    /// Emit the non-authoritative primary-care shortage HLT pack.
    HrsaPrimaryCareHeldPack,
    Corpus {
        path: std::path::PathBuf,
    },
    Score {
        path: std::path::PathBuf,
    },
    #[command(name = "tier-sla")]
    TierSla {
        path: std::path::PathBuf,
    },
    Gap {
        #[arg(long)]
        scale: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::CmsAccessBaseline => println!("{}", shield_cms_access::baseline_json()?),
        Commands::CmsAccessHeldPack => println!("{}", shield_cms_access::held_pack_json()?),
        Commands::CmsRuralityBaseline => {
            println!("{}", shield_cms_access::rurality_baseline_json()?)
        }
        Commands::CmsRuralityHeldPack => {
            println!("{}", shield_cms_access::rurality_held_pack_json()?)
        }
        Commands::HrsaPrimaryCareBaseline => {
            println!("{}", shield_cms_access::hrsa_primary_care_baseline_json()?)
        }
        Commands::HrsaPrimaryCareHeldPack => {
            println!("{}", shield_cms_access::hrsa_primary_care_held_pack_json()?)
        }
        Commands::Corpus { path } => {
            let text = std::fs::read_to_string(&path)?;
            let entry = shield_corpus::CorpusEntry::from_markdown(&text)?;
            println!("id: {}", entry.id);
            println!("validate: {:?}", entry.validate());
        }
        Commands::Score { path } => {
            let text = std::fs::read_to_string(&path)?;
            let entry = shield_corpus::CorpusEntry::from_markdown(&text)?;
            let scorer = shield_score::ProvisionalScorer::default();
            for dim in shield_score::Dimension::all() {
                let score = scorer.score(&entry, dim);
                println!("{}: {}", dim.code(), score.value());
            }
        }
        Commands::TierSla { path } => {
            let text = std::fs::read_to_string(&path)?;
            let entry = shield_corpus::CorpusEntry::from_markdown(&text)?;
            println!("tier: {:?}", shield_tier::classify(&entry));
            println!(
                "tier_sla_gap: {}",
                shield_tier::tier_sla_gap(&entry).is_some()
            );
        }
        Commands::Gap { scale } => {
            let scale_value = shield_corpus::Scale::parse(&scale)
                .ok_or_else(|| format!("invalid scale: {}", scale))?;
            let rubric = shield_score::Rubric::v0();
            let result = shield_gap::find_gaps(&[], &rubric, scale_value, &[], false);
            println!("null_result: {}", result.null_result);
            println!("regions: {}", result.regions.len());
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }

    #[test]
    fn parses_gap() {
        assert!(Cli::try_parse_from(["shield", "gap", "--scale", "national"]).is_ok());
    }

    #[test]
    fn parses_corpus() {
        assert!(Cli::try_parse_from(["shield", "corpus", "some.md"]).is_ok());
    }

    #[test]
    fn parses_cms_access_commands() {
        assert!(Cli::try_parse_from(["shield", "cms-access-baseline"]).is_ok());
        assert!(Cli::try_parse_from(["shield", "cms-access-held-pack"]).is_ok());
        assert!(Cli::try_parse_from(["shield", "cms-rurality-baseline"]).is_ok());
        assert!(Cli::try_parse_from(["shield", "cms-rurality-held-pack"]).is_ok());
        assert!(Cli::try_parse_from(["shield", "hrsa-primary-care-baseline"]).is_ok());
        assert!(Cli::try_parse_from(["shield", "hrsa-primary-care-held-pack"]).is_ok());
    }
}
