use wolf_serialise::WolfSerialise;

#[derive(Debug, Clone, PartialEq)]
pub struct MoveCommand {
    pub dx: f64,
    pub dy: f64,
}

impl WolfSerialise for MoveCommand {
    fn wolf_serialise<W: std::io::Write>(&self, out_stream: &mut W) -> std::io::Result<()> {
        (self.dx as f32).wolf_serialise(out_stream)?;
        (self.dy as f32).wolf_serialise(out_stream)?;
        Ok(())
    }
    fn wolf_deserialise<R: std::io::Read>(in_stream: &mut R) -> std::io::Result<Self> {
        let dx = f32::wolf_deserialise(in_stream)? as f64;
        let dy = f32::wolf_deserialise(in_stream)? as f64;
        Ok(MoveCommand { dx, dy })
    }
}
