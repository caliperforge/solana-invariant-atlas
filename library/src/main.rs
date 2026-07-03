//! atlas-driver: suggest / emit / scorecard over the pinned
//! cf-invariants-anchor crates. Deliberately clap-free so the same
//! sources also compile under `cargo build-sbf` (the CI load-bearing
//! toolchain check; the binary itself is host-side tooling).

use std::env;
use std::fs;
use std::process::ExitCode;

use atlas_driver::{parse_target, render, scorecard_markdown, suggest};
use cf_invariants_anchor_core::{ContractSurface, Scorecard};

const USAGE: &str = "\
usage:
  atlas-driver suggest <surface.json>
      print ranked invariant candidates (JSON) for a parsed surface
  atlas-driver emit <surface.json> <candidate-index> <crucible|trident>
      print the rendered fixture source for one candidate
  atlas-driver scorecard <scorecard.json>
      print the markdown rendering of a scorecard
";

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    match run(&args) {
        Ok(out) => {
            println!("{out}");
            ExitCode::SUCCESS
        }
        Err(msg) => {
            eprintln!("atlas-driver: {msg}\n{USAGE}");
            ExitCode::FAILURE
        }
    }
}

fn run(args: &[String]) -> Result<String, String> {
    let cmd = args.first().ok_or("missing subcommand")?;
    match cmd.as_str() {
        "suggest" => {
            let surface = read_surface(args.get(1).ok_or("missing <surface.json>")?)?;
            serde_json::to_string_pretty(&suggest(&surface))
                .map_err(|e| format!("serialize candidates: {e}"))
        }
        "emit" => {
            let surface = read_surface(args.get(1).ok_or("missing <surface.json>")?)?;
            let index: usize = args
                .get(2)
                .ok_or("missing <candidate-index>")?
                .parse()
                .map_err(|e| format!("candidate-index: {e}"))?;
            let target = args
                .get(3)
                .and_then(|t| parse_target(t))
                .ok_or("target must be crucible or trident")?;
            let candidates = suggest(&surface);
            let candidate = candidates
                .get(index)
                .ok_or_else(|| format!("candidate-index {index} out of range ({} candidates)", candidates.len()))?;
            Ok(render(&surface, candidate, target))
        }
        "scorecard" => {
            let path = args.get(1).ok_or("missing <scorecard.json>")?;
            let raw = fs::read_to_string(path).map_err(|e| format!("read {path}: {e}"))?;
            let scorecard: Scorecard =
                serde_json::from_str(&raw).map_err(|e| format!("parse {path}: {e}"))?;
            Ok(scorecard_markdown(&scorecard))
        }
        other => Err(format!("unknown subcommand: {other}")),
    }
}

fn read_surface(path: &str) -> Result<ContractSurface, String> {
    let raw = fs::read_to_string(path).map_err(|e| format!("read {path}: {e}"))?;
    serde_json::from_str(&raw).map_err(|e| format!("parse {path}: {e}"))
}
