#[inline(always)]
pub fn std_now() -> std::time::Instant {
    std::time::Instant::now()
}

#[inline(always)]
pub fn tokio_now() -> tokio::time::Instant {
    tokio::time::Instant::now()
}
