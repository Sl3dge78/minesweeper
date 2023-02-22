use crate::math::*;

#[allow(dead_code)]
pub fn gjk_2d(a: &dyn GJKShape, b: &dyn GJKShape) -> bool {

    let mut simplex = Vec::new();
    let direction = b.center() - a.center();
    if !add_vertex(&mut simplex, a, b, direction) {
        return false;
    }
    if !add_vertex(&mut simplex, a, b, -direction) {
        return false;
    }
    loop {
        match evolve_simplex(&mut simplex, a, b) {
            EvolveResult::Intersects => return true,
            EvolveResult::Continue => continue,
            EvolveResult::NoIntersection => return false,
        }
    }
}

enum EvolveResult {
    Intersects,
    Continue,
    NoIntersection,
}

fn add_vertex(simplex: &mut Vec<Vec2>, a: &dyn GJKShape, b: &dyn GJKShape, direction: Vec2) -> bool {
    let v = a.support(direction) - b.support(-direction);
    simplex.push(v);
    
    direction.dot(v) > 0.0
}

fn evolve_simplex(simplex: &mut Vec<Vec2>, a: &dyn GJKShape, b: &dyn GJKShape) -> EvolveResult {
    let direction;
    match simplex.len() {
        2 => {
            let cb = simplex[1] - simplex[0];
            let c0 = -simplex[0];
            direction = triple_product(cb, c0, cb);
        },
        3 => {
            let a0 = -simplex[2];
            let ab = simplex[1] - simplex[2];
            let ac = simplex[0] - simplex[2];

            let ab_perp = triple_product(ac, ab, ab);
            let ac_perp = triple_product(ab, ac, ac);

            if ab_perp.dot(a0) > 0.0 {
                simplex.swap_remove(0);
                direction = ab_perp;
            } else if ac_perp.dot(a0) > 0.0 {
                simplex.swap_remove(1);
                direction = ac_perp;
            } else {
                return EvolveResult::Intersects;
            }
        },
        _ => unreachable!()
    }

    if !add_vertex(simplex, a, b, -direction) {
        return EvolveResult::NoIntersection;
    } else {
        return EvolveResult::Continue;
    }
}

fn triple_product(a: Vec2, b: Vec2, c: Vec2) -> Vec2 {
    let a = Vec3::new(a.x, a.y, 0.0).normalize();
    let b = Vec3::new(b.x, b.y, 0.0).normalize();
    let c = Vec3::new(c.x, c.y, 0.0).normalize();

    let result = a.cross(b).cross(c);
    Vec2::new(result.x, result.y)
}

pub trait GJKShape {
    fn support(&self, direction: Vec2) -> Vec2;
    fn center(&self) -> Vec2;
}

impl GJKShape for Vec<Vec2> {
    fn support(&self, direction: Vec2) -> Vec2 {
        let mut furthest_distance = 0.0;
        let mut furthest_vertex = self[0];
        for v in self.iter() {
            let distance = v.dot(direction.normalize());
            if distance > furthest_distance {
                furthest_distance = distance;
                furthest_vertex = *v;
            }
        }
        furthest_vertex
    }

    fn center(&self) -> Vec2 {
        let mut signed_area = 0.0;
        let mut centroid = Vec2::zero();
        let mut last : Vec2 = *self.last().unwrap();
        for i in 0..self.len() {
            let p1 = self[i];
            let p0 = last;
            let a = p0.x * p1.y - p0.y * p1.x;
            signed_area += a;
            centroid += (p0 + p1) * a;
            last = p1;
        }
        
        signed_area *= 0.5;
        centroid / (6.0 * signed_area)
    }
}

#[cfg(test)]
mod tests {
    use crate::collision::gjk::*;

    #[test]
    fn center() {
        let a = vec![Vec2::new(-1.0, -1.0), Vec2::new(1.0, -1.0), Vec2::new(1.0, 1.0), Vec2::new(-1.0, 1.0)];
        assert!(a.center() == Vec2::new(0.0, 0.0));
        let a = vec![Vec2::new(0.0, -1.0), Vec2::new(2.0, -1.0), Vec2::new(2.0, 1.0), Vec2::new(0.0, 1.0)];
        assert!(a.center() == Vec2::new(1.0, 0.0));
        let a = vec![Vec2::new(0.0, -1.0), Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0), Vec2::new(-1.0, 0.0)];
        assert!(a.center() == Vec2::new(0.0, 0.0));
    }

    #[test]
    fn support() {
        let a = vec![Vec2::new(-1.0, -1.0), Vec2::new(1.0, -1.0), Vec2::new(1.0, 1.0), Vec2::new(-1.0, 1.0)];
        assert!(a.support(Vec2::new(-1.0, -1.0).normalize()) == Vec2::new(-1.0, -1.0));
        let a = vec![Vec2::new(0.0, -1.0), Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0), Vec2::new(-1.0, 0.0)];
        assert!(a.support(Vec2::new(-1.0, 0.0).normalize()) == Vec2::new(-1.0, 0.0));
        assert!(a.support(Vec2::new(1.0, 0.0).normalize()) == Vec2::new(1.0, 0.0));
        let a = vec![Vec2::new(0.0, -1.0), Vec2::new(0.0, -1.1), Vec2::new(0.0, 1.0), Vec2::new(-1.0, 0.0)];
        assert!(a.support(Vec2::new(0.0, -1.0).normalize()) == Vec2::new(0.0, -1.1));

        let a = vec![ Vec2::new(0.5, 0.5), Vec2::new(1.5, 1.0), Vec2::new(0.75, 2.0)];
        assert!(a.support(Vec2::new(1.0, 0.0).normalize()) == Vec2::new(1.5, 1.0));
    }
    
    #[test]
    fn ttriple_product() {
        assert!(triple_product(Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0), Vec2::new(1.0, 0.0)) == Vec2::new(0.0, 1.0)); // Basic
        assert!(triple_product(Vec2::new(2.0, 0.0), Vec2::new(0.0, 1.0), Vec2::new(2.0, 0.0)) == Vec2::new(0.0, 1.0)); // Normalize
    }

    #[test]
    fn test_2d_gjk() {
        let a = vec![ Vec2::new(0.0, 0.0), Vec2::new(0.0, 1.0), Vec2::new(1.0, 1.0), Vec2::new(1.0, 0.0)];
        let b = vec![ Vec2::new(0.5, 0.5), Vec2::new(1.5, 1.0), Vec2::new(0.75, 2.0)];
        assert!(gjk_2d(&a, &b));

        let b = vec![ Vec2::new(2.0, 1.0), Vec2::new(1.5, 1.0), Vec2::new(1.75, 2.0)];
        assert!(!gjk_2d(&a, &b));
    }
}

