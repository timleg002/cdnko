use std::{io::Write, path::Path};
use actix_multipart::Multipart;
use actix_web::{web, Result};
use tokio_stream::StreamExt;
use uuid::Uuid;
use std::fs::File;

use crate::error::CdnkoError;


pub async fn save_file(mut payload: Multipart) -> Result<Vec<String>> {
    // Iterate over multipart stream (multiple files)

    let mut saved_files = vec![];

    while let Ok(Some(mut field)) = payload.try_next().await {
        let mut file_extension = "".to_string();
             
        if let Some(content_disposition) = field.content_disposition() {
            if let Some(filename) = content_disposition.get_filename() {
                // Acquires the file extension from the file,
                // if the extension is not present,
                // returns a blank string.
                //
                // For now, we are not saving the file name.

                file_extension = Path
                    ::new(filename)
                    .extension()
                    .map_or(
                        "".to_string(),
                        |val| format!(".{}", val.to_str().unwrap_or_default())
                    );
            }
        } else {
            return Err(CdnkoError::BadClientData.into());
        }

        let file_path = format!("{}{}", 
            Uuid
                ::new_v4()
                .to_simple()
                .to_string()
                .to_lowercase(),
            file_extension  
        );

        saved_files.push(file_path.clone());
        
        // File::create is blocking operation, use threadpool
        let mut file = web
            ::block(|| File::create(file_path)) // format!("./images/{}", file_path))
            .await?;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk?;

            // filesystem operations are blocking, we have to use threadpool
            file = web
                ::block(
                    move || file
                        .write_all(&data)
                        .map(|_| file)
                )
                .await?;
        }
    }
    
    Ok(saved_files)
}