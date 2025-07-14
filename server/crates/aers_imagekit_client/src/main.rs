// use aers_imagekit_client::ImagekitClient;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let uploader = ImagekitClient::new(
//         "public_b3JXmhrMjodPOOdBhSA7ZVmvMp8=",
//         "private_XfAev+SIN0dmZSo0M2I37YziqCY=",
//     )?;

//     let result = uploader
//         .upload_file(
//             "/Users/USER/Desktop/projects/audify/server/crates/aers_imagekit_client/src/lib.rs",
//             "oyegoke-test",
//         )
//         .await?;

//     println!("{:#?}", result);
//     Ok(())
// }



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .build()?;

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Authorization", "Basic cHJpdmF0ZV9YZkFlditTSU4wZG1aU8wTTJJMzdZemlxQ1k9Og==".parse()?);

    let form = reqwest::multipart::Form::new()
        .part("file", reqwest::multipart::Part::bytes(std::fs::read("/Users/USER/Desktop/documents /Oyegoke Rebecca.pdf")?).file_name("Oyegoke Rebecca.pdf"))
        .text("fileName", "oyegoke-test")
        .text("publicKey", "public_b3JXmhrMjodPdBhSA7ZVmvMp8=");

    let request = client.request(reqwest::Method::POST, "https://upload.imagekit.io/api/v1/files/upload")
        .headers(headers)
        .multipart(form);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}