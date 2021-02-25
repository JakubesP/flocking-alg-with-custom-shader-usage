#[macro_use]
extern crate lazy_static;
extern crate swgl;
use wasm_bindgen::prelude::*;

use swgl::camera2d::interface::CameraType;
use swgl::camera2d::ratio_view::RatioView;
use swgl::gl_wrapper::basics::clear_canvas;
use swgl::gl_wrapper::shader::{split_vfshader_to_shader_source, Program};
use swgl::gl_wrapper::vertex_array_object::PrimitiveType;
use swgl::global_tools::vector2::Vector2;
use swgl::graphics_2d::color::Color;
use swgl::graphics_2d::renderer::geometry_renderer::GeometryRenderer;
use swgl::graphics_2d::renderer::renderer_conf::RendererConf;
use swgl::graphics_2d::vertex_2d::predefined::color_vertex2d::ColorVertex2D;
use swgl::resources_loader;

mod app_state;
mod gl_setup;

mod flocking;
use flocking::Flock;
// -----------------------------------------------------------------------------------------

const DISPLAY_SIZE: f32 = 1000.0;
const BORDER_THICK: f32 = 50.0;
const BORDER_COLOR: u32 = 0x222222ff;
const OUTLINE_COLOR: u32 = 0xffffffff;

// -----------------------------------------------------------------------------------------

#[wasm_bindgen]
pub struct AppState {
    context: swgl::AppContext,

    last_tick: f32,
    time: f32,

    camera: RatioView,

    flock: Flock,
    batch_renderer: GeometryRenderer<ColorVertex2D>,
}

#[wasm_bindgen]
impl AppState {
    #[wasm_bindgen(constructor)]
    pub async fn new(last_tick: f32, width: f32, height: f32) -> Self {
        // ----------------------------- init webgl ---------------------------

        console_error_panic_hook::set_once();
        let (_, context) =
            gl_setup::initialize_webgl_context("#canvas", Color::from_hex(0x222222ff))
                .expect("Cannot initialize WebGL");

        // ----------------------------- load resources -----------------------

        let shader_file_path = "static/my_shader.glsl";
        let loaded_resource = resources_loader::get_files(&[shader_file_path])
            .await
            .unwrap();
        let shader_code =
            resources_loader::unwrap_text_content(&loaded_resource[shader_file_path]).unwrap();

        // ----------------------------- prepare objects ----------------------

        let camera = RatioView::new(Vector2::new(width, height), DISPLAY_SIZE);

        let batch_renderer = GeometryRenderer::init_with_custom_shader(
            &context,
            400,
            Program::new(&context, &split_vfshader_to_shader_source(&shader_code)).unwrap(),
            RendererConf::default(),
        )
        .unwrap();

        let flock = flocking::Flock::new(100, camera.scene_relative_size).unwrap();

        // ----------------------------- construct app ------------------------
        Self {
            context,
            last_tick,
            time: 0.0,
            camera,
            flock,
            batch_renderer,
        }
    }

    pub fn update(&mut self, time: f32, width: f32, height: f32) -> Result<(), JsValue> {
        app_state::update_dynamic_data(time, height, width);
        self.camera.update_canvas_size(Vector2::new(width, height));

        let now = time;
        let dt = (now - self.last_tick) / 1000.0;
        self.last_tick = now;
        self.time = time;

        let current_state = app_state::get_curr_state();
        let mouse_pos = self
            .camera
            .map_pixel_coords_to_game_coords(&current_state.mouse_pos);

        self.flock.update(
            dt,
            self.camera.scene_relative_size,
            BORDER_THICK,
            &mouse_pos,
        );

        Ok(())
    }

    pub fn render(&mut self) {
        clear_canvas(&self.context);

        let alpha_value = (self.time / 500.0).sin() / 2.0 + 0.5;

        self.batch_renderer
            .program()
            .set1f(&self.context, "alpha_value", alpha_value)
            .unwrap();
        self.flock
            .draw(&self.context, &self.batch_renderer, &self.camera);

        self.batch_renderer
            .program()
            .set1f(&self.context, "alpha_value", 1.0)
            .unwrap();
        self.batch_renderer
            .draw(
                &self.context,
                &border_vertices(),
                PrimitiveType::Triangles,
                &self.camera,
            )
            .unwrap();
        self.batch_renderer
            .draw(
                &self.context,
                &outline_vertices(),
                PrimitiveType::LineLoop,
                &self.camera,
            )
            .unwrap();
    }
}

