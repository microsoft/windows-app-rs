#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clashing_extern_declarations,
    clippy::all
)]
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
