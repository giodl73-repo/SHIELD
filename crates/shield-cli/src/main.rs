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
    /// Reproduce the same-vintage HRSA component-geography bridge.
    HrsaGeographyBaseline,
    /// Emit the non-authoritative HRSA geography HLT pack.
    HrsaGeographyHeldPack,
    /// Reproduce HRSA designation-recorded primary-care capacity formulas.
    HrsaCapacityBaseline,
    /// Emit the non-authoritative HRSA capacity-formula HLT pack.
    HrsaCapacityHeldPack,
    /// Reproduce CMS hospital available-bed use from annual cost reports.
    CmsOperationalCapacityBaseline,
    /// Emit the non-authoritative CMS operational-capacity HLT pack.
    CmsOperationalCapacityHeldPack,
    /// Reproduce CMS-certified service modes and employed workforce fields.
    CmsCertifiedServicesWorkforceBaseline,
    /// Emit the non-authoritative CMS services/workforce HLT pack.
    CmsCertifiedServicesWorkforceHeldPack,
    /// Reproduce CMS emergency-care process measures and coverage.
    CmsEmergencyCareTimelinessBaseline,
    /// Emit the non-authoritative CMS emergency-care HLT pack.
    CmsEmergencyCareTimelinessHeldPack,
    /// Reproduce county Original Medicare emergency-use demand context.
    CmsCountyEmergencyDemandBaseline,
    /// Emit the non-authoritative county emergency-demand HLT pack.
    CmsCountyEmergencyDemandHeldPack,
    /// Reproduce observed CMS inpatient origin-ZIP to hospital flow.
    CmsInpatientOriginDestinationBaseline,
    /// Emit the non-authoritative inpatient origin-destination HLT pack.
    CmsInpatientOriginDestinationHeldPack,
    /// Reproduce published national NEMSIS EMS destination observations.
    NemsisEmsDestinationBaseline,
    /// Emit the non-authoritative national EMS destination HLT pack.
    NemsisEmsDestinationHeldPack,
    /// Reproduce Minnesota's published stroke-system drive-time coverage.
    MinnesotaStrokeDriveTimeBaseline,
    /// Emit the non-authoritative Minnesota stroke-access HLT pack.
    MinnesotaStrokeDriveTimeHeldPack,
    /// Reproduce calendar 2025 NYC EMS response-to-scene observations.
    NycEmsResponseTimeBaseline,
    /// Emit the non-authoritative NYC EMS response-time HLT pack.
    NycEmsResponseTimeHeldPack,
    /// Reproduce NYC severity-one response tails and unmatched target context.
    NycEmsResponseDistributionTargetBaseline,
    /// Emit the non-authoritative NYC response-distribution HLT pack.
    NycEmsResponseDistributionTargetHeldPack,
    /// Reproduce the official NYC Local Law 119 Category 9 publication.
    #[command(name = "nyc-ems-local-law-119-category9-baseline")]
    NycEmsLocalLaw119Category9Baseline,
    /// Emit the non-authoritative Category 9 HLT pack.
    #[command(name = "nyc-ems-local-law-119-category9-held-pack")]
    NycEmsLocalLaw119Category9HeldPack,
    /// Reproduce the enacted Local Law 119 reporting-scope audit.
    #[command(name = "nyc-ems-local-law-119-reporting-scope-baseline")]
    NycEmsLocalLaw119ReportingScopeBaseline,
    /// Emit the non-authoritative reporting-scope correction HLT pack.
    #[command(name = "nyc-ems-local-law-119-reporting-scope-held-pack")]
    NycEmsLocalLaw119ReportingScopeHeldPack,
    /// Reproduce the bounded Category 9 borough-month operations context.
    NycEmsCategory9OperationsContextBaseline,
    /// Emit the non-authoritative Category 9 operations-context HLT pack.
    NycEmsCategory9OperationsContextHeldPack,
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
        Commands::HrsaGeographyBaseline => {
            println!("{}", shield_cms_access::hrsa_geography_baseline_json()?)
        }
        Commands::HrsaGeographyHeldPack => {
            println!("{}", shield_cms_access::hrsa_geography_held_pack_json()?)
        }
        Commands::HrsaCapacityBaseline => {
            println!("{}", shield_cms_access::hrsa_capacity_baseline_json()?)
        }
        Commands::HrsaCapacityHeldPack => {
            println!("{}", shield_cms_access::hrsa_capacity_held_pack_json()?)
        }
        Commands::CmsOperationalCapacityBaseline => println!(
            "{}",
            shield_cms_access::cms_operational_capacity_baseline_json()?
        ),
        Commands::CmsOperationalCapacityHeldPack => println!(
            "{}",
            shield_cms_access::cms_operational_capacity_held_pack_json()?
        ),
        Commands::CmsCertifiedServicesWorkforceBaseline => println!(
            "{}",
            shield_cms_access::cms_certified_services_workforce_baseline_json()?
        ),
        Commands::CmsCertifiedServicesWorkforceHeldPack => println!(
            "{}",
            shield_cms_access::cms_certified_services_workforce_held_pack_json()?
        ),
        Commands::CmsEmergencyCareTimelinessBaseline => println!(
            "{}",
            shield_cms_access::cms_emergency_care_timeliness_baseline_json()?
        ),
        Commands::CmsEmergencyCareTimelinessHeldPack => println!(
            "{}",
            shield_cms_access::cms_emergency_care_timeliness_held_pack_json()?
        ),
        Commands::CmsCountyEmergencyDemandBaseline => println!(
            "{}",
            shield_cms_access::cms_county_emergency_demand_baseline_json()?
        ),
        Commands::CmsCountyEmergencyDemandHeldPack => println!(
            "{}",
            shield_cms_access::cms_county_emergency_demand_held_pack_json()?
        ),
        Commands::CmsInpatientOriginDestinationBaseline => println!(
            "{}",
            shield_cms_access::cms_inpatient_origin_destination_baseline_json()?
        ),
        Commands::CmsInpatientOriginDestinationHeldPack => println!(
            "{}",
            shield_cms_access::cms_inpatient_origin_destination_held_pack_json()?
        ),
        Commands::NemsisEmsDestinationBaseline => println!(
            "{}",
            shield_cms_access::nemsis_ems_destination_baseline_json()?
        ),
        Commands::NemsisEmsDestinationHeldPack => println!(
            "{}",
            shield_cms_access::nemsis_ems_destination_held_pack_json()?
        ),
        Commands::MinnesotaStrokeDriveTimeBaseline => println!(
            "{}",
            shield_cms_access::minnesota_stroke_drive_time_baseline_json()?
        ),
        Commands::MinnesotaStrokeDriveTimeHeldPack => println!(
            "{}",
            shield_cms_access::minnesota_stroke_drive_time_held_pack_json()?
        ),
        Commands::NycEmsResponseTimeBaseline => println!(
            "{}",
            shield_cms_access::nyc_ems_response_time_baseline_json()?
        ),
        Commands::NycEmsResponseTimeHeldPack => println!(
            "{}",
            shield_cms_access::nyc_ems_response_time_held_pack_json()?
        ),
        Commands::NycEmsResponseDistributionTargetBaseline => println!(
            "{}",
            shield_cms_access::nyc_ems_response_distribution_target_baseline_json()?
        ),
        Commands::NycEmsResponseDistributionTargetHeldPack => println!(
            "{}",
            shield_cms_access::nyc_ems_response_distribution_target_held_pack_json()?
        ),
        Commands::NycEmsLocalLaw119Category9Baseline => println!(
            "{}",
            shield_cms_access::nyc_ems_local_law_119_category9_baseline_json()?
        ),
        Commands::NycEmsLocalLaw119Category9HeldPack => println!(
            "{}",
            shield_cms_access::nyc_ems_local_law_119_category9_held_pack_json()?
        ),
        Commands::NycEmsLocalLaw119ReportingScopeBaseline => println!(
            "{}",
            shield_cms_access::nyc_ems_local_law_119_reporting_scope_baseline_json()?
        ),
        Commands::NycEmsLocalLaw119ReportingScopeHeldPack => println!(
            "{}",
            shield_cms_access::nyc_ems_local_law_119_reporting_scope_held_pack_json()?
        ),
        Commands::NycEmsCategory9OperationsContextBaseline => println!(
            "{}",
            shield_cms_access::nyc_ems_category9_operations_context_baseline_json()?
        ),
        Commands::NycEmsCategory9OperationsContextHeldPack => println!(
            "{}",
            shield_cms_access::nyc_ems_category9_operations_context_held_pack_json()?
        ),
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
        assert!(Cli::try_parse_from(["shield", "hrsa-geography-baseline"]).is_ok());
        assert!(Cli::try_parse_from(["shield", "hrsa-geography-held-pack"]).is_ok());
        assert!(Cli::try_parse_from(["shield", "hrsa-capacity-baseline"]).is_ok());
        assert!(Cli::try_parse_from(["shield", "hrsa-capacity-held-pack"]).is_ok());
        assert!(Cli::try_parse_from(["shield", "cms-operational-capacity-baseline"]).is_ok());
        assert!(Cli::try_parse_from(["shield", "cms-operational-capacity-held-pack"]).is_ok());
        assert!(
            Cli::try_parse_from(["shield", "cms-certified-services-workforce-baseline"]).is_ok()
        );
        assert!(
            Cli::try_parse_from(["shield", "cms-certified-services-workforce-held-pack"]).is_ok()
        );
        assert!(Cli::try_parse_from(["shield", "cms-emergency-care-timeliness-baseline"]).is_ok());
        assert!(Cli::try_parse_from(["shield", "cms-emergency-care-timeliness-held-pack"]).is_ok());
        assert!(Cli::try_parse_from(["shield", "cms-county-emergency-demand-baseline"]).is_ok());
        assert!(Cli::try_parse_from(["shield", "cms-county-emergency-demand-held-pack"]).is_ok());
        assert!(
            Cli::try_parse_from(["shield", "cms-inpatient-origin-destination-baseline"]).is_ok()
        );
        assert!(
            Cli::try_parse_from(["shield", "cms-inpatient-origin-destination-held-pack"]).is_ok()
        );
        assert!(Cli::try_parse_from(["shield", "nemsis-ems-destination-baseline"]).is_ok());
        assert!(Cli::try_parse_from(["shield", "nemsis-ems-destination-held-pack"]).is_ok());
        assert!(Cli::try_parse_from(["shield", "minnesota-stroke-drive-time-baseline"]).is_ok());
        assert!(Cli::try_parse_from(["shield", "minnesota-stroke-drive-time-held-pack"]).is_ok());
        assert!(Cli::try_parse_from(["shield", "nyc-ems-response-time-baseline"]).is_ok());
        assert!(Cli::try_parse_from(["shield", "nyc-ems-response-time-held-pack"]).is_ok());
        assert!(
            Cli::try_parse_from(["shield", "nyc-ems-response-distribution-target-baseline"])
                .is_ok()
        );
        assert!(
            Cli::try_parse_from(["shield", "nyc-ems-response-distribution-target-held-pack"])
                .is_ok()
        );
    }
}
