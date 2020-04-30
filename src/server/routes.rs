//! Server configuration
//! Supported routes and preferences
use crate::server::{Image, SupportedRequest};
use crate::server::extractor::{ApiJson, ApiUrl};

use actix_web::{web, guard, HttpResponse, Result};

use actix_multipart::Multipart;

/// Post request method for [`SupportedRequest`] types
/// Creates ['Image'] from path(as a name) and extracted data from request,
/// Store it in database and return response with name
///
/// # Errors
/// If extraction filed
/// If database storing failed
///
async fn create<T: Into<SupportedRequest>>(path: web::Path<String>, request: T) -> Result<HttpResponse> {
    let data = request.into().try_extract().await?;
    let image = Image::new(path.into_inner(), data)?;
    let preview = image.generate_preview()?;
    image.store(std::path::Path::new("./images"))?;
    preview.store(std::path::Path::new("./images"))?;
    Ok(HttpResponse::Ok()
        .body(format!("Image {} uploaded", image.name())))
}


/// Get request method
/// Loads ['Image'] from server's database and creates "image/jpeg" response with preview
///
/// # Errors
/// If not exist in database
/// If preview generation failed
///
// async fn find(name: web::Path<String>) -> Result<HttpResponse> {
//     let image = Image::find(name.into_inner())?;
//     Ok(HttpResponse::Ok()
//         .header("content-type", "image/jpeg")
//         .body(image.generate_preview()?))
// }

/// Configure routes
/// Set guards for parsing "content-type" fields of request's headers
/// and substitutes corresponding generic type for ['create()']
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/images/{name}")
            .name("images")
            .route(
                web::post()
                    .guard(guard::Header("content-type", "application/x-www-form-urlencoded; charset=utf-8"))
                    .to(create::<ApiUrl>))

            .route(
                web::post()
                    .guard(guard::Header("content-type", "application/json"))
                    .to(create::<ApiJson>))
            .route(
                web::post()
                    .guard(MultipartTypeGuard())
                    .to(create::<Multipart>))
            // .route(
            //     web::get()
            //         .to(find)
            // )
    );
}

/// Manual ['Guard'] for multipart/form-data content type.
/// Created because in the standard content-type
/// field a multipart request contains an automatically
/// generated boundary. Using the standard actix HeaderGuard
/// it is not possible to match multipart/form-data type.
///
pub struct MultipartTypeGuard();

impl guard::Guard for MultipartTypeGuard {
    fn check(&self, req: &actix_web::dev::RequestHead) -> bool {
        let field = "content-type";
        let required = "multipart/form-data";
        if let Some(val) = req.headers.get(field) {
            return val.as_bytes().starts_with(required.as_bytes());
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};
    use crate::server::JsonMessage;

    // const REFERENCE_PNG: &[u8] = &[137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 1, 0, 0, 0, 1, 8, 6, 0, 0, 0, 31, 21, 196, 137, 0, 0, 0, 4, 115, 66, 73, 84, 8, 8, 8, 8, 124, 8, 100, 136, 0, 0, 0, 11, 73, 68, 65, 84, 8, 153, 99, 248, 15, 4, 0, 9, 251, 3, 253, 227, 85, 242, 156, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130];


    #[actix_rt::test]
    async fn test_index_get() {
        let mut app = test::init_service(App::new()
            .route("/images/{name}",web::post()
            .guard(guard::Header("content-type", "application/json"))
            .to(create::<ApiJson>))
        ).await;

        // simple base64 encoded one pixel .png;
        let simple_json = JsonMessage {
            data: "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAABHNCSVQICAgIfAhkiAAAAAtJREFUCJlj+A8EAAn7A/3jVfKcAAAAAElFTkSuQmCC".to_string()
        };

        let req = test::TestRequest::post().uri("/images/test_json")
            .set_json(&simple_json)
            .to_request();

        let resp: JsonMessage = test::read_response_json(&mut app, req).await;
        // let resp = test::call_service(&mut app, req).await;
        // let x = 0;
        assert_eq!(resp.data.as_str(), "Base64 decoding failed");
    }

    // #[actix_rt::test]
    // async fn test_index_post() {
        // let mut app = test::init_service(App::new().route("/", web::get().to(index))).await;
        // let req = test::TestRequest::post().uri("/").to_request();
        // let resp = test::call_service(&mut app, req).await;
        // assert!(resp.status().is_client_error());
    // }
}