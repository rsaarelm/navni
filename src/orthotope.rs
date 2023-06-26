use std::{
    fmt::Debug,
    ops::{Add, Div, Sub},
};

use num_traits::{Euclid, One, Zero};

pub type Rect<T> = Orthotope<T, 2>;
pub type Cube<T> = Orthotope<T, 3>;

pub trait Element:
    Copy
    + Default
    + PartialOrd
    + Sub<Output = Self>
    + Div<Output = Self>
    + Zero
    + One
{
}

impl<T> Element for T where
    T: Copy
        + Default
        + PartialOrd
        + Sub<Output = Self>
        + Div<Output = Self>
        + Zero
        + One
{
}

/// Cartesian product of several ranges.
///
/// Equivalent to an axis-aligned bounding rectangle, bounding box etc.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Orthotope<T, const N: usize> {
    p0: [T; N],
    p1: [T; N],
}

pub fn area<T: Element>(w: T, h: T) -> Orthotope<T, 2> {
    Orthotope::sized([w, h])
}

pub fn volume<T: Element, const N: usize>(
    p: impl Into<[T; N]>,
) -> Orthotope<T, N> {
    Orthotope::sized(p.into())
}

impl<T, const N: usize> Orthotope<T, N> {
    /// Faster than `Orthotope::new`, but does not check that dimensions are
    /// positive.
    ///
    /// # Safety
    ///
    /// Caller must ensure `p1[i] >= p0[i]` for all i.
    pub unsafe fn new_unsafe(
        p0: impl Into<[T; N]>,
        p1: impl Into<[T; N]>,
    ) -> Orthotope<T, N> {
        Orthotope {
            p0: p0.into(),
            p1: p1.into(),
        }
    }
}

impl<T: Element, const N: usize> Default for Orthotope<T, N> {
    fn default() -> Self {
        Orthotope {
            p0: [T::default(); N],
            p1: [T::default(); N],
        }
    }
}

impl<T: Element, const N: usize> Orthotope<T, N> {
    /// Create a new orthotope. If p1 has components that are smaller than
    /// p0's, the corresponding range is clamped to zero.
    pub fn new(
        p0: impl Into<[T; N]>,
        p1: impl Into<[T; N]>,
    ) -> Orthotope<T, N> {
        let (p0, p1) = (p0.into(), p1.into());

        Orthotope {
            p0,
            p1: std::array::from_fn(|i| pmax(p0[i], p1[i])),
        }
    }

    pub fn sized(p: impl Into<[T; N]>) -> Orthotope<T, N> {
        Orthotope::new([T::zero(); N], p)
    }

    /// Builds an orthotope from the elementwise minimum and maximum of the
    /// points in the input point cloud.
    ///
    /// NB. The resulting orthotope does not contain the outer rim of the
    /// points since the component ranges are exclusive on the outer end.
    pub fn from_points(
        it: impl IntoIterator<Item = impl Into<[T; N]>>,
    ) -> Orthotope<T, N> {
        let mut it = it.into_iter();
        if let Some(p) = it.next().map(|e| e.into()) {
            let (p0, p1) =
                it.map(|e| e.into()).fold((p, p), |(mut p0, mut p1), p| {
                    for i in 0..N {
                        p0[i] = pmin(p0[i], p[i]);
                        p1[i] = pmax(p1[i], p[i]);
                    }
                    (p0, p1)
                });
            Orthotope { p0, p1 }
        } else {
            Default::default()
        }
    }

    /// Builds an orthotope guaranteed to contain every point in the point
    /// cloud. For integer `T` the result is the smallest such orthotope.
    pub fn from_points_inclusive(
        it: impl IntoIterator<Item = impl Into<[T; N]>>,
    ) -> Orthotope<T, N> {
        let mut it = it.into_iter();
        if let Some(p0) = it.next().map(|e| e.into()) {
            let mut p1 = p0;
            for e in p1.iter_mut() {
                *e = *e + T::one();
            }

            let (p0, p1) =
                it.map(|e| e.into()).fold((p0, p1), |(mut p0, mut p1), p| {
                    for i in 0..N {
                        p0[i] = pmin(p0[i], p[i]);
                        p1[i] = pmax(p1[i], p[i] + T::one());
                    }
                    (p0, p1)
                });
            Orthotope { p0, p1 }
        } else {
            Default::default()
        }
    }

    pub fn is_empty(&self) -> bool {
        (0..N).any(|i| self.p1[i] <= self.p0[i])
    }

