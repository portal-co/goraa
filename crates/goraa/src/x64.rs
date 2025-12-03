use crate::*;

use portal_pc_asm_common::types::reg::Reg;
use portal_solutions_asm_x86_64::{
    ConditionCode, X64Arch,
    out::{Writer, WriterCore, arg::MemArgKind},
};
use rand::Rng;
pub trait GoraaDeps: WriterCore {
    fn mash(&mut self, cfg: X64Arch) -> Result<(), Self::Error> {
        let sp = MemArgKind::Mem {
            base: Reg(4),
            offset: None,
            disp: 0,
            size: portal_pc_asm_common::types::mem::MemorySize::_64,
            reg_class: portal_solutions_asm_x86_64::RegisterClass::Gpr,
        };
        self.xchg(cfg, &sp, &Reg(0))?;
        self.pushf(cfg)?;
        self.push(cfg, &0u64)?;
        Ok(())
    }
}
impl<T: WriterCore + ?Sized> GoraaDeps for T {}
impl<T: GoraaDeps + Writer<ShimLabel<L>>, L: Clone, R: Rng> Writer<L> for Goraa<T, R> {
    fn set_label(
        &mut self,
        cfg: portal_solutions_asm_x86_64::X64Arch,
        s: L,
    ) -> Result<(), Self::Error> {
        let a = loop {
            let a = self.1.random();
            if a % 16 != 4 {
                break a;
            }
        };
        let sp = MemArgKind::Mem {
            base: Reg(4),
            offset: None,
            disp: 0,
            size: portal_pc_asm_common::types::mem::MemorySize::_64,
            reg_class: portal_solutions_asm_x86_64::RegisterClass::Gpr,
        };
        self.0.mash(cfg)?;
        self.0.set_label(
            cfg,
            ShimLabel {
                wrapped: s.clone(),
                kind: ShimKind::None,
            },
        )?;
        self.0.push(cfg, &Reg(a))?;
        self.0.cmp0(cfg, &sp)?;
        self.0.lea_label(
            cfg,
            &Reg(a),
            ShimLabel {
                wrapped: s.clone(),
                kind: ShimKind::JmpD,
            },
        )?;
        self.0.jcc(cfg, ConditionCode::NE, &Reg(a))?;
        self.0.pop(cfg, &Reg(a))?;
        self.0.popf(cfg)?;
        self.0.xchg(cfg, &sp, &Reg(0))?;
        self.0.push(cfg, &Reg(a))?;
        self.0.set_label(
            cfg,
            ShimLabel {
                wrapped: s.clone(),
                kind: ShimKind::JmpD,
            },
        )?;
        self.0.pop(cfg, &Reg(a))?;
        Ok(())
    }

    fn lea_label(
        &mut self,
        cfg: portal_solutions_asm_x86_64::X64Arch,
        dest: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
        label: L,
    ) -> Result<(), Self::Error> {
        self.0.lea_label(
            cfg,
            dest,
            ShimLabel {
                wrapped: label,
                kind: ShimKind::None,
            },
        )?;

        Ok(())
    }
}
impl<T: GoraaDeps, R: Rng> WriterCore for Goraa<T, R> {
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

    fn push(
        &mut self,
        cfg: portal_solutions_asm_x86_64::X64Arch,
        op: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn pop(
        &mut self,
        cfg: portal_solutions_asm_x86_64::X64Arch,
        op: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn call(
        &mut self,
        cfg: portal_solutions_asm_x86_64::X64Arch,
        op: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn jmp(
        &mut self,
        cfg: portal_solutions_asm_x86_64::X64Arch,
        op: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn cmp0(
        &mut self,
        cfg: portal_solutions_asm_x86_64::X64Arch,
        op: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
    ) -> Result<(), Self::Error> {
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

    fn u32(
        &mut self,
        cfg: portal_solutions_asm_x86_64::X64Arch,
        op: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn not(
        &mut self,
        cfg: portal_solutions_asm_x86_64::X64Arch,
        op: &(dyn portal_solutions_asm_x86_64::out::arg::MemArg + '_),
    ) -> Result<(), Self::Error> {
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
