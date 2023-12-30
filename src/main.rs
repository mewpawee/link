use leptos::*;
use thaw::*;

fn main() {
    mount_to_body(App)
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Button variant=ButtonVariant::Primary>"Primary"</Button>
    }
}
