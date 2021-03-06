use argh::FromArgs;

#[derive(FromArgs)]
#[argh(description = "my documentation example program")]
struct Opts {
    #[argh(subcommand)]
    cmd: SubCommand,
}

#[derive(FromArgs)]
#[argh(subcommand)]
enum SubCommand {
    Add(Nums),
    #[cfg(feature = "docs")]
    Help(HelpOptions),
}

#[derive(FromArgs)]
#[argh(subcommand, name = "open-help", description = "open the help pages in the browser")]
struct HelpOptions {
    #[argh(option, default = "true", description = "open the browser")]
    open_browser: bool,
}

#[derive(FromArgs)]
#[argh(subcommand, name = "add", description = "add two numbers")]
struct Nums {
    #[argh(positional)]
    a: i32,
    #[argh(positional)]
    b: i32,
}

#[cfg(feature = "docs")]
fn run_help_server(opts: HelpOptions) {
    use percent_encoding::percent_decode_str;
    use std::io::{Cursor, Read};
    use tiny_http::{Method, Request, Response, Server};
    use url::Url;
    use zip::read::ZipArchive;

    const DOCS: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/docs.zip"));

    let mut archive = ZipArchive::new(Cursor::new(DOCS)).expect("Could not open zip file");
    let server = Server::http("127.0.0.1:10101").expect("Could not listen on localhost:10101");
    println!("Server is running at 127.0.0.1:10101");

    if opts.open_browser {
        std::thread::spawn(|| {
            opener::open("http://localhost:10101/")
                .expect("Could not open the browser, the server is still running though")
        });
    }

    fn respond(req: Request, res: Response<impl Read>) {
        if let Err(err) = req.respond(res) {
            eprintln!("Could not response to a request: {:?}", err);
        }
    }

    for request in server.incoming_requests() {
        if *request.method() != Method::Get {
            respond(request, Response::from_string("").with_status_code(405));
        } else {
            let req = Url::parse(&format!("http://a/{}", request.url())).unwrap();
            let mut path = percent_decode_str(req.path()).decode_utf8_lossy();

            if path.ends_with('/') {
                path += "index.html";
            }

            match archive.by_name(&path.trim_start_matches('/')) {
                Ok(file) => {
                    let mime_type =
                        mime_guess::from_path(path.as_ref()).first_or(mime_guess::mime::TEXT_PLAIN);

                    respond(
                        request,
                        tiny_http::Response::new(
                            (200).into(),
                            vec![
                                tiny_http::Header::from_bytes("content-type", mime_type.as_ref())
                                    .unwrap(),
                            ],
                            file,
                            None,
                            None,
                        ),
                    );
                }
                Err(zip::result::ZipError::FileNotFound) => {
                    respond(
                        request,
                        tiny_http::Response::from_string("").with_status_code(404),
                    );
                }
                _ => panic!(),
            }
        }
    }
}

fn main() {
    let opts = argh::from_env::<Opts>();

    match opts.cmd {
        SubCommand::Add(Nums { a, b }) => println!("{} + {} = {}", a, b, a + b),
        #[cfg(feature = "docs")]
        SubCommand::Help(helpopts) => run_help_server(helpopts),
    }
}

