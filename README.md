# chelaid - ID generator

chelaid = chelate + id

## Development

### Structure

```
domain::
  value_object::
    id
  repository::
    id

app::
  usecase::
    generate
    stats

infra::
  interface::
    cli
    memcached_binary
    memcached_text_meta
    memcached_text_basic
    http
    grpc
  repository::
    id
```

## LICENSE

[MIT](./LICENSE)

## AUTHOR

[handlename](https://github.com/handlename)
