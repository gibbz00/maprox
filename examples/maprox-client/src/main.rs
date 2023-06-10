use flatgeobuf::{FallibleStreamingIterator, FgbReader};
use geozero::ToGeo;
use log::info;
use maprox_common::{Event, MaproxConnection, MAPROX_CONNECTION_URL};
use matchbox_socket::WebRtcSocket;
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

    let (socket, loop_fut) = WebRtcSocket::new_reliable(MAPROX_CONNECTION_URL);

    std::thread::spawn(move || {
        let executor = async_executor::Executor::new();
        let task = executor.spawn(loop_fut);
        let _ = futures_lite::future::block_on(executor.run(task));
    });

    let mut maprox_connection = MaproxConnection::new(socket);
    let mut sent_geometries = false;

    loop {
        maprox_connection.register_peers();

        if maprox_connection.connected_peers_count() == 0 {
            info!("Waiting for a maprox connection.");
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
