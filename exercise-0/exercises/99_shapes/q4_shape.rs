// TODO: Copy the `Point` struct and any related `impl` blocks here.

// TODO: Copy the `Circle` struct and any related `impl` blocks here.

// TODO: Copy the `Rectangle` struct and any related `impl` blocks here.

// TODO: Copy the `rect_overlap`, `circle_overlap`, and `circle_and_rect_overlap` functions here.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Shape {
    // TODO: Add required variants here
    // Hint: Check the slides and Rust book for ways to embed data in enum variants.
}

impl Shape {
    // TODO: Implement required methods here
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_creation_1() {
        let shape_a = Shape::circle(Point::new(5, 10), 30);
        let shape_b = Shape::circle(Point::new(5, 10), 30);
        assert_eq!(shape_a, shape_b);
    }

    #[test]
    fn test_shape_creation_2() {
        let shape_a = Shape::rectangle_from_min_corner(Point::new(5, 10), 30, 30);
        let shape_b = Shape::rectangle_from_opposite_corners(Point::new(35, 10), Point::new(5, 40));
        assert_eq!(shape_a, shape_b);
    }

    #[test]
    fn test_shape_overlap_1() {
        let shape_a = Shape::circle(Point::new(5, 10), 30);
        let shape_b = Shape::circle(Point::new(35, 10), 30);
        assert!(shape_a.overlap(&shape_b));
        assert!(shape_b.overlap(&shape_a));

        let shape_c = Shape::circle(Point::new(66, 10), 30);
        assert!(!shape_a.overlap(&shape_c));
        assert!(!shape_c.overlap(&shape_a));
        assert!(shape_b.overlap(&shape_c));
        assert!(shape_c.overlap(&shape_b));
    }

    #[test]
    fn test_shape_overlap_2() {
        let shape_a =
            Shape::rectangle_from_opposite_corners(Point::new(5, 10), Point::new(100, 150));
        let shape_b =
            Shape::rectangle_from_opposite_corners(Point::new(5, 10), Point::new(100, 150));
        assert!(shape_a.overlap(&shape_b));
        assert!(shape_b.overlap(&shape_a));
    }

    #[test]
    fn test_shape_overlap_3() {
        let shape_a = Shape::rectangle_from_min_corner(Point::new(-5, -5), 15, 15);
        let shape_b = Shape::rectangle_from_opposite_corners(Point::new(5, 5), Point::new(20, 30));
        assert!(shape_a.overlap(&shape_b));
        assert!(shape_b.overlap(&shape_a));

        let shape_c =
            Shape::rectangle_from_opposite_corners(Point::new(15, 15), Point::new(25, 25));
        assert!(!shape_a.overlap(&shape_c));
        assert!(!shape_c.overlap(&shape_a));
    }

    #[test]
    fn test_shape_overlap_4() {
        let shape_a = Shape::circle(Point::new(5, 10), 30);
        let shape_b =
            Shape::rectangle_from_opposite_corners(Point::new(5, 10), Point::new(100, 150));
        assert!(shape_a.overlap(&shape_b));
        assert!(shape_b.overlap(&shape_a));
    }

    #[test]
    fn test_shape_overlap_5() {
        let shape_a = Shape::circle(Point::new(5, 10), 30);
        let shape_b = Shape::rectangle_from_min_corner(Point::new(100, 100), 50, 50);
        assert!(!shape_a.overlap(&shape_b));
        assert!(!shape_b.overlap(&shape_a));
    }

    #[test]
    fn test_shape_overlap_6() {
        let shape_a = Shape::circle(Point::new(10, 10), 5);
        let shape_b = Shape::rectangle_from_min_corner(Point::new(5, 5), 5, 5);
        assert!(shape_a.overlap(&shape_b));
        assert!(shape_b.overlap(&shape_a));
    }

    #[test]
    fn test_shape_overlap_7() {
        let shape_a = Shape::circle(Point::new(10, 10), 50);
        let shape_b = Shape::rectangle_from_opposite_corners(Point::new(10, 10), Point::new(-5, 0));
        assert!(shape_a.overlap(&shape_b));
        assert!(shape_b.overlap(&shape_a));
    }
}
