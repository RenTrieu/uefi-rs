// SPDX-License-Identifier: MIT OR Apache-2.0

//! Facilities for dealing with UEFI operation results.

use core::fmt::Debug;

/// The error type that we use, essentially a status code + optional additional data
mod error;
pub use error::Error;

/// Definition of UEFI's standard status codes
mod status;
pub use status::{Status, StatusExt};

/// Return type of most UEFI functions. Both success and error payloads are optional.
///
/// Almost all UEFI operations provide a status code as an output which
/// indicates either success, a warning, or an error. This type alias maps
/// [`Status::SUCCESS`] to the `Ok` variant (with optional `Output` data), and
/// maps both warning and error statuses to the `Err` variant of type [`Error`],
/// which may carry optional inner `ErrData`.
///
/// Warnings are treated as errors by default because they generally indicate
/// an abnormal situation.
///
/// Some convenience methods are provided by the [`ResultExt`] trait.
pub type Result<Output = (), ErrData = ()> = core::result::Result<Output, Error<ErrData>>;

/// Extension trait which provides some convenience methods for [`Result`].
pub trait ResultExt<Output, ErrData: Debug> {
    /// Extract the UEFI status from this result
    fn status(&self) -> Status;

    /// Transform the ErrData value to ()
    fn discard_errdata(self) -> Result<Output>;

    /// Calls `op` if the result contains a warning, otherwise returns
    /// the result unchanged.
    ///
    /// By default warning statuses are treated as errors (i.e. stored in the
    /// `Err` variant) because they generally indicate an abnormal
    /// situation. In rare cases though it may be helpful to handle a
    /// warning. This method is similar to [`Result::or_else`], except that
    /// `op` is called only when the status is a warning.
    ///
    /// # Example
    ///
    /// ```
    /// use uefi::{Result, ResultExt, Status};
    ///
    /// # use uefi::StatusExt;
    /// # fn x() -> uefi::Result {
    /// # let some_result = Status::WARN_RESET_REQUIRED.to_result();
    /// // Treat a specific warning as success, propagate others as errors.
    /// some_result.handle_warning(|err| {
    ///     if err.status() == Status::WARN_RESET_REQUIRED {
    ///         Ok(())
    ///     } else {
    ///         Err(err)
    ///     }
    /// })?;
    /// # Status::SUCCESS.to_result()
    /// # }
    /// ```
    fn handle_warning<O>(self, op: O) -> Result<Output, ErrData>
    where
        O: FnOnce(Error<ErrData>) -> Result<Output, ErrData>;
}

impl<Output, ErrData: Debug> ResultExt<Output, ErrData> for Result<Output, ErrData> {
    fn status(&self) -> Status {
        match self {
            Ok(_) => Status::SUCCESS,
            Err(e) => e.status(),
        }
    }

    fn discard_errdata(self) -> Result<Output> {
        match self {
            Ok(o) => Ok(o),
            Err(e) => Err(e.status().into()),
        }
    }

    fn handle_warning<O>(self, op: O) -> Self
    where
        O: FnOnce(Error<ErrData>) -> Self,
    {
        match self {
            Ok(output) => Ok(output),
            Err(err) => {
                if err.status().is_warning() {
                    op(err)
                } else {
                    Err(err)
                }
            }
        }
    }
}
