use std::fmt;
use std::ops::*;

pub struct Ray {
    origin: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn get_dir(&self) -> Vec3 {
        self.dir
    }

    pub fn get_origin(&self) -> Vec3 {
        self.origin
    }

    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.get_origin() + self.get_dir() * t
    }

    pub fn new(origin: Vec3, dir: Vec3) -> Ray {
        Ray { origin, dir }
    }
}

pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3(x, y, z)
    }

    pub fn dot(a: Vec3, b: Vec3) -> f32 {
        a.0 * b.0 + a.1 * b.1 + a.2 * b.2
    }

    pub fn zero() -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }

    pub fn magn(&self) -> f32 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }

    pub fn to_unit(self) -> Vec3 {
        Vec3(
            self.0 / self.magn(),
            self.1 / self.magn(),
            self.2 / self.magn(),
        )
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vec3({}, {}, {})", self.0, self.1, self.2)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self(self.0 + other.0, self.1 + other.1, self.2 + other.2);
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Vec3(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl Copy for Vec3 {}
impl Clone for Vec3 {
    fn clone(&self) -> Vec3 {
        *self
    }
}

pub fn color(ray: Ray) -> Vec3 {
    if hit_sphere(Vec3(0.0, 0.0, -10.0), 0.5, &ray) {
        return Vec3(1.0, 0.0, 0.0);
    } else {
        let unit_dir = ray.get_dir().to_unit();
        let t = 0.5 * (unit_dir.1 + 1.0);

        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

pub fn hit_sphere(center: Vec3, radius: f32, ray: &Ray) -> bool {
    let oc = (*ray).get_origin() - center;
    let a = Vec3::dot(ray.get_dir(), ray.get_dir());
    let b = 2.0 * Vec3::dot(oc, ray.get_dir());
    let c = Vec3::dot(oc, oc) - radius * radius;
    let discr = b * b - 4.0 * a * c;
    discr > 0.0
}
