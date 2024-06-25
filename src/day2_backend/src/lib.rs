#[ic_cdk::query]
fn greet(name: String, surname: f32) -> String {
    format!("Hello, {} {}!", name, surname)
}
