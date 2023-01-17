// Copyright © 2022-2023 The async_main crate contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use alloc::{rc::Rc, vec::Vec};
use core::{cell::Cell, fmt, future::Future, marker::PhantomData};

use crate::Spawn;

/// Handle to a `!Send` asynchronous task spawner.
#[derive(Clone, Default)]
pub struct LocalSpawner(
    Rc<Cell<Vec<async_std::task::JoinHandle<()>>>>,
    PhantomData<*mut ()>,
);

impl fmt::Debug for LocalSpawner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("LocalSpawner")
            .field(&"..")
            .field(&self.1)
            .finish()
    }
}

impl LocalSpawner {
    /// Block on a future and run the task pool until all tasks have completed.
    pub fn block_on(self, f: impl Future<Output = ()> + 'static) {
        async_std::task::block_on(async move {
            self.spawn_local(f);
            loop {
                let handles = self.0.take();
                if handles.is_empty() {
                    break;
                }
                for join in handles {
                    join.await;
                }
            }
        });
    }
}

impl Spawn for LocalSpawner {
    #[inline(always)]
    fn spawn_local(&self, f: impl Future<Output = ()> + 'static) {
        let mut joins = self.0.take();

        joins.push(async_std::task::Builder::new().local(f).unwrap());

        self.0.set(joins);
    }

    #[inline(always)]
    fn spawn(&self, f: impl Future<Output = ()> + Send + 'static) {
        let mut joins = self.0.take();

        joins.push(async_std::task::Builder::new().spawn(f).unwrap());

        self.0.set(joins);
    }
}
