// SPDX-License-Identifier: Apache-2.0
//! Check whether a model invocation may use a route.
//!
//! Validate endpoint, tenant, classifications, retention and processing location before prompt disclosure; classifier signals do not confer authority.
//!
//! Proposed local interface only. No implementation or wire format is provided.

/// Proposed boundary for: check whether a model invocation may use a route.
///
/// Implementations and concrete types await the component design and hub contracts.
/// This declaration does not enforce authentication, authorization, or durability.
pub trait RouteAdmission {
    /// Input whose concrete shape and validation rules are still to be specified.
    type InvocationRequest;
    /// Output whose concrete shape and evidence requirements are still to be specified.
    type AdmittedRoute;
    /// Failure reported without manufacturing a successful or authorized result.
    type Error;

    /// Check whether a model invocation may use a route.
    ///
    /// # Errors
    ///
    /// Implementations must report failed validation or unavailable required dependencies.
    fn admit(&self, input: &Self::InvocationRequest) -> Result<Self::AdmittedRoute, Self::Error>;
}
