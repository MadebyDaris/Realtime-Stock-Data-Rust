### Using Reqwest and Serde in rust
# What is serde and why is it useful?
# Reqwest
first need to understand the difference between blocking client and client
- Concurrency: The non-blocking client allows for concurrent request handling, while the blocking client handles requests sequentially.
- Runtime: The non-blocking client requires an asynchronous runtime like Tokio or async-std, while the blocking client does not.

## Response and client