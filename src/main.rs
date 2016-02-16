extern crate tempdir;
extern crate hyper;
extern crate iron;
extern crate router;
extern crate rustc_serialize;

use iron::prelude::*;
use iron:: AfterMiddleware;
use iron::headers;

use router::Router;

mod compile;
mod help;

struct CORS;

impl AfterMiddleware for CORS {
    fn after(&self, _: &mut Request, mut res: Response) -> IronResult<Response> {
        res.headers.set(headers::AccessControlAllowOrigin::Any);
        Ok(res)
    }
}

fn main() {
    println!("On 3000");

    let mut router = Router::new();

    router.get("/", help::help);
    router.post("/rust", compile::rust);
    router.post("/cpp", compile::cpp);

    let mut chain = Chain::new(router);
    chain.link_after(CORS);

    Iron::new(chain).http("localhost:3000").unwrap();
}
