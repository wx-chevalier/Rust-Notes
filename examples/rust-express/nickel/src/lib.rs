#![doc(test(attr(deny(warnings))))]

extern crate groupable;
pub extern crate hyper;
extern crate modifier;
extern crate mustache;
extern crate plugin;
extern crate regex;
extern crate serde;
extern crate serde_json;
extern crate time;
extern crate typemap;
extern crate url;

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

pub use body_parser::{BodyError, FormBody, JsonBody};
pub use default_error_handler::DefaultErrorHandler;
pub use favicon_handler::FaviconHandler;
pub use middleware::{Action, Continue, ErrorHandler, Halt, Middleware, MiddlewareResult};
pub use mimes::MediaType;
pub use mount::{Mount, Mountable};
pub use nickel::{Nickel, Options};
pub use nickel_error::NickelError;
pub use query_string::QueryString;
pub use request::Request;
pub use responder::Responder;
pub use response::Response;
pub use router::{HttpRouter, Route, RouteResult, Router};
pub use server::ListeningServer;
pub use static_files_handler::StaticFilesHandler;
pub use template_cache::{ReloadPolicy, TemplateCache};
pub use urlencoded::{Params, Query};

#[macro_use]
pub mod macros;

mod body_parser;
mod default_error_handler;
pub mod extensions;
mod favicon_handler;
mod middleware;
pub mod mimes;
mod mount;
mod nickel;
mod nickel_error;
mod query_string;
mod request;
mod responder;
mod response;
pub mod router;
mod server;
mod static_files_handler;
pub mod template_cache;
mod urlencoded;

pub mod status {
    pub use hyper::status::StatusCode;
}
