use std::convert::Infallible;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use ammonia::clean;
use bytes::BufMut;
use dotenv::dotenv;
use futures::TryStreamExt;
use uuid::Uuid;
use warp::{
    http::{StatusCode, Uri},
    multipart::{FormData, Part},
    Filter, Rejection, Reply,
};

use walkdir::WalkDir;

static FILES_INDEX_HEAD: &'static str = "<!DOCTYPE html PUBLIC \"-//W3C//DTD HTML 3.2 Final//EN\"><html><head><meta http-equiv=\"content-type\" content=\"text/html; charset=UTF-8\"><title>Index of /files</title></head><body><h1>Index of /files</h1><table><tbody><tr><th valign=\"top\"><img src=\"./assets/blank.gif\" alt=\"[ICO]\"></th><th>Name</th><th>Size</th><th>Description</a></th></tr><tr><th colspan=\"4\"><hr></th></tr><tr><td valign=\"top\"><img src=\"./assets/back.gif\" alt=\"[PARENTDIR]\"></td><td><a href=\"./\">Parent Directory</a> </td><td align=\"right\"> - </td><td>&nbsp;</td></tr>";

static FILES_INDEX_FOOTER: &'static str =
    "<tr><th colspan=\"4\"><hr></th></tr></tbody></table></body></html>";

#[tokio::main]
async fn main() {
    dotenv().ok();
    let hny_assets = warp::path("assets").and(warp::fs::dir("./res/assets/"));

    let hny_uploads_index = warp::path::end().and(warp::fs::file("./res/index.html"));

    let hny_backup_index = warp::path("backup").and(warp::fs::file("./res/backup/index.html"));
    let hny_sql = warp::path!("employees.sql").and(warp::fs::file("./res/backup/sakila.sql"));
    let hny_zip = warp::path!("public_html.zip").and(warp::fs::file("./res/backup/zipbomb.zip"));

    //let hny_files_index = warp::path("files").and(warp::fs::file("./res/files/index.html")); //DONE: make interactive

    let files = warp::path("files").and(warp::path::tail()).map(|reply: warp::filters::path::Tail | {
        //println!("replay_path: {:?}", reply.as_str());
        if reply.as_str() == "" {
            let mut buffer = String::from(FILES_INDEX_HEAD);
            for entry in WalkDir::new("./uploaded_files") {
                let entry = entry.unwrap();
                if entry.path().file_stem().unwrap().to_str().unwrap() != "uploaded_files" {
                    let mut size = entry.metadata().unwrap().len() as f64;
                    let mut size_fix = "";
                    if size > 1024.0 {
                        size = size / 1024.0;
                        size_fix = "K";
                        if size > 1024.0 {
                            size = size / 1024.0;
                            size_fix = "M";
                        }
                    }
                    buffer.push_str(format!("<tr><td valign=\"top\"><img src=\"./assets/binary.gif\" alt=\"[   ]\"></td><td><a href=\"./files/{0}\">{0}</a></td><td align=\"right\"> {1}{2} </td><td>&nbsp;</td></tr>", entry.clone().path().file_stem().unwrap().to_str().unwrap(), f64::trunc(size * 10.0) / 10.0, size_fix).as_str());
                }
            }
            buffer.push_str(FILES_INDEX_FOOTER);
            return warp::reply::with_header(buffer, "Content-Type", "text/html").into_response();
        }
        if reply.as_str().contains("php") || reply.as_str().contains("php5") || reply.as_str().contains("php6") || reply.as_str().contains("php7") || reply.as_str().contains("php8") {
            return warp::reply::html("PHP Engine not enabled.").into_response();
        } else if reply.as_str().contains("phtml") || reply.as_str().contains("html") || reply.as_str().contains("htm") || reply.as_str().contains("html5") {
            return warp::reply::html("Page not available").into_response();
        } else if reply.as_str().contains("mp4") || reply.as_str().contains("avi") || reply.as_str().contains("mov") || reply.as_str().contains("mkv") || reply.as_str().contains("webm") {
            return warp::redirect::redirect(Uri::from_static("https://www.youtube.com/watch?v=dQw4w9WgXcQ")).into_response();
        } else if reply.as_str().to_lowercase().contains("exe") {
            let mut buffer = Vec::new();
            let reply_name = clean(reply.as_str());
            match File::open("./res/calc.exe") {
                Ok(f) => {
                    let mut reader = BufReader::new(f);
                    reader.read_to_end(&mut buffer).unwrap();
                },
                Err(e) => eprintln!("Failed to access file: {}", e)
            }
            return warp::reply::with_header(buffer, "Content-Type", mime_guess::from_path(reply_name).first_or_text_plain().essence_str()).into_response();
        } else {
            let mut buffer = Vec::new();
            let reply_name = clean(reply.as_str());
            let path = format!("./uploaded_files/{}.disabled", reply_name);
            match File::open(path.clone()) {
                Ok(f) => {
                    let mut reader = BufReader::new(f);
                    reader.read_to_end(&mut buffer).unwrap();
                },
                Err(e) => eprintln!("Failed to access file: {}", e)
            }
            return warp::reply::with_header(buffer, "Content-Type", mime_guess::from_path(reply_name).first_or_text_plain().essence_str()).into_response();
        }
    });

    let hny_phpinfo = warp::path("phpinfo.php").and(warp::fs::file("./res/phpinfo.php.html"));

    let hny_config = warp::path("config.env").and(warp::fs::file("./res/config.env.txt"));

    let hny_upload = warp::path("upload.php").and(warp::fs::file("./res/upload.php.html"));

    let upload_route = warp::path("upload.php")
        .and(warp::post())
        .and(warp::multipart::form().max_length(8_000_000_000))
        .and_then(upload);

    let router = upload_route
        .or(hny_assets)
        .or(hny_uploads_index)
        .or(hny_backup_index)
        .or(hny_sql)
        .or(hny_zip)
        .or(files)
        .or(hny_phpinfo)
        .or(hny_config)
        .or(hny_upload)
        .recover(handle_rejection);
    warp::serve(router).run(([0, 0, 0, 0], 4000)).await;
}

