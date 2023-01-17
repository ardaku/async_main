// Copyright © 2022-2023 The async_main crate contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use alloc::rc::Rc;
use core::{cell::Cell, fmt, future::Future};

use futures::task::{LocalSpawnExt, SpawnExt};

type PoolHandle = Rc<Cell<Option<futures::executor::LocalPool>>>;

/// Handle to a `!Send` asynchronous task spawner.
#[derive(Clone)]
pub struct LocalSpawner(PoolHandle, futures::executor::LocalSpawner);

impl fmt::Debug for LocalSpawner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("LocalSpawner")
            .field(&"..")
            .field(&self.1)
            .finish()
    }
}

impl Default for LocalSpawner {
    fn default() -> Self {
        let local_pool = futures::executor::LocalPool::new();
        let local_spawner = local_pool.spawner();

        Self(Rc::new(Cell::new(Some(local_pool))), local_spawner)
    }
}

impl LocalSpawner {
    /// Block on a future and run the task pool until all tasks have completed.
    pub fn block_on(self, f: impl Future<Output = ()> + 'static) {
        self.1.spawn_local(f).unwrap();
        self.0.take().unwrap().run();
    }
}

impl super::Spawn for LocalSpawner {
    #[inline(always)]
    fn spawn_local(&self, f: impl Future<Output = ()> + 'static) {
        self.1.spawn_local(f).unwrap();
    }

    #[inline(always)]
    fn spawn(&self, f: impl Future<Output = ()> + Send + 'static) {
        self.1.spawn(f).unwrap();
    }
}
