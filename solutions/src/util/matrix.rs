//! Utility trait for managing matrixes with points.

use nalgebra::{DMatrix, Point2};

/// A point type expected for use with [`MatrixPointAccess`].
///
/// Points use `i32` components as some expected operations with a point can
/// include offsets from [`Vector2`][nalgebra::Vector2] with signed components.
pub type MatrixPoint = Point2<i32>;

pub fn matrix_point_from_usize(x: usize, y: usize) -> MatrixPoint {
    let cast_x: i32 = x.try_into().unwrap_or_else(|error| {
        panic!("x dimension could not be cast: {error:?}");
    });
    let cast_y: i32 = y.try_into().unwrap_or_else(|error| {
        panic!("y dimension could not be cast: {error:?}");
    });
    MatrixPoint::new(cast_x, cast_y)
}

/// An extension trait to manage matrix indexes via points.
pub trait MatrixPointAccess<T> {
    /// Check the matrix can be indexed by a point.
    fn contains_point(&self, point: MatrixPoint) -> bool;
    /// Get a value reference indexed by a point.
    fn get_at_point(&self, point: MatrixPoint) -> Option<&T>;
    /// Get a mutable value reference indexed by a point.
    fn get_at_point_mut(&mut self, point: MatrixPoint) -> Option<&mut T>;
    /// Get an iterator of points that can index the matrix.
    fn points(&self) -> impl Iterator<Item = MatrixPoint> + '_;
}

impl<T> MatrixPointAccess<T> for DMatrix<T> {
    fn contains_point(&self, point: MatrixPoint) -> bool {
        if point.x < 0 || point.y < 0 {
            return false;
        }
        #[expect(
            clippy::cast_sign_loss,
            reason = "already checked x is non-negative"
        )]
        let col = point.x as usize;
        #[expect(
            clippy::cast_sign_loss,
            reason = "already checked y is non-negative"
        )]
        let row = point.y as usize;
        col < self.ncols() && row < self.nrows()
    }

    fn get_at_point(&self, point: MatrixPoint) -> Option<&T> {
        if self.contains_point(point) {
            // indexing is (row, col)
            #[expect(
                clippy::cast_sign_loss,
                reason = "already determined point components are positive with contains_point"
            )]
            Some(&self[(point.y as usize, point.x as usize)])
        } else {
            None
        }
    }

    fn get_at_point_mut(&mut self, point: MatrixPoint) -> Option<&mut T> {
        if self.contains_point(point) {
            // indexing is (row, col)
            #[expect(
                clippy::cast_sign_loss,
                reason = "already determined point components are positive with contains_point"
            )]
            Some(&mut self[(point.y as usize, point.x as usize)])
        } else {
            None
        }
    }

    fn points(&self) -> impl Iterator<Item = MatrixPoint> + '_ {
        let rows: i32 = self.nrows().try_into().unwrap_or_else(|error| {
            panic!("matrix dimensions could not be cast, cannot iterate points: {error:?}");
        });
        let cols: i32 = self.ncols().try_into().unwrap_or_else(|error| {
            panic!("matrix dimensions could not be cast, cannot iterate points: {error:?}");
        });
        (0..rows)
            .flat_map(move |y| (0..cols).map(move |x| MatrixPoint::new(x, y)))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn contains_point_detects_inside_bounds() {
        let matrix = DMatrix::from_iterator(3, 3, 0..9);

        let point_00 = MatrixPoint::origin();
        assert!(
            matrix.contains_point(point_00),
            "expected {point_00:?} inside bounds"
        );
        let point_02 = MatrixPoint::new(0, 2);
        assert!(
            matrix.contains_point(point_02),
            "expected {point_02:?} inside bounds"
        );
        let point_21 = MatrixPoint::new(2, 1);
        assert!(
            matrix.contains_point(point_21),
            "expected {point_21:?} inside bounds"
        );
    }

    #[test]
    fn contains_point_detects_outside_bounds() {
        let matrix = DMatrix::from_iterator(3, 3, 0..9);

        let point_30 = MatrixPoint::new(3, 0);
        assert!(
            !matrix.contains_point(point_30),
            "expected {point_30:?} outside bounds"
        );
        let point_23 = MatrixPoint::new(2, 3);
        assert!(
            !matrix.contains_point(point_23),
            "expected {point_23:?} outside bounds"
        );

        let point_0n1 = MatrixPoint::new(0, -1);
        assert!(
            !matrix.contains_point(point_0n1),
            "expected {point_0n1:?} outside bounds"
        );
        let point_n11 = MatrixPoint::new(-1, 1);
        assert!(
            !matrix.contains_point(point_n11),
            "expected {point_n11:?} outside bounds"
        );

        let point_12_25 = MatrixPoint::new(12, 25);
        assert!(
            !matrix.contains_point(point_12_25),
            "expected {point_12_25:?} outside bounds"
        );
        let point_n20_n10 = MatrixPoint::new(-20, -10);
        assert!(
            !matrix.contains_point(point_n20_n10),
            "expected {point_n20_n10:?} outside bounds"
        );
    }

    #[test]
    fn get_at_point_successfully() {
        let matrix = DMatrix::from_iterator(3, 3, 0..9);

        assert_eq!(
            matrix.get_at_point(MatrixPoint::origin()),
            Some(0).as_ref()
        );
        assert_eq!(
            matrix.get_at_point(MatrixPoint::new(0, 2)),
            Some(2).as_ref()
        );
        assert_eq!(
            matrix.get_at_point(MatrixPoint::new(1, 0)),
            Some(3).as_ref()
        );
        assert_eq!(
            matrix.get_at_point(MatrixPoint::new(2, 2)),
            Some(8).as_ref()
        );
    }

    #[test]
    fn get_at_point_returns_none_with_point_outside_bounds() {
        let matrix = DMatrix::from_iterator(3, 3, 0..9);

        let point_12_25 = MatrixPoint::new(12, 25);
        let result_12_25 = matrix.get_at_point(point_12_25);
        assert!(
            result_12_25.is_none(),
            "got result for point {point_12_25:?}: {result_12_25:?}"
        );

        let point_n20_n10 = MatrixPoint::new(-20, -10);
        let result_n20_n10 = matrix.get_at_point(point_n20_n10);
        assert!(
            result_n20_n10.is_none(),
            "got result for point {point_n20_n10:?}: {result_n20_n10:?}"
        );
    }

    #[test]
    fn get_at_point_mut_successfully() {
        let mut matrix = DMatrix::from_iterator(3, 3, 0..9);
        let point = MatrixPoint::new(0, 1);
        let value_ref = matrix
            .get_at_point_mut(point)
            .expect("expected result for point");
        *value_ref = -5;
        assert_eq!(matrix.get_at_point(point), Some(-5).as_ref());
    }

    #[test]
    fn points_returns_all_points() {
        let matrix = DMatrix::from_iterator(3, 3, 0..9);
        let generated: HashSet<MatrixPoint> = matrix.points().collect();
        let expected: HashSet<MatrixPoint> = HashSet::from([
            Point2::new(0, 0),
            Point2::new(0, 1),
            Point2::new(0, 2),
            Point2::new(1, 0),
            Point2::new(1, 1),
            Point2::new(1, 2),
            Point2::new(2, 0),
            Point2::new(2, 1),
            Point2::new(2, 2),
        ]);
        assert_eq!(generated, expected);
    }
}
