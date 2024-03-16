#[derive(PartialEq, Debug, Clone)]
pub enum TupleKind {
    None,
    Point,
    Vector,
    Color,
}

/// A very basic element representing points, vectors and colors
#[derive(Debug)]
pub struct Tuple {
    /// A Tuple can be a point, a vector, a color or none of them
    kind: TupleKind,
    /// The four float64 elements of a Tuple
    elements: [f64; 4],
}

impl Tuple {
    /// Returns a tuple with the four float64 elements given them and a none kind
    ///
    /// # Arguments
    ///
    /// * `e0` - A float64 representing the first element
    /// * `e1` - A float64 representing the second element
    /// * `e2` - A float64 representing the third element
    /// * `e3` - A float64 representing the fourth element
    pub fn new(e0: f64, e1: f64, e2: f64, e3: f64) -> Tuple {
        Tuple {
            kind: TupleKind::None,
            elements: [e0, e1, e2, e3],
        }
    }

    /// Returns a tuple with the three float64 elements given them and a point kind
    ///
    /// # Arguments
    ///
    /// * `e0` - A float64 representing the first element
    /// * `e1` - A float64 representing the second element
    /// * `e2` - A float64 representing the third element
    fn new_point(e0: f64, e1: f64, e2: f64) -> Tuple {
        Tuple {
            kind: TupleKind::Point,
            elements: [e0, e1, e2, 0.0],
        }
    }

    /// Returns a tuple with the three float64 elements given them and a vector kind
    ///
    /// # Arguments
    ///
    /// * `e0` - A float64 representing the first element
    /// * `e1` - A float64 representing the second element
    /// * `e2` - A float64 representing the third element
    fn new_vector(e0: f64, e1: f64, e2: f64) -> Tuple {
        Tuple {
            kind: TupleKind::Vector,
            elements: [e0, e1, e2, 0.0],
        }
    }

    /// Returns a tuple with the three float64 elements given them and a color kind
    ///
    /// # Arguments
    ///
    /// * `e0` - A float64 representing the first element
    /// * `e1` - A float64 representing the second element
    /// * `e2` - A float64 representing the third element
    fn new_color(e0: f64, e1: f64, e2: f64) -> Tuple {
        Tuple {
            kind: TupleKind::Color,
            elements: [e0, e1, e2, 0.0],
        }
    }

    /// Returns if both tuples are equals (it has some rounding at the fifth house)
    ///
    /// # Arguments
    ///
    /// * `other` - A tuple to compare with
    pub fn is_equal(&self, other: &Tuple) -> bool {
        self.kind == other.kind
            && is_equal(&self.elements[0], &other.elements[0])
            && is_equal(&self.elements[1], &other.elements[1])
            && is_equal(&self.elements[2], &other.elements[2])
            && is_equal(&self.elements[3], &other.elements[3])
    }

    /// Returns a new tuple with the result of adding other tuple to self
    ///
    /// # Arguments
    ///
    /// * `other` - A tuple to add to self
    ///
    /// # Return
    ///
    /// * `Option<Tuple>` - It can return none if it is trying to add two points
    /// or a tuple otherwise
    pub fn add(&self, other: &Tuple) -> Option<Tuple> {
        if self.kind == TupleKind::Point && other.kind == TupleKind::Point {
            None
        } else {
            let kind: TupleKind =
                if self.kind == TupleKind::Vector && other.kind == TupleKind::Vector {
                    TupleKind::Vector
                } else {
                    TupleKind::Point
                };

            Some(Tuple {
                kind: kind,
                elements: [
                    self.elements[0] + other.elements[0],
                    self.elements[1] + other.elements[1],
                    self.elements[2] + other.elements[2],
                    self.elements[3] + other.elements[3],
                ],
            })
        }
    }

    /// Returns a new tuple with the result of subtracting other tuple from self
    ///
    /// # Arguments
    ///
    /// * `other` - A tuple to sub from self
    ///
    /// # Return
    ///
    /// * `Option<Tuple>` - It can return none if it is trying to sub a point from a vector
    /// or a tuple otherwise
    pub fn sub(&self, other: &Tuple) -> Option<Tuple> {
        if self.kind == TupleKind::Vector && other.kind == TupleKind::Point {
            None
        } else {
            let kind: TupleKind =
                if self.kind == TupleKind::Point && other.kind == TupleKind::Vector {
                    TupleKind::Point
                } else {
                    TupleKind::Vector
                };

            Some(Tuple {
                kind: kind,
                elements: [
                    self.elements[0] - other.elements[0],
                    self.elements[1] - other.elements[1],
                    self.elements[2] - other.elements[2],
                    self.elements[3] - other.elements[3],
                ],
            })
        }
    }

    /// Returns a new tuple with the result of opposite self
    ///
    /// # Return
    ///
    /// * `Tuple` - It returns the opposite tuple
    pub fn neg(&self) -> Tuple {
        Tuple {
            kind: self.kind.clone(),
            elements: [
                -self.elements[0],
                -self.elements[1],
                -self.elements[2],
                -self.elements[3],
            ]
        }
    }
}

fn is_equal(a: &f64, b: &f64) -> bool {
    const EPSILON: f64 = 0.00001;

    (a - b).abs() < EPSILON
}

#[cfg(test)]
mod tests {
    use crate::primitives::{Tuple, TupleKind};

    #[test]
    fn new() {
        let result = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert_eq!(result.kind, TupleKind::None);
        assert_eq!(result.elements[0], 4.3);
        assert_eq!(result.elements[1], -4.2);
        assert_eq!(result.elements[2], 3.1);
        assert_eq!(result.elements[3], 1.0);
    }

