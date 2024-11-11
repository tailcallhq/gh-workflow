# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0](https://github.com/tailcallhq/rust-gh-workflow/compare/gh-workflow-v0.2.1...gh-workflow-v0.3.0) - 2024-11-11

### Other

- reorder fields in Job

## v0.2.0 (2024-11-11)

<csr-id-d058f85eed4f6904cda24330f2f4e1c95a926257/>
<csr-id-d5a2987bb6aadbf0430b7a712f9f63d6552acf3e/>
<csr-id-96be2435d64d679b267ceeac4918b2b8bc07a77d/>

### Chore

 - <csr-id-d058f85eed4f6904cda24330f2f4e1c95a926257/> update build.rs using `Generate` and custom name
 - <csr-id-d5a2987bb6aadbf0430b7a712f9f63d6552acf3e/> impl `Step::working_directory`

### New Features

 - <csr-id-f2441994724242e1ea761bf6cadd023f346b383e/> add `add_with` operator` on StepValue
 - <csr-id-9a4b40af898fa766ce3b65c23dfe4c775839b785/> add Setters for Event
 - <csr-id-375862bc427518f715165c97442ab172ae379b8f/> Make generate public fixes #26

### Bug Fixes

 - <csr-id-c5b83cb65e16e0b2d0d8ab2a1b5b519db4693d82/> `Job::runs_on`
 - <csr-id-8027f4f3a6a95ae02044f5ac83386c2b0c53ec27/> add names to default Rust workflow
 - <csr-id-790765acf303891b491a37ee7d07413debd90f67/> Improve type-safety for Step

### Refactor

 - <csr-id-96be2435d64d679b267ceeac4918b2b8bc07a77d/> many many changes

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Improve type-safety for Step ([`790765a`](https://github.com/tailcallhq/rust-gh-workflow/commit/790765acf303891b491a37ee7d07413debd90f67))
</details>

## v0.1.2 (2024-11-08)

<csr-id-9230d1f1dc00b87038ec17f39deca77c9be6ffa6/>

### Chore

 - <csr-id-9230d1f1dc00b87038ec17f39deca77c9be6ffa6/> lint fixes

### New Features

 - <csr-id-5a3f18477a2bddc662d3f100e7dd6cbf002cdd2f/> add `Workflow::setup_rust()` helper

### Bug Fixes

 - <csr-id-f74da3df992ee7c10564679c10b695e5b96a85c6/> add wasm to possible targets

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gh-workflow v0.1.2 ([`e71bec6`](https://github.com/tailcallhq/rust-gh-workflow/commit/e71bec6475696c55f7db6cb399fe1572bde5d017))
    - Lint fixes ([`9230d1f`](https://github.com/tailcallhq/rust-gh-workflow/commit/9230d1f1dc00b87038ec17f39deca77c9be6ffa6))
    - Add `Workflow::setup_rust()` helper ([`5a3f184`](https://github.com/tailcallhq/rust-gh-workflow/commit/5a3f18477a2bddc662d3f100e7dd6cbf002cdd2f))
    - Add wasm to possible targets ([`f74da3d`](https://github.com/tailcallhq/rust-gh-workflow/commit/f74da3df992ee7c10564679c10b695e5b96a85c6))
</details>

## v0.1.1 (2024-11-08)

### Bug Fixes

 - <csr-id-55d81b06bf673774e9559915cbe1cb49ccf0a6c5/> add readme file

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gh-workflow v0.1.1 ([`4dfbd11`](https://github.com/tailcallhq/rust-gh-workflow/commit/4dfbd11bb17f22245601c7a1d36d701328e8e646))
    - Release gh-workflow v0.1.1 ([`c58445e`](https://github.com/tailcallhq/rust-gh-workflow/commit/c58445effa55bca60e2283205feba758365efe51))
    - Release gh-workflow v0.1.1 ([`677a89e`](https://github.com/tailcallhq/rust-gh-workflow/commit/677a89e38a74410db14c7546499a1ce818befd96))
    - Add readme file ([`55d81b0`](https://github.com/tailcallhq/rust-gh-workflow/commit/55d81b06bf673774e9559915cbe1cb49ccf0a6c5))
</details>

## v0.1.0 (2024-11-08)

<csr-id-7f8b45f3f2065fc953da5fd4447183d0bcb94e38/>
<csr-id-d73d9faf093e15fc9d91a9318ca84f6113a310b0/>
<csr-id-1a004fc27cb6c44a6b76d5ca20b50dbfc90b4efe/>
<csr-id-55f52de53065d626aaea8007651251a9a86acc98/>
<csr-id-90545329d44378175e1bbabe5595868720961dad/>
<csr-id-dccdb3612602559e3c39e83ae3894086e06c5a5d/>
<csr-id-5e43aa120b9f37227bc0d1e9d2c3c840a652319e/>
<csr-id-cbf51abc82da429539cff463aebb83f941b62922/>
<csr-id-17a92e4d66226b5b22feadce6b2b79326be328d1/>

### Chore

 - <csr-id-7f8b45f3f2065fc953da5fd4447183d0bcb94e38/> update license
 - <csr-id-d73d9faf093e15fc9d91a9318ca84f6113a310b0/> add license
 - <csr-id-1a004fc27cb6c44a6b76d5ca20b50dbfc90b4efe/> update build
 - <csr-id-55f52de53065d626aaea8007651251a9a86acc98/> accept path `AsRef<Path>`
 - <csr-id-90545329d44378175e1bbabe5595868720961dad/> add warning in generated config
 - <csr-id-dccdb3612602559e3c39e83ae3894086e06c5a5d/> autogen CI
 - <csr-id-5e43aa120b9f37227bc0d1e9d2c3c840a652319e/> update folder structure

### Chore

 - <csr-id-17a92e4d66226b5b22feadce6b2b79326be328d1/> update license

### Chore

 - <csr-id-cbf51abc82da429539cff463aebb83f941b62922/> add changelog

### New Features

 - <csr-id-8f3615d7d3e6b71946b09c68a4ec8dc7cb3418a7/> validate on CI
 - <csr-id-d770fc8bec19a8dd6e7b680b8d61819383b50498/> add events

### Bug Fixes

 - <csr-id-e91cd8944cfa9cf758889c4157f12bc01758fee1/> tests
   * refactor: move tests from `workflow.rs` to tests dir
* remove unused deps
* move fixtures dir
* chore: rename files
* chore: import updates
* chore: update workflows
* chore: update workflow
* fix err on workflow-bench.yml
* partial
* partially fix tests
* fix tests
* drop unused struct
* revert unwanted change in `needs`
* revert unwanted change in `needs`
* drop `build_matrix.yml`
* revert

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 18 commits contributed to the release.
 - 12 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 4 unique issues were worked on: [#13](https://github.com/tailcallhq/rust-gh-workflow/issues/13), [#14](https://github.com/tailcallhq/rust-gh-workflow/issues/14), [#19](https://github.com/tailcallhq/rust-gh-workflow/issues/19), [#21](https://github.com/tailcallhq/rust-gh-workflow/issues/21)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#13](https://github.com/tailcallhq/rust-gh-workflow/issues/13)**
    - Tests ([`e91cd89`](https://github.com/tailcallhq/rust-gh-workflow/commit/e91cd8944cfa9cf758889c4157f12bc01758fee1))
 * **[#14](https://github.com/tailcallhq/rust-gh-workflow/issues/14)**
    - Autogen CI ([`dccdb36`](https://github.com/tailcallhq/rust-gh-workflow/commit/dccdb3612602559e3c39e83ae3894086e06c5a5d))
 * **[#19](https://github.com/tailcallhq/rust-gh-workflow/issues/19)**
    - Accept path `AsRef<Path>` ([`55f52de`](https://github.com/tailcallhq/rust-gh-workflow/commit/55f52de53065d626aaea8007651251a9a86acc98))
 * **[#21](https://github.com/tailcallhq/rust-gh-workflow/issues/21)**
    - Validate on CI ([`8f3615d`](https://github.com/tailcallhq/rust-gh-workflow/commit/8f3615d7d3e6b71946b09c68a4ec8dc7cb3418a7))
 * **Uncategorized**
    - Release gh-workflow v0.1.0 ([`374d65c`](https://github.com/tailcallhq/rust-gh-workflow/commit/374d65c8c638b50efe5724d44f2bc1b409ab5a56))
    - Update license ([`17a92e4`](https://github.com/tailcallhq/rust-gh-workflow/commit/17a92e4d66226b5b22feadce6b2b79326be328d1))
    - Release gh-workflow v0.1.0 ([`3d5543b`](https://github.com/tailcallhq/rust-gh-workflow/commit/3d5543b427685752770a75ce3d078f7b38f7a5f2))
    - Release gh-workflow v0.1.0 ([`4602f22`](https://github.com/tailcallhq/rust-gh-workflow/commit/4602f22c2437cad2467ee083402b0eb7f29ab045))
    - Add changelog ([`cbf51ab`](https://github.com/tailcallhq/rust-gh-workflow/commit/cbf51abc82da429539cff463aebb83f941b62922))
    - Update license ([`7f8b45f`](https://github.com/tailcallhq/rust-gh-workflow/commit/7f8b45f3f2065fc953da5fd4447183d0bcb94e38))
    - Update manifest ([`770b3e3`](https://github.com/tailcallhq/rust-gh-workflow/commit/770b3e33773db936b91a416fb3ac26c809b2ad14))
    - Add license ([`d73d9fa`](https://github.com/tailcallhq/rust-gh-workflow/commit/d73d9faf093e15fc9d91a9318ca84f6113a310b0))
    - Update build ([`1a004fc`](https://github.com/tailcallhq/rust-gh-workflow/commit/1a004fc27cb6c44a6b76d5ca20b50dbfc90b4efe))
    - Update event ([`a0bf957`](https://github.com/tailcallhq/rust-gh-workflow/commit/a0bf95769e66eed96cc9b5b81c51bcf38e1b49fc))
    - Drop commented code ([`fe53956`](https://github.com/tailcallhq/rust-gh-workflow/commit/fe539566c65313f19f8773fea6cf78aa49cb7e65))
    - Add events ([`d770fc8`](https://github.com/tailcallhq/rust-gh-workflow/commit/d770fc8bec19a8dd6e7b680b8d61819383b50498))
    - Add warning in generated config ([`9054532`](https://github.com/tailcallhq/rust-gh-workflow/commit/90545329d44378175e1bbabe5595868720961dad))
    - Update folder structure ([`5e43aa1`](https://github.com/tailcallhq/rust-gh-workflow/commit/5e43aa120b9f37227bc0d1e9d2c3c840a652319e))
</details>

