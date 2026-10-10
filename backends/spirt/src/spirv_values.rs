//! Named enumerants from the pinned Khronos SPIR-V grammar.
//! https://github.com/KhronosGroup/SPIRV-Headers/blob/2acb319af38d43be3ea76bfabf3998e5281d8d12/include/spirv/unified1/spirv.core.grammar.json
pub const STORAGE_BUFFER: u32 = 12;
pub const STORAGE_INPUT: u32 = 1;
pub const SCOPE_DEVICE: u32 = 1;
pub const SEMANTICS_RELAXED: u32 = 0;
pub const ARRAY_STRIDE: u32 = 6;
pub const BLOCK: u32 = 2;
pub const OFFSET: u32 = 35;
pub const DESCRIPTOR_SET: u32 = 34;
pub const BINDING: u32 = 33;
pub const NON_WRITABLE: u32 = 24;
pub const BUILTIN: u32 = 11;
pub const GLOBAL_INVOCATION_ID: u32 = 28;
pub const LOCAL_INVOCATION_ID: u32 = 27;
pub const WORKGROUP_ID: u32 = 26;
pub const NUM_WORKGROUPS: u32 = 24;
pub const SHADER: u32 = 1;
pub const VULKAN_MEMORY_MODEL: u32 = 5345;
pub const VULKAN_MEMORY_MODEL_DEVICE_SCOPE: u32 = 5346;
pub const ADDRESSING_LOGICAL: u32 = 0;
pub const MEMORY_MODEL_VULKAN: u32 = 3;
pub const LOCAL_SIZE: u32 = 17;
pub const GL_COMPUTE: u32 = 5;

#[cfg(test)]
mod tests {
    use super::*;
    use spirt::spv::spec::{OperandKindDef, Spec};
    #[test]
    fn enumerants_match_pinned_khronos_grammar() {
        let spec = Spec::get();
        for (kind, variant, value) in [
            ("StorageClass", "StorageBuffer", STORAGE_BUFFER),
            ("StorageClass", "Input", STORAGE_INPUT),
            ("Scope", "Device", SCOPE_DEVICE),
            ("Decoration", "ArrayStride", ARRAY_STRIDE),
            ("Decoration", "Block", BLOCK),
            ("Decoration", "Offset", OFFSET),
            ("Decoration", "DescriptorSet", DESCRIPTOR_SET),
            ("Decoration", "Binding", BINDING),
            ("Decoration", "NonWritable", NON_WRITABLE),
            ("Decoration", "BuiltIn", BUILTIN),
            ("BuiltIn", "GlobalInvocationId", GLOBAL_INVOCATION_ID),
            ("BuiltIn", "LocalInvocationId", LOCAL_INVOCATION_ID),
            ("BuiltIn", "WorkgroupId", WORKGROUP_ID),
            ("BuiltIn", "NumWorkgroups", NUM_WORKGROUPS),
            ("Capability", "Shader", SHADER),
            ("Capability", "VulkanMemoryModel", VULKAN_MEMORY_MODEL),
            (
                "Capability",
                "VulkanMemoryModelDeviceScope",
                VULKAN_MEMORY_MODEL_DEVICE_SCOPE,
            ),
            ("AddressingModel", "Logical", ADDRESSING_LOGICAL),
            ("MemoryModel", "Vulkan", MEMORY_MODEL_VULKAN),
            ("ExecutionMode", "LocalSize", LOCAL_SIZE),
            ("ExecutionModel", "GLCompute", GL_COMPUTE),
        ] {
            let kind = spec.operand_kinds.lookup(kind).unwrap();
            let OperandKindDef::ValueEnum { variants } = &spec.operand_kinds[kind] else {
                panic!("expected value enum")
            };
            assert_eq!(variants.lookup(variant).map(|v| v as u32), Some(value));
        }
        let kind = spec.operand_kinds.lookup("MemorySemantics").unwrap();
        let OperandKindDef::BitEnum { empty_name, .. } = &spec.operand_kinds[kind] else {
            panic!("expected bit enum")
        };
        assert_eq!(*empty_name, "None");
        assert_eq!(SEMANTICS_RELAXED, 0);
    }
}
