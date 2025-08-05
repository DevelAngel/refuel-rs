use crate::price_list::PriceListItem;
use crate::types::StationPriceChange;

use chrono::prelude::*;
use leptos::prelude::*;

#[component]
pub fn AllPrices() -> impl IntoView {
    let list = create_resource(
        || (), //< run once
        |_| async move { get_all_prices().await.expect("get_all_prices resource") },
    );

    view! {
        <Suspense fallback=move || view! { <p>"Loading Price List with History..."</p> }>
            {move || { list.read().map(|list| list.into_iter()
                .map(|n| view! {
                    <PriceListItem item=n/>
                })
                .collect_view()
            )}}
        </Suspense>
    }
}

#[server]
pub async fn get_all_prices() -> Result<Vec<StationPriceChange>, ServerFnError> {
    use refuel_db::establish_connection_sqlite;
    use refuel_db::prelude::StationPriceChange as DBStationPriceChange;

    // simulate some time to acquire the informations
    //tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    let conn = &mut establish_connection_sqlite();
    let list = DBStationPriceChange::load_all(conn);
    let list = list
        .into_iter()
        .map(|rs| StationPriceChange::from(rs))
        .collect();
    Ok(list)
}
