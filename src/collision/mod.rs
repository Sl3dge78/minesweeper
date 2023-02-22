use crate::math::*;

mod gjk;

#[derive(Debug)]
pub struct AABB {
    min: Vec3,
    max: Vec3,
}

#[allow(dead_code)]
impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> AABB {
        AABB { min, max }
    }

    pub fn from_size(center: Vec3, size: f32) -> AABB {
        Self::new(
            Vector3 {
                x: center.x - size / 2.0,
                y: center.y - size / 2.0,
                z: center.z - size / 2.0,
            },
            Vector3 {
                x: center.x + size / 2.0,
                y: center.y + size / 2.0,
                z: center.z + size / 2.0,
            },
        )
    }

    pub fn is_in(&self, p: Point3) -> bool {
        if p.x > self.min.x
            && p.x < self.max.x
            && p.y > self.min.y
            && p.y < self.max.y
            && p.z > self.min.z
            && p.z < self.max.z
        {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::collision::*;

    #[test]
    fn aabb() {
        let aabb = AABB::new(
            Vec3 {
                x: -1.0,
                y: -1.0,
                z: -1.0,
            },
            Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        );
        assert!(aabb.is_in(Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }));
        assert!(aabb.is_in(Point3 {
            x: 0.349,
            y: 0.234,
            z: 0.123
        }));
        assert!(!aabb.is_in(Point3 {
            x: 2.0,
            y: 0.0,
            z: 0.0
        }));
        assert!(!aabb.is_in(Point3 {
            x: 0.0,
            y: 2.0,
            z: 0.0
        }));
        assert!(!aabb.is_in(Point3 {
            x: 0.0,
            y: 0.0,
            z: 2.0
        }));
    }
}
