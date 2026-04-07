/// A named VM snapshot. Created by [`crate::vm::Vm::save_snapshot`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Snapshot {
    tag: String,
}

impl Snapshot {
    #[must_use]
    pub fn new(tag: impl Into<String>) -> Self {
        Self { tag: tag.into() }
    }

    /// The snapshot tag name.
    #[must_use]
    pub fn tag(&self) -> &str {
        &self.tag
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot_equality() {
        let a = Snapshot::new("v1");
        let b = Snapshot::new("v1");
        let c = Snapshot::new("v2");
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn snapshot_tag_accessor() {
        let s = Snapshot::new("my-snap");
        assert_eq!(s.tag(), "my-snap");
    }
}
