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
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_IDENTITY_PUBLISHER: &str =
    "CN=Microsoft Corporation, O=Microsoft Corporation, L=Redmond, S=Washington, C=US";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_IDENTITY_PUBLISHERID: &str = "8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_IDENTITY_PUBLISHERID_W: &str = "8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_IDENTITY_PUBLISHER_W: &str =
    "CN=Microsoft Corporation, O=Microsoft Corporation, L=Redmond, S=Washington, C=US";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_PKG_DDLM_ARM64_FAMILY: &str =
    "Microsoft.WinAppRuntime.DDLM.1001.524.1918.0-a6_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_PKG_DDLM_ARM64_FAMILY_W: &str =
    "Microsoft.WinAppRuntime.DDLM.1001.524.1918.0-a6_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_PKG_DDLM_X64_FAMILY: &str =
    "Microsoft.WinAppRuntime.DDLM.1001.524.1918.0-x6_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_PKG_DDLM_X64_FAMILY_W: &str =
    "Microsoft.WinAppRuntime.DDLM.1001.524.1918.0-x6_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_PKG_DDLM_X86_FAMILY: &str =
    "Microsoft.WinAppRuntime.DDLM.1001.524.1918.0-x8_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_PKG_DDLM_X86_FAMILY_W: &str =
    "Microsoft.WinAppRuntime.DDLM.1001.524.1918.0-x8_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_PKG_FRAMEWORK_FAMILY: &str = "Microsoft.WindowsAppRuntime.1.1_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_PKG_FRAMEWORK_FAMILY_W: &str = "Microsoft.WindowsAppRuntime.1.1_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_PKG_MAIN_FAMILY: &str =
    "MicrosoftCorporationII.WinAppRuntime.Main.1.1_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_PKG_MAIN_FAMILY_W: &str =
    "MicrosoftCorporationII.WinAppRuntime.Main.1.1_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_PKG_SINGLETON_FAMILY: &str =
    "MicrosoftCorporationII.WinAppRuntime.Singleton_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_PKG_SINGLETON_FAMILY_W: &str =
    "MicrosoftCorporationII.WinAppRuntime.Singleton_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_VERSION_BUILD: u16 = 1918u16;
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_VERSION_DOTQUADSTRING: &str = "1001.524.1918.0";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_VERSION_DOTQUADSTRING_W: &str = "1001.524.1918.0";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_VERSION_MAJOR: u16 = 1001u16;
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_VERSION_MINOR: u16 = 524u16;
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_VERSION_REVISION: u16 = 0u16;
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_VERSION_UINT64: u64 = 281758702375927808u64;
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WAS_RELEASE_CHANNEL: &str = "";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WAS_RELEASE_CHANNEL_W: &str = "";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WAS_RELEASE_FMT_VERSION_STAG: &str = "";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WAS_RELEASE_FMT_VERSION_STAG_W: &str = "";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WAS_RELEASE_FMT_VERSION_TAG: &str = "";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WAS_RELEASE_FMT_VERSION_TAG_W: &str = "";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WAS_RELEASE_MAJOR: u32 = 1u32;
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WAS_RELEASE_MAJORMINOR: u32 = 65537u32;
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WAS_RELEASE_MINOR: u32 = 1u32;
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WAS_RELEASE_PATCH: u32 = 1u32;
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WAS_RELEASE_VERSION_STAG: &str = "";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WAS_RELEASE_VERSION_STAG_W: &str = "";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WAS_RELEASE_VERSION_TAG: &str = "";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WAS_RELEASE_VERSION_TAG_W: &str = "";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RELEASE_CHANNEL: &str = "";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RELEASE_CHANNEL_W: &str = "";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RELEASE_FORMATTED_VERSION_SHORTTAG: &str = "";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RELEASE_FORMATTED_VERSION_SHORTTAG_W: &str = "";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RELEASE_FORMATTED_VERSION_TAG: &str = "";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RELEASE_FORMATTED_VERSION_TAG_W: &str = "";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RELEASE_MAJOR: u32 = 1u32;
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RELEASE_MAJORMINOR: u32 = 65537u32;
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RELEASE_MINOR: u32 = 1u32;
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RELEASE_PATCH: u32 = 1u32;
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RELEASE_VERSION_SHORTTAG: &str = "";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RELEASE_VERSION_SHORTTAG_W: &str = "";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RELEASE_VERSION_TAG: &str = "";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RELEASE_VERSION_TAG_W: &str = "";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RUNTIME_IDENTITY_PUBLISHER: &str =
    "CN=Microsoft Corporation, O=Microsoft Corporation, L=Redmond, S=Washington, C=US";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RUNTIME_IDENTITY_PUBLISHERID: &str = "8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RUNTIME_IDENTITY_PUBLISHERID_W: &str = "8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RUNTIME_IDENTITY_PUBLISHER_W: &str =
    "CN=Microsoft Corporation, O=Microsoft Corporation, L=Redmond, S=Washington, C=US";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RUNTIME_PACKAGE_DDLM_ARM64_PACKAGEFAMILYNAME: &str =
    "Microsoft.WinAppRuntime.DDLM.1001.524.1918.0-a6_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RUNTIME_PACKAGE_DDLM_ARM64_PACKAGEFAMILYNAME_W: &str =
    "Microsoft.WinAppRuntime.DDLM.1001.524.1918.0-a6_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RUNTIME_PACKAGE_DDLM_X64_PACKAGEFAMILYNAME: &str =
    "Microsoft.WinAppRuntime.DDLM.1001.524.1918.0-x6_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RUNTIME_PACKAGE_DDLM_X64_PACKAGEFAMILYNAME_W: &str =
    "Microsoft.WinAppRuntime.DDLM.1001.524.1918.0-x6_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RUNTIME_PACKAGE_DDLM_X86_PACKAGEFAMILYNAME: &str =
    "Microsoft.WinAppRuntime.DDLM.1001.524.1918.0-x8_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RUNTIME_PACKAGE_DDLM_X86_PACKAGEFAMILYNAME_W: &str =
    "Microsoft.WinAppRuntime.DDLM.1001.524.1918.0-x8_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RUNTIME_PACKAGE_FRAMEWORK_PACKAGEFAMILYNAME: &str =
    "Microsoft.WindowsAppRuntime.1.1_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RUNTIME_PACKAGE_FRAMEWORK_PACKAGEFAMILYNAME_W: &str =
    "Microsoft.WindowsAppRuntime.1.1_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RUNTIME_PACKAGE_MAIN_PACKAGEFAMILYNAME: &str =
    "MicrosoftCorporationII.WinAppRuntime.Main.1.1_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RUNTIME_PACKAGE_MAIN_PACKAGEFAMILYNAME_W: &str =
    "MicrosoftCorporationII.WinAppRuntime.Main.1.1_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RUNTIME_PACKAGE_SINGLETON_PACKAGEFAMILYNAME: &str =
    "MicrosoftCorporationII.WinAppRuntime.Singleton_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RUNTIME_PACKAGE_SINGLETON_PACKAGEFAMILYNAME_W: &str =
    "MicrosoftCorporationII.WinAppRuntime.Singleton_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RUNTIME_VERSION_DOTQUADSTRING: &str = "1001.524.1918.0";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RUNTIME_VERSION_DOTQUADSTRING_W: &str = "1001.524.1918.0";
#[cfg(feature = "implement")]
::core::include!("impl.rs");
