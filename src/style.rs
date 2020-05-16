use crate::geometry::{Rect, Size};
use crate::number::Number;

#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "serde", feature = "serde_kebab_case"), serde(rename_all = "kebab-case"))]
#[cfg_attr(all(feature = "serde", feature = "serde_camel_case"), serde(rename_all = "camelCase"))]
pub enum AlignItems {
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
}

impl Default for AlignItems {
    fn default() -> Self {
        Self::Stretch
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "serde", feature = "serde_kebab_case"), serde(rename_all = "kebab-case"))]
#[cfg_attr(all(feature = "serde", feature = "serde_camel_case"), serde(rename_all = "camelCase"))]
pub enum AlignSelf {
    Auto,
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
}

impl Default for AlignSelf {
    fn default() -> Self {
        Self::Auto
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "serde", feature = "serde_kebab_case"), serde(rename_all = "kebab-case"))]
#[cfg_attr(all(feature = "serde", feature = "serde_camel_case"), serde(rename_all = "camelCase"))]
pub enum AlignContent {
    FlexStart,
    FlexEnd,
    Center,
    Stretch,
    SpaceBetween,
    SpaceAround,
}

impl Default for AlignContent {
    fn default() -> Self {
        Self::Stretch
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "serde", feature = "serde_kebab_case"), serde(rename_all = "kebab-case"))]
#[cfg_attr(all(feature = "serde", feature = "serde_camel_case"), serde(rename_all = "camelCase"))]
pub enum Direction {
    Inherit,
    #[cfg_attr(feature = "serde", serde(rename = "ltr"))]
    LTR,
    #[cfg_attr(feature = "serde", serde(rename = "rtl"))]
    RTL,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Inherit
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "serde", feature = "serde_kebab_case"), serde(rename_all = "kebab-case"))]
#[cfg_attr(all(feature = "serde", feature = "serde_camel_case"), serde(rename_all = "camelCase"))]
pub enum Display {
    #[cfg_attr(feature = "serde", serde(rename = "flex"))]
    Flex,
    #[cfg_attr(feature = "serde", serde(rename = "none"))]
    None,
}

impl Default for Display {
    fn default() -> Self {
        Self::Flex
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "serde", feature = "serde_kebab_case"), serde(rename_all = "kebab-case"))]
#[cfg_attr(all(feature = "serde", feature = "serde_camel_case"), serde(rename_all = "camelCase"))]
pub enum FlexDirection {
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}

impl Default for FlexDirection {
    fn default() -> Self {
        Self::Row
    }
}

impl FlexDirection {
    #[inline]
    pub(crate) fn is_row(self) -> bool {
        match self {
            Self::Row | Self::RowReverse => true,
            _ => false,
        }
    }

    #[inline]
    pub(crate) fn is_column(self) -> bool {
        match self {
            Self::Column | Self::ColumnReverse => true,
            _ => false,
        }
    }

    #[inline]
    pub(crate) fn is_reverse(self) -> bool {
        match self {
            Self::RowReverse | Self::ColumnReverse => true,
            _ => false,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "serde", feature = "serde_kebab_case"), serde(rename_all = "kebab-case"))]
#[cfg_attr(all(feature = "serde", feature = "serde_camel_case"), serde(rename_all = "camelCase"))]
pub enum JustifyContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl Default for JustifyContent {
    fn default() -> Self {
        Self::FlexStart
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "serde", feature = "serde_kebab_case"), serde(rename_all = "kebab-case"))]
#[cfg_attr(all(feature = "serde", feature = "serde_camel_case"), serde(rename_all = "camelCase"))]
pub enum Overflow {
    Visible,
    Hidden,
    Scroll,
}

impl Default for Overflow {
    fn default() -> Self {
        Self::Visible
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "serde", feature = "serde_kebab_case"), serde(rename_all = "kebab-case"))]
#[cfg_attr(all(feature = "serde", feature = "serde_camel_case"), serde(rename_all = "camelCase"))]
pub enum PositionType {
    Relative,
    Absolute,
}

impl Default for PositionType {
    fn default() -> Self {
        Self::Relative
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "serde", feature = "serde_kebab_case"), serde(rename_all = "kebab-case"))]
#[cfg_attr(all(feature = "serde", feature = "serde_camel_case"), serde(rename_all = "camelCase"))]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

impl Default for FlexWrap {
    fn default() -> Self {
        Self::NoWrap
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "serde", feature = "serde_kebab_case"), serde(rename_all = "kebab-case"))]
#[cfg_attr(all(feature = "serde", feature = "serde_camel_case"), serde(rename_all = "camelCase"))]
pub enum Dimension {
    Undefined,
    Auto,
    Points(f32),
    Percent(f32),
}

