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

    fn handle(&mut self, _: Ping, ctx: &mut Context<Self>) {
        println!("Listener actor received ping");
        //panic!("Listener panicking!!!");  // if the thread panics, supervisor also panics
        ctx.stop();  // supervisor restarts a stopped actor, not a panicked thread
    }
}

impl Supervised for ListenerActor {
    fn restarting(&mut self, _: &mut Context<ListenerActor>) {
        println!("listener actor restarting");
    }
}

struct HeartBeatActor(Addr<ListenerActor>);

impl Actor for HeartBeatActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("heartbeat actor starting");
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
            println!("heartbeat actor heartbeat");
            act.0.do_send(Ping);
            act.heartbeat(ctx)
        });
        ctx.notify(Ping);
    }
}

fn listen() -> Addr<ListenerActor> {
    Supervisor::start(|_| {
        println!("starting listener actor");
        ListenerActor
    })
}

fn heartbeat(listener_addr: Addr<ListenerActor>) {
    Arbiter::start(|_| {
        println!("starting heartbeat actor");
        HeartBeatActor(listener_addr)
    });
}

fn main() {
    let sys = System::new("separate sys and arbiters");

    let listener_addr = listen();
    heartbeat(listener_addr);

    println!("running system");
    sys.run();
}
