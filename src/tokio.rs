// Copyright © 2022-2023 The async_main crate contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use alloc::{rc::Rc, vec::Vec};
use core::{cell::Cell, fmt, future::Future};

use crate::Spawn;

/// Handle to a `!Send` asynchronous task spawner.
#[derive(Clone)]
pub struct LocalSpawner(
    Rc<tokio::runtime::Runtime>,
    Rc<tokio::task::LocalSet>,
    Rc<Cell<Vec<tokio::task::JoinHandle<()>>>>,
);

impl fmt::Debug for LocalSpawner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("LocalSpawner")
            .field(&self.0)
            .field(&self.1)
            .field(&"..")
            .finish()
    }
}

impl Default for LocalSpawner {
    fn default() -> Self {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let runtime = Rc::new(runtime);
        let local_set = Rc::new(tokio::task::LocalSet::new());

        Self(runtime, local_set, Default::default())
    }
}

impl LocalSpawner {
    /// Block on a future and run the task pool until all tasks have completed.
    pub fn block_on(self, f: impl Future<Output = ()> + 'static) {
        self.1.clone().block_on(&self.0.clone(), async move {
            self.spawn_local(f);
            loop {
                let handles = self.2.take();
                if handles.is_empty() {
                    break;
                }
                for join in handles {
                    join.await.unwrap();
                }
            }
        });
    }
}

impl Spawn for LocalSpawner {
    #[inline(always)]
    fn spawn_local(&self, f: impl Future<Output = ()> + 'static) {
        let mut joins = self.2.take();

        joins.push(self.1.spawn_local(f));

        self.2.set(joins);
    }
}
