use std::prelude::v1::*;

use std::any::TypeId;
use std::hash::{BuildHasherDefault, Hasher};
use std::collections::HashMap;

use crate::task;

/// A macro to create a `static` of type `LocalKey`
///
/// This macro is intentionally similar to the `thread_local!`, and creates a
/// `static` which has a `get_mut` method to access the data on a task.
///
/// The data associated with each task local is per-task, so different tasks
/// will contain different values.
#[macro_export]
macro_rules! task_local {
    (static $NAME:ident: $t:ty = $e:expr) => (
        static $NAME: $crate::task::LocalKey<$t> = {
            fn __init() -> $t { $e }
            fn __key() -> ::std::any::TypeId {
                struct __A;
                ::std::any::TypeId::of::<__A>()
            }
            $crate::task::LocalKey {
                __init: __init,
                __key: __key,
            }
        };
    )
}

pub struct LocalMap(HashMap<TypeId, Box<Opaque>, BuildHasherDefault<IdHasher>>);

pub fn local_map() -> LocalMap {
    LocalMap(HashMap::default())
}

pub trait Opaque: Send {}
impl<T: Send> Opaque for T {}

/// A key for task-local data stored in a future's task.
///
/// This type is generated by the `task_local!` macro and performs very
/// similarly to the `thread_local!` macro and `std::thread::LocalKey` types.
/// Data associated with a `LocalKey<T>` is stored inside of a future's task,
/// and the data is destroyed when the future is completed and the task is
/// destroyed.
///
/// Task-local data can migrate between threads and hence requires a `Send`
/// bound. Additionally, task-local data also requires the `'static` bound to
/// ensure it lives long enough. When a key is accessed for the first time the
/// task's data is initialized with the provided initialization expression to
/// the macro.
#[derive(Debug)]
pub struct LocalKey<T> {
    // "private" fields which have to be public to get around macro hygiene, not
    // included in the stability story for this type. Can change at any time.
    #[doc(hidden)]
    pub __key: fn() -> TypeId,
    #[doc(hidden)]
    pub __init: fn() -> T,
}

pub struct IdHasher {
    id: u64,
}

impl Default for IdHasher {
    fn default() -> IdHasher {
        IdHasher { id: 0 }
    }
}

impl Hasher for IdHasher {
    fn write(&mut self, _bytes: &[u8]) {
        // TODO: need to do something sensible
        panic!("can only hash u64");
    }

    fn write_u64(&mut self, u: u64) {
        self.id = u;
    }

    fn finish(&self) -> u64 {
        self.id
    }
}

impl<T: Send + 'static> LocalKey<T> {
    /// Access this task-local key.
    ///
    /// This function will access this task-local key to retrieve the data
    /// associated with the current task and this key. If this is the first time
    /// this key has been accessed on this task, then the key will be
    /// initialized with the initialization expression provided at the time the
    /// `task_local!` macro was called.
    pub fn get_mut<'a>(&'static self, cx: &'a mut task::Context) -> &'a mut T {
        let key = (self.__key)();
        let data = &mut cx.map.inner.0;
        let entry: &mut Box<Opaque> = data.entry(key).or_insert_with(|| {
            Box::new((self.__init)())
        });
        unsafe { &mut *(&mut **entry as *mut Opaque as *mut T) }
    }
}
