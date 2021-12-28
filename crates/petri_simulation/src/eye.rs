use crate::utils::*;
use crate::*;

use std::f32::consts::*;

/// How far our eye can see:
///
/// ```text
/// -----------------
/// |               |
/// |               |
/// |               |
/// |@      %      %|
/// |               |
/// |               |
/// |               |
/// -----------------
/// ```
///
/// If @ marks our creature and % marks food, then a FOV_RANGE of:
///
/// - 0.1 = 10% of the map = bird sees no foods (at least in this case)
/// - 0.5 = 50% of the map = bird sees one of the foods
/// - 1.0 = 100% of the map = bird sees both foods
const FOV_RANGE: f32 = 0.25;

/// How wide our eye can see.
///
/// If @> marks our creature (rotated to the right) and . marks the area
/// our creature sees, then a FOV_ANGLE of:
///
/// - PI/2 = 90° =
/// ```text
///   -----------------
///   |             /.|
///   |           /...|
///   |         /.....|
///   |       @>......|
///   |         \.....|
///   |           \...|
///   |             \.|
///   -----------------
/// ```
///
/// - PI = 180° =
/// ```text
///   -----------------
///   |       |.......|
///   |       |.......|
///   |       |.......|
///   |       @>......|
///   |       |.......|
///   |       |.......|
///   |       |.......|
///   -----------------
/// ```
///
/// - 2 * PI = 360° =
/// ```text
///   -----------------
///   |...............|
///   |...............|
///   |...............|
///   |.......@>......|
///   |...............|
///   |...............|
///   |...............|
///   -----------------
/// ```
///
/// Field of view depends on both FOV_RANGE and FOV_ANGLE:
///
/// - FOV_RANGE=0.4, FOV_ANGLE=PI/2:
/// ```text
///   -----------------
///   |       @       |
///   |     /.v.\     |
///   |   /.......\   |
///   |   ---------   |
///   |               |
///   |               |
///   |               |
///   -----------------
/// ```
///
/// - FOV_RANGE=0.5, FOV_ANGLE=2*PI:
/// ```text
///   -----------------
///   |               |
///   |      ---      |
///   |     /...\     |
///   |    |..@..|    |
///   |     \.../     |
///   |      ---      |
///   |               |
///   -----------------
/// ```
const FOV_ANGLE: f32 = PI + FRAC_PI_4;

/// How much photoreceptors there are in a single eye.
///
/// More cells means our birds will have more "crisp" vision, allowing
/// them to locate the food more precisely - but the trade-off is that
/// the evolution process will then take longer, or even fail, unable
/// to find any solution.
///
/// I've found values between 3~11 sufficient, with eyes having more
/// than ~20 photoreceptors yielding progressively worse results.
const CELLS: usize = 9;

#[derive(Component, Debug)]
pub struct Eye {
    pub fov_range: f32,
    pub fov_angle: f32,
    pub cells: usize,
}

impl Eye {
    pub fn new(fov_range: f32, fov_angle: f32, cells: usize) -> Self {
        assert!(fov_range > 0.0);
        assert!(fov_angle > 0.0);
        assert!(cells > 0);

        Self {
            fov_range,
            fov_angle,
            cells,
        }
    }

