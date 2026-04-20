use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use textmod_compiler::error::CompilerError;

#[derive(Parser)]
#[command(name = "textmod-compiler")]
#[command(about = "Slice & Dice textmod compiler — extract, build, and cross-check textmods")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Extract a textmod into IR (directory of structured files)
    Extract {
        /// Path to the textmod file
        input: PathBuf,
        /// Output directory for the IR
        #[arg(short, long)]
        output: PathBuf,
    },
    /// Build a textmod from an IR directory
    Build {
        /// Path to the IR directory
        input: PathBuf,
        /// Output file path for the built textmod
        #[arg(short, long)]
        output: PathBuf,
        /// Optional overlay IR directory to merge
        #[arg(long)]
        overlay: Option<PathBuf>,
    },
    /// Check a textmod: re-extract (structural) + cross-reference rules, optionally round-trip
    Check {
        /// Path to the textmod file
        input: PathBuf,
        /// Also run round-trip IR comparison (extract -> build -> extract)
        #[arg(long)]
        round_trip: bool,
    },
    /// Generate JSON Schema for the IR types
    Schema {
        /// Output file path (prints to stdout if omitted)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Overlay an IR (JSON) onto a base textmod
    Overlay {
        /// Path to the base textmod file
        base: PathBuf,
        /// Path to the overlay IR JSON file
        #[arg(long = "with")]
        with_ir: PathBuf,
        /// Output file path
        #[arg(short, long)]
        output: PathBuf,
    },
}

fn main() -> Result<(), CompilerError> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Extract { input, output } => {
            let textmod = fs::read_to_string(&input)?;
            let ir = textmod_compiler::extract(&textmod)?;
            let json = textmod_compiler::ir_to_json(&ir)
?;
            fs::create_dir_all(&output)?;
            fs::write(output.join("registry.json"), json)?;
            println!("Extracted {} heroes, {} replica_items, {} monsters, {} bosses, {} structural",
                ir.heroes.len(), ir.replica_items.len(), ir.monsters.len(),
                ir.bosses.len(), ir.structural.len());
        }
        Commands::Build { input, output, overlay } => {
            let registry_path = input.join("registry.json");
            let json = fs::read_to_string(&registry_path)?;
            let mut ir = textmod_compiler::ir_from_json(&json)
?;

            if let Some(overlay_dir) = overlay {
                let overlay_json = fs::read_to_string(overlay_dir.join("registry.json"))?;
                let overlay_ir = textmod_compiler::ir_from_json(&overlay_json)
                    .map_err(|e| CompilerError::BuildError {
                        component: "json".to_string(),
                        message: e.to_string(),
                    })?;
                ir = textmod_compiler::merge(ir, overlay_ir)?;
            }

            let sprites: HashMap<String, String> = HashMap::new();
            let textmod = textmod_compiler::build(&ir, &sprites)?;
            fs::write(&output, textmod)?;
            println!("Built textmod written to {}", output.display());
        }
        Commands::Check { input, round_trip } => {
            let textmod = fs::read_to_string(&input)?;

            // Structural check: extraction succeeds = structurally valid
            let ir = textmod_compiler::extract(&textmod)?;
            println!("Structural check: OK (extraction succeeded)");

            // Cross-reference validation (operates on structured IR)
            let xref_report = textmod_compiler::check_references(&ir);
            print!("{}", xref_report);

            if !xref_report.is_ok() {
                return Err(CompilerError::ValidationError {
                    message: format!(
                        "{} cross-reference errors, {} warnings",
                        xref_report.errors.len(),
                        xref_report.warnings.len()
                    ),
                });
            }

            // Optional round-trip IR comparison
            if round_trip {
                let ir1 = textmod_compiler::extract(&textmod)?;
                let sprites: HashMap<String, String> = HashMap::new();
                let rebuilt = textmod_compiler::build(&ir1, &sprites)?;
                let ir2 = textmod_compiler::extract(&rebuilt)?;

                let json1 = textmod_compiler::ir_to_json(&ir1)?;
                let json2 = textmod_compiler::ir_to_json(&ir2)?;

                if json1 == json2 {
                    println!("Round-trip passed: identical IR");
                } else {
                    println!("Round-trip FAILED: IRs differ");
                    return Err(CompilerError::ValidationError {
                        message: "Round-trip IR mismatch".to_string(),
                    });
                }
            }
        }
        Commands::Schema { output } => {
            let schema = schemars::schema_for!(textmod_compiler::ir::ModIR);
            let json = serde_json::to_string_pretty(&schema)
                .map_err(|e| CompilerError::BuildError {
                    component: "schema".to_string(),
                    message: e.to_string(),
                })?;
            if let Some(path) = output {
                fs::write(&path, &json)?;
                println!("Schema written to {}", path.display());
            } else {
                println!("{}", json);
            }
        }
        Commands::Overlay { base, with_ir, output } => {
            let base_text = fs::read_to_string(&base)?;
            let base_ir = textmod_compiler::extract(&base_text)?;

            let overlay_json = fs::read_to_string(&with_ir)?;
            let overlay_ir = textmod_compiler::ir_from_json(&overlay_json)
                .map_err(|e| CompilerError::BuildError {
                    component: "json".to_string(),
                    message: e.to_string(),
                })?;

            let merged = textmod_compiler::merge(base_ir, overlay_ir)?;
            let sprites: HashMap<String, String> = HashMap::new();
            let textmod = textmod_compiler::build(&merged, &sprites)?;
            fs::write(&output, &textmod)?;
            println!("Overlay applied: {} heroes, {} replica items, {} monsters, {} bosses",
                merged.heroes.len(), merged.replica_items.len(),
                merged.monsters.len(), merged.bosses.len());
            println!("Output written to {}", output.display());
        }
    }

    Ok(())
}
