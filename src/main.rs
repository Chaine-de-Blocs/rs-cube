use three_d::*;

fn main() {
    let window = Window::new("Cube", Some((400, 400))).unwrap();
    let context = window.gl();

    let mut camera = Camera::new_perspective(&context, vec3(0.0, 2.0, 2.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0), degrees(60.0), window.viewport().aspect(), 0.1, 10.0).unwrap();

    let positions: Vec<f32> = vec![
        /* Face 1 */
        -0.5, -0.55, 0.6,
        -0.5, 0.55, 0.6,
        0.5, -0.55, 0.6,
        -0.5, 0.55, 0.6,
        0.5, -0.55, 0.6,
        0.5, 0.55, 0.6,
        /* Face 2 */
        0.5, 0.5, 0.5,
        0.5, -0.5, 0.5,
        0.5, -0.5, -0.5,
        0.5, 0.5, 0.5,
        0.5, 0.5, -0.5,
        0.5, -0.5, -0.5,
        /* Face 3 */
        -0.5, 0.5, 0.5,
        0.5, 0.5, 0.5,
        0.5, 0.5, -0.5,
        -0.5, 0.5, 0.5,
        -0.5, 0.5, -0.5,
        0.5, 0.5, -0.5,
    ];
    let colors: Vec<u8> = vec![
        /* Face 1 */
        0, 112, 252, 255,
        0, 112, 252, 255,
        0, 112, 252, 255,
        0, 112, 252, 255,
        0, 112, 252, 255,
        0, 112, 252, 255,
        /* Face 2 */
        255, 255, 255, 255,
        255, 255, 255, 255,
        255, 255, 255, 255,
        255, 255, 255, 255,
        255, 255, 255, 255,
        255, 255, 255, 255,
        /* Face 3 */
        255, 255, 255, 255,
        255, 255, 255, 255,
        255, 255, 255, 255,
        255, 255, 255, 255,
        255, 255, 255, 255,
        255, 255, 255, 255,
    ];
    let cpu_mesh = CPUMesh {
        positions, colors: Some(colors), ..Default::default()
    };

    let mesh = Mesh::new(&context, &cpu_mesh).unwrap();

    window.render_loop(move |frame_input: FrameInput| {
        camera.set_aspect(frame_input.viewport.aspect()).unwrap();
        Screen::write(&context, &ClearState::color_and_depth(0.0, 0.0, 0.0, 1.0, 1.0), || {
            let transformation = Mat4::from_angle_y(radians((frame_input.accumulated_time * -0.00005) as f32));

            mesh.render_color(RenderStates::default(), frame_input.viewport, &transformation, &camera)?;
            Ok(())
        }).unwrap();

        FrameOutput::default()
    }).unwrap();
}