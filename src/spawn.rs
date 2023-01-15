// Copyright © 2022-2023 The async_main crate contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use alloc::{boxed::Box, sync::Arc};
use core::{fmt::Debug, future::Future};

/// Implementation for spawning tasks on an executor.
///
/// This trait can be used to create a [`Spawner`].
pub trait Spawn: Debug {
    /// Spawn a [`Future`] without the [`Send`] requirement.
    ///
    /// This forces the executor to always run the task on the same thread that
    /// this method is called on.
    fn spawn_local(&self, f: Box<dyn Future<Output = ()> + 'static>);

    /// Spawn a [`Future`] that is [`Send`].
    ///
    /// This allows the executor to run the task on whatever thread it
    /// determines is most efficient.
    fn spawn(&self, f: Box<dyn Future<Output = ()> + Send + 'static>) {
        self.spawn_local(f)
    }
}

/// Handle to an asynchronous task spawner that can only spawn tasks on the
/// current thread.
#[derive(Debug)]
pub struct LocalSpawner(Arc<dyn Spawn>);

impl LocalSpawner {
    /// Create a new spawner that can only spawn tasks on the current thread.
    pub fn new<S: Spawn + 'static>(spawn: S) -> Self {
        Self(Arc::new(spawn))
    }
}

/// Handle to an asynchronous task spawner that can spawn tasks from any thread.
#[derive(Debug)]
pub struct Spawner(Arc<dyn Spawn + Send + Sync>);

impl Spawner {
    /// Create a new spawner that can spawn tasks from any thread.
    pub fn new<S: Spawn + Send + Sync + 'static>(spawn: S) -> Self {
        Self(Arc::new(spawn))
    }
}
