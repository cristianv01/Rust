
// TODO: Copy the `Point` struct and any related `impl` blocks here.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rectangle {
    // TODO: Add required fields here
}

impl Rectangle {
    // TODO: Implement required methods here
    //
    // The `from_opposite_corners` constructor receives any two diagonally opposite corners
    // of the rectangle to create.
    //
    // The `from_min_corner` constructor receives the corner with the smallest x and y position,
    // the width of the rectangle and the height of the rectangle
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle_creation_1() {
        let rect_a = Rectangle::from_opposite_corners(Point::new(0, 0), Point::new(100, 100));
        let rect_b = Rectangle::from_min_corner(Point::new(0, 0), 100, 100);

        assert_eq!(rect_a, rect_b);
    }

    #[test]
    fn test_rectangle_creation_2() {
        let rect_a = Rectangle::from_opposite_corners(Point::new(50, -15), Point::new(-20, 115));
        let rect_b = Rectangle::from_min_corner(Point::new(-20, -15), 70, 130);

        assert_eq!(rect_a, rect_b);
    }

    #[test]
    fn test_rectangle_methods() {
        let rect_a = Rectangle::from_opposite_corners(Point::new(50, -15), Point::new(-20, 115));

        assert_eq!(rect_a.x_min(), -20);
        assert_eq!(rect_a.x_max(), 50);
        assert_eq!(rect_a.y_min(), -15);
        assert_eq!(rect_a.y_max(), 115);
    }
}
