extern crate actix;
extern crate futures;
extern crate tokio_timer;

use futures::Future;
use std::time::{Duration, Instant};
use tokio_timer::Delay;

fn main() {
    actix::run(
        || {
            println!("Starting tokio timer...");
            Delay::new(Instant::now() + Duration::from_millis(1500))
                .map(|_| {
                    println!("Got response from tokio timer. Shutting down...");
                    actix::System::current().stop()
                })
                .map_err(|_| ())
        }
    );
}
