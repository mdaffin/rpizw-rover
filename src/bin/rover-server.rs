#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate iron;
extern crate router;
extern crate logger;
#[macro_use]
extern crate chan;
extern crate chan_signal;
#[macro_use]
extern crate log;
extern crate env_logger;

extern crate rpizw_rover;

use iron::prelude::*;
use iron::status;
use iron::middleware::AfterMiddleware;
use iron::headers::ContentType;
use iron::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use logger::Logger;
use router::Router;
use rpizw_rover::Rover;
use chan_signal::Signal;

const PWM_CHIP: u32 = 0;
const LEFT_PWM: u32 = 0;
const RIGHT_PWM: u32 = 1;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum ResponsePayload {
    Message { success: bool, message: String },
    Error { success: bool, error: String },
    Simple { success: bool },
}

impl ResponsePayload {
    pub fn error(error: String) -> ResponsePayload {
        ResponsePayload::Error {
            success: false,
            error: error,
        }
    }
    pub fn success() -> ResponsePayload {
        ResponsePayload::Simple { success: true }
    }
}

struct ErrorMiddleware;

impl AfterMiddleware for ErrorMiddleware {
    fn after(&self, _: &mut Request, res: Response) -> IronResult<Response> {
        Ok(res)
    }
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        let mut resp =
            Response::with((status::InternalServerError,
                            serde_json::to_string(&ResponsePayload::error(err.to_string()))
                                .unwrap()));
        resp.headers.set(ContentType(Mime(TopLevel::Application,
                                          SubLevel::Json,
                                          vec![(Attr::Charset, Value::Utf8)])));
        Ok(resp)
    }
}

fn main() {
    let (logger_before, logger_after) = Logger::new(None);

    env_logger::init().unwrap();
    reset_rover().unwrap();

    let mut router = Router::new();
    router.put("/api/reset", reset, "reset");
    router.put("/api/stop", stop, "stop");
    router.put("/api/enable", enable, "enable");
    router.put("/api/disable", disable, "disable");
    router.put("/api/speed", set_speed, "set_speed");

    let mut chain = Chain::new(router);
    chain.link_before(logger_before);
    chain.link_after(ErrorMiddleware {});
    chain.link_after(logger_after);

    let signal = chan_signal::notify(&[Signal::INT, Signal::TERM]);
    let mut serv = Iron::new(chain).http("0.0.0.0:3000").unwrap();
    info!("listening on 0.0.0.0:3000");

    // Block until SIGINT or SIGTERM is sent.
    chan_select! {
        signal.recv() -> _ => {
            info!("received signal shutting down");
            // Shutdown the server. Note that there is currently a bug in hyper
            // that means the server does not actually stop listening at this
            // point.
            serv.close().ok();
        }
    }

    {
        // Ensure we stop the rover and cleanup.
        let rover = Rover::new(PWM_CHIP, LEFT_PWM, RIGHT_PWM).unwrap();
        rover.unexport().unwrap();
    }
}

/// Resets the rover to its default settings.
fn reset(_: &mut Request) -> IronResult<Response> {
    itry!(reset_rover());
    let mut resp = Response::with((status::Ok,
                                   serde_json::to_string(&ResponsePayload::success()).unwrap()));
    resp.headers.set(ContentType(Mime(TopLevel::Application,
                                      SubLevel::Json,
                                      vec![(Attr::Charset, Value::Utf8)])));
    Ok(resp)
}

/// Stops the rover from moving. Equlivent to settings its speed to 0.
fn stop(_: &mut Request) -> IronResult<Response> {
    let rover = itry!(Rover::new(PWM_CHIP, LEFT_PWM, RIGHT_PWM));
    itry!(rover.stop());
    let mut resp = Response::with((status::Ok,
                                   serde_json::to_string(&ResponsePayload::success()).unwrap()));
    resp.headers.set(ContentType(Mime(TopLevel::Application,
                                      SubLevel::Json,
                                      vec![(Attr::Charset, Value::Utf8)])));
    Ok(resp)
}

/// Enables the rover, allowing it to move. The rover will start moving at what
/// ever its speed was last set to (this includes stop). It is recomended to
/// call `speed` or `stop` before enabling movment if you are unsure about its
/// previous speed.
fn enable(_: &mut Request) -> IronResult<Response> {
    let rover = itry!(Rover::new(PWM_CHIP, LEFT_PWM, RIGHT_PWM));
    itry!(rover.enable(true));
    let mut resp = Response::with((status::Ok,
                                   serde_json::to_string(&ResponsePayload::success()).unwrap()));
    resp.headers.set(ContentType(Mime(TopLevel::Application,
                                      SubLevel::Json,
                                      vec![(Attr::Charset, Value::Utf8)])));
    Ok(resp)
}

/// Disables the rover, stopping it from moving and reacting to future calls to
/// speed/stop. Note that this is a soft stop, it does not cause the rover to
/// `break` like calling `stop` does. As a result the rover will coast for a
/// short period of time. If this is not desired then call `stop` followed by a
/// short delay before disabling the rover.
fn disable(_: &mut Request) -> IronResult<Response> {
    let rover = itry!(Rover::new(PWM_CHIP, LEFT_PWM, RIGHT_PWM));
    itry!(rover.enable(false));
    let mut resp = Response::with((status::Ok,
                                   serde_json::to_string(&ResponsePayload::success()).unwrap()));
    resp.headers.set(ContentType(Mime(TopLevel::Application,
                                      SubLevel::Json,
                                      vec![(Attr::Charset, Value::Utf8)])));
    Ok(resp)
}

/// Sets the speed of the rover. The speed can be any value from 100 to -100. 0
/// causes the rover to break and negitive numbers cause it to go in reverse.
fn set_speed(_: &mut Request) -> IronResult<Response> {
    let rover = itry!(Rover::new(PWM_CHIP, LEFT_PWM, RIGHT_PWM));
    itry!(rover.set_speed(100, 100));
    let mut resp = Response::with((status::Ok,
                                   serde_json::to_string(&ResponsePayload::success()).unwrap()));
    resp.headers.set(ContentType(Mime(TopLevel::Application,
                                      SubLevel::Json,
                                      vec![(Attr::Charset, Value::Utf8)])));
    Ok(resp)
}

/// Helper function to ensure the rover is stopped, enabled and ready to start.
fn reset_rover() -> rpizw_rover::error::Result<()> {
    let rover = Rover::new(PWM_CHIP, LEFT_PWM, RIGHT_PWM)?;
    rover.export()?;
    rover.enable(false)?;
    rover.unexport()?;
    rover.export()?;
    rover.stop()?;
    rover.enable(true)
}
