use client::Voxel2Client;
use pollster::FutureExt as _;

mod client;
mod server;

fn main() {
    let client = Voxel2Client::new().block_on();
    client.run();
}
