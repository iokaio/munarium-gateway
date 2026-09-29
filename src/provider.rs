// SPDX-License-Identifier: Apache-2.0
//! Dispatch a durably admitted invocation through a provider boundary.
//!
//! Extract the Server implementation lineage first. Stream fragments and ambiguous outcomes must retain invocation identity.
//!
//! Proposed local interface only. No implementation or wire format is provided.

/// Proposed boundary for: dispatch a durably admitted invocation through a provider boundary.
///
/// Implementations and concrete types await the component design and hub contracts.
/// This declaration does not enforce authentication, authorization, or durability.
pub trait ProviderAdapter {
    /// Input whose concrete shape and validation rules are still to be specified.
    type AdmittedInvocation;
    /// Output whose concrete shape and evidence requirements are still to be specified.
    type ProviderOutcome;
    /// Failure reported without manufacturing a successful or authorized result.
    type Error;

    /// Dispatch a durably admitted invocation through a provider boundary.
    ///
    /// # Errors
    ///
    /// Implementations must report failed validation or unavailable required dependencies.
    fn invoke(
        &mut self,
        input: &Self::AdmittedInvocation,
    ) -> Result<Self::ProviderOutcome, Self::Error>;
}
