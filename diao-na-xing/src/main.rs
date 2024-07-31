use dioxus::prelude::*;
mod app;
use dioxus_desktop::Config;

fn main() {
    dioxus_desktop::launch_with_props(app::app, (), Config::default());
}