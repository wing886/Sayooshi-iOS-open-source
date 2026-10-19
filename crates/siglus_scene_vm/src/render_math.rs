use crate::layer::Sprite;

#[derive(Debug, Clone, Copy)]
pub struct ProjectedPoint {
    pub x: f32,
    pub y: f32,
    pub depth: f32,
    /// Homogeneous clip-space W. 2D paths use 1.0; the 3D camera path uses
    /// camera-space Z, matching D3DX perspective projection. The renderer
    /// keeps this value so UV/world varyings remain perspective-correct even
    /// though the quad's screen position is prepared on the CPU.
    pub clip_w: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct SpriteQuadGeometry {
    pub projected: [ProjectedPoint; 4],
    pub world: [[f32; 3]; 4],
}

#[derive(Debug, Clone, Copy)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }

    fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    fn cross(self, rhs: Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    fn normalize(self) -> Self {
        let len2 = self.dot(self);
        if len2 <= f32::EPSILON {
            return Self::new(0.0, 0.0, 0.0);
        }
        let inv = len2.sqrt().recip();
        Self::new(self.x * inv, self.y * inv, self.z * inv)
    }
}

fn rotate_x(v: Vec3, angle: f32) -> Vec3 {
    let (s, c) = angle.sin_cos();
    Vec3::new(v.x, v.y * c - v.z * s, v.y * s + v.z * c)
}

fn rotate_y(v: Vec3, angle: f32) -> Vec3 {
    let (s, c) = angle.sin_cos();
    Vec3::new(v.x * c + v.z * s, v.y, -v.x * s + v.z * c)
}

fn rotate_z(v: Vec3, angle: f32) -> Vec3 {
    let (s, c) = angle.sin_cos();
    Vec3::new(v.x * c - v.y * s, v.x * s + v.y * c, v.z)
}

fn uses_3d(sprite: &Sprite) -> bool {
    sprite.billboard
        || sprite.camera_enabled
        || sprite.z.abs() > f32::EPSILON
        || sprite.pivot_z.abs() > f32::EPSILON
        || (sprite.scale_z - 1.0).abs() > 1e-6
        || sprite.rotate_x.abs() > f32::EPSILON
        || sprite.rotate_y.abs() > f32::EPSILON
}

fn transform_local_point(sprite: &Sprite, px: f32, py: f32, dst_x: f32, dst_y: f32) -> Vec3 {
    let (anchor_x, anchor_y, mut p) = if sprite.object_anchor {
        (
            dst_x,
            dst_y,
            Vec3::new(
                px - sprite.texture_center_x - sprite.pivot_x,
                py - sprite.texture_center_y - sprite.pivot_y,
                -sprite.pivot_z,
            ),
        )
    } else {
        (
            dst_x + sprite.pivot_x,
            dst_y + sprite.pivot_y,
            Vec3::new(px - sprite.pivot_x, py - sprite.pivot_y, -sprite.pivot_z),
        )
    };
    p.x *= sprite.scale_x;
    p.y *= sprite.scale_y;
    p.z *= sprite.scale_z;
    // D3DXMatrixRotationYawPitchRoll(yaw, pitch, roll) applies roll,
    // then pitch, then yaw to row vectors.
    p = rotate_z(p, sprite.rotate);
    p = rotate_x(p, sprite.rotate_x);
    p = rotate_y(p, sprite.rotate_y);
    p.add(Vec3::new(anchor_x, anchor_y, sprite.z + sprite.pivot_z))
}

fn camera_basis(sprite: &Sprite) -> (Vec3, Vec3, Vec3, Vec3) {
    let eye = Vec3::new(
        sprite.camera_eye[0],
        sprite.camera_eye[1],
        sprite.camera_eye[2],
    );
    let target = Vec3::new(
        sprite.camera_target[0],
        sprite.camera_target[1],
        sprite.camera_target[2],
    );
    let up = Vec3::new(
        sprite.camera_up[0],
        sprite.camera_up[1],
        sprite.camera_up[2],
    );
    let forward = target.sub(eye).normalize();
    let right = up.cross(forward).normalize();
    let up2 = forward.cross(right).normalize();
    (eye, forward, right, up2)
}

fn transform_billboard_point(sprite: &Sprite, px: f32, py: f32, dst_x: f32, dst_y: f32) -> Vec3 {
    let (_, _, right, up) = camera_basis(sprite);
    let lx = (px - sprite.pivot_x) * sprite.scale_x;
    // Texture-space Y grows downward, while WORLD Y grows upward.
    let ly = -(py - sprite.pivot_y) * sprite.scale_y;
    let (s, c) = sprite.rotate.sin_cos();
    let rx = lx * c - ly * s;
    let ry = lx * s + ly * c;
    let anchor = Vec3::new(
        dst_x + sprite.pivot_x,
        dst_y + sprite.pivot_y,
        sprite.z + sprite.pivot_z,
    );
    anchor.add(Vec3::new(
        right.x * rx + up.x * ry,
        right.y * rx + up.y * ry,
        right.z * rx + up.z * ry,
    ))
}

fn project_point(sprite: &Sprite, p: Vec3, win_w: f32, win_h: f32) -> Option<ProjectedPoint> {
    if !sprite.camera_enabled {
        let depth = (0.5 - p.z / 100000.0).clamp(0.0, 1.0);
        return Some(ProjectedPoint {
            x: p.x,
            y: p.y,
            depth,
            clip_w: 1.0,
        });
    }

    let (eye, zaxis, xaxis, yaxis) = camera_basis(sprite);

    let rel = p.sub(eye);
    let cx = rel.dot(xaxis);
    let cy = rel.dot(yaxis);
    let cz = rel.dot(zaxis);
    if cz <= 1e-3 {
        return None;
    }

    let aspect = if win_h.abs() > f32::EPSILON {
        win_w / win_h
    } else {
        1.0
    };
    let vfov = sprite
        .camera_view_angle_deg
        .to_radians()
        .clamp(1e-3, std::f32::consts::PI - 1e-3);
    let tan_half_v = (vfov * 0.5).tan().max(1e-3);
    let tan_half_h = (tan_half_v * aspect.max(1e-3)).max(1e-3);

    let x_ndc = cx / (cz * tan_half_h);
    let y_ndc = cy / (cz * tan_half_v);

    let sx = (x_ndc + 1.0) * 0.5 * win_w;
    let sy = (1.0 - y_ndc) * 0.5 * win_h;
    let near = 1.0;
    let far = 10000.0;
    let depth = (far / (far - near) - near * far / ((far - near) * cz)).clamp(0.0, 1.0);
    Some(ProjectedPoint {
        x: sx,
        y: sy,
        depth,
        clip_w: cz,
    })
}

fn signed_area(a: (f32, f32), b: (f32, f32), c: (f32, f32)) -> f32 {
    (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)
}

pub fn sprite_quad_geometry_rect(
    sprite: &Sprite,
    dst_x: f32,
    dst_y: f32,
    local_left: f32,
    local_top: f32,
    local_right: f32,
    local_bottom: f32,
    win_w: f32,
    win_h: f32,
) -> Option<SpriteQuadGeometry> {
    let xf = if sprite.billboard {
        transform_billboard_point as fn(&Sprite, f32, f32, f32, f32) -> Vec3
    } else {
        transform_local_point as fn(&Sprite, f32, f32, f32, f32) -> Vec3
    };

    let w0 = xf(sprite, local_left, local_top, dst_x, dst_y);
    let w1 = xf(sprite, local_right, local_top, dst_x, dst_y);
    let w2 = xf(sprite, local_right, local_bottom, dst_x, dst_y);
    let w3 = xf(sprite, local_left, local_bottom, dst_x, dst_y);
    let world = [
        [w0.x, w0.y, w0.z],
        [w1.x, w1.y, w1.z],
        [w2.x, w2.y, w2.z],
        [w3.x, w3.y, w3.z],
    ];

    if !uses_3d(sprite) {
        return Some(SpriteQuadGeometry {
            projected: [
                ProjectedPoint {
                    x: w0.x,
                    y: w0.y,
                    depth: 0.0,
                    clip_w: 1.0,
                },
                ProjectedPoint {
                    x: w1.x,
                    y: w1.y,
                    depth: 0.0,
                    clip_w: 1.0,
                },
                ProjectedPoint {
                    x: w2.x,
                    y: w2.y,
                    depth: 0.0,
                    clip_w: 1.0,
                },
                ProjectedPoint {
                    x: w3.x,
                    y: w3.y,
                    depth: 0.0,
                    clip_w: 1.0,
                },
            ],
            world,
        });
    }

    let p0 = project_point(sprite, w0, win_w, win_h)?;
    let p1 = project_point(sprite, w1, win_w, win_h)?;
    let p2 = project_point(sprite, w2, win_w, win_h)?;
    let p3 = project_point(sprite, w3, win_w, win_h)?;

    if sprite.culling && signed_area((p0.x, p0.y), (p1.x, p1.y), (p2.x, p2.y)) <= 0.0 {
        return None;
    }

    Some(SpriteQuadGeometry {
        projected: [p0, p1, p2, p3],
        world,
    })
}

pub fn sprite_quad_points_rect(
    sprite: &Sprite,
    dst_x: f32,
    dst_y: f32,
    local_left: f32,
    local_top: f32,
    local_right: f32,
    local_bottom: f32,
    win_w: f32,
    win_h: f32,
) -> Option<[ProjectedPoint; 4]> {
    sprite_quad_geometry_rect(
        sprite,
        dst_x,
        dst_y,
        local_left,
        local_top,
        local_right,
        local_bottom,
        win_w,
        win_h,
    )
    .map(|quad| quad.projected)
}

pub fn sprite_quad_points(
    sprite: &Sprite,
    dst_x: f32,
    dst_y: f32,
    dst_w: f32,
    dst_h: f32,
    win_w: f32,
    win_h: f32,
) -> Option<[ProjectedPoint; 4]> {
    sprite_quad_points_rect(sprite, dst_x, dst_y, 0.0, 0.0, dst_w, dst_h, win_w, win_h)
}

pub fn project_model_point(
    sprite: &Sprite,
    local: [f32; 3],
    anchor_x: f32,
    anchor_y: f32,
    win_w: f32,
    win_h: f32,
) -> Option<ProjectedPoint> {
    let mut p = Vec3::new(
        local[0] - sprite.pivot_x,
        local[1] - sprite.pivot_y,
        local[2] - sprite.pivot_z,
    );
    p.x *= sprite.scale_x;
    p.y *= sprite.scale_y;
    p.z *= sprite.scale_z;
    p = rotate_z(p, sprite.rotate);
    p = rotate_x(p, sprite.rotate_x);
    p = rotate_y(p, sprite.rotate_y);
    p = p.add(Vec3::new(
        anchor_x + sprite.pivot_x,
        anchor_y + sprite.pivot_y,
        sprite.z + sprite.pivot_z,
    ));
    project_point(sprite, p, win_w, win_h)
}

#[cfg(test)]
mod tests {
    use super::{Vec3, project_point, rotate_x, rotate_y, rotate_z, sprite_quad_points};
    use crate::layer::Sprite;