impl Default for Dimension {
    fn default() -> Self {
        Self::Undefined
    }
}

impl Dimension {
    pub(crate) fn resolve(self, parent_dim: Number) -> Number {
        match self {
            Dimension::Points(points) => Number::Defined(points),
            Dimension::Percent(percent) => parent_dim * percent,
            _ => Number::Undefined,
        }
    }

    pub(crate) fn is_defined(self) -> bool {
        match self {
            Dimension::Points(_) => true,
            Dimension::Percent(_) => true,
            _ => false,
        }
    }
}

impl Default for Rect<Dimension> {
    fn default() -> Self {
        Self { start: Default::default(), end: Default::default(), top: Default::default(), bottom: Default::default() }
    }
}

impl Default for Size<Dimension> {
    fn default() -> Self {
        Self { width: Dimension::Auto, height: Dimension::Auto }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[cfg_attr(all(feature = "serde", feature = "serde_kebab_case"), serde(rename_all = "kebab-case"))]
#[cfg_attr(all(feature = "serde", feature = "serde_camel_case"), serde(rename_all = "camelCase"))]
pub struct Style {
    pub display: Display,
    pub position_type: PositionType,
    pub direction: Direction,
    pub flex_direction: FlexDirection,
    pub flex_wrap: FlexWrap,
    pub overflow: Overflow,
    pub align_items: AlignItems,
    pub align_self: AlignSelf,
    pub align_content: AlignContent,
    pub justify_content: JustifyContent,
    pub position: Rect<Dimension>,
    pub margin: Rect<Dimension>,
    pub padding: Rect<Dimension>,
    pub border: Rect<Dimension>,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: Dimension,
    pub size: Size<Dimension>,
    pub min_size: Size<Dimension>,
    pub max_size: Size<Dimension>,
    pub aspect_ratio: Number,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            display: Default::default(),
            position_type: Default::default(),
            direction: Default::default(),
            flex_direction: Default::default(),
            flex_wrap: Default::default(),
            overflow: Default::default(),
            align_items: Default::default(),
            align_self: Default::default(),
            align_content: Default::default(),
            justify_content: Default::default(),
            position: Default::default(),
            margin: Default::default(),
            padding: Default::default(),
            border: Default::default(),
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: Dimension::Auto,
            size: Default::default(),
            min_size: Default::default(),
            max_size: Default::default(),
            aspect_ratio: Default::default(),
        }
    }
}

impl Style {
    pub(crate) fn min_main_size(&self, direction: FlexDirection) -> Dimension {
        if direction.is_row() {
            self.min_size.width
        } else {
            self.min_size.height
        }
    }

    pub(crate) fn max_main_size(&self, direction: FlexDirection) -> Dimension {
        if direction.is_row() {
            self.max_size.width
        } else {
            self.max_size.height
        }
    }

    pub(crate) fn main_margin_start(&self, direction: FlexDirection) -> Dimension {
        if direction.is_row() {
            self.margin.start
        } else {
            self.margin.top
        }
    }

    pub(crate) fn main_margin_end(&self, direction: FlexDirection) -> Dimension {
        if direction.is_row() {
            self.margin.end
        } else {
            self.margin.bottom
        }
    }

    pub(crate) fn cross_size(&self, direction: FlexDirection) -> Dimension {
        if direction.is_row() {
            self.size.height
        } else {
            self.size.width
        }
    }

    pub(crate) fn min_cross_size(&self, direction: FlexDirection) -> Dimension {
        if direction.is_row() {
            self.min_size.height
        } else {
            self.min_size.width
        }
    }

    pub(crate) fn max_cross_size(&self, direction: FlexDirection) -> Dimension {
        if direction.is_row() {
            self.max_size.height
        } else {
            self.max_size.width
        }
    }

    pub(crate) fn cross_margin_start(&self, direction: FlexDirection) -> Dimension {
        if direction.is_row() {
            self.margin.top
        } else {
            self.margin.start
        }
    }

    pub(crate) fn cross_margin_end(&self, direction: FlexDirection) -> Dimension {
        if direction.is_row() {
            self.margin.bottom
        } else {
            self.margin.end
        }
    }

    pub(crate) fn align_self(&self, parent: &Style) -> AlignSelf {
        if self.align_self == AlignSelf::Auto {
            match parent.align_items {
                AlignItems::FlexStart => AlignSelf::FlexStart,
                AlignItems::FlexEnd => AlignSelf::FlexEnd,
                AlignItems::Center => AlignSelf::Center,
                AlignItems::Baseline => AlignSelf::Baseline,
                AlignItems::Stretch => AlignSelf::Stretch,
            }
        } else {
            self.align_self
        }
    }
}
