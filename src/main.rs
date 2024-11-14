use anyhow::Result;
use rldr::Reloader;

fn main() -> Result<()> {
    let reloader = Reloader::new();
    reloader.run()?;
    return Ok(());
}
