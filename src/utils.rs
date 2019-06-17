use std::fmt::Debug;

/// Allow for debugging without requiring `std::fmt::Debug`
/// taken from
/// `https://www.reddit.com/r/rust/comments/6poulm/tip_print_a_t_without_requiring_t_debug/`
pub trait AsDebug {
    /// convert self to &Debug if we can or panic.
    fn as_debug(&self) -> &Debug;
}

impl<T> AsDebug for T {
    default fn as_debug(&self) -> &Debug {
        panic!("Debug not implemented for {}", unsafe {
            std::intrinsics::type_name::<T>()
        });
    }
}

impl<T: Debug> AsDebug for T {
    fn as_debug(&self) -> &Debug {
        self
    }
}
