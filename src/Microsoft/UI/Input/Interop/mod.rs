#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clashing_extern_declarations,
    clippy::all
)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IPenDeviceInteropStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPenDeviceInteropStatics {
    type Vtable = IPenDeviceInteropStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc2a59f2a_e077_5d30_a1bd_cf84dd09ee39);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenDeviceInteropStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub FromPointerPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        pointerpoint: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'UI_Input_Interop'*"]
pub struct PenDeviceInterop {}
impl PenDeviceInterop {
    #[doc = "*Required features: 'UI_Input_Interop'*"]
    pub fn FromPointerPoint<'a, Param0: ::windows::core::IntoParam<'a, super::PointerPoint>>(
        pointerpoint: Param0,
    ) -> ::windows::core::Result<::windows::Devices::Input::PenDevice> {
        Self::IPenDeviceInteropStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FromPointerPoint)(
                ::core::mem::transmute_copy(this),
                pointerpoint.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Devices::Input::PenDevice>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPenDeviceInteropStatics<
        R,
        F: FnOnce(&IPenDeviceInteropStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            PenDeviceInterop,
            IPenDeviceInteropStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for PenDeviceInterop {
    const NAME: &'static str = "Microsoft.UI.Input.Interop.PenDeviceInterop";
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
