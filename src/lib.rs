//! Type safe validation of arbitrary data
//!
//! Provides the `Valid<T>` struct which wraps some data, after verifiying that it meets some
//! criteria:
//! ```
//! # use validity::*;
//! #[derive(Debug)]
//! struct PhoneNumber(String);
//!
//! enum InvalidPhoneNumber {
//!   NonDigit,
//!   WrongLength,
//! }
//!
//! impl Validate for PhoneNumber {
//!   type Error = InvalidPhoneNumber;
//!
//!   fn is_valid(&self) -> Result<(), Self::Error> {
//!     if self.0.len() == 11 {
//!         return Err(InvalidPhoneNumber::WrongLength);
//!     }
//!
//!     if self.0.chars().any(|c| !c.is_digit(10)) {
//!         return Err(InvalidPhoneNumber::NonDigit);
//!     }
//!
//!     Ok(())
//!   }
//! }
//!
//! fn main() {
//!   let number = PhoneNumber("01234567890".to_string());
//!   if let Ok(number) = number.validate() {
//!     handle_phone_number(number);
//!   } else {
//!     println!("error!");
//!   }
//! }
//!
//! fn handle_phone_number(number: Valid<PhoneNumber>) {
//!   println!("This is a definitely valid phone number: {:?}", number.into_inner());
//! }
//! ```

use std::ops::Deref;

/// A thin wrapper around a value that guarantees that it is "valid"
///
/// A `Valid<T>` can only be constructed by calling [`Validate::validate`] and then handling the
/// possible error
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Valid<T>(T);

impl<T> Valid<T> {
    /// Consume self and return the inner value
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Deref for Valid<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A trait which defines what it means for a type to be "valid"
///
/// Because validity is defined by a trait, each type has a single definition of "valid", so
/// newtype wrappers are recommended to give additional meaning to each type.
///
/// For example:
/// ```
/// # use validity::*;
/// #[derive(Debug)]
/// struct PhoneNumber(String);
///
/// enum InvalidPhoneNumber {
///   NonDigit,
///   WrongLength,
/// }
///
/// impl Validate for PhoneNumber {
///   type Error = InvalidPhoneNumber;
///
///   fn is_valid(&self) -> Result<(), Self::Error> {
///     if self.0.len() == 11 {
///         return Err(InvalidPhoneNumber::WrongLength);
///     }
///
///     if self.0.chars().any(|c| !c.is_digit(10)) {
///         return Err(InvalidPhoneNumber::NonDigit);
///     }
///
///     Ok(())
///   }
/// }
///
/// fn main() {
///   let number = PhoneNumber("01234567890".to_string());
///   if let Ok(number) = number.validate() {
///     handle_phone_number(number);
///   } else {
///     println!("error!");
///   }
/// }
///
/// fn handle_phone_number(number: Valid<PhoneNumber>) {
///   println!("This is a definitely valid phone number: {:?}", number.into_inner());
/// }
/// ```
pub trait Validate {
    /// The error returned by validation operations
    type Error;

    /// Perform the validation on this object
    ///
    /// Valid data should return `Ok(())`, and invalid data should return `Err(Self::Error)` which
    /// indicates the reason why validation failed
    fn is_valid(&self) -> Result<(), Self::Error>;

    /// Validate this object, and if successful return a `Valid<Self>` which acts as a "proof of
    /// validity"
    ///
    /// If validation fails, the error is propagated upwards
    fn validate(self) -> Result<Valid<Self>, Self::Error>
    where
        Self: Sized,
    {
        match self.is_valid() {
            Ok(()) => Ok(Valid(self)),
            Err(e) => Err(e),
        }
    }
}

