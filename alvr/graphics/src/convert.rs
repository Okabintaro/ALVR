use crate::GraphicsContext;
use alvr_common::anyhow::Result;
use alvr_common::glam::UVec2;
use ash::vk::{self, Handle};
use std::{
    ffi::{c_char, CStr},
    mem,
    num::NonZeroU32,
    os::raw::c_void,
    ptr,
    rc::Rc,
    sync::Arc,
};
use wgpu::{
    hal, Device, Instance, InstanceDescriptor, InstanceFlags, Texture, TextureDescriptor,
    TextureUsages,
};
use wgpu_core::api;

pub const TARGET_VULKAN_VERSION: u32 = vk::make_api_version(0, 1, 1, 0);

pub fn create_entry_vk() -> Result<ash::Entry> {
    Ok(unsafe { ash::Entry::load()? })
}

pub fn get_instance_proc_addr_vk(
    entry: &ash::Entry,
) -> unsafe extern "system" fn(*const c_void, *const c_char) -> Option<unsafe extern "system" fn()>
{
    unsafe { mem::transmute(entry.static_fn().get_instance_proc_addr) }
}

pub fn get_instance_create_info_vk(entry: &ash::Entry) -> Result<vk::InstanceCreateInfo> {
    let mut flags = InstanceFlags::empty();
    if cfg!(debug_assertions) {
        flags |= InstanceFlags::VALIDATION;
        flags |= InstanceFlags::DEBUG;
    }

    let exts_ptrs = <hal::api::Vulkan as hal::Api>::Instance::desired_extensions(
        entry,
        TARGET_VULKAN_VERSION,
        flags,
    )?
    .iter()
    .map(|x| x.as_ptr())
    .collect::<Vec<_>>();

    // todo: contribute better way to get layers from wgpu
    let layers_ptrs = entry
        .enumerate_instance_layer_properties()
        .unwrap()
        .iter()
        .filter_map(|props| {
            let name = unsafe { CStr::from_ptr(props.layer_name.as_ptr()) };
            if name.to_str().unwrap() == "VK_LAYER_KHRONOS_validation" {
                Some(props.layer_name.as_ptr())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    Ok(vk::InstanceCreateInfo::builder()
        .application_info(&vk::ApplicationInfo::builder().api_version(TARGET_VULKAN_VERSION))
        .enabled_extension_names(&exts_ptrs)
        .enabled_layer_names(&layers_ptrs)
        .build())
}

pub fn vk_instance_from_ptr(entry: &ash::Entry, instance: *const c_void) -> ash::Instance {
    unsafe { ash::Instance::load(entry.static_fn(), vk::Instance::from_raw(instance as _)) }
}

pub fn vk_physical_device_from_ptr(physical_device: *const c_void) -> vk::PhysicalDevice {
    vk::PhysicalDevice::from_raw(physical_device as _)
}

// #[derive(Clone)]
// pub struct GlBackend {
//     pub egl_display: egl::Display,
//     pub egl_config: egl::Config,
//     pub egl_context: egl::Context,
//     pub gl_context: Rc<gl::Context>,
//     create_image: CreateImageFn,
//     destroy_image: DestroyImageFn,
//     get_native_client_buffer: GetNativeClientBufferFn,
//     image_target_texture_2d: ImageTargetTexture2DFn,
// }

#[derive(Clone)]
pub struct VulkanBackend {}

pub struct GraphicsContextInitHandles {
    // pub entry: ash
}

impl GraphicsContext<VulkanBackend> {
    // fn get_fn_ptr(adapter: &wgpu::Adapter, name: &str) -> *const c_void {
    //     unsafe {
    //         adapter.as_hal::<api::Gles, _, _>(|a| {
    //             let egl = a.unwrap().adapter_context().egl_instance().unwrap();
    //             egl.get_proc_address(name).unwrap() as *const c_void
    //         })
    //     }
    // }

    pub fn new_vulkan() -> Self {
        // let flags = if cfg!(debug_assertions) {
        //     InstanceFlags::DEBUG | InstanceFlags::VALIDATION
        // } else {
        //     InstanceFlags::empty()
        // };

        // let instance = Instance::new(InstanceDescriptor {
        //     backends: wgpu::Backends::GL,
        //     flags,
        //     dx12_shader_compiler: Default::default(),
        //     gles_minor_version: Default::default(),
        // });
        // let adapter = instance.enumerate_adapters(wgpu::Backends::GL).remove(0);
        // let (device, queue) =
        //     pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default(), None))
        //         .unwrap();

        // let raw_instance = unsafe { instance.as_hal::<api::Gles>() }.unwrap();

        // let egl_display = raw_instance.raw_display();
        // let egl_config = raw_instance.egl_config();

        // let (egl_context, gl_context) = unsafe {
        //     adapter.as_hal::<api::Gles, _, _>(|raw_adapter| {
        //         let adapter_context = raw_adapter.unwrap().adapter_context();
        //         let egl_context = egl::Context::from_ptr(adapter_context.raw_context());
        //         let gl_context = gl::Context::from_loader_function(|s| {
        //             adapter_context
        //                 .egl_instance()
        //                 .unwrap()
        //                 .get_proc_address(s)
        //                 .unwrap() as *const _
        //         });

        //         (egl_context, Rc::new(gl_context))
        //     })
        // };

        // Self {
        //     instance: Arc::new(instance),
        //     adapter: Arc::new(adapter),
        //     device: Arc::new(device),
        //     queue: Arc::new(queue),
        //     backend_handles: VulkanBackend {
        //         // egl_display,
        //         // egl_config,
        //         // egl_context,
        //         // gl_context,
        //         // create_image,
        //         // destroy_image,
        //         // get_native_client_buffer,
        //         // image_target_texture_2d,
        //     },
        // }

        todo!()
    }

    // # Safety
    // `buffer` must be a valid AHardwareBuffer.
    // `texture` must be a valid GL texture.
    // pub unsafe fn bind_ahardwarebuffer_to_gl_ext_texture(
    //     &self,
    //     buffer: *const c_void,
    //     texture: gl::Texture,
    // ) -> egl::EGLImage {
    //     let client_buffer = (self.backend_handles.get_native_client_buffer)(buffer);

    //     let image = (self.backend_handles.create_image)(
    //         self.backend_handles.egl_display.as_ptr(),
    //         egl::NO_CONTEXT,
    //         EGL_NATIVE_BUFFER_ANDROID,
    //         client_buffer,
    //         ptr::null(),
    //     );

    //     self.backend_handles
    //         .gl_context
    //         .bind_texture(GL_TEXTURE_EXTERNAL_OES, Some(texture));

    //     (self.backend_handles.image_target_texture_2d)(GL_TEXTURE_EXTERNAL_OES, image);

    //     image
    // }

    // # Safety
    // `image` must be a valid EGLImage.
    // pub unsafe fn destroy_image(&self, image: egl::EGLImage) {
    //     (self.backend_handles.destroy_image)(self.backend_handles.egl_display.as_ptr(), image);
    // }
}

// This is used to convert OpenXR swapchains to wgpu
// textures should be arrays of depth 2, RGBA8UnormSrgb
// pub fn create_texture_from_gles(device: &Device, texture: u32, resolution: UVec2) -> Texture {
//     unsafe {
//         let hal_texture = device
//             .as_hal::<api::Gles, _, _>(|device| {
//                 device.unwrap().texture_from_raw_renderbuffer(
//                     NonZeroU32::new(texture).unwrap(),
//                     &hal::TextureDescriptor {
//                         label: None,
//                         size: wgpu::Extent3d {
//                             width: resolution.x,
//                             height: resolution.y,
//                             depth_or_array_layers: 2,
//                         },
//                         mip_level_count: 1,
//                         sample_count: 1,
//                         dimension: wgpu::TextureDimension::D2,
//                         format: wgpu::TextureFormat::Rgba8UnormSrgb,
//                         usage: hal::TextureUses::COLOR_TARGET,
//                         memory_flags: hal::MemoryFlags::empty(),
//                         view_formats: vec![],
//                     },
//                     Some(Box::new(())),
//                 )
//             })
//             .unwrap();

//         device.create_texture_from_hal::<api::Gles>(
//             hal_texture,
//             &TextureDescriptor {
//                 label: None,
//                 size: wgpu::Extent3d {
//                     width: resolution.x,
//                     height: resolution.y,
//                     depth_or_array_layers: 2,
//                 },
//                 mip_level_count: 1,
//                 sample_count: 1,
//                 dimension: wgpu::TextureDimension::D2,
//                 format: wgpu::TextureFormat::Rgba8UnormSrgb,
//                 usage: TextureUsages::RENDER_ATTACHMENT,
//                 view_formats: &[],
//             },
//         )
//     }
// }

// pub fn create_gl_swapchain(device: &Device, textures: Vec<u32>, resolution: UVec2) -> Vec<Texture> {
//     textures
//         .into_iter()
//         .map(|texture| create_texture_from_gles(device, texture, resolution))
//         .collect()
// }
