extern crate actix;

use actix::prelude::*;
use std::{thread, time};

struct HeartBeatActor;

impl Actor for HeartBeatActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("simple actor starting");
        self.heartbeat(ctx)
    }
}

impl HeartBeatActor {
    fn heartbeat(&mut self, ctx: &mut Context<Self>) {
        ctx.run_later(time::Duration::new(2, 0), |act, ctx| {
            println!("simple actor heartbeat");
            act.heartbeat(ctx)
        });
    }
}

fn main() {
    actix::System::run(|| {
        println!("running system");
        Arbiter::start(|_| {
            println!("starting simple actor");
            HeartBeatActor
        });
    });
}
