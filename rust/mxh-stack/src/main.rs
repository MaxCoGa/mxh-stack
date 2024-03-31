use std::{
    collections::HashMap, 
    env,     
    fs::File,
    io::{self, Read as _},
    path::PathBuf
};

use actix_utils::future::{ready, Ready};
use actix_web::{
    get, post,
    dev::{self, ServiceResponse},
    // Error,
    http::{header::ContentType, StatusCode},
    middleware::{ErrorHandlerResponse, ErrorHandlers, Logger},
    web, App, FromRequest, HttpRequest, HttpResponse, HttpServer, Responder, Result,
    cookie::Key,
};
use actix_session::{Session, SessionMiddleware, storage::CookieSessionStore};
use actix_web_lab::respond::Html;
use actix_files::Files;  
use minijinja::path_loader;
use minijinja_autoreload::AutoReloader;
use openssl::{
    pkey::{PKey, Private},
    ssl::{SslAcceptor, SslFiletype, SslMethod},
};

struct MiniJinjaRenderer {
    tmpl_env: web::Data<minijinja_autoreload::AutoReloader>,
}

impl MiniJinjaRenderer {
    fn render(
        &self,
        tmpl: &str,
        ctx: impl Into<minijinja::value::Value>,
    ) -> actix_web::Result<Html> {
        self.tmpl_env
            .acquire_env()
            .map_err(|_| actix_web::error::ErrorInternalServerError("could not acquire template env"))?
            .get_template(tmpl)
            .map_err(|_| actix_web::error::ErrorInternalServerError("could not find template"))?
            .render(ctx.into())
            .map(Html)
            .map_err(|err| {
                log::error!("{err}");
                actix_web::error::ErrorInternalServerError("template error")
            })
    }
}

impl FromRequest for MiniJinjaRenderer {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _pl: &mut dev::Payload) -> Self::Future {
        let tmpl_env = <web::Data<minijinja_autoreload::AutoReloader>>::extract(req)
            .into_inner()
            .unwrap();

        ready(Ok(Self { tmpl_env }))
    }
}

async fn index(
    tmpl_env: MiniJinjaRenderer,
    query: web::Query<HashMap<String, String>>,
) -> actix_web::Result<impl Responder> {
    if let Some(name) = query.get("name") {
        tmpl_env.render(
            "user.html",
            minijinja::context! {
                name,
                text => "Welcome!",
            },
        )
    } else {
        tmpl_env.render("index.html", ())
    }
}

// FALLBACK favicon
#[get("/favicon.ico")]
async fn favicon() -> actix_web::Result<actix_files::NamedFile> {
    Ok(actix_files::NamedFile::open("static/favicon.ico")?)
}



async fn sessionCookieTest(session: Session) -> Result<HttpResponse, actix_web::Error> {
    // access session data
    if let Some(count) = session.get::<i32>("counter")? {
        session.insert("counter", count + 1)?;
    } else {
        session.insert("counter", 1)?;
    }

    Ok(HttpResponse::Ok().body(format!(
        "Count is {:?}!",
        session.get::<i32>("counter")?.unwrap()
    )))
}



use mongodb::{Client, options::ClientOptions, Database};

// use std::error::Error;
async fn startmongodb() -> Result<Client, mongodb::error::Error>  { 
    // mongodb
    println!("STARTING MONGODB...");
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;

    // Manually set an option.
    // client_options.app_name = Some("My App".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;

    // List the names of the databases in that deployment.
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }
    // Ok(())
    return Ok(client);
}
// async fn create_database_connection() -> Result<Client, mongodb::error::Error> {
//     // dotenv().ok(); //Loading environment variables from .env file
//     // let connection_parameters = mongo_connection::ConnectionString{
//     //     username: env::var("USERNAME").expect("No username found on .env"),
//     //     password: env::var("PASSWORD").expect("No password found on .env"),
//     //     cluster: env::var("CLUSTER").expect("No cluster found on .env")
//     // };
//     // let mut url: String = mongo_connection::ConnectionString::build_connection_string();
//     // println!("{}", url);
//     let options = ClientOptions::parse("mongodb://localhost:27017").await?; //&url
//     return Client::with_options(options).await;
// }

struct AppState {
    // pub db: Database,
    pub client: Client
 }    
//  https://helabenkhalfallah.medium.com/rust-rest-api-actix-mongo-db-abc128ce5857


