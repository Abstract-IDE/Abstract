use std::future::Future;

use once_cell::sync::Lazy;
use tokio::runtime::Runtime;

pub static TOKIO: Lazy<Runtime> = Lazy::new(|| {
    // Builder::new_multi_thread()
    //     // .worker_threads(2) // keep this small
    //     // .enable_time()
    //     // .enable_io()
    //     .build()
    //     .expect("Failed to build Tokio runtime");
    Runtime::new().expect("tokio runtime")
});

pub fn spawn<F>(future: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    TOKIO.spawn(future);
}
