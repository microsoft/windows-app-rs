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
pub struct IKnownResourceQualifierNameStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IKnownResourceQualifierNameStatics {
    type Vtable = IKnownResourceQualifierNameStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xdd6cdedc_559b_50c8_ac53_82fe21f915f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownResourceQualifierNameStatics_abi(
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
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceCandidate(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IResourceCandidate {
    type Vtable = IResourceCandidate_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x6c54bc0c_ef1e_57b8_b478_34fece737356);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceCandidate_abi(
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
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result_size__: *mut u32,
        result__: *mut *mut u8,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ResourceCandidateKind,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceCandidateFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IResourceCandidateFactory {
    type Vtable = IResourceCandidateFactory_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xbb2b30f8_c19b_5f43_88d9_69ad728a32f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceCandidateFactory_abi(
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
        kind: ResourceCandidateKind,
        data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        data_array_size: u32,
        data: *const u8,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceContext(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IResourceContext {
    type Vtable = IResourceContext_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x96fb48dc_f77d_55ff_af12_34861e3d4939);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceContext_abi(
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
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceLoader(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IResourceLoader {
    type Vtable = IResourceLoader_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xbc3f76bf_da46_54cd_8715_8b8aaf16eaac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceLoader_abi(
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
        resourceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        resourceuri: ::windows::core::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceLoaderFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IResourceLoaderFactory {
    type Vtable = IResourceLoaderFactory_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x871f83aa_fb34_50d6_b9b9_2c35f3ffc004);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceLoaderFactory_abi(
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
        filename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        filename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        resourcemap: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceLoaderStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IResourceLoaderStatics {
    type Vtable = IResourceLoaderStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xec9c894a_1466_5f2f_8eee_a70cbd2b51bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceLoaderStatics_abi(
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
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceManager(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IResourceManager {
    type Vtable = IResourceManager_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xac2291ef_81be_5c99_a0ae_bcee0180b8a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManager_abi(
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
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceManagerFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IResourceManagerFactory {
    type Vtable = IResourceManagerFactory_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xd6acf18f_458a_535b_a5c4_ac2dc4e49099);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManagerFactory_abi(
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
        filename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceMap(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IResourceMap {
    type Vtable = IResourceMap_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4abbd9bc_df4e_5c7b_812c_7e7bb0c22377);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceMap_abi(
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
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        reference: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        reference: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        resource: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        resource: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        context: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        index: u32,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        index: u32,
        context: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        resource: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        resource: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        context: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceNotFoundEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IResourceNotFoundEventArgs {
    type Vtable = IResourceNotFoundEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x64abb08b_e77d_5b26_830f_15941e0e8200);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceNotFoundEventArgs_abi(
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
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        candidate: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
pub struct KnownResourceQualifierName {}
impl KnownResourceQualifierName {
    pub fn Contrast() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownResourceQualifierNameStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Custom() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownResourceQualifierNameStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn DeviceFamily() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownResourceQualifierNameStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn HomeRegion() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownResourceQualifierNameStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Language() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownResourceQualifierNameStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn LayoutDirection() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownResourceQualifierNameStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Scale() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownResourceQualifierNameStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn TargetSize() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownResourceQualifierNameStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Theme() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownResourceQualifierNameStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn IKnownResourceQualifierNameStatics<
        R,
        F: FnOnce(&IKnownResourceQualifierNameStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            KnownResourceQualifierName,
            IKnownResourceQualifierNameStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for KnownResourceQualifierName {
    const NAME: &'static str =
        "Microsoft.Windows.ApplicationModel.Resources.KnownResourceQualifierName";
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct ResourceCandidate(pub ::windows::core::IInspectable);
impl ResourceCandidate {
    pub fn ValueAsString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ValueAsBytes(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<u8> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                ::windows::core::Array::<u8>::set_abi_len(&mut result__),
                &mut result__ as *mut _ as _,
            )
            .and_then(|| result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ResourceCandidateKind> {
        let this = self;
        unsafe {
            let mut result__: ResourceCandidateKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ResourceCandidateKind>(result__)
        }
    }
    pub fn QualifierValues(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IMapView<
            ::windows::core::HSTRING,
            ::windows::core::HSTRING,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IMapView<
                ::windows::core::HSTRING,
                ::windows::core::HSTRING,
            >>(result__)
        }
    }
    pub fn CreateInstance<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        kind: ResourceCandidateKind,
        data: Param1,
    ) -> ::windows::core::Result<ResourceCandidate> {
        Self::IResourceCandidateFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                kind,
                data.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ResourceCandidate>(result__)
        })
    }
    pub fn CreateInstance2(
        data: &[<u8 as ::windows::core::DefaultType>::DefaultType],
    ) -> ::windows::core::Result<ResourceCandidate> {
        Self::IResourceCandidateFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                data.len() as u32,
                ::core::mem::transmute(data.as_ptr()),
                &mut result__,
            )
            .from_abi::<ResourceCandidate>(result__)
        })
    }
    pub fn IResourceCandidateFactory<
        R,
        F: FnOnce(&IResourceCandidateFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            ResourceCandidate,
            IResourceCandidateFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ResourceCandidate {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.ApplicationModel.Resources.ResourceCandidate;{6c54bc0c-ef1e-57b8-b478-34fece737356})" ) ;
}
unsafe impl ::windows::core::Interface for ResourceCandidate {
    type Vtable = IResourceCandidate_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x6c54bc0c_ef1e_57b8_b478_34fece737356);
}
impl ::windows::core::RuntimeName for ResourceCandidate {
    const NAME: &'static str = "Microsoft.Windows.ApplicationModel.Resources.ResourceCandidate";
}
impl ::core::convert::From<ResourceCandidate> for ::windows::core::IUnknown {
    fn from(value: ResourceCandidate) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ResourceCandidate> for ::windows::core::IUnknown {
    fn from(value: &ResourceCandidate) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ResourceCandidate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ResourceCandidate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ResourceCandidate> for ::windows::core::IInspectable {
    fn from(value: ResourceCandidate) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ResourceCandidate> for ::windows::core::IInspectable {
    fn from(value: &ResourceCandidate) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ResourceCandidate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ResourceCandidate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ResourceCandidate {}
unsafe impl ::core::marker::Sync for ResourceCandidate {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct ResourceCandidateKind(pub i32);
impl ResourceCandidateKind {
    pub const Unknown: ResourceCandidateKind = ResourceCandidateKind(0i32);
    pub const String: ResourceCandidateKind = ResourceCandidateKind(1i32);
    pub const FilePath: ResourceCandidateKind = ResourceCandidateKind(2i32);
    pub const EmbeddedData: ResourceCandidateKind = ResourceCandidateKind(3i32);
}
impl ::core::convert::From<i32> for ResourceCandidateKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ResourceCandidateKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ResourceCandidateKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.ApplicationModel.Resources.ResourceCandidateKind;i4)",
    );
}
impl ::windows::core::DefaultType for ResourceCandidateKind {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct ResourceContext(pub ::windows::core::IInspectable);
impl ResourceContext {
    pub fn QualifierValues(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IMap<
            ::windows::core::HSTRING,
            ::windows::core::HSTRING,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IMap<
                ::windows::core::HSTRING,
                ::windows::core::HSTRING,
            >>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ResourceContext {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.ApplicationModel.Resources.ResourceContext;{96fb48dc-f77d-55ff-af12-34861e3d4939})" ) ;
}
unsafe impl ::windows::core::Interface for ResourceContext {
    type Vtable = IResourceContext_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x96fb48dc_f77d_55ff_af12_34861e3d4939);
}
impl ::windows::core::RuntimeName for ResourceContext {
    const NAME: &'static str = "Microsoft.Windows.ApplicationModel.Resources.ResourceContext";
}
impl ::core::convert::From<ResourceContext> for ::windows::core::IUnknown {
    fn from(value: ResourceContext) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ResourceContext> for ::windows::core::IUnknown {
    fn from(value: &ResourceContext) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ResourceContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ResourceContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ResourceContext> for ::windows::core::IInspectable {
    fn from(value: ResourceContext) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ResourceContext> for ::windows::core::IInspectable {
    fn from(value: &ResourceContext) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ResourceContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ResourceContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ResourceContext {}
unsafe impl ::core::marker::Sync for ResourceContext {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct ResourceLoader(pub ::windows::core::IInspectable);
impl ResourceLoader {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            ResourceLoader,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn GetString<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        resourceid: Param0,
    ) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                resourceid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn GetStringForUri<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Uri>,
    >(
        &self,
        resourceuri: Param0,
    ) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                resourceuri.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        filename: Param0,
    ) -> ::windows::core::Result<ResourceLoader> {
        Self::IResourceLoaderFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                filename.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ResourceLoader>(result__)
        })
    }
    pub fn CreateInstance2<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        filename: Param0,
        resourcemap: Param1,
    ) -> ::windows::core::Result<ResourceLoader> {
        Self::IResourceLoaderFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                filename.into_param().abi(),
                resourcemap.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ResourceLoader>(result__)
        })
    }
    pub fn GetDefaultResourceFilePath() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IResourceLoaderStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn IResourceLoaderFactory<
        R,
        F: FnOnce(&IResourceLoaderFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ResourceLoader, IResourceLoaderFactory> =
            ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IResourceLoaderStatics<
        R,
        F: FnOnce(&IResourceLoaderStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ResourceLoader, IResourceLoaderStatics> =
            ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ResourceLoader {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.ApplicationModel.Resources.ResourceLoader;{bc3f76bf-da46-54cd-8715-8b8aaf16eaac})" ) ;
}
unsafe impl ::windows::core::Interface for ResourceLoader {
    type Vtable = IResourceLoader_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xbc3f76bf_da46_54cd_8715_8b8aaf16eaac);
}
impl ::windows::core::RuntimeName for ResourceLoader {
    const NAME: &'static str = "Microsoft.Windows.ApplicationModel.Resources.ResourceLoader";
}
impl ::core::convert::From<ResourceLoader> for ::windows::core::IUnknown {
    fn from(value: ResourceLoader) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ResourceLoader> for ::windows::core::IUnknown {
    fn from(value: &ResourceLoader) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ResourceLoader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ResourceLoader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ResourceLoader> for ::windows::core::IInspectable {
    fn from(value: ResourceLoader) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ResourceLoader> for ::windows::core::IInspectable {
    fn from(value: &ResourceLoader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ResourceLoader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ResourceLoader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ResourceLoader {}
unsafe impl ::core::marker::Sync for ResourceLoader {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct ResourceManager(pub ::windows::core::IInspectable);
impl ResourceManager {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            ResourceManager,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn MainResourceMap(&self) -> ::windows::core::Result<ResourceMap> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ResourceMap>(result__)
        }
    }
    pub fn CreateResourceContext(&self) -> ::windows::core::Result<ResourceContext> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ResourceContext>(result__)
        }
    }
    pub fn ResourceNotFound<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<ResourceManager, ResourceNotFoundEventArgs>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveResourceNotFound<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        filename: Param0,
    ) -> ::windows::core::Result<ResourceManager> {
        Self::IResourceManagerFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                filename.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ResourceManager>(result__)
        })
    }
    pub fn IResourceManagerFactory<
        R,
        F: FnOnce(&IResourceManagerFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ResourceManager, IResourceManagerFactory> =
            ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ResourceManager {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.ApplicationModel.Resources.ResourceManager;{ac2291ef-81be-5c99-a0ae-bcee0180b8a8})" ) ;
}
unsafe impl ::windows::core::Interface for ResourceManager {
    type Vtable = IResourceManager_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xac2291ef_81be_5c99_a0ae_bcee0180b8a8);
}
impl ::windows::core::RuntimeName for ResourceManager {
    const NAME: &'static str = "Microsoft.Windows.ApplicationModel.Resources.ResourceManager";
}
impl ::core::convert::From<ResourceManager> for ::windows::core::IUnknown {
    fn from(value: ResourceManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ResourceManager> for ::windows::core::IUnknown {
    fn from(value: &ResourceManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ResourceManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ResourceManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ResourceManager> for ::windows::core::IInspectable {
    fn from(value: ResourceManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ResourceManager> for ::windows::core::IInspectable {
    fn from(value: &ResourceManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ResourceManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ResourceManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ResourceManager {}
unsafe impl ::core::marker::Sync for ResourceManager {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct ResourceMap(pub ::windows::core::IInspectable);
impl ResourceMap {
    pub fn ResourceCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn GetSubtree<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        reference: Param0,
    ) -> ::windows::core::Result<ResourceMap> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                reference.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ResourceMap>(result__)
        }
    }
    pub fn TryGetSubtree<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        reference: Param0,
    ) -> ::windows::core::Result<ResourceMap> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                reference.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ResourceMap>(result__)
        }
    }
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        resource: Param0,
    ) -> ::windows::core::Result<ResourceCandidate> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                resource.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ResourceCandidate>(result__)
        }
    }
    pub fn GetValueWithContext<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, ResourceContext>,
    >(
        &self,
        resource: Param0,
        context: Param1,
    ) -> ::windows::core::Result<ResourceCandidate> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                resource.into_param().abi(),
                context.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ResourceCandidate>(result__)
        }
    }
    pub fn GetValueByIndex(
        &self,
        index: u32,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IKeyValuePair<
            ::windows::core::HSTRING,
            ResourceCandidate,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                index,
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IKeyValuePair<
                ::windows::core::HSTRING,
                ResourceCandidate,
            >>(result__)
        }
    }
    pub fn GetValueByIndexWithContext<
        'a,
        Param1: ::windows::core::IntoParam<'a, ResourceContext>,
    >(
        &self,
        index: u32,
        context: Param1,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IKeyValuePair<
            ::windows::core::HSTRING,
            ResourceCandidate,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                index,
                context.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IKeyValuePair<
                ::windows::core::HSTRING,
                ResourceCandidate,
            >>(result__)
        }
    }
    pub fn TryGetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        resource: Param0,
    ) -> ::windows::core::Result<ResourceCandidate> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                resource.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ResourceCandidate>(result__)
        }
    }
    pub fn TryGetValueWithContext<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, ResourceContext>,
    >(
        &self,
        resource: Param0,
        context: Param1,
    ) -> ::windows::core::Result<ResourceCandidate> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                resource.into_param().abi(),
                context.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ResourceCandidate>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ResourceMap {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.ApplicationModel.Resources.ResourceMap;{4abbd9bc-df4e-5c7b-812c-7e7bb0c22377})" ) ;
}
unsafe impl ::windows::core::Interface for ResourceMap {
    type Vtable = IResourceMap_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4abbd9bc_df4e_5c7b_812c_7e7bb0c22377);
}
impl ::windows::core::RuntimeName for ResourceMap {
    const NAME: &'static str = "Microsoft.Windows.ApplicationModel.Resources.ResourceMap";
}
impl ::core::convert::From<ResourceMap> for ::windows::core::IUnknown {
    fn from(value: ResourceMap) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ResourceMap> for ::windows::core::IUnknown {
    fn from(value: &ResourceMap) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ResourceMap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ResourceMap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ResourceMap> for ::windows::core::IInspectable {
    fn from(value: ResourceMap) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ResourceMap> for ::windows::core::IInspectable {
    fn from(value: &ResourceMap) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ResourceMap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ResourceMap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ResourceMap {}
unsafe impl ::core::marker::Sync for ResourceMap {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct ResourceNotFoundEventArgs(pub ::windows::core::IInspectable);
impl ResourceNotFoundEventArgs {
    pub fn Context(&self) -> ::windows::core::Result<ResourceContext> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ResourceContext>(result__)
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetResolvedCandidate<'a, Param0: ::windows::core::IntoParam<'a, ResourceCandidate>>(
        &self,
        candidate: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                candidate.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ResourceNotFoundEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.ApplicationModel.Resources.ResourceNotFoundEventArgs;{64abb08b-e77d-5b26-830f-15941e0e8200})" ) ;
}
unsafe impl ::windows::core::Interface for ResourceNotFoundEventArgs {
    type Vtable = IResourceNotFoundEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x64abb08b_e77d_5b26_830f_15941e0e8200);
}
impl ::windows::core::RuntimeName for ResourceNotFoundEventArgs {
    const NAME: &'static str =
        "Microsoft.Windows.ApplicationModel.Resources.ResourceNotFoundEventArgs";
}
impl ::core::convert::From<ResourceNotFoundEventArgs> for ::windows::core::IUnknown {
    fn from(value: ResourceNotFoundEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ResourceNotFoundEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ResourceNotFoundEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ResourceNotFoundEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a ResourceNotFoundEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ResourceNotFoundEventArgs> for ::windows::core::IInspectable {
    fn from(value: ResourceNotFoundEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ResourceNotFoundEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ResourceNotFoundEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for ResourceNotFoundEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a ResourceNotFoundEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ResourceNotFoundEventArgs {}
unsafe impl ::core::marker::Sync for ResourceNotFoundEventArgs {}
