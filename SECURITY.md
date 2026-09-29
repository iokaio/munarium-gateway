# Security

Do not file a vulnerability as an issue or a pull request.

Report a suspected vulnerability in anything in this repository privately, by either route:

- GitHub's private vulnerability reporting ("Report a vulnerability" under the Security tab), or
- email to **info@ioka.io** with "security" in the subject.

Say what you found, where, and how to reproduce it. Do not include live credentials, customer data,
or a proof of concept run against a system you do not operate. You will get an acknowledgement
within two business days, and a fix, or a recorded decision, on the affected path before any related
release. Credit is given if you ask for it.

## Supported versions

Munarium Gateway has no release. Until the first tagged release, `main` is the only line and a fix
lands there. Once releases exist, security fixes go to the current minor release and to the previous
one for six months after its successor ships; an older release gets a fix only where the
vulnerability is in a contract it still speaks.

A finding in the design is welcome now, through the same private channel if it has security
consequences and as an ordinary issue otherwise. The threat model this component is built against
is in [README.md](README.md) and, for the platform as a whole, in the hub
([iokaio/munarium-platform](https://github.com/iokaio/munarium-platform)).

## What matters most here

As runtime behavior is implemented, these are the classes of finding taken most seriously and most
quickly:

- **Spending outside the approved ceiling**: a child task creating new identifiers to escape the root task's budget, concurrent reservations that over-admit, or a nominal monetary cap that the provider interface cannot actually enforce being presented as enforced.
- **A dispatch whose admission evidence was not durably recorded.**
- **A prompt sent to an endpoint, tenant boundary, data classification, retention posture or processing location the deployment policy did not allow**, including a hosted-model request treated as local because the retrieval index was local.
- **Raw prompt or completion content retained in public diagnostics or the ledger** by default, or a redaction that does not hold.
- **A classifier's silence or agreement manufacturing authority.** Content-safety and prompt-injection signals can trigger review or restrictions; they cannot authorize.

## What is deliberate, and is not a defect

- **Gateway distinguishes an exact monetary bound from a conservative one and from a token-only limit**, and documents which applies. Unknown pricing produces a conservative reservation reconciled at settlement, not a precise-looking figure.
- **Cancellation and timeout account for billable work the provider may still have completed.** A cancelled invocation with a later settlement correction is the design.
- **Provider names in the architecture are targets, not certifications.** One hosted and one local route are qualified first; Azure-hosted models, Bedrock, Vertex AI, NVIDIA endpoints, vLLM and Ollama follow the same adapter contract when their evidence exists.

When a local development profile exists, its test identity provider, test broker, disposable target
and generated sample credentials are development conveniences confined to that profile. They are
not vulnerabilities in themselves. A path by which they reach a production deployment unnoticed is.

## Findings that cross components

A contract ambiguity that lets two components disagree about authority, a canonicalization
difference between clients, or a gap between what a release advertises and what its evidence
supports is still a security finding. Report it here, or to any other Munarium repository, through
the same private channel; it is routed to the hub and the affected repositories together. Do not
open a public issue for it in the hub.

## Secrets

If you have committed a token or key, treat it as compromised: rotate it first, then report it.
