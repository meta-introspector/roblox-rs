use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};

/// Memory pool for reusing Luau tables and other objects
#[derive(Debug)]
pub struct ObjectPool {
    // Pools for different object types
    pools: HashMap<String, Pool>,
    // Statistics for pool usage
    stats: PoolStats,
    // Configuration for pool behavior
    config: PoolConfig,
}

#[derive(Debug)]
struct Pool {
    // Available objects in the pool
    available: VecDeque<PooledObject>,
    // Maximum size of this pool
    max_size: usize,
    // Number of objects currently in use
    in_use: usize,
    // Size of each object in this pool
    object_size: usize,
}

#[derive(Debug, Clone)]
pub struct PooledObject {
    // Type of the object (e.g., "table", "array")
    type_name: String,
    // Size of the object in bytes
    size: usize,
    // Number of times this object has been reused
    reuse_count: usize,
}

#[derive(Debug)]
pub struct PoolStats {
    // Total allocations made
    total_allocations: usize,
    // Number of objects reused from pools
    total_reuses: usize,
    // Peak memory usage
    peak_memory: usize,
    // Current memory usage
    current_memory: usize,
}

#[derive(Debug, Clone)]
pub struct PoolConfig {
    // Maximum size for each pool type
    pub max_pool_sizes: HashMap<String, usize>,
    // Whether to grow pools dynamically
    pub dynamic_growth: bool,
    // Growth factor when pools are full
    pub growth_factor: f32,
    // Maximum memory allowed for all pools
    pub max_total_memory: usize,
}

impl ObjectPool {
    pub fn new(config: PoolConfig) -> Self {
        Self {
            pools: HashMap::new(),
            stats: PoolStats {
                total_allocations: 0,
                total_reuses: 0,
                peak_memory: 0,
                current_memory: 0,
            },
            config,
        }
    }

    /// Get an object from the pool or create a new one
    pub fn acquire(&mut self, type_name: &str, size: usize) -> PooledObject {
        let pool = self.pools.entry(type_name.to_string()).or_insert_with(|| Pool {
            available: VecDeque::new(),
            max_size: self.config.max_pool_sizes
                .get(type_name)
                .cloned()
                .unwrap_or(10),
            in_use: 0,
            object_size: size,
        });

        // Try to get an object from the pool
        if let Some(mut obj) = pool.available.pop_front() {
            obj.reuse_count += 1;
            pool.in_use += 1;
            self.stats.total_reuses += 1;
            obj
        } else {
            // Create new object if pool is not full
            if pool.in_use < pool.max_size || self.config.dynamic_growth {
                pool.in_use += 1;
                self.stats.total_allocations += 1;
                self.stats.current_memory += size;
                self.stats.peak_memory = self.stats.peak_memory.max(self.stats.current_memory);

                // Grow pool if needed
                if self.config.dynamic_growth && pool.in_use >= pool.max_size {
                    pool.max_size = ((pool.max_size as f32) * self.config.growth_factor) as usize;
                }

                PooledObject {
                    type_name: type_name.to_string(),
                    size,
                    reuse_count: 0,
                }
            } else {
                // Pool is full, return a new object without tracking
                self.stats.total_allocations += 1;
                PooledObject {
                    type_name: type_name.to_string(),
                    size,
                    reuse_count: 0,
                }
            }
        }
    }

    /// Return an object to the pool
    pub fn release(&mut self, obj: PooledObject) {
        if let Some(pool) = self.pools.get_mut(&obj.type_name) {
            if pool.available.len() < pool.max_size {
                pool.available.push_back(obj);
                pool.in_use -= 1;
                self.stats.current_memory -= obj.size;
            }
        }
    }

    /// Get current pool statistics
    pub fn get_stats(&self) -> &PoolStats {
        &self.stats
    }

    /// Optimize pool sizes based on usage patterns
    pub fn optimize_pools(&mut self) {
        for (type_name, pool) in &mut self.pools {
            // If pool is frequently full, increase size
            if pool.in_use == pool.max_size && self.stats.total_reuses > self.stats.total_allocations {
                let new_size = ((pool.max_size as f32) * self.config.growth_factor) as usize;
                pool.max_size = new_size;
                
                // Update config
                self.config.max_pool_sizes.insert(type_name.clone(), new_size);
            }
            
            // If pool is rarely used, decrease size
            if pool.in_use < pool.max_size / 4 && pool.max_size > 10 {
                let new_size = pool.max_size / 2;
                pool.max_size = new_size;
                
                // Update config
                self.config.max_pool_sizes.insert(type_name.clone(), new_size);
                
                // Remove excess objects
                while pool.available.len() > new_size {
                    if let Some(obj) = pool.available.pop_back() {
                        self.stats.current_memory -= obj.size;
                    }
                }
            }
        }
    }

    /// Generate Luau code for pool initialization
    pub fn generate_pool_code(&self) -> String {
        let mut code = String::new();
        code.push_str("local ObjectPool = {\n");
        code.push_str("    pools = {},\n");
        code.push_str("    stats = { total_allocs = 0, total_reuses = 0 },\n");
        code.push_str("}\n\n");

        // Add pool methods
        code.push_str("function ObjectPool:acquire(type_name, size)\n");
        code.push_str("    local pool = self.pools[type_name]\n");
        code.push_str("    if not pool then\n");
        code.push_str("        pool = { available = {}, in_use = 0 }\n");
        code.push_str("        self.pools[type_name] = pool\n");
        code.push_str("    end\n\n");
        code.push_str("    if #pool.available > 0 then\n");
        code.push_str("        local obj = table.remove(pool.available)\n");
        code.push_str("        pool.in_use = pool.in_use + 1\n");
        code.push_str("        self.stats.total_reuses = self.stats.total_reuses + 1\n");
        code.push_str("        return obj\n");
        code.push_str("    else\n");
        code.push_str("        pool.in_use = pool.in_use + 1\n");
        code.push_str("        self.stats.total_allocs = self.stats.total_allocs + 1\n");
        code.push_str("        return {}\n");
        code.push_str("    end\n");
        code.push_str("end\n\n");

        code.push_str("function ObjectPool:release(obj, type_name)\n");
        code.push_str("    local pool = self.pools[type_name]\n");
        code.push_str("    if pool then\n");
        code.push_str("        -- Clear the object\n");
        code.push_str("        for k in pairs(obj) do obj[k] = nil end\n");
        code.push_str("        table.insert(pool.available, obj)\n");
        code.push_str("        pool.in_use = pool.in_use - 1\n");
        code.push_str("    end\n");
        code.push_str("end\n\n");

        code.push_str("return ObjectPool\n");
        code
    }
}
