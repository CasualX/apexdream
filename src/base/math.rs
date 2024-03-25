#![allow(dead_code)]

pub fn add(lhs: [f32; 3], rhs: [f32; 3]) -> [f32; 3] {
    [lhs[0] + rhs[0], lhs[1] + rhs[1], lhs[2] + rhs[2]]
}
pub fn sub(lhs: [f32; 3], rhs: [f32; 3]) -> [f32; 3] {
    [lhs[0] - rhs[0], lhs[1] - rhs[1], lhs[2] - rhs[2]]
}
pub fn mul(lhs: [f32; 3], rhs: [f32; 3]) -> [f32; 3] {
    [lhs[0] * rhs[0], lhs[1] * rhs[1], lhs[2] * rhs[2]]
}
pub fn muls(v: [f32; 3], s: f32) -> [f32; 3] {
    [v[0] * s, v[1] * s, v[2] * s]
}
pub fn sqr(v: [f32; 3]) -> [f32; 3] {
    mul(v, v)
}
pub fn hadd(v: [f32; 3]) -> f32 {
    v[0] + v[1] + v[2]
}
pub fn dist(lhs: [f32; 3], rhs: [f32; 3]) -> f32 {
    dist2(lhs, rhs).sqrt()
}
pub fn dist2(lhs: [f32; 3], rhs: [f32; 3]) -> f32 {
    len2(sub(lhs, rhs))
}
pub fn len(v: [f32; 3]) -> f32 {
    len2(v).sqrt()
}
pub fn len2(v: [f32; 3]) -> f32 {
    v[0] * v[0] + v[1] * v[1] + v[2] * v[2]
}
pub fn norm(v: [f32; 3]) -> [f32; 3] {
    let len = len(v);
    if len > 0.0 {
        muls(v, 1.0 / len)
    } else {
        v
    }
}
pub fn lerp(start: [f32; 3], end: [f32; 3], t: f32) -> [f32; 3] {
    [
        start[0] + (end[0] - start[0]) * t,
        start[1] + (end[1] - start[1]) * t,
        start[2] + (end[2] - start[2]) * t,
    ]
}
pub fn dot(lhs: [f32; 3], rhs: [f32; 3]) -> f32 {
    lhs[0] * rhs[0] + lhs[1] * rhs[1] + lhs[2] * rhs[2]
}

pub fn qangle(v: [f32; 3]) -> [f32; 3] {
    let tmp;
    let yaw;
    let pitch;

    if v[1] == 0.0 && v[0] == 0.0 {
        yaw = 0.0;
        if v[2] > 0.0 {
            pitch = 270.0;
        } else {
            pitch = 90.0;
        }
    } else {
        yaw = f32::atan2(v[1], v[0]).to_degrees();
        tmp = (v[0] * v[0] + v[1] * v[1]).sqrt();
        pitch = f32::atan2(-v[2], tmp).to_degrees();
    }
    [pitch, yaw, 0.0]
}
pub fn qvec(v: [f32; 3]) -> [f32; 3] {
    let pitch = v[0].to_radians();
    let (sp, cp) = pitch.sin_cos();
    let yaw = v[1].to_radians();
    let (sy, cy) = yaw.sin_cos();
    [cp * cy, cp * sy, -sp]
}
pub fn qnorm(v: [f32; 3]) -> [f32; 3] {
    let mut pitch = v[0];
    while pitch <= -180.0 {
        pitch += 360.0;
    }
    while pitch > 180.0 {
        pitch -= 360.0;
    }
    let mut yaw = v[1];
    while yaw <= -180.0 {
        yaw += 360.0;
    }
    while yaw > 180.0 {
        yaw -= 360.0;
    }
    [pitch, yaw, 0.0]
}
pub fn qdiff(lhs: [f32; 3], rhs: [f32; 3]) -> f32 {
    let delta = qnorm(sub(lhs, rhs));
    (delta[0] * delta[0] + delta[1] * delta[1]).sqrt()
}
pub fn smoothstep(x: f32) -> f32 {
    (x * x) * (3.0 - (x + x))
}
// Projects a point onto a line
pub fn project(a: [f32; 3], d: [f32; 3], p: [f32; 3]) -> [f32; 3] {
    let v = sub(p, a);
    let t = dot(v, d);
    add(a, muls(d, t))
}

pub fn inverse(mat: &mut [f32; 16]) -> Option<&mut [f32; 16]> {
    let v00 = mat[0];
    let v01 = mat[1];
    let v02 = mat[2];
    let v03 = mat[3];
    let v10 = mat[4];
    let v11 = mat[5];
    let v12 = mat[6];
    let v13 = mat[7];
    let v20 = mat[8];
    let v21 = mat[9];
    let v22 = mat[10];
    let v23 = mat[11];
    let v30 = mat[12];
    let v31 = mat[13];
    let v32 = mat[14];
    let v33 = mat[15];

    let tmp00 = v00 * v11 - v01 * v10;
    let tmp01 = v00 * v12 - v02 * v10;
    let tmp02 = v00 * v13 - v03 * v10;
    let tmp03 = v01 * v12 - v02 * v11;
    let tmp04 = v01 * v13 - v03 * v11;
    let tmp05 = v02 * v13 - v03 * v12;
    let tmp06 = v20 * v31 - v21 * v30;
    let tmp07 = v20 * v32 - v22 * v30;
    let tmp08 = v20 * v33 - v23 * v30;
    let tmp09 = v21 * v32 - v22 * v31;
    let tmp10 = v21 * v33 - v23 * v31;
    let tmp11 = v22 * v33 - v23 * v32;

    let det = tmp00 * tmp11 - tmp01 * tmp10 + tmp02 * tmp09 + tmp03 * tmp08 - tmp04 * tmp07
        + tmp05 * tmp06;

    if det.abs() <= 0.000001 {
        return None;
    }
    let det_inv = 1.0 / det;

    mat[0] = (v11 * tmp11 - v12 * tmp10 + v13 * tmp09) * det_inv;
    mat[1] = (v02 * tmp10 - v01 * tmp11 - v03 * tmp09) * det_inv;
    mat[2] = (v31 * tmp05 - v32 * tmp04 + v33 * tmp03) * det_inv;
    mat[3] = (v22 * tmp04 - v21 * tmp05 - v23 * tmp03) * det_inv;
    mat[4] = (v12 * tmp08 - v10 * tmp11 - v13 * tmp07) * det_inv;
    mat[5] = (v00 * tmp11 - v02 * tmp08 + v03 * tmp07) * det_inv;
    mat[6] = (v32 * tmp02 - v30 * tmp05 - v33 * tmp01) * det_inv;
    mat[7] = (v20 * tmp05 - v22 * tmp02 + v23 * tmp01) * det_inv;
    mat[8] = (v10 * tmp10 - v11 * tmp08 + v13 * tmp06) * det_inv;
    mat[9] = (v01 * tmp08 - v00 * tmp10 - v03 * tmp06) * det_inv;
    mat[10] = (v30 * tmp04 - v31 * tmp02 + v33 * tmp00) * det_inv;
    mat[11] = (v21 * tmp02 - v20 * tmp04 - v23 * tmp00) * det_inv;
    mat[12] = (v11 * tmp07 - v10 * tmp09 - v12 * tmp06) * det_inv;
    mat[13] = (v00 * tmp09 - v01 * tmp07 + v02 * tmp06) * det_inv;
    mat[14] = (v31 * tmp01 - v30 * tmp03 - v32 * tmp00) * det_inv;
    mat[15] = (v20 * tmp03 - v21 * tmp01 + v22 * tmp00) * det_inv;

    Some(mat)
}
