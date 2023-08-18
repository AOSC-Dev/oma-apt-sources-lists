# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.1.0 (2023-08-18)

<csr-id-c2d237db5c0a4a802bf3c53f2d94518b61a64875/>
<csr-id-0df4611ec7293010551f95659f28f1b1da9cc0c8/>
<csr-id-10595046b4bff922fe47e313ddb9db49047635e9/>
<csr-id-b757a9a4f3b1ffadc9d05f6133ce4b59e0623fe7/>
<csr-id-658ccc1a4ced1670885e95467a77d6b8c1f87fa1/>

### Chore

 - <csr-id-c2d237db5c0a4a802bf3c53f2d94518b61a64875/> use 2021 edition and remove upstream rustfmt.toml and toolchain
   - Also cargo clippy
 - <csr-id-0df4611ec7293010551f95659f28f1b1da9cc0c8/> Update dependencies
 - <csr-id-10595046b4bff922fe47e313ddb9db49047635e9/> Upgrade reqwest dependency
 - <csr-id-b757a9a4f3b1ffadc9d05f6133ce4b59e0623fe7/> Upgrade Rust to 1.35.0

### Chore

 - <csr-id-49dd2c48094a838bae10e5a699be40df0dac83d5/> add changelog

### New Features

 - <csr-id-1268de431fb64af45cf6e301509092ea5e225f5b/> Add callback-based entries iterator for SourcesLists

### Bug Fixes

 - <csr-id-3e72f14a2d2dc1fbcb5fbd39e587e1930a262749/> Fix the upgrade example

### Other

 - <csr-id-658ccc1a4ced1670885e95467a77d6b8c1f87fa1/> add methods for fetching dist URLs

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 24 commits contributed to the release over the course of 1744 calendar days.
 - 8 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Add changelog ([`49dd2c4`](https://github.com/AOSC-Dev/oma-apt-sources-lists/commit/49dd2c48094a838bae10e5a699be40df0dac83d5))
    - Use 2021 edition and remove upstream rustfmt.toml and toolchain ([`c2d237d`](https://github.com/AOSC-Dev/oma-apt-sources-lists/commit/c2d237db5c0a4a802bf3c53f2d94518b61a64875))
    - Update dependencies ([`0df4611`](https://github.com/AOSC-Dev/oma-apt-sources-lists/commit/0df4611ec7293010551f95659f28f1b1da9cc0c8))
    - Add callback-based entries iterator for SourcesLists ([`1268de4`](https://github.com/AOSC-Dev/oma-apt-sources-lists/commit/1268de431fb64af45cf6e301509092ea5e225f5b))
    - Upgrade reqwest dependency ([`1059504`](https://github.com/AOSC-Dev/oma-apt-sources-lists/commit/10595046b4bff922fe47e313ddb9db49047635e9))
    - Upgrade Rust to 1.35.0 ([`b757a9a`](https://github.com/AOSC-Dev/oma-apt-sources-lists/commit/b757a9a4f3b1ffadc9d05f6133ce4b59e0623fe7))
    - Fix the upgrade example ([`3e72f14`](https://github.com/AOSC-Dev/oma-apt-sources-lists/commit/3e72f14a2d2dc1fbcb5fbd39e587e1930a262749))
    - :ambulance: Fix repo modifications to apply to all entries of a given URI ([`0f2c82f`](https://github.com/AOSC-Dev/oma-apt-sources-lists/commit/0f2c82fb44faf4aec1c9cce8ad8eef74451c5ecd))
    - :ambulance: Fix dist_replace only replacing exact suite matches ([`a6703be`](https://github.com/AOSC-Dev/oma-apt-sources-lists/commit/a6703be9b5a3939ef68afdccc0d11e061add7d0f))
    - :boom: Support retaining a repo if it is in the retain set ([`4d3f946`](https://github.com/AOSC-Dev/oma-apt-sources-lists/commit/4d3f946f4eca07824b0235e4b72bb219381610f1))
    - :bomb: :hammer: :sparkles: SourcesFile -> SourcesList; Repo Modification Support ([`b44bf5a`](https://github.com/AOSC-Dev/oma-apt-sources-lists/commit/b44bf5a2126f93dc4cc4663acdcc7140aeedbe41))
    - :hammer: Refactoring ([`a9178b5`](https://github.com/AOSC-Dev/oma-apt-sources-lists/commit/a9178b560e7bdec0bf62916e2854f69f42727cc5))
    - :sparkles: Add SourcesList::dist_replace ([`e6da744`](https://github.com/AOSC-Dev/oma-apt-sources-lists/commit/e6da744aa64e61e57afd04ea38487868bc4287ee))
    - :hammer: Allow files to be written after all in-memory modifications have been made ([`a3cfd08`](https://github.com/AOSC-Dev/oma-apt-sources-lists/commit/a3cfd083993c32546a04c8da6686ae50fa0e206d))
    - :sparkles: Prevent storing duplicate SourceLines ([`aff87d2`](https://github.com/AOSC-Dev/oma-apt-sources-lists/commit/aff87d2cd91b374a591e047f479650d4474717e1))
    - :ambulance: Fix writing of source lists ([`2c69e3a`](https://github.com/AOSC-Dev/oma-apt-sources-lists/commit/2c69e3a646bca1af25f1172d616fcd4e743ddb13))
    - :sparkles: On insert, update entries which already exist ([`27eee86`](https://github.com/AOSC-Dev/oma-apt-sources-lists/commit/27eee86c3f331cecc94623bc0662f2b3631ba2b4))
    - :sparkles: Support inserting, removing, and commenting entries ([`d23b0c2`](https://github.com/AOSC-Dev/oma-apt-sources-lists/commit/d23b0c21c176a4a028a5c175328143a2992c4fb6))
    - :ambulance: Fix options displaying in SourceEntry ([`dd43eb5`](https://github.com/AOSC-Dev/oma-apt-sources-lists/commit/dd43eb58f21a35150a5f69a99135708a5f76e0c6))
    - Derive Clone/Debug for SourcesList ([`b1b7ede`](https://github.com/AOSC-Dev/oma-apt-sources-lists/commit/b1b7ede81d64dc3318bc7e3f427512e3d1c26524))
    - Add SourcesList::dist_upgrade ([`af69276`](https://github.com/AOSC-Dev/oma-apt-sources-lists/commit/af69276c681cb5566ec1111614f8bd7c3433d488))
    - Add methods for fetching dist URLs ([`658ccc1`](https://github.com/AOSC-Dev/oma-apt-sources-lists/commit/658ccc1a4ced1670885e95467a77d6b8c1f87fa1))
    - Support getting URLs to the dist and pool paths ([`aa99485`](https://github.com/AOSC-Dev/oma-apt-sources-lists/commit/aa9948592d2108bb6aa4ca8e89427e48db5d8969))
    - Initial commit ([`b8ebe4d`](https://github.com/AOSC-Dev/oma-apt-sources-lists/commit/b8ebe4dbb7b116a906c78b588cb4fe4b5bd2bafd))
</details>

