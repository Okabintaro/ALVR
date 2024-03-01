use crate::XrContext;
use alvr_common::anyhow::Result;
use alvr_graphics::{GraphicsContext, VulkanBackend, TARGET_VULKAN_VERSION};
use std::{ffi::CStr, mem};

pub fn create_graphics_context(xr_context: &XrContext) -> Result<GraphicsContext<VulkanBackend>> {
    let entry = alvr_graphics::create_entry_vk()?;

    let gfkd = xr_context
        .instance
        .graphics_requirements(xr_context.system)?;

    let raw_instance = unsafe {
        let raw_instance_ptr = xr_context.instance.create_vulkan_instance(
            xr_context.system,
            alvr_graphics::get_instance_proc_addr_vk(&entry),
            &alvr_graphics::get_instance_create_info_vk(&entry) as *const _ as *const _,
        )??;
        alvr_graphics::vk_instance_from_ptr(&entry, raw_instance_ptr)
    };

    todo!()
}

pub fn create_swapchain(
    session: &xr::Session<xr::OpenGlEs>,
    resolution: UVec2,
    foveation: Option<&xr::FoveationProfileFB>,
) -> xr::Swapchain<xr::OpenGlEs> {
    let swapchain_info = xr::SwapchainCreateInfo {
        create_flags: xr::SwapchainCreateFlags::EMPTY,
        usage_flags: xr::SwapchainUsageFlags::COLOR_ATTACHMENT | xr::SwapchainUsageFlags::SAMPLED,
        format: glow::SRGB8_ALPHA8,
        sample_count: 1,
        width: resolution.x,
        height: resolution.y,
        face_count: 1,
        array_size: 1,
        mip_count: 1,
    };

    if let Some(foveation) = foveation {
        let swapchain = session
            .create_swapchain_with_foveation(
                &swapchain_info,
                xr::SwapchainCreateFoveationFlagsFB::SCALED_BIN,
            )
            .unwrap();

        swapchain.update_foveation(foveation).unwrap();

        swapchain
    } else {
        session.create_swapchain(&swapchain_info).unwrap()
    }
}
