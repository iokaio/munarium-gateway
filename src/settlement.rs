// SPDX-License-Identifier: Apache-2.0
//! Record observed usage and explicit accounting corrections.
//!
//! Retain price assumptions and provider metadata; timeout or cancellation does not prove that no billable work occurred.
//!
//! Proposed local interface only. No implementation or wire format is provided.

/// Proposed boundary for: record observed usage and explicit accounting corrections.
///
/// Implementations and concrete types await the component design and hub contracts.
/// This declaration does not enforce authentication, authorization, or durability.
pub trait UsageSettlement {
    /// Input whose concrete shape and validation rules are still to be specified.
    type UsageEvidence;
    /// Output whose concrete shape and evidence requirements are still to be specified.
    type SettlementRecord;
    /// Failure reported without manufacturing a successful or authorized result.
    type Error;

    /// Record observed usage and explicit accounting corrections.
    ///
    /// # Errors
    ///
    /// Implementations must report failed validation or unavailable required dependencies.
    fn settle(
        &mut self,
        input: &Self::UsageEvidence,
    ) -> Result<Self::SettlementRecord, Self::Error>;
}
