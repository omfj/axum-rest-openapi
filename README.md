# Axum REST API with OpenAPI and Utoipa

Start the application, migrations will be run for you.

```sh
cargo run
```

See documentation at:

- Swagger UI: http://localhost:3000/swagger/
- Scalar UI: http://localhost:3000/scalar/

In a another terminal try these endpoints.

```sh
# Health check
curl http://localhost:3000/

# Get all posts
curl http://localhost:3000/posts | jq

# Get posts from a specific users
curl http://localhost:3000/user/1/posts | jq

# Create a post for a user
curl -H "Authorization: Bearer token123" \
        -H "Content-Type: application/json" \
        -d '{"title": "My first post", "content": "This is the body of my first post."}' \
        -X POST http://localhost:3000/posts

```
