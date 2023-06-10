use flatgeobuf::{FallibleStreamingIterator, FgbReader};
use geozero::ToGeo;
use log::info;
use maprox_common::{Event, MaproxConnection, MAPROX_CONNECTION_URL};
use std::{fs::File, io::BufReader, time::Duration};

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
                .unwrap_or_else(|_| "simple_example=info,matchbox_socket=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    async_main().await
}

async fn async_main() {
    let mut maprox_connection = MaproxConnection::new(MAPROX_CONNECTION_URL);
    let mut sent_geometries = false;

    loop {
        maprox_connection.register_peers().unwrap();
        futures_timer::Delay::new(Duration::from_millis(500)).await;

        if maprox_connection.connected_peers_count() == 0 {
            info!("Waiting for a maprox connection.");
            continue;
        }

        if !sent_geometries {
            info!("Reading 'countries.fgb'");
            #[cfg(target_arch = "wasm32")]
            {
                let resp = gloo_net::Request::get("https://flatgeobuf.org/test/data/countries.fgb")
                    .send()
                    .await
                    .unwrap();
                let bytes = resp.binary().await.unwrap();
                let mut reader = BufReader::new(bytes.as_slice());
            }
            #[cfg(not(target_arch = "wasm32"))]
            let mut reader = BufReader::new(File::open("countries.fgb").unwrap());

            let mut flatgeobuf_reader = FgbReader::open(&mut reader)
                .unwrap()
                .select_all_seq()
                .unwrap();
            info!("Sending geometries.");
            while let Some(simple_feature) = flatgeobuf_reader.next().unwrap() {
                if let Ok(geometry) = simple_feature.to_geo() {
                    maprox_connection.send_event(Event::RenderGeometry(geometry));
                }
            }
            info!("sent geometries");
            sent_geometries = true;
        }

        maprox_connection.send_event(Event::Increment);
    }
}
