use graphics::types::{Rectangle, Vec2d};
use rand::prelude::*;

pub fn random_start(extents: Rectangle<f64>, border: f64) -> Vec2d<f64> {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(extents[0] + border, extents[2] - border);
    let y = rng.gen_range(extents[1] + border, extents[3] - border);

    [x, y]
}

pub fn collision(rect1: Rectangle<f64>, rect2: Rectangle<f64>) -> bool {
    rect1[0] < rect2[0] + rect2[2]
        && rect1[0] + rect1[2] > rect2[0]
        && rect1[1] < rect2[1] + rect2[3]
        && rect1[1] + rect1[3] > rect2[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_start() {
        let point = random_start([0.0, 0.0, 10.0, 10.0], 2.0);
        assert!(point[0] >= 1.0 && point[0] <= 8.0);
        assert!(point[1] >= 1.0 && point[1] <= 8.0);
    }

    #[test]
    fn test_collision() {
        /* +-+-+-+
         * | | | |
         * +-+-+-+
         */
        assert_eq!(collision([0.0, 0.0, 2.0, 2.0], [1.0, 0.0, 2.0, 2.0]), true);
        assert_eq!(collision([1.0, 0.0, 2.0, 2.0], [0.0, 0.0, 2.0, 2.0]), true);

        /* +---+
         * | +-+-+
         * +-+-+ |
         *   +---+
         */
        assert_eq!(collision([0.0, 0.0, 2.0, 2.0], [1.0, 1.0, 2.0, 2.0]), true);
        assert_eq!(collision([1.0, 1.0, 2.0, 2.0], [0.0, 0.0, 2.0, 2.0]), true);

        /* +----+
         * | ++ |
         * | ++ |
         * +----+
         */
        assert_eq!(collision([0.0, 0.0, 4.0, 4.0], [2.0, 2.0, 1.0, 1.0]), true);
        assert_eq!(collision([2.0, 2.0, 1.0, 1.0], [0.0, 0.0, 4.0, 4.0]), true);

        assert_eq!(collision([0.0, 0.0, 2.0, 2.0], [2.0, 0.0, 2.0, 2.0]), false);
        assert_eq!(collision([2.0, 2.0, 2.0, 2.0], [0.0, 0.0, 2.0, 2.0]), false);
    }
}
