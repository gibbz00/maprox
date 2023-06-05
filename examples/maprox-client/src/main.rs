use flatgeobuf::{FallibleStreamingIterator, FgbReader};
use futures::{select, FutureExt};
use futures_timer::Delay;
use geozero::ToGeo;
use log::info;
use maprox_common::{Event, MaproxConnection, MAPROX_CONNECTION_URL};
use matchbox_socket::WebRtcSocket;
use std::{fs::File, io::BufReader, time::Duration};

async fn async_main() {
    let (socket, loop_fut) = WebRtcSocket::new_reliable(MAPROX_CONNECTION_URL);
    let loop_fut = loop_fut.fuse();
    futures::pin_mut!(loop_fut);

    let timeout = Delay::new(Duration::from_millis(100));
    futures::pin_mut!(timeout);

    let mut maprox_connection = MaproxConnection::new(socket);

    loop {
        maprox_connection.register_peers();

        select! {
            // Restart this loop every 100ms
            _ = (&mut timeout).fuse() => {
                timeout.reset(Duration::from_millis(100));
            }

            // Or break if the message loop ends (disconnected, closed, etc.)
            _ = &mut loop_fut => {
                break;
            }
        }

        if maprox_connection.connected_peers_count() == 0 {
            info!("waiting for maprox connection");
            continue;
        }

        info!("reading countries.fgb");
        let mut reader = BufReader::new(File::open("countries.fgb").unwrap());
        let mut flatgeobuf_reader = FgbReader::open(&mut reader).unwrap().select_all().unwrap();
        info!("sending geometries");
        while let Some(simple_feature) = flatgeobuf_reader.next().unwrap() {
            if let Ok(geometry) = simple_feature.to_geo() {
                maprox_connection.send_event(Event::RenderGeometry(geometry));
            }
        }
        info!("sent geometries");

        // TODO: make send_event async
        // break;

        select! {
            // Restart this loop every 100ms
            _ = (&mut timeout).fuse() => {
                timeout.reset(Duration::from_secs(30));
            }

            // Or break if the message loop ends (disconnected, closed, etc.)
            _ = &mut loop_fut => {
                break;
            }
        }

        // for event in maprox_connection.receive_event() {
        //     match event {
        //         Event::Increment => info!("Incremented!"),
        //         _ => (),
        //     }
        // }
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    wasm_bindgen_futures::spawn_local(async_main());
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    use tracing_subscriber::prelude::*;
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "maprox_client=info,maprox_common=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    async_main().await
}
