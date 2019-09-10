//! This module contains the necessary functions to convert an Amethyst f32 object to generic physics object.

pub mod vec_conversor {
    //! This module contains the necessary functions to convert an Amethyst f32 `Vector3` to generic physics `Vector3`.

    use amethyst_core::math::Vector3;

    use crate::PtReal;

    /// Used to convert an amethyst `Vector3` `Quaternion` to the physics `Vector3`.
    #[allow(clippy::wrong_self_convention)]
    pub fn to_physics<N>(v: &Vector3<f32>) -> Vector3<N>
    where
        N: PtReal,
    {
        Vector3::new(v.x.into(), v.y.into(), v.z.into())
    }

    /// Used to convert a physics `Vector3` to the amethyst `Transform` `Vector3`.
    pub fn from_physics<N>(v: &Vector3<N>) -> Vector3<f32>
    where
        N: PtReal,
    {
        Vector3::new(v.x.into(), v.y.into(), v.z.into())
    }
}

pub mod quat_conversor {
    //! This module contains the necessary functions to convert an Amethyst f32 `Quaternion` to generic physics `Quaternion`.
    use amethyst_core::math::{Quaternion, Vector4};

    use crate::PtReal;

    /// Used to convert an amethyst `f32` `Quaternion` to a generic `Quaternion`.
    #[allow(clippy::wrong_self_convention)]
    pub fn to_physics<N>(r: &Quaternion<f32>) -> Quaternion<N>
    where
        N: PtReal,
    {
        Quaternion::from(Vector4::new(r.i.into(), r.j.into(), r.k.into(), r.w.into()))
    }

    /// Used to convert a generic `Quaternion` to the `f32` `Quaternion`.
    pub fn from_physics<N>(r: &Quaternion<N>) -> Quaternion<f32>
    where
        N: PtReal,
    {
        Quaternion::from(Vector4::new(
            N::into(r.i),
            N::into(r.j),
            N::into(r.k),
            N::into(r.w),
        ))
    }
}

pub mod transf_conversor {
    //! This module contains the necessary functions to convert an Amethyst f32 `Isometry` to generic physics `Isometry`.
    use amethyst_core::math::{
        Isometry3, Translation3, UnitQuaternion,
    };

    use crate::{
        conversors::{quat_conversor, vec_conversor},
        PtReal,
    };

    /// Used to convert an amethyst `f32` `Isometry` to the generic `Isometry`.
    #[allow(clippy::wrong_self_convention)]
    pub fn to_physics<N>(t: &Isometry3<f32>) -> Isometry3<N>
    where
        N: PtReal,
    {
        Isometry3::from_parts(
            Translation3::from(vec_conversor::to_physics(&t.translation.vector)),
            UnitQuaternion::new_normalize(quat_conversor::to_physics(&t.rotation)),
        )
    }

    /// Used to convert a generic `Isometry` to the amethyst `f32` `Isometry`.
    pub fn from_physics<N>(t: &Isometry3<N>) -> Isometry3<f32>
    where
        N: PtReal,
    {
        Isometry3::from_parts(
            Translation3::from(vec_conversor::from_physics(&t.translation.vector)),
            UnitQuaternion::new_normalize(quat_conversor::from_physics(&t.rotation)),
        )
    }
}
