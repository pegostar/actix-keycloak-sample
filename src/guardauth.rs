use actix_web::guard::{Guard, GuardContext};
use actix_web::http::header;
use actix_web::HttpResponse;
use serde_json::{json, Value};
use crate::openid::get_introspect;
use crate::structs::base::AuthGuard;

/// This trait implements the Guard trait for the AuthGuard struct
/// It checks if the request has a valid authorization header and returns a boolean value
impl Guard for AuthGuard {
    /// This method takes a reference to a GuardContext as an argument and returns a boolean value
    /// It checks if the request has a valid authorization header by calling the get_introspect function
    /// If the header is missing or invalid, it returns false and sends an HTTP response with status code 401 and an error message
    /// If the header is valid, it returns true
    fn check(&self, ctx: &GuardContext<'_>) -> bool  {
        // The auth_header variable stores the value of the authorization header from the request
        let auth_header = ctx.head().headers().get(header::AUTHORIZATION);

        // If the auth_header is None, it means the request has no authorization header
        if auth_header.is_none() {
            // The method sends an HTTP response with status code 401 and an error message as the body
            HttpResponse::Unauthorized().json(json!({"error" : "Access denied"}));
            return false;

        } else {
            // The bearer variable stores the string representation of the auth_header value
            let bearer:&str = auth_header.unwrap().to_str().unwrap();
            // The access variable stores the substring of the bearer value that contains the access token
            let access = &bearer[7..];

            // The path variable stores the URL of the introspection endpoint of the authentication server
            let path = "https://pego/auth/realms/pegostar/protocol/openid-connect/token/introspect";

            let payload = json!({
            "client_id": "pegostar",
            "client_secret": "0a396b26-401e-49ad-a7c4-049ffe69c03e",
            "token": access
             });

            let mut response: serde_json::Value=Default::default();

            // The handle variable stores the result of spawning a new thread that calls the get_introspect function with the path and payload as arguments
            let handle = std::thread::spawn(move || {
                // The validtoken variable stores a boolean value that indicates if the access token is valid or not
                let mut validtoken = false;

                // The data variable stores the result of calling the get_introspect function
                let data = get_introspect(path, payload);

                // If the data is Ok, it means the function succeeded
                if data.is_ok() {
                    // The response variable is updated with the value of data
                    response = data.unwrap();

                    // The status variable stores an optional reference to the name field of the response value
                    let status: Option<&Value> = response.get("name");

                    // If the status is Some, it means the access token is valid and has a name associated with it
                    if status.is_some() {
                        validtoken = true;
                    }
                }

                validtoken
            });

            // The check variable stores the result of joining the handle thread and unwrapping its value
            let check =handle.join().unwrap();

            // If check is false, it means the access token is invalid or expired
            if !check {
                // The method sends an HTTP response with status code 401 and an error message as the body
                HttpResponse::Unauthorized().json(json!({"error" : "Access denied"}));
            }

            return check;
        }
    }
}
