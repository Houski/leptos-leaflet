use leptos::*;
use leptos_leaflet::{components::*, pos_opt, position, positions};
use leptos_meta::*;
use leptos_router::*;
use std::time::Duration;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet href="https://unpkg.com/leaflet@1.9.3/dist/leaflet.css" />
        <Script src="https://unpkg.com/leaflet@1.9.3/dist/leaflet.js"/>
        <Stylesheet id="leptos" href="/pkg/start-axum.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    let (marker_position, set_marker_position) = create_signal(cx, Position::new(51.49, -0.08));
    let (map, set_map) = create_signal(cx, LeafletMap::new());
    
    create_effect(cx, move |_| {
        // set_interval_with_handle(
        //     move || {
        //         set_marker_position.update(|pos| {
        //             pos.lat += 0.001;
        //             pos.lng += 0.001;
        //         });
        //     },
        //     Duration::from_millis(200),
        // )
        // .ok()
    });
    
    create_effect(cx, move |_| {
        #[cfg(target_arch = "wasm32")]
        if let LeafletMap { map: Some(map) } = map.get() {
            log!("Map context {:?}", map);
        }
    });

    view! { cx,
          <MapContainer style="height: 400px" center=pos_opt!(51.505, -0.09) zoom=13.0 set_view=true map=set_map>
              <TileLayer url="https://tile.openstreetmap.org/{z}/{x}/{y}.png" attribution="&copy; <a href=\"https://www.openstreetmap.org/copyright\">OpenStreetMap</a> contributors"/>
              <Marker position=marker_position >
                  <Popup>
                      <strong>{"A pretty CSS3 popup"}</strong>
                  </Popup>
              </Marker>
                <Marker position=position!(51.5, -0.065) draggable=true >
                  <Popup>
                      <strong>{"A pretty CSS3 popup"}</strong>
                  </Popup>
              </Marker>
              <Tooltip position=position!(51.5, -0.06) permanent=true direction="top">
                  <strong>{"And a tooltip"}</strong>
              </Tooltip>
              <Polyline positions=positions(&[(51.505, -0.09), (51.51, -0.1), (51.51, -0.12)])/>
              <Polygon color="purple" positions=positions(&[ (51.515, -0.09), (51.52, -0.1), (51.52, -0.12)]) >
                <Tooltip sticky=true direction="top">
                    <strong>{"I'm a polygon"}</strong>
                </Tooltip>
            </Polygon>
            <Circle center=position!(51.505, -0.09) color="blue" radius=200.0>
                <Tooltip sticky=true>{"I'm a circle"}</Tooltip>
            </Circle>
        </MapContainer>
    }
}