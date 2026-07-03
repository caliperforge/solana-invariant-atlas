# Coverage map (excerpt)

Compressed view of the full map for the README. The full table with
file-level citations, reader-proofing footer, applicability survey,
and appendix lives at [docs/coverage_map.md](coverage_map.md). This
excerpt exists so a README reader gets the shape of the map without
opening the full doc; T-satlas-06 drops this section into the
README's coverage-map slot per the build spec section 8 order.

Column semantics: each program-level comparator column shows whether a
class-shaped fixture is shipped (paired clean + planted variants of
the same program with the invariant proven to fire on planted and pass
on clean) at the cell. The rightmost column is temporarily-clean as of
2026-07-02 per the eval section 12.2 and does NOT read as a durable
exclusivity claim.

| Invariant class | Trident v0.12.0 | Crucible v0.2.0 | Anchor native + solana-program-test v1.1.2 | mollusk v0.13.4 | LiteSVM v0.13.1 | Audit-shop suites (awesome-trident-tests, `main` @ 2026-07-02) | Uncovered by comparators as of 2026-07-02 (temporarily-clean) |
|---|---|---|---|---|---|---|---|
| `balance_conservation` | harness-only | harness-only | harness-only, unit-level only | harness-only, unit-level only | (empty) | per-target, no class-shaped fixture | shipped by atlas |
| `monotonic_accounting` | harness-only | harness-only | harness-only, unit-level only | harness-only, unit-level only | (empty) | per-target, no class-shaped fixture | shipped by atlas |
| `access_control` | harness-only | harness-only | harness-only, unit-level only | harness-only, unit-level only | (empty) | per-target, no class-shaped fixture | shipped by atlas |
| `access_control.collateral_authority` (sub-shape) | harness-only | harness-only | (empty) | (empty) | (empty) | (empty) | shipped by atlas |
| `oracle_freshness` (planned, Phase 2) | (planned) | (planned) | (planned) | (planned) | (planned) | (planned) | not shipped by atlas |

**Reader notes for the excerpt.** Solfuzz and sig-fuzz are VM-conformance
harnesses, not program-level comparators, so they are not columns above
(eval section 6). The rightmost column carries an explicit
"as of 2026-07-02" date because no comparator surveyed as of that date
ships a class-shaped fixture matching the atlas rows; this is a
temporarily-clean observation and not a claim of durable exclusivity.
The full map's reader-proofing footer, maintenance rules, applicability
survey, and appendix are at [docs/coverage_map.md](coverage_map.md).
