#[diplomat::bridge]
pub mod ffi {
    use stream_cancel::Trigger;

    /// Subscription handle for managing event streams
    #[diplomat::opaque]
    pub struct Subscription {
        pub(crate) id: u64,
        pub(crate) trigger: Trigger,
    }

    impl Subscription {
        /// Gets the subscription ID
        pub fn id(&self) -> u64 {
            self.id
        }
    }
}

