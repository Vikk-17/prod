# Email Newsletter


### Difference between unix socket and tcp socket
A UNIX socket, AKA Unix Domain Socket, is an inter-process communication mechanism that allows bidirectional data exchange between processes running on the same machine.

IP sockets (especially TCP/IP sockets) are a mechanism allowing communication between processes over the network. In some cases, you can use TCP/IP sockets to talk with processes running on the same computer (by using the loopback interface).

UNIX domain sockets know that they’re executing on the same system, so they can avoid some checks and operations (like routing); which makes them faster and lighter than IP sockets. So if you plan to communicate with processes on the same host, this is a better option than IP sockets.


## Docs

- [actix_web](https://actix.rs/)
- [actix_web / rust docs](https://docs.rs/actix-web/latest/actix_web/index.html)
- [App](https://docs.rs/actix-web/4.11.0/actix_web/struct.App.html)
- [Testing Org](https://doc.rust-lang.org/book/ch11-03-test-organization.html)
- [Actix web test](https://actix.rs/docs/testing/)
- [Menifast target specification](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#cargo-targets)



## References

- [Unix vs TCP/IP Socket](https://serverfault.com/questions/124517/what-is-the-difference-between-unix-sockets-and-tcp-ip-sockets)
- [Email Masking](https://www.kaspersky.com/resource-center/definitions/what-is-email-masking)

