#!/bin/bash
# Convert generated hero .txt files to IR JSON using the compiler.
#
# Each .txt file contains a raw hero modifier string. We wrap it as a
# single-modifier textmod, extract to IR, strip the raw field, and save
# as a clean JSON file.
#
# Usage: ./tools/convert_heroes_to_ir.sh [output_dir]
#   output_dir defaults to heroes_ir/

set -euo pipefail

COMPILER="./compiler/target/release/textmod-compiler"
GENERATED_DIR="./generated"
OUTPUT_DIR="${1:-./heroes_ir}"
TMP_DIR=$(mktemp -d)

# Ensure compiler is built
if [ ! -f "$COMPILER" ]; then
    echo "Building compiler..."
    ~/.cargo/bin/cargo build --release --manifest-path compiler/Cargo.toml 2>&1 | tail -1
fi

mkdir -p "$OUTPUT_DIR"

count=0
errors=0

for txt_file in "$GENERATED_DIR"/*.txt; do
    filename=$(basename "$txt_file" .txt)

    # Skip the original backup
    if [[ "$filename" == *"_original"* ]]; then
        echo "SKIP: $filename (backup file)"
        continue
    fi

    # The .txt file contains a raw modifier string. The compiler's splitter
    # expects comma-separated modifiers, so we ensure trailing comma.
    content=$(cat "$txt_file")
    # Trim whitespace and ensure trailing comma
    content=$(echo "$content" | sed 's/[[:space:]]*$//')
    if [[ "$content" != *"," ]]; then
        content="${content},"
    fi

    # Write as a minimal textmod
    echo "$content" > "$TMP_DIR/${filename}.textmod"

    # Extract to IR
    ir_tmp="$TMP_DIR/${filename}_ir"
    if ! "$COMPILER" extract "$TMP_DIR/${filename}.textmod" --output "$ir_tmp" 2>/dev/null; then
        echo "ERROR: Failed to extract $filename"
        errors=$((errors + 1))
        continue
    fi

    # Extract just the hero(es) from the IR, strip raw field and original_modifiers
    python3 -c "
import json, sys

with open('${ir_tmp}/registry.json') as f:
    ir = json.load(f)

heroes = ir.get('heroes', [])
if not heroes:
    print('WARNING: No heroes found in $filename', file=sys.stderr)
    sys.exit(1)

# Process each hero: remove raw field so the builder uses field-based emission
for hero in heroes:
    hero.pop('raw', None)
    # Also strip sprite_name to just the Pokemon name (not the full encoded data)
    # The sprite data comes from sprites.json at build time

output = heroes[0] if len(heroes) == 1 else heroes
print(json.dumps(output, indent=2))
" > "$OUTPUT_DIR/${filename}.json" 2>/dev/null

    if [ $? -eq 0 ] && [ -s "$OUTPUT_DIR/${filename}.json" ]; then
        echo "  OK: $filename -> $OUTPUT_DIR/${filename}.json"
        count=$((count + 1))
    else
        echo "ERROR: Failed to process $filename"
        errors=$((errors + 1))
    fi
done

rm -rf "$TMP_DIR"

echo ""
echo "Converted $count heroes, $errors errors"
echo "Output directory: $OUTPUT_DIR"
