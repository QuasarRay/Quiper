//! Versioned, implementation-independent contracts for the portable worker boundary.
#![forbid(unsafe_code)]
pub mod canonical;
pub mod validate;

use serde::{Deserialize, Serialize};

pub const KIR_SCHEMA: &str = "kuiper.kir/1";
pub const INTEGER_PROFILE: &str = "kuiper.integer32/1";
pub const ARTIFACT_SCHEMA: &str = "kuiper.artifact/1";
pub const SPIRV_FORMAT: &str = "khronos.spirv.vulkan1.2/1";
pub const WORD_ABI: &str = "kuiper.storage-words32/1";
pub const PROTOCOL: &str = "kuiper.worker/1";
pub const EXECUTION_SCHEMA: &str = "kuiper.execution/1";
pub const MAX_MESSAGE: usize = 16 * 1024 * 1024;
pub const MAX_WORDS: usize = 1_048_576;
pub const MAX_NODES: usize = 16_384;
pub const MAX_DEPTH: usize = 32;
pub const MAX_RESOURCES: usize = 16;
pub const MAX_WORK_STEPS: u64 = 10_000_000;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Diagnostic {
    pub code: String,
    pub stage: String,
    pub message: String,
}
impl Diagnostic {
    pub fn new(stage: &str, code: &str, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            stage: stage.into(),
            message: message.into(),
        }
    }
}
impl std::fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}: {}", self.stage, self.code, self.message)
    }
}
impl std::error::Error for Diagnostic {}
pub type Result<T> = std::result::Result<T, Diagnostic>;
pub type ValueId = u32;

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Scalar {
    U32,
    I32,
    Bool,
}
#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Access {
    Read,
    Write,
    ReadWrite,
    Atomic,
}
#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Builtin {
    GlobalId,
    LocalId,
    WorkgroupId,
    NumWorkgroups,
}
#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Binary {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    BitAnd,
    BitOr,
    BitXor,
    ShiftLeft,
    ShiftRight,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    LogicalAnd,
    LogicalOr,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ValueDecl {
    pub id: ValueId,
    pub ty: Scalar,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Resource {
    pub id: u32,
    pub element: Scalar,
    pub access: Access,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct LoopParam {
    pub value: ValueDecl,
    pub initial: ValueId,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Region {
    pub instructions: Vec<Instruction>,
    pub outputs: Vec<ValueId>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Instruction {
    Constant {
        result: ValueDecl,
        bits: u32,
    },
    Builtin {
        result: ValueId,
        builtin: Builtin,
        axis: u32,
    },
    Parameter {
        result: ValueId,
        index: u32,
    },
    ResourceLength {
        result: ValueId,
        resource: u32,
    },
    Binary {
        result: ValueId,
        op: Binary,
        left: ValueId,
        right: ValueId,
    },
    Load {
        result: ValueId,
        resource: u32,
        index: ValueId,
    },
    Store {
        resource: u32,
        index: ValueId,
        value: ValueId,
    },
    AtomicAdd {
        result: ValueId,
        resource: u32,
        index: ValueId,
        value: ValueId,
    },
    Guard {
        condition: ValueId,
        code: u32,
    },
    Select {
        condition: ValueId,
        then_region: Region,
        else_region: Region,
        results: Vec<ValueDecl>,
    },
    While {
        carried: Vec<LoopParam>,
        condition: Region,
        body: Region,
        results: Vec<ValueDecl>,
        iteration_limit: u32,
    },
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Kernel {
    pub name: String,
    pub local_size: [u32; 3],
    pub resources: Vec<Resource>,
    pub parameters: Vec<Scalar>,
    pub body: Region,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Package {
    pub schema: String,
    pub profile: String,
    pub kernels: Vec<Kernel>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BufferArg {
    pub resource: u32,
    pub words: Vec<u32>,
    pub offset: u32,
    pub length: u32,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Invocation {
    pub entry: String,
    pub workgroups: [u32; 3],
    pub parameters: Vec<u32>,
    pub buffers: Vec<BufferArg>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Binding {
    pub resource: u32,
    pub binding: u32,
    pub element: Scalar,
    pub access: Access,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Reflection {
    pub entry: String,
    pub local_size: [u32; 3],
    pub bindings: Vec<Binding>,
    pub parameter_types: Vec<Scalar>,
    pub parameter_binding: u32,
    pub guard_binding: u32,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Evidence {
    pub policy: String,
    pub input_digest: String,
    pub output_digest: String,
    pub compiler: String,
    pub dependencies: Vec<String>,
    pub checks: Vec<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Artifact {
    pub schema: String,
    pub format: String,
    pub abi: String,
    pub parent_digest: String,
    pub words: Vec<u32>,
    pub reflection: Reflection,
    pub requirements: Vec<String>,
    pub evidence: Evidence,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Execution {
    pub buffers: Vec<BufferArg>,
    pub guard: u32,
    pub device: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Endpoint {
    pub role: String,
    pub consumes: Vec<String>,
    pub produces: Vec<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub protocol: String,
    pub id: String,
    pub executable: String,
    pub executable_digest: String,
    pub endpoints: Vec<Endpoint>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "method", rename_all = "snake_case", deny_unknown_fields)]
// Requests are processed singly; boxing would complicate the stable Rust API.
#[allow(clippy::large_enum_variant)]
pub enum Request {
    Describe {},
    Compile {
        package: Package,
        entry: String,
    },
    Execute {
        artifact: Artifact,
        invocation: Invocation,
    },
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
// Replies are processed singly and own their bounded payload.
#[allow(clippy::large_enum_variant)]
pub enum Response {
    Description {
        protocol: String,
        id: String,
        endpoints: Vec<Endpoint>,
    },
    Compiled {
        artifact: Artifact,
    },
    Executed {
        execution: Execution,
    },
    Rejected {
        diagnostic: Diagnostic,
    },
}

/// The parameter descriptor starts with each owned view's offset and length.
pub fn parameter_words(reflection: &Reflection, invocation: &Invocation) -> Result<Vec<u32>> {
    let mut words = Vec::with_capacity(reflection.bindings.len() * 2 + invocation.parameters.len());
    for binding in &reflection.bindings {
        let buffer = invocation
            .buffers
            .iter()
            .find(|b| b.resource == binding.resource)
            .ok_or_else(|| {
                Diagnostic::new("abi", "missing-resource", "invocation omits a resource")
            })?;
        words.extend([buffer.offset, buffer.length]);
    }
    words.extend(&invocation.parameters);
    if words.is_empty() {
        words.push(0);
    }
    Ok(words)
}
