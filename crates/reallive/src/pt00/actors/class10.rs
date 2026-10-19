//! Actor 10 (vtable `0x1001C11C`): follows actor 1 around.

use super::super::{Pt00, ftol};

impl Pt00 {
    /// `sub_10011F10` (slot 2).
    pub(crate) fn f_10011f10(&mut self, this: u32) {
        self.f_10004ab0(this);
        self.sw(this, 314, 0);
        self.sw(this, 313, 0);
        self.sw(this, 315, 0);
    }

    /// `sub_10011F70` (slot 6).
    pub(crate) fn f_10011f70(&mut self, this: u32) {
        self.sr(this, 24, 0);
    }

    /// `sub_10011F80` (slot 7).
    pub(crate) fn f_10011f80(&mut self, this: u32) {
        match self.r(this, 12) {
            3 => {
                self.vcall(this, 20, 1, 0);
            }
            2 => {
                self.vcall(this, 20, 2, 0);
            }
            _ => {}
        }
    }

    /// `sub_10011F30` (slot 8).
    pub(crate) fn f_10011f30(&mut self, this: u32) {
        match self.r(this, 16) {
            1 => self.f_10011fb0(this),
            2 => self.f_10012140(this),
            3 => {
                self.walk_field(this, 1, 15.0);
                self.sr(this, 112, self.r(this, 112) + 1);
                self.sr(this, 20, 0);
            }
            4 => {
                // (The DLL sets the target to the current position.)
                let here = [self.r(this, 40), self.r(this, 44), self.r(this, 48)];
                self.set_target(this, here);
                self.walk_field(this, 1, 15.0);
                self.sr(this, 112, self.r(this, 112) + 1);
                if self.at_target(this) {
                    self.vcall(this, 16, 0, 0);
                }
            }
            _ => {}
        }
    }

    /// An occasional bark when on camera.
    fn bark(&mut self, this: u32) {
        if self.f_10004770(self.r(this, 40), self.r(this, 44), self.r(this, 48))
            && self.r(this, 124) % 30 == 0
            && self.rand() % 6 == 0
        {
            self.f_10014040(29, 255);
        }
    }

    /// `sub_10011FB0` (state 1): circles actor 1 in mode `F[1044]` 2,
    /// otherwise trails 50 frames behind it.
    fn f_10011fb0(&mut self, this: u32) {
        let leader = self.actor(1);
        if self.g(this, 4176) == 2 {
            let n = self.w(this, 313);
            if n == 0 {
                let start = f64::from(self.r(this, 124) % 360) * 0.01745329251944444 - 1.2566370614;
                self.sd(this + 1256, start);
            }
            let a = f64::from(n % 360) * 0.03490658503888889 + self.d(this + 1256);
            self.sr(
                this,
                68,
                ftol(a.cos() * 500.0 + f64::from(self.r(leader, 40))),
            );
            self.sr(this, 72, 0);
            self.sr(
                this,
                76,
                ftol(a.sin() * 500.0 + f64::from(self.r(leader, 48))),
            );
            self.sw(this, 313, n + 1);
        } else {
            self.sw(this, 313, 0);
            let p = self.f_10005510(leader, 50);
            self.set_target(this, p);
        }
        self.walk_field(this, 1, 15.0);
        self.sr(this, 112, self.r(this, 112) + 1);
        self.bark(this);
    }

    /// `sub_10012140` (state 2): strolling round home.
    fn f_10012140(&mut self, this: u32) {
        let a = f64::from(self.r(this, 124) % 360) * 0.01745329251944444;
        self.sr(
            this,
            68,
            ftol(a.cos() * 1000.0 + f64::from(self.r(this, 28))),
        );
        self.sr(this, 72, 0);
        self.sr(
            this,
            76,
            ftol(a.sin() * 1000.0 + f64::from(self.r(this, 36))),
        );
        self.walk_field(this, 1, 15.0);
        self.sr(this, 112, self.r(this, 112) + 1);
        self.bark(this);
    }
}
