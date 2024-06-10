# Publishing
- Bump up version in `Cargo.toml`
- Run `cargo publish`

# Launching
On AWS instance:
```
cargo install tbop-tcs-psi --bin server
server -s 0.0.0.0:5555
```
In another session:
```
cargo install tbop-tcs-psi --bin client
client -s 0.0.0.0:5555 -c 0.0.0.0:5556 -p
```
On Azure instance:
```
cargo install tbop-tcs-psi --bin client
client -s tcs-cars.sytes.net:5555 -c tcs-cars.sytes.net:5556
```
Enjoy.
