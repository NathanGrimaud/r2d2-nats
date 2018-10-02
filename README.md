# r2d2-nats

[rust-nats](https://github.com/jedisct1/rust-nats) support library for the [r2d2](https://github.com/sfackler/r2d2) connection pool.

This library is strongly inspired by [r2d2-redis](https://github.com/sorccu/r2d2-redis)

# Example

```rust
extern crate r2d2;
extern crate r2d2_nats;
extern crate nats;

use std::thread;
use r2d2_nats::NatsConnectionManager;


fn main() {
    let manager = NatsConnectionManager::new("nats://user:password@127.0.0.1".to_owned()).unwrap();
    let pool = r2d2::Pool::builder().build(manager).unwrap();
    let mut handles = vec![];
    for _i in 0..10i32 {
        let pool = pool.clone();
        handles.push(thread::spawn(move || {
            let mut conn = pool.get().unwrap();
            conn.publish("nats", _i.to_string().as_bytes()).unwrap()
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
}
```