    pub fn perceive<'a>(
        &self,
        position: &Transform,
        targets: impl Iterator<Item = &'a Transform>,
    ) -> Vec<f32> {
        let mut cells = vec![0.0; self.cells];

        // A base X Vector in 2D space
        let x = Vec2::new(1.0, 0.0);

        let half_fov = self.fov_angle / 2.0;

        // Range of angles to the left and right of our eye that it can see
        let inside_fov = -half_fov..=half_fov;

        // Get the angle of where our eye is looking toward
        let (_, eye_angle) = position.rotation.to_axis_angle();
        // Get our eye's position in the 2D plane
        let eye_position = position.translation.truncate();

        for target in targets {
            // We only want to determine the 2D length/plane, so we get a Vec2 from here, which is
            // the diff Vector between our eye and its target. The Z axis is discarded
            let diff_vec = target.translation.truncate() - eye_position;

            let dist = diff_vec.length();

            // Initial check to see if target is within FOV range.
            if dist >= self.fov_range {
                continue;
            }

            // Work out the angle of the diff Vector
            let target_angle = x.angle_between(diff_vec);

            // Get the difference between angle to target and current angle of the eye
            let angle = wrap(target_angle - eye_angle, -PI, PI);

            // Check if the resulting angle is outside of the FOV
            if !inside_fov.contains(&angle) {
                continue;
            }

            // Encode which eye cell will "see" the target based on the angle within the FOV
            let cell = ((angle + half_fov) / self.fov_angle) * (self.cells as f32);
            let cell = (cell as usize).min(self.cells - 1);

            // Determine the cell response/energy based on a ratio of target distance to FOV range
            let energy = (self.fov_range - dist) / self.fov_range;

            cells[cell] += energy;
        }

        cells
    }
}

