#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clashing_extern_declarations,
    clippy::all
)]
#[doc = "*Required features: 'UI_Composition_SystemBackdrops'*"]
#[repr(transparent)]
pub struct DesktopAcrylicController(::windows::core::IUnknown);
impl DesktopAcrylicController {
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
            DesktopAcrylicController,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Composition_SystemBackdrops'*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: 'UI_Composition_SystemBackdrops'*"]
    pub fn FallbackColor(&self) -> ::windows::core::Result<::windows::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: ::windows::UI::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FallbackColor)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Composition_SystemBackdrops'*"]
    pub fn SetFallbackColor<'a, Param0: ::windows::core::IntoParam<'a, ::windows::UI::Color>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFallbackColor)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'UI_Composition_SystemBackdrops'*"]
    pub fn LuminosityOpacity(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LuminosityOpacity)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Composition_SystemBackdrops'*"]
    pub fn SetLuminosityOpacity(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetLuminosityOpacity)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'UI_Composition_SystemBackdrops'*"]
    pub fn TintColor(&self) -> ::windows::core::Result<::windows::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: ::windows::UI::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TintColor)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Composition_SystemBackdrops'*"]
    pub fn SetTintColor<'a, Param0: ::windows::core::IntoParam<'a, ::windows::UI::Color>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetTintColor)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'UI_Composition_SystemBackdrops'*"]
    pub fn TintOpacity(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TintOpacity)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Composition_SystemBackdrops'*"]
    pub fn SetTintOpacity(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetTintOpacity)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'UI_Composition_SystemBackdrops'*"]
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::IDesktopAcrylicControllerStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSupported)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Composition_SystemBackdrops'*"]
    pub fn SetTargetWithWindowId<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::WindowId>,
        Param1: ::windows::core::IntoParam<'a, ::windows::UI::Composition::CompositionTarget>,
    >(
        &self,
        windowid: Param0,
        desktopwindowtarget: Param1,
    ) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISystemBackdropController>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetTargetWithWindowId)(
                ::core::mem::transmute_copy(this),
                windowid.into_param().abi(),
                desktopwindowtarget.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Composition_SystemBackdrops'*"]
    pub fn SetTargetWithCoreWindow<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::UI::Core::CoreWindow>,
        Param1: ::windows::core::IntoParam<'a, ::windows::UI::Composition::CompositionTarget>,
    >(
        &self,
        corewindow: Param0,
        compositiontarget: Param1,
    ) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISystemBackdropController>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetTargetWithCoreWindow)(
                ::core::mem::transmute_copy(this),
                corewindow.into_param().abi(),
                compositiontarget.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IDesktopAcrylicControllerStatics<
        R,
        F: FnOnce(&IDesktopAcrylicControllerStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            DesktopAcrylicController,
            IDesktopAcrylicControllerStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DesktopAcrylicController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DesktopAcrylicController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DesktopAcrylicController {}
impl ::core::fmt::Debug for DesktopAcrylicController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DesktopAcrylicController")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DesktopAcrylicController {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.SystemBackdrops.DesktopAcrylicController;{7c20a6af-8eb3-5f08-bdfc-6d35e35dfe45})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DesktopAcrylicController {
    type Vtable = IDesktopAcrylicController_Vtbl;
    const IID: ::windows::core::GUID =
        <IDesktopAcrylicController as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DesktopAcrylicController {
    const NAME: &'static str = "Microsoft.UI.Composition.SystemBackdrops.DesktopAcrylicController";
}
impl ::core::convert::From<DesktopAcrylicController> for ::windows::core::IUnknown {
    fn from(value: DesktopAcrylicController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DesktopAcrylicController> for ::windows::core::IUnknown {
    fn from(value: &DesktopAcrylicController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DesktopAcrylicController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a DesktopAcrylicController
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DesktopAcrylicController> for ::windows::core::IInspectable {
    fn from(value: DesktopAcrylicController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DesktopAcrylicController> for ::windows::core::IInspectable {
    fn from(value: &DesktopAcrylicController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for DesktopAcrylicController
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a DesktopAcrylicController
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<DesktopAcrylicController> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: DesktopAcrylicController) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DesktopAcrylicController> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &DesktopAcrylicController) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable>
    for DesktopAcrylicController
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable>
    for &DesktopAcrylicController
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::core::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<DesktopAcrylicController> for ISystemBackdropController {
    type Error = ::windows::core::Error;
    fn try_from(value: DesktopAcrylicController) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DesktopAcrylicController> for ISystemBackdropController {
    type Error = ::windows::core::Error;
    fn try_from(value: &DesktopAcrylicController) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISystemBackdropController> for DesktopAcrylicController {
    fn into_param(self) -> ::windows::core::Param<'a, ISystemBackdropController> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISystemBackdropController> for &DesktopAcrylicController {
    fn into_param(self) -> ::windows::core::Param<'a, ISystemBackdropController> {
        ::core::convert::TryInto::<ISystemBackdropController>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for DesktopAcrylicController {}
unsafe impl ::core::marker::Sync for DesktopAcrylicController {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDesktopAcrylicController(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDesktopAcrylicController {
    type Vtable = IDesktopAcrylicController_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x7c20a6af_8eb3_5f08_bdfc_6d35e35dfe45);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopAcrylicController_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub FallbackColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub SetFallbackColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub LuminosityOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub SetLuminosityOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows::core::HRESULT,
    pub TintColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub SetTintColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub TintOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub SetTintOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDesktopAcrylicControllerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDesktopAcrylicControllerStatics {
    type Vtable = IDesktopAcrylicControllerStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa9e8f790_79ef_5416_9b67_6bcfe867c8b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopAcrylicControllerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsSupported: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMicaController(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMicaController {
    type Vtable = IMicaController_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x2de996a9_0a2a_5889_a89c_1f84060a8cab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicaController_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub FallbackColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub SetFallbackColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub LuminosityOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub SetLuminosityOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows::core::HRESULT,
    pub TintColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub SetTintColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub TintOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub SetTintOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMicaControllerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMicaControllerStatics {
    type Vtable = IMicaControllerStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x7d85d834_d514_5250_b7c4_0b7850d1efdc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicaControllerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsSupported: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'UI_Composition_SystemBackdrops'*"]
#[repr(transparent)]
pub struct ISystemBackdropController(::windows::core::IUnknown);
impl ISystemBackdropController {
    #[doc = "*Required features: 'UI_Composition_SystemBackdrops'*"]
    pub fn SetTargetWithWindowId<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::WindowId>,
        Param1: ::windows::core::IntoParam<'a, ::windows::UI::Composition::CompositionTarget>,
    >(
        &self,
        windowid: Param0,
        desktopwindowtarget: Param1,
    ) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetTargetWithWindowId)(
                ::core::mem::transmute_copy(this),
                windowid.into_param().abi(),
                desktopwindowtarget.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Composition_SystemBackdrops'*"]
    pub fn SetTargetWithCoreWindow<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::UI::Core::CoreWindow>,
        Param1: ::windows::core::IntoParam<'a, ::windows::UI::Composition::CompositionTarget>,
    >(
        &self,
        corewindow: Param0,
        compositiontarget: Param1,
    ) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetTargetWithCoreWindow)(
                ::core::mem::transmute_copy(this),
                corewindow.into_param().abi(),
                compositiontarget.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Composition_SystemBackdrops'*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok()
        }
    }
}
impl ::core::convert::From<ISystemBackdropController> for ::windows::core::IUnknown {
    fn from(value: ISystemBackdropController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISystemBackdropController> for ::windows::core::IUnknown {
    fn from(value: &ISystemBackdropController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISystemBackdropController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a ISystemBackdropController
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISystemBackdropController> for ::windows::core::IInspectable {
    fn from(value: ISystemBackdropController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISystemBackdropController> for ::windows::core::IInspectable {
    fn from(value: &ISystemBackdropController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for ISystemBackdropController
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a ISystemBackdropController
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ISystemBackdropController> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: ISystemBackdropController) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ISystemBackdropController> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &ISystemBackdropController) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable>
    for ISystemBackdropController
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable>
    for &ISystemBackdropController
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::core::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for ISystemBackdropController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISystemBackdropController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISystemBackdropController {}
impl ::core::fmt::Debug for ISystemBackdropController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISystemBackdropController")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ISystemBackdropController {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{5632d76c-0b74-5b52-aa33-80262068aeb2}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ISystemBackdropController {
    type Vtable = ISystemBackdropController_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x5632d76c_0b74_5b52_aa33_80262068aeb2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemBackdropController_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetTargetWithWindowId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        windowid: super::super::WindowId,
        desktopwindowtarget: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetTargetWithCoreWindow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        corewindow: ::windows::core::RawPtr,
        compositiontarget: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'UI_Composition_SystemBackdrops'*"]
#[repr(transparent)]
pub struct MicaController(::windows::core::IUnknown);
impl MicaController {
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
            MicaController,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Composition_SystemBackdrops'*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: 'UI_Composition_SystemBackdrops'*"]
    pub fn FallbackColor(&self) -> ::windows::core::Result<::windows::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: ::windows::UI::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FallbackColor)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Composition_SystemBackdrops'*"]
    pub fn SetFallbackColor<'a, Param0: ::windows::core::IntoParam<'a, ::windows::UI::Color>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFallbackColor)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'UI_Composition_SystemBackdrops'*"]
    pub fn LuminosityOpacity(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LuminosityOpacity)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Composition_SystemBackdrops'*"]
    pub fn SetLuminosityOpacity(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetLuminosityOpacity)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'UI_Composition_SystemBackdrops'*"]
    pub fn TintColor(&self) -> ::windows::core::Result<::windows::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: ::windows::UI::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TintColor)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Composition_SystemBackdrops'*"]
    pub fn SetTintColor<'a, Param0: ::windows::core::IntoParam<'a, ::windows::UI::Color>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetTintColor)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'UI_Composition_SystemBackdrops'*"]
    pub fn TintOpacity(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TintOpacity)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Composition_SystemBackdrops'*"]
    pub fn SetTintOpacity(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetTintOpacity)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'UI_Composition_SystemBackdrops'*"]
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::IMicaControllerStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSupported)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Composition_SystemBackdrops'*"]
    pub fn SetTargetWithWindowId<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::WindowId>,
        Param1: ::windows::core::IntoParam<'a, ::windows::UI::Composition::CompositionTarget>,
    >(
        &self,
        windowid: Param0,
        desktopwindowtarget: Param1,
    ) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISystemBackdropController>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetTargetWithWindowId)(
                ::core::mem::transmute_copy(this),
                windowid.into_param().abi(),
                desktopwindowtarget.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Composition_SystemBackdrops'*"]
    pub fn SetTargetWithCoreWindow<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::UI::Core::CoreWindow>,
        Param1: ::windows::core::IntoParam<'a, ::windows::UI::Composition::CompositionTarget>,
    >(
        &self,
        corewindow: Param0,
        compositiontarget: Param1,
    ) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISystemBackdropController>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetTargetWithCoreWindow)(
                ::core::mem::transmute_copy(this),
                corewindow.into_param().abi(),
                compositiontarget.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IMicaControllerStatics<
        R,
        F: FnOnce(&IMicaControllerStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MicaController, IMicaControllerStatics> =
            ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MicaController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MicaController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MicaController {}
impl ::core::fmt::Debug for MicaController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MicaController").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MicaController {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.SystemBackdrops.MicaController;{2de996a9-0a2a-5889-a89c-1f84060a8cab})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MicaController {
    type Vtable = IMicaController_Vtbl;
    const IID: ::windows::core::GUID = <IMicaController as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MicaController {
    const NAME: &'static str = "Microsoft.UI.Composition.SystemBackdrops.MicaController";
}
impl ::core::convert::From<MicaController> for ::windows::core::IUnknown {
    fn from(value: MicaController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MicaController> for ::windows::core::IUnknown {
    fn from(value: &MicaController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MicaController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MicaController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MicaController> for ::windows::core::IInspectable {
    fn from(value: MicaController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MicaController> for ::windows::core::IInspectable {
    fn from(value: &MicaController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MicaController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MicaController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<MicaController> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: MicaController) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MicaController> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &MicaController) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable> for MicaController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable> for &MicaController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::core::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<MicaController> for ISystemBackdropController {
    type Error = ::windows::core::Error;
    fn try_from(value: MicaController) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MicaController> for ISystemBackdropController {
    type Error = ::windows::core::Error;
    fn try_from(value: &MicaController) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISystemBackdropController> for MicaController {
    fn into_param(self) -> ::windows::core::Param<'a, ISystemBackdropController> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISystemBackdropController> for &MicaController {
    fn into_param(self) -> ::windows::core::Param<'a, ISystemBackdropController> {
        ::core::convert::TryInto::<ISystemBackdropController>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MicaController {}
unsafe impl ::core::marker::Sync for MicaController {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
