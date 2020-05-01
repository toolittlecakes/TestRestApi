use futures::{StreamExt, TryStreamExt};

use actix_multipart::{Multipart, Field};
use actix_web::{web, client, Result};
use serde::{Deserialize, Serialize};
use async_trait::async_trait;

use crate::server::{ApiError, Image};

pub type ApiJsonRequest = web::Json<Vec<JsonMessage>>;

pub type ApiUrlRequest = web::Json<Vec<UrlMessage>>;


/// Required structure of Json request.
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct JsonMessage {
    pub name: String,
    pub data: String,
}
/// Required structure of request with image's URL.
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UrlMessage {
    pub name: String,
    pub url: String,
}

pub struct MultipartField {
    pub field: Field
}

#[async_trait(? Send)]
pub trait TryIntoImage: Sized {
    /// Performs the conversion.
    async fn try_into_image(self) -> Result<Image>;
}

#[async_trait(? Send)]
impl TryIntoImage for JsonMessage {
    async fn try_into_image(self) -> Result<Image> {
        let JsonMessage { name, data } = self;
        let encoded: String = data.chars().filter(|ch| !ch.is_whitespace()).collect();
        let decoded = base64::decode(encoded.as_bytes()).map_err(|_| ApiError::Base64Decoding)?;
        let image = Image::create(name, decoded)?;
        Ok(image)
    }
}

#[async_trait(? Send)]
impl TryIntoImage for UrlMessage {
    async fn try_into_image(self) -> Result<Image> {
        let UrlMessage { name, url } = self;
        if url.contains("localhost") || url.contains("127.0.0.1") {
            return Err(ApiError::LocalhostUrl.into());
        }
        let client = client::Client::default();

        // Create request builder and send request
        let mut response = client.get(url)
            .header("User-Agent", "test_rest_api")
            .send()
            .await?;

        let data = response.body().await?.to_vec();
        let image = Image::create(name, data)?;
        Ok(image)
    }
}

#[async_trait(? Send)]
impl TryIntoImage for MultipartField {
    async fn try_into_image(self) -> Result<Image> {
        let mut field = self.field;
        let content_type = field.content_disposition().unwrap();
        let name = content_type.get_filename().unwrap().to_string();

        let mut buff = vec![];
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            buff.extend_from_slice(&data);
        }
        let image = Image::create(name, buff)?;
        Ok(image)
    }
}







#[async_trait(? Send)]
pub trait SupportedRequest {
    async fn extract(self) -> Vec<Result<Image>>;
}

#[async_trait(? Send)]
impl<T> SupportedRequest for web::Json<Vec<T>>
    where T: TryIntoImage
{
    async fn extract(self) -> Vec<Result<Image>> {
        let messages = self.into_inner();
        let mut images = vec![];
        for message in messages {
            images.push(message.try_into_image().await);
        }
        images
    }
}

#[async_trait(? Send)]
impl SupportedRequest for Multipart {
    async fn extract(mut self) -> Vec<Result<Image>> {
        let mut images = vec![];
        while let Ok(Some(field)) = self.try_next().await {
            let field = MultipartField { field };
            images.push(field.try_into_image().await);
        }
        images
    }
}

/*
/// The type by which the polymorphic call of extractors is implemented
/// without traits for supported types of requests
//
// It would be better to use TryFrom trait for polymorphic data extraction from different types,
// but it is currently impossible to use traits with async.
//
// So, try_extract function implements polymorphic data extraction from supported request types
// by casting requests to enum and matching it instead trait implementation
    /// Polymorphic function for extraction binary data from all supported request's types
    ///
    /// # Errors
    /// Will return error from internal extraction functions

    /// Implementation of extracting binary data from base64 encoded Json
    ///
    /// # Errors
    /// Will return error if data is not correct base64 encoded
    ///


    /// Implementation of extracting binary data from remote source by URL
        ///
        /// # Errors
        /// Will return error if cannot connect to URL or if response parsing fails
        ///


    /// Implementation of extracting binary data from multipart/form-data request
    ///
    /// # Errors
    /// Will return error if body of request is not compatible to multipart/form-data content type
    ///

*/
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use actix_web::{web, ResponseError};
//     use actix_web::http::{HeaderMap, HeaderName, header, HeaderValue};
//
//     const REFERENCE_PNG: &[u8] = &[137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 1, 0, 0, 0, 1, 8, 6, 0, 0, 0, 31, 21, 196, 137, 0, 0, 0, 4, 115, 66, 73, 84, 8, 8, 8, 8, 124, 8, 100, 136, 0, 0, 0, 11, 73, 68, 65, 84, 8, 153, 99, 248, 15, 4, 0, 9, 251, 3, 253, 227, 85, 242, 156, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130];
//
//
//     #[actix_rt::test]
//     async fn from_json() {
//         let simple_json = web::Json(JsonMessage {
//             data: base64::encode(REFERENCE_PNG)
//         });
//
//         let req = SupportedRequest::from(simple_json);
//
//         let res = req.try_extract().await.unwrap();
//
//         assert_eq!(REFERENCE_PNG, res.as_slice());
//     }
//
//
//     #[actix_rt::test]
//     async fn from_json_invalid() {
//         let simple_json = web::Json(JsonMessage {
//             data: "invalid base64".to_string()
//         });
//
//         let req = SupportedRequest::from(simple_json);
//
//         let res = req.try_extract().await;
//         assert!(res.is_err());
//         assert_eq!(ApiError::Base64Decoding.status_code(), res.unwrap_err().as_response_error().status_code());
//     }
//
//     // #[actix_rt::test]
//     // async fn from_url() {
//     //     let simple_json = web::Form(FormUrl {
//     //         url:
//     //     });
//     //
//     //     let req = SupportedRequest::from(simple_json);
//     //
//     //     let res = req.try_extract().await.unwrap();
//     //
//     //     assert_eq!(REFERENCE_PNG, res.as_slice());
//     // }
// }