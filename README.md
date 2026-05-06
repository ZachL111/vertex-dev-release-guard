# vertex-dev-release-guard

`vertex-dev-release-guard` explores developer tools with a small Rust codebase and local fixtures. The technical goal is to build a Rust toolkit that studies release behavior through bounded scenario files, with conflict explanations and offline replay mode.

## Use Case

I want this repository to be useful as a quick reading exercise: fixtures first, implementation second, verifier last.

## Vertex Dev Release Guard Review Notes

The first comparison I would make is `review cost` against `diagnostic quality` because it shows where the rule is most opinionated.

## Highlights

- `fixtures/domain_review.csv` adds cases for change width and diagnostic quality.
- `metadata/domain-review.json` records the same cases in structured form.
- `config/review-profile.json` captures the read order and the two review questions.
- `examples/vertex-dev-release-walkthrough.md` walks through the case spread.
- The Rust code includes a review path for `review cost` and `diagnostic quality`.
- `docs/field-notes.md` explains the strongest and weakest cases.

## Code Layout

The repository has two validation layers: the original compact policy fixture and the domain review fixture. They are separate so one can change without hiding failures in the other.

The Rust addition stays small enough to inspect in one sitting.

## Run The Check

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/verify.ps1
```

## Regression Path

The check exercises the source code and the review fixture. `edge` is the high score at 229; `stress` is the low score at 152.

## Future Work

This remains a local project with deterministic fixtures. It does not depend on credentials, hosted services, or live data. Future work should add richer malformed inputs before widening the public API.
