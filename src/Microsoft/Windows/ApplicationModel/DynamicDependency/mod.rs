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
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct AddPackageDependencyOptions(pub ::windows::core::IInspectable);
impl AddPackageDependencyOptions {
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
            AddPackageDependencyOptions,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Rank(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetRank(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn PrependIfRankCollision(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetPrependIfRankCollision(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AddPackageDependencyOptions {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.ApplicationModel.DynamicDependency.AddPackageDependencyOptions;{01b801fd-24e3-5e6b-9f1c-805ab410b604})" ) ;
}
unsafe impl ::windows::core::Interface for AddPackageDependencyOptions {
    type Vtable = IAddPackageDependencyOptions_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x01b801fd_24e3_5e6b_9f1c_805ab410b604);
}
impl ::windows::core::RuntimeName for AddPackageDependencyOptions {
    const NAME: &'static str =
        "Microsoft.Windows.ApplicationModel.DynamicDependency.AddPackageDependencyOptions";
}
impl ::core::convert::From<AddPackageDependencyOptions> for ::windows::core::IUnknown {
    fn from(value: AddPackageDependencyOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AddPackageDependencyOptions> for ::windows::core::IUnknown {
    fn from(value: &AddPackageDependencyOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AddPackageDependencyOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a AddPackageDependencyOptions
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AddPackageDependencyOptions> for ::windows::core::IInspectable {
    fn from(value: AddPackageDependencyOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AddPackageDependencyOptions> for ::windows::core::IInspectable {
    fn from(value: &AddPackageDependencyOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for AddPackageDependencyOptions
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a AddPackageDependencyOptions
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AddPackageDependencyOptions {}
unsafe impl ::core::marker::Sync for AddPackageDependencyOptions {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CreatePackageDependencyOptions(pub ::windows::core::IInspectable);
impl CreatePackageDependencyOptions {
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
            CreatePackageDependencyOptions,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Architectures(
        &self,
    ) -> ::windows::core::Result<PackageDependencyProcessorArchitectures> {
        let this = self;
        unsafe {
            let mut result__: PackageDependencyProcessorArchitectures = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PackageDependencyProcessorArchitectures>(result__)
        }
    }
    pub fn SetArchitectures(
        &self,
        value: PackageDependencyProcessorArchitectures,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn VerifyDependencyResolution(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetVerifyDependencyResolution(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn LifetimeArtifactKind(
        &self,
    ) -> ::windows::core::Result<PackageDependencyLifetimeArtifactKind> {
        let this = self;
        unsafe {
            let mut result__: PackageDependencyLifetimeArtifactKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PackageDependencyLifetimeArtifactKind>(result__)
        }
    }
    pub fn SetLifetimeArtifactKind(
        &self,
        value: PackageDependencyLifetimeArtifactKind,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn LifetimeArtifact(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetLifetimeArtifact<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CreatePackageDependencyOptions {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.ApplicationModel.DynamicDependency.CreatePackageDependencyOptions;{cdbb820f-3c69-55dc-a017-b4132574c5d6})" ) ;
}
unsafe impl ::windows::core::Interface for CreatePackageDependencyOptions {
    type Vtable = ICreatePackageDependencyOptions_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xcdbb820f_3c69_55dc_a017_b4132574c5d6);
}
impl ::windows::core::RuntimeName for CreatePackageDependencyOptions {
    const NAME: &'static str =
        "Microsoft.Windows.ApplicationModel.DynamicDependency.CreatePackageDependencyOptions";
}
impl ::core::convert::From<CreatePackageDependencyOptions> for ::windows::core::IUnknown {
    fn from(value: CreatePackageDependencyOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CreatePackageDependencyOptions> for ::windows::core::IUnknown {
    fn from(value: &CreatePackageDependencyOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CreatePackageDependencyOptions
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CreatePackageDependencyOptions
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CreatePackageDependencyOptions> for ::windows::core::IInspectable {
    fn from(value: CreatePackageDependencyOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CreatePackageDependencyOptions> for ::windows::core::IInspectable {
    fn from(value: &CreatePackageDependencyOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CreatePackageDependencyOptions
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CreatePackageDependencyOptions
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CreatePackageDependencyOptions {}
unsafe impl ::core::marker::Sync for CreatePackageDependencyOptions {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAddPackageDependencyOptions(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAddPackageDependencyOptions {
    type Vtable = IAddPackageDependencyOptions_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x01b801fd_24e3_5e6b_9f1c_805ab410b604);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAddPackageDependencyOptions_abi(
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
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICreatePackageDependencyOptions(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICreatePackageDependencyOptions {
    type Vtable = ICreatePackageDependencyOptions_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xcdbb820f_3c69_55dc_a017_b4132574c5d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreatePackageDependencyOptions_abi(
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
        result__: *mut PackageDependencyProcessorArchitectures,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: PackageDependencyProcessorArchitectures,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut PackageDependencyLifetimeArtifactKind,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: PackageDependencyLifetimeArtifactKind,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPackageDependency(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPackageDependency {
    type Vtable = IPackageDependency_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x32ae7b95_e358_5a48_9669_c97d85ad6556);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageDependency_abi(
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
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        options: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPackageDependencyContext(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPackageDependencyContext {
    type Vtable = IPackageDependencyContext_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x9902c35a_a3f5_5645_af0f_cdf9fca00d5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageDependencyContext_abi(
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
        result__: *mut PackageDependencyContextId,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPackageDependencyContextFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPackageDependencyContextFactory {
    type Vtable = IPackageDependencyContextFactory_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x9914f24f_bebf_516b_adab_5c3e8bf323f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageDependencyContextFactory_abi(
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
        contextid: PackageDependencyContextId,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPackageDependencyRankStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPackageDependencyRankStatics {
    type Vtable = IPackageDependencyRankStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x260583bd_a4ab_53fd_a190_c446bfdb5384);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageDependencyRankStatics_abi(
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
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPackageDependencyStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPackageDependencyStatics {
    type Vtable = IPackageDependencyStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x17b656e1_1a58_5f3c_84a8_4430f6e749c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageDependencyStatics_abi(
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
        id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        minversion: ::windows::ApplicationModel::PackageVersion,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        minversion: ::windows::ApplicationModel::PackageVersion,
        options: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        minversion: ::windows::ApplicationModel::PackageVersion,
        options: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct PackageDependency(pub ::windows::core::IInspectable);
impl PackageDependency {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
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
    pub fn Delete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Add(&self) -> ::windows::core::Result<PackageDependencyContext> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PackageDependencyContext>(result__)
        }
    }
    pub fn Add2<'a, Param0: ::windows::core::IntoParam<'a, AddPackageDependencyOptions>>(
        &self,
        options: Param0,
    ) -> ::windows::core::Result<PackageDependencyContext> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                options.into_param().abi(),
                &mut result__,
            )
            .from_abi::<PackageDependencyContext>(result__)
        }
    }
    pub fn GetFromId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        id: Param0,
    ) -> ::windows::core::Result<PackageDependency> {
        Self::IPackageDependencyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                id.into_param().abi(),
                &mut result__,
            )
            .from_abi::<PackageDependency>(result__)
        })
    }
    pub fn GetFromIdForSystem<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        id: Param0,
    ) -> ::windows::core::Result<PackageDependency> {
        Self::IPackageDependencyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                id.into_param().abi(),
                &mut result__,
            )
            .from_abi::<PackageDependency>(result__)
        })
    }
    pub fn Create<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, ::windows::ApplicationModel::PackageVersion>,
    >(
        packagefamilyname: Param0,
        minversion: Param1,
    ) -> ::windows::core::Result<PackageDependency> {
        Self::IPackageDependencyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                packagefamilyname.into_param().abi(),
                minversion.into_param().abi(),
                &mut result__,
            )
            .from_abi::<PackageDependency>(result__)
        })
    }
    pub fn Create2<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, ::windows::ApplicationModel::PackageVersion>,
        Param2: ::windows::core::IntoParam<'a, CreatePackageDependencyOptions>,
    >(
        packagefamilyname: Param0,
        minversion: Param1,
        options: Param2,
    ) -> ::windows::core::Result<PackageDependency> {
        Self::IPackageDependencyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                packagefamilyname.into_param().abi(),
                minversion.into_param().abi(),
                options.into_param().abi(),
                &mut result__,
            )
            .from_abi::<PackageDependency>(result__)
        })
    }
    pub fn CreateForSystem<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, ::windows::ApplicationModel::PackageVersion>,
        Param2: ::windows::core::IntoParam<'a, CreatePackageDependencyOptions>,
    >(
        packagefamilyname: Param0,
        minversion: Param1,
        options: Param2,
    ) -> ::windows::core::Result<PackageDependency> {
        Self::IPackageDependencyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                packagefamilyname.into_param().abi(),
                minversion.into_param().abi(),
                options.into_param().abi(),
                &mut result__,
            )
            .from_abi::<PackageDependency>(result__)
        })
    }
    pub fn GenerationId() -> ::windows::core::Result<u32> {
        Self::IPackageDependencyStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        })
    }
    pub fn IPackageDependencyStatics<
        R,
        F: FnOnce(&IPackageDependencyStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            PackageDependency,
            IPackageDependencyStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PackageDependency {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.ApplicationModel.DynamicDependency.PackageDependency;{32ae7b95-e358-5a48-9669-c97d85ad6556})" ) ;
}
unsafe impl ::windows::core::Interface for PackageDependency {
    type Vtable = IPackageDependency_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x32ae7b95_e358_5a48_9669_c97d85ad6556);
}
impl ::windows::core::RuntimeName for PackageDependency {
    const NAME: &'static str =
        "Microsoft.Windows.ApplicationModel.DynamicDependency.PackageDependency";
}
impl ::core::convert::From<PackageDependency> for ::windows::core::IUnknown {
    fn from(value: PackageDependency) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PackageDependency> for ::windows::core::IUnknown {
    fn from(value: &PackageDependency) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PackageDependency {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PackageDependency {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PackageDependency> for ::windows::core::IInspectable {
    fn from(value: PackageDependency) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PackageDependency> for ::windows::core::IInspectable {
    fn from(value: &PackageDependency) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PackageDependency {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PackageDependency {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PackageDependency {}
unsafe impl ::core::marker::Sync for PackageDependency {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct PackageDependencyContext(pub ::windows::core::IInspectable);
impl PackageDependencyContext {
    pub fn ContextId(&self) -> ::windows::core::Result<PackageDependencyContextId> {
        let this = self;
        unsafe {
            let mut result__: PackageDependencyContextId = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PackageDependencyContextId>(result__)
        }
    }
    pub fn PackageDependencyId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
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
    pub fn PackageFullName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Remove(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn CreateInstance<
        'a,
        Param0: ::windows::core::IntoParam<'a, PackageDependencyContextId>,
    >(
        contextid: Param0,
    ) -> ::windows::core::Result<PackageDependencyContext> {
        Self::IPackageDependencyContextFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                contextid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<PackageDependencyContext>(result__)
        })
    }
    pub fn IPackageDependencyContextFactory<
        R,
        F: FnOnce(&IPackageDependencyContextFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            PackageDependencyContext,
            IPackageDependencyContextFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PackageDependencyContext {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.ApplicationModel.DynamicDependency.PackageDependencyContext;{9902c35a-a3f5-5645-af0f-cdf9fca00d5e})" ) ;
}
unsafe impl ::windows::core::Interface for PackageDependencyContext {
    type Vtable = IPackageDependencyContext_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x9902c35a_a3f5_5645_af0f_cdf9fca00d5e);
}
impl ::windows::core::RuntimeName for PackageDependencyContext {
    const NAME: &'static str =
        "Microsoft.Windows.ApplicationModel.DynamicDependency.PackageDependencyContext";
}
impl ::core::convert::From<PackageDependencyContext> for ::windows::core::IUnknown {
    fn from(value: PackageDependencyContext) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PackageDependencyContext> for ::windows::core::IUnknown {
    fn from(value: &PackageDependencyContext) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PackageDependencyContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a PackageDependencyContext
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PackageDependencyContext> for ::windows::core::IInspectable {
    fn from(value: PackageDependencyContext) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PackageDependencyContext> for ::windows::core::IInspectable {
    fn from(value: &PackageDependencyContext) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for PackageDependencyContext
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a PackageDependencyContext
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PackageDependencyContext {}
unsafe impl ::core::marker::Sync for PackageDependencyContext {}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct PackageDependencyContextId {
    pub Id: u64,
}
impl PackageDependencyContextId {}
impl ::core::default::Default for PackageDependencyContextId {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PackageDependencyContextId {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PackageDependencyContextId")
            .field("Id", &self.Id)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PackageDependencyContextId {
    fn eq(&self, other: &Self) -> bool {
        self.Id == other.Id
    }
}
impl ::core::cmp::Eq for PackageDependencyContextId {}
unsafe impl ::windows::core::Abi for PackageDependencyContextId {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PackageDependencyContextId {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"struct(Microsoft.Windows.ApplicationModel.DynamicDependency.PackageDependencyContextId;u8)" ) ;
}
impl ::windows::core::DefaultType for PackageDependencyContextId {
    type DefaultType = Self;
}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct PackageDependencyLifetimeArtifactKind(pub i32);
impl PackageDependencyLifetimeArtifactKind {
    pub const Process: PackageDependencyLifetimeArtifactKind =
        PackageDependencyLifetimeArtifactKind(0i32);
    pub const FilePath: PackageDependencyLifetimeArtifactKind =
        PackageDependencyLifetimeArtifactKind(1i32);
    pub const RegistryKey: PackageDependencyLifetimeArtifactKind =
        PackageDependencyLifetimeArtifactKind(2i32);
}
impl ::core::convert::From<i32> for PackageDependencyLifetimeArtifactKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PackageDependencyLifetimeArtifactKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PackageDependencyLifetimeArtifactKind {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"enum(Microsoft.Windows.ApplicationModel.DynamicDependency.PackageDependencyLifetimeArtifactKind;i4)" ) ;
}
impl ::windows::core::DefaultType for PackageDependencyLifetimeArtifactKind {
    type DefaultType = Self;
}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct PackageDependencyProcessorArchitectures(pub u32);
impl PackageDependencyProcessorArchitectures {
    pub const None: PackageDependencyProcessorArchitectures =
        PackageDependencyProcessorArchitectures(0u32);
    pub const Neutral: PackageDependencyProcessorArchitectures =
        PackageDependencyProcessorArchitectures(1u32);
    pub const X86: PackageDependencyProcessorArchitectures =
        PackageDependencyProcessorArchitectures(2u32);
    pub const X64: PackageDependencyProcessorArchitectures =
        PackageDependencyProcessorArchitectures(4u32);
    pub const Arm: PackageDependencyProcessorArchitectures =
        PackageDependencyProcessorArchitectures(8u32);
    pub const Arm64: PackageDependencyProcessorArchitectures =
        PackageDependencyProcessorArchitectures(16u32);
    pub const X86OnArm64: PackageDependencyProcessorArchitectures =
        PackageDependencyProcessorArchitectures(32u32);
}
impl ::core::convert::From<u32> for PackageDependencyProcessorArchitectures {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PackageDependencyProcessorArchitectures {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PackageDependencyProcessorArchitectures {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"enum(Microsoft.Windows.ApplicationModel.DynamicDependency.PackageDependencyProcessorArchitectures;u4)" ) ;
}
impl ::windows::core::DefaultType for PackageDependencyProcessorArchitectures {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for PackageDependencyProcessorArchitectures {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for PackageDependencyProcessorArchitectures {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for PackageDependencyProcessorArchitectures {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for PackageDependencyProcessorArchitectures {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for PackageDependencyProcessorArchitectures {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub struct PackageDependencyRank {}
impl PackageDependencyRank {
    pub fn Default() -> ::windows::core::Result<i32> {
        Self::IPackageDependencyRankStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        })
    }
    pub fn IPackageDependencyRankStatics<
        R,
        F: FnOnce(&IPackageDependencyRankStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            PackageDependencyRank,
            IPackageDependencyRankStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for PackageDependencyRank {
    const NAME: &'static str =
        "Microsoft.Windows.ApplicationModel.DynamicDependency.PackageDependencyRank";
}
