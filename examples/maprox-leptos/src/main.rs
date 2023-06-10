use leptos::{component, view, IntoView, Scope};

fn main() {
    leptos::mount_to_body(|cx| {
        view! { cx,
            <App/>
        }
    });
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <iframe width=900 height=600 src="http://localhost:1334" />
    }
}
