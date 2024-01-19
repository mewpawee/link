use leptos::*;
use leptos_icons::*;
use icondata as i;

fn main() {
    mount_to_body(App)
}

pub struct LinkInfo {
  name: String,
  url: String,
  icon: i::Icon,
}

#[component]
pub fn App() -> impl IntoView {
    // variables
    let username = "@mewpawee";
    let links = vec![
        LinkInfo{
            name: "LinkedIn".to_string(),
            url:"https://www.linkedin.com/in/pawee-tanti".to_string(),
            icon: i::AiLinkedinFilled,
        },
        LinkInfo{
            name: "Blog".to_string(),
            url:"https://www.mewpawee.xyz".to_string(),
            icon: i::AiBookFilled,
        },
    ];

    view! {
        <div class="flex justify-center px-4 py-48">
            <div class="max-w-2xl mx-0 my-auto w-full h-full text-center">
                <Avatar/>
                <h1 class="my-4 text-2xl font-bold">{username}</h1>
                {links
                    .into_iter()
                    .map(|link| {
                        view! { <Link info=link/> }
                    })
                    .collect_view()}

            </div>
        </div>
    }
}

#[component]
pub fn Avatar() -> impl IntoView {
    view! {
        <div class="relative inline-flex items-center justify-center w-48 h-48 overflow-hidden bg-gray-100 rounded-full dark:bg-gray-600">
            <span class="font-medium text-gray-600 dark:text-gray-300">MP</span>
        </div>
    }
}

#[component]
pub fn Link(info: LinkInfo) -> impl IntoView {
    view! {
        <a
            href=info.url
            target="_blank"
            class="my-0.5 w-full inline-flex items-center justify-center p-5 text-base font-medium text-gray-500 rounded-lg bg-gray-50 hover:text-gray-900 hover:bg-gray-100 dark:text-gray-400 dark:bg-gray-800 dark:hover:bg-gray-700 dark:hover:text-white"
        >
            <Icon icon=info.icon/>
            <span>{info.name}</span>
        </a>
    }
}

