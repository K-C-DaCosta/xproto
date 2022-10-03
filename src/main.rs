use std::{
    os::unix::net::UnixStream,
    time::{Duration, Instant},
};

use xproto::*;

fn main() {
    let xserver = UnixStream::connect(PATH_TO_UNIX_DOMAIN_SOCKET).expect("cant connect to xserver");
    xserver
        .set_read_timeout(Some(std::time::Duration::from_millis(10)))
        .unwrap();



    let mut ctx = RequestConnection::new(ByteOrder::LittleEndian, 11, 0, AuthProtocol::None)
        .connect(xserver)
        .unwrap();

    let window = ctx
        .create_window()
        .with_width(512)
        .with_height(512)
        .with_pos((128, 128))
        .with_title("My X Window =)")
        .build()
        .unwrap();

    let mut t0 = Instant::now();
    let mut grabbed = false;
    loop {
        // let elapsed_time = t0.elapsed().as_millis();
        // if elapsed_time < 60 {
        //     if grabbed == false {
        //         grabbed = true;
        //         ctx.socket_cb(|socket| {
        //             let res = grab_keyboard(
        //                 socket,
        //                 false,
        //                 window.id(),
        //                 Timestamp::current_time(),
        //                 SynchKind::Asynchronous,
        //                 SynchKind::Asynchronous,
        //             );
        //             if let Err(e) = res {
        //                 println!("grab resp = {:?}", e);
        //             }
        //             if let Err(e) = check_for_error(socket) {
        //                 println!("xorg err ={:?}", e);
        //             }
        //         });
        //     }
        // } else {
        //     t0 = Instant::now();
        //     grabbed = false;
        //     // ctx.socket_cb(|socket| {
        //     //     let _ = ungrab_keyboard(socket, Timestamp::current_time());
        //     //     let _ = xio::flush_read(socket);
        //     // });
        // }

        ctx.socket_cb(|socket| {
            if let Ok(header) = xio::read_primitive::<xproto::events::EventHeaderTest, _>(socket) {
                println!("event header = {:?}", header);
            }
        });

        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}
