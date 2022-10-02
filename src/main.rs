use std::os::unix::net::UnixStream;

use xproto::*;

fn main() {
    let xserver = UnixStream::connect(PATH_TO_UNIX_DOMAIN_SOCKET).expect("cant connect to xserver");
    xserver
        .set_read_timeout(Some(std::time::Duration::from_millis(30)))
        .unwrap();

    let mut ctx = RequestConnection::new(ByteOrder::LittleEndian, 11, 0, AuthProtocol::None)
        .connect(xserver)
        .unwrap();

    ctx.create_window()
        .with_width(512)
        .with_height(512)
        .with_pos((128, 128))
        .with_title("my xlib client sucks")
        .build()
        .unwrap();

    loop {
        //busy wait
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
