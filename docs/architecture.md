# Munarium Gateway implementation architecture

**Proposed design; scaffold only.** Based on section 12 of the
[platform plan, revision 4](https://github.com/iokaio/munarium-platform/blob/main/docs/platform-plan.md), with lifecycle and failure rules in
sections 17–19 and 22. See the hub's
[scaffold decision proposal](https://github.com/iokaio/munarium-platform/blob/main/docs/decisions/0001-scaffold-boundaries.md)
for the distinction between local interfaces and normative contracts.

## Responsibility and current boundary

Model route admission, durable shared-budget reservation, and invocation settlement. Gateway belongs to the **mediation plane**.
The crate declares interfaces only: no concrete implementations, serialization,
network listeners, persistence, service authentication or target operations exist.

The associated input, output and error types are intentionally unspecified.
These are proposed in-process seams for implementation work, not a released Rust API
or a second definition of the shared wire contract. A trait signature does not enforce
the trust assumptions below. Async runtime, transport and storage choices remain open.

## Module map

| Source | Proposed interface | Responsibility |
|---|---|---|
| [routing](../src/routing.rs) | `RouteAdmission` | Validate endpoint, tenant, classifications, retention and processing location before prompt disclosure; classifier signals do not confer authority. |
| [budget](../src/budget.rs) | `BudgetLedger` | Child tasks share the approved ceiling. Unknown prices, incomplete usage and cancellation require conservative accounting, not invented precision. |
| [provider](../src/provider.rs) | `ProviderAdapter` | Extract the Server implementation lineage first. Stream fragments and ambiguous outcomes must retain invocation identity. |
| [settlement](../src/settlement.rs) | `UsageSettlement` | Retain price assumptions and provider metadata; timeout or cancellation does not prove that no billable work occurred. |

## Planned flow and state ownership

Authenticate invocation → enforce endpoint and data policy → durably reserve against root-task ceiling → dispatch admitted invocation → retain identity on stream/tool outputs → settle usage and corrections. Cancellation or missing usage remains an accounting obligation.

Own reservation/settlement state only through the versioned accounting lineage extracted with Server S8. Reuse existing provider behavior and compatibility tests before adding admission caps. Do not create a competing accounting engine in this scaffold.

## Dependencies and failure behavior

| Dependency | Required input or service | Failure rule |
|---|---|---|
| Server S8 | Extraction point, existing provider/accounting tests, compatibility seam | Standalone implementation waits for the shared ownership decision. |
| Registry / policy inputs | Approved endpoints and data-release context | Unknown classification, endpoint or location is not implicitly allowed. |
| Server / durable budget store | Reservation and required invocation records | No durable admission record, no dispatch. |
| Hosted/local providers | Usage and timeout semantics for each qualified route | Uncertain billable work must be reconciled rather than erased. |

No dependency is linked into this scaffold. Supported contract versions are **none**.
Future adapters must consume a reviewed, versioned contract and identify its digest;
a floating hub branch is design context, never deployment authority.

## Threat assumptions

Treat agent code, supplied content and self-reported identity as untrusted.
Host administrators, release roots and required signing authorities remain explicit
trust assumptions of a qualified deployment. Process separation alone does not prove
independent administration.

| Threat | Required control to implement and test |
|---|---|
| Runaway nested-task spending | Atomic shared-root admission, bounded delegation, explicit settlement. |
| Prompt exfiltration through a route | Endpoint/data/location checks before dispatch and redacted diagnostics. |
| Usage ambiguity or accounting divergence | One extracted lineage, versioned prices and recorded corrections. |

The [validation specification](validation.md) connects these requirements to the hub
invariants. No test evidence is implied by this design.

## Decisions needed before implementation

Inventory and pin the actual Server gateway before extraction. Decide reservation transaction, currency/price representation, shared-task limits, streaming reconciliation and enforceable monetary bounds; no provider SDK is selected by this scaffold.

A cross-component semantic change starts in a hub decision record. Keep publication,
activation and component implementation separate. Use expand, migrate, remove for
future breaking contract changes; never duplicate hashing, identity or grant rules.

## Deferred scope

Broad provider coverage, duplicated provider API surfaces, and exact monetary guarantees unsupported by a provider contract.

The [implementation plan](implementation-plan.md) sequences the first useful increment.
No deployment recipe, service port or live-provider configuration is supplied at this stage.
