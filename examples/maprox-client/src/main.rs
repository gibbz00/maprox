use flatgeobuf::{FallibleStreamingIterator, FgbReader};
use geozero::ToGeo;
use log::info;
use maprox_api::{
    events::Event,
    handle::{MaproxHandle, MAPROX_CONNECTION_URL},
};
use std::{io::BufReader, time::Duration};
use wasm_bindgen::UnwrapThrowExt;

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();
    wasm_bindgen_futures::spawn_local(async_main());
}

async fn async_main() {
    let maprox_handle = MaproxHandle::new(MAPROX_CONNECTION_URL);
    let mut sent_geometries = false;

    let maprox_handle_clone = maprox_handle.clone();
    let _listener = gloo_events::EventListener::new(
        &gloo_utils::document()
            .get_element_by_id("bt")
            .unwrap_throw(),
        "click",
        move |_event| maprox_handle_clone.send_event(Event::RefreshColors),
    );

    loop {
        maprox_handle.register_peers().unwrap();

        futures_timer::Delay::new(Duration::from_millis(500)).await;

        if maprox_handle.connected_peers_count() == 0 {
            info!("Waiting for a maprox connection.");
            continue;
        }

        if !sent_geometries {
            info!("Reading 'countries.fgb'");
            let bytes: Vec<u8>;

            let resp =
                gloo_net::http::Request::get("https://flatgeobuf.org/test/data/countries.fgb")
                    .send()
                    .await
                    .unwrap();
            bytes = resp.binary().await.unwrap();

            let mut reader = BufReader::new(bytes.as_slice());
            let mut flatgeobuf_reader = FgbReader::open(&mut reader)
                .unwrap()
                .select_all_seq()
                .unwrap();
            info!("Sending geometries.");
            while let Some(simple_feature) = flatgeobuf_reader.next().unwrap() {
                if let Ok(geometry) = simple_feature.to_geo() {
                    maprox_handle.send_event(Event::RenderGeometry(geometry));
                }
            }
            info!("sent geometries");
            sent_geometries = true;
        }

        maprox_handle.send_event(Event::Increment);
    }
}
