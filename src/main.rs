use leptos::*;
use thaw::*;

fn main() {
    mount_to_body(App)
}

struct LinkInfo {
  name: String,
  url: String,
  icon: icondata::AiIcon,
}

#[component]
pub fn App() -> impl IntoView {
    // variables
    let username = "mewpawee";
    let links = vec![
        LinkInfo{
            name: "LinkedIn".to_string(),
            url:"https://www.linkedin.com/in/pawee-tanti".to_string(),
            icon: icondata::AiIcon::AiLinkedinFilled,
        },
        LinkInfo{
            name: "Blog".to_string(),
            url:"https://www.mewpawee.xyz".to_string(),
            icon: icondata::AiIcon::AiBookFilled,
        },
    ];

    view! {
        <Layout style="padding: 64px 16px 32px;">
            <Layout style="max-width:680px; margin: 0px auto; width:100%; height:100%; text-align:center;">
                <Layout style="margin: 0px auto;">
                    <Avatar round=true size=100/>
                </Layout>
                <h3>{username}</h3>
                {links
                    .into_iter()
                    .map(|link| {
                        view! { <Link info=link/> }
                    })
                    .collect_view()}
            </Layout>
        </Layout>
    }
}

#[component]
pub fn Link(info: LinkInfo) -> impl IntoView {
    view! {
        <Card>
            <Grid cols=3>
                <GridItem>
                    <Icon icon=icondata::Icon::from(info.icon)/>
                </GridItem>
                <GridItem>
                    <a href=info.url>{info.name}</a>
                </GridItem>
                <GridItem>extra</GridItem>
            </Grid>
        </Card>
    }
}
