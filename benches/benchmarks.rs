use criterion::{black_box, criterion_group, criterion_main, Criterion};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string, to_value, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub user_id: i32,
    pub id: i32,
    pub title: String,
    pub body: String,
}

const POSTS_COUNT: usize = 100;

fn setup_data() -> String {
    let posts: Vec<Post> = (0..POSTS_COUNT)
        .map(|i| Post {
            user_id: i as i32,
            id: i as i32,
            title: format!("Title {}", i),
            body: format!("Body of post {}", i),
        })
        .collect();

    to_string(&posts).unwrap()
}

fn structured_de(json: &str) -> Value {
    let posts: Vec<Post> = from_str(json).unwrap();
    to_value(posts).unwrap()
}

fn unstructured_de(json: &str) -> Value {
    from_str(json).unwrap()
}

fn criterion_benchmark(c: &mut Criterion) {
    let posts_json = setup_data();

    c.bench_function("structured_de", |b| {
        b.iter(|| structured_de(black_box(&posts_json)))
    });

    c.bench_function("unstructured_de", |b| {
        b.iter(|| unstructured_de(black_box(&posts_json)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
