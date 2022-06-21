#[doc(hidden)]
#[repr(transparent)]
pub struct IKnownResourceQualifierNameStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKnownResourceQualifierNameStatics {
    type Vtable = IKnownResourceQualifierNameStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xdd6cdedc_559b_50c8_ac53_82fe21f915f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownResourceQualifierNameStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Contrast: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub Custom: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub DeviceFamily: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub HomeRegion: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub Language: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub LayoutDirection: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub Scale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub TargetSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub Theme: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceCandidate(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IResourceCandidate {
    type Vtable = IResourceCandidate_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x6c54bc0c_ef1e_57b8_b478_34fece737356);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceCandidate_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ValueAsString: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub ValueAsBytes: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut u8,
    ) -> ::windows::core::HRESULT,
    pub Kind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ResourceCandidateKind,
    ) -> ::windows::core::HRESULT,
    pub QualifierValues: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceCandidateFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IResourceCandidateFactory {
    type Vtable = IResourceCandidateFactory_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xbb2b30f8_c19b_5f43_88d9_69ad728a32f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceCandidateFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        kind: ResourceCandidateKind,
        data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub CreateInstance2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        data_array_size: u32,
        data: *const u8,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
#[repr(transparent)]
pub struct IResourceContext(::windows::core::IUnknown);
impl IResourceContext {
    #[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
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
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).QualifierValues)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Collections::IMap<
                ::windows::core::HSTRING,
                ::windows::core::HSTRING,
            >>(result__)
        }
    }
}
impl ::core::convert::From<IResourceContext> for ::windows::core::IUnknown {
    fn from(value: IResourceContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IResourceContext> for ::windows::core::IUnknown {
    fn from(value: &IResourceContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IResourceContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IResourceContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IResourceContext> for ::windows::core::IInspectable {
    fn from(value: IResourceContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IResourceContext> for ::windows::core::IInspectable {
    fn from(value: &IResourceContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IResourceContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IResourceContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IResourceContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IResourceContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IResourceContext {}
impl ::core::fmt::Debug for IResourceContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IResourceContext").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IResourceContext {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{96fb48dc-f77d-55ff-af12-34861e3d4939}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IResourceContext {
    type Vtable = IResourceContext_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x96fb48dc_f77d_55ff_af12_34861e3d4939);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceContext_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub QualifierValues: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceContext2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IResourceContext2 {
    type Vtable = IResourceContext2_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x7a3b1158_798c_5949_969d_03510b9ce6ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceContext2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceLoader(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IResourceLoader {
    type Vtable = IResourceLoader_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xbc3f76bf_da46_54cd_8715_8b8aaf16eaac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceLoader_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub GetString: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub GetStringForUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourceuri: ::windows::core::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceLoaderFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IResourceLoaderFactory {
    type Vtable = IResourceLoaderFactory_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x871f83aa_fb34_50d6_b9b9_2c35f3ffc004);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceLoaderFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        filename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub CreateInstance2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        filename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        resourcemap: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceLoaderStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IResourceLoaderStatics {
    type Vtable = IResourceLoaderStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xec9c894a_1466_5f2f_8eee_a70cbd2b51bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceLoaderStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub GetDefaultResourceFilePath: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
#[repr(transparent)]
pub struct IResourceManager(::windows::core::IUnknown);
impl IResourceManager {
    #[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
    pub fn MainResourceMap(&self) -> ::windows::core::Result<ResourceMap> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).MainResourceMap)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<ResourceMap>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
    pub fn CreateResourceContext(&self) -> ::windows::core::Result<ResourceContext> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).CreateResourceContext)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<ResourceContext>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
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
            let mut result__ =
                ::core::mem::MaybeUninit::<::windows::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).ResourceNotFound)(
                ::windows::core::Interface::as_raw(this),
                handler.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
    pub fn RemoveResourceNotFound<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveResourceNotFound)(
                ::windows::core::Interface::as_raw(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
}
impl ::core::convert::From<IResourceManager> for ::windows::core::IUnknown {
    fn from(value: IResourceManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IResourceManager> for ::windows::core::IUnknown {
    fn from(value: &IResourceManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IResourceManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IResourceManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IResourceManager> for ::windows::core::IInspectable {
    fn from(value: IResourceManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IResourceManager> for ::windows::core::IInspectable {
    fn from(value: &IResourceManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IResourceManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IResourceManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IResourceManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IResourceManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IResourceManager {}
impl ::core::fmt::Debug for IResourceManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IResourceManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IResourceManager {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{ac2291ef-81be-5c99-a0ae-bcee0180b8a8}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IResourceManager {
    type Vtable = IResourceManager_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xac2291ef_81be_5c99_a0ae_bcee0180b8a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManager_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub MainResourceMap: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub CreateResourceContext: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub ResourceNotFound: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveResourceNotFound: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceManager2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IResourceManager2 {
    type Vtable = IResourceManager2_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x7ec10160_a154_5c42_8268_30e306b1f585);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManager2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceManagerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IResourceManagerFactory {
    type Vtable = IResourceManagerFactory_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xd6acf18f_458a_535b_a5c4_ac2dc4e49099);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManagerFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        filename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceMap(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IResourceMap {
    type Vtable = IResourceMap_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4abbd9bc_df4e_5c7b_812c_7e7bb0c22377);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceMap_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ResourceCount: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub GetSubtree: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        reference: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub TryGetSubtree: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        reference: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resource: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetValueWithContext: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resource: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        context: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetValueByIndex: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        index: u32,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetValueByIndexWithContext: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        index: u32,
        context: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub TryGetValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resource: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub TryGetValueWithContext: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resource: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        context: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceNotFoundEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IResourceNotFoundEventArgs {
    type Vtable = IResourceNotFoundEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x64abb08b_e77d_5b26_830f_15941e0e8200);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceNotFoundEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Context: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetResolvedCandidate: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        candidate: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
pub struct KnownResourceQualifierName {}
impl KnownResourceQualifierName {
    #[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
    pub fn Contrast() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownResourceQualifierNameStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<
                ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
            >::zeroed();
            (::windows::core::Interface::vtable(this).Contrast)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
    pub fn Custom() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownResourceQualifierNameStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<
                ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
            >::zeroed();
            (::windows::core::Interface::vtable(this).Custom)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
    pub fn DeviceFamily() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownResourceQualifierNameStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<
                ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
            >::zeroed();
            (::windows::core::Interface::vtable(this).DeviceFamily)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
    pub fn HomeRegion() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownResourceQualifierNameStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<
                ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
            >::zeroed();
            (::windows::core::Interface::vtable(this).HomeRegion)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
    pub fn Language() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownResourceQualifierNameStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<
                ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
            >::zeroed();
            (::windows::core::Interface::vtable(this).Language)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
    pub fn LayoutDirection() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownResourceQualifierNameStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<
                ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
            >::zeroed();
            (::windows::core::Interface::vtable(this).LayoutDirection)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
    pub fn Scale() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownResourceQualifierNameStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<
                ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
            >::zeroed();
            (::windows::core::Interface::vtable(this).Scale)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
    pub fn TargetSize() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownResourceQualifierNameStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<
                ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
            >::zeroed();
            (::windows::core::Interface::vtable(this).TargetSize)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
    pub fn Theme() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownResourceQualifierNameStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<
                ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
            >::zeroed();
            (::windows::core::Interface::vtable(this).Theme)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
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
#[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
#[repr(transparent)]
pub struct ResourceCandidate(::windows::core::IUnknown);
impl ResourceCandidate {
    #[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
    pub fn ValueAsString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<
                ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
            >::zeroed();
            (::windows::core::Interface::vtable(this).ValueAsString)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
    pub fn ValueAsBytes(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::Array<u8>>::zeroed();
            (::windows::core::Interface::vtable(this).ValueAsBytes)(
                ::windows::core::Interface::as_raw(this),
                ::windows::core::Array::<u8>::set_abi_len(result__.assume_init_mut()),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        }
    }
    #[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ResourceCandidateKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ResourceCandidateKind>::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<ResourceCandidateKind>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
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
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).QualifierValues)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Collections::IMapView<
                ::windows::core::HSTRING,
                ::windows::core::HSTRING,
            >>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
    pub fn CreateInstance<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        kind: ResourceCandidateKind,
        data: Param1,
    ) -> ::windows::core::Result<ResourceCandidate> {
        Self::IResourceCandidateFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(
                ::windows::core::Interface::as_raw(this),
                kind,
                data.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<ResourceCandidate>(result__)
        })
    }
    #[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
    pub fn CreateInstance2(data: &[u8]) -> ::windows::core::Result<ResourceCandidate> {
        Self::IResourceCandidateFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance2)(
                ::windows::core::Interface::as_raw(this),
                data.len() as u32,
                ::core::mem::transmute(data.as_ptr()),
                result__.as_mut_ptr(),
            )
            .from_abi::<ResourceCandidate>(result__)
        })
    }
    #[doc(hidden)]
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
impl ::core::clone::Clone for ResourceCandidate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ResourceCandidate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ResourceCandidate {}
impl ::core::fmt::Debug for ResourceCandidate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceCandidate").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ResourceCandidate {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.ApplicationModel.Resources.ResourceCandidate;{6c54bc0c-ef1e-57b8-b478-34fece737356})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ResourceCandidate {
    type Vtable = IResourceCandidate_Vtbl;
    const IID: ::windows::core::GUID = <IResourceCandidate as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ResourceCandidate {
    const NAME: &'static str = "Microsoft.Windows.ApplicationModel.Resources.ResourceCandidate";
}
impl ::core::convert::From<ResourceCandidate> for ::windows::core::IUnknown {
    fn from(value: ResourceCandidate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ResourceCandidate> for ::windows::core::IUnknown {
    fn from(value: &ResourceCandidate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ResourceCandidate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ResourceCandidate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ResourceCandidate> for ::windows::core::IInspectable {
    fn from(value: ResourceCandidate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ResourceCandidate> for ::windows::core::IInspectable {
    fn from(value: &ResourceCandidate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ResourceCandidate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ResourceCandidate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ResourceCandidate {}
unsafe impl ::core::marker::Sync for ResourceCandidate {}
#[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ResourceCandidateKind(pub i32);
impl ResourceCandidateKind {
    pub const Unknown: Self = Self(0i32);
    pub const String: Self = Self(1i32);
    pub const FilePath: Self = Self(2i32);
    pub const EmbeddedData: Self = Self(3i32);
}
impl ::core::marker::Copy for ResourceCandidateKind {}
impl ::core::clone::Clone for ResourceCandidateKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ResourceCandidateKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ResourceCandidateKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for ResourceCandidateKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceCandidateKind")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ResourceCandidateKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.ApplicationModel.Resources.ResourceCandidateKind;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
#[repr(transparent)]
pub struct ResourceContext(::windows::core::IUnknown);
impl ResourceContext {
    #[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
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
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).QualifierValues)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Collections::IMap<
                ::windows::core::HSTRING,
                ::windows::core::HSTRING,
            >>(result__)
        }
    }
}
impl ::core::clone::Clone for ResourceContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ResourceContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ResourceContext {}
impl ::core::fmt::Debug for ResourceContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceContext").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ResourceContext {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.ApplicationModel.Resources.ResourceContext;{96fb48dc-f77d-55ff-af12-34861e3d4939})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ResourceContext {
    type Vtable = IResourceContext_Vtbl;
    const IID: ::windows::core::GUID = <IResourceContext as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ResourceContext {
    const NAME: &'static str = "Microsoft.Windows.ApplicationModel.Resources.ResourceContext";
}
impl ::core::convert::From<ResourceContext> for ::windows::core::IUnknown {
    fn from(value: ResourceContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ResourceContext> for ::windows::core::IUnknown {
    fn from(value: &ResourceContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ResourceContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ResourceContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ResourceContext> for ::windows::core::IInspectable {
    fn from(value: ResourceContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ResourceContext> for ::windows::core::IInspectable {
    fn from(value: &ResourceContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ResourceContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ResourceContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ResourceContext> for IResourceContext {
    type Error = ::windows::core::Error;
    fn try_from(value: ResourceContext) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ResourceContext> for IResourceContext {
    type Error = ::windows::core::Error;
    fn try_from(value: &ResourceContext) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IResourceContext> for ResourceContext {
    fn into_param(self) -> ::windows::core::Param<'a, IResourceContext> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IResourceContext> for &ResourceContext {
    fn into_param(self) -> ::windows::core::Param<'a, IResourceContext> {
        ::core::convert::TryInto::<IResourceContext>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ResourceContext {}
unsafe impl ::core::marker::Sync for ResourceContext {}
#[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
#[repr(transparent)]
pub struct ResourceLoader(::windows::core::IUnknown);
impl ResourceLoader {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            ResourceLoader,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
    pub fn GetString<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        resourceid: Param0,
    ) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<
                ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
            >::zeroed();
            (::windows::core::Interface::vtable(this).GetString)(
                ::windows::core::Interface::as_raw(this),
                resourceid.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
    pub fn GetStringForUri<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Uri>,
    >(
        &self,
        resourceuri: Param0,
    ) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<
                ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
            >::zeroed();
            (::windows::core::Interface::vtable(this).GetStringForUri)(
                ::windows::core::Interface::as_raw(this),
                resourceuri.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
    pub fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        filename: Param0,
    ) -> ::windows::core::Result<ResourceLoader> {
        Self::IResourceLoaderFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(
                ::windows::core::Interface::as_raw(this),
                filename.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<ResourceLoader>(result__)
        })
    }
    #[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
    pub fn CreateInstance2<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        filename: Param0,
        resourcemap: Param1,
    ) -> ::windows::core::Result<ResourceLoader> {
        Self::IResourceLoaderFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance2)(
                ::windows::core::Interface::as_raw(this),
                filename.into_param().abi(),
                resourcemap.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<ResourceLoader>(result__)
        })
    }
    #[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
    pub fn GetDefaultResourceFilePath() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IResourceLoaderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<
                ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
            >::zeroed();
            (::windows::core::Interface::vtable(this).GetDefaultResourceFilePath)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
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
    #[doc(hidden)]
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
impl ::core::clone::Clone for ResourceLoader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ResourceLoader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ResourceLoader {}
impl ::core::fmt::Debug for ResourceLoader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceLoader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ResourceLoader {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.ApplicationModel.Resources.ResourceLoader;{bc3f76bf-da46-54cd-8715-8b8aaf16eaac})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ResourceLoader {
    type Vtable = IResourceLoader_Vtbl;
    const IID: ::windows::core::GUID = <IResourceLoader as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ResourceLoader {
    const NAME: &'static str = "Microsoft.Windows.ApplicationModel.Resources.ResourceLoader";
}
impl ::core::convert::From<ResourceLoader> for ::windows::core::IUnknown {
    fn from(value: ResourceLoader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ResourceLoader> for ::windows::core::IUnknown {
    fn from(value: &ResourceLoader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ResourceLoader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ResourceLoader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ResourceLoader> for ::windows::core::IInspectable {
    fn from(value: ResourceLoader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ResourceLoader> for ::windows::core::IInspectable {
    fn from(value: &ResourceLoader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ResourceLoader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ResourceLoader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ResourceLoader {}
unsafe impl ::core::marker::Sync for ResourceLoader {}
#[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
#[repr(transparent)]
pub struct ResourceManager(::windows::core::IUnknown);
impl ResourceManager {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            ResourceManager,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
    pub fn MainResourceMap(&self) -> ::windows::core::Result<ResourceMap> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).MainResourceMap)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<ResourceMap>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
    pub fn CreateResourceContext(&self) -> ::windows::core::Result<ResourceContext> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).CreateResourceContext)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<ResourceContext>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
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
            let mut result__ =
                ::core::mem::MaybeUninit::<::windows::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).ResourceNotFound)(
                ::windows::core::Interface::as_raw(this),
                handler.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
    pub fn RemoveResourceNotFound<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveResourceNotFound)(
                ::windows::core::Interface::as_raw(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
    pub fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        filename: Param0,
    ) -> ::windows::core::Result<ResourceManager> {
        Self::IResourceManagerFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(
                ::windows::core::Interface::as_raw(this),
                filename.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<ResourceManager>(result__)
        })
    }
    #[doc(hidden)]
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
impl ::core::clone::Clone for ResourceManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ResourceManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ResourceManager {}
impl ::core::fmt::Debug for ResourceManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ResourceManager {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.ApplicationModel.Resources.ResourceManager;{ac2291ef-81be-5c99-a0ae-bcee0180b8a8})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ResourceManager {
    type Vtable = IResourceManager_Vtbl;
    const IID: ::windows::core::GUID = <IResourceManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ResourceManager {
    const NAME: &'static str = "Microsoft.Windows.ApplicationModel.Resources.ResourceManager";
}
impl ::core::convert::From<ResourceManager> for ::windows::core::IUnknown {
    fn from(value: ResourceManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ResourceManager> for ::windows::core::IUnknown {
    fn from(value: &ResourceManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ResourceManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ResourceManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ResourceManager> for ::windows::core::IInspectable {
    fn from(value: ResourceManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ResourceManager> for ::windows::core::IInspectable {
    fn from(value: &ResourceManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ResourceManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ResourceManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ResourceManager> for IResourceManager {
    type Error = ::windows::core::Error;
    fn try_from(value: ResourceManager) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ResourceManager> for IResourceManager {
    type Error = ::windows::core::Error;
    fn try_from(value: &ResourceManager) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IResourceManager> for ResourceManager {
    fn into_param(self) -> ::windows::core::Param<'a, IResourceManager> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IResourceManager> for &ResourceManager {
    fn into_param(self) -> ::windows::core::Param<'a, IResourceManager> {
        ::core::convert::TryInto::<IResourceManager>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ResourceManager {}
unsafe impl ::core::marker::Sync for ResourceManager {}
#[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
#[repr(transparent)]
pub struct ResourceMap(::windows::core::IUnknown);
impl ResourceMap {
    #[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
    pub fn ResourceCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows::core::Interface::vtable(this).ResourceCount)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
    pub fn GetSubtree<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        reference: Param0,
    ) -> ::windows::core::Result<ResourceMap> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).GetSubtree)(
                ::windows::core::Interface::as_raw(this),
                reference.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<ResourceMap>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
    pub fn TryGetSubtree<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        reference: Param0,
    ) -> ::windows::core::Result<ResourceMap> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).TryGetSubtree)(
                ::windows::core::Interface::as_raw(this),
                reference.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<ResourceMap>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        resource: Param0,
    ) -> ::windows::core::Result<ResourceCandidate> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).GetValue)(
                ::windows::core::Interface::as_raw(this),
                resource.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<ResourceCandidate>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
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
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).GetValueWithContext)(
                ::windows::core::Interface::as_raw(this),
                resource.into_param().abi(),
                context.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<ResourceCandidate>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
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
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).GetValueByIndex)(
                ::windows::core::Interface::as_raw(this),
                index,
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Collections::IKeyValuePair<
                ::windows::core::HSTRING,
                ResourceCandidate,
            >>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
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
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).GetValueByIndexWithContext)(
                ::windows::core::Interface::as_raw(this),
                index,
                context.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Collections::IKeyValuePair<
                ::windows::core::HSTRING,
                ResourceCandidate,
            >>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
    pub fn TryGetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        resource: Param0,
    ) -> ::windows::core::Result<ResourceCandidate> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).TryGetValue)(
                ::windows::core::Interface::as_raw(this),
                resource.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<ResourceCandidate>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
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
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).TryGetValueWithContext)(
                ::windows::core::Interface::as_raw(this),
                resource.into_param().abi(),
                context.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<ResourceCandidate>(result__)
        }
    }
}
impl ::core::clone::Clone for ResourceMap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ResourceMap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ResourceMap {}
impl ::core::fmt::Debug for ResourceMap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceMap").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ResourceMap {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.ApplicationModel.Resources.ResourceMap;{4abbd9bc-df4e-5c7b-812c-7e7bb0c22377})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ResourceMap {
    type Vtable = IResourceMap_Vtbl;
    const IID: ::windows::core::GUID = <IResourceMap as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ResourceMap {
    const NAME: &'static str = "Microsoft.Windows.ApplicationModel.Resources.ResourceMap";
}
impl ::core::convert::From<ResourceMap> for ::windows::core::IUnknown {
    fn from(value: ResourceMap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ResourceMap> for ::windows::core::IUnknown {
    fn from(value: &ResourceMap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ResourceMap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ResourceMap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ResourceMap> for ::windows::core::IInspectable {
    fn from(value: ResourceMap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ResourceMap> for ::windows::core::IInspectable {
    fn from(value: &ResourceMap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ResourceMap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ResourceMap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ResourceMap {}
unsafe impl ::core::marker::Sync for ResourceMap {}
#[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
#[repr(transparent)]
pub struct ResourceNotFoundEventArgs(::windows::core::IUnknown);
impl ResourceNotFoundEventArgs {
    #[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
    pub fn Context(&self) -> ::windows::core::Result<ResourceContext> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).Context)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<ResourceContext>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<
                ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
            >::zeroed();
            (::windows::core::Interface::vtable(this).Name)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
    pub fn SetResolvedCandidate<'a, Param0: ::windows::core::IntoParam<'a, ResourceCandidate>>(
        &self,
        candidate: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetResolvedCandidate)(
                ::windows::core::Interface::as_raw(this),
                candidate.into_param().abi(),
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for ResourceNotFoundEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ResourceNotFoundEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ResourceNotFoundEventArgs {}
impl ::core::fmt::Debug for ResourceNotFoundEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceNotFoundEventArgs")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ResourceNotFoundEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.ApplicationModel.Resources.ResourceNotFoundEventArgs;{64abb08b-e77d-5b26-830f-15941e0e8200})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ResourceNotFoundEventArgs {
    type Vtable = IResourceNotFoundEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        <IResourceNotFoundEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ResourceNotFoundEventArgs {
    const NAME: &'static str =
        "Microsoft.Windows.ApplicationModel.Resources.ResourceNotFoundEventArgs";
}
impl ::core::convert::From<ResourceNotFoundEventArgs> for ::windows::core::IUnknown {
    fn from(value: ResourceNotFoundEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ResourceNotFoundEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ResourceNotFoundEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ResourceNotFoundEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a ResourceNotFoundEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ResourceNotFoundEventArgs> for ::windows::core::IInspectable {
    fn from(value: ResourceNotFoundEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ResourceNotFoundEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ResourceNotFoundEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for ResourceNotFoundEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a ResourceNotFoundEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ResourceNotFoundEventArgs {}
unsafe impl ::core::marker::Sync for ResourceNotFoundEventArgs {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
