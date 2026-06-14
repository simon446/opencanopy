#!/usr/bin/env bash
# SPDX-License-Identifier: CERN-OHL-S-2.0
#
# Render the electronics design to PNGs for the docs site (docs/assets/renders/), headless:
#   * per-subsystem schematics       -> e-sch-<subsystem>.png   (8 functional blocks)
#   * full controller board, 3D      -> e-board-3d.png
#   * full controller board, top/bot -> e-board-top.png / e-board-bottom.png
#
# Toolchain (installed on demand, kept out of the repo): @tscircuit/cli, bun, @resvg/resvg-js.
# Requires node+npm and network. Run from anywhere:  ./render.sh
set -euo pipefail
cd "$(dirname "$0")"
REPO=$(cd ../../.. && pwd)
RENDERS="$REPO/docs/assets/renders"
WORK="${TSCI_WORK:-$(mktemp -d)}"
echo "==> toolchain: $WORK   renders -> $RENDERS"

# 1. Toolchain
if [ ! -x "$WORK/node_modules/.bin/tsci" ]; then
  ( cd "$WORK" && npm init -y >/dev/null 2>&1 \
      && npm install @tscircuit/cli @resvg/resvg-js >/dev/null )
fi
command -v bun >/dev/null 2>&1 || npm install -g bun >/dev/null
TSCI="$WORK/node_modules/.bin/tsci"
cat > "$WORK/svg2png.mjs" <<'JS'
import { Resvg } from '@resvg/resvg-js'
import { readFileSync, writeFileSync } from 'fs'
const [,, inp, outp, width] = process.argv
const r = new Resvg(readFileSync(inp,'utf8'), { fitTo:{mode:'width',value:parseInt(width||'1100')}, background:'white' })
writeFileSync(outp, r.render().asPng()); console.log('png', outp)
JS

# 2. (Re)generate the board + subsystem blocks from the netlist
python3 gen_tscircuit.py
python3 gen_tscircuit.py --subsystems
mkdir -p "$RENDERS"

# 3. Per-subsystem schematics -> PNG
for f in blocks/*.circuit.tsx; do
  name=$(basename "$f" .circuit.tsx)
  cp "$f" "$WORK/blk.circuit.tsx"
  ( cd "$WORK" && "$TSCI" export -f schematic-svg -o "$name.svg" blk.circuit.tsx >/dev/null 2>&1 \
      && node svg2png.mjs "$name.svg" "$name.png" 1200 >/dev/null )
  cp "$WORK/$name.png" "$RENDERS/e-sch-$name.png"
  echo "  e-sch-$name.png"
done

# 4. Full board: 3D + top + bottom
cp controller.circuit.tsx "$WORK/board.circuit.tsx"
( cd "$WORK"
  "$TSCI" snapshot board.circuit.tsx --3d --camera-preset top-right-corner -u >/dev/null 2>&1
  "$TSCI" build board.circuit.tsx --pcb-png >/dev/null 2>&1
  "$TSCI" export -f pcb-svg -o board.pcb.svg board.circuit.tsx >/dev/null 2>&1 || true )
cp "$WORK/__snapshots__/board.circuit-3d.snap.png" "$RENDERS/e-board-3d.png"
cp "$WORK/dist/board/pcb.png" "$RENDERS/e-board-top.png"
echo "  e-board-3d.png  e-board-top.png"
echo "==> done"