    #[test]
    fn new_point() {
        let result = Tuple::new_point(4.0, -4.0, 3.0);
        assert_eq!(result.kind, TupleKind::Point);
        assert_eq!(result.elements[0], 4.0);
        assert_eq!(result.elements[1], -4.0);
        assert_eq!(result.elements[2], 3.0);
        assert_eq!(result.elements[3], 0.0);
    }

    #[test]
    fn new_vector() {
        let result = Tuple::new_vector(4.0, -4.0, 3.0);
        assert_eq!(result.kind, TupleKind::Vector);
        assert_eq!(result.elements[0], 4.0);
        assert_eq!(result.elements[1], -4.0);
        assert_eq!(result.elements[2], 3.0);
        assert_eq!(result.elements[3], 0.0);
    }

    #[test]
    fn new_color() {
        let result = Tuple::new_color(4.0, 4.0, 3.0);
        assert_eq!(result.kind, TupleKind::Color);
        assert_eq!(result.elements[0], 4.0);
        assert_eq!(result.elements[1], 4.0);
        assert_eq!(result.elements[2], 3.0);
        assert_eq!(result.elements[3], 0.0);
    }

    #[test]
    fn is_equal() {
        let t1 = Tuple::new(4.0, -4.0, 3.0, -5.2);
        let t2 = Tuple::new(3.999999, -4.000001, 3.0, -5.2);
        assert!(&t1.is_equal(&t2));
    }

    #[test]
    fn add() {
        // add vector to vector should return a vector
        let t1 = Tuple::new_vector(3.0, -2.0, 5.0);
        let t2 = Tuple::new_vector(-2.0, 3.0, 1.0);
        let r = Tuple::new_vector(1.0, 1.0, 6.0);
        let a = &t1.add(&t2);
        assert!(!&a.is_none());
        assert!(&a.is_some());
        assert!(&r.is_equal(a.as_ref().unwrap()));

        // add point to vector should return a point
        let t1 = Tuple::new_vector(3.0, -2.0, 5.0);
        let t2 = Tuple::new_point(-2.0, 3.0, 1.0);
        let r = Tuple::new_point(1.0, 1.0, 6.0);
        let a = &t1.add(&t2);
        assert!(!&a.is_none());
        assert!(&a.is_some());
        assert!(&r.is_equal(a.as_ref().unwrap()));

        // add vector to point should return a point
        let t1 = Tuple::new_point(3.0, -2.0, 5.0);
        let t2 = Tuple::new_vector(-2.0, 3.0, 1.0);
        let r = Tuple::new_point(1.0, 1.0, 6.0);
        let a = &t1.add(&t2);
        assert!(!&a.is_none());
        assert!(&a.is_some());
        assert!(&r.is_equal(a.as_ref().unwrap()));

        // add point to point should return none
        let t1 = Tuple::new_point(3.0, -2.0, 5.0);
        let t2 = Tuple::new_point(-2.0, 3.0, 1.0);
        let a = &t1.add(&t2);
        assert!(&a.is_none());
        assert!(!&a.is_some());
    }

    #[test]
    fn sub() {
        // sub point from point should return a vector
        let t1 = Tuple::new_point(3.0, 2.0, 1.0);
        let t2 = Tuple::new_point(5.0, 6.0, 7.0);
        let r = Tuple::new_vector(-2.0, -4.0, -6.0);
        let a = &t1.sub(&t2);
        assert!(!&a.is_none());
        assert!(&a.is_some());
        assert!(&r.is_equal(a.as_ref().unwrap()));

        // sub vector from point should return a point
        let t1 = Tuple::new_point(3.0, 2.0, 1.0);
        let t2 = Tuple::new_vector(5.0, 6.0, 7.0);
        let r = Tuple::new_point(-2.0, -4.0, -6.0);
        let a = &t1.sub(&t2);
        assert!(!&a.is_none());
        assert!(&a.is_some());
        assert!(&r.is_equal(a.as_ref().unwrap()));

        // sub vector from vector should return a vector
        let t1 = Tuple::new_vector(3.0, 2.0, 1.0);
        let t2 = Tuple::new_vector(5.0, 6.0, 7.0);
        let r = Tuple::new_vector(-2.0, -4.0, -6.0);
        let a = &t1.sub(&t2);
        assert!(!&a.is_none());
        assert!(&a.is_some());
        assert!(&r.is_equal(a.as_ref().unwrap()));

        // sub point from vector should return none
        let t1 = Tuple::new_vector(3.0, 2.0, 1.0);
        let t2 = Tuple::new_point(5.0, 6.0, 7.0);
        let a = &t1.sub(&t2);
        assert!(&a.is_none());
        assert!(!&a.is_some());
    }

    #[test]
    fn neg() {
        // neg a tuple shoud return a opposite tuple
        let t1 = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let r = Tuple::new(-1.0, 2.0, -3.0, 4.0);
        let a = &t1.neg();
        assert!(&r.is_equal(&a));

        // neg a point shoud return a opposite point
        let t1 = Tuple::new_point(1.0, -2.0, 3.0);
        let r = Tuple::new_point(-1.0, 2.0, -3.0);
        let a = &t1.neg();
        assert!(&r.is_equal(&a));

        // neg a vector shoud return a opposite vector
        let t1 = Tuple::new_vector(1.0, -2.0, 3.0);
        let r = Tuple::new_vector(-1.0, 2.0, -3.0);
        let a = &t1.neg();
        assert!(&r.is_equal(&a));
    }
}