    pub fn contains(&self, e: impl Into<[T; N]>) -> bool {
        let e = e.into();
        (0..N).all(move |i| (self.p0[i]..self.p1[i]).contains(&e[i]))
    }

    pub fn contains_other(&self, r: &Self) -> bool {
        (0..N).all(|i| (self.p0[i] <= r.p0[i] && self.p1[i] >= r.p1[i]))
    }

    /// Return the product of the components of the dimension vector of the
    /// orthotope.
    ///
    /// NB. This can overflow easily with large multidimensional orthotopes.
    pub fn volume(&self) -> T {
        (0..N)
            .map(move |i| self.p1[i] - self.p0[i])
            .fold(T::one(), |a, b| a * b)
    }

    /// Return vector with dimensions of the orthotope.
    pub fn dim(&self) -> [T; N] {
        let mut ret = self.p1;
        for i in 0..N {
            ret[i] = ret[i] - self.p0[i];
        }
        ret
    }

    pub fn min(&self) -> [T; N] {
        self.p0
    }

    pub fn max(&self) -> [T; N] {
        self.p1
    }

    pub fn width(&self) -> T {
        self.p1[0] - self.p0[0]
    }

    pub fn height(&self) -> T {
        debug_assert!(N >= 2);
        self.p1[1] - self.p0[1]
    }

    pub fn depth(&self) -> T {
        debug_assert!(N >= 3);
        self.p1[2] - self.p0[2]
    }

    pub fn inflate(&self, amount: impl Into<[T; N]>) -> Self {
        let amount = amount.into();
        let (mut p0, mut p1) = (self.p0, self.p1);
        for i in 0..N {
            p0[i] = p0[i] - amount[i];
            p1[i] = p1[i] + amount[i];
        }

        Orthotope::new(p0, p1)
    }

    pub fn center(&self) -> [T; N] {
        let two = T::one() + T::one();
        let dim = self.dim();
        let mut ret = self.p0;
        for i in 0..N {
            ret[i] = ret[i] + dim[i] / two;
        }
        ret
    }

    /// Clamp each coordinate of input point to be within the bounds.
    pub fn clamp<E: Into<[T; N]> + From<[T; N]>>(&self, e: E) -> E {
        let mut p = e.into();
        for i in 0..N {
            if p[i] < self.p0[i] {
                p[i] = self.p0[i];
            } else if p[i] >= self.p1[i] {
                p[i] = self.p1[i] - T::one();
            }
        }
        E::from(p)
    }

    /// Return the orthotope of the intersection of `self` and `rhs`.
    pub fn intersection(&self, rhs: &Self) -> Self {
        Orthotope::new(
            std::array::from_fn(|i| pmax(self.p0[i], rhs.p0[i])),
            std::array::from_fn(|i| pmin(self.p1[i], rhs.p1[i])),
        )
    }

    /// Return the smallest orthotope that contains `self` and `rhs`.
    pub fn union(&self, rhs: &Self) -> Self {
        Orthotope::new(
            std::array::from_fn(|i| pmin(self.p0[i], rhs.p0[i])),
            std::array::from_fn(|i| pmax(self.p1[i], rhs.p1[i])),
        )
    }

    pub fn longest_axis(&self) -> usize {
        let d = self.dim();
        let mut ret = 0;
        for i in 1..N {
            if d[i] > d[ret] {
                ret = i;
            }
        }
        ret
    }

    pub fn split_along(&self, axis: usize) -> [Self; 2] {
        let mut sp0 = self.p0;
        let mut sp1 = self.p1;

        let two = T::one() + T::one();
        let midpoint = self.p0[axis] + (self.p1[axis] - self.p0[axis]) / two;

        sp0[axis] = midpoint;
        sp1[axis] = midpoint;

        std::array::from_fn(|i| {
            if i == 0 {
                Orthotope::new(self.p0, sp1)
            } else {
                Orthotope::new(sp0, self.p1)
            }
        })
    }
}

impl<T, const N: usize> Orthotope<T, N>
where
    T: Element + Euclid,
{
    /// Projects a point into the inside of the orthotope using modular
    /// arithmetic on each axis. A point leaving across one end will return on
    /// the other end.
    pub fn mod_proj<E>(&self, p: E) -> E
    where
        E: From<[T; N]> + Into<[T; N]>,
    {
        let mut p = p.into();
        for i in 0..N {
            p[i] = p[i] - self.p0[i];
            p[i] = p[i].rem_euclid(&(self.p1[i] - self.p0[i]));
            p[i] = p[i] + self.p0[i];
        }
        E::from(p)
    }
}

