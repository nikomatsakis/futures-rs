//! Tasks used to drive a future computation
//!
//! It's intended over time a particular operation (such as servicing an HTTP
//! request) will involve many futures. This entire operation, however, can be
//! thought of as one unit, as the entire result is essentially just moving
//! through one large state machine.
//!
//! A "task" is the unit of abstraction for what is driving this state machine
//! and tree of futures forward. A task is used to poll futures and schedule
//! futures with, and has utilities for sharing data between tasks and handles
//! for notifying when a future is ready. Each task also has its own set of
//! task-local data generated by `task_local!`.
//!
//! Note that libraries typically should not manage tasks themselves, but rather
//! leave that to event loops and other "executors" (see the `executor` module),
//! or by using the `wait` method to create and execute a task directly on the
//! current thread.
//!
//! ## Functions
//!
//! There is an important bare function in this module: `park`. The `park`
//! function is similar to the standard library's `thread::park` method where it
//! returns a handle to wake up a task at a later date (via an `unpark` method).

#[doc(hidden)]
#[deprecated(since = "0.1.4", note = "import through the executor module instead")]
#[cfg(feature = "with-deprecated")]
pub use task_impl::{Spawn, spawn, Unpark, Executor, Run};

pub use task_impl::{Task, LocalKey, park, with_unpark_event, UnparkEvent, EventSet};

#[doc(hidden)]
#[deprecated(since = "0.1.4", note = "import through the executor module instead")]
#[cfg(feature = "with-deprecated")]
#[allow(deprecated)]
pub use task_impl::TaskRc;
