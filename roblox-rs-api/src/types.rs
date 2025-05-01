use std::ops::{Add, Sub, Mul, Div};
use serde::{Serialize, Deserialize};

/// Represents a 3D vector
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    /// Create a new Vector3
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    
    /// Get the magnitude of the vector
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    
    /// Get the unit vector in the same direction
    pub fn unit(&self) -> Self {
        let mag = self.magnitude();
        if mag == 0.0 {
            *self
        } else {
            *self / mag
        }
    }
    
    /// Get the dot product with another vector
    pub fn dot(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    
    /// Get the cross product with another vector
    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Add for Vector3 {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Self;
    
    fn mul(self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Div<f64> for Vector3 {
    type Output = Self;
    
    fn div(self, scalar: f64) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

/// Represents a color with RGB values
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Color3 {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color3 {
    /// Create a new Color3
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }
    
    /// Create a Color3 from RGB values (0-255)
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: r as f64 / 255.0,
            g: g as f64 / 255.0,
            b: b as f64 / 255.0,
        }
    }
}

/// Represents a coordinate frame (position and orientation)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CFrame {
    pub position: Vector3,
    /// 3x3 rotation matrix stored as [right, up, back] vectors
    pub orientation: [Vector3; 3],
}

impl CFrame {
    /// Create a new CFrame at the given position with identity rotation
    pub fn new(position: Vector3) -> Self {
        Self {
            position,
            orientation: [
                Vector3::new(1.0, 0.0, 0.0),
                Vector3::new(0.0, 1.0, 0.0),
                Vector3::new(0.0, 0.0, 1.0),
            ],
        }
    }
    
    /// Create a CFrame looking at a target from a position
    pub fn look_at(position: Vector3, target: Vector3) -> Self {
        let forward = (target - position).unit();
        let right = Vector3::new(0.0, 1.0, 0.0).cross(&forward).unit();
        let up = forward.cross(&right);
        
        Self {
            position,
            orientation: [right, up, forward],
        }
    }
    
    /// Transform a point from object space to world space
    pub fn transform_point(&self, point: Vector3) -> Vector3 {
        let [right, up, back] = self.orientation;
        self.position + Vector3::new(
            point.dot(&right),
            point.dot(&up),
            point.dot(&back),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vector3_operations() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);
        
        let sum = v1 + v2;
        assert_eq!(sum, Vector3::new(5.0, 7.0, 9.0));
        
        let scaled = v1 * 2.0;
        assert_eq!(scaled, Vector3::new(2.0, 4.0, 6.0));
        
        let dot = v1.dot(&v2);
        assert_eq!(dot, 32.0);
    }
    
    #[test]
    fn test_color3() {
        let color = Color3::from_rgb(255, 128, 0);
        assert_eq!(color.r, 1.0);
        assert_eq!(color.g, 0.5019607843137255);
        assert_eq!(color.b, 0.0);
    }
    
    #[test]
    fn test_cframe() {
        let pos = Vector3::new(1.0, 2.0, 3.0);
        let cf = CFrame::new(pos);
        
        let point = Vector3::new(0.0, 1.0, 0.0);
        let transformed = cf.transform_point(point);
        
        assert_eq!(transformed, Vector3::new(1.0, 3.0, 3.0));
    }
} 