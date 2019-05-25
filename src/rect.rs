use cgmath::*;

#[derive(Copy, Clone, Debug)]
pub struct Rect {
    pub position: Vector2<f32>,
    pub bounds: Vector2<f32>,
}

impl Default for Rect {
    fn default() -> Self {
        Self {
            position: vec2(0.0, 0.0),
            bounds: vec2(0.0, 0.0),
        }
    }
}

impl Rect {
    pub fn new(position: Vector2<f32>, bounds: Vector2<f32>) -> Self {
        Self { position, bounds }
    }

    pub fn contains_point(&self, point: Vector2<f32>) -> bool {
        point.x >= self.position.x
            && point.x <= self.position.x + self.bounds.x
            && point.y >= self.position.y
            && point.y <= self.position.y + self.bounds.y
    }

    pub fn center_point(&self) -> Vector2<f32> {
        vec2(
            self.position.x + self.bounds.x / 2.0,
            self.position.y + self.bounds.y / 2.0,
        )
    }

    pub fn center_y(&self) -> Vector2<f32> {
        vec2(self.position.x, self.position.y + self.bounds.y / 2.0)
    }

    pub fn center_x(&self) -> Vector2<f32> {
        vec2(self.position.x + self.bounds.x / 2.0, self.position.y)
    }

    pub fn as_matrix_within_window(&self, window_dim: Vector2<f32>) -> Matrix4<f32> {
        let scale = Matrix4::from_nonuniform_scale(
            self.bounds.x / window_dim.x,
            self.bounds.y / window_dim.y,
            1.0,
        );

        let x_move =
            2.0 * (self.position.x + self.bounds.x / 2.0 - window_dim.x / 2.0) / window_dim.x;
        let y_move =
            -2.0 * (self.position.y + self.bounds.y / 2.0 - window_dim.y / 2.0) / window_dim.y;
        let translation = Matrix4::from_translation(vec3(x_move, y_move, 0.0));

        translation * scale // scale then translate
    }

    pub fn right_edge(&self) -> f32 {
        self.position.x + self.bounds.x
    }
}

#[test]
fn test_bounds_and_position_as_matrix() {
    fn vec4_from_2(vec: Vector2<f32>) -> Vector4<f32> {
        vec4(vec.x, vec.y, 1.0, 1.0)
    }

    fn bounds_and_position_as_matrix(
        window_dim: Vector2<f32>,
        bounds: Vector2<f32>,
        position: Vector2<f32>,
    ) -> Matrix4<f32> {
        let rect = Rect::new(position, bounds);
        rect.as_matrix_within_window(window_dim)
    }

    {
        // quad half the size of the screen, positioned at the top-left of the screen
        let transform =
            bounds_and_position_as_matrix(vec2(200.0, 100.0), vec2(100.0, 50.0), vec2(0.0, 0.0));

        assert_eq!(
            vec4_from_2(vec2(-1.0, 0.0)),
            transform * vec4_from_2(vec2(-1.0, -1.0)) // top-left coord
        );
        assert_eq!(
            vec4_from_2(vec2(0.0, 0.0)),
            transform * vec4_from_2(vec2(1.0, -1.0)) // top-right coord
        );
        assert_eq!(
            vec4_from_2(vec2(0.0, 1.0)),
            transform * vec4_from_2(vec2(1.0, 1.0)) // bottom-right coord
        );
        assert_eq!(
            vec4_from_2(vec2(-1.0, 1.0)),
            transform * vec4_from_2(vec2(-1.0, 1.0)) // bottom-left coord
        );
    }

    {
        // quad half the size of the screen, positioned at the bottom-right of the screen
        let transform =
            bounds_and_position_as_matrix(vec2(200.0, 100.0), vec2(100.0, 50.0), vec2(100.0, 50.0));

        assert_eq!(
            vec4_from_2(vec2(0.0, -1.0)),
            transform * vec4_from_2(vec2(-1.0, -1.0)) // top-left coord
        );
        assert_eq!(
            vec4_from_2(vec2(1.0, -1.0)),
            transform * vec4_from_2(vec2(1.0, -1.0)) // top-right coord
        );
        assert_eq!(
            vec4_from_2(vec2(1.0, 0.0)),
            transform * vec4_from_2(vec2(1.0, 1.0)) // bottom-right coord
        );
        assert_eq!(
            vec4_from_2(vec2(0.0, 0.0)),
            transform * vec4_from_2(vec2(-1.0, 1.0)) // bottom-left coord
        );
    }

    {
        let transform =
            bounds_and_position_as_matrix(vec2(100.0, 50.0), vec2(100.0, 50.0), vec2(0.0, 0.0));

        use crate::quad::*;
        // All points on the unit quad should remain the same for a quad filling the screen
        for vertex in &QUAD {
            let coord = vec4_from_2(vertex.pos.into());
            assert_eq!(coord, transform * coord);
        }
    }
}
