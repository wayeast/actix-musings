extern crate actix;
extern crate futures;
extern crate tokio;

use actix::prelude::*;
use futures::Future;
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

    fn handle(&mut self, _: Ping, _: &mut Context<Self>) -> Self::Result {
        println!("Listener actor received ping");
        thread::sleep(time::Duration::from_millis(1000));
        println!("Listener actor responding after hard work")
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
        let resp = self.0.send(Ping);
        tokio::spawn(
            resp.map(|_| println!("heartbeat actor got response from listener"))
                .map_err(|_| ())
        );
        ctx.run_later(time::Duration::new(2, 0), |act, ctx| {
            println!("heartbeat actor heartbeat");
            // let resp = act.0.send(Ping);
            // tokio::spawn(
            //     resp.map(|_| println!("heartbeat actor got response from listener"))
            //         .map_err(|_| ())
            // );
            act.heartbeat(ctx)
        });
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
