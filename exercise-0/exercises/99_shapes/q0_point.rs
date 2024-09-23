#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    // TODO: Add required fields here
}

impl Point {
    // TODO: Implement required methods here
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_creation() {
        let x: i32 = 5;
        let y: i32 = 10;
        let point_a = Point { x, y };
        let point_b = Point::new(x, y);

        assert_eq!(point_a.x, point_b.x);
        assert_eq!(point_a.y, point_b.y);
        assert_eq!(point_a, point_b);
    }

    #[test]
    fn test_point_creation_2() {
        let point_a = Point { x: -15, y: 22 };
        let point_b = Point::new(-15, 21);

        assert_eq!(point_a.x, point_b.x);
        assert_ne!(point_a.y, point_b.y);
        assert_ne!(point_a, point_b);
    }
}
