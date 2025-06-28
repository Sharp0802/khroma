
# Khroma

An idiomatic, asynchronous, and high-level Rust SDK for interacting with a [ChromaDB](https://www.trychroma.com/) vector database.

This library provides a safe and ergonomic interface, abstracting away the raw HTTP requests into a stateful, object-oriented API that is a joy to use.

## Features

-   **Fluent, High-Level API:** Interact with Chroma through clean, stateful objects: `Khroma` -> `Tenant` -> `Database` -> `Collection`.
-   **Fully Asynchronous:** Built on `tokio` and `reqwest` for non-blocking I/O, perfect for high-performance applications.
-   **Type-Safe:** All API models are strongly typed, ensuring correctness at compile time.
-   **Ergonomic Error Handling:** A single `KhromaError` enum makes handling API and network errors straightforward.
-   **Complete API Coverage:** Provides access to the full range of ChromaDB v2 API endpoints, from server health checks to complex collection queries.

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
khroma = "0.1.0"
```

## Quick Start

Here is a complete example of how to connect to Chroma, ensure a collection exists, upsert some data, and perform a query.

```rust
use khroma::Khroma;
use khroma::models::{
    CreateCollectionPayload,
    EmbeddingsPayload,
    QueryRequestPayload,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Create a new Khroma client
    let client = Khroma::new("http://localhost:8000", None)?;
    println!("Server version: {}", client.version().await?);

    // 2. Get or create a tenant and database
    let tenant = match client.get_tenant("my-tenant").await {
        Ok(tenant) => tenant,
        Err(_) => client.create_tenant("my-tenant").await?,
    };
    let database = match tenant.get_database("my-database").await {
        Ok(db) => db,
        Err(_) => tenant.create_database("my-database").await?,
    };

    // 3. Get or create a collection using the built-in helper
    let collection = database.get_or_create_collection(
        CreateCollectionPayload {
            name: "my-awesome-collection".to_string(),
            ..Default::default()
        }
    ).await?;

    println!("Collection '{}' is ready.", collection.name);

    // 4. Add or update (upsert) records
    collection.upsert(&khroma::models::UpsertCollectionRecordsPayload {
        ids: vec!["id1".into(), "id2".into()],
        embeddings: Some(EmbeddingsPayload::Float(vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
        ])),
        documents: Some(vec![
            Some("This is a document about Rust.".into()),
            Some("This is a document about ChromaDB.".into()),
        ]),
        ..Default::default()
    }).await?;

    println!("Upserted 2 records.");
    println!("Collection now has {} records.", collection.count().await?);

    // 5. Query the collection
    let query_result = collection.query(
        &QueryRequestPayload {
            query_embeddings: vec![vec![1.1, 2.1, 3.1]], // Find vectors similar to this
            n_results: Some(1),
            ..Default::default()
        },
        None, // limit
        None, // offset
    ).await?;

    println!("Query results: {:?}", query_result.documents);
    // Expected output: Some([["This is a document about Rust."]])

    // 6. Clean up
    database.delete_collection(&collection.id.to_string()).await?;
    println!("Cleaned up collection.");

    Ok(())
}
```

## API Concepts

The SDK is designed around a hierarchy of stateful handles. This makes the API intuitive and reduces the need to pass IDs repeatedly.

-   `Khroma`: The main entry point. Used for server-level operations (`version`, `heartbeat`) and for getting `Tenant` handles.
-   `Tenant`: Represents a specific tenant. Used to manage databases within that tenant (`create_database`, `get_database`).
-   `Database`: Represents a database within a tenant. Used to manage collections (`create_collection`, `list_collections`).
-   `Collection`: Represents a collection. This is where most of the work happens: `add`, `upsert`, `query`, `get`, `delete`, etc.

## Detailed Examples

### Filtering with `where` clauses

You can filter `get`, `query`, and `delete` operations using `where` and `where_document` clauses. Use the `serde_json::json!` macro for easy filter creation.

```rust
use khroma::models::{GetRequestPayload, RawWhereFields};
use serde_json::json;

// Assume `collection` is a valid handle from the Quick Start example.
// Add metadata to your records
collection.upsert(&khroma::models::UpsertCollectionRecordsPayload {
    ids: vec!["id3".into(), "id4".into()],
    metadatas: Some(vec![
        Some(json!({"topic": "rust", "year": 2023}).as_object().unwrap().clone()),
        Some(json!({"topic": "ai", "year": 2023}).as_object().unwrap().clone()),
    ]),
    ..Default::default()
}).await?;

// Get records where topic is "rust"
let get_result = collection.get(&GetRequestPayload {
    where_fields: RawWhereFields {
        r#where: Some(json!({"topic": "rust"})),
        ..Default::default()
    },
    ..Default::default()
}).await?;

println!("Filtered get results: {:?}", get_result.ids);
// Expected output: ["id3"]
```

### Deleting Records

You can delete records by ID or by a `where` filter.

```rust
use khroma::models::DeleteCollectionRecordsPayload;
use serde_json::json;

// Delete by ID
collection.delete(&DeleteCollectionRecordsPayload {
    ids: Some(vec!["id1".to_string()]),
    ..Default::default()
}).await?;

// Delete by metadata filter
collection.delete(&DeleteCollectionRecordsPayload {
    where_fields: khroma::models::RawWhereFields {
        r#where: Some(json!({"year": 2023})),
        ..Default::default()
    },
    ..Default::default()
}).await?;
```

## Error Handling

All fallible API calls return a `Result<T, KhromaError>`. The `KhromaError` enum provides detailed information about the cause of the failure:

-   `KhromaError::Reqwest`: For network or transport-level errors.
-   `KhromaError::Api`: For errors returned by the ChromaDB server (e.g., 404 Not Found, 401 Unauthorized). Includes the status code and server message.
-   `KhromaError::Parse`: For issues deserializing the server's response.
-   `KhromaError::Url`: For malformed base URLs.

## Contributing

Contributions are welcome! Please feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.
