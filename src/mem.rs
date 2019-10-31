pub fn leak<T>(x: T) -> &'static mut T {
    Box::leak(Box::new(x))
}

pub unsafe fn reclaim<T>(x: *mut T) -> Box<T> {
    Box::from_raw(x)
}

pub unsafe fn free<T>(x: *mut T) {
    reclaim(x);
}