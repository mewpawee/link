use leptos::*;
use thaw::*;

fn main() {
    mount_to_body(App)
}

struct Link {
  name: String,
  url: String,
}

#[component]
pub fn App() -> impl IntoView {
    let values = vec![
        Link{name: "LinkedIn".to_string(),url:"https://www.linkedin.com/in/pawee-tanti".to_string()},
        Link{name: "Blog".to_string(),url:"https://www.mewpawee.xyz".to_string()},
    ];

    view! {
        <Layout style="padding: 64px 16px 32px;">
            <Layout style="max-width:680px; margin: 0px auto; width:100%; height:100%; text-align:center;">
                <Space vertical=true>
                <Layout style="margin: 0px auto;">
                    <Avatar round=true size=100/>
                </Layout>
                <h3>@mewpawee</h3>
                {values.into_iter()
                    .map(|link| view! {
                        <Layout style="margin: 0px auto; width:100%;">
                            <Card>
                                <a href={link.url}>{link.name}</a>
                            </Card>
                        </Layout>
                    })
                    .collect_view()
                }
                </Space>    
            </Layout>
        </Layout>
    }
}
