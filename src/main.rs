extern crate actix;
extern crate futures;

use actix::prelude::*;
use std::time;

struct Ping;

impl Message for Ping {
    type Result = bool;
}

struct ListenerActor;

impl Actor for ListenerActor {
    type Context = Context<Self>;
}

impl Handler<Ping> for ListenerActor {
    type Result = bool;

    fn handle(&mut self, _: Ping, _: &mut Context<Self>) -> Self::Result {
        println!("Listener actor received ping");
        true
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

impl HeartBeatActor {
    fn heartbeat(&mut self, ctx: &mut Context<Self>) {
        //self.0.do_send(Ping);
        ctx.run_later(time::Duration::new(2, 0), |act, ctx| {
            println!("simple actor heartbeat");
            act.0.do_send(Ping);
            act.heartbeat(ctx)
        });
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

#[cfg(test)]
mod tests {
    use super::{ListenerActor, Ping};
    use actix::run;
    use actix::prelude::*;
    use futures::prelude::*;

    #[test]
    fn it_works() {
        let addr = ListenerActor {}.start();
        let res = addr.send(Ping);
        run(|| {
            res.map(|r| {
                assert_eq!(r, true);
                System::current().stop();
            }).map_err(|_| ())
        });
    }
}
