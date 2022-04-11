#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clashing_extern_declarations,
    clippy::all
)]
#[doc = "*Required features: `\"UI_Composition_Experimental\"`*"]
pub struct ExpCompositionVisualSurface {}
impl ExpCompositionVisualSurface {
    #[doc = "*Required features: `\"UI_Composition_Experimental\"`*"]
    pub fn CreateVisualSurfaceWithRealizationSize<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Compositor>,
        Param1: ::windows::core::IntoParam<'a, ::windows::Graphics::SizeInt32>,
    >(
        compositor: Param0,
        realizationsize: Param1,
        frozen: bool,
    ) -> ::windows::core::Result<super::CompositionVisualSurface> {
        Self::IExpCompositionVisualSurfaceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateVisualSurfaceWithRealizationSize)(
                ::core::mem::transmute_copy(this),
                compositor.into_param().abi(),
                realizationsize.into_param().abi(),
                frozen,
                &mut result__,
            )
            .from_abi::<super::CompositionVisualSurface>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IExpCompositionVisualSurfaceStatics<
        R,
        F: FnOnce(&IExpCompositionVisualSurfaceStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            ExpCompositionVisualSurface,
            IExpCompositionVisualSurfaceStatics,
        > = ::windows::core::FactoryCache::from_library(b"dcompi.dll\0");
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for ExpCompositionVisualSurface {
    const NAME: &'static str = "Microsoft.UI.Composition.Experimental.ExpCompositionVisualSurface";
}
#[doc = "*Required features: `\"UI_Composition_Experimental\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ExpExpressionNotificationProperty(pub i32);
impl ExpExpressionNotificationProperty {
    pub const Undefined: Self = Self(0i32);
    pub const Clip: Self = Self(1i32);
    pub const Offset: Self = Self(2i32);
    pub const Opacity: Self = Self(3i32);
    pub const Size: Self = Self(4i32);
    pub const RelativeOffset: Self = Self(5i32);
    pub const RelativeSize: Self = Self(6i32);
    pub const AnchorPoint: Self = Self(7i32);
    pub const CenterPoint: Self = Self(8i32);
    pub const Orientation: Self = Self(9i32);
    pub const RotationAngle: Self = Self(10i32);
    pub const RotationAxis: Self = Self(11i32);
    pub const Scale: Self = Self(12i32);
    pub const TransformMatrix: Self = Self(13i32);
    pub const BottomInset: Self = Self(14i32);
    pub const LeftInset: Self = Self(15i32);
    pub const RightInset: Self = Self(16i32);
    pub const TopInset: Self = Self(17i32);
    pub const LeftRadiusX: Self = Self(18i32);
    pub const LeftRadiusY: Self = Self(19i32);
    pub const BottomRightRadiusX: Self = Self(20i32);
    pub const BottomRightRadiusY: Self = Self(21i32);
    pub const TopLeftRadiusX: Self = Self(22i32);
    pub const TopLeftRadiusY: Self = Self(23i32);
    pub const TopRightRadiusX: Self = Self(24i32);
    pub const TopRightRadiusY: Self = Self(25i32);
}
impl ::core::marker::Copy for ExpExpressionNotificationProperty {}
impl ::core::clone::Clone for ExpExpressionNotificationProperty {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ExpExpressionNotificationProperty {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ExpExpressionNotificationProperty {
    type Abi = Self;
}
impl ::core::fmt::Debug for ExpExpressionNotificationProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExpExpressionNotificationProperty")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ExpExpressionNotificationProperty {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Composition.Experimental.ExpExpressionNotificationProperty;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Composition_Experimental\"`*"]
#[repr(transparent)]
pub struct IExpCompositionPropertyChanged(::windows::core::IUnknown);
impl IExpCompositionPropertyChanged {
    #[doc = "*Required features: `\"UI_Composition_Experimental\"`*"]
    pub fn SetPropertyChangedListener<
        'a,
        Param1: ::windows::core::IntoParam<'a, IExpCompositionPropertyChangedListener>,
    >(
        &self,
        property: ExpExpressionNotificationProperty,
        listener: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetPropertyChangedListener)(
                ::core::mem::transmute_copy(this),
                property,
                listener.into_param().abi(),
            )
            .ok()
        }
    }
}
impl ::core::convert::From<IExpCompositionPropertyChanged> for ::windows::core::IUnknown {
    fn from(value: IExpCompositionPropertyChanged) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IExpCompositionPropertyChanged> for ::windows::core::IUnknown {
    fn from(value: &IExpCompositionPropertyChanged) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for IExpCompositionPropertyChanged
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a IExpCompositionPropertyChanged
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IExpCompositionPropertyChanged> for ::windows::core::IInspectable {
    fn from(value: IExpCompositionPropertyChanged) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IExpCompositionPropertyChanged> for ::windows::core::IInspectable {
    fn from(value: &IExpCompositionPropertyChanged) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for IExpCompositionPropertyChanged
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a IExpCompositionPropertyChanged
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IExpCompositionPropertyChanged {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IExpCompositionPropertyChanged {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IExpCompositionPropertyChanged {}
impl ::core::fmt::Debug for IExpCompositionPropertyChanged {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IExpCompositionPropertyChanged")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IExpCompositionPropertyChanged {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{12b579a9-6a27-5cde-a2a1-c557bb7dfdb3}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IExpCompositionPropertyChanged {
    type Vtable = IExpCompositionPropertyChanged_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x12b579a9_6a27_5cde_a2a1_c557bb7dfdb3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpCompositionPropertyChanged_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub SetPropertyChangedListener: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        property: ExpExpressionNotificationProperty,
        listener: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Composition_Experimental\"`*"]
#[repr(transparent)]
pub struct IExpCompositionPropertyChangedListener(::windows::core::IUnknown);
impl IExpCompositionPropertyChangedListener {
    #[doc = "*Required features: `\"UI_Composition_Experimental\"`*"]
    pub fn NotifyBooleanPropertyChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::CompositionObject>,
    >(
        &self,
        target: Param0,
        property: ExpExpressionNotificationProperty,
        value: bool,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).NotifyBooleanPropertyChanged)(
                ::core::mem::transmute_copy(this),
                target.into_param().abi(),
                property,
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Experimental\"`*"]
    pub fn NotifyColorPropertyChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::CompositionObject>,
        Param2: ::windows::core::IntoParam<'a, ::windows::UI::Color>,
    >(
        &self,
        target: Param0,
        property: ExpExpressionNotificationProperty,
        value: Param2,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).NotifyColorPropertyChanged)(
                ::core::mem::transmute_copy(this),
                target.into_param().abi(),
                property,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Experimental\"`*"]
    pub fn NotifyMatrix3x2PropertyChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::CompositionObject>,
        Param2: ::windows::core::IntoParam<'a, ::windows::Foundation::Numerics::Matrix3x2>,
    >(
        &self,
        target: Param0,
        property: ExpExpressionNotificationProperty,
        value: Param2,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).NotifyMatrix3x2PropertyChanged)(
                ::core::mem::transmute_copy(this),
                target.into_param().abi(),
                property,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Experimental\"`*"]
    pub fn NotifyMatrix4x4PropertyChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::CompositionObject>,
        Param2: ::windows::core::IntoParam<'a, ::windows::Foundation::Numerics::Matrix4x4>,
    >(
        &self,
        target: Param0,
        property: ExpExpressionNotificationProperty,
        value: Param2,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).NotifyMatrix4x4PropertyChanged)(
                ::core::mem::transmute_copy(this),
                target.into_param().abi(),
                property,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Experimental\"`*"]
    pub fn NotifyReferencePropertyChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::CompositionObject>,
    >(
        &self,
        target: Param0,
        property: ExpExpressionNotificationProperty,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).NotifyReferencePropertyChanged)(
                ::core::mem::transmute_copy(this),
                target.into_param().abi(),
                property,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Experimental\"`*"]
    pub fn NotifySinglePropertyChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::CompositionObject>,
    >(
        &self,
        target: Param0,
        property: ExpExpressionNotificationProperty,
        value: f32,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).NotifySinglePropertyChanged)(
                ::core::mem::transmute_copy(this),
                target.into_param().abi(),
                property,
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Experimental\"`*"]
    pub fn NotifyVector2PropertyChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::CompositionObject>,
        Param2: ::windows::core::IntoParam<'a, ::windows::Foundation::Numerics::Vector2>,
    >(
        &self,
        target: Param0,
        property: ExpExpressionNotificationProperty,
        value: Param2,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).NotifyVector2PropertyChanged)(
                ::core::mem::transmute_copy(this),
                target.into_param().abi(),
                property,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Experimental\"`*"]
    pub fn NotifyVector3PropertyChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::CompositionObject>,
        Param2: ::windows::core::IntoParam<'a, ::windows::Foundation::Numerics::Vector3>,
    >(
        &self,
        target: Param0,
        property: ExpExpressionNotificationProperty,
        value: Param2,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).NotifyVector3PropertyChanged)(
                ::core::mem::transmute_copy(this),
                target.into_param().abi(),
                property,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Experimental\"`*"]
    pub fn NotifyVector4PropertyChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::CompositionObject>,
        Param2: ::windows::core::IntoParam<'a, ::windows::Foundation::Numerics::Vector4>,
    >(
        &self,
        target: Param0,
        property: ExpExpressionNotificationProperty,
        value: Param2,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).NotifyVector4PropertyChanged)(
                ::core::mem::transmute_copy(this),
                target.into_param().abi(),
                property,
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
impl ::core::convert::From<IExpCompositionPropertyChangedListener> for ::windows::core::IUnknown {
    fn from(value: IExpCompositionPropertyChangedListener) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IExpCompositionPropertyChangedListener> for ::windows::core::IUnknown {
    fn from(value: &IExpCompositionPropertyChangedListener) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for IExpCompositionPropertyChangedListener
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a IExpCompositionPropertyChangedListener
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IExpCompositionPropertyChangedListener>
    for ::windows::core::IInspectable
{
    fn from(value: IExpCompositionPropertyChangedListener) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IExpCompositionPropertyChangedListener>
    for ::windows::core::IInspectable
{
    fn from(value: &IExpCompositionPropertyChangedListener) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for IExpCompositionPropertyChangedListener
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a IExpCompositionPropertyChangedListener
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IExpCompositionPropertyChangedListener {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IExpCompositionPropertyChangedListener {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IExpCompositionPropertyChangedListener {}
impl ::core::fmt::Debug for IExpCompositionPropertyChangedListener {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IExpCompositionPropertyChangedListener")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IExpCompositionPropertyChangedListener {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{5f9c3d96-1e77-5980-8b28-7a9b8614a863}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IExpCompositionPropertyChangedListener {
    type Vtable = IExpCompositionPropertyChangedListener_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x5f9c3d96_1e77_5980_8b28_7a9b8614a863);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpCompositionPropertyChangedListener_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub NotifyBooleanPropertyChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: ::windows::core::RawPtr,
        property: ExpExpressionNotificationProperty,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub NotifyColorPropertyChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: ::windows::core::RawPtr,
        property: ExpExpressionNotificationProperty,
        value: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub NotifyMatrix3x2PropertyChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: ::windows::core::RawPtr,
        property: ExpExpressionNotificationProperty,
        value: ::windows::Foundation::Numerics::Matrix3x2,
    ) -> ::windows::core::HRESULT,
    pub NotifyMatrix4x4PropertyChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: ::windows::core::RawPtr,
        property: ExpExpressionNotificationProperty,
        value: ::windows::Foundation::Numerics::Matrix4x4,
    ) -> ::windows::core::HRESULT,
    pub NotifyReferencePropertyChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: ::windows::core::RawPtr,
        property: ExpExpressionNotificationProperty,
    ) -> ::windows::core::HRESULT,
    pub NotifySinglePropertyChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: ::windows::core::RawPtr,
        property: ExpExpressionNotificationProperty,
        value: f32,
    ) -> ::windows::core::HRESULT,
    pub NotifyVector2PropertyChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: ::windows::core::RawPtr,
        property: ExpExpressionNotificationProperty,
        value: ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::HRESULT,
    pub NotifyVector3PropertyChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: ::windows::core::RawPtr,
        property: ExpExpressionNotificationProperty,
        value: ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::core::HRESULT,
    pub NotifyVector4PropertyChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: ::windows::core::RawPtr,
        property: ExpExpressionNotificationProperty,
        value: ::windows::Foundation::Numerics::Vector4,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IExpCompositionVisualSurfaceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IExpCompositionVisualSurfaceStatics {
    type Vtable = IExpCompositionVisualSurfaceStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x46193461_9018_5674_a09c_4ae6a29daa35);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpCompositionVisualSurfaceStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CreateVisualSurfaceWithRealizationSize:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            compositor: ::windows::core::RawPtr,
            realizationsize: ::windows::Graphics::SizeInt32,
            frozen: bool,
            result__: *mut ::windows::core::RawPtr,
        ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Composition_Experimental\"`*"]
#[repr(transparent)]
pub struct IExpCompositor(::windows::core::IUnknown);
impl IExpCompositor {
    #[doc = "*Required features: `\"UI_Composition_Experimental\"`*"]
    pub fn OpenSharedManipulationTransformFromHandle(
        &self,
        handle: u64,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OpenSharedManipulationTransformFromHandle)(
                ::core::mem::transmute_copy(this),
                handle,
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
impl ::core::convert::From<IExpCompositor> for ::windows::core::IUnknown {
    fn from(value: IExpCompositor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IExpCompositor> for ::windows::core::IUnknown {
    fn from(value: &IExpCompositor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IExpCompositor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IExpCompositor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IExpCompositor> for ::windows::core::IInspectable {
    fn from(value: IExpCompositor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IExpCompositor> for ::windows::core::IInspectable {
    fn from(value: &IExpCompositor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IExpCompositor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IExpCompositor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IExpCompositor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IExpCompositor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IExpCompositor {}
impl ::core::fmt::Debug for IExpCompositor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IExpCompositor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IExpCompositor {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{ddfe7441-66c9-5654-9e80-ff2677295995}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IExpCompositor {
    type Vtable = IExpCompositor_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xddfe7441_66c9_5654_9e80_ff2677295995);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpCompositor_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub OpenSharedManipulationTransformFromHandle:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            handle: u64,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Composition_Experimental\"`*"]
#[repr(transparent)]
pub struct IExpVisual(::windows::core::IUnknown);
impl IExpVisual {
    #[doc = "*Required features: `\"UI_Composition_Experimental\"`*"]
    pub fn SetInteraction<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        interaction: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetInteraction)(
                ::core::mem::transmute_copy(this),
                interaction.into_param().abi(),
            )
            .ok()
        }
    }
}
impl ::core::convert::From<IExpVisual> for ::windows::core::IUnknown {
    fn from(value: IExpVisual) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IExpVisual> for ::windows::core::IUnknown {
    fn from(value: &IExpVisual) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IExpVisual {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IExpVisual {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IExpVisual> for ::windows::core::IInspectable {
    fn from(value: IExpVisual) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IExpVisual> for ::windows::core::IInspectable {
    fn from(value: &IExpVisual) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IExpVisual {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IExpVisual {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IExpVisual {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IExpVisual {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IExpVisual {}
impl ::core::fmt::Debug for IExpVisual {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IExpVisual").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IExpVisual {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{8faccf79-665b-578f-8197-f8a64f8833d6}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IExpVisual {
    type Vtable = IExpVisual_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x8faccf79_665b_578f_8197_f8a64f8833d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpVisual_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub SetInteraction: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        interaction: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
