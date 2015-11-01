#[macro_use] extern crate log;
extern crate env_logger;
extern crate rustc_serialize;
extern crate mustache;
extern crate getopts;
extern crate mime;
extern crate iron;
extern crate mount;
extern crate persistent;
extern crate pulldown_cmark;
#[macro_use] extern crate router;
extern crate staticfile;
extern crate typemap;

use std::io::{self, Read};
use std::net::Ipv4Addr;
use std::fmt;
use std::fs::File;
use std::fs::read_dir;
use std::path::Path;
use iron::headers::{ContentLength, ContentType};
use iron::prelude::*;
use iron::status;
use mount::Mount;
use router::Router;
use staticfile::Static;
use typemap::Key;

use compat::PathExt;

mod compat;
mod cmark;

macro_rules! try_return {
    ($e:expr) => {{
        match $e {
            Ok(v) => v,
            Err(e) => { error!("Error: {}", e); return; }
        }
    }}
}

// workaround (because mustache::Error doesn't implement std::error::Error
#[derive(Debug)]
struct TemplateError(mustache::Error);

impl fmt::Display for TemplateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl ::std::error::Error for TemplateError {
    fn description(&self) -> &str {
        use mustache::Error::*;
        match self.0 {
            NestedOptions => "Nested options",
            UnsupportedType => "Unsupported type",
            InvalidStr => "Invalid str",
            MissingElements => "Missing elements",
            KeyIsNotString => "Key is not a string",
            NoFilename => "No filename",
            IoError(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&::std::error::Error> {
        use mustache::Error::*;
        match self.0 {
            IoError(ref e) => Some(e),
            _ => None
        }
    }
}

#[derive(RustcEncodable)]
struct Ctx {
    content: String,
    title: String,
}

#[derive(Clone)]
struct RustKrConfig {
    port: u16,
    doc_dir: String,
    template: mustache::Template,
}

impl Key for RustKrConfig {
    type Value = RustKrConfig;
}

impl RustKrConfig {
    fn is_bad_title(&self, title: &str) -> bool {
        for c in title.chars() {
            match c {
                'A'...'Z' | 'a'...'z' | '0'...'9' | '_' | '-' => continue,
                _ => return true,
            }
        }

        false
    }

    fn read_page(&self, title: &str) -> io::Result<String> {
        let path = format!("{}/{}.md", self.doc_dir, title);
        let path = Path::new(&path);
        let mut f = try!(File::open(&path));
        let mut text = String::new();
        try!(f.read_to_string(&mut text));
        let md = cmark::to_html(&text);
        Ok(format!("{}", md))
    }

    pub fn list_pages(&self) -> String {
        let dir = Path::new(&self.doc_dir);
        if !dir.exists() {
            return "No pages found".to_owned();
        }

        let files = match read_dir(&dir) {
            Ok(files) => files,
            Err(_) => return "Error during reading dir".to_owned(),
        };
        let mut pages = vec![];
        for file in files {
            let file = match file {
                Ok(f) => f.path(),
                _ => continue,
            };
            if file.is_dir() {
                continue;
            }
            match file.as_os_str().to_str() {
                None => continue,
                Some(s) => {
                    if s.ends_with(".md") {
                        let pagename = file.file_stem();
                        match pagename {
                            None => continue,
                            Some(pagename) => {
                                let pagename = match pagename.to_str() {
                                    Some(p) => p,
                                    None => continue,
                                };
                                if self.is_bad_title(pagename) {
                                    continue;
                                }
                                pages.push(pagename.to_string());
                            }
                        }
                    }
                }
            }
        }

        pages.sort();

        if pages.len() > 0 {
            let mut ret = "<ul>\n".to_owned();
            for page in pages.iter() {
                ret = ret + &format!(r#"<li><a href="/pages/{}">{}</a></li>"#, *page, *page);
            }
            ret = ret + "</ul>";
            ret
        } else {
            "No pages found".to_owned()
        }
    }

    fn show_not_found(&self, _req: &mut Request) -> IronResult<Response> {
        let ctx = Ctx {
            title: "Not Found".to_owned(),
            content: "헐".to_owned(),
        };
        self.show_template(&ctx).map(|res| res.set(status::NotFound))
    }

    fn show_bad_request(&self, _req: &mut Request) -> IronResult<Response> {
        let ctx = Ctx {
            title: "Bad request".to_owned(),
            content: "헐".to_owned(),
        };
        self.show_template(&ctx).map(|res| res.set(status::BadRequest))
    }

    fn show_template(&self, ctx: &Ctx) -> IronResult<Response> {
        let mut output = vec![];
        match self.template.render(&mut output, ctx) {
            Ok(()) => {}
            Err(e) => return Err(IronError::new(TemplateError(e), &*output))
        }

        let mut res = Response::with(&*output).set(status::Ok);

        {
            let headers = &mut res.headers;

            headers.set(ContentLength(output.len() as u64));
            let content_type = mime::Mime(mime::TopLevel::Text, mime::SubLevel::Html, vec![]);
            headers.set(ContentType(content_type));
        }

        Ok(res)
    }
}

fn handle_index_page(req: &mut Request) -> IronResult<Response> {
    render_page(req, "index")
}

fn handle_page(req: &mut Request) -> IronResult<Response> {
    let title = {
        let params = req.extensions.get::<Router>().unwrap();
        params.find("title").unwrap().to_owned()
    };
    render_page(req, &title)
}

fn handle_list_pages(req: &mut Request) -> IronResult<Response> {
    let rskr = req.get_ref::<persistent::Read<RustKrConfig>>().unwrap();
    let ctx = Ctx {
        title: "모든 문서".to_owned(),
        content: rskr.list_pages(),
    };
    rskr.show_template(&ctx)
}

fn render_page(req: &mut Request, title: &str) -> IronResult<Response> {
    let rskr = req.get::<persistent::Read<RustKrConfig>>().unwrap();
    let content = rskr.read_page(title);
    let ctx = match content.ok() {
        Some(content) => Ctx {
            title: title.to_owned(),
            content: content,
        },
        None => {
            return rskr.show_not_found(req);
        }
    };
    rskr.show_template(&ctx)
}

fn main() {
    env_logger::init().unwrap();

    let mut opts = getopts::Options::new();
    opts.optopt("p", "port", "server port number", "PORT");
    opts.optopt("", "docs", "path of markdown docs", "PATH");
    opts.optopt("", "static", "path of static files", "PATH");
    opts.optopt("", "template", "template path", "PATH");
    opts.optopt("", "num-threads", "size of thread pool", "NUM");

    let args: Vec<_> = std::env::args().skip(1).collect();
    let matches = opts.parse(&args).ok().expect("Bad opts");
    let port: u16 = matches.opt_str("port").unwrap_or("8000".to_string()).parse().unwrap();
    let doc_dir = matches.opt_str("docs").unwrap_or("docs".to_string());
    let static_dir = matches.opt_str("static").unwrap_or("static".to_string());
    let template_path = matches.opt_str("template")
                               .unwrap_or("templates/default.mustache".to_string());

    debug!("port: {} / doc_dir: {} / static_dir: {} / template_path: {}",
           port, doc_dir, static_dir, template_path);

    let template = mustache::compile_path(Path::new(&template_path)).unwrap();

    let rskr = RustKrConfig {
        port: port,
        doc_dir: doc_dir,
        template: template,
    };

    let mut handler = Chain::new({
        let mut mount = Mount::new();
        mount.mount("/static", Static::new(Path::new(&static_dir)));
        mount.mount("/", router!(
            get "/"       => handle_index_page,
            get "/pages"  => handle_list_pages,
            get "/pages/_pages"  => handle_list_pages,  // legacy
            get "/pages/:title" => handle_page
        ));
        mount
    });

    handler.link(persistent::Read::<RustKrConfig>::both(rskr));

    let server = Iron::new(handler);

    let addr = (Ipv4Addr::new(127, 0, 0, 1), port);
    server.http(addr).unwrap();
    debug!("listening...");
}
