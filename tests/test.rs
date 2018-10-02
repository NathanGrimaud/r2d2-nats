extern crate nats;
extern crate r2d2;
extern crate r2d2_nats;

use r2d2_nats::NatsConnectionManager;
use std::sync::mpsc;
use std::thread;

#[test]
fn test_basic() {
    let manager = NatsConnectionManager::new("nats://user:password@127.0.0.1".to_owned()).unwrap();
    let pool = r2d2::Pool::builder().max_size(2).build(manager).unwrap();

    let (s1, r1) = mpsc::channel();
    let (s2, r2) = mpsc::channel();

    let pool1 = pool.clone();
    let t1 = thread::spawn(move || {
        let conn = pool1.get().unwrap();
        s1.send(()).unwrap();
        r2.recv().unwrap();
        drop(conn);
    });

    let pool2 = pool.clone();
    let t2 = thread::spawn(move || {
        let conn = pool2.get().unwrap();
        s2.send(()).unwrap();
        r1.recv().unwrap();
        drop(conn);
    });

    t1.join().unwrap();
    t2.join().unwrap();

    pool.get().unwrap();
}

#[test]
fn test_is_valid() {
    let manager = NatsConnectionManager::new("nats://user:password@127.0.0.1".to_owned()).unwrap();
    let pool = r2d2::Pool::builder().max_size(1).build(manager).unwrap();
    pool.get().unwrap();
}

#[test]
fn test_publish() {
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
