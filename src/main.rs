use zip::read::ZipArchive;
use std::io::Cursor;

const DOCS: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/docs.zip"));

fn main() {
    let mut archive = ZipArchive::new(Cursor::new(DOCS)).unwrap();
    let server = tiny_http::Server::http("0.0.0.0:10101").unwrap();

    std::thread::spawn(|| {
        opener::open("http://localhost:10101/").unwrap();
    });
    for request in server.incoming_requests() {
        if *request.method() != tiny_http::Method::Get {
            request.respond(tiny_http::Response::from_string("").with_status_code(405)).unwrap();
        } else {
            let req = url::Url::parse(&format!("http://a/{}", request.url())).unwrap();
            let mut path = percent_encoding::percent_decode_str(req.path()).decode_utf8_lossy();

            if path.ends_with("/") {
                path += "index.html";
            }

            match archive.by_name(&path.trim_start_matches("/")) {
                Ok(file) => {
                    request.respond(tiny_http::Response::new(
                        (200).into(),
                        vec![tiny_http::Header::from_bytes("content-type", mime_guess::from_path(path.as_ref()).first_or(mime_guess::mime::TEXT_PLAIN).as_ref()).unwrap()],
                        file,
                        None,
                        None,
                    )).unwrap();
                },
                Err(zip::result::ZipError::FileNotFound) => {
                    request.respond(tiny_http::Response::from_string("").with_status_code(404)).unwrap();
                },
                _ => panic!(),
            }
        }
    }
}
