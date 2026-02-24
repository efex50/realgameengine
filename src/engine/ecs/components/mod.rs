use bytemuck::{Pod, Zeroable};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub position: [f32; 2],
    pub scale: [f32; 2],
    pub rotation: f32, // in radians
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0],
            scale: [1.0, 1.0],
            rotation: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sprite {
    pub color: [f32; 4], // RGBA
}

impl Default for Sprite {
    fn default() -> Self {
        Self {
            color: [1.0, 1.0, 1.0, 1.0], // White
        }
    }
}

// Ensure it can be sent to shader
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct SpriteInstance {
    pub position: [f32; 2],
    pub scale: [f32; 2],
    pub rotation: f32,
    pub color: [f32; 4],
}

impl SpriteInstance {
    pub fn from_components(transform: &Transform, sprite: &Sprite) -> Self {
        Self {
            position: transform.position,
            scale: transform.scale,
            rotation: transform.rotation,
            color: sprite.color,
        }
    }
}
