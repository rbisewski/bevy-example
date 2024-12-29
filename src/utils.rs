/*
 * Generates a random number between the given range
 *
 * @param    u32   minimum
 * @param    u32   maximum
 *
 * @returns  u32   random number between min and max
 */
pub fn random(min: u32, max: u32) -> u32 {
    if min == 1 && max == 1 {
        return 1;
    } else if min == max {
        return max;
    }
    fastrand::u32(min..max)
}
