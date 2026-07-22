pub type Rounding = miqat::Rounding;

#[uniffi::remote(Enum)]
pub enum Rounding {
    Nearest,
    Ceil,
    None,
}
