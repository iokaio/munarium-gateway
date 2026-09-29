// SPDX-License-Identifier: Apache-2.0
//! Durably reserve capacity under a shared root-task budget.
//!
//! Child tasks share the approved ceiling. Unknown prices, incomplete usage and cancellation require conservative accounting, not invented precision.
//!
//! Proposed local interface only. No implementation or wire format is provided.

/// Proposed boundary for: durably reserve capacity under a shared root-task budget.
///
/// Implementations and concrete types await the component design and hub contracts.
/// This declaration does not enforce authentication, authorization, or durability.
pub trait BudgetLedger {
    /// Input whose concrete shape and validation rules are still to be specified.
    type ReservationRequest;
    /// Output whose concrete shape and evidence requirements are still to be specified.
    type Reservation;
    /// Failure reported without manufacturing a successful or authorized result.
    type Error;

    /// Durably reserve capacity under a shared root-task budget.
    ///
    /// # Errors
    ///
    /// Implementations must report failed validation or unavailable required dependencies.
    fn reserve(
        &mut self,
        input: &Self::ReservationRequest,
    ) -> Result<Self::Reservation, Self::Error>;
}