// -----------------------------------------------------------------------------------------

// I know, it is long :)
fn border_vertices() -> [ColorVertex2D; 24] {
    let color = Color::from_hex(BORDER_COLOR);

    [
        // left border
        ColorVertex2D::new(Vector2::new(0.0, 0.0), color, 0.0),
        ColorVertex2D::new(Vector2::new(BORDER_THICK, 0.0), color, 0.0),
        ColorVertex2D::new(Vector2::new(0.0, DISPLAY_SIZE), color, 0.0),
        ColorVertex2D::new(Vector2::new(0.0, DISPLAY_SIZE), color, 0.0),
        ColorVertex2D::new(Vector2::new(BORDER_THICK, 1000.0), color, 0.0),
        ColorVertex2D::new(Vector2::new(BORDER_THICK, 0.0), color, 0.0),
        // right border
        ColorVertex2D::new(Vector2::new(DISPLAY_SIZE - BORDER_THICK, 0.0), color, 0.0),
        ColorVertex2D::new(Vector2::new(DISPLAY_SIZE, 0.0), color, 0.0),
        ColorVertex2D::new(
            Vector2::new(DISPLAY_SIZE - BORDER_THICK, DISPLAY_SIZE),
            color,
            0.0,
        ),
        ColorVertex2D::new(
            Vector2::new(DISPLAY_SIZE - BORDER_THICK, DISPLAY_SIZE),
            color,
            0.0,
        ),
        ColorVertex2D::new(Vector2::new(DISPLAY_SIZE, DISPLAY_SIZE), color, 0.0),
        ColorVertex2D::new(Vector2::new(DISPLAY_SIZE, 0.0), color, 0.0),
        // top border
        ColorVertex2D::new(Vector2::new(0.0, 0.0), color, 0.0),
        ColorVertex2D::new(Vector2::new(0.0, BORDER_THICK), color, 0.0),
        ColorVertex2D::new(Vector2::new(DISPLAY_SIZE, 0.0), color, 0.0),
        ColorVertex2D::new(Vector2::new(DISPLAY_SIZE, 0.0), color, 0.0),
        ColorVertex2D::new(Vector2::new(1000.0, BORDER_THICK), color, 0.0),
        ColorVertex2D::new(Vector2::new(0.0, BORDER_THICK), color, 0.0),
        // bottom border
        ColorVertex2D::new(Vector2::new(0.0, DISPLAY_SIZE - BORDER_THICK), color, 0.0),
        ColorVertex2D::new(Vector2::new(0.0, DISPLAY_SIZE), color, 0.0),
        ColorVertex2D::new(
            Vector2::new(DISPLAY_SIZE, DISPLAY_SIZE - BORDER_THICK),
            color,
            0.0,
        ),
        ColorVertex2D::new(
            Vector2::new(DISPLAY_SIZE, DISPLAY_SIZE - BORDER_THICK),
            color,
            0.0,
        ),
        ColorVertex2D::new(Vector2::new(DISPLAY_SIZE, DISPLAY_SIZE), color, 0.0),
        ColorVertex2D::new(Vector2::new(0.0, DISPLAY_SIZE), color, 0.0),
    ]
}

fn outline_vertices() -> [ColorVertex2D; 4] {
    let color = Color::from_hex(OUTLINE_COLOR);
    [
        ColorVertex2D::new(Vector2::new(BORDER_THICK, BORDER_THICK), color, 0.0),
        ColorVertex2D::new(
            Vector2::new(DISPLAY_SIZE - BORDER_THICK, BORDER_THICK),
            color,
            0.0,
        ),
        ColorVertex2D::new(
            Vector2::new(DISPLAY_SIZE - BORDER_THICK, DISPLAY_SIZE - BORDER_THICK),
            color,
            0.0,
        ),
        ColorVertex2D::new(
            Vector2::new(0.0 + BORDER_THICK, DISPLAY_SIZE - BORDER_THICK),
            color,
            0.0,
        ),
    ]
}
