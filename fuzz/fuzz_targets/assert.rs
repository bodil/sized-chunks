use std::panic::{catch_unwind, set_hook, take_hook, AssertUnwindSafe};

pub fn assert_panic<A, F>(f: F)
where
    F: FnOnce() -> A,
{
    let old_hook = take_hook();
    set_hook(Box::new(|_| {}));
    let result = catch_unwind(AssertUnwindSafe(f));
    set_hook(old_hook);
    assert!(
        result.is_err(),
        "action that should have panicked didn't panic"
    );
}
