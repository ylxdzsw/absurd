pub trait MonadExt {
    fn ignore(&self) {}

    fn apply<F: FnOnce(&Self)>(&self, f: F) -> &Self {
        f(self); self
    }

    fn apply_mut<F: FnOnce(&mut Self)>(&mut self, f: F) -> &mut Self {
        f(self); self
    }
}

pub trait SizedMonadExt: Sized {
    fn apply_owned<F: FnOnce(&mut Self)>(mut self, f: F) -> Self {
        f(&mut self); self
    }
}
