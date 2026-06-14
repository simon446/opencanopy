#!/usr/bin/env bash
# SPDX-License-Identifier: CERN-OHL-S-2.0
#
# Headless, code-only PCB build: controller netlist -> tscircuit -> autoroute -> Gerbers/PnP/BOM.
# No KiCad GUI. Requires: node + npm, bun, and network (first run installs the toolchain).
#
#   ./build.sh            # generate .tsx, build, export Gerbers + PNG + KiCad PCB into out/
#
# Outputs land in out/. They are a machine-routed DRAFT (placeholder IC/connector footprints,
# autorouter-grade placement) — see README.md before trusting them.
set -euo pipefail
cd "$(dirname "$0")"

WORK="${TSCI_WORK:-$(mktemp -d)}"          # toolchain install dir (kept out of the repo)
echo "==> toolchain dir: $WORK"

# 1. Toolchain (idempotent): tscircuit CLI + the bun runtime it needs.
if [ ! -x "$WORK/node_modules/.bin/tsci" ]; then
  ( cd "$WORK" && npm init -y >/dev/null 2>&1 && npm install @tscircuit/cli >/dev/null )
fi
command -v bun >/dev/null 2>&1 || npm install -g bun >/dev/null
TSCI="$WORK/node_modules/.bin/tsci"

# 2. Regenerate the board from the netlist (source of truth).
python3 gen_tscircuit.py
cp controller.circuit.tsx "$WORK/board.circuit.tsx"

# 3. Build (eval + autoroute) and export the fab package.
( cd "$WORK"
  "$TSCI" build  board.circuit.tsx                                   # validates netlist + routes
  "$TSCI" export -f gerbers   -o controller.gerbers.zip board.circuit.tsx
  "$TSCI" export -f kicad_pcb -o controller.kicad_pcb  board.circuit.tsx
  "$TSCI" build  board.circuit.tsx --pcb-png )

mkdir -p out
cp "$WORK/controller.gerbers.zip" "$WORK/controller.kicad_pcb" out/
cp "$WORK/dist/board/pcb.png" out/controller.pcb.png
echo "==> wrote out/controller.gerbers.zip, out/controller.kicad_pcb, out/controller.pcb.png"
