use crate::spec::{base, Cc, FloatAbi, LinkerFlavor, Target, TargetOptions};

pub(crate) fn target() -> Target {
    let mut base = base::l4re::opts();

    let extra_link_args = &["-zmax-page-size=0x1000","-zcommon-page-size=0x1000"];
    base.add_pre_link_args(LinkerFlavor::Unix(Cc::Yes), extra_link_args);
    base.add_pre_link_args(LinkerFlavor::Unix(Cc::No), extra_link_args);

    Target {
        llvm_target: "armv7-unknown-l4re-uclibc".into(),
        metadata: crate::spec::TargetMetadata {
            description: Some("Armv7-A L4Re, softfloat".into()),
            tier: Some(3),
            host_tools: Some(false),
            std: Some(true),
        },
        pointer_width: 32,
        data_layout: "e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64".into(),
        arch: "arm".into(),
        options: TargetOptions {
            abi: "eabi".into(),
            llvm_floatabi: Some(FloatAbi::Soft),
            features: "+v7,+thumb2,+soft-float,-neon".into(),
            max_atomic_width: Some(64),
            mcount: "__mcount".into(),
            ..base
        }
    }
}
