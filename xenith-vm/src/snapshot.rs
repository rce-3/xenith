/// A named VM snapshot. Created by [`crate::vm::Vm::save_snapshot`].
#[derive(Debug, Clone)]
pub struct Snapshot {
    pub tag: String,
}

impl Snapshot {
    #[must_use]
    pub fn new(tag: impl Into<String>) -> Self {
        Self { tag: tag.into() }
    }
}