    fn camera_sprite() -> Sprite {
        Sprite {
            camera_enabled: true,
            camera_eye: [0.0, 0.0, 0.0],
            camera_target: [0.0, 0.0, 1.0],
            camera_up: [0.0, 1.0, 0.0],
            camera_view_angle_deg: 60.0,
            ..Default::default()
        }
    }

    #[test]
    fn yaw_pitch_roll_matches_d3dx_application_order() {
        let quarter = std::f32::consts::FRAC_PI_2;
        let mut point = Vec3::new(1.0, 2.0, 3.0);
        point = rotate_z(point, quarter);
        point = rotate_x(point, quarter);
        point = rotate_y(point, quarter);
        assert!((point.x - 1.0).abs() < 1e-5);
        assert!((point.y + 3.0).abs() < 1e-5);
        assert!((point.z - 2.0).abs() < 1e-5);
    }

    #[test]
    fn billboard_texture_top_projects_above_its_bottom() {
        let mut sprite = camera_sprite();
        sprite.billboard = true;
        sprite.z = 100.0;
        let quad = sprite_quad_points(&sprite, 0.0, 0.0, 32.0, 64.0, 640.0, 480.0).unwrap();
        assert!(quad[0].y < quad[3].y);
    }

    #[test]
    fn world_view_angle_is_vertical_and_does_not_change_with_window_width() {
        let sprite = camera_sprite();
        let point = Vec3::new(0.0, 1.0, 10.0);
        let narrow = project_point(&sprite, point, 640.0, 480.0).unwrap();
        let wide = project_point(&sprite, point, 1280.0, 480.0).unwrap();
        assert!((narrow.y - wide.y).abs() < 1e-4);
    }

    #[test]
    fn world_depth_matches_d3d_lh_near_and_far_planes() {
        let sprite = camera_sprite();
        let near = project_point(&sprite, Vec3::new(0.0, 0.0, 1.0), 640.0, 480.0).unwrap();
        let far = project_point(&sprite, Vec3::new(0.0, 0.0, 10000.0), 640.0, 480.0).unwrap();
        assert!(near.depth.abs() < 1e-6);
        assert!((far.depth - 1.0).abs() < 1e-6);
    }
}
