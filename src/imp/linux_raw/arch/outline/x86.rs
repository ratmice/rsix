#![allow(dead_code)]

use crate::imp::linux_raw::reg::{ArgReg, RetReg, SyscallNumber, A0, A1, A2, A3, A4, A5, R0};
use crate::imp::linux_raw::vdso_wrappers::SyscallType;

// x86 (using fastcall) prefers to pass a1 and a2 first, before a0, because
// fastcall passes the first two arguments in ecx and edx, which are the second
// and third Linux syscall arguments.
//
// First we declare the actual assembly routines with `rsix_reordered_*`
// names and reorgered arguments.
extern "fastcall" {
    fn rsix_reordered_syscall0_readonly(nr: SyscallNumber) -> RetReg<R0>;
    fn rsix_reordered_syscall1(a0: ArgReg<A0>, nr: SyscallNumber) -> RetReg<R0>;
    fn rsix_reordered_syscall1_readonly(a0: ArgReg<A0>, nr: SyscallNumber) -> RetReg<R0>;
    fn rsix_reordered_syscall1_noreturn(a0: ArgReg<A0>, nr: SyscallNumber) -> !;
    fn rsix_reordered_syscall2(a1: ArgReg<A1>, a0: ArgReg<A0>, nr: SyscallNumber) -> RetReg<R0>;
    fn rsix_reordered_syscall2_readonly(
        a1: ArgReg<A1>,
        a0: ArgReg<A0>,
        nr: SyscallNumber,
    ) -> RetReg<R0>;
    fn rsix_reordered_syscall3(
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
        a0: ArgReg<A0>,
        nr: SyscallNumber,
    ) -> RetReg<R0>;
    fn rsix_reordered_syscall3_readonly(
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
        a0: ArgReg<A0>,
        nr: SyscallNumber,
    ) -> RetReg<R0>;
    fn rsix_reordered_syscall4(
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
        a0: ArgReg<A0>,
        a3: ArgReg<A3>,
        nr: SyscallNumber,
    ) -> RetReg<R0>;
    fn rsix_reordered_syscall4_readonly(
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
        a0: ArgReg<A0>,
        a3: ArgReg<A3>,
        nr: SyscallNumber,
    ) -> RetReg<R0>;
    fn rsix_reordered_syscall5(
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
        a0: ArgReg<A0>,
        a3: ArgReg<A3>,
        a4: ArgReg<A4>,
        nr: SyscallNumber,
    ) -> RetReg<R0>;
    fn rsix_reordered_syscall5_readonly(
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
        a0: ArgReg<A0>,
        a3: ArgReg<A3>,
        a4: ArgReg<A4>,
        nr: SyscallNumber,
    ) -> RetReg<R0>;
    fn rsix_reordered_syscall6(
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
        a0: ArgReg<A0>,
        a3: ArgReg<A3>,
        a4: ArgReg<A4>,
        a5: ArgReg<A5>,
        nr: SyscallNumber,
    ) -> RetReg<R0>;
    fn rsix_reordered_syscall6_readonly(
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
        a0: ArgReg<A0>,
        a3: ArgReg<A3>,
        a4: ArgReg<A4>,
        a5: ArgReg<A5>,
        nr: SyscallNumber,
    ) -> RetReg<R0>;
}

// Then we define inline wrapper functions that do the reordering.
mod reorder {
    use super::*;

