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
//!   type Context<'a> = ();
//!   type Error = InvalidPhoneNumber;
//!
//!   fn is_valid(&self, _ctx: Self::Context<'_>) -> Result<(), Self::Error> {
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
//!
//! Some validation requires access to some context. For example, you may want to validate that an
//! email address exists in your database. For that, you can pass this context via the `Context`
//! associated type.
//!
//! When validating, you can call `foo.validate_with(context)`:
//! ```
//! # use validity::*;
//! # struct Database {}
//! # impl Database {
//! #   fn check_phone_exists(&self, _: &PhoneNumber) -> bool { true }
//! #   fn new() -> Self { Self {} }
//! # }
//! #[derive(Debug)]
//! struct PhoneNumber(String);
//!
//! enum InvalidPhoneNumber {
//!   NonDigit,
//!   WrongLength,
//!   NotInDatabase,
//! }
//!
//! impl Validate for PhoneNumber {
//!   type Context<'a> = Database;
//!   type Error = InvalidPhoneNumber;
//!
//!   fn is_valid(&self, db: Self::Context<'_>) -> Result<(), Self::Error> {
//!     if self.0.len() == 11 {
//!         return Err(InvalidPhoneNumber::WrongLength);
//!     }
//!
//!     if self.0.chars().any(|c| !c.is_digit(10)) {
//!         return Err(InvalidPhoneNumber::NonDigit);
//!     }
//!
//!     if !db.check_phone_exists(self) {
//!         return Err(InvalidPhoneNumber::NotInDatabase);
//!     }
//!
//!     Ok(())
//!   }
//! }
//! ```
//! You can then call this with:
//! ```rust
//! # use validity::*;
//! # struct Database {}
//! # impl Database {
//! #   fn check_phone_exists(&self, _: &PhoneNumber) -> bool { true }
//! #   fn new() -> Self { Self {} }
//! # }
//! # #[derive(Debug)]
//! # struct PhoneNumber(String);
//! # enum InvalidPhoneNumber {
//! #   NonDigit,
//! #   WrongLength,
//! #   NotInDatabase,
//! # }
//! # impl Validate for PhoneNumber {
//! #   type Context<'a> = Database;
//! #   type Error = InvalidPhoneNumber;
//! #   fn is_valid(&self, db: Self::Context<'_>) -> Result<(), Self::Error> {
//! #     if self.0.len() == 11 {
//! #         return Err(InvalidPhoneNumber::WrongLength);
//! #     }
//! #     if self.0.chars().any(|c| !c.is_digit(10)) {
//! #         return Err(InvalidPhoneNumber::NonDigit);
//! #     }
//! #     if !db.check_phone_exists(self) {
//! #         return Err(InvalidPhoneNumber::NotInDatabase);
//! #     }
//! #     Ok(())
//! #   }
//! # }
//! let db = Database::new();
//! let phone = PhoneNumber("01234567890".to_string());
//! phone.validate_with(db);
//! ```

use core::ops::Deref;

/// A thin wrapper around a value that guarantees that it is "valid"
///
/// A `Valid<T>` can only be constructed by calling [`Validate::validate`] and then handling the
/// possible error
///
/// Note, `Valid<T>` is not `repr(transparent)`, so using `transmute` to forcibly convert is
/// undefined behaviour.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Valid<T>(T);

impl<T> Valid<T> {
    /// Consume self and return the inner value
    pub fn into_inner(self) -> T {
        self.0
    }

    /// Create a `Valid<T>` without validating
    ///
    /// This is only available with the `test-mock` feature enabled.
    ///
    /// It goes without saying that this function invalidates all compile-time guarantees. It's
    /// provided as an "escape hatch", intended for testing. While 
    #[cfg(feature = "test-mock")]
    pub fn danger_new_unvalidated(t: T) -> Self {
        Self(t)
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
///   type Context<'a> = ();
///   type Error = InvalidPhoneNumber;
///
///   fn is_valid(&self, _ctx: Self::Context<'_>) -> Result<(), Self::Error> {
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
    /// Context required for validation
    type Context<'a>;

    /// The error returned by validation operations
    type Error;

    /// Perform the validation on this object
    ///
    /// Valid data should return `Ok(())`, and invalid data should return `Err(Self::Error)` which
    /// indicates the reason why validation failed
    fn is_valid(&self, ctx: Self::Context<'_>) -> Result<(), Self::Error>;

    /// Validate with the given context
    fn validate_with(self, ctx: Self::Context<'_>) -> Result<Valid<Self>, Self::Error>
    where
        Self: Sized,
    {
        match self.is_valid(ctx) {
            Ok(()) => Ok(Valid(self)),
            Err(e) => Err(e),
        }
    }

    /// Validate this object, and if successful return a `Valid<Self>` which acts as a "proof of
    /// validity"
    ///
    /// If validation fails, the error is propagated upwards
    fn validate(self) -> Result<Valid<Self>, Self::Error>
    where
        Self: for<'a> Validate<Context<'a> = ()>,
        Self: Sized,
    {
        self.validate_with(())
    }
}
