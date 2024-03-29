use rand::*;
use std::fmt;
use std::ops::*;
use std::vec::Vec;

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

    pub fn sqr_magn(&self) -> f32 {
        self.magn() * self.magn()
    }

    pub fn to_unit(self) -> Vec3 {
        Vec3(
            self.0 / self.magn(),
            self.1 / self.magn(),
            self.2 / self.magn(),
        )
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - n * 2.0 * Vec3::dot(v, n)
    }

    pub fn refract(v: Vec3, n: Vec3, ni_div_nt: f32, refracted: &mut Vec3) -> bool {
        let unit_v = v.to_unit();
        let dot_prod = Vec3::dot(unit_v, n);
        let discr = 1.0 - ni_div_nt * ni_div_nt * (1.0 - dot_prod * dot_prod);
        if discr > 0.0 {
            *refracted = (unit_v - n * dot_prod) * ni_div_nt - n * discr.sqrt();
            true
        } else {
            false
        }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vec3({}, {}, {})", self.0, self.1, self.2)
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Vec3(-self.0, -self.1, -self.2)
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

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Vec3(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        if other == 0.0 {
            panic!("Cant divide by zero");
        }

        Vec3(self.0 / other, self.1 / other, self.2 / other)
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
        if other == 0.0 {
            panic!("You cant divide by zero");
        }
        self.0 /= other;
        self.1 /= other;
        self.2 /= other;
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        if other.0 == 0.0 || other.1 == 0.0 || other.2 == 0.0 {
            panic!("Cant divide by zero");
        }

        Vec3(self.0 / other.0, self.1 / other.1, self.2 / other.2)
    }
}

impl Copy for Vec3 {}
impl Clone for Vec3 {
    fn clone(&self) -> Vec3 {
        *self
    }
}

#[derive(Clone)]
pub struct HitInfo {
    hit_param: f32,
    hit_point: Vec3,
    normal: Vec3,
    material: Material,
}

impl HitInfo {
    pub fn new() -> HitInfo {
        HitInfo {
            hit_param: 0.0,
            hit_point: Vec3::zero(),
            normal: Vec3::zero(),
            material: Material::new(Vec3::zero(), true, false, false, 1.0, 1.5),
        }
    }
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, info: &mut HitInfo) -> bool;
}

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Material,
}

impl Sphere {
    pub fn new(material: Material, center: Vec3, radius: f32) -> Sphere {
        Sphere {
            material,
            center,
            radius,
        }
    }

    pub fn get_material(&self) -> Material {
        self.material.clone()
    }

    pub fn unit_from_origin(material: Material) -> Sphere {
        Sphere {
            material,
            center: Vec3(0.0, 0.0, 0.0),
            radius: 1.0,
        }
    }

    pub fn unit_from_point(center: Vec3, material: Material) -> Sphere {
        Sphere {
            material,
            center,
            radius: 1.0,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, info: &mut HitInfo) -> bool {
        let oc = ray.get_origin() - self.center;
        let a = Vec3::dot(ray.get_dir(), ray.get_dir());
        let b = 2.0 * Vec3::dot(oc, ray.get_dir());
        let c = Vec3::dot(oc, oc) - self.radius * self.radius;
        let discr = b * b - 4.0 * a * c;
        info.material = self.get_material();
        if discr > 0.0 {
            let mut temp = (-b - discr.sqrt()) / (2.0 * a);
            if temp > t_min && temp < t_max {
                info.hit_param = temp;
                info.hit_point = ray.point_at_parameter(temp);
                info.normal = (info.hit_point - self.center) / self.radius;
                return true;
            } else {
                temp = (-b + discr.sqrt()) / (2.0 * a);
                if temp > t_min && temp < t_max {
                    info.hit_param = temp;
                    info.hit_point = ray.point_at_parameter(temp);
                    info.normal = (info.hit_point - self.center) / self.radius;
                    return true;
                } else {
                    return false;
                }
                return false;
            }
        } else {
            return false;
        }
    }
}

pub struct HitableList<T>
where
    T: Hitable,
{
    list: Vec<T>,
    list_size: i32,
}

impl<T> HitableList<T>
where
    T: Hitable,
{
    pub fn new() -> HitableList<T> {
        HitableList::<T> {
            list: Vec::<T>::new(),
            list_size: 0,
        }
    }

    pub fn new_filled_list(items: Vec<T>) -> HitableList<T> {
        let list_size = items.len() as i32;
        HitableList::<T> {
            list: items,
            list_size,
        }
    }
}

impl<T> Hitable for HitableList<T>
where
    T: Hitable,
{
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, info: &mut HitInfo) -> bool {
        let mut temp_info: HitInfo = HitInfo::new();
        let mut hit_any: bool = false;
        let mut closest_yet: f32 = t_max;
        for item in self.list.iter() {
            if item.hit(ray, t_min, closest_yet, &mut temp_info) {
                hit_any = true;
                closest_yet = temp_info.hit_param;
                info.hit_param = temp_info.hit_param;
                info.hit_point = temp_info.hit_point;
                info.normal = temp_info.normal;
                info.material = temp_info.material.clone();
            }
        }
        hit_any
    }
}

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            lower_left_corner: Vec3(-2.0, -1.0, -1.0),
            horizontal: Vec3(4.0, 0.0, 0.0),
            vertical: Vec3(0.0, 2.0, 0.0),
            origin: Vec3(0.0, 0.0, 0.0),
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}

