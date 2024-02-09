use remote_keyboard_mouse_server::server::Server;

fn main() {
    let s = Server::new(6789);
    let t = s.listen();

    t.join().unwrap();
    println!("Server stopped");
}
