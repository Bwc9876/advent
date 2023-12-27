use crate::pos::Position;

/// Get the area of a polygon given its vertices.
///
/// This is the shoelace formula.
///
pub fn area(verts: &[Position]) -> isize {
    verts
        .windows(2)
        .map(|w| ((w[0].x) * (w[1].y)) - ((w[0].y) * (w[1].x)))
        .sum::<isize>()
        / 2
}

/// Get the perimeter of a polygon given its vertices.
///
/// This is the sum of the distances between each vertex.
///
pub fn perimeter(verts: &[Position]) -> isize {
    verts
        .windows(2)
        .map(|w| (w[0].x - w[1].x).abs() + (w[0].y - w[1].y).abs())
        .sum::<isize>()
}
