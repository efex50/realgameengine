

#[derive(Debug)]
pub struct GameObject {
    pub position: [f32; 2],
    pub rotation: f32,
}

pub struct EngineWorld{
    pub objects: Vec<GameObject>
}

impl EngineWorld {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, position: [f32; 2]) {
        self.objects.push(GameObject {
            position,
            rotation: 0.0,
        });
    }
}

