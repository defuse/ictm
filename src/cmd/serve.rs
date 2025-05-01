use anyhow::Result;
use warp::Filter;

pub async fn run_serve(dir: String, port: u16) -> Result<()> {
    let routes = warp::fs::dir(dir);
    println!("Serving on http://localhost:{}", port);
    warp::serve(routes).run(([127,0,0,1], port)).await;
    Ok(())
}