pub trait Scattering {
    fn scatter(
        &self,
        ray_in: Ray,
        info: HitInfo,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}

#[derive(Clone)]
pub struct Material {
    albedo: Vec3,
    metal: bool,
    diffuse: bool,
    dielectric: bool,
    fuzzines: f32,
    refraction_index: f32,
}

impl Material {
    pub fn new(
        albedo: Vec3,
        metal: bool,
        diffuse: bool,
        dielectric: bool,
        fuzzines: f32,
        refraction_index: f32,
    ) -> Material {
        if metal & diffuse & dielectric || (!metal && !diffuse && !dielectric) {
            panic!("You cant have both materials, pick one");
        }

        Material {
            albedo,
            metal,
            diffuse,
            dielectric,
            fuzzines,
            refraction_index,
        }
    }

    pub fn schlick(cosine: f32, refraction_index: f32) -> f32
    {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * ((1.0 - cosine).powi(5))
    }
}

impl Scattering for Material {
    fn scatter(
        &self,
        ray_in: Ray,
        info: HitInfo,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        if self.diffuse {
            let target = info.hit_point + info.normal + random_point_in_unit_sphere();
            *scattered = Ray::new(info.hit_point, target - info.hit_point);
            *attenuation = self.albedo;
            true
        } else if self.metal {
            let reflected = Vec3::reflect(ray_in.get_dir().to_unit(), info.normal);
            *scattered = Ray::new(
                info.hit_point,
                reflected + random_point_in_unit_sphere() * self.fuzzines,
            );
            *attenuation = self.albedo;
            Vec3::dot(scattered.get_dir(), info.normal) > 0.0
        } else {
            let outward_normal: Vec3;
            let reflected = Vec3::reflect(ray_in.get_dir(), info.normal);
            let ni_div_nt: f32;
            *attenuation = Vec3(1.0, 1.0, 0.0);
            let mut refracted: Vec3 = Vec3::zero();
            let mut reflection_probe;
            let mut cosine;
            if Vec3::dot(ray_in.get_dir(), info.normal) > 0.0 {
                outward_normal = -info.normal;
                ni_div_nt = self.refraction_index;
                cosine = self.refraction_index * Vec3::dot(ray_in.get_dir(), info.normal) / ray_in.get_dir().magn();
            } else {
                outward_normal = info.normal;
                ni_div_nt = 1.0 / self.refraction_index;
                cosine = -Vec3::dot(ray_in.get_dir(), info.normal) / ray_in.get_dir().magn();
            }
            if Vec3::refract(ray_in.get_dir(), outward_normal, ni_div_nt, &mut refracted) {
                reflection_probe = Material::schlick(cosine, self.refraction_index);
            } else {
                *scattered = Ray::new(info.hit_point, reflected);
                reflection_probe = 1.0;
            }
            let rand1: f32 = rand::thread_rng().gen();
            if rand1 < reflection_probe
            {
                *scattered = Ray::new(info.hit_point, reflected);
            }
            else
            {
                *scattered = Ray::new(info.hit_point, refracted);
            }
            true
        }
    }
}

pub fn color<T: Hitable>(ray: Ray, world: &T, depth: i32) -> Vec3 {
    let mut hit_info = HitInfo::new();

    if world.hit(&ray, 0.001, std::f32::MAX, &mut hit_info) {
        let mut scattered: Ray = Ray::new(Vec3::zero(), Vec3::zero());
        let mut attenuation: Vec3 = Vec3::zero();
        if depth < 50
            && hit_info
                .material
                .scatter(ray, hit_info.clone(), &mut attenuation, &mut scattered)
        {
            attenuation * color(scattered, world, depth)
        } else {
            Vec3::zero()
        }
    } else {
        //let unit_dir = ray.get_dir().to_unit();
        let t = 0.5 * (ray.get_dir().to_unit().1 + 1.0);
        Vec3(1.0, 1.0, 1.0) * (1.0 - t) + Vec3(0.5, 0.7, 1.0) * t
    }
    /*let t = hit_sphere(Vec3(0.0, 0.0, -1.0), 0.5, &ray);
    if t > 0.0 {
        let norm = (ray.point_at_parameter(t) - Vec3(0.0, 0.0, -1.0)).to_unit();
        Vec3(norm.0 + 1.0, norm.1 + 1.0, norm.2 + 1.0) * 0.5
    } else {
        let unit_dir = ray.get_dir().to_unit();
        let t = 0.5 * (unit_dir.1 + 1.0);

        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }*/
}

pub fn random_point_in_unit_sphere() -> Vec3 {
    let mut point;
    loop {
        point = Vec3(rand::thread_rng().gen(), rand::thread_rng().gen(), rand::thread_rng().gen()) * 2.0 - Vec3::new(1.0, 1.0, 1.0);
        if point.sqr_magn() < 1.0 {
            return point;
        }
    }
}

pub fn hit_sphere(center: Vec3, radius: f32, ray: &Ray) -> f32 {
    let oc = ray.get_origin() - center;
    let a = Vec3::dot(ray.get_dir(), ray.get_dir());
    let b = 2.0 * Vec3::dot(oc, ray.get_dir());
    let c = Vec3::dot(oc, oc) - radius * radius;
    let discr = b * b - 4.0 * a * c;
    if discr < 0.0 {
        -1.0
    } else {
        (-b - discr.sqrt()) / (2.0 * a)
    }
}
