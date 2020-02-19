## 2020-02-19, Version 0.11.0
### Commits
- [[`698f58f9f3`](https://github.com/datrs/sparse-bitfield/commit/698f58f9f333ac5a60bc8c0351cf48baaffb6c03)] (cargo-release) version 0.11.0 (Bruno Tavares)
- [[`3895909bfa`](https://github.com/datrs/sparse-bitfield/commit/3895909bfad1e41b5f1ce2a8bace99728d50e9ca)] Bump to 2018 edition (#16) (Bruno Tavares)
- [[`f4c97ee26d`](https://github.com/datrs/sparse-bitfield/commit/f4c97ee26dfbd6fcc798a64badf08658e8591730)] To bytes (#17) (Bruno Tavares)
- [[`9753787b3a`](https://github.com/datrs/sparse-bitfield/commit/9753787b3a16f5f70dc7e521fb4b72f25ced84af)] Update changelog (Yoshua Wuyts)

### Stats
```diff
 .travis.yml         |  6 +++---
 CHANGELOG.md        | 15 +++++++++++++++
 Cargo.toml          | 22 ++++++++++++----------
 src/lib.rs          | 25 +++++++++++++++++++++----
 tests/model.rs      |  4 +---
 tests/regression.rs |  2 --
 tests/test.rs       | 23 ++++++++++++++++++++---
 7 files changed, 72 insertions(+), 25 deletions(-)
```


## 2019-09-07, Version 0.9.0
### Commits
- [[`ca1817de93`](https://github.com/datrs/sparse-bitfield/commit/ca1817de9355848bed3f4f622448fda5de6ddcd8)] (cargo-release) version 0.9.0 (Yoshua Wuyts)
- [[`5da8f11f70`](https://github.com/datrs/sparse-bitfield/commit/5da8f11f70c5078de302eb62211bb7447ead0269)] update err bound (Yoshua Wuyts)
- [[`d3faf791e4`](https://github.com/datrs/sparse-bitfield/commit/d3faf791e4283104120e28a24a434db86b495368)] Update changelog (Yoshua Wuyts)

### Stats
```diff
 CHANGELOG.md | 23 +++++++++++++++++++++++
 Cargo.toml   |  5 ++---
 src/lib.rs   |  5 ++---
 3 files changed, 27 insertions(+), 6 deletions(-)
```


## 2018-09-03, Version 0.8.1
### Commits
- [[`d77e66b097`](https://github.com/datrs/sparse-bitfield/commit/d77e66b0976f44d3d822dcc78e10ff8b94c2ee45)] (cargo-release) version 0.8.1 (Yoshua Wuyts)
- [[`47ad870ccc`](https://github.com/datrs/sparse-bitfield/commit/47ad870ccc8f403b08b96da5982641de08e8bd3c)] set the bit correctly (#12) (周汉成)
- [[`aee39fded8`](https://github.com/datrs/sparse-bitfield/commit/aee39fded840f676103158ae7fc364886c3f873c)] Update .github (Yoshua Wuyts)
- [[`a921fcef8e`](https://github.com/datrs/sparse-bitfield/commit/a921fcef8ef62a764ed767bc911420e5aa13b774)] init from file (#10) (Yoshua Wuyts)
- [[`049fe04193`](https://github.com/datrs/sparse-bitfield/commit/049fe04193afce481969a849e8303d31fdb78981)] use built-in power of two method (Yoshua Wuyts)
- [[`d730c4057b`](https://github.com/datrs/sparse-bitfield/commit/d730c4057bd05dd57b647f3c9e2e91c9d3ff4ef7)] (cargo-release) start next development iteration 0.8.1-alpha.0 (Yoshua Wuyts)

### Stats
```diff
 .github/ISSUE_TEMPLATE.md                 | 40 ++-----------------------
 .github/ISSUE_TEMPLATE/bug_report.md      | 23 ++++++++++++++-
 .github/ISSUE_TEMPLATE/feature_request.md | 30 +++++++++++++++++++-
 .github/ISSUE_TEMPLATE/question.md        | 18 +++++++++++-
 Cargo.toml                                |  5 +--
 src/lib.rs                                | 51 +++++++++++++++++++++-----------
 tests/fixtures/40_normal.txt              |  1 +-
 tests/test.rs                             | 15 +++++++++-
 8 files changed, 129 insertions(+), 54 deletions(-)
```


