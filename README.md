# requestr
Store, share and run http request templates easily

## Usage

```bash
requestr examples/jsonplaceholder/posts.yml -p title="My title" -p body="Some long stuff" -p id=5
requestr examples/jsonplaceholder/filter.yml -p id=1
requestr examples/postman-echo/post.yml --env examples/test.yml -p title="My title" -p summary="My summary"
```