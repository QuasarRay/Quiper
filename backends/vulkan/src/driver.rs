//! Synchronous, dedicated-allocation Vulkan execution with owned object lifetimes.
use ash::{Entry, vk};
use kuiper_contracts::{Artifact, Diagnostic, Execution, Invocation, Result, parameter_words};
use std::cell::Cell;
use std::ffi::CString;
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::rc::Rc;

fn fail(code: &str, message: impl Into<String>) -> Diagnostic {
    Diagnostic::new("vulkan", code, message)
}
fn vk_error(stage: &str, error: vk::Result) -> Diagnostic {
    fail(stage, format!("Vulkan returned {error:?}"))
}

fn admit_features(api: u32, robust: bool, model: bool, scope: bool) -> Result<()> {
    if api < vk::API_VERSION_1_2 || !robust || !model || !scope {
        return Err(fail(
            "device-features",
            "Vulkan 1.2, robustBufferAccess, vulkanMemoryModel and vulkanMemoryModelDeviceScope must all be supported and enabled",
        ));
    }
    Ok(())
}
fn admit_limits(
    limits: &vk::PhysicalDeviceLimits,
    artifact: &Artifact,
    invocation: &Invocation,
) -> Result<()> {
    let count = artifact.reflection.bindings.len() as u32 + 2;
    let local = artifact.reflection.local_size;
    if limits.max_bound_descriptor_sets < 1
        || limits.max_per_stage_descriptor_storage_buffers < count
        || limits.max_descriptor_set_storage_buffers < count
        || limits.max_memory_allocation_count < count
        || limits.max_compute_work_group_invocations < local[0]
    {
        return Err(fail(
            "device-limits",
            "device interface or local invocation limits are insufficient",
        ));
    }
    for (axis, local_size) in local.iter().enumerate() {
        if *local_size > limits.max_compute_work_group_size[axis]
            || invocation.workgroups[axis] > limits.max_compute_work_group_count[axis]
        {
            return Err(fail(
                "device-limits",
                "workgroup dimensions exceed the selected device limits",
            ));
        }
    }
    let parameter_count =
        (artifact.reflection.bindings.len() * 2 + invocation.parameters.len()).max(1);
    if (parameter_count as u64) * 4 > u64::from(limits.max_storage_buffer_range)
        || invocation
            .buffers
            .iter()
            .any(|b| (b.words.len() as u64) * 4 > u64::from(limits.max_storage_buffer_range))
        || limits.max_storage_buffer_range < 4
    {
        return Err(fail(
            "device-limits",
            "storage buffer range exceeds the selected device limits",
        ));
    }
    Ok(())
}

