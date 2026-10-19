//! The actor classes and the virtual-call dispatch.

mod cats;
mod class0;
mod class1;
mod class10;
mod class2;
mod class3;
mod class4;
mod class5;
mod class7;
mod class8;
mod class9;

use super::Pt00;

impl Pt00 {
    /// Runs the virtual function at `function` on `this`.
    pub(crate) fn dispatch(&mut self, function: u32, this: u32, a: i32, b: i32) -> i32 {
        match function {
            // Shared slots.
            0x1000_4A70 => self.f_10004a70(this, b),
            0x1000_4AB0 => self.f_10004ab0(this),
            0x1000_4DF0 => self.f_10004df0(this),
            0x1000_5010 => self.f_10005010(this),
            0x1000_5050 => self.f_10005050(this, a, b),
            0x1000_4EF0 => self.f_10004ef0(this),
            0x1000_1980 => {}
            // Class 0.
            0x1000_5950 => self.f_10005950(this),
            0x1000_58B0 => self.f_100058b0(this),
            // Class 6.
            0x1000_CB40 => self.f_1000cb40(this),
            0x1000_CB30 => self.f_1000cb30(this),
            _ => self.dispatch_classes(function, this, a, b),
        }
        0
    }

    /// The class-specific virtual functions.
    fn dispatch_classes(&mut self, function: u32, this: u32, _a: i32, _b: i32) {
        match function {
            0x1000_7070 => self.f_10007070(this),
            0x1000_7100 => self.f_10007100(this),
            0x1000_76F0 => self.f_100076f0(this),
            0x1000_8AD0 => self.f_10008ad0(this),
            0x1000_8B10 => self.f_10008b10(this),
            0x1000_8A80 => self.f_10008a80(this),
            0x1000_95B0 => self.f_100095b0(this),
            0x1000_9630 => self.f_10009630(this),
            0x1000_9B60 => self.f_10009b60(this),
            0x1000_AE40 => self.f_1000ae40(this),
            0x1000_B4A0 => self.f_1000b4a0(this),
            0x1000_C2B0 => self.f_1000c2b0(this),
            0x1000_C300 => self.f_1000c300(this),
            0x1000_C490 => self.f_1000c490(this),
            0x1000_CBC0 => self.f_1000cbc0(this),
            0x1000_CBE0 => self.f_1000cbe0(this),
            0x1000_CC80 => self.f_1000cc80(this),
            0x1000_D480 => self.f_1000d480(this),
            0x1000_E7F0 => self.f_1000e7f0(this),
            0x1000_E840 => self.f_1000e840(this),
            0x1000_EE90 => self.f_1000ee90(this),
            0x1000_FFC0 => self.f_1000ffc0(this),
            0x1000_FEE0 => self.f_1000fee0(this),
            0x1001_1F10 => self.f_10011f10(this),
            0x1001_1F70 => self.f_10011f70(this),
            0x1001_1F80 => self.f_10011f80(this),
            0x1001_1F30 => self.f_10011f30(this),
            0x1001_22F0 => self.f_100122f0(this),
            0x1001_2320 => self.f_10012320(this),
            0x1001_26B0 => self.f_100126b0(this),
            _ => {}
        }
    }

    /// `sub_1000CAE0`: class 6's constructor (the vtable is already set).
    pub(crate) fn f_1000cae0(&mut self, _this: u32) {}

    /// `sub_1000CB40` (class 6 slot 7, call 100): back to state 1.
    pub(crate) fn f_1000cb40(&mut self, this: u32) {
        self.vcall(this, 20, 1, 0);
    }

    /// `sub_1000CB30` (class 6 slot 8): in state 1, take the position and
    /// pattern the script put in `F[610..=613]`.
    pub(crate) fn f_1000cb30(&mut self, this: u32) {
        if self.r(this, 16) == 1 {
            self.sr(this, 40, 2 * self.g(this, 2440));
            self.sr(this, 44, self.g(this, 2444));
            self.sr(this, 48, self.g(this, 2448));
            self.sr(this, 80, self.g(this, 2452));
            self.sr(this, 84, 100);
            self.sr(this, 88, 1);
            let pattern = self.r(this, 80);
            self.sr(this, 80, self.i(0x1001_EEE4 + 4 * pattern as u32));
        }
    }
}
