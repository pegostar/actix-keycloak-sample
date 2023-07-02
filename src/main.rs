mod structs;
mod openid;
mod guardauth;

use actix_web::{post, HttpResponse, Responder, web, HttpServer, App};
use serde_json::json;

use crate::openid::{get_token, set_logout, set_renew};
use crate::structs::base::{AuthGuard, Login, TokenApplication};

// This function is an asynchronous handler for the POST request to the "/login" endpoint
// It takes a web form of type Login as an argument and returns an implementation of Responder
#[post("/login")]
async fn login(web::Form(form): web::Form<Login>) -> impl Responder  {
    // The path variable stores the URL of the authentication server
    let path = "https://pego/auth/realms/pegostar/protocol/openid-connect/token";

    // The payload variable stores the JSON object with the login credentials and the client information
    let payload = json!({
            "username": form.username,
            "password": form.password,
            "client_id": "pegostar",
            "client_secret": "0a396b26-401e-49ad-a7c4-049ffe69c03e",
            "grant_type": "password"
        });

    // The result variable stores the result of calling the get_token function with the path and payload as arguments
    let result = get_token(path, payload).await;

    let serialized = serde_json::to_string(&result.unwrap()).unwrap();

    // The function returns an HTTP response with status code 200 and the serialized result as the body
    HttpResponse::Ok().body(serialized)
}

// This function is an asynchronous handler for the POST request to the "/renew" endpoint
// It takes a web form of type TokenApplication as an argument and returns an implementation of Responder
#[post("/renew")]
async fn renew(web::Form(form): web::Form<TokenApplication>) -> impl Responder {
    // The path variable stores the URL of the authentication server
    let path = "https://pego/auth/realms/pegostar/protocol/openid-connect/token";

    // The payload variable stores the JSON object with the client information and the refresh token
    let payload = json!({
            "client_id": "pegostar",
            "client_secret": "0a396b26-401e-49ad-a7c4-049ffe69c03e",
            "refresh_token": form.refresh_token,
            "grant_type": "refresh_token"
        });

    // The result variable stores the result of calling the set_renew function with the path, the access token and the payload as arguments
    let result = set_renew(path,form.access_token.as_str(), payload).await;

    // The serialized variable stores the JSON string representation of the result
    let serialized = serde_json::to_string(&result.unwrap()).unwrap();

    // The function returns an HTTP response with status code 200 and the serialized result as the body
    HttpResponse::Ok().body(serialized)
}

// This function is an asynchronous handler for the POST request to the "/logout" endpoint
// It takes a web form of type TokenApplication as an argument and returns an implementation of Responder
#[post("/logout")]
async fn logout(web::Form(form): web::Form<TokenApplication>) -> impl Responder {
    // The path variable stores the URL of the authentication server
    let path = "https://pego/auth/realms/pegostar/protocol/openid-connect/logout";

    // The payload variable stores the JSON object with the client information and the refresh token
    let payload = json!({
            "client_id": "pegostar",
            "client_secret": "0a396b26-401e-49ad-a7c4-049ffe69c03e",
            "refresh_token": form.refresh_token
        });

    // The function calls the set_logout function with the path, the access token and the payload as arguments
    set_logout(path,form.access_token.as_str(), payload).await;

    // The function returns an HTTP response with status code 200 and no body
    HttpResponse::Ok()
}

//Sample reserved area
async fn reserverarea() -> impl Responder {
    HttpResponse::Ok().body("Welcome to reserved area")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(login)
            .service(renew)
            .service(logout)
            .route("/reserved/test", web::get().guard(AuthGuard).to(reserverarea))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