impl<T, const N: usize> Orthotope<T, N>
where
    T: Element + Euclid + TryInto<usize> + TryFrom<usize>,
{
    pub fn idx(&self, p: impl Into<[T; N]>) -> usize {
        let p = p.into();

        let size: [T; N] = self.dim();
        let mut span = [0; N];
        for i in 0..N {
            let Ok(x) = size[i].try_into() else {
                panic!("bad range");
            };
            span[i] = x;
        }

        let mut ret = 0;
        let mut scale = 1;
        for i in 0..N {
            let Ok(x) = (p[i] - self.p0[i]).rem_euclid(&size[i]).try_into() else {
                panic!("bad range");
            };
            ret += x * scale;
            scale *= span[i];
        }

        ret
    }

    /// Return the cell containing the given point in the lattice that has self as
    /// the unit cell.
    pub fn lattice_cell_for(&self, p: impl Into<[T; N]>) -> Self {
        let mut p = p.into();
        let mut p2 = p;

        let size: [T; N] = self.dim();

        for i in 0..N {
            p[i] =
                self.p0[i] + size[i] * (p[i] - self.p0[i]).div_euclid(&size[i]);
            p2[i] = p[i] + size[i];
        }

        Orthotope::new(p, p2)
    }

    pub fn get(&self, n: usize) -> [T; N] {
        let size: [T; N] = self.dim();
        let mut span = [0; N];
        for i in 0..N {
            let Ok(x) = size[i].try_into() else {
                panic!("bad range");
            };
            span[i] = x;
        }

        let mut v = [0; N];
        let mut scale = 1;
        for i in 0..N {
            v[i] = (n / scale) % span[i];
            scale *= span[i];
        }

        let mut e = [Default::default(); N];
        for i in 0..N {
            let Ok(x) = T::try_from(v[i]) else {
                panic!("bad range");
            };
            e[i] = self.p0[i] + x;
        }
        e
    }
}

impl<T: Element> Orthotope<T, 2> {
    /// Split a rectangle into four quarters.
    pub fn partition(&self) -> [Self; 4] {
        let center = self.center();
        let xp = [self.p0[0], center[0], self.p1[0]];
        let yp = [self.p0[1], center[1], self.p1[1]];
        std::array::from_fn(|i| {
            let x = i % 2;
            let y = (i / 2) % 2;
            Orthotope::new([xp[x], yp[y]], [xp[x + 1], yp[y + 1]])
        })
    }
}

impl<T: Element> Orthotope<T, 3> {
    /// Split a cube into eight octants.
    pub fn partition(&self) -> [Self; 8] {
        let center = self.center();
        let xp = [self.p0[0], center[0], self.p1[0]];
        let yp = [self.p0[1], center[1], self.p1[1]];
        let zp = [self.p0[2], center[2], self.p1[2]];
        std::array::from_fn(|i| {
            let x = i % 2;
            let y = (i / 2) % 2;
            let z = i / 4;
            Orthotope::new(
                [xp[x], yp[y], zp[z]],
                [xp[x + 1], yp[y + 1], zp[z + 1]],
            )
        })
    }
}

impl<E, T, const N: usize> Add<E> for Orthotope<T, N>
where
    E: Into<[T; N]>,
    T: Element,
{
    type Output = Orthotope<T, N>;

    fn add(self, rhs: E) -> Self::Output {
        let rhs = rhs.into();
        let mut ret = self;
        for i in 0..N {
            ret.p0[i] = ret.p0[i] + rhs[i];
            ret.p1[i] = ret.p1[i] + rhs[i];
        }
        ret
    }
}

impl<E, T, const N: usize> Sub<E> for Orthotope<T, N>
where
    E: Into<[T; N]>,
    T: Element,
{
    type Output = Orthotope<T, N>;

    fn sub(self, rhs: E) -> Self::Output {
        let rhs = rhs.into();
        let mut ret = self;
        for i in 0..N {
            ret.p0[i] = ret.p0[i] - rhs[i];
            ret.p1[i] = ret.p1[i] - rhs[i];
        }
        ret
    }
}

impl<T: Element, const N: usize> IntoIterator for Orthotope<T, N> {
    type Item = [T; N];

    type IntoIter = OrthotopeIter<T, N>;

    fn into_iter(self) -> OrthotopeIter<T, N> {
        OrthotopeIter {
            inner: self,
            x: self.p0,
        }
    }
}

