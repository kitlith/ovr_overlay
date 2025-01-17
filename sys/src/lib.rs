#![allow(unused_unsafe, clippy::missing_safety_doc, clippy::too_many_arguments)] // suppress some warnings generated by autocxx

use autocxx::prelude::*; // use all the main autocxx functions

include_cpp! {
    #include "openvr.h"

    // TrackedDeviceIndex_t constants
    generate_pod!("vr::TrackedDeviceIndex_t")
    generate!("vr::k_unTrackedDeviceIndex_Hmd")
    generate!("vr::k_unMaxTrackedDeviceCount")
    generate!("vr::k_unTrackedDeviceIndexOther")
    generate!("vr::k_unTrackedDeviceIndexInvalid")

    generate!("vr::VR_Init")
    generate_pod!("vr::EVRApplicationType")
    generate!("vr::VR_Shutdown")

    generate!("vr::IVRSystem")
    generate!("vr::VRSystem")
    generate!("vr::k_unMaxPropertyStringSize")

    generate!("vr::IVROverlay")
    generate!("vr::VROverlay")
    generate_pod!("vr::EVROverlayError")
    generate_pod!("vr::VROverlayHandle_t")

    generate!("vr::IVRChaperoneSetup")
    generate!("vr::VRChaperoneSetup")

    generate!("vr::VR_GetVRInitErrorAsSymbol")
    generate_pod!("vr::EVRInitError")

    generate_pod!("vr::ETrackingUniverseOrigin")
    generate!("vr::HmdMatrix34_t")

    generate_pod!("vr::VRTextureBounds_t")

    // input
    generate!("vr::IVRInput")
    generate!("vr::VRInput")
    generate_pod!("vr::EVRInputError")
    generate_pod!("vr::VRActionSetHandle_t")
    generate_pod!("vr::VRActionHandle_t")
    generate_pod!("vr::VRInputValueHandle_t")
    generate_pod!("vr::VRActiveActionSet_t")
    generate_pod!("vr::InputDigitalActionData_t")
    generate_pod!("vr::TrackedDevicePose_t")
    generate_pod!("vr::InputPoseActionData_t")
    generate_pod!("vr::InputOriginInfo_t")

    // applications
    generate!("vr::IVRApplications")
    generate!("vr::VRApplications")
    generate_pod!("vr::EVRApplicationError")
}

//pub use ffi::vr::*;
pub use ffi::vr::*;
pub use ffi::{make_string, ToCppString};