// Import module from src/routes/home
mod routes;
pub use routes::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // When using `Key::generate()` it is important to initialize outside of the
    // `HttpServer::new` closure. When deployed the secret key should be read from a
    // configuration file or environment variables.
    let secret_key = Key::generate();

    // load TLS keys
    // to create a self-signed temporary cert for testing:
    // `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/C=CN/CN=localhost'`
    // windows fix btw: https://stackoverflow.com/questions/65553557/why-rust-is-failing-to-build-command-for-openssl-sys-v0-9-60-even-after-local-in
    // build TLS config from files
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

    // set the encrypted private key
    builder
        .set_private_key(&load_encrypted_private_key())
        .unwrap();
    // builder
    //     .set_private_key_file("./cert/key-pass.pem", openssl::ssl::SslFiletype::PEM)
    //     .unwrap();

    // set the certificate chain file location
    builder.set_certificate_chain_file("./cert/cert-pass.pem").unwrap();


    // MONGODB CLIENT
    // let client = startmongodb().await.unwrap();
    // let client_data = web::Data::new(startmongodb());
    // A Client is needed to connect to MongoDB:
    // let client_uri = "mongodb://127.0.0.1:27017";
    // let mut options = ClientOptions::parse(&client_uri).await?;
    // let client = Client::with_options(options)?;
    // let db = client.database("my_database");
    //https://github.com/actix/examples/blob/master/databases/mongodb/src/main.rs
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    let client = Client::with_uri_str(uri).await.expect("failed to connect");
    // create_username_index(&client).await;

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // If TEMPLATE_AUTORELOAD is set, then the path tracking is enabled.
    let enable_template_autoreload = env::var("TEMPLATE_AUTORELOAD").as_deref() == Ok("true");

    if enable_template_autoreload {
        log::info!("template auto-reloading is enabled");
    } else {
        log::info!(
            "template auto-reloading is disabled; run with TEMPLATE_AUTORELOAD=true to enable"
        );
    }

    // The closure is invoked every time the environment is outdated to recreate it.
    let tmpl_reloader = AutoReloader::new(move |notifier| {
        let mut env: minijinja::Environment<'static> = minijinja::Environment::new();

        let tmpl_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("templates");

        // if watch_path is never called, no fs watcher is created
        if enable_template_autoreload {
            notifier.watch_path(&tmpl_path, true);
        }

        env.set_loader(path_loader(tmpl_path));

        Ok(env)
    });

    let tmpl_reloader = web::Data::new(tmpl_reloader);

    log::info!("starting HTTP server at http://localhost:8080");
    log::info!("starting HTTPS server at https://localhost:8443");

    HttpServer::new(move || {
        App::new()
            // SESSION COOKIE
            .wrap(
                // create cookie based session middleware
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_secure(false)
                    .build()
            )
            .service(web::resource("/session").to(sessionCookieTest)) // test session cookie

            // MONGODB CONNECTION
            // .app_data(client_data)
            .app_data(web::Data::new(client.clone()))

            .app_data(tmpl_reloader.clone())
            .service(web::resource("/").route(web::get().to(index)))
            .service(Files::new("/static","./static/"))
            .service(favicon)
            // .service(Files::new("/static", std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("static")))
            .route("/home", web::get().to(home))

            // LOGIN
            .route("/login", web::get().to(login_form))
            .route("/login", web::post().to(login))

            // ERROR CODE
            .wrap(ErrorHandlers::new().handler(StatusCode::NOT_FOUND, not_found))
            .wrap(Logger::default())
    })
    .workers(2)
    .bind(("0.0.0.0", 8080))?
    .bind_openssl("0.0.0.0:8443", builder)?
    .run()
    .await
}

/// Error handler for a 404 Page not found error.
fn not_found<B>(svc_res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let res = get_error_response(&svc_res, "Page not found");

    Ok(ErrorHandlerResponse::Response(ServiceResponse::new(
        svc_res.into_parts().0,
        res.map_into_right_body(),
    )))
}

/// Generic error handler.
fn get_error_response<B>(res: &ServiceResponse<B>, error: &str) -> HttpResponse {
    let req = res.request();

    let tmpl_env = MiniJinjaRenderer::extract(req).into_inner().unwrap();

    // Provide a fallback to a simple plain text response in case an error occurs during the
    // rendering of the error page.
    let fallback = |err: &str| {
        HttpResponse::build(res.status())
            .content_type(ContentType::plaintext())
            .body(err.to_string())
    };

    let ctx = minijinja::context! {
        error => error,
        status_code => res.status().as_str(),
    };

    match tmpl_env.render("error.html", ctx) {
        Ok(body) => body
            .customize()
            .with_status(res.status())
            .respond_to(req)
            .map_into_boxed_body(),

        Err(_) => fallback(error),
    }
}


fn load_encrypted_private_key() -> PKey<Private> {
    let mut file = File::open("./cert/key-pass.pem").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");

    PKey::private_key_from_pem_passphrase(&buffer, b"allo").unwrap()
}