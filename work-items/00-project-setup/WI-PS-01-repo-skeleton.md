# WI-PS-01 — Repository skeleton

| Field | Value |
|---|---|
| Track | Project & Repo |
| Milestone | M0-01 |
| Depends on | — |
| Spec refs | §14.1 |
| Status | Not started |

## Objective

Stand up the public repository structure so every other track has a home for its artifacts.

## Deliverables

- [ ] Top-level folders created exactly as in spec §14.1: `docs/`, `firmware/`, `electronics/`,
      `mechanical/`, `validation/`, `scripts/`, `.github/`, `LICENSES/`.
- [ ] Placeholder `README.md` in each major subtree describing its purpose.
- [ ] `.gitignore` tuned for KiCad, PlatformIO/CMake, Python, and CAD scratch files.
- [ ] Root `README.md` stub (full content owned by [WI-DOC-01](../06-documentation/WI-DOC-01-readme.md)).

## Acceptance criteria

- Directory tree matches spec §14.1 one-for-one.
- A new contributor can clone and immediately see where their discipline's files belong.

## Notes

Do not commit large binary CAD/Gerber blobs yet — just the structure and `.gitkeep`/README stubs.
