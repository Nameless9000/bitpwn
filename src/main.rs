use std::{io::{Read, Write}, fs::File};
use tempfile::NamedTempFile;

use actix_web::{web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use actix_multipart::{
    form::{
        tempfile::{TempFile, TempFileConfig},
        MultipartForm,
    }
};
use actix_files::NamedFile;

#[derive(MultipartForm)]
struct UploadForm {
    vars: TempFile,
}

const MAX_FILE_SIZE: usize = 1024 * 15; // 15 KB

async fn index() -> impl Responder {
    // as jesus intended
    let html = r#"<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0, shrink-to-fit=no"><title>BitPwn - Purchase Unlocker</title><meta name="theme-color" content='#212529'><meta name="twitter:card" content="summary"><meta name="twitter:title" content="BitPwn - Purchase Unlocker"><meta property="og:title" content="BitPwn - Purchase Unlocker"><meta name="description" content="The free bitlife purchase unlocker for android."><meta property="og:type" content="website"><meta property="og:description" content="The free bitlife purchase unlocker for android."><meta name="twitter:description" content="The free bitlife purchase unlocker for android."><link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootswatch@5.2.3/dist/darkly/bootstrap.min.css"><link rel="manifest" href="https://pastebin.com/raw/y4Ps3DS7"><link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Lato:ital,wght@0,400;0,700;1,400&amp;display=swap"><style>.card{--bs-card-border-color:rgba(255,255,255,0.15)!important;color:#adb5bd}.container-fluid.dark,.container.dark{background:#212529;color:#fff}.bs-icon{--bs-icon-size:.75rem;display:flex;flex-shrink:0;justify-content:center;align-items:center;font-size:var(--bs-icon-size);width:calc(var(--bs-icon-size) * 2);height:calc(var(--bs-icon-size) * 2);color:var(--bs-primary)}.bs-icon-xs{--bs-icon-size:1rem;width:calc(var(--bs-icon-size) * 1.5);height:calc(var(--bs-icon-size) * 1.5)}.bs-icon-sm{--bs-icon-size:1rem}.bs-icon-md{--bs-icon-size:1.5rem}.bs-icon-lg{--bs-icon-size:2rem}.bs-icon-xl{--bs-icon-size:2.5rem}.bs-icon.bs-icon-primary{color:var(--bs-white);background:var(--bs-primary)}.bs-icon.bs-icon-primary-light{color:var(--bs-primary);background:rgba(var(--bs-primary-rgb),.2)}.bs-icon.bs-icon-semi-white{color:var(--bs-primary);background:rgba(255,255,255,.5)}.bs-icon.bs-icon-rounded{border-radius:.5rem}.bs-icon.bs-icon-circle{border-radius:50%}</style></head><body style="background: var(--bs-indigo);"><section class="py-4 py-xl-5"><div class="container"><div class="row gy-2 row-cols-1 d-flex justify-content-center align-items-center"><div class="col"><section><div class="container text-center dark p-3" style="border-radius: 15px;"><h3 class="text-start" style="text-align: center;">BitPwn - Purchase Unlocker</h3><p class="text-start">Free BitLife purchase unlocker for android.</p><div class="card bg-dark"><div class="card-body text-start"><h4 class="card-title">Upload MonetizationVars</h4><h6 class="text-muted card-subtitle mb-2">If you experience any issues, try deleting your MonetizationVars and retry the process.</h6><form method="post" action="/" target="_blank" enctype="multipart/form-data"><div class="mb-3"><input class="form-control" name="vars" type="file"></div><div><button class="btn btn-primary d-block w-100" type="submit">Unlock Purchases</button></div></form></div></div></div></section></div><div class="col"><section><div class="container text-center dark p-3" style="border-radius: 15px;"><h3 class="text-start" style="text-align: center;">Tutorial</h3><div class="card bg-dark"><div class="card-body text-start"><h6 class="text-muted card-subtitle mb-2">You can get ZArchiver from the play store. If it says you cannot upload, copy file to downloads first.</h6><div style="position: relative;overflow: hidden;padding-top: 56.25%;"><iframe allowfullscreen="" frameborder="0" src="https://player.vimeo.com/video/834563572" style="width: 100%;height: 100%;position: absolute;top: 0;left: 0;"></iframe></div></div></div></div></section></div></div></div><footer class="text-center bg-dark" style="margin-top: 15px;"></footer></section><footer class="text-center bg-dark"><div class="container text-white py-4 py-lg-5"><ul class="list-inline"><li class="list-inline-item me-4"><a class="link-light" href="https://www.reddit.com/r/BitLifeRebels/comments/140qx11/bitlife_purchase_unlocker/" target="_blank">Reddit Post</a></li><li class="list-inline-item"><a class="link-light" href="https://www.reddit.com/u/Crabby-Thug" target="_blank">Created by u/Crabby-Thug</a></li></ul><p class="text-muted mb-0">Copyright © 2023 BitPwn</p></div></footer><script src="https://cdn.jsdelivr.net/npm/bootstrap@5.2.3/dist/js/bootstrap.bundle.min.js"></script></body></html>"#;

    HttpResponse::Ok().body(html)
}

async fn upload(MultipartForm(mut form): MultipartForm<UploadForm>, req: HttpRequest) -> impl Responder {
    match form.vars.size {
        0 => return HttpResponse::BadRequest()
                .body(format!("The uploaded file contained zero bytes.")),
        length if length > MAX_FILE_SIZE.try_into().unwrap() => {
            return HttpResponse::BadRequest()
                .body(format!("The uploaded file is too large. Maximum size is {} bytes.", MAX_FILE_SIZE));
        },
        _ => {}
    };

    let mut buffer: String = String::new();

    let file_contents: Result<usize, std::io::Error> = form.vars.file.read_to_string(&mut buffer);
    if file_contents.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    let modified_contents: String = str::replace(&buffer, "jgnJwIT", "jgnNwIT");

    let temp_file: Result<NamedTempFile, std::io::Error> = NamedTempFile::new();
    if temp_file.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    let mut temp_file: File = temp_file
        .ok()
        .unwrap()
        .into_file();

    temp_file.write(modified_contents.as_bytes()).unwrap();

    NamedFile::from_file(temp_file, "MonetizationVars")
        .unwrap()
        .into_response(&req)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::fs::create_dir_all("./tmp")?;

    println!("Starting HTTP server at http://127.0.0.1:3000");

    HttpServer::new(|| {
        App::new()
            .app_data(TempFileConfig::default().directory("./tmp"))
            .service(
                web::resource("/")
                    .route(web::get().to(index))
                    .route(web::post().to(upload)),
            )
    })
        .bind(("127.0.0.1", 3000))?
        .workers(2)
        .run()
        .await
}
