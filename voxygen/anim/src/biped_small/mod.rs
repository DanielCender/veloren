pub mod idle;
pub mod run;

// Reexports
pub use self::{idle::IdleAnimation, run::RunAnimation};

use super::{make_bone, vek::*, FigureBoneData, Skeleton};
use common::comp::{self};
use core::convert::TryFrom;

pub type Body = comp::biped_small::Body;

skeleton_impls!(struct BipedSmallSkeleton {
    + head,
    + chest,
    + shorts,
    + tail,
    + main,
    + hand_l,
    + hand_r,
    + foot_l,
    + foot_r,
});

impl Skeleton for BipedSmallSkeleton {
    type Attr = SkeletonAttr;
    type Body = Body;

    const BONE_COUNT: usize = 9;
    #[cfg(feature = "use-dyn-lib")]
    const COMPUTE_FN: &'static [u8] = b"biped_small_compute_mats\0";

    #[cfg_attr(feature = "be-dyn-lib", export_name = "biped_small_compute_mats")]
    fn compute_matrices_inner(
        &self,
        base_mat: Mat4<f32>,
        buf: &mut [FigureBoneData; super::MAX_BONE_COUNT],
    ) -> Vec3<f32> {
        let chest_mat = base_mat * Mat4::<f32>::from(self.chest);
        let shorts_mat = chest_mat * Mat4::<f32>::from(self.shorts);

        *(<&mut [_; Self::BONE_COUNT]>::try_from(&mut buf[0..Self::BONE_COUNT]).unwrap()) = [
            make_bone(chest_mat * Mat4::<f32>::from(self.head)),
            make_bone(chest_mat),
            make_bone(shorts_mat),
            make_bone(shorts_mat * Mat4::<f32>::from(self.tail)),
            make_bone(chest_mat * Mat4::<f32>::from(self.main)),
            make_bone(chest_mat * Mat4::<f32>::from(self.hand_l)),
            make_bone(chest_mat * Mat4::<f32>::from(self.hand_r)),
            make_bone(base_mat * Mat4::<f32>::from(self.foot_l)),
            make_bone(base_mat * Mat4::<f32>::from(self.foot_r)),
        ];
        Vec3::default()
    }
}

pub struct SkeletonAttr {
    head: (f32, f32),
    chest: (f32, f32),
    shorts: (f32, f32),
    tail: (f32, f32),
    hand: (f32, f32, f32),
    foot: (f32, f32, f32),
}

impl<'a> std::convert::TryFrom<&'a comp::Body> for SkeletonAttr {
    type Error = ();

    fn try_from(body: &'a comp::Body) -> Result<Self, Self::Error> {
        match body {
            comp::Body::BipedSmall(body) => Ok(SkeletonAttr::from(body)),
            _ => Err(()),
        }
    }
}

impl Default for SkeletonAttr {
    fn default() -> Self {
        Self {
            head: (0.0, 0.0),
            chest: (0.0, 0.0),
            shorts: (0.0, 0.0),
            tail: (0.0, 0.0),
            hand: (0.0, 0.0, 0.0),
            foot: (0.0, 0.0, 0.0),
        }
    }
}

impl<'a> From<&'a Body> for SkeletonAttr {
    fn from(body: &'a Body) -> Self {
        use comp::biped_small::Species::*;
        Self {
            head: match (body.species, body.body_type) {
                (Gnome, _) => (-1.0, 9.0),
                (Sahagin, _) => (7.0, -3.5),
                (Adlet, _) => (0.0, 7.0),
            },
            chest: match (body.species, body.body_type) {
                (Gnome, _) => (0.0, 9.0),
                (Sahagin, _) => (0.0, 15.0),
                (Adlet, _) => (0.0, 11.0),
            },
            shorts: match (body.species, body.body_type) {
                (Gnome, _) => (0.0, -3.0),
                (Sahagin, _) => (0.5, -7.0),
                (Adlet, _) => (0.0, -3.0),
            },
            tail: match (body.species, body.body_type) {
                (Gnome, _) => (0.0, 0.0),
                (Sahagin, _) => (-2.5, -2.0),
                (Adlet, _) => (-4.5, -2.0),
            },
            hand: match (body.species, body.body_type) {
                (Gnome, _) => (6.0, 0.5, -1.0),
                (Sahagin, _) => (9.5, 3.5, -2.0),
                (Adlet, _) => (6.0, -0.5, 0.0),
            },
            foot: match (body.species, body.body_type) {
                (Gnome, _) => (3.0, 0.0, 4.0),
                (Sahagin, _) => (3.0, 1.0, 8.0),
                (Adlet, _) => (3.0, 0.5, 7.0),
            },
        }
    }
}
