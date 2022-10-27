#[cfg(target_arch = "wasm32")]
pub fn spawn<F>(f: F)
where
    F: FnOnce() + 'static,
{
    f();
}

#[cfg(not(target_arch = "wasm32"))]
pub fn spawn<F, T>(f: F)
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static
{
    std::thread::spawn(f);
}