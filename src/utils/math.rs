pub(crate) fn lerp(start: f64, stop: f64, amount: f64) -> f64 {
    (1.0 - amount) * start + amount * stop
}

pub(crate) fn sanitize_degrees_int(degrees: i32) -> u32 {
    match degrees {
        value if value < 0 => value as u32 + 360,
        value => value as u32 % 360,
    }
}

pub(crate) fn sanitize_degrees_double(degrees: f64) -> f64 {
    match degrees {
        value if value < 0.0 => value + 360.0,
        value => value % 360.0,
    }
}

pub(crate) fn rotate_direction(from: f64, to: f64) -> f64 {
    let increasing_difference = sanitize_degrees_double(to - from);

    if increasing_difference <= 180.0 {
        1.0
    } else {
        -1.0
    }
}

pub(crate) fn difference_degrees(a: f64, b: f64) -> f64 {
    180.0 - ((a - b).abs() - 180.0).abs()
}

pub(crate) fn matrix_multiply(row: [f64; 3], matrix: [[f64; 3]; 3]) -> [f64; 3] {
    let a = row[0] * matrix[0][0] + row[1] * matrix[0][1] + row[2] * matrix[0][2];
    let b = row[0] * matrix[1][0] + row[1] * matrix[1][1] + row[2] * matrix[1][2];
    let c = row[0] * matrix[2][0] + row[1] * matrix[2][1] + row[2] * matrix[2][2];

    [a, b, c]
}
