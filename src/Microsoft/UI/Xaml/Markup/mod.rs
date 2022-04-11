#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clashing_extern_declarations,
    clippy::all
)]
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
#[repr(transparent)]
pub struct IComponentConnector(::windows::core::IUnknown);
impl IComponentConnector {
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn Connect<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(
        &self,
        connectionid: i32,
        target: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Connect)(
                ::core::mem::transmute_copy(this),
                connectionid,
                target.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
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
            (::windows::core::Interface::vtable(this).GetBindingConnector)(
                ::core::mem::transmute_copy(this),
                connectionid,
                target.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IComponentConnector>(result__)
        }
    }
}
impl ::core::convert::From<IComponentConnector> for ::windows::core::IUnknown {
    fn from(value: IComponentConnector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComponentConnector> for ::windows::core::IUnknown {
    fn from(value: &IComponentConnector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IComponentConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IComponentConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IComponentConnector> for ::windows::core::IInspectable {
    fn from(value: IComponentConnector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComponentConnector> for ::windows::core::IInspectable {
    fn from(value: &IComponentConnector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IComponentConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IComponentConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IComponentConnector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComponentConnector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComponentConnector {}
impl ::core::fmt::Debug for IComponentConnector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComponentConnector").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IComponentConnector {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{ad401812-b091-51d0-b915-2d682cd2af10}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IComponentConnector {
    type Vtable = IComponentConnector_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xad401812_b091_51d0_b915_2d682cd2af10);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComponentConnector_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Connect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        connectionid: i32,
        target: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetBindingConnector: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        connectionid: i32,
        target: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
#[repr(transparent)]
pub struct IDataTemplateComponent(::windows::core::IUnknown);
impl IDataTemplateComponent {
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn Recycle(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Recycle)(::core::mem::transmute_copy(this))
                .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
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
            (::windows::core::Interface::vtable(this).ProcessBindings)(
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
impl ::core::convert::From<IDataTemplateComponent> for ::windows::core::IUnknown {
    fn from(value: IDataTemplateComponent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDataTemplateComponent> for ::windows::core::IUnknown {
    fn from(value: &IDataTemplateComponent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDataTemplateComponent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDataTemplateComponent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDataTemplateComponent> for ::windows::core::IInspectable {
    fn from(value: IDataTemplateComponent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDataTemplateComponent> for ::windows::core::IInspectable {
    fn from(value: &IDataTemplateComponent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IDataTemplateComponent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a IDataTemplateComponent
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDataTemplateComponent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDataTemplateComponent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataTemplateComponent {}
impl ::core::fmt::Debug for IDataTemplateComponent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataTemplateComponent")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IDataTemplateComponent {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{1743ddf7-38ba-58c9-a2a6-b0ae28713bee}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IDataTemplateComponent {
    type Vtable = IDataTemplateComponent_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x1743ddf7_38ba_58c9_a2a6_b0ae28713bee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataTemplateComponent_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Recycle:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ProcessBindings: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        item: *mut ::core::ffi::c_void,
        itemindex: i32,
        phase: i32,
        nextphase: *mut i32,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMarkupExtension(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMarkupExtension {
    type Vtable = IMarkupExtension_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc355371e_091d_5136_af4a_baf5e00616bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMarkupExtension_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMarkupExtensionFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMarkupExtensionFactory {
    type Vtable = IMarkupExtensionFactory_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x20651afa_5f3a_5f0c_adb1_b6551f53a6a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMarkupExtensionFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMarkupExtensionOverrides(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMarkupExtensionOverrides {
    type Vtable = IMarkupExtensionOverrides_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa12aa575_5d31_5b68_a30f_8495412a351d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMarkupExtensionOverrides_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ProvideValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub ProvideValueWithIXamlServiceProvider: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        serviceprovider: ::windows::core::RawPtr,
        result__: *mut *mut ::core::ffi::c_void,
    )
        -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
#[repr(transparent)]
pub struct IProvideValueTarget(::windows::core::IUnknown);
impl IProvideValueTarget {
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn TargetObject(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TargetObject)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn TargetProperty(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TargetProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
impl ::core::convert::From<IProvideValueTarget> for ::windows::core::IUnknown {
    fn from(value: IProvideValueTarget) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProvideValueTarget> for ::windows::core::IUnknown {
    fn from(value: &IProvideValueTarget) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IProvideValueTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IProvideValueTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IProvideValueTarget> for ::windows::core::IInspectable {
    fn from(value: IProvideValueTarget) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProvideValueTarget> for ::windows::core::IInspectable {
    fn from(value: &IProvideValueTarget) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IProvideValueTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IProvideValueTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IProvideValueTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProvideValueTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProvideValueTarget {}
impl ::core::fmt::Debug for IProvideValueTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProvideValueTarget").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IProvideValueTarget {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{3f01ff68-3efd-591d-a506-de13fcaabd83}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IProvideValueTarget {
    type Vtable = IProvideValueTarget_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x3f01ff68_3efd_591d_a506_de13fcaabd83);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvideValueTarget_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub TargetObject: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub TargetProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProvideValueTargetProperty(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IProvideValueTargetProperty {
    type Vtable = IProvideValueTargetProperty_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xce777b1f_b42e_59d1_870d_12fdf0629133);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvideValueTargetProperty_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub Type: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::UI::Xaml::Interop::TypeName>,
    ) -> ::windows::core::HRESULT,
    pub DeclaringType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::UI::Xaml::Interop::TypeName>,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
#[repr(transparent)]
pub struct IRootObjectProvider(::windows::core::IUnknown);
impl IRootObjectProvider {
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn RootObject(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RootObject)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
impl ::core::convert::From<IRootObjectProvider> for ::windows::core::IUnknown {
    fn from(value: IRootObjectProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRootObjectProvider> for ::windows::core::IUnknown {
    fn from(value: &IRootObjectProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRootObjectProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRootObjectProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IRootObjectProvider> for ::windows::core::IInspectable {
    fn from(value: IRootObjectProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRootObjectProvider> for ::windows::core::IInspectable {
    fn from(value: &IRootObjectProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IRootObjectProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IRootObjectProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRootObjectProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRootObjectProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRootObjectProvider {}
impl ::core::fmt::Debug for IRootObjectProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRootObjectProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IRootObjectProvider {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{13d63599-352f-5eb8-81c1-bc62fb12d6da}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IRootObjectProvider {
    type Vtable = IRootObjectProvider_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x13d63599_352f_5eb8_81c1_bc62fb12d6da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRootObjectProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub RootObject: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
#[repr(transparent)]
pub struct IUriContext(::windows::core::IUnknown);
impl IUriContext {
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn BaseUri(&self) -> ::windows::core::Result<::windows::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BaseUri)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Uri>(result__)
        }
    }
}
impl ::core::convert::From<IUriContext> for ::windows::core::IUnknown {
    fn from(value: IUriContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUriContext> for ::windows::core::IUnknown {
    fn from(value: &IUriContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUriContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUriContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IUriContext> for ::windows::core::IInspectable {
    fn from(value: IUriContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUriContext> for ::windows::core::IInspectable {
    fn from(value: &IUriContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IUriContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IUriContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUriContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUriContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUriContext {}
impl ::core::fmt::Debug for IUriContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUriContext").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IUriContext {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{fb8605f6-8f05-52ee-a01c-3a9e118a6ea2}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IUriContext {
    type Vtable = IUriContext_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xfb8605f6_8f05_52ee_a01c_3a9e118a6ea2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUriContext_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub BaseUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlBinaryWriter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXamlBinaryWriter {
    type Vtable = IXamlBinaryWriter_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x8fb45e3b_e689_55bf_aa11_d83b1c1cdda1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlBinaryWriter_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlBinaryWriterStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXamlBinaryWriterStatics {
    type Vtable = IXamlBinaryWriterStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x774907fc_c846_517f_abcc_c3f7e8c3ffc9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlBinaryWriterStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Write: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        inputstreams: ::windows::core::RawPtr,
        outputstreams: ::windows::core::RawPtr,
        xamlmetadataprovider: ::windows::core::RawPtr,
        result__: *mut XamlBinaryWriterErrorInformation,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
#[repr(transparent)]
pub struct IXamlBindScopeDiagnostics(::windows::core::IUnknown);
impl IXamlBindScopeDiagnostics {
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn Disable(&self, linenumber: i32, columnnumber: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Disable)(
                ::core::mem::transmute_copy(this),
                linenumber,
                columnnumber,
            )
            .ok()
        }
    }
}
impl ::core::convert::From<IXamlBindScopeDiagnostics> for ::windows::core::IUnknown {
    fn from(value: IXamlBindScopeDiagnostics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXamlBindScopeDiagnostics> for ::windows::core::IUnknown {
    fn from(value: &IXamlBindScopeDiagnostics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXamlBindScopeDiagnostics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a IXamlBindScopeDiagnostics
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXamlBindScopeDiagnostics> for ::windows::core::IInspectable {
    fn from(value: IXamlBindScopeDiagnostics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXamlBindScopeDiagnostics> for ::windows::core::IInspectable {
    fn from(value: &IXamlBindScopeDiagnostics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for IXamlBindScopeDiagnostics
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a IXamlBindScopeDiagnostics
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXamlBindScopeDiagnostics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXamlBindScopeDiagnostics {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXamlBindScopeDiagnostics {}
impl ::core::fmt::Debug for IXamlBindScopeDiagnostics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXamlBindScopeDiagnostics")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IXamlBindScopeDiagnostics {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{3ea84e4e-fdfe-55a8-a561-edf5697846d7}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IXamlBindScopeDiagnostics {
    type Vtable = IXamlBindScopeDiagnostics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x3ea84e4e_fdfe_55a8_a561_edf5697846d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlBindScopeDiagnostics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Disable: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        linenumber: i32,
        columnnumber: i32,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlBindingHelper(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXamlBindingHelper {
    type Vtable = IXamlBindingHelper_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x607a9bf2_5a6d_5c89_a756_bb44f24f28f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlBindingHelper_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlBindingHelperStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXamlBindingHelperStatics {
    type Vtable = IXamlBindingHelperStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x93c7dad3_f9c2_5372_84dc_9e9c4661d083);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlBindingHelperStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub DataTemplateComponentProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetDataTemplateComponent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub SetDataTemplateComponent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub SuspendRendering: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub ResumeRendering: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub ConvertValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        r#type: ::core::mem::ManuallyDrop<::windows::UI::Xaml::Interop::TypeName>,
        value: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetPropertyFromString: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: ::windows::core::RawPtr,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetPropertyFromBoolean: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub SetPropertyFromChar16: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: ::windows::core::RawPtr,
        value: u16,
    ) -> ::windows::core::HRESULT,
    pub SetPropertyFromDateTime: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: ::windows::core::RawPtr,
        value: ::windows::Foundation::DateTime,
    ) -> ::windows::core::HRESULT,
    pub SetPropertyFromDouble: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: ::windows::core::RawPtr,
        value: f64,
    ) -> ::windows::core::HRESULT,
    pub SetPropertyFromInt32: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: ::windows::core::RawPtr,
        value: i32,
    ) -> ::windows::core::HRESULT,
    pub SetPropertyFromUInt32: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: ::windows::core::RawPtr,
        value: u32,
    ) -> ::windows::core::HRESULT,
    pub SetPropertyFromInt64: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: ::windows::core::RawPtr,
        value: i64,
    ) -> ::windows::core::HRESULT,
    pub SetPropertyFromUInt64: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: ::windows::core::RawPtr,
        value: u64,
    ) -> ::windows::core::HRESULT,
    pub SetPropertyFromSingle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: ::windows::core::RawPtr,
        value: f32,
    ) -> ::windows::core::HRESULT,
    pub SetPropertyFromPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: ::windows::core::RawPtr,
        value: ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
    pub SetPropertyFromRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: ::windows::core::RawPtr,
        value: ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    pub SetPropertyFromSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: ::windows::core::RawPtr,
        value: ::windows::Foundation::Size,
    ) -> ::windows::core::HRESULT,
    pub SetPropertyFromTimeSpan: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: ::windows::core::RawPtr,
        value: ::windows::Foundation::TimeSpan,
    ) -> ::windows::core::HRESULT,
    pub SetPropertyFromByte: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: ::windows::core::RawPtr,
        value: u8,
    ) -> ::windows::core::HRESULT,
    pub SetPropertyFromUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub SetPropertyFromObject: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: ::windows::core::RawPtr,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlMarkupHelper(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXamlMarkupHelper {
    type Vtable = IXamlMarkupHelper_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xcd677310_3b06_5a13_b31a_401849570858);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlMarkupHelper_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlMarkupHelperStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXamlMarkupHelperStatics {
    type Vtable = IXamlMarkupHelperStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xd9a0f6e3_c6cc_5cb6_8999_85788701f339);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlMarkupHelperStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub UnloadObject: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
#[repr(transparent)]
pub struct IXamlMember(::windows::core::IUnknown);
impl IXamlMember {
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn IsAttachable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsAttachable)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn IsDependencyProperty(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDependencyProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn IsReadOnly(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsReadOnly)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn TargetType(&self) -> ::windows::core::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TargetType)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IXamlType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IXamlType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(
        &self,
        instance: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetValue)(
                ::core::mem::transmute_copy(this),
                instance.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
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
            (::windows::core::Interface::vtable(this).SetValue)(
                ::core::mem::transmute_copy(this),
                instance.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
impl ::core::convert::From<IXamlMember> for ::windows::core::IUnknown {
    fn from(value: IXamlMember) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXamlMember> for ::windows::core::IUnknown {
    fn from(value: &IXamlMember) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXamlMember {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXamlMember {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXamlMember> for ::windows::core::IInspectable {
    fn from(value: IXamlMember) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXamlMember> for ::windows::core::IInspectable {
    fn from(value: &IXamlMember) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IXamlMember {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IXamlMember {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXamlMember {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXamlMember {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXamlMember {}
impl ::core::fmt::Debug for IXamlMember {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXamlMember").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IXamlMember {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{bf3a2913-5c63-50ec-8660-61809be7b9b9}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IXamlMember {
    type Vtable = IXamlMember_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xbf3a2913_5c63_50ec_8660_61809be7b9b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlMember_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub IsAttachable: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub IsDependencyProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub IsReadOnly: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub TargetType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub Type: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        instance: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        instance: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
#[repr(transparent)]
pub struct IXamlMetadataProvider(::windows::core::IUnknown);
impl IXamlMetadataProvider {
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
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
            (::windows::core::Interface::vtable(this).GetXamlType)(
                ::core::mem::transmute_copy(this),
                r#type.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IXamlType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
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
            (::windows::core::Interface::vtable(this).GetXamlTypeByFullName)(
                ::core::mem::transmute_copy(this),
                fullname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IXamlType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn GetXmlnsDefinitions(
        &self,
    ) -> ::windows::core::Result<::windows::core::Array<XmlnsDefinition>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<XmlnsDefinition> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetXmlnsDefinitions)(
                ::core::mem::transmute_copy(this),
                ::windows::core::Array::<XmlnsDefinition>::set_abi_len(&mut result__),
                &mut result__ as *mut _ as _,
            )
            .and_then(|| result__)
        }
    }
}
impl ::core::convert::From<IXamlMetadataProvider> for ::windows::core::IUnknown {
    fn from(value: IXamlMetadataProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXamlMetadataProvider> for ::windows::core::IUnknown {
    fn from(value: &IXamlMetadataProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXamlMetadataProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXamlMetadataProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXamlMetadataProvider> for ::windows::core::IInspectable {
    fn from(value: IXamlMetadataProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXamlMetadataProvider> for ::windows::core::IInspectable {
    fn from(value: &IXamlMetadataProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IXamlMetadataProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a IXamlMetadataProvider
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXamlMetadataProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXamlMetadataProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXamlMetadataProvider {}
impl ::core::fmt::Debug for IXamlMetadataProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXamlMetadataProvider")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IXamlMetadataProvider {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{a96251f0-2214-5d53-8746-ce99a2593cd7}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IXamlMetadataProvider {
    type Vtable = IXamlMetadataProvider_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa96251f0_2214_5d53_8746_ce99a2593cd7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlMetadataProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub GetXamlType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        r#type: ::core::mem::ManuallyDrop<::windows::UI::Xaml::Interop::TypeName>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetXamlTypeByFullName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        fullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetXmlnsDefinitions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut ::core::mem::ManuallyDrop<XmlnsDefinition>,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlReader(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXamlReader {
    type Vtable = IXamlReader_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x54ce54c8_38c6_50d9_ac98_4b03eddbde9f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlReader_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlReaderStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXamlReaderStatics {
    type Vtable = IXamlReaderStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x82a4cd9e_435e_5aeb_8c4f_300cece45cae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlReaderStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Load: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        xaml: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub LoadWithInitialTemplateValidation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        xaml: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    )
        -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
#[repr(transparent)]
pub struct IXamlType(::windows::core::IUnknown);
impl IXamlType {
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn BaseType(&self) -> ::windows::core::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BaseType)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IXamlType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn ContentProperty(&self) -> ::windows::core::Result<IXamlMember> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IXamlMember>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn FullName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FullName)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn IsArray(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsArray)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn IsCollection(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCollection)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn IsConstructible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsConstructible)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn IsDictionary(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDictionary)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn IsMarkupExtension(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsMarkupExtension)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn IsBindable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsBindable)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn ItemType(&self) -> ::windows::core::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ItemType)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IXamlType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn KeyType(&self) -> ::windows::core::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyType)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IXamlType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn BoxedType(&self) -> ::windows::core::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BoxedType)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IXamlType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn UnderlyingType(
        &self,
    ) -> ::windows::core::Result<::windows::UI::Xaml::Interop::TypeName> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::UI::Xaml::Interop::TypeName> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UnderlyingType)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Xaml::Interop::TypeName>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn ActivateInstance(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ActivateInstance)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn CreateFromString<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromString)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn GetMember<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<IXamlMember> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetMember)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IXamlMember>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
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
            (::windows::core::Interface::vtable(this).AddToVector)(
                ::core::mem::transmute_copy(this),
                instance.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
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
            (::windows::core::Interface::vtable(this).AddToMap)(
                ::core::mem::transmute_copy(this),
                instance.into_param().abi(),
                key.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn RunInitializer(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RunInitializer)(::core::mem::transmute_copy(
                this,
            ))
            .ok()
        }
    }
}
impl ::core::convert::From<IXamlType> for ::windows::core::IUnknown {
    fn from(value: IXamlType) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXamlType> for ::windows::core::IUnknown {
    fn from(value: &IXamlType) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXamlType {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXamlType {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXamlType> for ::windows::core::IInspectable {
    fn from(value: IXamlType) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXamlType> for ::windows::core::IInspectable {
    fn from(value: &IXamlType) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IXamlType {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IXamlType {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXamlType {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXamlType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXamlType {}
impl ::core::fmt::Debug for IXamlType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXamlType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IXamlType {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{d24219df-7ec9-57f1-a27b-6af251d9c5bc}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IXamlType {
    type Vtable = IXamlType_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xd24219df_7ec9_57f1_a27b_6af251d9c5bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlType_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub BaseType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub ContentProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub FullName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub IsArray: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub IsCollection: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub IsConstructible: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub IsDictionary: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub IsMarkupExtension: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub IsBindable: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub ItemType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub KeyType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub BoxedType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub UnderlyingType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::UI::Xaml::Interop::TypeName>,
    ) -> ::windows::core::HRESULT,
    pub ActivateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CreateFromString: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetMember: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub AddToVector: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        instance: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub AddToMap: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        instance: *mut ::core::ffi::c_void,
        key: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub RunInitializer:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
#[repr(transparent)]
pub struct IXamlTypeResolver(::windows::core::IUnknown);
impl IXamlTypeResolver {
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn Resolve<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        qualifiedtypename: Param0,
    ) -> ::windows::core::Result<::windows::UI::Xaml::Interop::TypeName> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::UI::Xaml::Interop::TypeName> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Resolve)(
                ::core::mem::transmute_copy(this),
                qualifiedtypename.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::UI::Xaml::Interop::TypeName>(result__)
        }
    }
}
impl ::core::convert::From<IXamlTypeResolver> for ::windows::core::IUnknown {
    fn from(value: IXamlTypeResolver) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXamlTypeResolver> for ::windows::core::IUnknown {
    fn from(value: &IXamlTypeResolver) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXamlTypeResolver {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXamlTypeResolver {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXamlTypeResolver> for ::windows::core::IInspectable {
    fn from(value: IXamlTypeResolver) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXamlTypeResolver> for ::windows::core::IInspectable {
    fn from(value: &IXamlTypeResolver) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IXamlTypeResolver {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IXamlTypeResolver {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXamlTypeResolver {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXamlTypeResolver {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXamlTypeResolver {}
impl ::core::fmt::Debug for IXamlTypeResolver {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXamlTypeResolver").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IXamlTypeResolver {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{3fa15615-cacf-547f-b1ed-89dae8c67452}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IXamlTypeResolver {
    type Vtable = IXamlTypeResolver_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x3fa15615_cacf_547f_b1ed_89dae8c67452);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlTypeResolver_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Resolve: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        qualifiedtypename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::core::mem::ManuallyDrop<::windows::UI::Xaml::Interop::TypeName>,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
#[repr(transparent)]
pub struct MarkupExtension(::windows::core::IUnknown);
impl MarkupExtension {
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn new() -> ::windows::core::Result<MarkupExtension> {
        Self::IMarkupExtensionFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(
                ::core::mem::transmute_copy(this),
                ::core::ptr::null_mut(),
                &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _,
                &mut result__,
            )
            .from_abi::<MarkupExtension>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn compose<T: ::windows::core::Compose>(
        compose: T,
    ) -> ::windows::core::Result<MarkupExtension> {
        Self::IMarkupExtensionFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(
                ::core::mem::transmute_copy(this),
                ::core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .from_abi::<MarkupExtension>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMarkupExtensionFactory<
        R,
        F: FnOnce(&IMarkupExtensionFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MarkupExtension, IMarkupExtensionFactory> =
            ::windows::core::FactoryCache::from_library(b"Microsoft.UI.Xaml.dll\0");
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MarkupExtension {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MarkupExtension {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MarkupExtension {}
impl ::core::fmt::Debug for MarkupExtension {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MarkupExtension").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MarkupExtension {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Markup.MarkupExtension;{c355371e-091d-5136-af4a-baf5e00616bd})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MarkupExtension {
    type Vtable = IMarkupExtension_Vtbl;
    const IID: ::windows::core::GUID = <IMarkupExtension as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MarkupExtension {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.MarkupExtension";
}
impl ::core::convert::From<MarkupExtension> for ::windows::core::IUnknown {
    fn from(value: MarkupExtension) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MarkupExtension> for ::windows::core::IUnknown {
    fn from(value: &MarkupExtension) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MarkupExtension {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MarkupExtension {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MarkupExtension> for ::windows::core::IInspectable {
    fn from(value: MarkupExtension) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MarkupExtension> for ::windows::core::IInspectable {
    fn from(value: &MarkupExtension) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MarkupExtension {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MarkupExtension {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MarkupExtension {}
unsafe impl ::core::marker::Sync for MarkupExtension {}
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
#[repr(transparent)]
pub struct ProvideValueTargetProperty(::windows::core::IUnknown);
impl ProvideValueTargetProperty {
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
            ProvideValueTargetProperty,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::from_library(b"Microsoft.UI.Xaml.dll\0");
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<::windows::UI::Xaml::Interop::TypeName> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::UI::Xaml::Interop::TypeName> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Xaml::Interop::TypeName>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn DeclaringType(&self) -> ::windows::core::Result<::windows::UI::Xaml::Interop::TypeName> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::UI::Xaml::Interop::TypeName> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeclaringType)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Xaml::Interop::TypeName>(result__)
        }
    }
}
impl ::core::clone::Clone for ProvideValueTargetProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProvideValueTargetProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProvideValueTargetProperty {}
impl ::core::fmt::Debug for ProvideValueTargetProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProvideValueTargetProperty")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ProvideValueTargetProperty {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Markup.ProvideValueTargetProperty;{ce777b1f-b42e-59d1-870d-12fdf0629133})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ProvideValueTargetProperty {
    type Vtable = IProvideValueTargetProperty_Vtbl;
    const IID: ::windows::core::GUID =
        <IProvideValueTargetProperty as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ProvideValueTargetProperty {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.ProvideValueTargetProperty";
}
impl ::core::convert::From<ProvideValueTargetProperty> for ::windows::core::IUnknown {
    fn from(value: ProvideValueTargetProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProvideValueTargetProperty> for ::windows::core::IUnknown {
    fn from(value: &ProvideValueTargetProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProvideValueTargetProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a ProvideValueTargetProperty
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProvideValueTargetProperty> for ::windows::core::IInspectable {
    fn from(value: ProvideValueTargetProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProvideValueTargetProperty> for ::windows::core::IInspectable {
    fn from(value: &ProvideValueTargetProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for ProvideValueTargetProperty
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a ProvideValueTargetProperty
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProvideValueTargetProperty {}
unsafe impl ::core::marker::Sync for ProvideValueTargetProperty {}
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
#[repr(transparent)]
pub struct XamlBinaryWriter(::windows::core::IUnknown);
impl XamlBinaryWriter {
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
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
            (::windows::core::Interface::vtable(this).Write)(
                ::core::mem::transmute_copy(this),
                inputstreams.into_param().abi(),
                outputstreams.into_param().abi(),
                xamlmetadataprovider.into_param().abi(),
                &mut result__,
            )
            .from_abi::<XamlBinaryWriterErrorInformation>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IXamlBinaryWriterStatics<
        R,
        F: FnOnce(&IXamlBinaryWriterStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            XamlBinaryWriter,
            IXamlBinaryWriterStatics,
        > = ::windows::core::FactoryCache::from_library(b"Microsoft.UI.Xaml.dll\0");
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for XamlBinaryWriter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XamlBinaryWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XamlBinaryWriter {}
impl ::core::fmt::Debug for XamlBinaryWriter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XamlBinaryWriter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XamlBinaryWriter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Markup.XamlBinaryWriter;{8fb45e3b-e689-55bf-aa11-d83b1c1cdda1})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for XamlBinaryWriter {
    type Vtable = IXamlBinaryWriter_Vtbl;
    const IID: ::windows::core::GUID = <IXamlBinaryWriter as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XamlBinaryWriter {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.XamlBinaryWriter";
}
impl ::core::convert::From<XamlBinaryWriter> for ::windows::core::IUnknown {
    fn from(value: XamlBinaryWriter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XamlBinaryWriter> for ::windows::core::IUnknown {
    fn from(value: &XamlBinaryWriter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for XamlBinaryWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a XamlBinaryWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XamlBinaryWriter> for ::windows::core::IInspectable {
    fn from(value: XamlBinaryWriter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XamlBinaryWriter> for ::windows::core::IInspectable {
    fn from(value: &XamlBinaryWriter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for XamlBinaryWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a XamlBinaryWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for XamlBinaryWriter {}
unsafe impl ::core::marker::Sync for XamlBinaryWriter {}
#[repr(C)]
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
pub struct XamlBinaryWriterErrorInformation {
    pub InputStreamIndex: u32,
    pub LineNumber: u32,
    pub LinePosition: u32,
}
impl ::core::marker::Copy for XamlBinaryWriterErrorInformation {}
impl ::core::clone::Clone for XamlBinaryWriterErrorInformation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for XamlBinaryWriterErrorInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XamlBinaryWriterErrorInformation")
            .field("InputStreamIndex", &self.InputStreamIndex)
            .field("LineNumber", &self.LineNumber)
            .field("LinePosition", &self.LinePosition)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for XamlBinaryWriterErrorInformation {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for XamlBinaryWriterErrorInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"struct(Microsoft.UI.Xaml.Markup.XamlBinaryWriterErrorInformation;u4;u4;u4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for XamlBinaryWriterErrorInformation {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            ::windows::core::memcmp(
                self as *const _ as _,
                other as *const _ as _,
                core::mem::size_of::<XamlBinaryWriterErrorInformation>(),
            ) == 0
        }
    }
}
impl ::core::cmp::Eq for XamlBinaryWriterErrorInformation {}
impl ::core::default::Default for XamlBinaryWriterErrorInformation {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
#[repr(transparent)]
pub struct XamlBindingHelper(::windows::core::IUnknown);
impl XamlBindingHelper {
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn DataTemplateComponentProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DataTemplateComponentProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn GetDataTemplateComponent<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<IDataTemplateComponent> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDataTemplateComponent)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IDataTemplateComponent>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn SetDataTemplateComponent<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
        Param1: ::windows::core::IntoParam<'a, IDataTemplateComponent>,
    >(
        element: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetDataTemplateComponent)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn SuspendRendering<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(
        target: Param0,
    ) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SuspendRendering)(
                ::core::mem::transmute_copy(this),
                target.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn ResumeRendering<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(
        target: Param0,
    ) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).ResumeRendering)(
                ::core::mem::transmute_copy(this),
                target.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn ConvertValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::UI::Xaml::Interop::TypeName>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        r#type: Param0,
        value: Param1,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConvertValue)(
                ::core::mem::transmute_copy(this),
                r#type.into_param().abi(),
                value.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
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
            (::windows::core::Interface::vtable(this).SetPropertyFromString)(
                ::core::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
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
            (::windows::core::Interface::vtable(this).SetPropertyFromBoolean)(
                ::core::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
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
            (::windows::core::Interface::vtable(this).SetPropertyFromChar16)(
                ::core::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
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
            (::windows::core::Interface::vtable(this).SetPropertyFromDateTime)(
                ::core::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
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
            (::windows::core::Interface::vtable(this).SetPropertyFromDouble)(
                ::core::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
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
            (::windows::core::Interface::vtable(this).SetPropertyFromInt32)(
                ::core::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
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
            (::windows::core::Interface::vtable(this).SetPropertyFromUInt32)(
                ::core::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
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
            (::windows::core::Interface::vtable(this).SetPropertyFromInt64)(
                ::core::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
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
            (::windows::core::Interface::vtable(this).SetPropertyFromUInt64)(
                ::core::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
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
            (::windows::core::Interface::vtable(this).SetPropertyFromSingle)(
                ::core::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
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
            (::windows::core::Interface::vtable(this).SetPropertyFromPoint)(
                ::core::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
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
            (::windows::core::Interface::vtable(this).SetPropertyFromRect)(
                ::core::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
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
            (::windows::core::Interface::vtable(this).SetPropertyFromSize)(
                ::core::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
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
            (::windows::core::Interface::vtable(this).SetPropertyFromTimeSpan)(
                ::core::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
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
            (::windows::core::Interface::vtable(this).SetPropertyFromByte)(
                ::core::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
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
            (::windows::core::Interface::vtable(this).SetPropertyFromUri)(
                ::core::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
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
            (::windows::core::Interface::vtable(this).SetPropertyFromObject)(
                ::core::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc(hidden)]
    pub fn IXamlBindingHelperStatics<
        R,
        F: FnOnce(&IXamlBindingHelperStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            XamlBindingHelper,
            IXamlBindingHelperStatics,
        > = ::windows::core::FactoryCache::from_library(b"Microsoft.UI.Xaml.dll\0");
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for XamlBindingHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XamlBindingHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XamlBindingHelper {}
impl ::core::fmt::Debug for XamlBindingHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XamlBindingHelper").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XamlBindingHelper {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Markup.XamlBindingHelper;{607a9bf2-5a6d-5c89-a756-bb44f24f28f8})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for XamlBindingHelper {
    type Vtable = IXamlBindingHelper_Vtbl;
    const IID: ::windows::core::GUID = <IXamlBindingHelper as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XamlBindingHelper {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.XamlBindingHelper";
}
impl ::core::convert::From<XamlBindingHelper> for ::windows::core::IUnknown {
    fn from(value: XamlBindingHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XamlBindingHelper> for ::windows::core::IUnknown {
    fn from(value: &XamlBindingHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for XamlBindingHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a XamlBindingHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XamlBindingHelper> for ::windows::core::IInspectable {
    fn from(value: XamlBindingHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XamlBindingHelper> for ::windows::core::IInspectable {
    fn from(value: &XamlBindingHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for XamlBindingHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a XamlBindingHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for XamlBindingHelper {}
unsafe impl ::core::marker::Sync for XamlBindingHelper {}
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
#[repr(transparent)]
pub struct XamlMarkupHelper(::windows::core::IUnknown);
impl XamlMarkupHelper {
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn UnloadObject<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<()> {
        Self::IXamlMarkupHelperStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).UnloadObject)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc(hidden)]
    pub fn IXamlMarkupHelperStatics<
        R,
        F: FnOnce(&IXamlMarkupHelperStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            XamlMarkupHelper,
            IXamlMarkupHelperStatics,
        > = ::windows::core::FactoryCache::from_library(b"Microsoft.UI.Xaml.dll\0");
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for XamlMarkupHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XamlMarkupHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XamlMarkupHelper {}
impl ::core::fmt::Debug for XamlMarkupHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XamlMarkupHelper").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XamlMarkupHelper {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Markup.XamlMarkupHelper;{cd677310-3b06-5a13-b31a-401849570858})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for XamlMarkupHelper {
    type Vtable = IXamlMarkupHelper_Vtbl;
    const IID: ::windows::core::GUID = <IXamlMarkupHelper as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XamlMarkupHelper {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.XamlMarkupHelper";
}
impl ::core::convert::From<XamlMarkupHelper> for ::windows::core::IUnknown {
    fn from(value: XamlMarkupHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XamlMarkupHelper> for ::windows::core::IUnknown {
    fn from(value: &XamlMarkupHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for XamlMarkupHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a XamlMarkupHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XamlMarkupHelper> for ::windows::core::IInspectable {
    fn from(value: XamlMarkupHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XamlMarkupHelper> for ::windows::core::IInspectable {
    fn from(value: &XamlMarkupHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for XamlMarkupHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a XamlMarkupHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for XamlMarkupHelper {}
unsafe impl ::core::marker::Sync for XamlMarkupHelper {}
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
#[repr(transparent)]
pub struct XamlReader(::windows::core::IUnknown);
impl XamlReader {
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn Load<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        xaml: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IXamlReaderStatics(|this| unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Load)(
                ::core::mem::transmute_copy(this),
                xaml.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn LoadWithInitialTemplateValidation<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        xaml: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IXamlReaderStatics(|this| unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LoadWithInitialTemplateValidation)(
                ::core::mem::transmute_copy(this),
                xaml.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IXamlReaderStatics<R, F: FnOnce(&IXamlReaderStatics) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<XamlReader, IXamlReaderStatics> =
            ::windows::core::FactoryCache::from_library(b"Microsoft.UI.Xaml.dll\0");
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for XamlReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XamlReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XamlReader {}
impl ::core::fmt::Debug for XamlReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XamlReader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XamlReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Markup.XamlReader;{54ce54c8-38c6-50d9-ac98-4b03eddbde9f})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for XamlReader {
    type Vtable = IXamlReader_Vtbl;
    const IID: ::windows::core::GUID = <IXamlReader as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XamlReader {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.XamlReader";
}
impl ::core::convert::From<XamlReader> for ::windows::core::IUnknown {
    fn from(value: XamlReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XamlReader> for ::windows::core::IUnknown {
    fn from(value: &XamlReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for XamlReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a XamlReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XamlReader> for ::windows::core::IInspectable {
    fn from(value: XamlReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XamlReader> for ::windows::core::IInspectable {
    fn from(value: &XamlReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for XamlReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a XamlReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for XamlReader {}
unsafe impl ::core::marker::Sync for XamlReader {}
#[repr(C)]
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
pub struct XmlnsDefinition {
    pub XmlNamespace: ::windows::core::HSTRING,
    pub Namespace: ::windows::core::HSTRING,
}
impl ::core::clone::Clone for XmlnsDefinition {
    fn clone(&self) -> Self {
        Self {
            XmlNamespace: self.XmlNamespace.clone(),
            Namespace: self.Namespace.clone(),
        }
    }
}
impl ::core::fmt::Debug for XmlnsDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XmlnsDefinition")
            .field("XmlNamespace", &self.XmlNamespace)
            .field("Namespace", &self.Namespace)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for XmlnsDefinition {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
unsafe impl ::windows::core::RuntimeType for XmlnsDefinition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"struct(Microsoft.UI.Xaml.Markup.XmlnsDefinition;string;string)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(from.clone())
    }
}
impl ::core::cmp::PartialEq for XmlnsDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.XmlNamespace == other.XmlNamespace && self.Namespace == other.Namespace
    }
}
impl ::core::cmp::Eq for XmlnsDefinition {}
impl ::core::default::Default for XmlnsDefinition {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
