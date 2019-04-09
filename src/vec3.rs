use rand::random;
use std::ops::{Add, AddAssign, Mul, Neg, Sub};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn squared_length(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, rhs: Vec3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: -self.x * rhs.z + self.z * rhs.x,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn make_unit_vector(&self) -> Vec3 {
        let inv_n = 1.0 / self.length();
        Vec3 {
            x: self.x * inv_n,
            y: self.y * inv_n,
            z: self.z * inv_n,
        }
    }

    pub fn reflect(&self, n: Vec3) -> Vec3 {
        *self - 2.0 * self.dot(n) * n
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        // TODO: since we're using random() in a loop, caching rng should
        // increase performace
        let p = 2.0
            * (Vec3::new(random::<f32>(), random::<f32>(), random::<f32>())
                - Vec3::new(0.0, 0.0, 0.0));
        if p.squared_length() >= 1.0 {
            break p;
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Vec3;

    #[test]
    fn add() {
        assert_eq!(
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 2.0
            } + Vec3 {
                x: 2.0,
                y: 1.0,
                z: 2.0
            },
            Vec3 {
                x: 3.0,
                y: 1.0,
                z: 4.0
            }
        );
    }

    #[test]
    fn add_assign() {
        let mut x = Vec3::new(0.0, 0.0, 0.0);
        let y = Vec3::new(1.0, 2.0, 3.0);
        x += y;
        assert_eq!(
            x,
            Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0
            }
        );
    }

    #[test]
    fn cross() {
        assert_eq!(
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 2.0
            }
            .cross(Vec3 {
                x: 2.0,
                y: 1.0,
                z: 2.0
            }),
            Vec3 {
                x: -2.0,
                y: 2.0,
                z: 1.0
            }
        );
    }

    #[test]
    fn dot() {
        assert_eq!(
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 2.0
            }
            .dot(Vec3 {
                x: 2.0,
                y: 1.0,
                z: 2.0
            }),
            6.0
        );
    }

    #[test]
    fn length() {
        let v = Vec3 {
            x: -2.0,
            y: -2.0,
            z: -1.0,
        };
        let u = Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        };
        assert_eq!(v.length(), 3.0);
        assert_eq!(u.length(), 1.0);
    }

    #[test]
    fn squared_length() {
        let v = Vec3 {
            x: -2.0,
            y: -2.0,
            z: -1.0,
        };
        let u = Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        };
        assert_eq!(v.squared_length(), 9.0);
        assert_eq!(u.squared_length(), 1.0);
    }

    #[test]
    fn mul() {
        assert_eq!(
            3.0 * Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0
            },
            Vec3 {
                x: 3.0,
                y: 6.0,
                z: 9.0
            }
        );
    }

    #[test]
    fn hadamard() {
        let lhs = Vec3::new(1.0, 1.0, 1.0);
        let rhs = Vec3::new(2.0, 3.0, 4.0);
        assert_eq!(lhs * rhs, Vec3::new(2.0, 3.0, 4.0));
    }

    #[test]
    fn neg() {
        assert_eq!(
            -Vec3 {
                x: 1.0,
                y: -2.0,
                z: 3.0
            },
            Vec3 {
                x: -1.0,
                y: 2.0,
                z: -3.0
            }
        );
    }

    #[test]
    fn sub() {
        assert_eq!(
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 2.0
            } - Vec3 {
                x: 2.0,
                y: 1.0,
                z: 2.0
            },
            Vec3 {
                x: -1.0,
                y: -1.0,
                z: 0.0
            }
        );
    }
}