async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found".to_string())
    } else if err.find::<warp::reject::PayloadTooLarge>().is_some() {
        (StatusCode::BAD_REQUEST, "Payload too large".to_string())
    } else {
        eprintln!("unhandled error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
        )
    };

    Ok(warp::reply::with_status(message, code))
}

async fn upload(form: FormData) -> Result<impl Reply, Rejection> {
    let parts: Vec<Part> = form
        .try_collect()
        .await
        .map_err(|e| {
            eprintln!("form error: {}", e);
            warp::reject::reject()
        })
        .unwrap();
    let mut return_val = String::from("success: ");
    for p in parts {
        if p.name() == "fileToUpload" {
            let file_ending = clean(p.filename().unwrap_or(""));

            let value = p
                .stream()
                .try_fold(Vec::new(), |mut vec, data| {
                    vec.put(data);
                    async move { Ok(vec) }
                })
                .await
                .map_err(|e| {
                    eprintln!("reading file error: {}", e);
                    warp::reject::reject()
                })
                .unwrap();

            let file_uuid = Uuid::new_v4().to_string();
            let mut file_name = String::new();
            if file_ending.len() > 0 {
                file_name = format!("./uploaded_files/{}-{}.disabled", file_uuid, file_ending);
            } else {
                warp::reject::reject();
            }
            return_val.push_str(
                format!(
                    "D:\\xampp\\www\\uploads\\files\\{}-{}\n",
                    file_uuid, file_ending
                )
                .as_str(),
            );
            tokio::fs::write(&file_name, value)
                .await
                .map_err(|e| {
                    eprint!("error writing file: {}", e);
                    warp::reject::reject()
                })
                .unwrap();
            if let Ok(ntfy_endpoint) = std::env::var("NTFY_ENDPOINT") {
                let _ =
                    ureq::post(format!("{ntfy_endpoint}?title=New%20Honeypot%20Upload").as_str())
                        .send_string(file_name.as_str());
            }
            println!("created file: {}", file_name);
        }
    }
    Ok(return_val)
}

/*
let content_type = p.content_type();
match content_type {
Some(file_type) => match file_type {
"application/pdf" => {
file_ending = "pdf";
        }
            "image/png" => {
            file_ending = "png";
        }
            "image/jpeg" => {
            file_ending = "jpg";
        }
            "application/php" => {
            file_ending = "jpg";
        }
            v => {
            eprintln!("invalid file type found: {}", v);
            return Err(warp::reject::reject());
        }
        },
            None => {
            eprintln!("file type could not be determined");
            return Err(warp::reject::reject());
        }
        }*/
