use flatgeobuf::{FallibleStreamingIterator, FgbReader};
use geozero::ToGeo;
use log::info;
use maprox_common::{Event, MaproxConnection, MAPROX_CONNECTION_URL};
use std::{fs::File, io::BufReader, time::Duration};
use tracing_subscriber::{fmt::layer, prelude::*, EnvFilter};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "maprox_client=info,maprox_common=info".into()),
        )
        .with(layer())
        .init();

    let mut maprox_connection = MaproxConnection::new(MAPROX_CONNECTION_URL);
    let mut sent_geometries = false;

    loop {
        maprox_connection.register_peers().unwrap();

        if maprox_connection.connected_peers_count() == 0 {
            info!("Waiting for a maprox connection.");
            std::thread::sleep(Duration::from_secs(1));
            continue;
        }

        info!("Established a maprox connection!");

        if sent_geometries == false {
            info!("Reading 'countries.fgb'");
            let mut reader = BufReader::new(File::open("countries.fgb").unwrap());
            let mut flatgeobuf_reader = FgbReader::open(&mut reader).unwrap().select_all().unwrap();
            info!("Sending geometries.");
            while let Some(simple_feature) = flatgeobuf_reader.next().unwrap() {
                if let Ok(geometry) = simple_feature.to_geo() {
                    maprox_connection.send_event(Event::RenderGeometry(geometry));
                }
            }
            info!("sent geometries");
            sent_geometries = true;
        }

        std::thread::sleep(Duration::from_secs(1));

        maprox_connection.send_event(Event::Increment);
    }
}
