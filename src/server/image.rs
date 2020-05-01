use crate::server::ApiError;
use crate::transformation;

use serde::{Deserialize, Serialize};
use std::io::Write;

/// The type of data that the server collects.
/// # Fields
/// name is parsed from request's URL
/// binary_data is a interior form of image. Extracted from request
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Image {
    name: String,
    binary_data: Vec<u8>,
}

impl Image {
    /// Constructs a new Image from name and binary data
    ///
    /// # Errors
    /// If image's format is unknown
    ///
    pub fn create(name: String, binary_data: Vec<u8>) -> Result<Self, ApiError> {
        if !Self::is_supported_type(&binary_data) {
            return Err(ApiError::UnsupportedImageFormat)
        }
        Ok(Image {
            name,
            binary_data,
        })
    }
    /// Store existing Image to database.
    ///
    /// # Errors
    /// If Image with same name is already stored in database
    ///
    pub fn store(&self, dir: &std::path::Path) -> Result<(), ApiError> {
        if !dir.exists() {
            std::fs::create_dir(dir)?;
        }

        let file_path = dir.join(&self.name)
            .with_extension(
                immeta::load_from_buf(&self.binary_data).unwrap()
                    .mime_type()
                    .split('/')
                    .collect::<Vec<_>>()[1]
            );


        if file_path.exists() {
            return Err(ApiError::NameExists);
        } else {
            std::fs::File::create(&file_path)?
                .write(&self.binary_data)?;
        }
        Ok(())
    }

    /// Creates 100x100 jpg preview of Image
    ///
    /// # Errors
    /// If binary_data field cannot be red as image by opencv
    ///
    pub fn generate_preview(&self) -> Result<Image, ApiError> {
        let preview_data = transformation::resize_image(&self.binary_data, 100, 100)
            .ok_or(ApiError::PreviewGeneration)?;
        Ok(Image::create("preview_".to_string() + &self.name, preview_data)?)
    }


    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_supported_type(data: &[u8]) -> bool {
        match immeta::load_from_buf(data) {
            Ok(immeta::GenericMetadata::Jpeg(_)) |
            Ok(immeta::GenericMetadata::Png(_)) => true,
            _ => false,
        }
    }
}


// #[cfg(test)]
// mod tests {
//     use super::*;
//     use dotenv::dotenv;
//     #[test]
//     fn new() {
//         let image = Image::new(String::new(), vec![0u8]);
//         assert_eq!(image.name, String::new());
//         assert_eq!(image.binary_data, vec![0u8]);
//     }
//
//
//     #[test]
//     fn store() {
//         dotenv().ok();
//         db::init();
//
//         diesel::delete(image::table).execute(&db::connection().unwrap()).unwrap();
//
//         let image = Image::new("name".to_string(), vec![0u8]);
//
//         image.store().unwrap();
//         let conn = db::connection().unwrap();
//         let count = image::table.count().get_result(&conn);
//
//         assert_eq!(Ok(1), count);
//
//         // let found = image::table
//         //     .find("name".to_string())
//         //     .first(&conn).unwrap();
//
//         // assert_eq!(image, stored);
//         // assert_eq!(image, found);
//
//         diesel::delete(image::table).execute(&db::connection().unwrap()).unwrap();
//
//     }
//
//     #[test]
//     fn store_already_exist() {
//         dotenv().ok();
//         db::init();
//
//         diesel::delete(image::table).execute(&db::connection().unwrap()).unwrap();
//
//         let image = Image::new("name".to_string(), vec![0u8]);
//         image.store().unwrap();
//
//         let second = image.store();
//
//         assert_eq!(ApiError::NameExists, second.unwrap_err());
//
//         diesel::delete(image::table).execute(&db::connection().unwrap()).unwrap();
//     }
//
//
//     #[test]
//     fn find() {
//         dotenv().ok();
//         db::init();
//
//         diesel::delete(image::table).execute(&db::connection().unwrap()).unwrap();
//
//         let image = Image::new("name".to_string(), vec![0u8]);
//
//         image.store().unwrap();
//
//         let found = Image::find("name".to_string()).unwrap();
//
//         assert_eq!(image, found);
//
//         diesel::delete(image::table).execute(&db::connection().unwrap()).unwrap();
//
//     }
//
//     #[test]
//     fn find_not_exist() {
//         dotenv().ok();
//         db::init();
//
//         diesel::delete(image::table).execute(&db::connection().unwrap()).unwrap();
//
//         let found = Image::find("not_exist_name".to_string());
//
//         assert_eq!(ApiError::RecordNotFound, found.unwrap_err());
//
//         diesel::delete(image::table).execute(&db::connection().unwrap()).unwrap();
//
//     }
//
//
//     #[test]
//     fn delete() {
//         dotenv().ok();
//         db::init();
//
//         diesel::delete(image::table).execute(&db::connection().unwrap()).unwrap();
//
//         let stored = Image::new("name".to_string(), vec![]);
//         stored.store().unwrap();
//
//         let count_deleted = Image::delete("name".to_string());
//
//         assert_eq!(Ok(1), count_deleted);
//
//         let count = image::table.count().get_result(&db::connection().unwrap());
//
//         assert_eq!(Ok(0), count);
//     }
//
//     #[test]
//     fn delete_not_exist() {
//         dotenv().ok();
//         db::init();
//
//         diesel::delete(image::table).execute(&db::connection().unwrap()).unwrap();
//
//         let stored = Image::new("name".to_string(), vec![]);
//         stored.store().unwrap();
//
//         let count_deleted = Image::delete("another_name".to_string());
//
//         assert_eq!(Ok(0), count_deleted);
//
//         let count = image::table.count().get_result(&db::connection().unwrap());
//
//         assert_eq!(Ok(1), count);
//
//         diesel::delete(image::table).execute(&db::connection().unwrap()).unwrap();
//
//     }
//
// }