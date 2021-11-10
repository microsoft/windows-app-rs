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
pub struct IComponentConnector(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IComponentConnector {
    type Vtable = IComponentConnector_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xad401812_b091_51d0_b915_2d682cd2af10);
}
impl IComponentConnector {
    pub fn Connect<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(
        &self,
        connectionid: i32,
        target: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                connectionid,
                target.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn GetBindingConnector<
        'a,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        connectionid: i32,
        target: Param1,
    ) -> ::windows::core::Result<IComponentConnector> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                connectionid,
                target.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IComponentConnector>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IComponentConnector {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{ad401812-b091-51d0-b915-2d682cd2af10}");
}
impl ::core::convert::From<IComponentConnector> for ::windows::core::IUnknown {
    fn from(value: IComponentConnector) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IComponentConnector> for ::windows::core::IUnknown {
    fn from(value: &IComponentConnector) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IComponentConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IComponentConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IComponentConnector> for ::windows::core::IInspectable {
    fn from(value: IComponentConnector) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IComponentConnector> for ::windows::core::IInspectable {
    fn from(value: &IComponentConnector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IComponentConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IComponentConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComponentConnector_abi(
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
        connectionid: i32,
        target: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        connectionid: i32,
        target: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct IDataTemplateComponent(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDataTemplateComponent {
    type Vtable = IDataTemplateComponent_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x1743ddf7_38ba_58c9_a2a6_b0ae28713bee);
}
impl IDataTemplateComponent {
    pub fn Recycle(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn ProcessBindings<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        item: Param0,
        itemindex: i32,
        phase: i32,
        nextphase: &mut i32,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                item.into_param().abi(),
                itemindex,
                phase,
                nextphase,
            )
            .ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IDataTemplateComponent {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{1743ddf7-38ba-58c9-a2a6-b0ae28713bee}");
}
impl ::core::convert::From<IDataTemplateComponent> for ::windows::core::IUnknown {
    fn from(value: IDataTemplateComponent) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IDataTemplateComponent> for ::windows::core::IUnknown {
    fn from(value: &IDataTemplateComponent) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDataTemplateComponent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDataTemplateComponent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IDataTemplateComponent> for ::windows::core::IInspectable {
    fn from(value: IDataTemplateComponent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDataTemplateComponent> for ::windows::core::IInspectable {
    fn from(value: &IDataTemplateComponent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IDataTemplateComponent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a IDataTemplateComponent
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataTemplateComponent_abi(
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
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        item: ::windows::core::RawPtr,
        itemindex: i32,
        phase: i32,
        nextphase: *mut i32,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMarkupExtension(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMarkupExtension {
    type Vtable = IMarkupExtension_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc355371e_091d_5136_af4a_baf5e00616bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMarkupExtension_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMarkupExtensionFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMarkupExtensionFactory {
    type Vtable = IMarkupExtensionFactory_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x20651afa_5f3a_5f0c_adb1_b6551f53a6a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMarkupExtensionFactory_abi(
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
        baseinterface: ::windows::core::RawPtr,
        innerinterface: *mut ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMarkupExtensionOverrides(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMarkupExtensionOverrides {
    type Vtable = IMarkupExtensionOverrides_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa12aa575_5d31_5b68_a30f_8495412a351d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMarkupExtensionOverrides_abi(
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
        serviceprovider: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct IProvideValueTarget(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IProvideValueTarget {
    type Vtable = IProvideValueTarget_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x3f01ff68_3efd_591d_a506_de13fcaabd83);
}
impl IProvideValueTarget {
    pub fn TargetObject(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn TargetProperty(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IProvideValueTarget {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{3f01ff68-3efd-591d-a506-de13fcaabd83}");
}
impl ::core::convert::From<IProvideValueTarget> for ::windows::core::IUnknown {
    fn from(value: IProvideValueTarget) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IProvideValueTarget> for ::windows::core::IUnknown {
    fn from(value: &IProvideValueTarget) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IProvideValueTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IProvideValueTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IProvideValueTarget> for ::windows::core::IInspectable {
    fn from(value: IProvideValueTarget) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IProvideValueTarget> for ::windows::core::IInspectable {
    fn from(value: &IProvideValueTarget) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IProvideValueTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IProvideValueTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvideValueTarget_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProvideValueTargetProperty(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IProvideValueTargetProperty {
    type Vtable = IProvideValueTargetProperty_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xce777b1f_b42e_59d1_870d_12fdf0629133);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvideValueTargetProperty_abi(
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
        result__: *mut ::core::mem::ManuallyDrop<::windows::UI::Xaml::Interop::TypeName>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::UI::Xaml::Interop::TypeName>,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct IRootObjectProvider(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRootObjectProvider {
    type Vtable = IRootObjectProvider_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x13d63599_352f_5eb8_81c1_bc62fb12d6da);
}
impl IRootObjectProvider {
    pub fn RootObject(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IRootObjectProvider {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{13d63599-352f-5eb8-81c1-bc62fb12d6da}");
}
impl ::core::convert::From<IRootObjectProvider> for ::windows::core::IUnknown {
    fn from(value: IRootObjectProvider) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IRootObjectProvider> for ::windows::core::IUnknown {
    fn from(value: &IRootObjectProvider) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRootObjectProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRootObjectProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IRootObjectProvider> for ::windows::core::IInspectable {
    fn from(value: IRootObjectProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRootObjectProvider> for ::windows::core::IInspectable {
    fn from(value: &IRootObjectProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IRootObjectProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IRootObjectProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRootObjectProvider_abi(
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
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct IUriContext(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUriContext {
    type Vtable = IUriContext_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xfb8605f6_8f05_52ee_a01c_3a9e118a6ea2);
}
impl IUriContext {
    pub fn BaseUri(&self) -> ::windows::core::Result<::windows::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Uri>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IUriContext {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{fb8605f6-8f05-52ee-a01c-3a9e118a6ea2}");
}
impl ::core::convert::From<IUriContext> for ::windows::core::IUnknown {
    fn from(value: IUriContext) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IUriContext> for ::windows::core::IUnknown {
    fn from(value: &IUriContext) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUriContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUriContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IUriContext> for ::windows::core::IInspectable {
    fn from(value: IUriContext) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUriContext> for ::windows::core::IInspectable {
    fn from(value: &IUriContext) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IUriContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IUriContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUriContext_abi(
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
pub struct IXamlBinaryWriter(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlBinaryWriter {
    type Vtable = IXamlBinaryWriter_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x8fb45e3b_e689_55bf_aa11_d83b1c1cdda1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlBinaryWriter_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlBinaryWriterStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlBinaryWriterStatics {
    type Vtable = IXamlBinaryWriterStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x774907fc_c846_517f_abcc_c3f7e8c3ffc9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlBinaryWriterStatics_abi(
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
        inputstreams: ::windows::core::RawPtr,
        outputstreams: ::windows::core::RawPtr,
        xamlmetadataprovider: ::windows::core::RawPtr,
        result__: *mut XamlBinaryWriterErrorInformation,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct IXamlBindScopeDiagnostics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlBindScopeDiagnostics {
    type Vtable = IXamlBindScopeDiagnostics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x3ea84e4e_fdfe_55a8_a561_edf5697846d7);
}
impl IXamlBindScopeDiagnostics {
    pub fn Disable(&self, linenumber: i32, columnnumber: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                linenumber,
                columnnumber,
            )
            .ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IXamlBindScopeDiagnostics {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{3ea84e4e-fdfe-55a8-a561-edf5697846d7}");
}
impl ::core::convert::From<IXamlBindScopeDiagnostics> for ::windows::core::IUnknown {
    fn from(value: IXamlBindScopeDiagnostics) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IXamlBindScopeDiagnostics> for ::windows::core::IUnknown {
    fn from(value: &IXamlBindScopeDiagnostics) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXamlBindScopeDiagnostics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a IXamlBindScopeDiagnostics
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IXamlBindScopeDiagnostics> for ::windows::core::IInspectable {
    fn from(value: IXamlBindScopeDiagnostics) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXamlBindScopeDiagnostics> for ::windows::core::IInspectable {
    fn from(value: &IXamlBindScopeDiagnostics) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for IXamlBindScopeDiagnostics
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a IXamlBindScopeDiagnostics
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlBindScopeDiagnostics_abi(
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
        linenumber: i32,
        columnnumber: i32,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlBindingHelper(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlBindingHelper {
    type Vtable = IXamlBindingHelper_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x607a9bf2_5a6d_5c89_a756_bb44f24f28f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlBindingHelper_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlBindingHelperStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlBindingHelperStatics {
    type Vtable = IXamlBindingHelperStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x93c7dad3_f9c2_5372_84dc_9e9c4661d083);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlBindingHelperStatics_abi(
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
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Input",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        target: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Input",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    )))]
    usize,
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Input",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        target: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Input",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        r#type: ::core::mem::ManuallyDrop<::windows::UI::Xaml::Interop::TypeName>,
        value: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        dependencyobject: ::windows::core::RawPtr,
        propertytoset: ::windows::core::RawPtr,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        dependencyobject: ::windows::core::RawPtr,
        propertytoset: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        dependencyobject: ::windows::core::RawPtr,
        propertytoset: ::windows::core::RawPtr,
        value: u16,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        dependencyobject: ::windows::core::RawPtr,
        propertytoset: ::windows::core::RawPtr,
        value: ::windows::Foundation::DateTime,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        dependencyobject: ::windows::core::RawPtr,
        propertytoset: ::windows::core::RawPtr,
        value: f64,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        dependencyobject: ::windows::core::RawPtr,
        propertytoset: ::windows::core::RawPtr,
        value: i32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        dependencyobject: ::windows::core::RawPtr,
        propertytoset: ::windows::core::RawPtr,
        value: u32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        dependencyobject: ::windows::core::RawPtr,
        propertytoset: ::windows::core::RawPtr,
        value: i64,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        dependencyobject: ::windows::core::RawPtr,
        propertytoset: ::windows::core::RawPtr,
        value: u64,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        dependencyobject: ::windows::core::RawPtr,
        propertytoset: ::windows::core::RawPtr,
        value: f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        dependencyobject: ::windows::core::RawPtr,
        propertytoset: ::windows::core::RawPtr,
        value: ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        dependencyobject: ::windows::core::RawPtr,
        propertytoset: ::windows::core::RawPtr,
        value: ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        dependencyobject: ::windows::core::RawPtr,
        propertytoset: ::windows::core::RawPtr,
        value: ::windows::Foundation::Size,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        dependencyobject: ::windows::core::RawPtr,
        propertytoset: ::windows::core::RawPtr,
        value: ::windows::Foundation::TimeSpan,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        dependencyobject: ::windows::core::RawPtr,
        propertytoset: ::windows::core::RawPtr,
        value: u8,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        dependencyobject: ::windows::core::RawPtr,
        propertytoset: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        dependencyobject: ::windows::core::RawPtr,
        propertytoset: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlMarkupHelper(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlMarkupHelper {
    type Vtable = IXamlMarkupHelper_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xcd677310_3b06_5a13_b31a_401849570858);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlMarkupHelper_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlMarkupHelperStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlMarkupHelperStatics {
    type Vtable = IXamlMarkupHelperStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xd9a0f6e3_c6cc_5cb6_8999_85788701f339);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlMarkupHelperStatics_abi(
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
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
);
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct IXamlMember(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlMember {
    type Vtable = IXamlMember_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xbf3a2913_5c63_50ec_8660_61809be7b9b9);
}
impl IXamlMember {
    pub fn IsAttachable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn IsDependencyProperty(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn IsReadOnly(&self) -> ::windows::core::Result<bool> {
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
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn TargetType(&self) -> ::windows::core::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IXamlType>(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IXamlType>(result__)
        }
    }
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(
        &self,
        instance: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                instance.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        instance: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                instance.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IXamlMember {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{bf3a2913-5c63-50ec-8660-61809be7b9b9}");
}
impl ::core::convert::From<IXamlMember> for ::windows::core::IUnknown {
    fn from(value: IXamlMember) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IXamlMember> for ::windows::core::IUnknown {
    fn from(value: &IXamlMember) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXamlMember {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXamlMember {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IXamlMember> for ::windows::core::IInspectable {
    fn from(value: IXamlMember) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXamlMember> for ::windows::core::IInspectable {
    fn from(value: &IXamlMember) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IXamlMember {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IXamlMember {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlMember_abi(
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
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
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
        instance: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        instance: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct IXamlMetadataProvider(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlMetadataProvider {
    type Vtable = IXamlMetadataProvider_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa96251f0_2214_5d53_8746_ce99a2593cd7);
}
impl IXamlMetadataProvider {
    pub fn GetXamlType<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::UI::Xaml::Interop::TypeName>,
    >(
        &self,
        r#type: Param0,
    ) -> ::windows::core::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                r#type.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IXamlType>(result__)
        }
    }
    pub fn GetXamlTypeByFullName<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        fullname: Param0,
    ) -> ::windows::core::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                fullname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IXamlType>(result__)
        }
    }
    pub fn GetXmlnsDefinitions(
        &self,
    ) -> ::windows::core::Result<::windows::core::Array<XmlnsDefinition>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<XmlnsDefinition> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                ::windows::core::Array::<XmlnsDefinition>::set_abi_len(&mut result__),
                &mut result__ as *mut _ as _,
            )
            .and_then(|| result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IXamlMetadataProvider {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{a96251f0-2214-5d53-8746-ce99a2593cd7}");
}
impl ::core::convert::From<IXamlMetadataProvider> for ::windows::core::IUnknown {
    fn from(value: IXamlMetadataProvider) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IXamlMetadataProvider> for ::windows::core::IUnknown {
    fn from(value: &IXamlMetadataProvider) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXamlMetadataProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXamlMetadataProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IXamlMetadataProvider> for ::windows::core::IInspectable {
    fn from(value: IXamlMetadataProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXamlMetadataProvider> for ::windows::core::IInspectable {
    fn from(value: &IXamlMetadataProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IXamlMetadataProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a IXamlMetadataProvider
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlMetadataProvider_abi(
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
        r#type: ::core::mem::ManuallyDrop<::windows::UI::Xaml::Interop::TypeName>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        fullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result_size__: *mut u32,
        result__: *mut *mut ::core::mem::ManuallyDrop<XmlnsDefinition>,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlReader(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlReader {
    type Vtable = IXamlReader_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x54ce54c8_38c6_50d9_ac98_4b03eddbde9f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlReader_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlReaderStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlReaderStatics {
    type Vtable = IXamlReaderStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x82a4cd9e_435e_5aeb_8c4f_300cece45cae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlReaderStatics_abi(
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
        xaml: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        xaml: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct IXamlType(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlType {
    type Vtable = IXamlType_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xd24219df_7ec9_57f1_a27b_6af251d9c5bc);
}
impl IXamlType {
    pub fn BaseType(&self) -> ::windows::core::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IXamlType>(result__)
        }
    }
    pub fn ContentProperty(&self) -> ::windows::core::Result<IXamlMember> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IXamlMember>(result__)
        }
    }
    pub fn FullName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
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
    pub fn IsArray(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn IsCollection(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn IsConstructible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn IsDictionary(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn IsMarkupExtension(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn IsBindable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn ItemType(&self) -> ::windows::core::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IXamlType>(result__)
        }
    }
    pub fn KeyType(&self) -> ::windows::core::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IXamlType>(result__)
        }
    }
    pub fn BoxedType(&self) -> ::windows::core::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IXamlType>(result__)
        }
    }
    pub fn UnderlyingType(
        &self,
    ) -> ::windows::core::Result<::windows::UI::Xaml::Interop::TypeName> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::UI::Xaml::Interop::TypeName> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Xaml::Interop::TypeName>(result__)
        }
    }
    pub fn ActivateInstance(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn CreateFromString<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn GetMember<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<IXamlMember> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IXamlMember>(result__)
        }
    }
    pub fn AddToVector<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        instance: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                instance.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn AddToMap<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param2: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        instance: Param0,
        key: Param1,
        value: Param2,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                instance.into_param().abi(),
                key.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn RunInitializer(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IXamlType {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{d24219df-7ec9-57f1-a27b-6af251d9c5bc}");
}
impl ::core::convert::From<IXamlType> for ::windows::core::IUnknown {
    fn from(value: IXamlType) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IXamlType> for ::windows::core::IUnknown {
    fn from(value: &IXamlType) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXamlType {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXamlType {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IXamlType> for ::windows::core::IInspectable {
    fn from(value: IXamlType) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXamlType> for ::windows::core::IInspectable {
    fn from(value: &IXamlType) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IXamlType {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IXamlType {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlType_abi(
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
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut bool,
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
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::UI::Xaml::Interop::TypeName>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        instance: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        instance: ::windows::core::RawPtr,
        key: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct IXamlTypeResolver(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlTypeResolver {
    type Vtable = IXamlTypeResolver_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x3fa15615_cacf_547f_b1ed_89dae8c67452);
}
impl IXamlTypeResolver {
    pub fn Resolve<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        qualifiedtypename: Param0,
    ) -> ::windows::core::Result<::windows::UI::Xaml::Interop::TypeName> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::UI::Xaml::Interop::TypeName> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                qualifiedtypename.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::UI::Xaml::Interop::TypeName>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IXamlTypeResolver {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{3fa15615-cacf-547f-b1ed-89dae8c67452}");
}
impl ::core::convert::From<IXamlTypeResolver> for ::windows::core::IUnknown {
    fn from(value: IXamlTypeResolver) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IXamlTypeResolver> for ::windows::core::IUnknown {
    fn from(value: &IXamlTypeResolver) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXamlTypeResolver {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXamlTypeResolver {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IXamlTypeResolver> for ::windows::core::IInspectable {
    fn from(value: IXamlTypeResolver) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXamlTypeResolver> for ::windows::core::IInspectable {
    fn from(value: &IXamlTypeResolver) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IXamlTypeResolver {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IXamlTypeResolver {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlTypeResolver_abi(
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
        qualifiedtypename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::core::mem::ManuallyDrop<::windows::UI::Xaml::Interop::TypeName>,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct MarkupExtension(pub ::windows::core::IInspectable);
impl MarkupExtension {
    pub fn new() -> ::windows::core::Result<MarkupExtension> {
        Self::IMarkupExtensionFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                ::core::ptr::null_mut(),
                &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _,
                &mut result__,
            )
            .from_abi::<MarkupExtension>(result__)
        })
    }
    pub fn IMarkupExtensionFactory<
        R,
        F: FnOnce(&IMarkupExtensionFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MarkupExtension, IMarkupExtensionFactory> =
            ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for MarkupExtension {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Markup.MarkupExtension;{c355371e-091d-5136-af4a-baf5e00616bd})",
    );
}
unsafe impl ::windows::core::Interface for MarkupExtension {
    type Vtable = IMarkupExtension_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc355371e_091d_5136_af4a_baf5e00616bd);
}
impl ::windows::core::RuntimeName for MarkupExtension {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.MarkupExtension";
}
impl ::core::convert::From<MarkupExtension> for ::windows::core::IUnknown {
    fn from(value: MarkupExtension) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MarkupExtension> for ::windows::core::IUnknown {
    fn from(value: &MarkupExtension) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MarkupExtension {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MarkupExtension {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MarkupExtension> for ::windows::core::IInspectable {
    fn from(value: MarkupExtension) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MarkupExtension> for ::windows::core::IInspectable {
    fn from(value: &MarkupExtension) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MarkupExtension {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MarkupExtension {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MarkupExtension {}
unsafe impl ::core::marker::Sync for MarkupExtension {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct ProvideValueTargetProperty(pub ::windows::core::IInspectable);
impl ProvideValueTargetProperty {
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
            ProvideValueTargetProperty,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
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
    pub fn Type(&self) -> ::windows::core::Result<::windows::UI::Xaml::Interop::TypeName> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::UI::Xaml::Interop::TypeName> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Xaml::Interop::TypeName>(result__)
        }
    }
    pub fn DeclaringType(&self) -> ::windows::core::Result<::windows::UI::Xaml::Interop::TypeName> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::UI::Xaml::Interop::TypeName> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Xaml::Interop::TypeName>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ProvideValueTargetProperty {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Markup.ProvideValueTargetProperty;{ce777b1f-b42e-59d1-870d-12fdf0629133})" ) ;
}
unsafe impl ::windows::core::Interface for ProvideValueTargetProperty {
    type Vtable = IProvideValueTargetProperty_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xce777b1f_b42e_59d1_870d_12fdf0629133);
}
impl ::windows::core::RuntimeName for ProvideValueTargetProperty {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.ProvideValueTargetProperty";
}
impl ::core::convert::From<ProvideValueTargetProperty> for ::windows::core::IUnknown {
    fn from(value: ProvideValueTargetProperty) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ProvideValueTargetProperty> for ::windows::core::IUnknown {
    fn from(value: &ProvideValueTargetProperty) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProvideValueTargetProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a ProvideValueTargetProperty
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ProvideValueTargetProperty> for ::windows::core::IInspectable {
    fn from(value: ProvideValueTargetProperty) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ProvideValueTargetProperty> for ::windows::core::IInspectable {
    fn from(value: &ProvideValueTargetProperty) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for ProvideValueTargetProperty
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a ProvideValueTargetProperty
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ProvideValueTargetProperty {}
unsafe impl ::core::marker::Sync for ProvideValueTargetProperty {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct XamlBinaryWriter(pub ::windows::core::IInspectable);
impl XamlBinaryWriter {
    pub fn Write<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::Collections::IVector<
                ::windows::Storage::Streams::IRandomAccessStream,
            >,
        >,
        Param1: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::Collections::IVector<
                ::windows::Storage::Streams::IRandomAccessStream,
            >,
        >,
        Param2: ::windows::core::IntoParam<'a, IXamlMetadataProvider>,
    >(
        inputstreams: Param0,
        outputstreams: Param1,
        xamlmetadataprovider: Param2,
    ) -> ::windows::core::Result<XamlBinaryWriterErrorInformation> {
        Self::IXamlBinaryWriterStatics(|this| unsafe {
            let mut result__: XamlBinaryWriterErrorInformation = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                inputstreams.into_param().abi(),
                outputstreams.into_param().abi(),
                xamlmetadataprovider.into_param().abi(),
                &mut result__,
            )
            .from_abi::<XamlBinaryWriterErrorInformation>(result__)
        })
    }
    pub fn IXamlBinaryWriterStatics<
        R,
        F: FnOnce(&IXamlBinaryWriterStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            XamlBinaryWriter,
            IXamlBinaryWriterStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for XamlBinaryWriter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Markup.XamlBinaryWriter;{8fb45e3b-e689-55bf-aa11-d83b1c1cdda1})",
    );
}
unsafe impl ::windows::core::Interface for XamlBinaryWriter {
    type Vtable = IXamlBinaryWriter_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x8fb45e3b_e689_55bf_aa11_d83b1c1cdda1);
}
impl ::windows::core::RuntimeName for XamlBinaryWriter {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.XamlBinaryWriter";
}
impl ::core::convert::From<XamlBinaryWriter> for ::windows::core::IUnknown {
    fn from(value: XamlBinaryWriter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XamlBinaryWriter> for ::windows::core::IUnknown {
    fn from(value: &XamlBinaryWriter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for XamlBinaryWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a XamlBinaryWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XamlBinaryWriter> for ::windows::core::IInspectable {
    fn from(value: XamlBinaryWriter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XamlBinaryWriter> for ::windows::core::IInspectable {
    fn from(value: &XamlBinaryWriter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for XamlBinaryWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a XamlBinaryWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for XamlBinaryWriter {}
unsafe impl ::core::marker::Sync for XamlBinaryWriter {}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct XamlBinaryWriterErrorInformation {
    pub InputStreamIndex: u32,
    pub LineNumber: u32,
    pub LinePosition: u32,
}
impl XamlBinaryWriterErrorInformation {}
impl ::core::default::Default for XamlBinaryWriterErrorInformation {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for XamlBinaryWriterErrorInformation {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("XamlBinaryWriterErrorInformation")
            .field("InputStreamIndex", &self.InputStreamIndex)
            .field("LineNumber", &self.LineNumber)
            .field("LinePosition", &self.LinePosition)
            .finish()
    }
}
impl ::core::cmp::PartialEq for XamlBinaryWriterErrorInformation {
    fn eq(&self, other: &Self) -> bool {
        self.InputStreamIndex == other.InputStreamIndex
            && self.LineNumber == other.LineNumber
            && self.LinePosition == other.LinePosition
    }
}
impl ::core::cmp::Eq for XamlBinaryWriterErrorInformation {}
unsafe impl ::windows::core::Abi for XamlBinaryWriterErrorInformation {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for XamlBinaryWriterErrorInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"struct(Microsoft.UI.Xaml.Markup.XamlBinaryWriterErrorInformation;u4;u4;u4)",
    );
}
impl ::windows::core::DefaultType for XamlBinaryWriterErrorInformation {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct XamlBindingHelper(pub ::windows::core::IInspectable);
impl XamlBindingHelper {
    #[cfg(feature = "UI_Dispatching")]
    pub fn DataTemplateComponentProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetDataTemplateComponent<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<IDataTemplateComponent> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IDataTemplateComponent>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetDataTemplateComponent<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
        Param1: ::windows::core::IntoParam<'a, IDataTemplateComponent>,
    >(
        element: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Input",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn SuspendRendering<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(
        target: Param0,
    ) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                target.into_param().abi(),
            )
            .ok()
        })
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Input",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn ResumeRendering<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(
        target: Param0,
    ) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                target.into_param().abi(),
            )
            .ok()
        })
    }
    pub fn ConvertValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::UI::Xaml::Interop::TypeName>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        r#type: Param0,
        value: Param1,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                r#type.into_param().abi(),
                value.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetPropertyFromString<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        dependencyobject: Param0,
        propertytoset: Param1,
        value: Param2,
    ) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetPropertyFromBoolean<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        dependencyobject: Param0,
        propertytoset: Param1,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetPropertyFromChar16<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        dependencyobject: Param0,
        propertytoset: Param1,
        value: u16,
    ) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetPropertyFromDateTime<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param2: ::windows::core::IntoParam<'a, ::windows::Foundation::DateTime>,
    >(
        dependencyobject: Param0,
        propertytoset: Param1,
        value: Param2,
    ) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetPropertyFromDouble<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        dependencyobject: Param0,
        propertytoset: Param1,
        value: f64,
    ) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetPropertyFromInt32<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        dependencyobject: Param0,
        propertytoset: Param1,
        value: i32,
    ) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetPropertyFromUInt32<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        dependencyobject: Param0,
        propertytoset: Param1,
        value: u32,
    ) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetPropertyFromInt64<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        dependencyobject: Param0,
        propertytoset: Param1,
        value: i64,
    ) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetPropertyFromUInt64<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        dependencyobject: Param0,
        propertytoset: Param1,
        value: u64,
    ) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetPropertyFromSingle<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        dependencyobject: Param0,
        propertytoset: Param1,
        value: f32,
    ) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetPropertyFromPoint<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param2: ::windows::core::IntoParam<'a, ::windows::Foundation::Point>,
    >(
        dependencyobject: Param0,
        propertytoset: Param1,
        value: Param2,
    ) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetPropertyFromRect<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param2: ::windows::core::IntoParam<'a, ::windows::Foundation::Rect>,
    >(
        dependencyobject: Param0,
        propertytoset: Param1,
        value: Param2,
    ) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetPropertyFromSize<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param2: ::windows::core::IntoParam<'a, ::windows::Foundation::Size>,
    >(
        dependencyobject: Param0,
        propertytoset: Param1,
        value: Param2,
    ) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).24)(
                ::core::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetPropertyFromTimeSpan<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param2: ::windows::core::IntoParam<'a, ::windows::Foundation::TimeSpan>,
    >(
        dependencyobject: Param0,
        propertytoset: Param1,
        value: Param2,
    ) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetPropertyFromByte<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        dependencyobject: Param0,
        propertytoset: Param1,
        value: u8,
    ) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).26)(
                ::core::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetPropertyFromUri<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param2: ::windows::core::IntoParam<'a, ::windows::Foundation::Uri>,
    >(
        dependencyobject: Param0,
        propertytoset: Param1,
        value: Param2,
    ) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).27)(
                ::core::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetPropertyFromObject<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param2: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        dependencyobject: Param0,
        propertytoset: Param1,
        value: Param2,
    ) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).28)(
                ::core::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    pub fn IXamlBindingHelperStatics<
        R,
        F: FnOnce(&IXamlBindingHelperStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            XamlBindingHelper,
            IXamlBindingHelperStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for XamlBindingHelper {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Markup.XamlBindingHelper;{607a9bf2-5a6d-5c89-a756-bb44f24f28f8})",
    );
}
unsafe impl ::windows::core::Interface for XamlBindingHelper {
    type Vtable = IXamlBindingHelper_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x607a9bf2_5a6d_5c89_a756_bb44f24f28f8);
}
impl ::windows::core::RuntimeName for XamlBindingHelper {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.XamlBindingHelper";
}
impl ::core::convert::From<XamlBindingHelper> for ::windows::core::IUnknown {
    fn from(value: XamlBindingHelper) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XamlBindingHelper> for ::windows::core::IUnknown {
    fn from(value: &XamlBindingHelper) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for XamlBindingHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a XamlBindingHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XamlBindingHelper> for ::windows::core::IInspectable {
    fn from(value: XamlBindingHelper) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XamlBindingHelper> for ::windows::core::IInspectable {
    fn from(value: &XamlBindingHelper) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for XamlBindingHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a XamlBindingHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for XamlBindingHelper {}
unsafe impl ::core::marker::Sync for XamlBindingHelper {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct XamlMarkupHelper(pub ::windows::core::IInspectable);
impl XamlMarkupHelper {
    #[cfg(feature = "UI_Dispatching")]
    pub fn UnloadObject<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<()> {
        Self::IXamlMarkupHelperStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
            )
            .ok()
        })
    }
    pub fn IXamlMarkupHelperStatics<
        R,
        F: FnOnce(&IXamlMarkupHelperStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            XamlMarkupHelper,
            IXamlMarkupHelperStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for XamlMarkupHelper {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Markup.XamlMarkupHelper;{cd677310-3b06-5a13-b31a-401849570858})",
    );
}
unsafe impl ::windows::core::Interface for XamlMarkupHelper {
    type Vtable = IXamlMarkupHelper_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xcd677310_3b06_5a13_b31a_401849570858);
}
impl ::windows::core::RuntimeName for XamlMarkupHelper {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.XamlMarkupHelper";
}
impl ::core::convert::From<XamlMarkupHelper> for ::windows::core::IUnknown {
    fn from(value: XamlMarkupHelper) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XamlMarkupHelper> for ::windows::core::IUnknown {
    fn from(value: &XamlMarkupHelper) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for XamlMarkupHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a XamlMarkupHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XamlMarkupHelper> for ::windows::core::IInspectable {
    fn from(value: XamlMarkupHelper) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XamlMarkupHelper> for ::windows::core::IInspectable {
    fn from(value: &XamlMarkupHelper) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for XamlMarkupHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a XamlMarkupHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for XamlMarkupHelper {}
unsafe impl ::core::marker::Sync for XamlMarkupHelper {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct XamlReader(pub ::windows::core::IInspectable);
impl XamlReader {
    pub fn Load<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        xaml: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IXamlReaderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                xaml.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn LoadWithInitialTemplateValidation<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        xaml: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IXamlReaderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                xaml.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn IXamlReaderStatics<R, F: FnOnce(&IXamlReaderStatics) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<XamlReader, IXamlReaderStatics> =
            ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for XamlReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Markup.XamlReader;{54ce54c8-38c6-50d9-ac98-4b03eddbde9f})",
    );
}
unsafe impl ::windows::core::Interface for XamlReader {
    type Vtable = IXamlReader_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x54ce54c8_38c6_50d9_ac98_4b03eddbde9f);
}
impl ::windows::core::RuntimeName for XamlReader {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.XamlReader";
}
impl ::core::convert::From<XamlReader> for ::windows::core::IUnknown {
    fn from(value: XamlReader) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XamlReader> for ::windows::core::IUnknown {
    fn from(value: &XamlReader) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for XamlReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a XamlReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XamlReader> for ::windows::core::IInspectable {
    fn from(value: XamlReader) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XamlReader> for ::windows::core::IInspectable {
    fn from(value: &XamlReader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for XamlReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a XamlReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for XamlReader {}
unsafe impl ::core::marker::Sync for XamlReader {}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
pub struct XmlnsDefinition {
    pub XmlNamespace: ::windows::core::HSTRING,
    pub Namespace: ::windows::core::HSTRING,
}
impl XmlnsDefinition {}
impl ::core::default::Default for XmlnsDefinition {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for XmlnsDefinition {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("XmlnsDefinition")
            .field("XmlNamespace", &self.XmlNamespace)
            .field("Namespace", &self.Namespace)
            .finish()
    }
}
impl ::core::cmp::PartialEq for XmlnsDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.XmlNamespace == other.XmlNamespace && self.Namespace == other.Namespace
    }
}
impl ::core::cmp::Eq for XmlnsDefinition {}
unsafe impl ::windows::core::Abi for XmlnsDefinition {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
unsafe impl ::windows::core::RuntimeType for XmlnsDefinition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"struct(Microsoft.UI.Xaml.Markup.XmlnsDefinition;string;string)",
    );
}
impl ::windows::core::DefaultType for XmlnsDefinition {
    type DefaultType = Self;
}
