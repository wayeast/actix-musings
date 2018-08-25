extern crate actix;

use actix::prelude::*;
use std::{thread, time};

struct Ping;

impl Message for Ping {
    type Result = ();
}

struct ListenerActor;

impl Actor for ListenerActor {
    type Context = Context<Self>;
}

impl Handler<Ping> for ListenerActor {
    type Result = ();

    fn handle(&mut self, _: Ping, _: &mut Context<Self>) {
        println!("Listener actor received ping")
    }
}

struct HeartBeatActor(Addr<ListenerActor>);

impl Actor for HeartBeatActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("simple actor starting");
        self.heartbeat(ctx)
    }
}

impl Handler<Ping> for HeartBeatActor {
    type Result = ();

    fn handle(&mut self, _: Ping, _: &mut Context<Self>) {
        println!("Heartbeat actor received ping")
    }
}

impl HeartBeatActor {
    fn heartbeat(&mut self, ctx: &mut Context<Self>) {
        //self.0.do_send(Ping);
        ctx.run_later(time::Duration::new(2, 0), |act, ctx| {
            println!("simple actor heartbeat");
            act.0.do_send(Ping);
            act.heartbeat(ctx)
        });
        ctx.notify(Ping);
    }
}

fn main() {
    actix::System::run(|| {
        println!("running system");
        let listener_addr = Arbiter::start(|_| {
            println!("starting listener actor");
            ListenerActor
        });
        Arbiter::start(|_| {
            println!("starting simple actor");
            HeartBeatActor(listener_addr)
        });
    });
}
