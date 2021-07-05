use xmodmap_pke::{print_xmodmap_pke, xmodmap_pke};

fn main() {
    let (conn, _) = xcb::Connection::connect(None).unwrap();
    print_xmodmap_pke(&xmodmap_pke(&conn).unwrap());
}
