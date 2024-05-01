use criterion::{black_box, criterion_group, criterion_main, Criterion};
use serde::{Deserialize, Serialize};
use serde_json::from_str;

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub user_id: i32,
    pub id: i32,
    pub title: String,
    pub body: String,
}

fn deserialize_post(json: &str) -> Post {
    from_str(json).unwrap()
}

fn criterion_benchmark(c: &mut Criterion) {
    let post_json = r#"{
        "user_id": 1,
        "id": 101,
        "title": "Example Title",
        "body": "This is the body of the post."
    }"#;

    c.bench_function("deserialize_post", |b| {
        b.iter(|| deserialize_post(black_box(post_json)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
