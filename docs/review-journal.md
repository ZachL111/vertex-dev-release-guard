# Review Journal

The review surface for `vertex-dev-release-guard` is deliberately narrow: one fixture, one scoring rule, and one local check.

The local checks classify each case as `ship`, `watch`, or `hold`. That gives the project a small review vocabulary that matches its developer tools focus without claiming live deployment or external usage.

## Cases

- `baseline`: `change width`, score 183, lane `ship`
- `stress`: `diagnostic quality`, score 152, lane `ship`
- `edge`: `review cost`, score 229, lane `ship`
- `recovery`: `safe rewrite`, score 182, lane `ship`
- `stale`: `change width`, score 181, lane `ship`

## Note

The useful failure mode here is a wrong decision on a named case, not a vague style disagreement.
