// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Traits for working with Errors.
//!
//! # The `Error` trait
//!
//! `Error` is a trait representing the basic expectations for error values,
//! i.e. values of type `E` in `Result<T, E>`. At a minimum, errors must provide
//! a description, but they may optionally provide additional detail and cause
//! chain information:
//!
//! ```
//! pub trait Error: Send + Any {
//!     fn description(&self) -> &str;
//!
//!     fn detail(&self) -> Option<String> { None }
//!     fn cause(&self) -> Option<&Error> { None }
//! }
//! ```
//!
//! The `cause` method is generally used when errors cross "abstraction
//! boundaries", i.e.  when a one module must report an error that is "caused"
//! by an error from a lower-level module. This setup makes it possible for the
//! high-level module to provide its own errors that do not commit to any
//! particular implementation, but also reveal some of its implementation for
//! debugging via `cause` chains.
//!
//! The trait inherits from `Any` to allow *downcasting*: converting from a
//! trait object to a specific concrete type when applicable.
//!
//! # The `FromError` trait
//!
//! `FromError` is a simple trait that expresses conversions between different
//! error types. To provide maximum flexibility, it does not require either of
//! the types to actually implement the `Error` trait, although this will be the
//! common case.
//!
//! The main use of this trait is in the `try!` macro, which uses it to
//! automatically convert a given error to the error specified in a function's
//! return type.

use any::{Any, AnyRefExt, AnyMutRefExt};
use mem::{transmute, transmute_copy};
use option::{Option, Some, None};
use raw::TraitObject;
use intrinsics::TypeId;
use kinds::Send;
use string::String;

/// Base functionality for all errors in Rust.
pub trait Error: Send + Any {
    /// A short description of the error; usually a static string.
    fn description(&self) -> &str;

    /// A detailed description of the error, usually including dynamic information.
    fn detail(&self) -> Option<String> { None }

    /// The lower-level cause of this error, if any.
    fn cause(&self) -> Option<&Error> { None }
}

/// A trait for types that can be converted from a given error type `E`.
pub trait FromError<E> {
    /// Perform the conversion.
    fn from_error(err: E) -> Self;
}

// Any type is convertable from itself
impl<E> FromError<E> for E {
    fn from_error(err: E) -> E {
        err
    }
}

// FIXME (#https://github.com/rust-lang/rust/pull/17669/): Add this once multidispatch lands
// impl<E: Error> FromError<E> for Box<Error> {
//     fn from_err(err: E) -> Box<Error> {
//         box err as Box<Error>
//     }
// }

// Note: the definitions below are copied from core::any, and should be unified
// as soon as possible.

impl<'a> AnyRefExt<'a> for &'a Error {
    #[inline]
    fn is<T: 'static>(self) -> bool {
        // Get TypeId of the type this function is instantiated with
        let t = TypeId::of::<T>();

        // Get TypeId of the type in the trait object
        let boxed = self.get_type_id();

        // Compare both TypeIds on equality
        t == boxed
    }

    #[inline]
    fn downcast_ref<T: 'static>(self) -> Option<&'a T> {
        if self.is::<T>() {
            unsafe {
                // Get the raw representation of the trait object
                let to: TraitObject = transmute_copy(&self);

                // Extract the data pointer
                Some(transmute(to.data))
            }
        } else {
            None
        }
    }
}

impl<'a> AnyMutRefExt<'a> for &'a mut Error {
    #[inline]
    fn downcast_mut<T: 'static>(self) -> Option<&'a mut T> {
        if self.is::<T>() {
            unsafe {
                // Get the raw representation of the trait object
                let to: TraitObject = transmute_copy(&self);

                // Extract the data pointer
                Some(transmute(to.data))
            }
        } else {
            None
        }
    }
}
