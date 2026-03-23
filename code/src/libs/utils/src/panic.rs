use nvim_oxi::{
    Function, Result, api,
    lua::{self, Poppable, Pushable},
};

use std::panic::{self, AssertUnwindSafe};

/// The underlying panic-catching logic. Exposed so the macro can see it.
#[doc(hidden)]
pub fn safe_wrap_impl<F, A, R>(fallback: R, f: F) -> impl Fn(A) -> Result<R>
where
    F: Fn(A) -> Result<R> + std::panic::UnwindSafe,
    R: Clone,
{
    move |args| {
        let result = panic::catch_unwind(AssertUnwindSafe(|| f(args)));
        match result {
            Ok(Ok(val)) => Ok(val),
            Ok(Err(_api_err)) => Ok(fallback.clone()),
            Err(_panic_payload) => Ok(fallback.clone()),
        }
    }
}

/// Wraps any closure returning `nvim_oxi::Result<T>` to catch panics and errors.
///
/// # Usage
/// - `safe_wrap!(my_closure)` uses `Default::default()`.
/// - `safe_wrap!(custom_fallback, my_closure)` uses the custom fallback.
#[macro_export]
macro_rules! safe_wrap {
    // Single argument: Uses Default
    ($f:expr) => {
        // NOTE: Adjust `$crate::wl_utils::` if your module is named differently!
        $crate::safe_wrap_impl(Default::default(), $f)
    };

    // Two arguments: Fallback first
    ($fallback:expr, $f:expr) => {
        $crate::wl_utils::safe_wrap_impl($fallback, $f)
    };
}

pub trait SafeFunctionExt<A, R> {
    fn from_safe_fn<F>(f: F) -> Self
    where
        F: Fn(A) -> Result<R> + std::panic::UnwindSafe + 'static,
        R: Default + Clone;

    fn from_safe_fn_with_fallback<F>(fallback: R, f: F) -> Self
    where
        F: Fn(A) -> Result<R> + std::panic::UnwindSafe + 'static,
        R: Clone;
}

impl<A, R> SafeFunctionExt<A, R> for Function<A, R>
where
    A: Poppable + 'static,
    R: Pushable + 'static,
{
    fn from_safe_fn<F>(f: F) -> Self
    where
        F: Fn(A) -> Result<R> + std::panic::UnwindSafe + 'static,
        R: Default + Clone,
    {
        Function::from_fn(safe_wrap_impl(R::default(), f))
    }

    fn from_safe_fn_with_fallback<F>(fallback: R, f: F) -> Self
    where
        F: Fn(A) -> Result<R> + std::panic::UnwindSafe + 'static,
        R: Clone,
    {
        Function::from_fn(safe_wrap_impl(fallback, f))
    }
}

// FOR TESTING

pub fn panic_test() -> nvim_oxi::Result<()> {
    // Registration for testing our panic handler
    let test_opts = api::opts::CreateCommandOpts::builder().desc("Panic Test").build();
    api::create_user_command(
        "PanicTest",
        safe_wrap!(|_args| {
            panic!("MANUAL TEST PANIC!");
            #[allow(unreachable_code)]
            Ok::<(), nvim_oxi::Error>(())
        }),
        &test_opts,
    )?;

    let test_err_opts = api::opts::CreateCommandOpts::builder().desc("Err Test").build();
    api::create_user_command(
        "TestErr",
        |_args| Err::<(), nvim_oxi::Error>(nvim_oxi::Error::from(lua::Error::RuntimeError("Manual Error".into()))),
        &test_err_opts,
    )?;

    Ok(())
}
