use hyper::service::{make_service_fn,service_fn};
use hyper::{header, Body, Request, Response, Server};
use std::convert::Infallible;
use sha2::Sha256;
use hmac::{Hmac, Mac};
use serde::Deserialize;
use serde_json::Result;

#[derive(Debug)]
struct Commit {
    brach: String,
    commit_hex: String,
    add_file: i32,
    mod_file: i32,
    rem_file: i32,
}

#[derive(Deserialize)]
struct PushEvent {
    #[serde(rename = "ref")]
    branch_red: String,
    commits: Vec<SingleCommit>,
}

#[derive(Deserialize)]
struct SingleCommit {
    id: String,
    added: Vec<String>,
    rem: Vec<String>,
    modi: Vec<String>,
}

type HmacSha = Hmac<Sha256>;

async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let secret = b"kuro";
    println!("Entered handle!");

    if req.method() == hyper::Method::POST {
        let headers = req.headers().clone();
        let whole_body = hyper::body::to_bytes(req.into_body()).await.unwrap();

        println!("method POST handle!");
        if let Some(sig_h) = headers.get("X-Hub-Signature-256") {
            let sig_h = sig_h.to_str().unwrap();
            if let Some(sig_hex) = sig_h.strip_prefix("sha256=") {
                let mut mac = HmacSha::new_from_slice(secret).unwrap();
                mac.update(&whole_body);
                let res = mac.finalize().into_bytes();
                let res_hex = hex::encode(res);

                println!("sig created handle!");
                if res_hex.eq_ignore_ascii_case(sig_hex) {
                    println!("Recived! {:#?} ", whole_body);
                    return Ok(Response::new(Body::from("Ok")));
                } else {
                    println!("Wrong signature");
                    return Ok(Response::new(Body::from("Err")));
                }
            }
            return Ok(Response::new(Body::from("Err")));
        } else {
            return Ok(Response::new(Body::from("Err")));
        }
    } else {
            return Ok(Response::new(Body::from("Err")));
    }
}

#[tokio::main]
async fn main() {
    let addr = ([0, 0, 0, 0], 8080).into();
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });
    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