impl Default for Eye {
    fn default() -> Self {
        Eye::new(FOV_RANGE, FOV_ANGLE, CELLS)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_EYE_CELLS: usize = 13;

    struct TestCase {
        foods: Vec<Transform>,
        fov_range: f32,
        fov_angle: f32,
        x: f32,
        y: f32,
        rot: f32,
        expected_vision: &'static str,
    }

    impl TestCase {
        fn run(self) {
            let eye = Eye::new(self.fov_range, self.fov_angle, TEST_EYE_CELLS);

            let transform = Transform {
                translation: Vec3::new(self.x, self.y, 0.0),
                rotation: Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), self.rot),
                ..Default::default()
            };

            let actual_vision = eye.perceive(&transform, self.foods.iter());

            let actual_vision: Vec<_> = actual_vision
                .into_iter()
                .map(|cell| {
                    // As a reminder, the higher cell's value, the closer
                    // the food is:

                    if cell >= 0.7 {
                        // <0.7, 1.0>
                        // food is right in front of us
                        "#"
                    } else if cell >= 0.3 {
                        // <0.3, 0.7)
                        // food is somewhat further
                        "+"
                    } else if cell > 0.0 {
                        // <0.0, 0.3)
                        // food is pretty far away
                        "."
                    } else {
                        // 0.0
                        // no food in sight, this cell sees empty space
                        " "
                    }
                })
                .collect();

            let actual_vision = actual_vision.join("");

            // The finish line!
            assert_eq!(actual_vision, self.expected_vision);
        }
    }

    mod different_fov_ranges {
        use super::*;
        use test_case::test_case;

        #[test_case(1.0, "      +      ")] // Food is inside the FOV
        #[test_case(0.9, "      +      ")] // ditto
        #[test_case(0.8, "      +      ")] // ditto
        #[test_case(0.7, "      .      ")] // Food slowly disappears
        #[test_case(0.6, "      .      ")] // ditto
        #[test_case(0.5, "             ")] // Food disappeared!
        #[test_case(0.4, "             ")]
        #[test_case(0.3, "             ")]
        #[test_case(0.2, "             ")]
        #[test_case(0.1, "             ")]
        fn test(fov_range: f32, expected_vision: &'static str) {
            TestCase {
                foods: vec![Transform::from_xyz(1.0, 0.5, 0.0)],
                fov_angle: FRAC_2_PI,
                x: 0.5,
                y: 0.5,
                rot: 0.0,
                expected_vision,
                fov_range,
            }
            .run()
        }
    }

    mod different_rotations {
        use super::*;
        use test_case::test_case;

        #[test_case(0.00 * PI, "         +   ")] // Food is to our right
        #[test_case(0.25 * PI, "        +    ")]
        #[test_case(0.50 * PI, "      +      ")]
        #[test_case(0.75 * PI, "    +        ")]
        #[test_case(1.00 * PI, "   +         ")] // Food is behind us
        #[test_case(1.25 * PI, " +           ")] // (we continue to see it
        #[test_case(1.50 * PI, "+            ")] // due to 360° fov_angle.)
        #[test_case(1.75 * PI, "           + ")]
        #[test_case(2.00 * PI, "         +   ")] // Here we've done 360°
                                                 // #[test_case(2.25 * PI, "        +    ")] // (and a bit more, to
                                                 // #[test_case(2.50 * PI, "      +      ")] // prove the numbers wrap.)
        fn test(rot: f32, expected_vision: &'static str) {
            TestCase {
                foods: vec![Transform::from_xyz(0.5, 1.0, 0.0)],
                fov_range: 1.0,
                fov_angle: 2.0 * PI,
                x: 0.5,
                y: 0.5,
                rot,
                expected_vision,
            }
            .run()
        }
    }

    mod different_positions {
        use super::*;
        use test_case::test_case;

        // Checking the X axis:
        // (you can see the bird is "flying away" from the food)
        #[test_case(0.9, 0.5, "#           #")]
        #[test_case(0.8, 0.5, "  #       #  ")]
        #[test_case(0.7, 0.5, "   +     +   ")]
        #[test_case(0.6, 0.5, "    +   +    ")]
        #[test_case(0.5, 0.5, "    +   +    ")]
        #[test_case(0.4, 0.5, "     + +     ")]
        #[test_case(0.3, 0.5, "     . .     ")]
        #[test_case(0.2, 0.5, "     . .     ")]
        #[test_case(0.1, 0.5, "     . .     ")]
        #[test_case(0.0, 0.5, "             ")]
        //
        // Checking the Y axis:
        // (you can see the bird is "flying alongside" the food)
        #[test_case(0.5, 0.0, "            +")]
        #[test_case(0.5, 0.1, "          + .")]
        #[test_case(0.5, 0.2, "         +  +")]
        #[test_case(0.5, 0.3, "        + +  ")]
        #[test_case(0.5, 0.4, "      +  +   ")]
        #[test_case(0.5, 0.6, "   +  +      ")]
        #[test_case(0.5, 0.7, "  + +        ")]
        #[test_case(0.5, 0.8, "+  +         ")]
        #[test_case(0.5, 0.9, ". +          ")]
        #[test_case(0.5, 1.0, "+            ")]
        fn test(x: f32, y: f32, expected_vision: &'static str) {
            TestCase {
                foods: vec![
                    Transform::from_xyz(1.0, 0.4, 0.0),
                    Transform::from_xyz(1.0, 0.6, 0.0),
                ],
                fov_range: 1.0,
                fov_angle: FRAC_PI_2,
                rot: 0.0,
                x,
                y,
                expected_vision,
            }
            .run()
        }
    }

    mod different_fov_angles {
        use super::*;
        use test_case::test_case;

        #[test_case(0.25 * PI, " +         + ")] // FOV is narrow = 2 foods
        #[test_case(0.50 * PI, ".  +     +  .")]
        #[test_case(0.75 * PI, "  . +   + .  ")] // FOV gets progressively
        #[test_case(1.00 * PI, "   . + + .   ")] // wider and wider...
        #[test_case(1.25 * PI, "   . + + .   ")]
        #[test_case(1.50 * PI, ".   .+ +.   .")]
        #[test_case(1.75 * PI, ".   .+ +.   .")]
        #[test_case(2.00 * PI, "+.  .+ +.  .+")] // FOV is wide = 8 foods
        fn test(fov_angle: f32, expected_vision: &'static str) {
            TestCase {
                foods: vec![
                    Transform::from_xyz(0.0, 0.0, 0.0),
                    Transform::from_xyz(0.0, 0.33, 0.0),
                    Transform::from_xyz(0.0, 0.66, 0.0),
                    Transform::from_xyz(0.0, 1.0, 0.0),
                    Transform::from_xyz(1.0, 0.0, 0.0),
                    Transform::from_xyz(1.0, 0.33, 0.0),
                    Transform::from_xyz(1.0, 0.66, 0.0),
                    Transform::from_xyz(1.0, 1.0, 0.0),
                ],
                fov_range: 1.0,
                x: 0.5,
                y: 0.5,
                rot: 0.0,
                fov_angle,
                expected_vision,
            }
            .run()
        }
    }
}
