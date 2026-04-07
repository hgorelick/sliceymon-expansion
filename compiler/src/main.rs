use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use textmod_compiler::error::CompilerError;
use textmod_compiler::util;

#[derive(Parser)]
#[command(name = "textmod-compiler")]
#[command(about = "Slice & Dice textmod compiler — extract, build, and validate textmods")]
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
    /// Validate a textmod against structural rules
    Validate {
        /// Path to the textmod file
        input: PathBuf,
        /// Also run round-trip IR comparison (extract -> build -> extract)
        #[arg(long)]
        round_trip: bool,
    },
    /// Patch a base textmod with replacement hero files
    Patch {
        /// Path to the base textmod file
        base: PathBuf,
        /// Directory containing hero .txt files to patch in
        #[arg(short = 'H', long)]
        heroes: PathBuf,
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
            println!("Extracted {} heroes, {} captures, {} monsters, {} bosses, {} structural",
                ir.heroes.len(), ir.captures.len(), ir.monsters.len(),
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
        Commands::Validate { input, round_trip } => {
            let textmod = fs::read_to_string(&input)?;

            // Structural validation
            let report = textmod_compiler::validate(&textmod)?;
            print!("{}", report);

            if !report.is_ok() {
                return Err(CompilerError::ValidationError {
                    message: format!(
                        "{} errors, {} warnings",
                        report.errors.len(),
                        report.warnings.len()
                    ),
                });
            }

            // Cross-reference validation (operates on structured IR)
            if let Ok(ir) = textmod_compiler::extract(&textmod) {
                let xref_report = textmod_compiler::validate_cross_references(&ir);
                if !xref_report.errors.is_empty() {
                    println!("Cross-reference validation: {} errors", xref_report.errors.len());
                    for finding in &xref_report.errors {
                        println!("  ERROR [{}]: {}", finding.rule_id, finding.message);
                    }
                } else {
                    println!("Cross-reference validation: OK");
                }
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
        Commands::Patch { base, heroes, output } => {
            let base_text = fs::read_to_string(&base)?;
            let mut hero_patches: Vec<(String, String)> = Vec::new();
            let mut new_heroes: Vec<String> = Vec::new();

            let entries = fs::read_dir(&heroes)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) != Some("txt") {
                    continue;
                }
                let content = fs::read_to_string(&path)?.trim().to_string();
                if content.is_empty() {
                    continue;
                }
                let filename = path.file_stem().unwrap().to_string_lossy().to_string();
                let mn_name = util::extract_mn_name(&content)
                    .or_else(|| util::extract_last_n_name(&content))
                    .unwrap_or_else(|| "unknown".to_string());

                if filename.starts_with("line_new_") {
                    new_heroes.push(content);
                    println!("  + NEW: {} ({})", mn_name, filename);
                } else {
                    hero_patches.push((mn_name.clone(), content));
                    println!("  ~ REPLACE: {} ({})", mn_name, filename);
                }
            }

            // Apply patches directly on modifier list
            let result = patch_textmod(&base_text, &hero_patches, &new_heroes)?;
            fs::write(&output, &result)?;
            println!("\nPatched textmod written to {} ({} bytes)",
                output.display(), result.len());
        }
    }

    Ok(())
}

/// Patch a base textmod by replacing/adding hero modifiers.
/// This is CLI-level logic, not part of the core library.
fn patch_textmod(
    base_text: &str,
    hero_patches: &[(String, String)],
    new_heroes: &[String],
) -> Result<String, CompilerError> {
    let mut modifiers = textmod_compiler::extractor::splitter::split_modifiers(base_text)?;
    let mut replaced = 0;
    let mut not_found: Vec<String> = Vec::new();
    let mut color_replace_count: std::collections::HashMap<char, usize> = std::collections::HashMap::new();

    for (mn_name, new_content) in hero_patches {
        let new_color = util::extract_color(new_content);
        let found = if let Some(color) = new_color {
            let count = color_replace_count.entry(color).or_insert(0);
            let target_n = *count;
            let mut color_matches = 0usize;
            let mut found_pos = None;
            for (i, m) in modifiers.iter().enumerate() {
                let lower = m.to_lowercase();
                if lower.contains("heropool") && lower.contains("replica.") && util::has_color(m, color) {
                    if color_matches == target_n {
                        found_pos = Some(i);
                        break;
                    }
                    color_matches += 1;
                }
            }
            if found_pos.is_some() { *count += 1; }
            found_pos
        } else {
            let search = format!(".mn.{}@", mn_name);
            modifiers.iter().position(|m| m.contains(&search))
        };

        if let Some(pos) = found {
            modifiers[pos] = new_content.clone();
            replaced += 1;
        } else {
            not_found.push(mn_name.clone());
        }
    }

    if !new_heroes.is_empty() {
        let last_hero_pos = modifiers.iter().rposition(|m| {
            let lower = m.to_lowercase();
            lower.contains("heropool") && lower.contains("replica.")
        }).unwrap_or(modifiers.len() - 1);
        for (i, hero) in new_heroes.iter().enumerate() {
            modifiers.insert(last_hero_pos + 1 + i, hero.clone());
        }
    }

    if !not_found.is_empty() {
        eprintln!("Warning: {} heroes not found: {:?}", not_found.len(), not_found);
    }
    eprintln!("Patch: {} replaced, {} new, {} not found", replaced, new_heroes.len(), not_found.len());

    let output = modifiers.join(",\n\n");
    Ok(if output.is_empty() { output } else { format!("{},", output) })
}
