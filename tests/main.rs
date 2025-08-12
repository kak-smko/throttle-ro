use cache_ro::{Cache, CacheConfig};
use std::thread::sleep;
use std::time::Duration;
use throttle_ro::ThrottlesService;

#[test]
fn test_all() {
    test_initial_can_go_is_true();
    test_hit_increments_value();
    test_can_go_blocks_after_max_attempts();
    test_remove_clears_cache();
    test_expire_returns_default_when_none_set();
    test_expire_returns_custom_when_set()
}
fn test_initial_can_go_is_true() {
    let ip = "127.0.0.1".to_string();
    let cache = Cache::new(CacheConfig {
        persistent: false,
        ..Default::default()
    })
    .unwrap();
    let mut service = ThrottlesService::new(ip, 3, Duration::from_secs(60), "test_");

    assert!(service.can_go(&cache));
    Cache::drop()
}


fn test_hit_increments_value() {
    let ip = "127.0.0.2".to_string();
    let cache = Cache::new(CacheConfig {
        persistent: false,
        ..Default::default()
    })
    .unwrap();
    let mut service = ThrottlesService::new(ip.clone(), 3, Duration::from_secs(60), "test_");

    service.hit(&cache);
    assert_eq!(cache.get::<u32>(&service.key()), Some(1));
    let mut service = ThrottlesService::new(ip, 3, Duration::from_secs(60), "test_");

    service.hit(&cache);
    assert_eq!(cache.get::<u32>(&service.key()), Some(2));
    Cache::drop()
}

fn test_can_go_blocks_after_max_attempts() {
    let ip = "127.0.0.3".to_string();
    let cache = Cache::new(CacheConfig {
        persistent: false,
        ..Default::default()
    })
    .unwrap();
    let mut service = ThrottlesService::new(ip, 3, Duration::from_secs(60), "test_");

    service.hit(&cache);
    service.hit(&cache);
    service.hit(&cache);
    assert!(!service.can_go(&cache));
    Cache::drop()
}


fn test_remove_clears_cache() {
    let ip = "127.0.0.4".to_string();
    let cache = Cache::new(CacheConfig {
        persistent: false,
        ..Default::default()
    })
    .unwrap();
    let mut service = ThrottlesService::new(ip, 3, Duration::from_secs(60), "test_");

    service.hit(&cache);
    assert_eq!(cache.get::<u32>(&service.key()), Some(1));

    service.remove(&cache);
    assert_eq!(cache.get::<i32>(&service.key()), None);
    Cache::drop()
}

fn test_expire_returns_default_when_none_set() {
    let ip = "127.0.0.5".to_string();
    let cache = Cache::new(CacheConfig {
        persistent: false,
        ..Default::default()
    })
    .unwrap();
    let mut service = ThrottlesService::new(ip, 3, Duration::from_secs(60), "test_");

    let expire = service.get_expire(&cache);
    assert_eq!(expire, Duration::from_secs(60));
    Cache::drop()
}

fn test_expire_returns_custom_when_set() {
    let ip = "127.0.0.6".to_string();
    let cache = Cache::new(CacheConfig {
        persistent: false,
        ..Default::default()
    })
    .unwrap();
    cache.clear().unwrap();
    let mut service = ThrottlesService::new(ip, 2, Duration::from_secs(1), "test_");
    service.hit(&cache);
    service.hit(&cache);
    sleep(Duration::from_secs(1));
    assert!(service.can_go(&cache));
    Cache::drop()
}
