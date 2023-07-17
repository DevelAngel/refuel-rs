use crate::types::Station;
use crate::types::StationPriceChange;

use chrono::prelude::*;
use leptos::*;
use leptos_router::*;
use leptos_chart::*;
use theta_chart::series::{SNumber, STime};

#[component]
pub fn PriceHistory(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <StationList/>
        <Outlet/>
    }
}

#[component]
pub fn StationList(cx: Scope) -> impl IntoView {
    let list = create_resource(
        cx,
        || (), //< run once
        |_| async move {
            get_stations().await.unwrap()
        },
    );

    view! {
        cx,
        <div>
            <Suspense fallback=move || view! { cx, <p>"Loading Station List..."</p> }>
                <table class="primary">
                    <thead>
                        <tr>
                            <th>"ID"</th>
                            <th>"Refuel Station"</th>
                            <th>"Address"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {move || { list.read(cx).map(|list| list.into_iter()
                            .map(|n| view! { cx,
                                <tr>
                                    <td><div><A href={n.id.to_string()}>{n.id}</A></div></td>
                                    <td><div>{n.name}</div></td>
                                    <td><address>{n.addr}</address></td>
                                </tr>
                            })
                            .collect_view(cx)
                        )}}
                    </tbody>
                </table>
            </Suspense>
        </div>
    }
}

#[component]
pub fn StationPriceHistory(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);

    let list = create_resource(
        cx,
        move || params.with(|p| p.get("id").cloned().unwrap_or_default()),
        move |id| async move {
            get_price_history(id.parse::<i32>().expect("station id is no number")).await.unwrap()
        },
    );

    view! {
        cx,
        <div>
            <Suspense fallback=move || view! { cx, <p>"Loading Station Price History..."</p> }>
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
                {move || { list.read(cx).map(|list| {
                    let (list_price, list_updated): (Vec<_>, Vec<_>) = list.into_iter()
                        .map(|n| (
                            n.price[0] as f64 + 0.01f64 * n.price[1] as f64 + 0.001f64 * n.price[2] as f64,
                            n.updated.naive_local()
                        ))
                        .unzip();
                    let list_updated = STime::new(list_updated)
                        .set_format("%m");
                    let list_price = SNumber::new(list_price)
                        .set_range(1.6, 2.0);

                    let chart = Cartesian::new(
                            Series::Time(list_updated),
                            Series::Number(list_price),
                        )
                        .set_view(820, 620, 3, 100, 100, 20);

                    view! {
                        cx,
                        <LineChart chart=chart />
                    }
                })}}
            </Suspense>
        </div>
    }
}

#[server(GetStations, "/api", "GetCbor")]
pub async fn get_stations() -> Result<Vec<Station>, ServerFnError> {
    use refuel_db::establish_connection_sqlite;
    use refuel_db::prelude::Station as DBStation;

    // simulate some time to acquire the informations
    //tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    let conn = &mut establish_connection_sqlite();
    let list = DBStation::load_all(conn);
    let list = list.into_iter()
        .map(|rs| Station::from(rs))
        .collect();
    Ok(list)
}

#[server(GetPriceHistory, "/api", "GetCbor")]
pub async fn get_price_history(station_id: i32) -> Result<Vec<StationPriceChange>, ServerFnError> {
    use refuel_db::establish_connection_sqlite;
    use refuel_db::prelude::StationPriceChange as DBStationPriceChange;

    // simulate some time to acquire the informations
    //tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    let conn = &mut establish_connection_sqlite();
    let list = DBStationPriceChange::load_station(station_id, conn);
    let list = list.into_iter()
        .map(|rs| StationPriceChange::from(rs))
        .collect();
    Ok(list)
}
