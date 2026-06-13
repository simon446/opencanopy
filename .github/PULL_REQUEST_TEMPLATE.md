<!--
  Thanks for contributing to OpenCanopy. PRs MUST cite a work-item ID (e.g. WI-EE-03) and copy in
  that work item's acceptance checklist. PRs without a work-item reference will not be reviewed.
  See CONTRIBUTING.md for branch naming, per-track review, and the test-before-merge rule.
-->

## Work item

<!-- Required. Link the work-item file this PR advances, e.g. WI-FW-04. -->
- Work item: `WI-__-__` — <title>
- Track: <!-- Plant Science / Firmware / Electronics / Mechanical / Validation & QA / Documentation / Project & Repo -->
- Spec refs: <!-- e.g. §9.6, §16.3 -->
- Closes: <!-- #issue, if any -->

## Summary

<!-- What does this PR change, and why? -->

## Work-item acceptance checklist

<!--
  Paste the "Deliverables" / "Acceptance criteria" checkboxes from the cited work-item file and
  tick the ones this PR satisfies. Check off the boxes in the work-item file itself in this PR.
-->
- [ ] <deliverable 1>
- [ ] <deliverable 2>

## Testing / verification

<!-- How was this verified? Tick all that were actually run, and paste/attach evidence. -->
- [ ] Host unit tests pass (`firmware/controller/tests`)
- [ ] Simulation scenarios pass (`firmware/sim`)
- [ ] HIL tests pass (`firmware/hil`) — if hardware-affecting
- [ ] Markdown lint + docs link/reference check pass
- [ ] BOM check passes (`scripts/bom_check.py`) — if BOM-affecting
- [ ] ERC/DRC clean — if PCB-affecting
- [ ] STL manifold / CAD presence check — if mechanical-affecting
- [ ] N/A — explain: <!-- why no tests apply -->

## Safety (§17)

<!-- Required for firmware/electronics/mechanical changes; "N/A" otherwise. -->
- [ ] No regression to water/electrical isolation, pump fail-safe, leak lockout, or thermal limits — or risk-register entry added/updated (`docs/risk-register.md`).

## Reviewer

<!-- Tag the owning track's reviewer. A PR merges only after that track approves and CI is green. -->
- Owning-track reviewer: @