    #[inline]
    #[must_use]
    pub(in crate::imp::linux_raw) unsafe fn syscall0_readonly(nr: SyscallNumber) -> RetReg<R0> {
        rsix_reordered_syscall0_readonly(nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp::linux_raw) unsafe fn syscall1(
        nr: SyscallNumber,
        a0: ArgReg<A0>,
    ) -> RetReg<R0> {
        rsix_reordered_syscall1(a0, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp::linux_raw) unsafe fn syscall1_readonly(
        nr: SyscallNumber,
        a0: ArgReg<A0>,
    ) -> RetReg<R0> {
        rsix_reordered_syscall1_readonly(a0, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp::linux_raw) unsafe fn syscall1_noreturn(
        nr: SyscallNumber,
        a0: ArgReg<A0>,
    ) -> ! {
        rsix_reordered_syscall1_noreturn(a0, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp::linux_raw) unsafe fn syscall2(
        nr: SyscallNumber,
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
    ) -> RetReg<R0> {
        rsix_reordered_syscall2(a1, a0, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp::linux_raw) unsafe fn syscall2_readonly(
        nr: SyscallNumber,
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
    ) -> RetReg<R0> {
        rsix_reordered_syscall2_readonly(a1, a0, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp::linux_raw) unsafe fn syscall3(
        nr: SyscallNumber,
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
    ) -> RetReg<R0> {
        rsix_reordered_syscall3(a1, a2, a0, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp::linux_raw) unsafe fn syscall3_readonly(
        nr: SyscallNumber,
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
    ) -> RetReg<R0> {
        rsix_reordered_syscall3_readonly(a1, a2, a0, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp::linux_raw) unsafe fn syscall4(
        nr: SyscallNumber,
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
        a3: ArgReg<A3>,
    ) -> RetReg<R0> {
        rsix_reordered_syscall4(a1, a2, a0, a3, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp::linux_raw) unsafe fn syscall4_readonly(
        nr: SyscallNumber,
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
        a3: ArgReg<A3>,
    ) -> RetReg<R0> {
        rsix_reordered_syscall4_readonly(a1, a2, a0, a3, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp::linux_raw) unsafe fn syscall5(
        nr: SyscallNumber,
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
        a3: ArgReg<A3>,
        a4: ArgReg<A4>,
    ) -> RetReg<R0> {
        rsix_reordered_syscall5(a1, a2, a0, a3, a4, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp::linux_raw) unsafe fn syscall5_readonly(
        nr: SyscallNumber,
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
        a3: ArgReg<A3>,
        a4: ArgReg<A4>,
    ) -> RetReg<R0> {
        rsix_reordered_syscall5_readonly(a1, a2, a0, a3, a4, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp::linux_raw) unsafe fn syscall6(
        nr: SyscallNumber,
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
        a3: ArgReg<A3>,
        a4: ArgReg<A4>,
        a5: ArgReg<A5>,
    ) -> RetReg<R0> {
        rsix_reordered_syscall6(a1, a2, a0, a3, a4, a5, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp::linux_raw) unsafe fn syscall6_readonly(
        nr: SyscallNumber,
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
        a3: ArgReg<A3>,
        a4: ArgReg<A4>,
        a5: ArgReg<A5>,
    ) -> RetReg<R0> {
        rsix_reordered_syscall6_readonly(a1, a2, a0, a3, a4, a5, nr)
    }
}

pub(in crate::imp::linux_raw) use reorder::*;

// x86 prefers to route all syscalls through the vDSO, though this isn't
// always possible, so it also has a special form for doing the dispatch.
//
// First we declare the actual assembly routines with `rsix_reordered_*`
// names and reorgered arguments.
extern "fastcall" {
    fn rsix_reordered_indirect_syscall0(nr: SyscallNumber, callee: SyscallType) -> RetReg<R0>;
    fn rsix_reordered_indirect_syscall1(
        a0: ArgReg<A0>,
        nr: SyscallNumber,
        callee: SyscallType,
    ) -> RetReg<R0>;
    fn rsix_reordered_indirect_syscall1_noreturn(
        a0: ArgReg<A0>,
        nr: SyscallNumber,
        callee: SyscallType,
    ) -> !;
    fn rsix_reordered_indirect_syscall2(
        a1: ArgReg<A1>,
        a0: ArgReg<A0>,
        nr: SyscallNumber,
        callee: SyscallType,
    ) -> RetReg<R0>;
    fn rsix_reordered_indirect_syscall3(
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
        a0: ArgReg<A0>,
        nr: SyscallNumber,
        callee: SyscallType,
    ) -> RetReg<R0>;
    fn rsix_reordered_indirect_syscall4(
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
        a0: ArgReg<A0>,
        a3: ArgReg<A3>,
        nr: SyscallNumber,
        callee: SyscallType,
    ) -> RetReg<R0>;
    fn rsix_reordered_indirect_syscall5(
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
        a0: ArgReg<A0>,
        a3: ArgReg<A3>,
        a4: ArgReg<A4>,
        nr: SyscallNumber,
        callee: SyscallType,
    ) -> RetReg<R0>;
    fn rsix_reordered_indirect_syscall6(
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
        a0: ArgReg<A0>,
        a3: ArgReg<A3>,
        a4: ArgReg<A4>,
        a5: ArgReg<A5>,
        nr: SyscallNumber,
        callee: SyscallType,
    ) -> RetReg<R0>;
}

// Then we define inline wrapper functions that do the reordering.
mod reorder_indirect {
    use super::*;

    #[inline]
    #[must_use]
    pub(in crate::imp::linux_raw) unsafe fn indirect_syscall0(
        callee: SyscallType,
        nr: SyscallNumber,
    ) -> RetReg<R0> {
        rsix_reordered_indirect_syscall0(nr, callee)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp::linux_raw) unsafe fn indirect_syscall1(
        callee: SyscallType,
        nr: SyscallNumber,
        a0: ArgReg<A0>,
    ) -> RetReg<R0> {
        rsix_reordered_indirect_syscall1(a0, nr, callee)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp::linux_raw) unsafe fn indirect_syscall1_noreturn(
        callee: SyscallType,
        nr: SyscallNumber,
        a0: ArgReg<A0>,
    ) -> ! {
        rsix_reordered_indirect_syscall1_noreturn(a0, nr, callee)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp::linux_raw) unsafe fn indirect_syscall2(
        callee: SyscallType,
        nr: SyscallNumber,
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
    ) -> RetReg<R0> {
        rsix_reordered_indirect_syscall2(a1, a0, nr, callee)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp::linux_raw) unsafe fn indirect_syscall3(
        callee: SyscallType,
        nr: SyscallNumber,
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
    ) -> RetReg<R0> {
        rsix_reordered_indirect_syscall3(a1, a2, a0, nr, callee)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp::linux_raw) unsafe fn indirect_syscall4(
        callee: SyscallType,
        nr: SyscallNumber,
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
        a3: ArgReg<A3>,
    ) -> RetReg<R0> {
        rsix_reordered_indirect_syscall4(a1, a2, a0, a3, nr, callee)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp::linux_raw) unsafe fn indirect_syscall5(
        callee: SyscallType,
        nr: SyscallNumber,
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
        a3: ArgReg<A3>,
        a4: ArgReg<A4>,
    ) -> RetReg<R0> {
        rsix_reordered_indirect_syscall5(a1, a2, a0, a3, a4, nr, callee)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp::linux_raw) unsafe fn indirect_syscall6(
        callee: SyscallType,
        nr: SyscallNumber,
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
        a3: ArgReg<A3>,
        a4: ArgReg<A4>,
        a5: ArgReg<A5>,
    ) -> RetReg<R0> {
        rsix_reordered_indirect_syscall6(a1, a2, a0, a3, a4, a5, nr, callee)
    }
}

pub(in crate::imp::linux_raw) use reorder_indirect::*;
