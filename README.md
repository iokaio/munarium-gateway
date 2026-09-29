# Munarium Gateway

**Model-call mediation: routing, BYOK, budgets, screening.** Gateway is the mediation-plane
component of the Munarium Governance Platform that stands between an agent runtime and a model
provider. It selects the route, applies the deployment's data-release policy, reserves budget before
dispatch, settles observed usage afterwards, and leaves an invocation record that links partial
results and later reconciliation. A model call can itself be consequential because it moves data
and consumes a budget; Gateway authorizes it according to that consequence.

> **Status: Planned — Rust scaffold present.** This checkout contains a dependency-free,
> non-publishable [Cargo library](Cargo.toml) and documented interfaces under [src/](src/lib.rs).
> The interfaces have no implementations: no runtime service, client transport, database,
> provider integration or contract implementation is available. No production path is qualified.
> Build checks validate source structure, not governance capabilities. The
> [capability table](#capability-status) remains the authoritative functional status.

Gateway is one of nine components built around the existing Munarium foundation, Munarium Server
and Munarium Matrix. Their shared architecture, normative contracts, decision records, roadmap and
composition evidence live in the public hub,
[iokaio/munarium-platform](https://github.com/iokaio/munarium-platform). This repository will hold
Gateway's implementation, its unit and component tests, the migrations it owns, operational
diagnostics, package definitions, a local development recipe and release evidence. It is open
source from its first public commit, under the Apache License 2.0, with no proprietary edition.

## Start building

Read the [development index](docs/README.md), then the [architecture](docs/architecture.md),
[implementation plan](docs/implementation-plan.md) and [validation guide](docs/validation.md).
They map the public platform plan to source modules, dependencies, a first bounded work item
and acceptance cases. Runtime capabilities remain planned; supported contract versions are **none**.

## What Gateway is for

Munarium Server already contains a bring-your-own-key provider gateway with usage and monetary
accounting. The platform extracts that capability so other agent runtimes can use it, while
**preserving one implementation lineage** rather than maintaining incompatible accounting engines.
Gateway is that extraction, plus the new work the source architecture distinguishes from what
Server already does: monetary **admission caps** before dispatch, not only accounting after it.

Gateway is a mediation component, not an authority. It enforces the deployment's endpoint and data
policy and its hard budget checks locally, continuing to do so when Sentinel is unavailable.

## The design, as planned

### Extraction before expansion

The first task is to identify reusable Server code and its actual tests, then factor a versioned
library or service boundary **without changing existing Server behavior unexpectedly**. This is the
hub's S8 change to Server, and it is delivered as a separately tested feature.

The first reference release qualifies **one hosted-provider route and one local-inference route**.
Broader support for Azure-hosted models, Bedrock, Vertex AI, NVIDIA endpoints, vLLM, Ollama and
other providers follows the same adapter contract when its evidence exists. Provider names in the
architecture are targets, not a compatibility certification.

### Budget admission and settlement

A model invocation **reserves** an allowed amount of capacity before dispatch. The accounting
identity includes tenant, agent release, task and delegated work. **Settlement** records observed
usage, price assumptions, provider response metadata and corrections. Child tasks cannot spend
outside the root task's approved ceiling by creating new identifiers.

Unknown pricing or incomplete usage reports require a conservative policy: a reservation may be
based on a configured maximum, and a provider response may later reconcile the estimate. The
documentation states **where a monetary bound is exact, where it is conservative, and where only
token limits are enforced**. A nominal dollar cap must not imply an enforceable ceiling the provider
interface cannot support.

If required admission evidence cannot be durably recorded, **Gateway does not dispatch the call**.
Cancellation and timeout account for the possibility that the provider still completed billable
work. Streaming responses and tool-call outputs carry invocation identifiers so partial results and
later reconciliation can be linked.

### Data and endpoint controls

The model path is a data-disclosure path. Endpoint allowlists, tenant boundaries, data
classifications, retention preferences and approved processing locations are checked **before a
prompt is sent**. A local retrieval index does not make a subsequent hosted-model request local. The
reference documentation identifies exactly which data leaves the deployment.

Prompt-injection and content-safety classifiers provide signals that can trigger review or
additional restrictions. **Their absence or agreement cannot manufacture authority.** Deterministic
metadata (an approved endpoint, a verified sensitivity label) is distinguished from probabilistic
detection results.

Gateway avoids retaining raw prompt content in public diagnostics by default. Invocation hashes,
policy decisions, usage records and evidence references support accountability without turning the
ledger into an archive of private material.

## First public increment

**One hosted-provider path and one local-inference path, each with invocation receipts**, with
budget admission, endpoint policy and settlement, extracted from Server's provider gateway in a way
Server's existing tests still pass.

Target window: Stage 3 (months 7–9); the extraction from Server (S8) is scoped during Stages 1–2.

## Capability status

The labels are evidence labels, not editions: **Planned**, **Experimental**, **Conformance-tested**,
**Reference-qualified**, **Independently reviewed**. In the hub's component catalog this
repository is at **repository created**.

| Capability | Status | Evidence |
|---|---|---|
| Extraction of Server's provider gateway into a versioned library or service boundary (S8) | Planned | none |
| One hosted-provider route | Planned | none |
| One local-inference route | Planned | none |
| Budget reservation before dispatch; settlement with usage, price assumptions, provider metadata, corrections | Planned | none |
| Root-task ceilings enforced across child tasks | Planned | none |
| Endpoint allowlists, tenant boundaries, classification, retention and processing-location checks | Planned | none |
| Invocation records with identifiers on streamed and tool-call outputs | Planned | none |
| Cancellation and timeout accounting for possibly billable work | Planned | none |
| Redaction of raw prompt content from public diagnostics by default | Planned | none |
| Classifier signals as review triggers, distinguished from authority | Planned | none |
| Further providers (Azure-hosted, Bedrock, Vertex AI, NVIDIA, vLLM, Ollama and others) | Deferred; same adapter contract, own evidence | none |

Supported contract versions: **none**. Qualified provider routes: **none**. Operations available
today: **none**.

## Acceptance evidence for the first release

| Test | Required outcome |
|---|---|
| Concurrent budget reservations | Never over-admit against the shared ceiling |
| Nested task spending | A child task cannot exceed the root task's approved ceiling by creating new identifiers |
| Price-version change | Reservation and settlement record which price assumptions applied |
| Cancellation and timeout | Possibly billable work is accounted for and reconciled |
| Provider failure | Recorded; no phantom settlement |
| Wrong endpoint selection | Refused before dispatch |
| Admission evidence not durably recordable | No dispatch |
| Redaction | Raw prompt content absent from public diagnostics |
| Classifier disagreement | Triggers review or restriction; never authorizes |
| Performance | Provider latency reported separately from Gateway overhead |
| Server compatibility | Server's existing provider tests pass against the extracted boundary |

A blank evidence field means unverified, not passed.

## Invariants

| ID | Required property | Owner and first gate |
|---|---|---|
| INV-14 | Required-recording failure prevents new consequential dispatch | Server, Gate, Gateway; stages 2–3 |
| INV-15 | Task and child-task reservations cannot exceed the enforced shared budget | Gateway; stage 3 |
| INV-22 | A release advertises only the profiles and capabilities supported by its evidence | every component; every stage |

## Contracts, dependencies and neighbors

- **Contracts.** The hub's contracts directory is normative for the invocation record and the
  accounting identity. Gateway implements them. Supported contract versions: none yet.
- **Foundation.** Munarium Server 1.3.0 is the source of the provider gateway and of the usage and
  monetary accounting Gateway extracts; the hub's S8 is the Server change. The extraction preserves
  Server compatibility and one accounting lineage.
- **Gate** shares the data-classification and DLP adapter surface (consume verified labels; keep
  probabilistic signals distinguishable from authority); **Sentinel** reads budget saturation and
  invocation records; **Warden** supplies the verified principal context in the accounting
  identity; **Assure** includes Gateway's records in evidence packages.
- **External dependencies.** The two qualified provider routes, chosen and recorded with the first
  implementation.

## Not in scope

- A second, incompatible accounting engine.
- Duplicating a provider's full API surface before an actual use case requires it.
- Treating a nominal dollar cap as enforceable where the provider interface cannot support it.
- Treating a hosted-model request as local because the retrieval index is.
- Letting a classifier's silence or agreement authorize anything.
- Provider compatibility claims without the provider's own conformance evidence.

## Roadmap position

| Stage | Gateway's part |
|---|---|
| 0 · month 1 | This repository; the invocation-record and accounting-identity contracts drafted in the hub |
| 1–2 · months 2–6 | Inventory of reusable Server code and tests; the S8 boundary scoped in a hub decision record |
| 3 · months 7–9 | Extraction; one hosted and one local route; budget concurrency tests; daily use on bounded development tasks |
| 4 · months 10–12 | Inclusion in the reference composition and evidence packs |
| 5 · months 13+ | Further providers under the same adapter contract, demand-led |

Hard budget checks and endpoint policy are never removed to preserve a date.

## Repository layout

| Path | What exists |
|---|---|
| [Cargo.toml](Cargo.toml), [Cargo.lock](Cargo.lock) | Independent library, version 0.1.0-dev, publishing disabled, no external crate dependencies |
| [src/lib.rs](src/lib.rs) | Documented proposed module interfaces; no runtime implementations |
| [docs/](docs/README.md) | Architecture, implementation sequence and acceptance specifications |
| [CONTRIBUTING.md](CONTRIBUTING.md), [AGENTS.md](AGENTS.md), [CLAUDE.md](CLAUDE.md) | Contribution process and aligned development guidance |
| [.github/workflows/](.github/workflows/) | Automatic Rust, repository-hygiene and DCO checks |
| [scripts/](scripts/), [check_license.py](check_license.py) | Existing documentation, private-material and license checks |
| [LICENSE](LICENSE), [NOTICE](NOTICE), [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) | Licensing and dependency notices |

Subsystem modules: [routing](src/routing.rs), [budget](src/budget.rs), [provider](src/provider.rs), [settlement](src/settlement.rs).
Tests, fixtures, migrations, binaries and deployment assets arrive with the implementation that
uses them. The scaffold defines no shared wire types and depends on no sibling checkout.

## Development

Use Rust 1.98.1 with rustfmt, Clippy and the platform's native linker. From this repository root:

```console
cargo fmt --all --check
cargo build --offline --locked
cargo clippy --offline --locked --all-targets -- -D warnings
cargo test --offline --locked
cargo doc --offline --locked --no-deps
```

The crate currently has **zero runtime or conformance tests**. A successful test command checks
the scaffold only. The [validation guide](docs/validation.md) gives the required behavioral
test specifications and explains how to retain evidence when they are implemented.

Also run the existing hygiene gates:

```console
py check_license.py
py scripts/private_material_scan.py
py scripts/docs_linkcheck.py
gitleaks dir . --config .gitleaks.toml --no-banner --redact --exit-code 1
git diff --check
```

Use `python` or `python3` where `py` is unavailable. The new
[Rust workflow](.github/workflows/rust.yml) runs on main pushes and pull requests alongside
the existing [repository hygiene](.github/workflows/repo-hygiene.yml) and
[DCO](.github/workflows/dco.yml) workflows. They provide build and repository checks, not a
qualified runtime. No package is published or service deployed by these workflows.
Local checks do not imply hosted CI success. See [CONTRIBUTING.md](CONTRIBUTING.md).

## The platform

| Repository | Plane | Role |
|---|---|---|
| [iokaio/munarium-platform](https://github.com/iokaio/munarium-platform) | hub | Architecture, normative contracts, decision records, roadmap and composition evidence for the whole platform |
| [iokaio/munarium](https://github.com/iokaio/munarium) | foundation (mediation) | Munarium Server: governed memory, the append-only ledger, and the Server client libraries |
| [iokaio/munarium-matrix](https://github.com/iokaio/munarium-matrix) | foundation (mediation) | Munarium Matrix: governed, read-only structured evidence from enterprise data sources |
| [iokaio/munarium-registry](https://github.com/iokaio/munarium-registry) | authority | Inventory of agents, tools, manifests, and policy bundles |
| [iokaio/munarium-harness](https://github.com/iokaio/munarium-harness) | agent | SDKs that make the governed path easy for honest agents |
| [iokaio/munarium-warden](https://github.com/iokaio/munarium-warden) | authority | Workload identity, delegation, just-in-time credentials, kill switches |
| [iokaio/munarium-gate](https://github.com/iokaio/munarium-gate) | mediation | Policy decision and enforcement point for every tool call |
| [iokaio/munarium-gateway](https://github.com/iokaio/munarium-gateway) | mediation | Model-call mediation: routing, BYOK, budgets, screening |
| [iokaio/munarium-council](https://github.com/iokaio/munarium-council) | authority | Approvals, policy lifecycle, ratified governance transitions |
| [iokaio/munarium-sentinel](https://github.com/iokaio/munarium-sentinel) | assurance | Telemetry, anomaly detection, circuit breakers, incident replay |
| [iokaio/munarium-assure](https://github.com/iokaio/munarium-assure) | assurance | Control-framework mapping and evidence packs |
| [iokaio/munarium-console](https://github.com/iokaio/munarium-console) | assurance | One interface for approvers, operators, and auditors |
| [iokaio/munarium-clients-publish](https://github.com/iokaio/munarium-clients-publish) | tooling | The one place Munarium client packages are built for release and published from |
| [iokaio/munarium-demo](https://github.com/iokaio/munarium-demo) | examples | Munarium Demo: working applications and bundled datasets for evaluating the foundation |

The development tool VCP ([iokaio/vcp](https://github.com/iokaio/vcp)) is separate: not one of the
nine components and not a runtime dependency for adopters. Ioka's private repositories hold
planning material awaiting publication review and the proprietary Matrix analytics adapters;
nothing from them is copied into a public repository without that review.

## Licensing

Apache-2.0 ([LICENSE](LICENSE), [NOTICE](NOTICE)). The names are not part of that grant:
[TRADEMARK.md](TRADEMARK.md) says what you may do without asking, which is most things. There is
no proprietary edition of this component and none is planned; a capability that arrives later is
deferred roadmap work, not a commercial restriction.

## Contributing, support, security

Signed-off pull requests, no CLA ([CONTRIBUTING.md](CONTRIBUTING.md)). Questions go to Discussions,
defects and design findings to Issues, and suspected vulnerabilities to the private channel
[SECURITY.md](SECURITY.md) names, never a public issue. What is and is not supported:
[SUPPORT.md](SUPPORT.md). Conduct: [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md). Release history,
such as it is: [CHANGELOG.md](CHANGELOG.md).