struct Context {
    entry: Option<Entry>,
    instance: Option<ash::Instance>,
    device: Option<ash::Device>,
    queue: vk::Queue,
    family: u32,
    memory: vk::PhysicalDeviceMemoryProperties,
    name: String,
    retain: Cell<bool>,
    // Ash's handles are Send/Sync. This owner and all borrowed allocations are not:
    // queue submission and mapped host memory are used by one thread at a time.
    _single_thread: PhantomData<Rc<()>>,
}
impl Context {
    fn device(&self) -> &ash::Device {
        self.device.as_ref().expect("created device")
    }
    fn new(artifact: &Artifact, invocation: &Invocation) -> Result<Self> {
        if !cfg!(target_endian = "little") {
            return Err(fail(
                "host-endian",
                "word ABI execution currently requires a little-endian host",
            ));
        }
        // SAFETY: the loader stays owned until all instance and device objects end.
        let entry = unsafe { Entry::load() }.map_err(|e| fail("loader", e.to_string()))?;
        let version = unsafe { entry.try_enumerate_instance_version() }
            .map_err(|e| vk_error("instance-version", e))?
            .unwrap_or(vk::API_VERSION_1_0);
        if version < vk::API_VERSION_1_2 {
            return Err(fail(
                "device-features",
                "Vulkan loader does not support API 1.2",
            ));
        }
        let application = vk::ApplicationInfo::default().api_version(vk::API_VERSION_1_2);
        let create = vk::InstanceCreateInfo::default().application_info(&application);
        let instance = unsafe { entry.create_instance(&create, None) }
            .map_err(|e| vk_error("instance-create", e))?;
        // Acquire the owner immediately, so every later failing query destroys the instance.
        let mut context = Self {
            entry: Some(entry),
            instance: Some(instance),
            device: None,
            queue: vk::Queue::null(),
            family: 0,
            memory: vk::PhysicalDeviceMemoryProperties::default(),
            name: String::new(),
            retain: Cell::new(false),
            _single_thread: PhantomData,
        };
        let instance = context.instance.as_ref().unwrap();
        let physicals = unsafe { instance.enumerate_physical_devices() }
            .map_err(|e| vk_error("physical-devices", e))?;
        let mut selected = None;
        for physical in physicals {
            let properties = unsafe { instance.get_physical_device_properties(physical) };
            if properties.api_version < vk::API_VERSION_1_2 {
                continue;
            }
            let mut features12 = vk::PhysicalDeviceVulkan12Features::default();
            let mut features = vk::PhysicalDeviceFeatures2::default().push_next(&mut features12);
            unsafe { instance.get_physical_device_features2(physical, &mut features) };
            if admit_features(
                properties.api_version,
                features.features.robust_buffer_access != 0,
                features12.vulkan_memory_model != 0,
                features12.vulkan_memory_model_device_scope != 0,
            )
            .is_err()
                || admit_limits(&properties.limits, artifact, invocation).is_err()
            {
                continue;
            }
            let families =
                unsafe { instance.get_physical_device_queue_family_properties(physical) };
            if let Some((family, _)) = families
                .iter()
                .enumerate()
                .find(|(_, p)| p.queue_count > 0 && p.queue_flags.contains(vk::QueueFlags::COMPUTE))
            {
                selected = Some((physical, family as u32, properties));
                break;
            }
        }
        let (physical,family,properties)=selected.ok_or_else(||fail("device-features","no compute device satisfies the exact enabled feature and resource-limit requirements"))?;
        let priority = [1.0f32];
        let queues = [vk::DeviceQueueCreateInfo::default()
            .queue_family_index(family)
            .queue_priorities(&priority)];
        let features = vk::PhysicalDeviceFeatures::default().robust_buffer_access(true);
        let mut features12 = vk::PhysicalDeviceVulkan12Features::default()
            .vulkan_memory_model(true)
            .vulkan_memory_model_device_scope(true);
        let create = vk::DeviceCreateInfo::default()
            .queue_create_infos(&queues)
            .enabled_features(&features)
            .push_next(&mut features12);
        let device = unsafe { instance.create_device(physical, &create, None) }
            .map_err(|e| vk_error("device-create", e))?;
        // Record device ownership before any later query; partial initialization is owned.
        context.device = Some(device);
        context.family = family;
        context.queue = unsafe { context.device().get_device_queue(family, 0) };
        context.memory = unsafe {
            context
                .instance
                .as_ref()
                .unwrap()
                .get_physical_device_memory_properties(physical)
        };
        let bytes: Vec<u8> = properties
            .device_name
            .iter()
            .map(|b| *b as u8)
            .take_while(|b| *b != 0)
            .collect();
        context.name = String::from_utf8_lossy(&bytes).into_owned();
        Ok(context)
    }
}
impl Drop for Context {
    fn drop(&mut self) {
        if self.retain.get() {
            // Unexpected idle errors provide no safe release point. Keep native handles
            // and the loader alive rather than freeing possibly submitted allocations.
            // Ash's Device/Instance wrappers have no Drop implementation: skipping
            // their explicit destroy calls retains the native objects.
            self.device.take();
            self.instance.take();
            if let Some(entry) = self.entry.take() {
                std::mem::forget(entry);
            }
            return;
        }
        unsafe {
            if let Some(device) = self.device.take() {
                device.destroy_device(None);
            }
            if let Some(instance) = self.instance.take() {
                instance.destroy_instance(None);
            }
        }
    }
}

