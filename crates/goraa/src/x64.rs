use crate::*;

use portal_solutions_asm_x86_64::out::{Writer, WriterCore};
pub trait GoraaDeps: WriterCore{

}
impl<T: WriterCore+?Sized> GoraaDeps for T{}
impl<T: GoraaDeps + Writer<L>,L> Writer<L> for Goraa<T>{
    fn set_label(&mut self, cfg: portal_solutions_asm_x86_64::X64Arch, s: L) -> Result<(), Self::Error> {

        self.0.set_label(cfg, s)?;

        Ok(())
    }

    fn lea_label(
        &mut self,
        cfg: portal_solutions_asm_x86_64::X64Arch,
        dest: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
        label: L,
    ) -> Result<(), Self::Error> {
        self.0.lea_label(cfg, dest, label)?;

        Ok(())
    }
}
impl<T: GoraaDeps> WriterCore for Goraa<T>{
    type Error = T::Error;

    fn hlt(&mut self, cfg: portal_solutions_asm_x86_64::X64Arch) -> Result<(), Self::Error> {
        todo!()
    }

    fn xchg(
        &mut self,
        cfg: portal_solutions_asm_x86_64::X64Arch,
        dest: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
        src: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn mov(
        &mut self,
        cfg: portal_solutions_asm_x86_64::X64Arch,
        dest: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
        src: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn sub(
        &mut self,
        cfg: portal_solutions_asm_x86_64::X64Arch,
        a: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
        b: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn movsx(
        &mut self,
        cfg: portal_solutions_asm_x86_64::X64Arch,
        dest: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
        src: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn movzx(
        &mut self,
        cfg: portal_solutions_asm_x86_64::X64Arch,
        dest: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
        src: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn push(&mut self, cfg: portal_solutions_asm_x86_64::X64Arch, op: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_)) -> Result<(), Self::Error> {
        todo!()
    }

    fn pop(&mut self, cfg: portal_solutions_asm_x86_64::X64Arch, op: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_)) -> Result<(), Self::Error> {
        todo!()
    }

    fn call(&mut self, cfg: portal_solutions_asm_x86_64::X64Arch, op: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_)) -> Result<(), Self::Error> {
        todo!()
    }

    fn jmp(&mut self, cfg: portal_solutions_asm_x86_64::X64Arch, op: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_)) -> Result<(), Self::Error> {
        todo!()
    }

    fn cmp0(&mut self, cfg: portal_solutions_asm_x86_64::X64Arch, op: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_)) -> Result<(), Self::Error> {
        todo!()
    }

    fn cmovcc64(
        &mut self,
        cfg: portal_solutions_asm_x86_64::X64Arch,
        cond: portal_solutions_asm_x86_64::ConditionCode,
        op: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
        val: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn jcc(
        &mut self,
        cfg: portal_solutions_asm_x86_64::X64Arch,
        cond: portal_solutions_asm_x86_64::ConditionCode,
        op: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn u32(&mut self, cfg: portal_solutions_asm_x86_64::X64Arch, op: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_)) -> Result<(), Self::Error> {
        todo!()
    }

    fn not(&mut self, cfg: portal_solutions_asm_x86_64::X64Arch, op: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_)) -> Result<(), Self::Error> {
        todo!()
    }

    fn lea(
        &mut self,
        cfg: portal_solutions_asm_x86_64::X64Arch,
        dest: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
        src: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn get_ip(&mut self, cfg: portal_solutions_asm_x86_64::X64Arch) -> Result<(), Self::Error> {
        todo!()
    }

    fn ret(&mut self, cfg: portal_solutions_asm_x86_64::X64Arch) -> Result<(), Self::Error> {
        todo!()
    }

    fn mov64(
        &mut self,
        cfg: portal_solutions_asm_x86_64::X64Arch,
        r: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
        val: u64,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn mul(
        &mut self,
        cfg: portal_solutions_asm_x86_64::X64Arch,
        a: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
        b: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn div(
        &mut self,
        cfg: portal_solutions_asm_x86_64::X64Arch,
        a: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
        b: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn idiv(
        &mut self,
        cfg: portal_solutions_asm_x86_64::X64Arch,
        a: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
        b: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn and(
        &mut self,
        cfg: portal_solutions_asm_x86_64::X64Arch,
        a: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
        b: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn or(
        &mut self,
        cfg: portal_solutions_asm_x86_64::X64Arch,
        a: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
        b: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn eor(
        &mut self,
        cfg: portal_solutions_asm_x86_64::X64Arch,
        a: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
        b: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn shl(
        &mut self,
        cfg: portal_solutions_asm_x86_64::X64Arch,
        a: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
        b: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn shr(
        &mut self,
        cfg: portal_solutions_asm_x86_64::X64Arch,
        a: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
        b: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn fadd(
        &mut self,
        cfg: portal_solutions_asm_x86_64::X64Arch,
        dest: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
        src: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn fsub(
        &mut self,
        cfg: portal_solutions_asm_x86_64::X64Arch,
        dest: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
        src: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn fmul(
        &mut self,
        cfg: portal_solutions_asm_x86_64::X64Arch,
        dest: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
        src: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn fdiv(
        &mut self,
        cfg: portal_solutions_asm_x86_64::X64Arch,
        dest: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
        src: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn fmov(
        &mut self,
        cfg: portal_solutions_asm_x86_64::X64Arch,
        dest: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
        src: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
    ) -> Result<(), Self::Error> {
        todo!()
    }
}