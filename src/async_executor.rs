// Copyright © 2022-2023 The async_main crate contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use alloc::rc::Rc;
use core::future::Future;

/// Handle to a `!Send` asynchronous task spawner.
#[derive(Clone, Debug, Default)]
pub struct LocalSpawner(Rc<async_executor::LocalExecutor<'static>>);

impl LocalSpawner {
    /// Block on a future and run the task pool until all tasks have completed.
    pub fn block_on(self, f: impl Future<Output = ()> + 'static) {
        self.0.spawn(f).detach();
        futures_lite::future::block_on(async {
            while !self.0.is_empty() {
                self.0.tick().await
            }
        });
    }
}

impl super::Spawn for LocalSpawner {
    #[inline(always)]
    fn spawn_local(&self, f: impl Future<Output = ()> + 'static) {
        self.0.spawn(f).detach();
    }

    #[inline(always)]
    fn spawn(&self, f: impl Future<Output = ()> + Send + 'static) {
        self.0.spawn(f).detach();
    }
}
