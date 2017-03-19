extern crate rpizw_rover;
extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate unicase;
extern crate logger;
#[macro_use]
extern crate chan;
extern crate chan_signal;
#[macro_use]
extern crate log;
extern crate env_logger;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use iron::prelude::*;
use iron::{status, AfterMiddleware};
use iron::method::Method;
use iron::headers;
use iron::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use logger::Logger;
use router::Router;
use mount::Mount;
use staticfile::Static;
use std::path::Path;
use rpizw_rover::Rover;
use chan_signal::Signal;
use std::io::Read;
use unicase::UniCase;

const PWM_CHIP: u32 = 0;
const LEFT_PWM: u32 = 0;
const RIGHT_PWM: u32 = 1;

/// The payload that is json encoded and send back for every request.
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum ResponsePayload {
    Error { success: bool, error: String },
    Simple { success: bool },
}

impl ResponsePayload {
    /// The reponse that is sent when an error in encountered.
    pub fn error(error: String) -> ResponsePayload {
        ResponsePayload::Error {
            success: false,
            error: error,
        }
    }
    /// The response that is sent when a reqeust is carried out without error
    /// and there is no data to return to the client.
    pub fn success() -> ResponsePayload {
        ResponsePayload::Simple { success: true }
    }

    /// Converts the payload to a iron response with the ok status.
    pub fn to_response(self) -> Response {
        let mut res = Response::with((status::Ok, serde_json::to_string(&self).unwrap()));
        res.headers.set(headers::ContentType(Mime(TopLevel::Application,
                                                  SubLevel::Json,
                                                  vec![(Attr::Charset, Value::Utf8)])));
        res
    }
}

/// Reimplmentation of irons itry! macro that sets the body to a json message on error.
macro_rules! rtry {
    ($result:expr) => (rtry!($result, "{}"));
    ($result:expr, $message:expr) => (rtry!($result, $message, iron::status::InternalServerError));
    ($result:expr, $message:expr, $status:expr) => (match $result {
        ::std::result::Result::Ok(val) => val,
        ::std::result::Result::Err(err) => {
            let message = serde_json::to_string(&ResponsePayload::error(format!($message,
                                                err))).unwrap();
            return ::std::result::Result::Err(iron::IronError::new(err, ($status, message)))
        }
    });
}

fn main() {
    env_logger::init().unwrap();
    reset_rover().unwrap();

    let mut api_router = Router::new();
    api_router.put("/reset", reset, "reset");
    api_router.put("/stop", stop, "stop");
    api_router.put("/enable", enable, "enable");
    api_router.put("/disable", disable, "disable");
    api_router.put("/speed", set_speed, "set_speed");

    let mut api_chain = Chain::new(api_router);
    let cors_middleware = CORS {};

    api_chain.link_after(cors_middleware);

    let mut root_mount = Mount::new();
    root_mount.mount("/api/", api_chain);
    root_mount.mount("/", Static::new(Path::new("/srv/rover/ui")));

    let mut root_chain = Chain::new(root_mount);
    let (logger_before, logger_after) = Logger::new(None);
    root_chain.link_before(logger_before);
    root_chain.link_after(logger_after);

    let signal = chan_signal::notify(&[Signal::INT, Signal::TERM]);
    let mut serv = Iron::new(root_chain).http("0.0.0.0:3000").unwrap();
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

    // Ensure we stop the rover and cleanup.
    let rover = Rover::new(PWM_CHIP, LEFT_PWM, RIGHT_PWM).unwrap();
    rover.unexport().unwrap();
}

/// Resets the rover to its default settings.
fn reset(_: &mut Request) -> IronResult<Response> {
    rtry!(reset_rover());
    Ok(ResponsePayload::success().to_response())
}

/// Stops the rover from moving. Equlivent to settings its speed to 0.
fn stop(_: &mut Request) -> IronResult<Response> {
    let rover = rtry!(Rover::new(PWM_CHIP, LEFT_PWM, RIGHT_PWM));
    rtry!(rover.stop());
    Ok(ResponsePayload::success().to_response())
}

/// Enables the rover, allowing it to move. The rover will start moving at what
/// ever its speed was last set to (this includes stop). It is recomended to
/// call `speed` or `stop` before enabling movment if you are unsure about its
/// previous speed.
fn enable(_: &mut Request) -> IronResult<Response> {
    let rover = rtry!(Rover::new(PWM_CHIP, LEFT_PWM, RIGHT_PWM));
    rtry!(rover.enable(true));
    Ok(ResponsePayload::success().to_response())
}

/// Disables the rover, stopping it from moving and reacting to future calls to
/// speed/stop. Note that this is a soft stop, it does not cause the rover to
/// `break` like calling `stop` does. As a result the rover will coast for a
/// short period of time. If this is not desired then call `stop` followed by a
/// short delay before disabling the rover.
fn disable(_: &mut Request) -> IronResult<Response> {
    let rover = rtry!(Rover::new(PWM_CHIP, LEFT_PWM, RIGHT_PWM));
    rtry!(rover.enable(false));
    Ok(ResponsePayload::success().to_response())
}

/// Sets the speed of the rover. The speed can be any value from 100 to -100. 0
/// causes the rover to break and negitive numbers cause it to go in reverse.
fn set_speed(req: &mut Request) -> IronResult<Response> {
    #[derive(Serialize, Deserialize, Debug)]
    struct SpeedRequest {
        left: i8,
        right: i8,
    }
    let mut body = String::new();
    rtry!(req.body.read_to_string(&mut body));
    let SpeedRequest { left, right } = rtry!(serde_json::from_str(&body),
                                             "invalid json: {}",
                                             status::BadRequest);

    let rover = rtry!(Rover::new(PWM_CHIP, LEFT_PWM, RIGHT_PWM));
    rtry!(rover.set_speed(left, right));
    Ok(ResponsePayload::success().to_response())
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

struct CORS;

impl CORS {
    fn add_headers(res: &mut Response) {
        res.headers.set(headers::AccessControlAllowOrigin::Any);
        res.headers.set(headers::AccessControlAllowHeaders(
            vec![
                UniCase(String::from("accept")),
                UniCase(String::from("content-type"))
            ]
        ));
        res.headers.set(headers::AccessControlAllowMethods(vec![Method::Put]));
    }
}

impl AfterMiddleware for CORS {
    fn after(&self, req: &mut Request, mut res: Response) -> IronResult<Response> {
        if req.method == Method::Options {
            res = Response::with(status::Ok);
        }
        CORS::add_headers(&mut res);
        Ok(res)
    }

    fn catch(&self, _: &mut Request, mut err: IronError) -> IronResult<Response> {
        CORS::add_headers(&mut err.response);
        Err(err)
    }
}

