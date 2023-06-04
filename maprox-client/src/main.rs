use futures::{select, FutureExt};
use futures_timer::Delay;
use log::info;
use maprox_common::{Event, MaproxConnection, MAPROX_CONNECTION_URL};
use matchbox_socket::WebRtcSocket;
use std::time::Duration;

async fn async_main() {
    let (socket, loop_fut) = WebRtcSocket::new_reliable(MAPROX_CONNECTION_URL);
    let loop_fut = loop_fut.fuse();
    futures::pin_mut!(loop_fut);

    let timeout = Delay::new(Duration::from_millis(100));
    futures::pin_mut!(timeout);

    let mut maprox_connection = MaproxConnection::new(socket);

    loop {
        maprox_connection.register_peers();

        for event in maprox_connection.receive_event() {
            match event {
                Event::Increment => info!("Incremented!"),
            }
        }

        maprox_connection.send_event(Event::Increment);

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
