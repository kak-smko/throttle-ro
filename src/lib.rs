//! A throttling service that limits the number of attempts (hits) from an IP address
//! within a specified time period.
//!
//! # Example: Basic Rate Limiting
//!
//! ```
//! use std::time::Duration;
//! use cache_ro::Cache;
//! use throttle_ro::ThrottlesService;
//!
//! // Create a cache instance (in-memory for this example)
//! let cache = Cache::new(cache_ro::CacheConfig {
//!     persistent: false,
//!     ..Default::default()
//! }).unwrap();
//!
//! // Create a throttling service for IP "127.0.0.1"
//! // allowing maximum 5 attempts per minute
//! let ip = "127.0.0.1".to_string();
//! let mut service = ThrottlesService::new(
//!     ip,
//!     5, // max attempts
//!     Duration::from_secs(60), // time window
//!     "api_rate_limit_" // cache key prefix
//! );
//!
//! // Check if the IP is allowed to proceed
//! if service.can_go(&cache) {
//!     // Record the attempt
//!     service.hit(&cache);
//!     println!("Request allowed");
//!     // Process the request...
//! } else {
//!     println!("Rate limit exceeded - please try again later");
//!     // Return error or wait...
//! }
//!
//! // You can also manually clear the throttle if needed
//! // service.remove(&cache);
//! ```
//!


use cache_ro::Cache;
use std::time::Duration;

/// A service for throttling attempts from an IP address.
///
/// Tracks the number of attempts (hits) from a given IP address and determines
/// whether further attempts should be allowed based on configured limits.
///
/// # Examples
///
/// ```
/// use std::time::Duration;
/// use cache_ro::Cache;
/// use throttle_ro::ThrottlesService;
///
/// let cache = Cache::new(Default::default()); // In real usage, configure properly
/// let ip = "127.0.0.1".to_string();
/// let mut service = ThrottlesService::new(
///     ip,
///     5, // max attempts
///     Duration::from_secs(60), // time window
///     "rate_limit_"
/// );
///
/// if service.can_go(&cache) {
///     service.hit(&cache);
///     // Process the request
/// } else {
///     // Reject the request - rate limit exceeded
/// }
/// ```
pub struct ThrottlesService {
    ip: String,
    max_attempts: u32,
    period: Duration,
    prefix: String,
}

impl ThrottlesService {
    /// Creates a new `ThrottlesService` instance.
    ///
    /// # Arguments
    ///
    /// * `ip` - The IP address to track
    /// * `max_attempts` - Maximum number of allowed attempts in the time period
    /// * `period` - Duration of the throttling window
    /// * `prefix` - Prefix for cache keys to avoid collisions
    pub fn new(ip: String, max_attempts: u32, period: Duration, prefix: &str) -> Self {
        Self {
            ip,
            max_attempts,
            period,
            prefix: prefix.to_string(),
        }
    }

    /// Checks whether the IP is allowed to make another attempt.
    ///
    /// Returns `true` if the current attempt count is below the maximum allowed.
    pub fn can_go(&mut self, cache: &Cache) -> bool {
        let v = self.get_value(cache).unwrap_or(0);
        v < self.max_attempts
    }

    fn get_value(&mut self, cache: &Cache) -> Option<u32> {
        cache.get::<u32>(&self.key())
    }

    /// Generates the cache key for this IP.
    pub fn key(&self) -> String {
        format!("{}{}", self.prefix, self.ip)
    }

    /// Gets the remaining duration for the current throttling window.
    ///
    /// Returns the configured period if no expiration is set in the cache.
    pub fn get_expire(&mut self, cache: &Cache) -> Duration {
        let ex = cache.expire(&self.key());
        match ex {
            None => self.period,
            Some(a) => a,
        }
    }

    /// Records an attempt (hit) from the IP.
    ///
    /// Increments the attempt count and resets the expiration time.
    pub fn hit(&mut self, cache: &Cache) {
        let key = self.key();
        let expire = self.get_expire(cache);

        match self.get_value(cache) {
            None => cache.set::<u32>(&key, 1, expire).unwrap(),
            Some(v) => cache.set::<u32>(&key, v + 1, expire).unwrap(),
        }
    }

    /// Clears the attempt count for the IP.
    pub fn remove(&self, cache: &Cache) {
        cache.remove(&self.key()).unwrap();
    }
}