struct Buffer<'a> {
    context: &'a Context,
    buffer: vk::Buffer,
    memory: vk::DeviceMemory,
    mapping: Option<NonNull<std::ffi::c_void>>,
    coherent: bool,
    words: usize,
}
impl<'a> Buffer<'a> {
    fn new(context: &'a Context, words: &[u32]) -> Result<Self> {
        if words.is_empty() {
            return Err(fail(
                "allocation",
                "zero-sized storage allocations are rejected",
            ));
        }
        let bytes = (words.len() as u64)
            .checked_mul(4)
            .ok_or_else(|| fail("allocation", "buffer size overflowed"))?;
        let create = vk::BufferCreateInfo::default()
            .size(bytes)
            .usage(vk::BufferUsageFlags::STORAGE_BUFFER)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);
        let handle = unsafe { context.device().create_buffer(&create, None) }
            .map_err(|e| vk_error("buffer-create", e))?;
        let mut buffer = Self {
            context,
            buffer: handle,
            memory: vk::DeviceMemory::null(),
            mapping: None,
            coherent: false,
            words: words.len(),
        };
        let requirements = unsafe { context.device().get_buffer_memory_requirements(handle) };
        if requirements.size < bytes {
            return Err(fail(
                "allocation",
                "device allocation requirements are smaller than the buffer",
            ));
        }
        let mut candidates = vec![];
        for index in 0..context.memory.memory_type_count {
            let flags = context.memory.memory_types[index as usize].property_flags;
            if requirements.memory_type_bits & (1u32 << index) != 0
                && flags.contains(vk::MemoryPropertyFlags::HOST_VISIBLE)
            {
                let score = u8::from(flags.contains(vk::MemoryPropertyFlags::HOST_COHERENT)) * 2
                    + u8::from(flags.contains(vk::MemoryPropertyFlags::HOST_CACHED));
                candidates.push((score, index, flags));
            }
        }
        let (_, index, flags) = candidates
            .into_iter()
            .max_by_key(|(score, _, _)| *score)
            .ok_or_else(|| {
                fail(
                    "memory-type",
                    "no compatible host-visible storage memory is available",
                )
            })?;
        let allocate = vk::MemoryAllocateInfo::default()
            .allocation_size(requirements.size)
            .memory_type_index(index);
        buffer.memory = unsafe { context.device().allocate_memory(&allocate, None) }
            .map_err(|e| vk_error("memory-allocate", e))?;
        unsafe {
            context
                .device()
                .bind_buffer_memory(handle, buffer.memory, 0)
        }
        .map_err(|e| vk_error("memory-bind", e))?;
        let pointer = unsafe {
            context.device().map_memory(
                buffer.memory,
                0,
                vk::WHOLE_SIZE,
                vk::MemoryMapFlags::empty(),
            )
        }
        .map_err(|e| vk_error("memory-map", e))?;
        let mapping = NonNull::new(pointer)
            .ok_or_else(|| fail("memory-map", "device returned a null mapped pointer"))?;
        buffer.mapping = Some(mapping);
        buffer.coherent = flags.contains(vk::MemoryPropertyFlags::HOST_COHERENT);
        // SAFETY: fresh dedicated memory has no pending GPU access, mapping covers the
        // whole allocation and Vulkan's mapped-memory alignment covers u32 alignment.
        unsafe {
            std::ptr::copy_nonoverlapping(
                words.as_ptr(),
                mapping.as_ptr().cast::<u32>(),
                words.len(),
            )
        };
        if !buffer.coherent {
            let ranges = [vk::MappedMemoryRange::default()
                .memory(buffer.memory)
                .offset(0)
                .size(vk::WHOLE_SIZE)];
            unsafe { context.device().flush_mapped_memory_ranges(&ranges) }
                .map_err(|e| vk_error("memory-flush", e))?;
        }
        Ok(buffer)
    }
    fn read(&self) -> Result<Vec<u32>> {
        if !self.coherent {
            let ranges = [vk::MappedMemoryRange::default()
                .memory(self.memory)
                .offset(0)
                .size(vk::WHOLE_SIZE)];
            unsafe {
                self.context
                    .device()
                    .invalidate_mapped_memory_ranges(&ranges)
            }
            .map_err(|e| vk_error("memory-invalidate", e))?;
        }
        let mut output = vec![0u32; self.words];
        // Caller has waited for the sole queue submission's fence and its shader->host
        // barrier. No shader access overlaps this host read or the eventual unmap.
        unsafe {
            std::ptr::copy_nonoverlapping(
                self.mapping.unwrap().as_ptr().cast::<u32>(),
                output.as_mut_ptr(),
                self.words,
            )
        };
        Ok(output)
    }
}
impl Drop for Buffer<'_> {
    fn drop(&mut self) {
        if self.context.retain.get() {
            return;
        }
        unsafe {
            if self.mapping.take().is_some() {
                self.context.device().unmap_memory(self.memory);
            }
            self.context.device().destroy_buffer(self.buffer, None);
            if self.memory != vk::DeviceMemory::null() {
                self.context.device().free_memory(self.memory, None);
            }
        }
    }
}

