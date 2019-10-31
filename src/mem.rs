pub fn leak<T>(x: T) -> &'static mut T {
    Box::leak(Box::new(x))
}

pub unsafe fn free<T>(x: *mut T) {
    Box::from_raw(x);
}