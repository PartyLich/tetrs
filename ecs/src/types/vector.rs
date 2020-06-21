use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

/// A 2 element vector (in the vector v scalar sense)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector2<T>
where
    T: Clone,
{
    pub x: T,
    pub y: T,
}

impl<T> Default for Vector2<T>
where
    T: Default + Clone,
{
    fn default() -> Self {
        Vector2 {
            x: T::default(),
            y: T::default(),
        }
    }
}

impl<T> Add for Vector2<T>
where
    T: Add + Clone,
    <T as Add>::Output: Add + Clone,
{
    type Output = Vector2<T::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2::<T::Output> {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> AddAssign for Vector2<T>
where
    T: AddAssign + Clone,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> Sub for Vector2<T>
where
    T: Sub + Clone,
    <T as Sub>::Output: Sub + Clone,
{
    type Output = Vector2<T::Output>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2::<T::Output> {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> SubAssign for Vector2<T>
where
    T: SubAssign + Clone,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> Mul<T> for Vector2<T>
where
    T: Mul + Clone + Copy,
    <T as Mul>::Output: Mul + Clone,
{
    type Output = Vector2<T::Output>;

    fn mul(self, rhs: T) -> Self::Output {
        Vector2::<T::Output> {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> MulAssign<T> for Vector2<T>
where
    T: MulAssign + Clone + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

/// A 3 element vector (in the vector v scalar sense)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3<T>
where
    T: Clone,
{
    x: T,
    y: T,
    z: T,
}

impl<T> Default for Vector3<T>
where
    T: Default + Clone,
{
    fn default() -> Self {
        Vector3 {
            x: T::default(),
            y: T::default(),
            z: T::default(),
        }
    }
}

impl<T> Add for Vector3<T>
where
    T: Add + Clone,
    <T as Add>::Output: Add + Clone,
{
    type Output = Vector3<T::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3::<T::Output> {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> AddAssign for Vector3<T>
where
    T: AddAssign + Clone,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T> Sub for Vector3<T>
where
    T: Sub + Clone,
    <T as Sub>::Output: Sub + Clone,
{
    type Output = Vector3<T::Output>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3::<T::Output> {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> SubAssign for Vector3<T>
where
    T: SubAssign + Clone,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T> Mul<T> for Vector3<T>
where
    T: Mul + Clone + Copy,
    <T as Mul>::Output: Mul + Clone,
{
    type Output = Vector3<T::Output>;

    fn mul(self, rhs: T) -> Self::Output {
        Vector3::<T::Output> {
            // Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T> MulAssign<T> for Vector3<T>
where
    T: MulAssign + Clone + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn vect2_add() {
        let expected = Vector2 { x: 20, y: 20 };
        let actual = Vector2 { x: 0, y: 0 } + expected;

        assert_eq!(actual, expected);
    }

    #[test]
    fn vect2_add_assign() {
        let expected = Vector2 { x: 20, y: 20 };
        let mut actual = Vector2 { x: 0, y: 0 };
        actual += expected;

        assert_eq!(actual, expected);
    }

    #[test]
    fn vect2_mul() {
        let expected = Vector2 { x: 20, y: 20 };
        let actual = Vector2 { x: 10, y: 10 } * 2;

        assert_eq!(actual, expected);
    }

    #[test]
    fn vect2_default() {
        let expected = Vector2 { x: 0, y: 0 };
        let actual = Vector2::default();

        assert_eq!(actual, expected);
    }

    #[test]
    fn vect3_add() {
        let expected = Vector3 {
            x: 30,
            y: 30,
            z: 30,
        };
        let actual = Vector3 { x: 0, y: 0, z: 0 } + expected;

        assert_eq!(actual, expected);
    }

    #[test]
    fn vect3_add_assign() {
        let expected = Vector3 {
            x: 30,
            y: 30,
            z: 30,
        };
        let mut actual = Vector3 { x: 0, y: 0, z: 0 };
        actual += expected;

        assert_eq!(actual, expected);
    }

    #[test]
    fn vect3_mul() {
        let expected = Vector3 {
            x: 30,
            y: 30,
            z: 30,
        };
        let actual = Vector3 {
            x: 10,
            y: 10,
            z: 10,
        } * 3;

        assert_eq!(actual, expected);
    }

    #[test]
    fn vect3_default() {
        let expected = Vector3 { x: 0, y: 0, z: 0 };
        let actual = Vector3::default();

        assert_eq!(actual, expected);
    }
}
