//! jycut — local CLI that builds verifiable, fully editable 剪映/CapCut drafts
//! from a JSON plan. No account, no upload, deterministic.

use anyhow::{bail, Result};
use clap::{Parser, Subcommand};
use jycut::{draft, plan, probe, store};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "jycut", version, about = None, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Environment preflight: draft roots, running editors, ffprobe
    Doctor,
    /// Measure a media file (duration/size/streams) via ffprobe
    Probe {
        /// Media file to measure
        media: PathBuf,
    },
    /// Build a draft directory from a jycut-plan/v1 JSON file
    Build {
        /// Plan file (jycut-plan/v1)
        plan: PathBuf,
        /// New output directory for the draft
        #[arg(long)]
        out: PathBuf,
    },
    /// Structural lint over a built or published draft
    Verify {
        /// Draft directory
        draft: PathBuf,
    },
    /// Summarize a draft (tracks/segments/canvas/platform)
    Inspect {
        /// Draft directory
        draft: PathBuf,
    },
    /// Copy a built draft into the draft store and register it
    Publish {
        /// Draft directory (as produced by build)
        draft: PathBuf,
        /// Draft store root (default: detect, or JYCUT_DRAFT_ROOT)
        #[arg(long)]
        root: Option<PathBuf>,
        /// Publish even if 剪映/CapCut is running
        #[arg(long)]
        force: bool,
    },
}

fn run(cmd: Command) -> Result<()> {
    match cmd {
        Command::Doctor => print_json(store::doctor()?),
        Command::Probe { media } => print_json(serde_json::to_value(probe::probe(&media)?)?),
        Command::Build { plan, out } => {
            let plan = plan::Plan::load(&plan)?;
            let plan_dir = plan.parent.clone().unwrap_or_else(|| PathBuf::from("."));
            let report = draft::build(&plan, &plan_dir, &out, &|p| probe::probe(p))?;
            let verdict = draft::verify(Path::new(&report.out_dir))?;
            if verdict["ok"] != serde_json::json!(true) {
                bail!(
                    "built draft failed self-verification: {}",
                    verdict["issues"]
                );
            }
            print_json(serde_json::to_value(report)?)
        }
        Command::Verify { draft } => print_json(draft::verify(&draft)?),
        Command::Inspect { draft } => print_json(draft::inspect(&draft)?),
        Command::Publish { draft, root, force } => {
            let root = store::resolve_root(root.as_deref())?;
            print_json(store::publish(&draft, &root, force)?)
        }
    }
    Ok(())
}

fn print_json(v: serde_json::Value) {
    println!("{}", serde_json::to_string_pretty(&v).unwrap_or_default());
}

use std::path::Path;

fn main() {
    if let Err(e) = run(Cli::parse().command) {
        eprintln!("error: {e:#}");
        std::process::exit(1);
    }
}
