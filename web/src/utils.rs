use actix_web::{
  Error, web
};
use futures::StreamExt;
use actix_multipart::Multipart;
use std::io::Write;

// pub async fn save_upload_wasm<'a>(
//   file_path: String,
//   mut payload: Multipart
// ) -> Result<(String), Error> {

//   let mut all_path: &'a str = &"";

//   while let Some(item) = payload.next().await {
//       let mut field = item?;
//       let content_type = field.content_disposition().unwrap();
//       let filename = content_type.get_filename().unwrap();
//       let path = format!("{}{}", file_path, filename);
//       all_path = &path.as_str();
//       // File::create is blocking operation, use threadpool
//       let mut f = web::block(|| std::fs::File::create(path))
//           .await
//           .unwrap();
//       // Field in turn is stream of *Bytes* object
//       while let Some(chunk) = field.next().await {
//           let data = chunk.unwrap();
//           // filesystem operations are blocking, we have to use threadpool
//           f = web::block(move || f.write_all(&data).map(|_| f)).await?;
//       }

//   }
//   Ok(all_path.to_string())
// }