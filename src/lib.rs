use std::fmt::Debug;

pub mod io;

pub type INT8 = i8;
pub type INT16 = i16;
pub type INT32 = i32;
pub type CARD8 = u8;
pub type CARD16 = u16;
pub type CARD32 = u32;
pub type TIMESTAMP = CARD32;
pub type BOOL = CARD8;

// predefined atoms
pub const SECONDARY: Atom = Atom(2);
pub const ARC: Atom = Atom(3);
pub const ATOM: Atom = Atom(4);
pub const BITMAP: Atom = Atom(5);
pub const CARDINAL: Atom = Atom(6);
pub const COLORMAP: Atom = Atom(7);
pub const CURSOR: Atom = Atom(8);
pub const CUT_BUFFER: Atom = Atom(0);
pub const CUT_BUFFER1: Atom = Atom(10);
pub const CUT_BUFFER2: Atom = Atom(11);
pub const CUT_BUFFER3: Atom = Atom(12);
pub const CUT_BUFFER4: Atom = Atom(13);
pub const CUT_BUFFER5: Atom = Atom(14);
pub const CUT_BUFFER6: Atom = Atom(15);
pub const CUT_BUFFER7: Atom = Atom(16);
pub const DRAWABLE: Atom = Atom(17);
pub const FONT: Atom = Atom(18);
pub const INTEGER: Atom = Atom(19);
pub const PIXMAP: Atom = Atom(20);
pub const POINT: Atom = Atom(21);
pub const RECTANGLE: Atom = Atom(22);
pub const RESOURCE_MANAGER: Atom = Atom(23);
pub const RGB_COLOR_MAP: Atom = Atom(24);
pub const RGB_BEST_MAP: Atom = Atom(25);
pub const RGB_BLUE_MAP: Atom = Atom(26);
pub const RGB_DEFAULT_MAP: Atom = Atom(27);
pub const RGB_GRAY_MAP: Atom = Atom(28);
pub const RGB_GREEN_MAP: Atom = Atom(29);
pub const RGB_RED_MAP: Atom = Atom(30);
pub const STRING: Atom = Atom(31);
pub const VISUALID: Atom = Atom(32);
pub const WINDOW: Atom = Atom(33);
pub const WM_COMMAND: Atom = Atom(34);
pub const WM_HINTS: Atom = Atom(35);
pub const WM_CLIENT_MACHINE: Atom = Atom(36);
pub const WM_ICON_NAME: Atom = Atom(37);
pub const WM_ICON_SIZE: Atom = Atom(38);
pub const WM_NAME: Atom = Atom(39);
pub const PRIMARY: Atom = Atom(1);
pub const WM_NORMAL_HINTS: Atom = Atom(40);
pub const WM_SIZE_HINTS: Atom = Atom(41);
pub const WM_ZOOM_HINTS: Atom = Atom(42);
pub const MIN_SPACE: Atom = Atom(43);
pub const NORM_SPACE: Atom = Atom(44);
pub const MAX_SPACE: Atom = Atom(45);
pub const END_SPACE: Atom = Atom(46);
pub const SUPERSCRIPT_X: Atom = Atom(47);
pub const SUPERSCRIPT_Y: Atom = Atom(48);
pub const SUBSCRIPT_X: Atom = Atom(49);
pub const SUBSCRIPT_Y: Atom = Atom(50);
pub const UNDERLINE_POSITION: Atom = Atom(51);
pub const UNDERLINE_THICKNESS: Atom = Atom(52);
pub const STRIKEOUT_ASCENT: Atom = Atom(53);
pub const STRIKEOUT_DESCENT: Atom = Atom(54);
pub const ITALIC_ANGLE: Atom = Atom(55);
pub const X_HEIGHT: Atom = Atom(56);
pub const QUAD_WIDTH: Atom = Atom(57);
pub const WEIGHT: Atom = Atom(58);
pub const POINT_SIZE: Atom = Atom(59);
pub const RESOLUTION: Atom = Atom(60);
pub const COPYRIGHT: Atom = Atom(61);
pub const NOTICE: Atom = Atom(62);
pub const FONT_NAME: Atom = Atom(63);
pub const FAMILY_NAME: Atom = Atom(64);
pub const FULL_NAME: Atom = Atom(65);
pub const CAP_HEIGHT: Atom = Atom(66);
pub const WM_CLASS: Atom = Atom(67);
pub const WM_TRANSIENT_FOR: Atom = Atom(68);

