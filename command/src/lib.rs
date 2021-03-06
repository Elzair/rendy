//! Crate level docs.

#![forbid(overflowing_literals)]
#![deny(missing_copy_implementations)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![deny(intra_doc_link_resolution_failure)]
#![deny(path_statements)]
#![deny(trivial_bounds)]
#![deny(type_alias_bounds)]
#![deny(unconditional_recursion)]
#![deny(unions_with_drop_fields)]
#![deny(while_true)]
#![deny(unused)]
#![deny(bad_style)]
#![deny(future_incompatible)]
#![deny(rust_2018_compatibility)]
#![deny(rust_2018_idioms)]
#![allow(unused_unsafe)]

mod buffer;
mod encoder;
mod capability;
mod family;
mod pool;

pub use crate::{
    buffer::{
        CommandBuffer, ExecutableState, IndividualReset, InitialState, InvalidState, Level,
        MultiShot, NoIndividualReset, OneShot, PendingState, PrimaryLevel, RecordingState,
        RenderPassContinue, Resettable, SecondaryLevel, SimultaneousUse, Submit, Usage,
        Reset, Submittable,
    },
    encoder::{EncoderCommon, RenderPassEncoder, Encoder, RenderPassEncoderHRTB, DrawCommand, DispatchCommand},
    capability::{Capability, Compute, General, Graphics, Transfer, Supports},
    family::{families_from_device, Family, Submission},
    pool::CommandPool,
};
