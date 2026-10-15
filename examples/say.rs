// cargo run --example say -- <text>
use samiam::Voice;

fn main() -> std::io::Result<()> {
    let text = std::env::args().skip(1).collect::<Vec<_>>().join(" ");
    std::fs::write("samiam.wav", Voice::new().wav(&text))
}
