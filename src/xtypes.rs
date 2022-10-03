use super::*;

pub type INT8 = i8;
pub type INT16 = i16;
pub type INT32 = i32;
pub type CARD8 = u8;
pub type CARD16 = u16;
pub type CARD32 = u32;
pub type BOOL = CARD8;
pub type WINDOW = Atom;

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Timestamp(CARD32);

impl Timestamp {
    pub fn current_time() -> Self {
        Timestamp(0)
    }
    pub fn data(&self) -> CARD32 {
        self.0
    }
}
impl Into<CARD32> for Timestamp {
    fn into(self) -> CARD32 {
        self.data()
    }
}

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
    pub depth: CARD8,
    pub unused_0: CARD8,
    pub number_of_visual_types: CARD16,
    pub unused_1: CARD32,
    pub visuals: Vec<XVisualType>,
}
impl XDepth {
    pub fn from_socket<T: std::io::Read>(mut socket: T) -> std::io::Result<Self> {
        let mut res = Self::default();
        res.depth = xio::read_primitive(&mut socket)?;
        res.unused_0 = xio::read_primitive(&mut socket)?;
        res.number_of_visual_types = xio::read_primitive(&mut socket)?;
        res.unused_1 = xio::read_primitive(&mut socket)?;
        res.visuals = xio::read_primitive_list::<XVisualTypeIntermediate, _>(
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
    pub root: Atom,
    pub default_colormap: CARD32,
    pub white_pixel: CARD32,
    pub black_pixel: CARD32,
    pub current_input_masks: CARD32,
    pub width_pixels: CARD16,
    pub height_pixels: CARD16,
    pub width_in_millimeters: CARD16,
    pub height_in_millimeters: CARD16,
    pub min_installed_maps: CARD16,
    pub max_installed_maps: CARD16,
    pub root_visual: Atom,
    pub backing_stores: CARD8,
    pub save_unders: BOOL,
    pub root_depth: CARD8,
    pub number_of_depths_in_allowed_depths: CARD8,
    pub depth_list: Vec<XDepth>,
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
        res.root = xio::read_primitive(&mut socket)?;
        res.default_colormap = xio::read_primitive(&mut socket)?;
        res.white_pixel = xio::read_primitive(&mut socket)?;
        res.black_pixel = xio::read_primitive(&mut socket)?;
        res.current_input_masks = xio::read_primitive(&mut socket)?;
        res.width_pixels = xio::read_primitive(&mut socket)?;
        res.height_pixels = xio::read_primitive(&mut socket)?;
        res.width_in_millimeters = xio::read_primitive(&mut socket)?;
        res.height_in_millimeters = xio::read_primitive(&mut socket)?;
        res.min_installed_maps = xio::read_primitive(&mut socket)?;
        res.max_installed_maps = xio::read_primitive(&mut socket)?;
        res.root_visual = xio::read_primitive(&mut socket)?;
        res.backing_stores = xio::read_primitive(&mut socket)?;
        res.save_unders = xio::read_primitive(&mut socket)?;
        res.root_depth = xio::read_primitive(&mut socket)?;
        res.number_of_depths_in_allowed_depths = xio::read_primitive(&mut socket)?;
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
