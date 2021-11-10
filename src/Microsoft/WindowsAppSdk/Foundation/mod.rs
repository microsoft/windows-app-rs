#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[inline]
pub unsafe fn MddBootstrapInitialize<
    'a,
    Param1: ::windows::core::IntoParam<'a, ::windows::Win32::Foundation::PWSTR>,
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
                versiontag: ::windows::Win32::Foundation::PWSTR,
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
#[inline]
pub unsafe fn MddBootstrapShutdown() {
    #[cfg(windows)]
    {
        #[link(name = "microsoft.windowsappruntime.bootstrap")]
        extern "system" {
            fn MddBootstrapShutdown();
        }
        ::core::mem::transmute(MddBootstrapShutdown())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
