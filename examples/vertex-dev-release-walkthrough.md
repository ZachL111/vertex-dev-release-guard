# Vertex Dev Release Guard Walkthrough

This walk-through keeps the domain vocabulary close to the data instead of burying it in prose.

| Case | Focus | Score | Lane |
| --- | --- | ---: | --- |
| baseline | change width | 183 | ship |
| stress | diagnostic quality | 152 | ship |
| edge | review cost | 229 | ship |
| recovery | safe rewrite | 182 | ship |
| stale | change width | 181 | ship |

Start with `edge` and `stress`. They create the widest contrast in this repository's fixture set, which makes them better review anchors than the middle cases.

`edge` is the optimistic case; use it to make sure the scoring path still rewards strong signal.
