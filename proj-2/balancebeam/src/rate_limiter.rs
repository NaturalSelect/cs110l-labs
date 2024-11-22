struct RateLimiterInner {
    token: u32,
    last_acquired: std::time::Instant,
}

pub struct RateLimiter {
    rate: u32,
    inner: tokio::sync::RwLock<RateLimiterInner>,
}

impl RateLimiter {
    pub async fn acquire(&self) -> bool {
        let mut inner = self.inner.write().await;
        let now = std::time::Instant::now();
        let pass_sec = now.duration_since(inner.last_acquired).as_secs() as u32;
        let mut new_token = inner.token + pass_sec * self.rate / 60;
        if new_token > self.rate {
            new_token = self.rate;
        }
        if new_token > 0 {
            inner.token = new_token - 1;
            inner.last_acquired = std::time::Instant::now();
            return true;
        }
        return false;
    }

    pub fn new(rate: u32) -> RateLimiter {
        return RateLimiter {
            rate: rate,
            inner: tokio::sync::RwLock::new(RateLimiterInner {
                token: rate,
                last_acquired: std::time::Instant::now(),
            }),
        };
    }

    pub fn rate(&self) -> u32 {
        return self.rate;
    }
}