#[derive(Copy, Clone, Debug)]
pub enum XClass {
    StaticGray = 0,
    GrayScale = 1,
    StaticColor = 2,
    PseudoColor = 3,
    TrueColor = 4,
    DirectColor = 5,
    Unknown = 6,
}

impl Default for XClass {
    fn default() -> Self {
        Self::Unknown
    }
}
impl XClass {
    pub fn from_class_code(code: CARD8) -> Self {
        match code {
            0 => Self::StaticGray,
            1 => Self::GrayScale,
            2 => Self::StaticColor,
            3 => Self::PseudoColor,
            4 => Self::TrueColor,
            5 => Self::DirectColor,
            _ => panic!("invalid class code, io may contain a bug"),
        }
    }
}

#[derive(Copy, Clone, Default)]
pub struct XVisualType {
    pub visual_id: Atom,
    pub class: XClass,
    pub bits_per_rgb_value: CARD8,
    pub colormap_entries: CARD16,
    pub red_mask: CARD32,
    pub green_mask: CARD32,
    pub blue_mask: CARD32,
}

impl XVisualType {
    pub fn from_intermediate(inter: XVisualTypeIntermediate) -> Self {
        let visual_id = inter.visual_id;
        let class = XClass::from_class_code(inter.class);
        let bits_per_rgb_value = inter.bits_per_rgb_value;
        let colormap_entries = inter.colormap_entries;
        let red_mask = inter.red_mask;
        let green_mask = inter.green_mask;
        let blue_mask = inter.blue_mask;
        Self {
            visual_id,
            class,
            bits_per_rgb_value,
            colormap_entries,
            red_mask,
            green_mask,
            blue_mask,
        }
    }
}
impl Debug for XVisualType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let visual_id = self.visual_id;
        let class = self.class;
        let bits_per_rgb_value = self.bits_per_rgb_value;
        let colormap_entries = self.colormap_entries;
        let red_mask = self.red_mask;
        let green_mask = self.green_mask;
        let blue_mask = self.blue_mask;
        writeln!(f, "\t\tvisual_id = {:?}", visual_id)?;
        writeln!(f, "\t\tclass = {:?}", class)?;
        writeln!(f, "\t\tbits_per_rgb_value = {:?}", bits_per_rgb_value)?;
        writeln!(f, "\t\tcolormap_entries = {:?}", colormap_entries)?;
        writeln!(f, "\t\tred_mask = {}", red_mask)?;
        writeln!(f, "\t\tgreen_mask = {:?}", green_mask)?;
        writeln!(f, "\t\tblue_mask = {:?}", blue_mask)?;
        Ok(())
    }
}

#[repr(C, packed(1))]
#[derive(Copy, Clone, Default)]
pub struct XVisualTypeIntermediate {
    visual_id: Atom,
    class: CARD8,
    bits_per_rgb_value: CARD8,
    colormap_entries: CARD16,
    red_mask: CARD32,
    green_mask: CARD32,
    blue_mask: CARD32,
    unused: CARD32,
}

#[derive(Clone, Default)]
pub struct XDepth {
    depth: CARD8,
    unused_0: CARD8,
    number_of_visual_types: CARD16,
    unused_1: CARD32,
    visuals: Vec<XVisualType>,
}
impl XDepth {
    pub fn from_socket<T: std::io::Read>(mut socket: T) -> std::io::Result<Self> {
        let mut res = Self::default();
        res.depth = io::read_primitive(&mut socket)?;
        res.unused_0 = io::read_primitive(&mut socket)?;
        res.number_of_visual_types = io::read_primitive(&mut socket)?;
        res.unused_1 = io::read_primitive(&mut socket)?;
        res.visuals = io::read_primitive_list::<XVisualTypeIntermediate, _>(
            &mut socket,
            res.number_of_visual_types as usize,
        )?
        .into_iter()
        .map(|a| XVisualType::from_intermediate(a))
        .collect();
        Ok(res)
    }
}
impl Debug for XDepth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\tdepth = {:?}", self.depth)?;
        writeln!(f, "\tunused_0 = {:?}", self.unused_0)?;
        writeln!(
            f,
            "\tnumber_of_visual_types = {:?}",
            self.number_of_visual_types
        )?;
        writeln!(f, "\tunused_1 = {:?}", self.unused_1)?;

        write!(f, "\tvisuals:\n\t[\n")?;
        for visual in &self.visuals {
            writeln!(f, "{:?}", visual)?;
        }
        write!(f, "\t]")
    }
}

