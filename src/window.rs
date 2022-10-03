use super::*;

#[repr(C, packed(1))]
#[derive(Copy, Clone, Default)]
pub struct WindowValue {
    background_pixmap: Atom,
    background_pixel: CARD32,
    border_pixmap: Atom,
    border_pixel: CARD32,
    bit_gravity: CARD8,
    win_gravity: CARD8,
    backing_store: CARD8,
    backing_planes: CARD32,
    backing_pixel: CARD32,
    override_redirect: CARD8,
    save_under: CARD8,
    event_mask: CARD32,
    do_not_propagate_mask: CARD32,
    colormap: CARD32,
    cursor: CARD32,
}

#[derive(Clone)]
pub struct XWindow<S> {
    ctx: XContext<S>,
    id: Atom,
    parent_id: Atom,
    x: i16,
    y: i16,
    w: u16,
    h: u16,
}
impl<S> XWindow<S> {
    pub fn parent(&self) -> Atom {
        self.parent_id
    }
    pub fn id(&self) -> Atom {
        self.id
    }
}
pub struct WindowBuilder<'a, T> {
    ctx: &'a mut XContext<T>,
    opcode: CARD8,
    depth: CARD8,
    request_length: CARD16,
    wid: Atom,
    parent: Atom,
    x: INT16,
    y: INT16,
    width: CARD16,
    height: CARD16,
    border_width: CARD16,
    class: CARD16,
    visual: Atom,
    value_mask: CARD32,
    value_list: Vec<WindowValue>,
    window_id: Atom,
    title: Option<&'a str>,
}
impl<'a, T> WindowBuilder<'a, T>
where
    T: io::Write + io::Read,
{
    pub fn new(state: &'a mut XContext<T>) -> Self {
        let mut res_unitit = MaybeUninit::<Self>::zeroed();
        let window_id = Atom(state.gen_id());
        unsafe {
            let res = res_unitit.assume_init_mut();
            res.title = None;
            res.window_id = window_id;
            res.opcode = opcodes::CREATE_WINDOW;
            res.request_length = 8;
            res.wid = window_id;
            res.parent = state.info.list_of_screen[0].root;
            res.depth = 0;
            res.border_width = 0;
            res.class = 0;
            res.visual = Atom(0); //copy from parent
            res.value_mask = 0;
            res.value_list = vec![];
            res.ctx = state;

            res_unitit.assume_init_read()
        }
    }

    pub fn with_parent(mut self, parent: Atom) -> Self {
        self.parent = parent;
        self
    }

    pub fn with_pos(mut self, pos: (i16, i16)) -> Self {
        self.x = pos.0;
        self.y = pos.1;
        self
    }

    pub fn with_height(mut self, val: CARD16) -> Self {
        self.height = val;
        self
    }

    pub fn with_width(mut self, val: CARD16) -> Self {
        self.width = val;
        self
    }

    pub fn with_visual(mut self, val: CARD32) -> Self {
        self.visual.0 = val;
        self
    }

    pub fn with_value(mut self, value: WindowValue) -> Self {
        self.request_length += 1;
        self.value_list.push(value);
        self
    }

    pub fn with_title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }

    pub fn build(mut self) -> Result<XWindow<T>, io::Error> {
        let ctx = self.ctx;
        let &mut opcode = &mut self.opcode;
        let &mut depth = &mut self.depth;
        let &mut request_length = &mut self.request_length;
        let &mut wid = &mut self.wid;
        let &mut parent = &mut self.parent;
        let &mut x = &mut self.x;
        let &mut y = &mut self.y;
        let &mut width = &mut self.width;
        let &mut height = &mut self.height;
        let &mut border_width = &mut self.border_width;
        let &mut class = &mut self.class;
        let &mut visual = &mut self.visual;
        let &mut value_mask = &mut self.value_mask;
        let socket = &mut *ctx.socket.as_ref().borrow_mut();

        xio::write_primitive(socket, opcode)?;
        xio::write_primitive(socket, depth)?;
        xio::write_primitive(socket, request_length)?;
        xio::write_primitive(socket, wid)?;
        xio::write_primitive(socket, parent)?;
        xio::write_primitive(socket, x)?;
        xio::write_primitive(socket, y)?;
        xio::write_primitive(socket, width)?;
        xio::write_primitive(socket, height)?;
        xio::write_primitive(socket, border_width)?;
        xio::write_primitive(socket, class)?;
        xio::write_primitive(socket, visual)?;
        xio::write_primitive(socket, value_mask)?;

        if let Err(e) = check_for_error(socket) {
            println!("err = {:?}", e);
        } else {
            println!("no errors");
        }
        map_window(socket, self.window_id.0)?;

        allow_events(socket, EventMode::AsyncBoth, Timestamp::current_time())?;
        if let Err(e) = check_for_error(socket) {
            println!("err = {:?}", e);
        } else {
            println!("no errors");
        }

        // let grab_err = grab_keyboard(
        //     socket,
        //     true,
        //     Atom(0),
        //     Timestamp::current_time(),
        //     SynchKind::Asynchronous,
        //     SynchKind::Asynchronous,
        // );
        // println!("grab err ={:?} ",grab_err);
        // if let Err(e) = check_for_error(socket) {
        //     println!("err = {:?}", e);
        // } else {
        //     println!("no errors");
        // }

        if let Some(title) = self.title {
            property::change_property(
                socket,
                PropertyMode::Replace,
                xconsts::predefined_atoms::WM_NAME,
                xconsts::predefined_atoms::STRING,
                self.window_id.0,
                PropertyFormat::Bytes,
                title,
            )?;
            if let Err(e) = check_for_error(socket) {
                println!("err = {:?}", e);
            } else {
                println!("no errors");
            }
        }

        grab_button(
            socket,
            false,
            self.window_id,
            0,
            SynchKind::Asynchronous,
            SynchKind::Asynchronous,
            Atom(0),
            Atom(0),
            0,
            None,
        )?;
        if let Err(e) = check_for_error(socket) {
            println!("err = {:?}", e);
        } else {
            println!("no errors");
        }

        grab_key(
            socket,
            false,
            self.window_id,
            None,
            0,
            SynchKind::Asynchronous,
            SynchKind::Asynchronous,
        )?;
        if let Err(e) = check_for_error(socket) {
            println!("err = {:?}", e);
        } else {
            println!("no errors");
        }

        Ok(XWindow {
            ctx: ctx.clone(),
            id: self.window_id,
            parent_id: self.parent,
            x: self.x,
            y: self.y,
            w: self.width,
            h: self.height,
        })
    }
}
