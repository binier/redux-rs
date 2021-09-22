//! # redux - A Rust implementation of Redux.
//!
//! Redux provides a clean way of managing states in an application.
//! It could be user data such as preferences or information about the state of the program.
//!
//! ## Concepts
//!
//! In Redux data is immutable. The only way to change it is to take it and create some new data by following a set of rules.
//!
//! ### State
//!
//! A state is the form of data Redux manages. Theoretically it can be anything, but for an easy explanation let's take the following example:
//! We have a simple counter application. It does nothing more than counting.
//! Our state would look the following:
//!
//! ```
//! #[derive(Default)]
//! struct State {
//!     counter: i8
//! }
//! ```
//!
//! ### Actions
//!
//! To change the state we need to dispatch actions. In Rust, they would usually be represented by an enum.
//! For the counter, we want to increment and decrement it.
//!
//! ```
//! enum Action {
//!     Increment,
//!     Decrement
//! }
//! ```
//!
//! ### Reducer
//!
//! To actually change the state (read: create a new one), we need what is called a reducer.
//! It is a simple function which takes in the current state plus the action to perform and returns a new state.
//!
//!
//! ### Store
//!
//! To put it all together, we use a store which keeps track of a state and provides an easy to use API for dispatching actions.
//! The store takes the reducer and an initial state.
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc))]

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
#[cfg(feature = "std")]
use std::vec::Vec;

mod middleware;
mod reducer;
mod store;

pub use middleware::Middleware;
pub use reducer::Reducer;
#[cfg(not(feature = "devtools"))]
pub use store::Store;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ActionId(u64);

impl ActionId {
    pub const ZERO: Self = Self(0);

    #[inline(always)]
    fn increment(&mut self) -> Self {
        self.0 += 1;
        *self
    }
}

impl From<ActionId> for u64 {
    fn from(id: ActionId) -> Self {
        id.0
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ActionWithId<Action> {
    pub id: ActionId,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub action: Action
}
