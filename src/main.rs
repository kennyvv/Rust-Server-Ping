mod net;
mod io;

fn main() {
    let server = net::minecraft_server::bind(25565);
    server.run();
}
