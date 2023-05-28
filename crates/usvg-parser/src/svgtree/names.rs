// This file is autogenerated. Do not edit it!
// See ./codegen for details.

/// An element ID.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq)]
pub enum EId {
    A,
    Circle,
    ClipPath,
    Defs,
    Ellipse,
    FeBlend,
    FeColorMatrix,
    FeComponentTransfer,
    FeComposite,
    FeConvolveMatrix,
    FeDiffuseLighting,
    FeDisplacementMap,
    FeDistantLight,
    FeDropShadow,
    FeFlood,
    FeFuncA,
    FeFuncB,
    FeFuncG,
    FeFuncR,
    FeGaussianBlur,
    FeImage,
    FeMerge,
    FeMergeNode,
    FeMorphology,
    FeOffset,
    FePointLight,
    FeSpecularLighting,
    FeSpotLight,
    FeTile,
    FeTurbulence,
    Filter,
    G,
    Image,
    Line,
    LinearGradient,
    Marker,
    Mask,
    Path,
    Pattern,
    Polygon,
    Polyline,
    RadialGradient,
    Rect,
    Stop,
    Style,
    Svg,
    Switch,
    Symbol,
    Text,
    TextPath,
    Title,
    Tref,
    Tspan,
    Use
}

static ELEMENTS: Map<EId> = Map {
    key: 3347381344252206323,
    disps: &[
        (0, 13),
        (4, 4),
        (0, 16),
        (0, 6),
        (12, 29),
        (0, 2),
        (0, 15),
        (8, 48),
        (0, 0),
        (0, 33),
        (18, 49),
    ],
    entries: &[
        ("feMerge", EId::FeMerge),
        ("feDiffuseLighting", EId::FeDiffuseLighting),
        ("use", EId::Use),
        ("mask", EId::Mask),
        ("pattern", EId::Pattern),
        ("style", EId::Style),
        ("feConvolveMatrix", EId::FeConvolveMatrix),
        ("tspan", EId::Tspan),
        ("textPath", EId::TextPath),
        ("feDisplacementMap", EId::FeDisplacementMap),
        ("fePointLight", EId::FePointLight),
        ("line", EId::Line),
        ("feDistantLight", EId::FeDistantLight),
        ("feDropShadow", EId::FeDropShadow),
        ("defs", EId::Defs),
        ("switch", EId::Switch),
        ("feBlend", EId::FeBlend),
        ("feFuncB", EId::FeFuncB),
        ("path", EId::Path),
        ("feTurbulence", EId::FeTurbulence),
        ("circle", EId::Circle),
        ("polygon", EId::Polygon),
        ("feComponentTransfer", EId::FeComponentTransfer),
        ("g", EId::G),
        ("polyline", EId::Polyline),
        ("symbol", EId::Symbol),
        ("filter", EId::Filter),
        ("feColorMatrix", EId::FeColorMatrix),
        ("feComposite", EId::FeComposite),
        ("clipPath", EId::ClipPath),
        ("image", EId::Image),
        ("radialGradient", EId::RadialGradient),
        ("feFlood", EId::FeFlood),
        ("feTile", EId::FeTile),
        ("tref", EId::Tref),
        ("marker", EId::Marker),
        ("feFuncG", EId::FeFuncG),
        ("feSpotLight", EId::FeSpotLight),
        ("stop", EId::Stop),
        ("a", EId::A),
        ("text", EId::Text),
        ("svg", EId::Svg),
        ("title", EId::Title),
        ("feFuncA", EId::FeFuncA),
        ("feImage", EId::FeImage),
        ("feOffset", EId::FeOffset),
        ("feGaussianBlur", EId::FeGaussianBlur),
        ("rect", EId::Rect),
        ("feMergeNode", EId::FeMergeNode),
        ("linearGradient", EId::LinearGradient),
        ("feFuncR", EId::FeFuncR),
        ("ellipse", EId::Ellipse),
        ("feSpecularLighting", EId::FeSpecularLighting),
        ("feMorphology", EId::FeMorphology),
    ],
};

impl EId {
    pub(crate) fn from_str(text: &str) -> Option<EId> {
        ELEMENTS.get(text).cloned()
    }

    /// Returns the original string.
    #[inline(never)]
    pub fn to_str(self) -> &'static str {
        ELEMENTS.key(&self)
    }
}

