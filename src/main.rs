use dioxus::prelude::*;

fn main() {
    dioxus_desktop::launch(App);
}

fn App(cx: Scope) -> Element{
    cx.render(rsx!{
        div {
            "Hello world!"
        }
    })
}