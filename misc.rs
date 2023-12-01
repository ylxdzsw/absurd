pub trait ExtForIgnore {
    /// satisfy `#[must_use]` but do nothing
    fn ignore(&self) {}
}

impl<T: ?Sized> ExtForIgnore for T {}

pub trait SizedExtForApply: Sized {
    fn apply(mut self, f: impl FnOnce(&mut Self)) -> Self {
        f(&mut self); self
    }
}

impl<T> SizedExtForApply for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ignore() {
        let _ = ().ignore();
    }

    #[test]
    fn apply() {
        let x = 4.apply(|x| *x += 1);
        assert_eq!(x, 5)
    }
}
