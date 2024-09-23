
// TODO: Copy the `Point` struct and any related `impl` blocks here.

// TODO: Copy the `Circle` struct and any related `impl` blocks here.

// TODO: Copy the `Rectangle` struct and any related `impl` blocks here.

fn rect_overlap(rect_a: &Rectangle, rect_b: &Rectangle) -> bool {
    // TODO: Implement this function
}

fn circle_overlap(circle_a: &Circle, circle_b: &Circle) -> bool {
    // TODO: Implement this function
}

fn circle_and_rect_overlap(circle: &Circle, rect: &Rectangle) -> bool {
    // TODO: Implement this function
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_overlap_1() {
        let circle_a = Circle::new(Point::new(5, 10), 30);
        let circle_b = Circle::new(Point::new(5, 10), 30);
        assert!(circle_overlap(&circle_a, &circle_b));
    }

    #[test]
    fn test_circle_overlap_2() {
        let circle_a = Circle::new(Point::new(5, 10), 30);
        let circle_b = Circle::new(Point::new(35, 10), 30);
        assert!(circle_overlap(&circle_a, &circle_b));
        assert!(circle_overlap(&circle_b, &circle_a));

        let circle_c = Circle::new(Point::new(66, 10), 30);
        assert!(!circle_overlap(&circle_a, &circle_c));
        assert!(!circle_overlap(&circle_c, &circle_a));
        assert!(circle_overlap(&circle_b, &circle_c));
        assert!(circle_overlap(&circle_c, &circle_b));
    }

    #[test]
    fn test_rect_overlap_1() {
        let rect_a = Rectangle::from_opposite_corners(Point::new(5, 10), Point::new(100, 150));
        let rect_b = Rectangle::from_opposite_corners(Point::new(5, 10), Point::new(100, 150));
        assert!(rect_overlap(&rect_a, &rect_b));
        assert!(rect_overlap(&rect_b, &rect_a));
    }

    #[test]
    fn test_rect_overlap_2() {
        let rect_a = Rectangle::from_min_corner(Point::new(-5, -5), 15, 15);
        let rect_b = Rectangle::from_opposite_corners(Point::new(5, 5), Point::new(20, 30));
        assert!(rect_overlap(&rect_a, &rect_b));
        assert!(rect_overlap(&rect_b, &rect_a));

        let rect_c = Rectangle::from_opposite_corners(Point::new(15, 15), Point::new(25, 25));
        assert!(!rect_overlap(&rect_a, &rect_c));
        assert!(!rect_overlap(&rect_c, &rect_a));
    }

    #[test]
    fn test_rect_overlap_3() {
        let rect_a = Rectangle::from_opposite_corners(Point::new(5, 5), Point::new(10, 10));
        let rect_b = Rectangle::from_opposite_corners(Point::new(7, 7), Point::new(15, 15));
        assert!(rect_overlap(&rect_a, &rect_b));
        assert!(rect_overlap(&rect_b, &rect_a));
    }

    #[test]
    fn test_rect_overlap_4() {
        let rect_a = Rectangle::from_opposite_corners(Point::new(5, 5), Point::new(10, 10));
        let rect_b = Rectangle::from_opposite_corners(Point::new(7, 7), Point::new(0, 14));
        assert!(rect_overlap(&rect_a, &rect_b));
        assert!(rect_overlap(&rect_b, &rect_a));
    }

    #[test]
    fn test_rect_overlap_5() {
        let rect_a = Rectangle::from_opposite_corners(Point::new(5, 5), Point::new(10, 10));
        let rect_b = Rectangle::from_opposite_corners(Point::new(0, 0), Point::new(20, 20));
        assert!(rect_overlap(&rect_a, &rect_b));
        assert!(rect_overlap(&rect_b, &rect_a));
    }

    #[test]
    fn test_rect_overlap_6() {
        let rect_a = Rectangle::from_opposite_corners(Point::new(0, 0), Point::new(5, 5));
        let rect_b = Rectangle::from_opposite_corners(Point::new(-5, -5), Point::new(-1, -1));
        assert!(!rect_overlap(&rect_a, &rect_b));
        assert!(!rect_overlap(&rect_b, &rect_a));
    }

    #[test]
    fn test_rect_overlap_7() {
        let rect_a = Rectangle::from_opposite_corners(Point::new(0, 0), Point::new(5, 5));
        let rect_b = Rectangle::from_opposite_corners(Point::new(-5, -5), Point::new(5, 0));
        assert!(!rect_overlap(&rect_a, &rect_b));
        assert!(!rect_overlap(&rect_b, &rect_a));
    }

    #[test]
    fn test_rect_overlap_8() {
        let rect_a = Rectangle::from_opposite_corners(Point::new(0, 0), Point::new(5, 5));
        let rect_b = Rectangle::from_opposite_corners(Point::new(6, 6), Point::new(10, 5));
        assert!(!rect_overlap(&rect_a, &rect_b));
        assert!(!rect_overlap(&rect_b, &rect_a));
    }

    #[test]
    fn test_rect_overlap_9() {
        let rect_a = Rectangle::from_opposite_corners(Point::new(0, 0), Point::new(5, 5));
        let rect_b = Rectangle::from_opposite_corners(Point::new(6, 6), Point::new(12, 0));
        assert!(!rect_overlap(&rect_a, &rect_b));
        assert!(!rect_overlap(&rect_b, &rect_a));
    }

    #[test]
    fn test_circle_rect_overlap_1() {
        let circle = Circle::new(Point::new(5, 10), 30);
        let rect = Rectangle::from_opposite_corners(Point::new(5, 10), Point::new(100, 150));
        assert!(circle_and_rect_overlap(&circle, &rect));
    }

    #[test]
    fn test_circle_rect_overlap_2() {
        let circle = Circle::new(Point::new(5, 10), 30);
        let rect = Rectangle::from_min_corner(Point::new(100, 100), 50, 50);
        assert!(!circle_and_rect_overlap(&circle, &rect));
    }

    #[test]
    fn test_circle_rect_overlap_3() {
        let circle = Circle::new(Point::new(10, 10), 5);
        let rect = Rectangle::from_min_corner(Point::new(5, 5), 5, 5);
        assert!(circle_and_rect_overlap(&circle, &rect));
    }

    #[test]
    fn test_circle_rect_overlap_4() {
        let circle = Circle::new(Point::new(10, 10), 50);
        let rect = Rectangle::from_opposite_corners(Point::new(10, 10), Point::new(-5, 0));
        assert!(circle_and_rect_overlap(&circle, &rect));
    }

    #[test]
    fn test_circle_rect_overlap_5() {
        let circle = Circle::new(Point::new(10, 10), 50);
        let rect = Rectangle::from_opposite_corners(Point::new(10, 10), Point::new(-5, -50));
        assert!(circle_and_rect_overlap(&circle, &rect));
    }
}