struct Batch<'a> {
    context: &'a Context,
    buffers: Vec<Buffer<'a>>,
    descriptor_layout: vk::DescriptorSetLayout,
    descriptor_pool: vk::DescriptorPool,
    pipeline_layout: vk::PipelineLayout,
    shader: vk::ShaderModule,
    pipeline: vk::Pipeline,
    command_pool: vk::CommandPool,
    fence: vk::Fence,
    submitted: bool,
}
impl<'a> Batch<'a> {
    fn new(context: &'a Context) -> Self {
        Self {
            context,
            buffers: vec![],
            descriptor_layout: vk::DescriptorSetLayout::null(),
            descriptor_pool: vk::DescriptorPool::null(),
            pipeline_layout: vk::PipelineLayout::null(),
            shader: vk::ShaderModule::null(),
            pipeline: vk::Pipeline::null(),
            command_pool: vk::CommandPool::null(),
            fence: vk::Fence::null(),
            submitted: false,
        }
    }
    fn run(&mut self, artifact: &Artifact, invocation: &Invocation) -> Result<()> {
        let device = self.context.device();
        for binding in &artifact.reflection.bindings {
            let argument = invocation
                .buffers
                .iter()
                .find(|b| b.resource == binding.resource)
                .ok_or_else(|| fail("arguments", "missing reflected resource"))?;
            self.buffers
                .push(Buffer::new(self.context, &argument.words)?);
        }
        self.buffers.push(Buffer::new(
            self.context,
            &parameter_words(&artifact.reflection, invocation)?,
        )?);
        self.buffers.push(Buffer::new(self.context, &[0])?);
        let bindings: Vec<_> = (0..self.buffers.len())
            .map(|binding| {
                vk::DescriptorSetLayoutBinding::default()
                    .binding(binding as u32)
                    .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                    .descriptor_count(1)
                    .stage_flags(vk::ShaderStageFlags::COMPUTE)
            })
            .collect();
        self.descriptor_layout = unsafe {
            device.create_descriptor_set_layout(
                &vk::DescriptorSetLayoutCreateInfo::default().bindings(&bindings),
                None,
            )
        }
        .map_err(|e| vk_error("descriptor-layout", e))?;
        let pool_sizes = [vk::DescriptorPoolSize::default()
            .ty(vk::DescriptorType::STORAGE_BUFFER)
            .descriptor_count(self.buffers.len() as u32)];
        self.descriptor_pool = unsafe {
            device.create_descriptor_pool(
                &vk::DescriptorPoolCreateInfo::default()
                    .max_sets(1)
                    .pool_sizes(&pool_sizes),
                None,
            )
        }
        .map_err(|e| vk_error("descriptor-pool", e))?;
        let layouts = [self.descriptor_layout];
        let descriptor = unsafe {
            device.allocate_descriptor_sets(
                &vk::DescriptorSetAllocateInfo::default()
                    .descriptor_pool(self.descriptor_pool)
                    .set_layouts(&layouts),
            )
        }
        .map_err(|e| vk_error("descriptor-allocate", e))?[0];
        let infos: Vec<_> = self
            .buffers
            .iter()
            .map(|b| {
                vk::DescriptorBufferInfo::default()
                    .buffer(b.buffer)
                    .offset(0)
                    .range(b.words as u64 * 4)
            })
            .collect();
        let writes: Vec<_> = infos
            .iter()
            .enumerate()
            .map(|(binding, info)| {
                vk::WriteDescriptorSet::default()
                    .dst_set(descriptor)
                    .dst_binding(binding as u32)
                    .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                    .buffer_info(std::slice::from_ref(info))
            })
            .collect();
        unsafe { device.update_descriptor_sets(&writes, &[]) };
        self.pipeline_layout = unsafe {
            device.create_pipeline_layout(
                &vk::PipelineLayoutCreateInfo::default().set_layouts(&layouts),
                None,
            )
        }
        .map_err(|e| vk_error("pipeline-layout", e))?;
        self.shader = unsafe {
            device.create_shader_module(
                &vk::ShaderModuleCreateInfo::default().code(&artifact.words),
                None,
            )
        }
        .map_err(|e| vk_error("shader-create", e))?;
        let entry = CString::new(artifact.reflection.entry.as_str())
            .map_err(|_| fail("entry-name", "entry name contains NUL"))?;
        let stage = vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlags::COMPUTE)
            .module(self.shader)
            .name(&entry);
        let pipelines = [vk::ComputePipelineCreateInfo::default()
            .stage(stage)
            .layout(self.pipeline_layout)];
        match unsafe {
            device.create_compute_pipelines(vk::PipelineCache::null(), &pipelines, None)
        } {
            Ok(pipelines) => self.pipeline = pipelines[0],
            Err((partial, error)) => {
                for pipeline in partial {
                    if pipeline != vk::Pipeline::null() {
                        unsafe { device.destroy_pipeline(pipeline, None) }
                    }
                }
                return Err(vk_error("pipeline-create", error));
            }
        }
        self.command_pool = unsafe {
            device.create_command_pool(
                &vk::CommandPoolCreateInfo::default()
                    .queue_family_index(self.context.family)
                    .flags(vk::CommandPoolCreateFlags::TRANSIENT),
                None,
            )
        }
        .map_err(|e| vk_error("command-pool", e))?;
        let command = unsafe {
            device.allocate_command_buffers(
                &vk::CommandBufferAllocateInfo::default()
                    .command_pool(self.command_pool)
                    .level(vk::CommandBufferLevel::PRIMARY)
                    .command_buffer_count(1),
            )
        }
        .map_err(|e| vk_error("command-allocate", e))?[0];
        unsafe {
            device.begin_command_buffer(
                command,
                &vk::CommandBufferBeginInfo::default()
                    .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT),
            )
        }
        .map_err(|e| vk_error("command-begin", e))?;
        let upload = [vk::MemoryBarrier::default()
            .src_access_mask(vk::AccessFlags::HOST_WRITE)
            .dst_access_mask(vk::AccessFlags::SHADER_READ | vk::AccessFlags::SHADER_WRITE)];
        let download = [vk::MemoryBarrier::default()
            .src_access_mask(vk::AccessFlags::SHADER_WRITE)
            .dst_access_mask(vk::AccessFlags::HOST_READ)];
        unsafe {
            device.cmd_pipeline_barrier(
                command,
                vk::PipelineStageFlags::HOST,
                vk::PipelineStageFlags::COMPUTE_SHADER,
                vk::DependencyFlags::empty(),
                &upload,
                &[],
                &[],
            );
            device.cmd_bind_pipeline(command, vk::PipelineBindPoint::COMPUTE, self.pipeline);
            device.cmd_bind_descriptor_sets(
                command,
                vk::PipelineBindPoint::COMPUTE,
                self.pipeline_layout,
                0,
                &[descriptor],
                &[],
            );
            device.cmd_dispatch(
                command,
                invocation.workgroups[0],
                invocation.workgroups[1],
                invocation.workgroups[2],
            );
            device.cmd_pipeline_barrier(
                command,
                vk::PipelineStageFlags::COMPUTE_SHADER,
                vk::PipelineStageFlags::HOST,
                vk::DependencyFlags::empty(),
                &download,
                &[],
                &[],
            );
        }
        unsafe { device.end_command_buffer(command) }.map_err(|e| vk_error("command-end", e))?;
        self.fence = unsafe { device.create_fence(&vk::FenceCreateInfo::default(), None) }
            .map_err(|e| vk_error("fence-create", e))?;
        let commands = [command];
        let submissions = [vk::SubmitInfo::default().command_buffers(&commands)];
        // Submission may have side effects on failure. Set the lifetime state before
        // entering the driver, and release it only after a signalled fence or idle.
        self.submitted = true;
        unsafe { device.queue_submit(self.context.queue, &submissions, self.fence) }
            .map_err(|e| vk_error("queue-submit", e))?;
        unsafe { device.wait_for_fences(&[self.fence], true, 10_000_000_000) }
            .map_err(|e| vk_error("fence-wait", e))?;
        self.submitted = false;
        Ok(())
    }
}
impl Drop for Batch<'_> {
    fn drop(&mut self) {
        let device = self.context.device();
        if self.submitted {
            // This wait can outlast the ordinary fence timeout. The parent worker
            // deadline is the containment boundary for an unresponsive native driver.
            match unsafe { device.device_wait_idle() } {
                Ok(()) | Err(vk::Result::ERROR_DEVICE_LOST) => self.submitted = false,
                Err(_) => {
                    self.context.retain.set(true);
                    return;
                }
            }
        }
        if self.context.retain.get() {
            return;
        }
        unsafe {
            if self.fence != vk::Fence::null() {
                device.destroy_fence(self.fence, None);
            }
            if self.command_pool != vk::CommandPool::null() {
                device.destroy_command_pool(self.command_pool, None);
            }
            if self.pipeline != vk::Pipeline::null() {
                device.destroy_pipeline(self.pipeline, None);
            }
            if self.shader != vk::ShaderModule::null() {
                device.destroy_shader_module(self.shader, None);
            }
            if self.pipeline_layout != vk::PipelineLayout::null() {
                device.destroy_pipeline_layout(self.pipeline_layout, None);
            }
            if self.descriptor_pool != vk::DescriptorPool::null() {
                device.destroy_descriptor_pool(self.descriptor_pool, None);
            }
            if self.descriptor_layout != vk::DescriptorSetLayout::null() {
                device.destroy_descriptor_set_layout(self.descriptor_layout, None);
            }
        }
        // Owned buffers drop after this method, only after the queue release point.
    }
}

