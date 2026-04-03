# Deferred Ideas

## Slice & Dice Textmod Builder App

**Status**: Deferred — build after Sliceymon+ is complete and balanced

A web app that lets anyone create their own Slice & Dice textmod without needing to understand the raw text format.

**Features**:
- Image upload for heroes, items, monsters, bosses (auto-encodes sprites)
- Visual dice designer — pick face IDs, set pips, see a preview of each die
- Full character preview — see all tiers, evolution paths, spells
- Hero balance dashboard — shows pip budgets, blank counts, HP curves per tier
- Drag-and-drop roster management — assign Pokemon to colors, set P1/P2
- Capture pool editor with ball type selection
- Monster pool editor with floor assignments
- Boss designer
- One-click export to pasteable textmod
- Import existing textmod to edit

**Foundation**: The `build_mod.js` compiler we're building now becomes the backend. The app is a GUI over the registry JSON + component files.

**When**: After Sliceymon+ is shipped, tested, and balanced. The mod-building experience will inform what the app needs.