impl std::fmt::Debug for EId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl std::fmt::Display for EId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// An attribute ID.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq)]
pub enum AId {
    AlignmentBaseline,
    Amplitude,
    Azimuth,
    BaseFrequency,
    BaselineShift,
    Bias,
    Class,
    Clip,
    ClipPath,
    ClipRule,
    ClipPathUnits,
    Color,
    ColorInterpolation,
    ColorInterpolationFilters,
    ColorProfile,
    ColorRendering,
    Cx,
    Cy,
    D,
    DiffuseConstant,
    Direction,
    Display,
    Divisor,
    DominantBaseline,
    Dx,
    Dy,
    EdgeMode,
    Elevation,
    EnableBackground,
    Exponent,
    Fill,
    FillOpacity,
    FillRule,
    Filter,
    FilterUnits,
    FloodColor,
    FloodOpacity,
    Font,
    FontFamily,
    FontFeatureSettings,
    FontKerning,
    FontSize,
    FontSizeAdjust,
    FontStretch,
    FontStyle,
    FontSynthesis,
    FontVariant,
    FontVariantCaps,
    FontVariantEastAsian,
    FontVariantLigatures,
    FontVariantNumeric,
    FontVariantPosition,
    FontWeight,
    Fr,
    Fx,
    Fy,
    GlyphOrientationHorizontal,
    GlyphOrientationVertical,
    GradientTransform,
    GradientUnits,
    Height,
    Href,
    Id,
    ImageRendering,
    In,
    In2,
    InlineSize,
    Intercept,
    Isolation,
    K1,
    K2,
    K3,
    K4,
    KernelMatrix,
    KernelUnitLength,
    Kerning,
    Label,
    LengthAdjust,
    LetterSpacing,
    LightingColor,
    LimitingConeAngle,
    LineHeight,
    MarkerEnd,
    MarkerMid,
    MarkerStart,
    MarkerHeight,
    MarkerUnits,
    MarkerWidth,
    Mask,
    MaskBorder,
    MaskBorderMode,
    MaskBorderOutset,
    MaskBorderRepeat,
    MaskBorderSlice,
    MaskBorderSource,
    MaskBorderWidth,
    MaskClip,
    MaskComposite,
    MaskImage,
    MaskMode,
    MaskOrigin,
    MaskPosition,
    MaskSize,
    MaskType,
    MaskContentUnits,
    MaskUnits,
    MixBlendMode,
    Mode,
    NumOctaves,
    Offset,
    Opacity,
    Operator,
    Order,
    Orient,
    Overflow,
    PaintOrder,
    Path,
    PathLength,
    PatternContentUnits,
    PatternTransform,
    PatternUnits,
    Points,
    PointsAtX,
    PointsAtY,
    PointsAtZ,
    PreserveAlpha,
    PreserveAspectRatio,
    PrimitiveUnits,
    R,
    Radius,
    RefX,
    RefY,
    RequiredExtensions,
    RequiredFeatures,
    Result,
    Rotate,
    Rx,
    Ry,
    Scale,
    Seed,
    ShapeImageThreshold,
    ShapeInside,
    ShapeMargin,
    ShapePadding,
    ShapeRendering,
    ShapeSubtract,
    Side,
    Slope,
    Space,
    SpecularConstant,
    SpecularExponent,
    SpreadMethod,
    StartOffset,
    StdDeviation,
    StitchTiles,
    StopColor,
    StopOpacity,
    Stroke,
    StrokeDasharray,
    StrokeDashoffset,
    StrokeLinecap,
    StrokeLinejoin,
    StrokeMiterlimit,
    StrokeOpacity,
    StrokeWidth,
    Style,
    SurfaceScale,
    SystemLanguage,
    TableValues,
    TargetX,
    TargetY,
    TextAlign,
    TextAlignLast,
    TextAnchor,
    TextDecoration,
    TextDecorationColor,
    TextDecorationFill,
    TextDecorationLine,
    TextDecorationStroke,
    TextDecorationStyle,
    TextIndent,
    TextOrientation,
    TextOverflow,
    TextRendering,
    TextUnderlinePosition,
    TextLength,
    Transform,
    TransformBox,
    TransformOrigin,
    Type,
    UnicodeBidi,
    UnicodeRange,
    Values,
    VectorEffect,
    ViewBox,
    Visibility,
    WhiteSpace,
    Width,
    WordSpacing,
    WritingMode,
    X,
    X1,
    X2,
    XChannelSelector,
    Y,
    Y1,
    Y2,
    YChannelSelector,
    Z
}

