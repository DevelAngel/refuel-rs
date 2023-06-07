use chrono::prelude::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RefuelStation {
    name: String,
    addr: String,
    price: [u8; 3],
    updated: DateTime<Utc>,
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <Title text="Refuel WebApp"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Stylesheet id="picnic" href="/picnic.min.css"/>
        <Stylesheet id="leptos" href="/pkg/refuel.css"/>
        <Router>
            <nav>
                <A class="brand" href="">
                    <img class="logo" src="/favicon-1.png" />
                    <span>"Refuel"</span>
                </A>
                // responsive
                <input id="bmenub" type="checkbox" class="show"/>
                <label for="bmenub" class="burger pseudo button">"MENU"</label>
                // menu
                <div class="menu">
                    <A href="about" class="pseudo button icon-picture">"About"</A>
                </div>
            </nav>
            <main>
                <Routes>
                    <Route
                        path=""
                        view=move |cx| view! { cx,  <Home/> }
                    >
                    </Route>
                    <Route
                        path="about"
                        view=move |cx| view! { cx,  <About/> }
                    />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Home(cx: Scope) -> impl IntoView {
    let list = create_resource(
        cx,
        || (), //< run once
        |_| async move {
            get_current_prices().await.unwrap()
        },
    );

    view! {
        cx,
        <div>
            <Suspense fallback=move || view! { cx, <p>"Loading Price List..."</p> }>
                <table class="primary">
                    <thead>
                        <tr>
                            <th>"Refuel Station"</th>
                            <th>"Address"</th>
                            <th>"Price"</th>
                            <th>"Updated"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {move || { list.read(cx).map(|list| list.into_iter()
                            .map(|n| view! { cx,
                                <tr>
                                    <td><div>{n.name}</div></td>
                                    <td><address>{n.addr}</address></td>
                                    <td><span>{n.price[0]}","{n.price[1]}<sup>{n.price[2]}</sup></span></td>
                                    <td><div>{format!("{}", n.updated.with_timezone(&Local).format("%Y-%m-%d %H:%M"))}</div></td>
                                </tr>
                            })
                            .collect_view(cx)
                        )}}
                    </tbody>
                </table>
            </Suspense>
        </div>
        <Outlet/>
    }
}

#[component]
fn About(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div>
            <p>"It's me!"</p>
        </div>
    }
}

#[server(GetCurrentPrices, "/api", "GetCbor")]
pub async fn get_current_prices() -> Result<Vec<RefuelStation>, ServerFnError> {
    // simulate some time to acquire the informations
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    let list = vec![
        RefuelStation {
            name: "MyESSO".to_owned(),
            addr: "Marienfelder Chaussee 171, 12349 Berlin".to_owned(),
            price: [1, 78, 9],
            updated: Utc.with_ymd_and_hms(2023, 6, 4, 13, 0, 0).unwrap(),
        },
        RefuelStation {
            name: "MyJET".to_owned(),
            addr: "Rhinstr. 240, 13055 Berlin".to_owned(),
            price: [1, 79, 8],
            updated: Utc.with_ymd_and_hms(2023, 6, 4, 12, 0, 0).unwrap(),
        },
        RefuelStation {
            name: "MyTotalEnergies".to_owned(),
            addr: "Landsberger Allee 376, 12681 Berlin".to_owned(),
            price: [1, 81, 9],
            updated: Utc.with_ymd_and_hms(2023, 6, 4, 12, 30, 0).unwrap(),
        },
        RefuelStation {
            name: "MyAGIP ENI".to_owned(),
            addr: "Dietzgenstr. 127, 13158 Berlin".to_owned(),
            price: [1, 80, 9],
            updated: Utc.with_ymd_and_hms(2023, 6, 4, 15, 15, 0).unwrap(),
        },
    ];
    Ok(list)
}

