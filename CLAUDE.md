# CLAUDE.md — guidance for agents working in this repo

## Git workflow

- **Pushing directly to `main` is fine.** You do not need to open a branch or PR for routine work in
  this repo (the maintainer prefers committing straight to `main`).
- **Always sync before you push** — others (humans or agents) may have pushed since you last fetched,
  so a plain `git push` can be rejected as non-fast-forward. Before pushing:
  1. `git fetch origin`
  2. Rebase your local commits onto the latest `main`: `git pull --rebase origin main`
     (or `git rebase origin/main`).
  3. If the rebase reports conflicts, resolve them, `git add` the files, `git rebase --continue`, then
     re-run the tests/selftests below before pushing.
  4. `git push origin main`.
- A merge (`git pull --no-rebase`) is acceptable if a rebase gets messy, but **prefer rebase** to keep
  `main` linear.
- End commit messages with the trailer:
  `Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>`
- **Don't delete remote branches** unless explicitly asked.

## Before committing

Run the relevant checks (plain Python, no extra deps):

```sh
python3 scripts/bom_check.py --selftest
python3 scripts/dli_calculator.py --selftest
python3 validation/ppfd-measurements/model/photometric_model.py --selftest
```

CI runs these on every push; keep them green.

## Repo orientation

- `plan/tabletop_pepper_grower_v1_spec_v1_1.md` — the V1 engineering spec (source of truth for
  requirements; research basis R1–R17 in §2.2).
- `plan/work-items/` — work decomposed per specialist track; each item lists deliverables as
  checkboxes and an inline `Status`. Check items off and update `Status` as you complete them.
- `docs/` — published plant/design docs (e.g. `plant-profile-hot-pepper.md` is the single source of
  truth for plant targets; `references.md` holds R1–R17).
- `scripts/` — calculators/tooling; each ships a `--selftest`.
- `validation/` — test plans, measurements, and models (incl. the pre-order photometric gate).

## Track ownership

Work within the track you were asked to own; don't edit other tracks' deliverables unless asked. The
plant-science track (`plan/work-items/01-plant-science/`, `docs/plant-profile-*`, `docs/dli-targets`,
`docs/watering-model`, `docs/vpd-climate-model`, `docs/nutrient-ph-ec-guidance`,
`validation/ppfd-measurements/model/`) is the single source of truth for plant targets.
</content>
