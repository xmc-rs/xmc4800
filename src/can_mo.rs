#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mo: [MO; 256],
}
impl RegisterBlock {
    #[doc = "0x00..0x2000 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo(&self, n: usize) -> &MO {
        &self.mo[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x2000 - Message Object Registers"]
    #[inline(always)]
    pub fn mo_iter(&self) -> impl Iterator<Item = &MO> {
        self.mo.iter()
    }
}
#[doc = "Message Object Registers"]
pub use self::mo::MO;
#[doc = r"Cluster"]
#[doc = "Message Object Registers"]
pub mod mo;
