use async_openai::{
    types::{CreateImageRequestArgs, ImageSize, ResponseFormat},
    Client,
};
use std::error::Error;
use std::env;
//use dotenv::dotenv;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // create client, reads OPENAI_API_KEY environment variable for API key.
    //dotenv().ok();

    //let _api_key = env::var("OPENAI_API_KEY")
    //    .expect("OPENAI_API_KEY must be set");

    let prompt = gather_prompt();
    let count = gather_img_count() as u8;

    let client = Client::new();

    let request = CreateImageRequestArgs::default()
        .prompt(prompt)
        .n(count)
        .response_format(ResponseFormat::Url)
        .size(ImageSize::S1024x1024)
        .user("rust-img-gen")
        .build()?;

    let response = client.images().create(request).await?;

    // Download and save images to ./data directory.
    // Each url is downloaded and saved in dedicated Tokio task.
    // Directory is created if it doesn't exist.
    let paths = response.save("./data").await?;

    paths
        .iter()
        .for_each(|path| println!("Image file path: {}", path.display()));

    Ok(())
}

fn gather_prompt() -> String {
    // Prompt the user for the prompt
    print!("Enter the image prompt: ");
    io::stdout().flush().expect("Failed to flush stdout"); // Ensure the prompt is printed before reading input

    let mut prompt = String::new();
    io::stdin().read_line(&mut prompt).expect("Failed to read line");
    prompt.trim().to_string() // Trim any extra whitespace or newlines and return
}

fn gather_img_count() -> usize{
    loop {
        print!("How many?:");
        io::stdout().flush().expect("Failed to flush stdout"); // Ensure the prompt is printed before

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Try to parse the input as a number
        match input.trim().parse::<usize>() {
            Ok(number) => return number,
            Err(_) => println!("Please enter a valid number."),
        }
    }
}
