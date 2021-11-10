#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IPenDeviceInteropStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPenDeviceInteropStatics {
    type Vtable = IPenDeviceInteropStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc2a59f2a_e077_5d30_a1bd_cf84dd09ee39);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenDeviceInteropStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        pointerpoint: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
pub struct PenDeviceInterop {}
impl PenDeviceInterop {
    pub fn FromPointerPoint<'a, Param0: ::windows::core::IntoParam<'a, super::PointerPoint>>(
        pointerpoint: Param0,
    ) -> ::windows::core::Result<::windows::Devices::Input::PenDevice> {
        Self::IPenDeviceInteropStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                pointerpoint.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Devices::Input::PenDevice>(result__)
        })
    }
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
