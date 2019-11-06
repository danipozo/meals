#[cfg(test)]
use super::rocket;
use rocket::http::Status;
use rocket::local::Client;

/// Paths that should return status 200 OK are tested here
/// for that property.
#[test]
fn paths_ok() {
    let client = Client::new(rocket()).expect("Valid Rocket instance");
    let paths = vec!["/recipes", "/recipes/1", "/ingredients", "/ingredients/1"];

    for p in paths {
        let response = client.get(p).dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}

/// Paths that should return 404 Not found are tested here
/// for that property
#[test]
fn paths_not_found() {
    let client = Client::new(rocket()).expect("Valid Rocket instance");
    let paths = vec!["/recipes/100", "/ingredients/100"];

    for p in paths {
        let response = client.get(p).dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }
}