static ATTRIBUTES: Map<AId> = Map {
    key: 3347381344252206323,
    disps: &[
        (0, 166),
        (0, 2),
        (0, 36),
        (0, 48),
        (0, 27),
        (0, 100),
        (0, 5),
        (0, 197),
        (0, 22),
        (8, 79),
        (0, 52),
        (1, 180),
        (0, 21),
        (10, 120),
        (0, 40),
        (1, 30),
        (0, 24),
        (0, 1),
        (0, 138),
        (0, 10),
        (0, 0),
        (0, 4),
        (0, 35),
        (1, 160),
        (11, 71),
        (0, 13),
        (0, 54),
        (2, 39),
        (57, 121),
        (0, 53),
        (0, 24),
        (44, 157),
        (0, 14),
        (0, 76),
        (0, 15),
        (0, 163),
        (0, 9),
        (15, 185),
        (0, 17),
        (2, 115),
        (32, 70),
        (1, 1),
    ],
    entries: &[
        ("mask-border-source", AId::MaskBorderSource),
        ("stop-opacity", AId::StopOpacity),
        ("stroke-linejoin", AId::StrokeLinejoin),
        ("dominant-baseline", AId::DominantBaseline),
        ("spreadMethod", AId::SpreadMethod),
        ("points", AId::Points),
        ("stroke", AId::Stroke),
        ("values", AId::Values),
        ("text-align", AId::TextAlign),
        ("font-size", AId::FontSize),
        ("maskContentUnits", AId::MaskContentUnits),
        ("mask-size", AId::MaskSize),
        ("path", AId::Path),
        ("specularConstant", AId::SpecularConstant),
        ("color", AId::Color),
        ("display", AId::Display),
        ("fill-opacity", AId::FillOpacity),
        ("word-spacing", AId::WordSpacing),
        ("cy", AId::Cy),
        ("startOffset", AId::StartOffset),
        ("mask-origin", AId::MaskOrigin),
        ("lengthAdjust", AId::LengthAdjust),
        ("white-space", AId::WhiteSpace),
        ("opacity", AId::Opacity),
        ("divisor", AId::Divisor),
        ("gradientUnits", AId::GradientUnits),
        ("stroke-dashoffset", AId::StrokeDashoffset),
        ("fill", AId::Fill),
        ("space", AId::Space),
        ("baseline-shift", AId::BaselineShift),
        ("slope", AId::Slope),
        ("color-rendering", AId::ColorRendering),
        ("unicode-range", AId::UnicodeRange),
        ("font-variant-caps", AId::FontVariantCaps),
        ("text-rendering", AId::TextRendering),
        ("exponent", AId::Exponent),
        ("text-decoration-color", AId::TextDecorationColor),
        ("refX", AId::RefX),
        ("x1", AId::X1),
        ("scale", AId::Scale),
        ("clip-rule", AId::ClipRule),
        ("kerning", AId::Kerning),
        ("mix-blend-mode", AId::MixBlendMode),
        ("mask-clip", AId::MaskClip),
        ("mask-mode", AId::MaskMode),
        ("type", AId::Type),
        ("mask-border", AId::MaskBorder),
        ("mode", AId::Mode),
        ("mask-border-repeat", AId::MaskBorderRepeat),
        ("stroke-miterlimit", AId::StrokeMiterlimit),
        ("letter-spacing", AId::LetterSpacing),
        ("font-stretch", AId::FontStretch),
        ("offset", AId::Offset),
        ("clip-path", AId::ClipPath),
        ("markerHeight", AId::MarkerHeight),
        ("text-underline-position", AId::TextUnderlinePosition),
        ("text-align-last", AId::TextAlignLast),
        ("width", AId::Width),
        ("font", AId::Font),
        ("font-family", AId::FontFamily),
        ("mask-position", AId::MaskPosition),
        ("result", AId::Result),
        ("font-size-adjust", AId::FontSizeAdjust),
        ("shape-margin", AId::ShapeMargin),
        ("direction", AId::Direction),
        ("font-variant", AId::FontVariant),
        ("radius", AId::Radius),
        ("maskUnits", AId::MaskUnits),
        ("clipPathUnits", AId::ClipPathUnits),
        ("text-orientation", AId::TextOrientation),
        ("amplitude", AId::Amplitude),
        ("ry", AId::Ry),
        ("mask-type", AId::MaskType),
        ("filter", AId::Filter),
        ("in", AId::In),
        ("dx", AId::Dx),
        ("seed", AId::Seed),
        ("class", AId::Class),
        ("color-profile", AId::ColorProfile),
        ("x", AId::X),
        ("href", AId::Href),
        ("font-feature-settings", AId::FontFeatureSettings),
        ("fill-rule", AId::FillRule),
        ("fr", AId::Fr),
        ("mask-border-mode", AId::MaskBorderMode),
        ("text-decoration-style", AId::TextDecorationStyle),
        ("numOctaves", AId::NumOctaves),
        ("shape-padding", AId::ShapePadding),
        ("visibility", AId::Visibility),
        ("id", AId::Id),
        ("targetX", AId::TargetX),
        ("transform-box", AId::TransformBox),
        ("stitchTiles", AId::StitchTiles),
        ("text-decoration-line", AId::TextDecorationLine),
        ("requiredFeatures", AId::RequiredFeatures),
        ("intercept", AId::Intercept),
        ("tableValues", AId::TableValues),
        ("text-overflow", AId::TextOverflow),
        ("paint-order", AId::PaintOrder),
        ("azimuth", AId::Azimuth),
        ("text-indent", AId::TextIndent),
        ("isolation", AId::Isolation),
        ("text-decoration-stroke", AId::TextDecorationStroke),
        ("gradientTransform", AId::GradientTransform),
        ("stroke-linecap", AId::StrokeLinecap),
        ("systemLanguage", AId::SystemLanguage),
        ("stroke-width", AId::StrokeWidth),
        ("requiredExtensions", AId::RequiredExtensions),
        ("writing-mode", AId::WritingMode),
        ("order", AId::Order),
        ("height", AId::Height),
        ("shape-image-threshold", AId::ShapeImageThreshold),
        ("style", AId::Style),
        ("pointsAtZ", AId::PointsAtZ),
        ("rx", AId::Rx),
        ("font-synthesis", AId::FontSynthesis),
        ("markerUnits", AId::MarkerUnits),
        ("lighting-color", AId::LightingColor),
        ("cx", AId::Cx),
        ("alignment-baseline", AId::AlignmentBaseline),
        ("font-kerning", AId::FontKerning),
        ("kernelMatrix", AId::KernelMatrix),
        ("glyph-orientation-vertical", AId::GlyphOrientationVertical),
        ("mask-border-outset", AId::MaskBorderOutset),
        ("primitiveUnits", AId::PrimitiveUnits),
        ("textLength", AId::TextLength),
        ("text-decoration-fill", AId::TextDecorationFill),
        ("fy", AId::Fy),
        ("unicode-bidi", AId::UnicodeBidi),
        ("k3", AId::K3),
        ("font-variant-numeric", AId::FontVariantNumeric),
        ("orient", AId::Orient),
        ("k1", AId::K1),
        ("refY", AId::RefY),
        ("edgeMode", AId::EdgeMode),
        ("shape-rendering", AId::ShapeRendering),
        ("font-variant-ligatures", AId::FontVariantLigatures),
        ("y", AId::Y),
        ("marker-end", AId::MarkerEnd),
        ("rotate", AId::Rotate),
        ("transform", AId::Transform),
        ("patternContentUnits", AId::PatternContentUnits),
        ("mask", AId::Mask),
        ("font-variant-position", AId::FontVariantPosition),
        ("elevation", AId::Elevation),
        ("pathLength", AId::PathLength),
        ("text-decoration", AId::TextDecoration),
        ("shape-subtract", AId::ShapeSubtract),
        ("preserveAspectRatio", AId::PreserveAspectRatio),
        ("y2", AId::Y2),
        ("dy", AId::Dy),
        ("yChannelSelector", AId::YChannelSelector),
        ("xChannelSelector", AId::XChannelSelector),
        ("image-rendering", AId::ImageRendering),
        ("z", AId::Z),
        ("patternUnits", AId::PatternUnits),
        ("marker-start", AId::MarkerStart),
        ("pointsAtY", AId::PointsAtY),
        ("d", AId::D),
        ("shape-inside", AId::ShapeInside),
        ("preserveAlpha", AId::PreserveAlpha),
        ("color-interpolation", AId::ColorInterpolation),
        ("operator", AId::Operator),
        ("marker-mid", AId::MarkerMid),
        ("kernelUnitLength", AId::KernelUnitLength),
        ("bias", AId::Bias),
        ("mask-border-slice", AId::MaskBorderSlice),
        ("pointsAtX", AId::PointsAtX),
        ("stop-color", AId::StopColor),
        ("line-height", AId::LineHeight),
        ("label", AId::Label),
        ("fx", AId::Fx),
        ("patternTransform", AId::PatternTransform),
        ("glyph-orientation-horizontal", AId::GlyphOrientationHorizontal),
        ("transform-origin", AId::TransformOrigin),
        ("font-weight", AId::FontWeight),
        ("overflow", AId::Overflow),
        ("y1", AId::Y1),
        ("r", AId::R),
        ("k2", AId::K2),
        ("text-anchor", AId::TextAnchor),
        ("inline-size", AId::InlineSize),
        ("x2", AId::X2),
        ("limitingConeAngle", AId::LimitingConeAngle),
        ("color-interpolation-filters", AId::ColorInterpolationFilters),
        ("stdDeviation", AId::StdDeviation),
        ("baseFrequency", AId::BaseFrequency),
        ("surfaceScale", AId::SurfaceScale),
        ("font-variant-east-asian", AId::FontVariantEastAsian),
        ("mask-image", AId::MaskImage),
        ("clip", AId::Clip),
        ("markerWidth", AId::MarkerWidth),
        ("flood-color", AId::FloodColor),
        ("in2", AId::In2),
        ("side", AId::Side),
        ("mask-composite", AId::MaskComposite),
        ("k4", AId::K4),
        ("mask-border-width", AId::MaskBorderWidth),
        ("vector-effect", AId::VectorEffect),
        ("viewBox", AId::ViewBox),
        ("font-style", AId::FontStyle),
        ("enable-background", AId::EnableBackground),
        ("filterUnits", AId::FilterUnits),
        ("stroke-dasharray", AId::StrokeDasharray),
        ("specularExponent", AId::SpecularExponent),
        ("stroke-opacity", AId::StrokeOpacity),
        ("targetY", AId::TargetY),
        ("flood-opacity", AId::FloodOpacity),
        ("diffuseConstant", AId::DiffuseConstant),
    ],
};

