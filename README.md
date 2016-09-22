# `elastic_rotor`

This repo is an experiment to provide an asynchronous [`rotor_http`](https://github.com/tailhook/rotor-http) implementation of the Elasticsearch REST API.

## Notes

The idea is to provide an out-of-the-box connection pool that's suitable for streaming or other high-throughput use cases.
`rotor` itself is probably going to lose a lot of love to `tokio`, so whether this particular implementation goes anywhere is a valid question.
Personally, I think it'll depend on just _how fast_ the `futures` implementation in `hyper` ends up being.
If the difference is enough to justify `rotor` and its lack of DNS or TLS support then it might fit a niche for internal communication in a closed network.

If that turns out the be the case, then to make this client useful, it will also need to do a lot of network programming that won't be necessary in a `tokio` implementation, like self-healing in the face of network errors.
This is an interesting requirement though, because it means we could bake support for Elasticsearch's cluster health right into the protocol.