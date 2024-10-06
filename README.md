# Rust JSON Article Serializer

This project demonstrates how to work with JSON in Rust using the `serde` library. The project serializes a custom `Article` structure (containing the article's title, author, and a list of paragraphs) into JSON format and writes it to a file.

## Features
- Defines custom Rust structs (`Article`, `Paragraph`) and serializes them into JSON format.
- Writes the JSON data to a file (`article.json`).
- Uses the `serde` and `serde_json` crates for serialization and deserialization.

## How It Works
1. **Article Struct**: Contains the article's title, author, and a list of paragraphs.
2. **Serialization**: The `Article` struct is serialized into a well-formatted JSON string.
3. **File Writing**: The serialized JSON is written into a file named `article.json`.

## Example Output
An example of the generated JSON file (`article.json`):
```json
{
  "article": "How to work with JSON in Rust",
  "author": "Kani",
  "paragraphs": [
    {
      "name": "First sentence"
    },
    {
      "name": "Body of paragraph"
    },
    {
      "name": "End of the paragraph"
    }
  ]
}