impl AId {
    pub(crate) fn from_str(text: &str) -> Option<AId> {
        ATTRIBUTES.get(text).cloned()
    }

    /// Returns the original string.
    #[inline(never)]
    pub fn to_str(self) -> &'static str {
        ATTRIBUTES.key(&self)
    }
}

impl std::fmt::Debug for AId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl std::fmt::Display for AId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// A stripped down `phf` crate fork.
//
// https://github.com/sfackler/rust-phf

struct Map<V: 'static> {
    pub key: u64,
    pub disps: &'static [(u32, u32)],
    pub entries: &'static[(&'static str, V)],
}

impl<V: PartialEq> Map<V> {
    fn get(&self, key: &str) -> Option<&V> {
        use std::borrow::Borrow;

        let hash = hash(key, self.key);
        let index = get_index(hash, self.disps, self.entries.len());
        let entry = &self.entries[index as usize];
        let b = entry.0.borrow();
        if b == key {
            Some(&entry.1)
        } else {
            None
        }
    }

    fn key(&self, value: &V) -> &'static str {
        self.entries.iter().find(|kv| kv.1 == *value).unwrap().0
    }
}

#[inline]
fn hash(x: &str, key: u64) -> u64 {
    use std::hash::Hasher;

    let mut hasher = siphasher::sip::SipHasher13::new_with_keys(0, key);
    hasher.write(x.as_bytes());
    hasher.finish()
}

#[inline]
fn get_index(hash: u64, disps: &[(u32, u32)], len: usize) -> u32 {
    let (g, f1, f2) = split(hash);
    let (d1, d2) = disps[(g % (disps.len() as u32)) as usize];
    displace(f1, f2, d1, d2) % (len as u32)
}

#[inline]
fn split(hash: u64) -> (u32, u32, u32) {
    const BITS: u32 = 21;
    const MASK: u64 = (1 << BITS) - 1;

    ((hash & MASK) as u32,
     ((hash >> BITS) & MASK) as u32,
     ((hash >> (2 * BITS)) & MASK) as u32)
}

#[inline]
fn displace(f1: u32, f2: u32, d1: u32, d2: u32) -> u32 {
    d2 + f1 * d1 + f2
}
