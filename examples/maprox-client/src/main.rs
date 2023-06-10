use bevy_tasks::{IoTaskPool, TaskPoolBuilder};
use flatgeobuf::{FallibleStreamingIterator, FgbReader};
use futures::{select, FutureExt};
use futures_timer::Delay;
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
    let mut maprox_connection = MaproxConnection::new(socket);
    IoTaskPool::init(|| TaskPoolBuilder::default().build());
    IoTaskPool::get().spawn(loop_fut).detach();

    let timeout = Delay::new(Duration::from_millis(100));
    futures::pin_mut!(timeout);

    loop {
        maprox_connection.register_peers();

        select! {
            _ = (&mut timeout).fuse() => {
                timeout.reset(Duration::from_millis(100));
            }
        }

        if maprox_connection.connected_peers_count() == 0 {
            info!("Waiting for a maprox connection.");
            continue;
        }

        info!("Established a maprox connection!");

        info!("Reading 'countries.fgb'");
        let mut reader = BufReader::new(File::open("countries.fgb").unwrap());
        let mut flatgeobuf_reader = FgbReader::open(&mut reader).unwrap().select_all().unwrap();
        info!("Sending geometries.");
        while let Some(simple_feature) = flatgeobuf_reader.next().unwrap() {
            if let Ok(geometry) = simple_feature.to_geo() {
                maprox_connection.send_event(Event::RenderGeometry(geometry));
            }
        }

        // WORKAROUND: maprox_connection is now async and can therefore not be awaited.
        std::thread::sleep(Duration::from_secs(1));
        info!("sent geometries");
        break;
    }
}