pub(crate) fn execute(artifact: &Artifact, invocation: &Invocation) -> Result<Execution> {
    let context = Context::new(artifact, invocation)?;
    let mut batch = Batch::new(&context);
    batch.run(artifact, invocation)?;
    let guard = batch.buffers[artifact.reflection.guard_binding as usize].read()?[0];
    if guard != 0 {
        return Err(fail(
            "guard-failed",
            format!("device guard failure mask: 0x{guard:08x}"),
        ));
    }
    let mut buffers = Vec::with_capacity(invocation.buffers.len());
    for argument in &invocation.buffers {
        let binding = artifact
            .reflection
            .bindings
            .iter()
            .position(|b| b.resource == argument.resource)
            .ok_or_else(|| fail("arguments", "missing reflected resource"))?;
        let mut output = argument.clone();
        output.words = batch.buffers[binding].read()?;
        buffers.push(output);
    }
    Ok(Execution {
        buffers,
        guard,
        device: context.name.clone(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_four_required_features_are_admitted_explicitly() {
        admit_features(vk::API_VERSION_1_2, true, true, true).unwrap();
        for tuple in [
            (vk::API_VERSION_1_1, true, true, true),
            (vk::API_VERSION_1_2, false, true, true),
            (vk::API_VERSION_1_2, true, false, true),
            (vk::API_VERSION_1_2, true, true, false),
        ] {
            assert_eq!(
                admit_features(tuple.0, tuple.1, tuple.2, tuple.3)
                    .unwrap_err()
                    .code,
                "device-features"
            );
        }
    }
    #[test]
    fn device_limits_cover_dispatch_descriptors_and_owned_allocations() {
        let artifact: Artifact = kuiper_contracts::canonical::parse(include_bytes!(
            "../tests/fixtures/pretested-loop.json"
        ))
        .unwrap();
        let invocation = Invocation {
            entry: artifact.reflection.entry.clone(),
            workgroups: [1, 1, 1],
            parameters: vec![0],
            buffers: vec![kuiper_contracts::BufferArg {
                resource: 0,
                words: vec![0; 4],
                offset: 0,
                length: 4,
            }],
        };
        let limits = vk::PhysicalDeviceLimits {
            max_bound_descriptor_sets: 1,
            max_per_stage_descriptor_storage_buffers: 3,
            max_descriptor_set_storage_buffers: 3,
            max_memory_allocation_count: 3,
            max_compute_work_group_invocations: 4,
            max_compute_work_group_size: [4, 1, 1],
            max_compute_work_group_count: [1, 1, 1],
            max_storage_buffer_range: 16,
            ..Default::default()
        };
        admit_limits(&limits, &artifact, &invocation).unwrap();
        let mut insufficient = limits;
        insufficient.max_storage_buffer_range = 12;
        assert!(admit_limits(&insufficient, &artifact, &invocation).is_err());
        let mut insufficient = limits;
        insufficient.max_per_stage_descriptor_storage_buffers = 2;
        assert!(admit_limits(&insufficient, &artifact, &invocation).is_err());
        let mut insufficient = limits;
        insufficient.max_compute_work_group_count[0] = 0;
        assert!(admit_limits(&insufficient, &artifact, &invocation).is_err());
    }
}
