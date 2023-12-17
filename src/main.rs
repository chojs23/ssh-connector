mod config;

use config::get_render_config;
use inquire::Select;

fn main() {
    inquire::set_global_render_config(get_render_config());
}
