mod pool;
mod manager;

pub use manager::{MemoryManager, MemoryStats, MemorySettings, MemoryHint};

pub use pool::{ObjectPool, PoolConfig, PooledObject, PoolStats};

/// Default configuration for object pools
pub fn default_memory_settings() -> MemorySettings {
    MemorySettings {
        enable_pooling: true,
        pool_config: default_pool_config(),
        gc_threshold: 1024 * 1024 * 100, // 100MB
        pre_allocate_tables: true,
        track_allocations: true,
    }
}

pub fn default_pool_config() -> PoolConfig {
    let mut max_pool_sizes = std::collections::HashMap::new();
    max_pool_sizes.insert("table".to_string(), 100);
    max_pool_sizes.insert("array".to_string(), 50);
    max_pool_sizes.insert("function".to_string(), 20);

    PoolConfig {
        max_pool_sizes,
        dynamic_growth: true,
        growth_factor: 1.5,
        max_total_memory: 1024 * 1024 * 10, // 10MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_pool() {
        let config = default_pool_config();
        let mut pool = ObjectPool::new(config);

        // Acquire objects
        let obj1 = pool.acquire("table", 100);
        let obj2 = pool.acquire("table", 100);
        assert_eq!(pool.get_stats().total_allocations, 2);

        // Release and reuse
        pool.release(obj1);
        let obj3 = pool.acquire("table", 100);
        assert_eq!(pool.get_stats().total_reuses, 1);

        // Check memory tracking
        assert_eq!(pool.get_stats().current_memory, 200); // obj2 + obj3
        assert_eq!(pool.get_stats().peak_memory, 200);
    }

    #[test]
    fn test_pool_optimization() {
        let config = default_pool_config();
        let mut pool = ObjectPool::new(config);

        // Fill pool
        for _ in 0..20 {
            pool.acquire("table", 100);
        }

        // Release all
        for _ in 0..20 {
            pool.release(PooledObject {
                type_name: "table".to_string(),
                size: 100,
                reuse_count: 0,
            });
        }

        // Optimize pools
        pool.optimize_pools();
        assert!(pool.get_stats().current_memory < 2000);
    }
}
