pub(crate) fn wrap(mut value: f32, min: f32, max: f32) -> f32 {
    debug_assert!(min < max, "Invalid bounds");
    let width = max - min;

    if value < min {
        value += width;

        while value < min {
            value += width;
        }
    } else if value > max {
        value -= width;

        while value > max {
            value -= width;
        }
    }

    value
}
