use iron::prelude::*;
use iron::status;

const HELP: &'static str = r"
Welcome to CodePlatter

CodePlatter is a server that can compile code snippets";

pub fn help(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, HELP)))
}
