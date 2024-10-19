use std::sync::Arc;
use std::default::Default;
use vulkano::buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage};
use vulkano::device::{Device, DeviceCreateInfo, QueueCreateInfo, QueueFlags};
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};
use vulkano::VulkanLibrary;

#[derive(BufferContents, Debug)]
#[repr(C)]
struct Thing {
    number: u32,
}

fn main() {
    let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
    let instance = Instance::new(library, InstanceCreateInfo::default()).expect("failed to create instance");
    let physical_device = instance
        .enumerate_physical_devices()
        .expect("could not enumerate devices")
        .nth(1)
        .expect("no Vulkan supported devices found");
    let queue_family_index = physical_device
        .queue_family_properties()
        .iter()
        .position(|properties| {
            properties.queue_flags.contains(QueueFlags::GRAPHICS)
        })
        .expect("could not find a graphical queue family") as u32;
    let (device, mut _queues) = Device::new(
        physical_device,
        DeviceCreateInfo {
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            ..Default::default()
        },
    ).expect("failed to create logical device");
    let allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

    let iter = (0..128).map(|i| Thing { number: i * 2 });
    let buffer = Buffer::from_iter(
        allocator.clone(),
        BufferCreateInfo {
            usage: BufferUsage::UNIFORM_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
        iter,
    ).unwrap();
    let content = buffer.read().unwrap();
    let _ = dbg!(content.iter());
    drop(content);

    let mut content = buffer.write().unwrap();
    content[1].number += 1;
    dbg!(content);
}