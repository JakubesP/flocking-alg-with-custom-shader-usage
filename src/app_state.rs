use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::Mutex;

use swgl::global_tools::vector2::Vector2;

// -----------------------------------------------------------------------------------------

lazy_static! {
    static ref APP_STATE: Mutex<Arc<AppState>> = Mutex::new(Arc::new(AppState::new()));
}

// -----------------------------------------------------------------------------------------

pub fn update_dynamic_data(time: f32, canvas_height: f32, canvas_width: f32) {
    let mut data = APP_STATE.lock().unwrap();
    *data = Arc::new(AppState {
        canvas_size: Vector2::new(canvas_height, canvas_width),
        time,
        keys: data.keys.clone(),
        ..*data.clone()
    });
}

// -----------------------------------------------------------------------------------------

pub fn get_curr_state() -> Arc<AppState> {
    APP_STATE.lock().unwrap().clone()
}

// -----------------------------------------------------------------------------------------

pub struct AppState {
    pub canvas_size: Vector2<f32>,
    pub mouse_pos: Vector2<f32>,
    pub mouse_down: bool,
    pub time: f32,
    pub keys: VecDeque<String>,
}

impl AppState {
    fn new() -> Self {
        Self {
            canvas_size: Vector2::zero(),
            mouse_pos: Vector2::zero(),
            mouse_down: false,
            time: 0.,
            keys: VecDeque::new(),
        }
    }
}

// -----------------------------------------------------------------------------------------

pub fn update_mouse_down(x: f32, y: f32, is_down: bool) {
    let mut data = APP_STATE.lock().unwrap();
    *data = Arc::new(AppState {
        mouse_down: is_down,
        mouse_pos: Vector2::new(x, y),
        keys: data.keys.clone(),
        ..*data.clone()
    });
}

// -----------------------------------------------------------------------------------------

pub fn update_mouse_position(x: f32, y: f32) {
    let mut data = APP_STATE.lock().unwrap();
    *data = Arc::new(AppState {
        mouse_pos: Vector2::new(x, y),
        keys: data.keys.clone(),
        ..*data.clone()
    });
}
