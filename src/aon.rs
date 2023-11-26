#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    wdogcfg: WDOGCFG,
    _reserved1: [u8; 0x04],
    wdogcount: WDOGCOUNT,
    _reserved2: [u8; 0x04],
    wdogs: WDOGS,
    _reserved3: [u8; 0x04],
    wdogfeed: WDOGFEED,
    wdogkey: WDOGKEY,
    wdogcmp0: WDOGCMP0,
    _reserved6: [u8; 0x1c],
    rtccfg: RTCCFG,
    _reserved7: [u8; 0x04],
    rtccountlo: RTCCOUNTLO,
    rtccounthi: RTCCOUNTHI,
    rtcs: RTCS,
    _reserved10: [u8; 0x0c],
    rtccmp0: RTCCMP0,
    _reserved11: [u8; 0x0c],
    lfrosccfg: LFROSCCFG,
    _reserved12: [u8; 0x08],
    lfclkmux: LFCLKMUX,
    backup_0: BACKUP_0,
    backup_1: BACKUP_1,
    backup_2: BACKUP_2,
    backup_3: BACKUP_3,
    backup_4: BACKUP_4,
    backup_5: BACKUP_5,
    backup_6: BACKUP_6,
    backup_7: BACKUP_7,
    backup_8: BACKUP_8,
    backup_9: BACKUP_9,
    backup_10: BACKUP_10,
    backup_11: BACKUP_11,
    backup_12: BACKUP_12,
    backup_13: BACKUP_13,
    backup_14: BACKUP_14,
    backup_15: BACKUP_15,
    _reserved29: [u8; 0x40],
    pmuwakeupi0: PMUWAKEUPI0,
    pmuwakeupi1: PMUWAKEUPI1,
    pmuwakeupi2: PMUWAKEUPI2,
    pmuwakeupi3: PMUWAKEUPI3,
    pmuwakeupi4: PMUWAKEUPI4,
    pmuwakeupi5: PMUWAKEUPI5,
    pmuwakeupi6: PMUWAKEUPI6,
    pmuwakeupi7: PMUWAKEUPI7,
    pmusleepi0: PMUSLEEPI0,
    pmusleepi1: PMUSLEEPI1,
    pmusleepi2: PMUSLEEPI2,
    pmusleepi3: PMUSLEEPI3,
    pmusleepi4: PMUSLEEPI4,
    pmusleepi5: PMUSLEEPI5,
    pmusleepi6: PMUSLEEPI6,
    pmusleepi7: PMUSLEEPI7,
    pmuie: PMUIE,
    pmucause: PMUCAUSE,
    pmusleep: PMUSLEEP,
    pmukey: PMUKEY,
    _reserved49: [u8; 0x01b0],
    aoncfg: AONCFG,
}
impl RegisterBlock {
    #[doc = "0x00 - wdog Configuration"]
    #[inline(always)]
    pub const fn wdogcfg(&self) -> &WDOGCFG {
        &self.wdogcfg
    }
    #[doc = "0x08 - Counter Register"]
    #[inline(always)]
    pub const fn wdogcount(&self) -> &WDOGCOUNT {
        &self.wdogcount
    }
    #[doc = "0x10 - Scaled value of Counter"]
    #[inline(always)]
    pub const fn wdogs(&self) -> &WDOGS {
        &self.wdogs
    }
    #[doc = "0x18 - Feed register"]
    #[inline(always)]
    pub const fn wdogfeed(&self) -> &WDOGFEED {
        &self.wdogfeed
    }
    #[doc = "0x1c - Key Register"]
    #[inline(always)]
    pub const fn wdogkey(&self) -> &WDOGKEY {
        &self.wdogkey
    }
    #[doc = "0x20 - Comparator 0"]
    #[inline(always)]
    pub const fn wdogcmp0(&self) -> &WDOGCMP0 {
        &self.wdogcmp0
    }
    #[doc = "0x40 - rtc Configuration"]
    #[inline(always)]
    pub const fn rtccfg(&self) -> &RTCCFG {
        &self.rtccfg
    }
    #[doc = "0x48 - Low bits of Counter"]
    #[inline(always)]
    pub const fn rtccountlo(&self) -> &RTCCOUNTLO {
        &self.rtccountlo
    }
    #[doc = "0x4c - High bits of Counter"]
    #[inline(always)]
    pub const fn rtccounthi(&self) -> &RTCCOUNTHI {
        &self.rtccounthi
    }
    #[doc = "0x50 - Scaled value of Counter"]
    #[inline(always)]
    pub const fn rtcs(&self) -> &RTCS {
        &self.rtcs
    }
    #[doc = "0x60 - Comparator 0"]
    #[inline(always)]
    pub const fn rtccmp0(&self) -> &RTCCMP0 {
        &self.rtccmp0
    }
    #[doc = "0x70 - Ring Oscillator Configuration and Status"]
    #[inline(always)]
    pub const fn lfrosccfg(&self) -> &LFROSCCFG {
        &self.lfrosccfg
    }
    #[doc = "0x7c - Low-Frequency Clock Mux Control and Status"]
    #[inline(always)]
    pub const fn lfclkmux(&self) -> &LFCLKMUX {
        &self.lfclkmux
    }
    #[doc = "0x80 - Backup Register 0"]
    #[inline(always)]
    pub const fn backup_0(&self) -> &BACKUP_0 {
        &self.backup_0
    }
    #[doc = "0x84 - Backup Register 1"]
    #[inline(always)]
    pub const fn backup_1(&self) -> &BACKUP_1 {
        &self.backup_1
    }
    #[doc = "0x88 - Backup Register 2"]
    #[inline(always)]
    pub const fn backup_2(&self) -> &BACKUP_2 {
        &self.backup_2
    }
    #[doc = "0x8c - Backup Register 3"]
    #[inline(always)]
    pub const fn backup_3(&self) -> &BACKUP_3 {
        &self.backup_3
    }
    #[doc = "0x90 - Backup Register 4"]
    #[inline(always)]
    pub const fn backup_4(&self) -> &BACKUP_4 {
        &self.backup_4
    }
    #[doc = "0x94 - Backup Register 5"]
    #[inline(always)]
    pub const fn backup_5(&self) -> &BACKUP_5 {
        &self.backup_5
    }
    #[doc = "0x98 - Backup Register 6"]
    #[inline(always)]
    pub const fn backup_6(&self) -> &BACKUP_6 {
        &self.backup_6
    }
    #[doc = "0x9c - Backup Register 7"]
    #[inline(always)]
    pub const fn backup_7(&self) -> &BACKUP_7 {
        &self.backup_7
    }
    #[doc = "0xa0 - Backup Register 8"]
    #[inline(always)]
    pub const fn backup_8(&self) -> &BACKUP_8 {
        &self.backup_8
    }
    #[doc = "0xa4 - Backup Register 9"]
    #[inline(always)]
    pub const fn backup_9(&self) -> &BACKUP_9 {
        &self.backup_9
    }
    #[doc = "0xa8 - Backup Register 10"]
    #[inline(always)]
    pub const fn backup_10(&self) -> &BACKUP_10 {
        &self.backup_10
    }
    #[doc = "0xac - Backup Register 11"]
    #[inline(always)]
    pub const fn backup_11(&self) -> &BACKUP_11 {
        &self.backup_11
    }
    #[doc = "0xb0 - Backup Register 12"]
    #[inline(always)]
    pub const fn backup_12(&self) -> &BACKUP_12 {
        &self.backup_12
    }
    #[doc = "0xb4 - Backup Register 13"]
    #[inline(always)]
    pub const fn backup_13(&self) -> &BACKUP_13 {
        &self.backup_13
    }
    #[doc = "0xb8 - Backup Register 14"]
    #[inline(always)]
    pub const fn backup_14(&self) -> &BACKUP_14 {
        &self.backup_14
    }
    #[doc = "0xbc - Backup Register 15"]
    #[inline(always)]
    pub const fn backup_15(&self) -> &BACKUP_15 {
        &self.backup_15
    }
    #[doc = "0x100 - Wakeup program instruction 0"]
    #[inline(always)]
    pub const fn pmuwakeupi0(&self) -> &PMUWAKEUPI0 {
        &self.pmuwakeupi0
    }
    #[doc = "0x104 - Wakeup program instruction 1"]
    #[inline(always)]
    pub const fn pmuwakeupi1(&self) -> &PMUWAKEUPI1 {
        &self.pmuwakeupi1
    }
    #[doc = "0x108 - Wakeup program instruction 2"]
    #[inline(always)]
    pub const fn pmuwakeupi2(&self) -> &PMUWAKEUPI2 {
        &self.pmuwakeupi2
    }
    #[doc = "0x10c - Wakeup program instruction 3"]
    #[inline(always)]
    pub const fn pmuwakeupi3(&self) -> &PMUWAKEUPI3 {
        &self.pmuwakeupi3
    }
    #[doc = "0x110 - Wakeup program instruction 4"]
    #[inline(always)]
    pub const fn pmuwakeupi4(&self) -> &PMUWAKEUPI4 {
        &self.pmuwakeupi4
    }
    #[doc = "0x114 - Wakeup program instruction 5"]
    #[inline(always)]
    pub const fn pmuwakeupi5(&self) -> &PMUWAKEUPI5 {
        &self.pmuwakeupi5
    }
    #[doc = "0x118 - Wakeup program instruction 6"]
    #[inline(always)]
    pub const fn pmuwakeupi6(&self) -> &PMUWAKEUPI6 {
        &self.pmuwakeupi6
    }
    #[doc = "0x11c - Wakeup program instruction 7"]
    #[inline(always)]
    pub const fn pmuwakeupi7(&self) -> &PMUWAKEUPI7 {
        &self.pmuwakeupi7
    }
    #[doc = "0x120 - Sleep program instruction 0"]
    #[inline(always)]
    pub const fn pmusleepi0(&self) -> &PMUSLEEPI0 {
        &self.pmusleepi0
    }
    #[doc = "0x124 - Sleep program instruction 1"]
    #[inline(always)]
    pub const fn pmusleepi1(&self) -> &PMUSLEEPI1 {
        &self.pmusleepi1
    }
    #[doc = "0x128 - Sleep program instruction 2"]
    #[inline(always)]
    pub const fn pmusleepi2(&self) -> &PMUSLEEPI2 {
        &self.pmusleepi2
    }
    #[doc = "0x12c - Sleep program instruction 3"]
    #[inline(always)]
    pub const fn pmusleepi3(&self) -> &PMUSLEEPI3 {
        &self.pmusleepi3
    }
    #[doc = "0x130 - Sleep program instruction 4"]
    #[inline(always)]
    pub const fn pmusleepi4(&self) -> &PMUSLEEPI4 {
        &self.pmusleepi4
    }
    #[doc = "0x134 - Sleep program instruction 5"]
    #[inline(always)]
    pub const fn pmusleepi5(&self) -> &PMUSLEEPI5 {
        &self.pmusleepi5
    }
    #[doc = "0x138 - Sleep program instruction 6"]
    #[inline(always)]
    pub const fn pmusleepi6(&self) -> &PMUSLEEPI6 {
        &self.pmusleepi6
    }
    #[doc = "0x13c - Sleep program instruction 7"]
    #[inline(always)]
    pub const fn pmusleepi7(&self) -> &PMUSLEEPI7 {
        &self.pmusleepi7
    }
    #[doc = "0x140 - PMU Interrupt Enables"]
    #[inline(always)]
    pub const fn pmuie(&self) -> &PMUIE {
        &self.pmuie
    }
    #[doc = "0x144 - PMU Wakeup Cause"]
    #[inline(always)]
    pub const fn pmucause(&self) -> &PMUCAUSE {
        &self.pmucause
    }
    #[doc = "0x148 - Initiate PMU Sleep Sequence"]
    #[inline(always)]
    pub const fn pmusleep(&self) -> &PMUSLEEP {
        &self.pmusleep
    }
    #[doc = "0x14c - PMU Key. Reads as 1 when PMU is unlocked"]
    #[inline(always)]
    pub const fn pmukey(&self) -> &PMUKEY {
        &self.pmukey
    }
    #[doc = "0x300 - AON Block Configuration Information"]
    #[inline(always)]
    pub const fn aoncfg(&self) -> &AONCFG {
        &self.aoncfg
    }
}
#[doc = "backup_0 (rw) register accessor: Backup Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`backup_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`backup_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backup_0`]
module"]
pub type BACKUP_0 = crate::Reg<backup_0::BACKUP_0_SPEC>;
#[doc = "Backup Register 0"]
pub mod backup_0;
#[doc = "backup_1 (rw) register accessor: Backup Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`backup_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`backup_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backup_1`]
module"]
pub type BACKUP_1 = crate::Reg<backup_1::BACKUP_1_SPEC>;
#[doc = "Backup Register 1"]
pub mod backup_1;
#[doc = "backup_2 (rw) register accessor: Backup Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`backup_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`backup_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backup_2`]
module"]
pub type BACKUP_2 = crate::Reg<backup_2::BACKUP_2_SPEC>;
#[doc = "Backup Register 2"]
pub mod backup_2;
#[doc = "backup_3 (rw) register accessor: Backup Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`backup_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`backup_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backup_3`]
module"]
pub type BACKUP_3 = crate::Reg<backup_3::BACKUP_3_SPEC>;
#[doc = "Backup Register 3"]
pub mod backup_3;
#[doc = "backup_4 (rw) register accessor: Backup Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`backup_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`backup_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backup_4`]
module"]
pub type BACKUP_4 = crate::Reg<backup_4::BACKUP_4_SPEC>;
#[doc = "Backup Register 4"]
pub mod backup_4;
#[doc = "backup_5 (rw) register accessor: Backup Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`backup_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`backup_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backup_5`]
module"]
pub type BACKUP_5 = crate::Reg<backup_5::BACKUP_5_SPEC>;
#[doc = "Backup Register 5"]
pub mod backup_5;
#[doc = "backup_6 (rw) register accessor: Backup Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`backup_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`backup_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backup_6`]
module"]
pub type BACKUP_6 = crate::Reg<backup_6::BACKUP_6_SPEC>;
#[doc = "Backup Register 6"]
pub mod backup_6;
#[doc = "backup_7 (rw) register accessor: Backup Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`backup_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`backup_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backup_7`]
module"]
pub type BACKUP_7 = crate::Reg<backup_7::BACKUP_7_SPEC>;
#[doc = "Backup Register 7"]
pub mod backup_7;
#[doc = "backup_8 (rw) register accessor: Backup Register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`backup_8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`backup_8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backup_8`]
module"]
pub type BACKUP_8 = crate::Reg<backup_8::BACKUP_8_SPEC>;
#[doc = "Backup Register 8"]
pub mod backup_8;
#[doc = "backup_9 (rw) register accessor: Backup Register 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`backup_9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`backup_9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backup_9`]
module"]
pub type BACKUP_9 = crate::Reg<backup_9::BACKUP_9_SPEC>;
#[doc = "Backup Register 9"]
pub mod backup_9;
#[doc = "backup_10 (rw) register accessor: Backup Register 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`backup_10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`backup_10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backup_10`]
module"]
pub type BACKUP_10 = crate::Reg<backup_10::BACKUP_10_SPEC>;
#[doc = "Backup Register 10"]
pub mod backup_10;
#[doc = "backup_11 (rw) register accessor: Backup Register 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`backup_11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`backup_11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backup_11`]
module"]
pub type BACKUP_11 = crate::Reg<backup_11::BACKUP_11_SPEC>;
#[doc = "Backup Register 11"]
pub mod backup_11;
#[doc = "backup_12 (rw) register accessor: Backup Register 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`backup_12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`backup_12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backup_12`]
module"]
pub type BACKUP_12 = crate::Reg<backup_12::BACKUP_12_SPEC>;
#[doc = "Backup Register 12"]
pub mod backup_12;
#[doc = "backup_13 (rw) register accessor: Backup Register 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`backup_13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`backup_13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backup_13`]
module"]
pub type BACKUP_13 = crate::Reg<backup_13::BACKUP_13_SPEC>;
#[doc = "Backup Register 13"]
pub mod backup_13;
#[doc = "backup_14 (rw) register accessor: Backup Register 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`backup_14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`backup_14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backup_14`]
module"]
pub type BACKUP_14 = crate::Reg<backup_14::BACKUP_14_SPEC>;
#[doc = "Backup Register 14"]
pub mod backup_14;
#[doc = "backup_15 (rw) register accessor: Backup Register 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`backup_15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`backup_15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backup_15`]
module"]
pub type BACKUP_15 = crate::Reg<backup_15::BACKUP_15_SPEC>;
#[doc = "Backup Register 15"]
pub mod backup_15;
#[doc = "wdogcfg (rw) register accessor: wdog Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdogcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdogcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogcfg`]
module"]
pub type WDOGCFG = crate::Reg<wdogcfg::WDOGCFG_SPEC>;
#[doc = "wdog Configuration"]
pub mod wdogcfg;
#[doc = "wdogcount (rw) register accessor: Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdogcount::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdogcount::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogcount`]
module"]
pub type WDOGCOUNT = crate::Reg<wdogcount::WDOGCOUNT_SPEC>;
#[doc = "Counter Register"]
pub mod wdogcount;
#[doc = "wdogs (rw) register accessor: Scaled value of Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdogs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdogs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogs`]
module"]
pub type WDOGS = crate::Reg<wdogs::WDOGS_SPEC>;
#[doc = "Scaled value of Counter"]
pub mod wdogs;
#[doc = "wdogfeed (rw) register accessor: Feed register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdogfeed::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdogfeed::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogfeed`]
module"]
pub type WDOGFEED = crate::Reg<wdogfeed::WDOGFEED_SPEC>;
#[doc = "Feed register"]
pub mod wdogfeed;
#[doc = "wdogkey (rw) register accessor: Key Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdogkey::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdogkey::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogkey`]
module"]
pub type WDOGKEY = crate::Reg<wdogkey::WDOGKEY_SPEC>;
#[doc = "Key Register"]
pub mod wdogkey;
#[doc = "wdogcmp0 (rw) register accessor: Comparator 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdogcmp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdogcmp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogcmp0`]
module"]
pub type WDOGCMP0 = crate::Reg<wdogcmp0::WDOGCMP0_SPEC>;
#[doc = "Comparator 0"]
pub mod wdogcmp0;
#[doc = "rtccfg (rw) register accessor: rtc Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtccfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtccfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtccfg`]
module"]
pub type RTCCFG = crate::Reg<rtccfg::RTCCFG_SPEC>;
#[doc = "rtc Configuration"]
pub mod rtccfg;
#[doc = "rtccountlo (rw) register accessor: Low bits of Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtccountlo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtccountlo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtccountlo`]
module"]
pub type RTCCOUNTLO = crate::Reg<rtccountlo::RTCCOUNTLO_SPEC>;
#[doc = "Low bits of Counter"]
pub mod rtccountlo;
#[doc = "rtccounthi (rw) register accessor: High bits of Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtccounthi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtccounthi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtccounthi`]
module"]
pub type RTCCOUNTHI = crate::Reg<rtccounthi::RTCCOUNTHI_SPEC>;
#[doc = "High bits of Counter"]
pub mod rtccounthi;
#[doc = "rtcs (rw) register accessor: Scaled value of Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcs`]
module"]
pub type RTCS = crate::Reg<rtcs::RTCS_SPEC>;
#[doc = "Scaled value of Counter"]
pub mod rtcs;
#[doc = "rtccmp0 (rw) register accessor: Comparator 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtccmp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtccmp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtccmp0`]
module"]
pub type RTCCMP0 = crate::Reg<rtccmp0::RTCCMP0_SPEC>;
#[doc = "Comparator 0"]
pub mod rtccmp0;
#[doc = "pmuwakeupi0 (rw) register accessor: Wakeup program instruction 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmuwakeupi0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmuwakeupi0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmuwakeupi0`]
module"]
pub type PMUWAKEUPI0 = crate::Reg<pmuwakeupi0::PMUWAKEUPI0_SPEC>;
#[doc = "Wakeup program instruction 0"]
pub mod pmuwakeupi0;
#[doc = "pmuwakeupi1 (rw) register accessor: Wakeup program instruction 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmuwakeupi1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmuwakeupi1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmuwakeupi1`]
module"]
pub type PMUWAKEUPI1 = crate::Reg<pmuwakeupi1::PMUWAKEUPI1_SPEC>;
#[doc = "Wakeup program instruction 1"]
pub mod pmuwakeupi1;
#[doc = "pmuwakeupi2 (rw) register accessor: Wakeup program instruction 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmuwakeupi2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmuwakeupi2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmuwakeupi2`]
module"]
pub type PMUWAKEUPI2 = crate::Reg<pmuwakeupi2::PMUWAKEUPI2_SPEC>;
#[doc = "Wakeup program instruction 2"]
pub mod pmuwakeupi2;
#[doc = "pmuwakeupi3 (rw) register accessor: Wakeup program instruction 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmuwakeupi3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmuwakeupi3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmuwakeupi3`]
module"]
pub type PMUWAKEUPI3 = crate::Reg<pmuwakeupi3::PMUWAKEUPI3_SPEC>;
#[doc = "Wakeup program instruction 3"]
pub mod pmuwakeupi3;
#[doc = "pmuwakeupi4 (rw) register accessor: Wakeup program instruction 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmuwakeupi4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmuwakeupi4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmuwakeupi4`]
module"]
pub type PMUWAKEUPI4 = crate::Reg<pmuwakeupi4::PMUWAKEUPI4_SPEC>;
#[doc = "Wakeup program instruction 4"]
pub mod pmuwakeupi4;
#[doc = "pmuwakeupi5 (rw) register accessor: Wakeup program instruction 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmuwakeupi5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmuwakeupi5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmuwakeupi5`]
module"]
pub type PMUWAKEUPI5 = crate::Reg<pmuwakeupi5::PMUWAKEUPI5_SPEC>;
#[doc = "Wakeup program instruction 5"]
pub mod pmuwakeupi5;
#[doc = "pmuwakeupi6 (rw) register accessor: Wakeup program instruction 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmuwakeupi6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmuwakeupi6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmuwakeupi6`]
module"]
pub type PMUWAKEUPI6 = crate::Reg<pmuwakeupi6::PMUWAKEUPI6_SPEC>;
#[doc = "Wakeup program instruction 6"]
pub mod pmuwakeupi6;
#[doc = "pmuwakeupi7 (rw) register accessor: Wakeup program instruction 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmuwakeupi7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmuwakeupi7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmuwakeupi7`]
module"]
pub type PMUWAKEUPI7 = crate::Reg<pmuwakeupi7::PMUWAKEUPI7_SPEC>;
#[doc = "Wakeup program instruction 7"]
pub mod pmuwakeupi7;
#[doc = "pmusleepi0 (rw) register accessor: Sleep program instruction 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmusleepi0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmusleepi0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmusleepi0`]
module"]
pub type PMUSLEEPI0 = crate::Reg<pmusleepi0::PMUSLEEPI0_SPEC>;
#[doc = "Sleep program instruction 0"]
pub mod pmusleepi0;
#[doc = "pmusleepi1 (rw) register accessor: Sleep program instruction 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmusleepi1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmusleepi1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmusleepi1`]
module"]
pub type PMUSLEEPI1 = crate::Reg<pmusleepi1::PMUSLEEPI1_SPEC>;
#[doc = "Sleep program instruction 1"]
pub mod pmusleepi1;
#[doc = "pmusleepi2 (rw) register accessor: Sleep program instruction 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmusleepi2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmusleepi2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmusleepi2`]
module"]
pub type PMUSLEEPI2 = crate::Reg<pmusleepi2::PMUSLEEPI2_SPEC>;
#[doc = "Sleep program instruction 2"]
pub mod pmusleepi2;
#[doc = "pmusleepi3 (rw) register accessor: Sleep program instruction 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmusleepi3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmusleepi3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmusleepi3`]
module"]
pub type PMUSLEEPI3 = crate::Reg<pmusleepi3::PMUSLEEPI3_SPEC>;
#[doc = "Sleep program instruction 3"]
pub mod pmusleepi3;
#[doc = "pmusleepi4 (rw) register accessor: Sleep program instruction 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmusleepi4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmusleepi4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmusleepi4`]
module"]
pub type PMUSLEEPI4 = crate::Reg<pmusleepi4::PMUSLEEPI4_SPEC>;
#[doc = "Sleep program instruction 4"]
pub mod pmusleepi4;
#[doc = "pmusleepi5 (rw) register accessor: Sleep program instruction 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmusleepi5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmusleepi5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmusleepi5`]
module"]
pub type PMUSLEEPI5 = crate::Reg<pmusleepi5::PMUSLEEPI5_SPEC>;
#[doc = "Sleep program instruction 5"]
pub mod pmusleepi5;
#[doc = "pmusleepi6 (rw) register accessor: Sleep program instruction 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmusleepi6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmusleepi6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmusleepi6`]
module"]
pub type PMUSLEEPI6 = crate::Reg<pmusleepi6::PMUSLEEPI6_SPEC>;
#[doc = "Sleep program instruction 6"]
pub mod pmusleepi6;
#[doc = "pmusleepi7 (rw) register accessor: Sleep program instruction 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmusleepi7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmusleepi7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmusleepi7`]
module"]
pub type PMUSLEEPI7 = crate::Reg<pmusleepi7::PMUSLEEPI7_SPEC>;
#[doc = "Sleep program instruction 7"]
pub mod pmusleepi7;
#[doc = "pmuie (rw) register accessor: PMU Interrupt Enables\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmuie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmuie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmuie`]
module"]
pub type PMUIE = crate::Reg<pmuie::PMUIE_SPEC>;
#[doc = "PMU Interrupt Enables"]
pub mod pmuie;
#[doc = "pmucause (rw) register accessor: PMU Wakeup Cause\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmucause::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmucause::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmucause`]
module"]
pub type PMUCAUSE = crate::Reg<pmucause::PMUCAUSE_SPEC>;
#[doc = "PMU Wakeup Cause"]
pub mod pmucause;
#[doc = "pmusleep (rw) register accessor: Initiate PMU Sleep Sequence\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmusleep::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmusleep::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmusleep`]
module"]
pub type PMUSLEEP = crate::Reg<pmusleep::PMUSLEEP_SPEC>;
#[doc = "Initiate PMU Sleep Sequence"]
pub mod pmusleep;
#[doc = "pmukey (rw) register accessor: PMU Key. Reads as 1 when PMU is unlocked\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmukey::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmukey::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmukey`]
module"]
pub type PMUKEY = crate::Reg<pmukey::PMUKEY_SPEC>;
#[doc = "PMU Key. Reads as 1 when PMU is unlocked"]
pub mod pmukey;
#[doc = "aoncfg (rw) register accessor: AON Block Configuration Information\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aoncfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aoncfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aoncfg`]
module"]
pub type AONCFG = crate::Reg<aoncfg::AONCFG_SPEC>;
#[doc = "AON Block Configuration Information"]
pub mod aoncfg;
#[doc = "lfrosccfg (rw) register accessor: Ring Oscillator Configuration and Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfrosccfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfrosccfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfrosccfg`]
module"]
pub type LFROSCCFG = crate::Reg<lfrosccfg::LFROSCCFG_SPEC>;
#[doc = "Ring Oscillator Configuration and Status"]
pub mod lfrosccfg;
#[doc = "lfclkmux (rw) register accessor: Low-Frequency Clock Mux Control and Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfclkmux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfclkmux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfclkmux`]
module"]
pub type LFCLKMUX = crate::Reg<lfclkmux::LFCLKMUX_SPEC>;
#[doc = "Low-Frequency Clock Mux Control and Status"]
pub mod lfclkmux;
