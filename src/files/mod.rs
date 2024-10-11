use axum::{extract::{Multipart, Request}, http::{header, StatusCode}};
use mime_guess::from_path;
use tokio::{fs::File, io::{AsyncReadExt, AsyncWriteExt}, time};

pub async fn create_file(mut multipart: Multipart) -> Result<StatusCode, StatusCode> {
    let time= time::Instant::now();
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        let _name = field.name().unwrap_or("file").to_string();
        let file_name = match field.file_name().map(|x| x.to_string()) {
            Some(file_name) => file_name,
            None => continue,
        };

        let data = field
            .bytes()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;


        let mut file = File::create(format!("./uploads/{}", file_name))
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        file.write_all(&data)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        println!("Uploaded file: {}, size: {}", file_name, data.len());
    }
    println!("Time taken: {:?}", time.elapsed());

    Ok(StatusCode::OK)
}

pub async fn read_file(Path(file_name): Path<String>) -> impl axum::response::IntoResponse {
    println!("Reading file: {}", file_name);
    let file_path = format!("./uploads/{}", file_name);
    let mut file = File::open(file_path.clone())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).unwrap();


    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).unwrap();

    let mime_type = from_path(file_path).first_or_octet_stream();
    let mime_type = mime_type.to_string();
    (StatusCode::OK,
        [
            (header::CONTENT_TYPE, mime_type),
            (header::CONTENT_DISPOSITION, format!("attachment; filename=\"{file_name}\"").to_string()),
        ],
        buffer,)



}
