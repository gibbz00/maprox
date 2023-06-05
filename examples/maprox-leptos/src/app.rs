use flatgeobuf::{FallibleStreamingIterator, FgbReader};
use futures::{select, FutureExt};
use futures_timer::Delay;
use geozero::ToGeo;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use log::info;
use maprox_common::{Event, MaproxConnection, MAPROX_CONNECTION_URL};
use matchbox_socket::WebRtcSocket;
use std::{io::BufReader, time::Duration};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/unigis-frontend.css"/>
        <Title text="Welcome to Leptos"/>

        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    let maprox_conn_action = create_action(cx, |_: &()| establish_maprox_connection());
    maprox_conn_action.dispatch(());

    let flatgeobuf_bytes_action = create_action(cx, |_: &()| get_geometries());
    flatgeobuf_bytes_action.dispatch(());

    let on_click = move |_| {
        info!("Reading geometries.");
        maprox_conn_action
            .value()
            .update_untracked(|maprox_connection| {
                let mut bytes = &flatgeobuf_bytes_action.value().get().unwrap().unwrap()[..];
                let mut buffer = BufReader::new(&mut bytes);
                let mut flatgeobuf_reader = FgbReader::open(&mut buffer)
                    .unwrap()
                    .select_all_seq()
                    .unwrap();
                info!("Sending geometries.");
                while let Some(simple_feature) = flatgeobuf_reader.next().unwrap() {
                    if let Ok(geometry) = simple_feature.to_geo() {
                        maprox_connection
                            .as_mut()
                            .unwrap()
                            .as_mut()
                            .unwrap()
                            .send_event(Event::RenderGeometry(geometry));
                    }
                }
                info!("Sent geometries.");
            });
    };

    view! { cx,
        <h1>"Welcome to Maprox-Leptos!"</h1>
        <iframe
            width="600"
            height="400"
            src="http://localhost:1334">
        </iframe>
        <hr/>
        <button on:click=on_click>"Fetch geometries!"</button>
    }
}

async fn establish_maprox_connection() -> Result<MaproxConnection, ()> {
    info!("Establishing a maprox connection.");
    let (socket, loop_fut) = WebRtcSocket::new_reliable(MAPROX_CONNECTION_URL);
    let loop_fut = loop_fut.fuse();
    futures::pin_mut!(loop_fut);

    let timeout = Delay::new(Duration::from_millis(100));
    futures::pin_mut!(timeout);

    let mut maprox_connection = MaproxConnection::new(socket);

    loop {
        maprox_connection.register_peers();

        select! {
            _ = (&mut timeout).fuse() => {
                timeout.reset(Duration::from_millis(100));
            }

            // Break if the message loop ends (disconnected, closed, etc.)
            _ = &mut loop_fut => {
                break;
            }
        }

        if maprox_connection.connected_peers_count() == 0 {
            info!("Waiting for a maprox connection.");
            continue;
        }

        return Ok(maprox_connection);
    }

    Err(())
}

#[server(GetGeometries, "/api")]
pub async fn get_geometries() -> Result<Vec<u8>, ServerFnError> {
    info!("Reading 'countries.fgb'");
    Ok(std::fs::read("countries.fgb").unwrap())
}
