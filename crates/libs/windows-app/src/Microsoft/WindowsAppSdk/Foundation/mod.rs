#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
#[inline]
pub unsafe fn MddBootstrapInitialize<
    'a,
    Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>,
    Param2: ::windows::core::IntoParam<'a, ::windows::Win32::Storage::Packaging::Appx::PACKAGE_VERSION>,
>(
    majorminorversion: u32,
    versiontag: Param1,
    minversion: Param2,
) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "microsoft.windowsappruntime.bootstrap")]
        extern "system" {
            fn MddBootstrapInitialize(
                majorminorversion: u32,
                versiontag: ::windows::core::PCWSTR,
                minversion: ::windows::Win32::Storage::Packaging::Appx::PACKAGE_VERSION,
            ) -> ::windows::core::HRESULT;
        }
        MddBootstrapInitialize(
            ::core::mem::transmute(majorminorversion),
            versiontag.into_param().abi(),
            minversion.into_param().abi(),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
#[inline]
pub unsafe fn MddBootstrapInitialize2<
    'a,
    Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>,
    Param2: ::windows::core::IntoParam<'a, ::windows::Win32::Storage::Packaging::Appx::PACKAGE_VERSION>,
>(
    majorminorversion: u32,
    versiontag: Param1,
    minversion: Param2,
    options: MddBootstrapInitializeOptions,
) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "microsoft.windowsappruntime.bootstrap")]
        extern "system" {
            fn MddBootstrapInitialize2(
                majorminorversion: u32,
                versiontag: ::windows::core::PCWSTR,
                minversion: ::windows::Win32::Storage::Packaging::Appx::PACKAGE_VERSION,
                options: MddBootstrapInitializeOptions,
            ) -> ::windows::core::HRESULT;
        }
        MddBootstrapInitialize2(
            ::core::mem::transmute(majorminorversion),
            versiontag.into_param().abi(),
            minversion.into_param().abi(),
            ::core::mem::transmute(options),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MddBootstrapInitializeOptions(pub i32);
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const MddBootstrapInitializeOptions_None: MddBootstrapInitializeOptions =
    MddBootstrapInitializeOptions(0i32);
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const MddBootstrapInitializeOptions_OnError_DebugBreak: MddBootstrapInitializeOptions =
    MddBootstrapInitializeOptions(1i32);
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const MddBootstrapInitializeOptions_OnError_DebugBreak_IfDebuggerAttached:
    MddBootstrapInitializeOptions = MddBootstrapInitializeOptions(2i32);
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const MddBootstrapInitializeOptions_OnError_FailFast: MddBootstrapInitializeOptions =
    MddBootstrapInitializeOptions(4i32);
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const MddBootstrapInitializeOptions_OnNoMatch_ShowUI: MddBootstrapInitializeOptions =
    MddBootstrapInitializeOptions(8i32);
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const MddBootstrapInitializeOptions_OnPackageIdentity_NOOP: MddBootstrapInitializeOptions =
    MddBootstrapInitializeOptions(16i32);
impl ::core::marker::Copy for MddBootstrapInitializeOptions {}
impl ::core::clone::Clone for MddBootstrapInitializeOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MddBootstrapInitializeOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MddBootstrapInitializeOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for MddBootstrapInitializeOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MddBootstrapInitializeOptions")
            .field(&self.0)
            .finish()
    }
}
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
#[inline]
pub unsafe fn MddBootstrapShutdown() {
    #[cfg(windows)]
    {
        #[link(name = "microsoft.windowsappruntime.bootstrap")]
        extern "system" {
            fn MddBootstrapShutdown();
        }
        MddBootstrapShutdown()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
