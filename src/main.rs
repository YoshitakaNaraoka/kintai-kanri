mod app;

use app::App;

fn main() {
    console_error_panic_hook::set_once();
    yew::start_app::<App>();
}