pub struct OrthotopeIter<T, const N: usize> {
    inner: Orthotope<T, N>,
    x: [T; N],
}

impl<T: Element, const N: usize> Iterator for OrthotopeIter<T, N> {
    type Item = [T; N];

    fn next(&mut self) -> Option<Self::Item> {
        for i in 0..(N - 1) {
            if self.x[i] >= self.inner.p1[i] {
                self.x[i] = self.inner.p0[i];
                self.x[i + 1] = self.x[i + 1] + T::one();
            }
        }
        if self.x[N - 1] >= self.inner.p1[N - 1] {
            // Out of content.
            return None;
        }
        let ret = self.x;
        self.x[0] = self.x[0] + T::one();
        Some(ret)
    }
}

/// Return the larger of the two numbers. If the numbers can't be ordered, try
/// to return the number that can be ordered with itself.
pub fn pmin<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        a
    } else if b.partial_cmp(&b).is_some() {
        b
    } else {
        a
    }
}

/// Return the smaller of the two numbers. If the numbers can't be ordered,
/// try to return the number that can be ordered with itself.
pub fn pmax<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else if b.partial_cmp(&b).is_some() {
        b
    } else {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indexing() {
        let bounds: Orthotope<i32, 3> = Orthotope::new([1, 2, 3], [4, 5, 6]);

        for (i, p) in bounds.into_iter().enumerate() {
            if i == 0 {
                assert_eq!(p, [1, 2, 3]);
            }
            assert_eq!(i, bounds.idx(p));
            assert_eq!(bounds.get(i), p);
        }
    }

    #[test]
    fn pmin_pmax() {
        assert_eq!(pmax(1.0, 2.0), 2.0);
        assert_eq!(pmax(f32::NAN, 2.0), 2.0);
        assert_eq!(pmax(1.0, f32::NAN), 1.0);
        assert!(pmax(f32::NAN, f32::NAN).is_nan());

        assert_eq!(pmin(1.0, 2.0), 1.0);
        assert_eq!(pmin(f32::NAN, 2.0), 2.0);
        assert_eq!(pmin(1.0, f32::NAN), 1.0);
        assert!(pmin(f32::NAN, f32::NAN).is_nan());
    }

    #[test]
    fn custom_numeric_type() {
        type F = fraction::Fraction;
        let bounds = area(F::from(10), F::from(20));

        assert_eq!(bounds.center(), [F::from(5), F::from(10)]);
    }

    #[test]
    fn partition() {
        // 2D
        let square: Orthotope<i32, 2> = volume([3, 4]);
        let qt: [Orthotope<i32, 2>; 4] = square.partition();
        for i in 0..4 {
            assert!(square.contains_other(&qt[i]));
            assert!(qt[i].volume() > 0);
            for j in 0..4 {
                if j == i {
                    continue;
                }
                assert_eq!(qt[i].intersection(&qt[j]).volume(), 0);
            }
        }
        assert_eq!(qt.iter().map(|o| o.volume()).sum::<i32>(), square.volume());

        // 3D
        let cube: Orthotope<i32, 3> = volume([3, 4, 5]);
        let oct: [Orthotope<i32, 3>; 8] = cube.partition();
        for i in 0..8 {
            assert!(cube.contains_other(&oct[i]));
            assert!(oct[i].volume() > 0);
            for j in 0..8 {
                if j == i {
                    continue;
                }
                assert!(oct[i].intersection(&oct[j]).is_empty());
            }
        }
        assert_eq!(oct.iter().map(|o| o.volume()).sum::<i32>(), cube.volume());
    }

    #[test]
    fn split() {
        let cube: Orthotope<i32, 3> = volume([3, 4, 5]);
        for axis in 0..3 {
            let [a, b] = cube.split_along(axis);
            assert!(a.volume() > 0);
            assert!(b.volume() > 0);
            assert_eq!(a.union(&b), cube);
            assert!(a.intersection(&b).is_empty());
            assert!(cube.contains_other(&a));
            assert!(cube.contains_other(&b));
        }

        let even_cube: Orthotope<i32, 3> = volume([2, 6, 10]);
        for axis in 0..3 {
            let [a, b] = even_cube.split_along(axis);
            assert_eq!(a.volume(), b.volume());
            assert_eq!(a.union(&b), even_cube);
            assert!(a.intersection(&b).is_empty());
            assert!(even_cube.contains_other(&a));
            assert!(even_cube.contains_other(&b));
        }
    }
}
