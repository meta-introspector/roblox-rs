use std::collections::HashMap;
use super::pool::{ObjectPool, PoolConfig, PooledObject};

/// Memory manager for optimizing Luau memory usage
pub struct MemoryManager {
    // Object pools for different types
    pools: HashMap<String, ObjectPool>,
    // Memory usage statistics
    stats: MemoryStats,
    // Memory optimization settings
    settings: MemorySettings,
}

#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub total_allocated: usize,
    pub total_freed: usize,
    pub peak_usage: usize,
    pub current_usage: usize,
    pub pool_hits: usize,
    pub pool_misses: usize,
}

#[derive(Debug, Clone)]
pub struct MemorySettings {
    pub enable_pooling: bool,
    pub pool_config: PoolConfig,
    pub gc_threshold: usize,
    pub pre_allocate_tables: bool,
    pub track_allocations: bool,
}

impl MemoryManager {
    pub fn new(settings: MemorySettings) -> Self {
        Self {
            pools: HashMap::new(),
            stats: MemoryStats {
                total_allocated: 0,
                total_freed: 0,
                peak_usage: 0,
                current_usage: 0,
                pool_hits: 0,
                pool_misses: 0,
            },
            settings,
        }
    }

    /// Allocate a new object, using pool if available
    pub fn allocate(&mut self, type_name: &str, size: usize) -> PooledObject {
        self.stats.total_allocated += size;
        self.stats.current_usage += size;
        self.stats.peak_usage = self.stats.peak_usage.max(self.stats.current_usage);

        if self.settings.enable_pooling {
            let pool = self.pools.entry(type_name.to_string()).or_insert_with(|| {
                ObjectPool::new(self.settings.pool_config.clone())
            });

            let obj = pool.acquire(type_name, size);
            if obj.reuse_count > 0 {
                self.stats.pool_hits += 1;
            } else {
                self.stats.pool_misses += 1;
            }
            obj
        } else {
            PooledObject {
                type_name: type_name.to_string(),
                size,
                reuse_count: 0,
            }
        }
    }

    /// Free an object, returning it to pool if possible
    pub fn free(&mut self, obj: PooledObject) {
        self.stats.total_freed += obj.size;
        self.stats.current_usage -= obj.size;

        if self.settings.enable_pooling {
            if let Some(pool) = self.pools.get_mut(&obj.type_name) {
                pool.release(obj);
            }
        }

        // Check if we need to trigger optimization
        if self.stats.current_usage > self.settings.gc_threshold {
            self.optimize_memory();
        }
    }

    /// Optimize memory usage across all pools
    pub fn optimize_memory(&mut self) {
        for pool in self.pools.values_mut() {
            pool.optimize_pools();
        }
    }

    /// Generate Luau code for memory management
    pub fn generate_memory_code(&self) -> String {
        let mut code = String::new();
        
        // Add memory manager
        code.push_str("local MemoryManager = {\n");
        code.push_str("    stats = {\n");
        code.push_str("        total_allocated = 0,\n");
        code.push_str("        total_freed = 0,\n");
        code.push_str("        current_usage = 0,\n");
        code.push_str("        pool_hits = 0,\n");
        code.push_str("        pool_misses = 0\n");
        code.push_str("    },\n");
        code.push_str("    settings = {\n");
        code.push_str(&format!("        gc_threshold = {},\n", self.settings.gc_threshold));
        code.push_str(&format!("        pre_allocate_tables = {},\n", 
            if self.settings.pre_allocate_tables { "true" } else { "false" }));
        code.push_str("    }\n");
        code.push_str("}\n\n");

        // Add allocation tracking
        if self.settings.track_allocations {
            code.push_str("function MemoryManager:trackAllocation(size)\n");
            code.push_str("    self.stats.total_allocated = self.stats.total_allocated + size\n");
            code.push_str("    self.stats.current_usage = self.stats.current_usage + size\n");
            code.push_str("end\n\n");

            code.push_str("function MemoryManager:trackFree(size)\n");
            code.push_str("    self.stats.total_freed = self.stats.total_freed + size\n");
            code.push_str("    self.stats.current_usage = self.stats.current_usage - size\n");
            code.push_str("end\n\n");
        }

        // Add table pre-allocation helper
        if self.settings.pre_allocate_tables {
            code.push_str("function MemoryManager:preAllocateTable(size)\n");
            code.push_str("    local t = table.create(size)\n");
            code.push_str("    self:trackAllocation(size * 8) -- Approximate size\n");
            code.push_str("    return t\n");
            code.push_str("end\n\n");
        }

        // Add memory optimization
        code.push_str("function MemoryManager:checkMemory()\n");
        code.push_str("    if self.stats.current_usage > self.settings.gc_threshold then\n");
        code.push_str("        -- Trigger Luau garbage collection\n");
        code.push_str("        collectgarbage()\n");
        code.push_str("        -- Update stats\n");
        code.push_str("        self.stats.current_usage = collectgarbage('count') * 1024\n");
        code.push_str("    end\n");
        code.push_str("end\n\n");

        code.push_str("return MemoryManager\n");
        code
    }

    /// Get current memory statistics
    pub fn get_stats(&self) -> &MemoryStats {
        &self.stats
    }

    /// Update memory settings
    pub fn update_settings(&mut self, settings: MemorySettings) {
        self.settings = settings;
        // Update pool configurations
        for pool in self.pools.values_mut() {
            pool.optimize_pools();
        }
    }

    /// Get memory optimization hints
    pub fn get_optimization_hints(&self) -> Vec<MemoryHint> {
        let mut hints = Vec::new();

        // Check pool efficiency
        for (type_name, pool) in &self.pools {
            let stats = pool.get_stats();
            let efficiency = stats.total_reuses as f64 / 
                           (stats.total_allocations + stats.total_reuses) as f64;

            if efficiency < 0.5 {
                hints.push(MemoryHint::ReducePoolSize(type_name.clone()));
            } else if efficiency > 0.9 {
                hints.push(MemoryHint::IncreasePoolSize(type_name.clone()));
            }
        }

        // Check memory pressure
        if self.stats.current_usage > self.settings.gc_threshold {
            hints.push(MemoryHint::HighMemoryPressure);
        }

        hints
    }
}

#[derive(Debug)]
pub enum MemoryHint {
    ReducePoolSize(String),
    IncreasePoolSize(String),
    HighMemoryPressure,
}
