use gl::COLOR_BUFFER_BIT;
use glfw_window_utility::glfw_error::GlfwError;
use glfw_window_utility::glfw_window::GlfwWindow;
use graphics::program::Program;
use graphics::vertex::pos_uv_norm_vertex::PositionUvNormalVertex;
use graphics::vertex_array::VertexArray;
use mathematics::linear_algebra::matrix::types::Mat4F32;
use mathematics::linear_algebra::vector::types::{Vector2F32, Vector3F32};

const BUFFER_B: &'static [PositionUvNormalVertex] = &[
    PositionUvNormalVertex {
        position: Vector3F32::new([100_f32, 0_f32, 0.0]),
        uv: Vector2F32::new([1.0, 0.0]),
        normal: Vector3F32::new([0_f32; 3])
    },
    PositionUvNormalVertex {
        position: Vector3F32::new([0_f32, 0_f32, 0.0]),
        uv: Vector2F32::new([1.0, 1.0]),
        normal: Vector3F32::new([0_f32; 3])
    },
    PositionUvNormalVertex {
        position: Vector3F32::new([0_f32, 100_f32, 0.0]),
        uv: Vector2F32::new([0.0, 1.0]),
        normal: Vector3F32::new([0_f32; 3])
    }
];


fn main() -> Result<(), GlfwError> {
    let vao = VertexArray::default();
    let program = Program::default();

    GlfwWindow::new([500; 2], "Grapher")
        .with_resize_viewport(true)
        .with_on_window_init({
            let vao = vao.clone();
            let program = program.clone();

            move |_| {
                {
                    program.load_from_source(
                        r#"
                        #version 330 core

                        uniform mat4 projection;

                        layout(location = 0) in vec3 aPos;
                        layout(location = 1) in vec2 aUv;

                        uniform vec2 mousePosition;

                        out vec2 Uv;

                        void main() {
                            gl_Position = projection * vec4(aPos + vec3(mousePosition, 0), 1);
                            Uv = aUv;
                        }
                    "#,
                        r#"
                        #version 330 core

                        out vec4 fragColor;

                        in vec2 Uv;

                        void main() {
                            fragColor = vec4(0, Uv, 1);
                        }
                    "#,
                    );
                }
                vao.set_vertices(&BUFFER_B, None);
            }
        })
        .with_on_window_render({
            let vao = vao.clone();
            let program = program.clone();

            move |_state| unsafe {
                let _time = _state.clock().elapsed() as f32;
                gl::ClearColor(1.0, 0.0, 0.0, 1.0);
                gl::Clear(COLOR_BUFFER_BIT);

                program.bind();
                vao.bind();

                let m2 = Mat4F32::orthographic(
                    0_f32,
                    _state.window_size()[0] as f32,
                    0_f32,
                    _state.window_size()[1] as f32,
                    -1000_f32,
                    1000_f32
                );

                program.bind_uniform("projection", &m2);
                let mouse = _state.input().mouse().relative_mouse_position();

                let mouse = Vector2F32::new([mouse[0] as f32, mouse[1] as f32]);
                program.bind_uniform("mousePosition", &mouse);

                gl::DrawArrays(gl::TRIANGLES, 0, 3);
            }
        })
        .run()
}