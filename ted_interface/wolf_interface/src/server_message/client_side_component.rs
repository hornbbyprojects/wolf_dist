use wolf_serialise::WolfSerialise;

#[derive(Debug, Clone, PartialEq, Eq, Hash, WolfSerialise)]
pub struct CreateComponentMessage {
    pub component_id: u32,
    pub data: CreateComponentData,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, WolfSerialise)]
pub struct CreateDrawableData {
    pub sprite: u32,
    pub depth: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, WolfSerialise)]
pub struct CreateColouredData {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, WolfSerialise)]
pub struct SlashAnimationData {
    pub start_tick: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, WolfSerialise)]
pub enum CreateComponentData {
    HealthBar,
    Coloured(CreateColouredData),
    WideVision,
    Drawable(CreateDrawableData),
    HealthProportionTenThousandths(u32),
    SlashAnimation(SlashAnimationData),
    Speech(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateComponentData {}

impl wolf_serialise::WolfSerialise for UpdateComponentData {
    fn wolf_serialise<W: std::io::Write>(&self, _out_stream: &mut W) -> std::io::Result<()> {
        unimplemented!()
    }
    fn wolf_deserialise<R: std::io::Read>(_in_stream: &mut R) -> std::io::Result<Self> {
        unimplemented!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, WolfSerialise)]
pub struct UpdateComponentMessage {
    pub component_id: u32,
    pub data: UpdateComponentData,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, WolfSerialise)]
pub struct RemoveComponentMessage {
    pub component_id: u32,
}
