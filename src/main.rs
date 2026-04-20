mod config;
mod market;
mod state;
mod components;

use dioxus::prelude::*;
use components::App;

fn main() {
    launch(App);
}