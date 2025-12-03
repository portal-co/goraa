#![no_std]

use core::fmt::Display;

pub struct Goraa<T,R>(pub T, pub R);
pub mod x64;
#[derive(Clone, Copy,PartialEq, Eq, PartialOrd, Ord,Hash,Debug)]
pub struct ShimLabel<L> {
    pub wrapped: L,
    pub kind: ShimKind,
}
#[derive(Clone, Copy,PartialEq, Eq, PartialOrd, Ord,Hash,Debug)]
pub enum ShimKind {
    None,
    JmpD,
}
impl Display for ShimKind{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self{
            ShimKind::None => Ok(()),
            ShimKind::JmpD => write!(f,"_jd_"),
        }
    }
}
impl<L: Display> Display for ShimLabel<L>{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f,"{}{}",&self.kind,&self.wrapped)
    }
}