#[derive(Clone, Default)]
pub struct XScreen {
    root: Atom,
    default_colormap: CARD32,
    white_pixel: CARD32,
    black_pixel: CARD32,
    current_input_masks: CARD32,
    width_pixels: CARD16,
    height_pixels: CARD16,
    width_in_millimeters: CARD16,
    height_in_millimeters: CARD16,
    min_installed_maps: CARD16,
    max_installed_maps: CARD16,
    root_visual: Atom,
    backing_stores: CARD8,
    save_unders: BOOL,
    root_depth: CARD8,
    number_of_depths_in_allowed_depths: CARD8,
    depth_list: Vec<XDepth>,
}
impl Debug for XScreen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "root = {:?}", self.root)?;
        writeln!(f, "default_colormap = {:?}", self.default_colormap)?;
        writeln!(f, "white_pixel = {:?}", self.white_pixel)?;
        writeln!(f, "black_pixel = {:?}", self.black_pixel)?;
        writeln!(f, "current_input_masks = {:?}", self.current_input_masks)?;
        writeln!(f, "width_pixels = {:?}", self.width_pixels)?;
        writeln!(f, "height_pixels = {:?}", self.height_pixels)?;
        writeln!(f, "width_in_millimeters = {:?}", self.width_in_millimeters)?;
        writeln!(
            f,
            "height_in_millimeters = {:?}",
            self.height_in_millimeters
        )?;
        writeln!(f, "min_installed_maps = {:?}", self.min_installed_maps)?;
        writeln!(f, "max_installed_maps = {:?}", self.max_installed_maps)?;
        writeln!(f, "root_visual = {:?}", self.root_visual)?;
        writeln!(f, "backing_stores = {:?}", self.backing_stores)?;
        writeln!(f, "save_unders = {:?}", self.save_unders)?;
        writeln!(f, "root_depth = {:?}", self.root_depth)?;
        writeln!(
            f,
            "number_of_depths_in_allowed_depths = {:?}",
            self.number_of_depths_in_allowed_depths
        )?;

        for depth in &self.depth_list {
            writeln!(f, "depth:\n{:?}", depth)?;
        }

        Ok(())
    }
}

impl XScreen {
    pub fn from_socket<T: std::io::Read>(
        mut socket: T,
        number_of_formats: usize,
    ) -> std::io::Result<Self> {
        let mut res = Self::default();
        res.root = io::read_primitive(&mut socket)?;
        res.default_colormap = io::read_primitive(&mut socket)?;
        res.white_pixel = io::read_primitive(&mut socket)?;
        res.black_pixel = io::read_primitive(&mut socket)?;
        res.current_input_masks = io::read_primitive(&mut socket)?;
        res.width_pixels = io::read_primitive(&mut socket)?;
        res.height_pixels = io::read_primitive(&mut socket)?;
        res.width_in_millimeters = io::read_primitive(&mut socket)?;
        res.height_in_millimeters = io::read_primitive(&mut socket)?;
        res.min_installed_maps = io::read_primitive(&mut socket)?;
        res.max_installed_maps = io::read_primitive(&mut socket)?;
        res.root_visual = io::read_primitive(&mut socket)?;
        res.backing_stores = io::read_primitive(&mut socket)?;
        res.save_unders = io::read_primitive(&mut socket)?;
        res.root_depth = io::read_primitive(&mut socket)?;
        res.number_of_depths_in_allowed_depths = io::read_primitive(&mut socket)?;
        res.depth_list = (0..number_of_formats)
            .map(|_| XDepth::from_socket(&mut socket))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(res)
    }
}

#[repr(C, packed(1))]
#[derive(Copy, Clone, Default, Debug)]
pub struct XFormat {
    pub depth: CARD8,
    pub bits_per_pixel: CARD8,
    pub scanline_pad: CARD8,
    pub padding: [u8; 5],
}

#[repr(C, packed(1))]
#[derive(Copy, Clone, Default, Debug)]
pub struct Atom(pub CARD32);

const ATOM_MASK: u32 = (1 << 29) - 1;

impl<T: Into<u32>> From<T> for Atom {
    fn from(val: T) -> Self {
        Self(val.into() & ATOM_MASK)
    }
}
