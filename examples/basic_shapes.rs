use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let render_state = RenderState {
        camera: Camera::Camera2D {
            rotation: 0.,
            zoom: vec2(1. / 400., -1. / 400.),
            target: vec2(200., 200.),
            offset: vec2(0., 0.),
        },
        ..Default::default()
    };

    loop {
        clear_background(LIGHTGRAY);

        let mut c = scene_graph().sprite_layer(&render_state);
        let w = screen_width();
        draw_line(&mut c, 40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(&mut c, w / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(&mut c, w - 30.0, w - 30.0, 15.0, YELLOW);
        draw_text(&mut c, "HELLO", 20.0, 20.0, 30.0, DARKGRAY);

        scene_graph().draw_canvas(c);

        next_frame().await
    }
}
