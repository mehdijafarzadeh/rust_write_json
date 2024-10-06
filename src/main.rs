use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::Write;

/// Represents a paragraph with a single sentence.
#[derive(Serialize, Deserialize)]
struct Paragraph {
    /// The content of the paragraph.
    name: String,
}

/// Represents an article with a title, an author, and multiple paragraphs.
#[derive(Serialize, Deserialize)]
struct Article {
    /// The title of the article.
    article: String,
    /// The author of the article.
    author: String,
    /// A list of paragraphs contained in the article.
    paragraphs: Vec<Paragraph>,
}

/// Serializes an `Article` struct to JSON and writes it to a file.
///
/// # Arguments
/// * `article` - A reference to the `Article` struct.
/// * `file_path` - The file path to write the JSON content to.
///
/// # Returns
/// * `Result<(), Box<dyn Error>>` - Returns `Ok(())` if successful, or an error if writing fails.
fn write_article_to_json_file(article: &Article, file_path: &str) -> Result<(), Box<dyn Error>> {
    // Serialize the article to a JSON string.
    let json = serde_json::to_string_pretty(article)?;

    // Open a file for writing, creating it if it doesn't exist.
    let mut file = File::create(file_path)?;

    // Write the JSON string to the file.
    file.write_all(json.as_bytes())?;

    println!("Article has been written to {}", file_path);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Create an example article.
    let article = Article {
        article: String::from("How to work with JSON in Rust"),
        author: String::from("Kani"),
        paragraphs: vec![
            Paragraph {
                name: String::from("First sentence"),
            },
            Paragraph {
                name: String::from("Body of paragraph"),
            },
            Paragraph {
                name: String::from("End of the paragraph"),
            },
        ],
    };

    // Define the file path to save the JSON output.
    let file_path = "article.json";

    // Write the article to the specified JSON file.
    write_article_to_json_file(&article, file_path)?;

    Ok(())
}
