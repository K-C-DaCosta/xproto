use std::os::unix::net::UnixStream;

fn main() {
    let xserver =
        UnixStream::connect(xproto::PATH_TO_UNIX_DOMAIN_SOCKET).expect("cant connect to xserver");
    xserver
        .set_read_timeout(Some(std::time::Duration::from_millis(10)))
        .unwrap();

    let mut ctx = xproto::RequestConnection::new(
        xproto::ByteOrder::LittleEndian,
        11,
        0,
        xproto::AuthProtocol::None,
    )
    .connect(xserver)
    .unwrap();

    //setup window
    let _window = ctx
        .create_window()
        .with_width(512)
        .with_height(512)
        .with_pos((128, 128))
        .with_title("My X Window =)")
        .build()
        .unwrap();

    loop {
        // listen for event messages
        ctx.socket_cb(|socket| {
            if let Ok(header) =
                xproto::xio::read_primitive::<xproto::events::EventHeaderTest, _>(socket)
            {
                println!("event header = {:?}", header);
            }
        });

        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}
