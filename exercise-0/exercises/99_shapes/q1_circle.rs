
// TODO: Copy the `Point` struct and any related `impl` blocks here.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Circle {
    // TODO: Add required fields here
}

impl Circle {
    // TODO: Implement required methods here
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_creation_1() {
        let circle_a = Circle {
            center: Point { x: 5, y: 10 },
            radius: 30_u16,
        };
        let circle_b = Circle::new(Point::new(5_i32, 10_i32), 30_u16);

        assert_eq!(circle_a.center.x, circle_b.center.x);
        assert_eq!(circle_a.center.y, circle_b.center.y);
        assert_eq!(circle_a.radius, circle_b.radius);
    }

    #[test]
    fn test_circle_creation_2() {
        let circle_a = Circle {
            center: Point {
                x: 5_i32,
                y: 10_i32,
            },
            radius: 15_u16,
        };
        let circle_b = Circle::new(Point::new(5, 11), 30_u16);

        assert_eq!(circle_a.center.x, circle_b.center.x);
        assert_ne!(circle_a.center.y, circle_b.center.y);
        assert_ne!(circle_a.radius, circle_b.radius);
    }

    #[test]
    fn test_circle_methods() {
        let circle = Circle::new(Point::new(7, 11), 200);
        assert_eq!(circle.cx(), 7);
        assert_ne!(circle.cx(), 10);
        assert_eq!(circle.cy(), 11);
        assert_ne!(circle.cy(), 10);
        assert_eq!(circle.radius(), 200);
        assert_ne!(circle.radius(), 1);
    }
}
