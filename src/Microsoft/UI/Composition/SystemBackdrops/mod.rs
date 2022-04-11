#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clashing_extern_declarations,
    clippy::all
)]
#[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
#[repr(transparent)]
pub struct CompositionBackdropPolicy(::windows::core::IUnknown);
impl CompositionBackdropPolicy {
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
            CompositionBackdropPolicy,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::from_library(b"wuceffectsi.dll\0");
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
    pub fn HighContrastBackgroundColor(&self) -> ::windows::core::Result<::windows::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: ::windows::UI::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HighContrastBackgroundColor)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
    pub fn SetHighContrastBackgroundColor<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::UI::Color>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetHighContrastBackgroundColor)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
    pub fn IsHighContrast(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsHighContrast)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
    pub fn SetIsHighContrast(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsHighContrast)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
    pub fn IsInputActive(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsInputActive)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
    pub fn SetIsInputActive(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsInputActive)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
    pub fn Theme(&self) -> ::windows::core::Result<SystemBackdropTheme> {
        let this = self;
        unsafe {
            let mut result__: SystemBackdropTheme = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Theme)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SystemBackdropTheme>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
    pub fn SetTheme(&self, value: SystemBackdropTheme) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetTheme)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for CompositionBackdropPolicy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CompositionBackdropPolicy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CompositionBackdropPolicy {}
impl ::core::fmt::Debug for CompositionBackdropPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CompositionBackdropPolicy")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CompositionBackdropPolicy {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.SystemBackdrops.CompositionBackdropPolicy;{8646927b-dbf9-5636-b63f-64f873f5c32b})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CompositionBackdropPolicy {
    type Vtable = ICompositionBackdropPolicy_Vtbl;
    const IID: ::windows::core::GUID =
        <ICompositionBackdropPolicy as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CompositionBackdropPolicy {
    const NAME: &'static str = "Microsoft.UI.Composition.SystemBackdrops.CompositionBackdropPolicy";
}
impl ::core::convert::From<CompositionBackdropPolicy> for ::windows::core::IUnknown {
    fn from(value: CompositionBackdropPolicy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CompositionBackdropPolicy> for ::windows::core::IUnknown {
    fn from(value: &CompositionBackdropPolicy) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CompositionBackdropPolicy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CompositionBackdropPolicy
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CompositionBackdropPolicy> for ::windows::core::IInspectable {
    fn from(value: CompositionBackdropPolicy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CompositionBackdropPolicy> for ::windows::core::IInspectable {
    fn from(value: &CompositionBackdropPolicy) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CompositionBackdropPolicy
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CompositionBackdropPolicy
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CompositionBackdropPolicy {}
unsafe impl ::core::marker::Sync for CompositionBackdropPolicy {}
#[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
#[repr(transparent)]
pub struct DesktopAcrylicController(::windows::core::IUnknown);
impl DesktopAcrylicController {
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
            DesktopAcrylicController,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::from_library(b"wuceffectsi.dll\0");
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
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
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
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
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
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
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
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
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
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
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
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
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
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
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
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
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
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
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
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
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
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
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
    pub fn State(&self) -> ::windows::core::Result<SystemBackdropState> {
        let this = &::windows::core::Interface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            let mut result__: SystemBackdropState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).State)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SystemBackdropState>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
    pub fn AddTarget<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::UI::Composition::ICompositionSupportsSystemBackdrop,
        >,
    >(
        &self,
        systembackdroptarget: Param0,
    ) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AddTarget)(
                ::core::mem::transmute_copy(this),
                systembackdroptarget.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
    pub fn RemoveAllTargets(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAllTargets)(
                ::core::mem::transmute_copy(this),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
    pub fn RemoveTarget<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::UI::Composition::ICompositionSupportsSystemBackdrop,
        >,
    >(
        &self,
        systembackdroptarget: Param0,
    ) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RemoveTarget)(
                ::core::mem::transmute_copy(this),
                systembackdroptarget.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
    pub fn ResetProperties(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).ResetProperties)(::core::mem::transmute_copy(
                this,
            ))
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
    pub fn SetPolicySource<
        'a,
        Param0: ::windows::core::IntoParam<'a, CompositionBackdropPolicy>,
    >(
        &self,
        policysource: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetPolicySource)(
                ::core::mem::transmute_copy(this),
                policysource.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
    pub fn StateChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                ISystemBackdropControllerWithTargets,
                ::windows::core::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StateChanged)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
    pub fn RemoveStateChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveStateChanged)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        > = ::windows::core::FactoryCache::from_library(b"wuceffectsi.dll\0");
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
impl ::core::convert::TryFrom<DesktopAcrylicController> for ISystemBackdropControllerWithTargets {
    type Error = ::windows::core::Error;
    fn try_from(value: DesktopAcrylicController) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DesktopAcrylicController> for ISystemBackdropControllerWithTargets {
    type Error = ::windows::core::Error;
    fn try_from(value: &DesktopAcrylicController) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISystemBackdropControllerWithTargets>
    for DesktopAcrylicController
{
    fn into_param(self) -> ::windows::core::Param<'a, ISystemBackdropControllerWithTargets> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISystemBackdropControllerWithTargets>
    for &DesktopAcrylicController
{
    fn into_param(self) -> ::windows::core::Param<'a, ISystemBackdropControllerWithTargets> {
        ::core::convert::TryInto::<ISystemBackdropControllerWithTargets>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for DesktopAcrylicController {}
unsafe impl ::core::marker::Sync for DesktopAcrylicController {}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompositionBackdropPolicy(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICompositionBackdropPolicy {
    type Vtable = ICompositionBackdropPolicy_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x8646927b_dbf9_5636_b63f_64f873f5c32b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionBackdropPolicy_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub HighContrastBackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub SetHighContrastBackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub IsHighContrast: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetIsHighContrast: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub IsInputActive: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetIsInputActive: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub Theme: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut SystemBackdropTheme,
    ) -> ::windows::core::HRESULT,
    pub SetTheme: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: SystemBackdropTheme,
    ) -> ::windows::core::HRESULT,
}
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
    pub base__: ::windows::core::IInspectableVtbl,
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
    pub base__: ::windows::core::IInspectableVtbl,
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
    pub base__: ::windows::core::IInspectableVtbl,
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
pub struct IMicaController2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMicaController2 {
    type Vtable = IMicaController2_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x227f737d_ace1_5fa4_a90b_6d810ed8730e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicaController2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Variant: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut MicaVariant,
    ) -> ::windows::core::HRESULT,
    pub SetVariant: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: MicaVariant,
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
    pub base__: ::windows::core::IInspectableVtbl,
    pub IsSupported: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
#[repr(transparent)]
pub struct ISystemBackdropController(::windows::core::IUnknown);
impl ISystemBackdropController {
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
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
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
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
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
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
    pub base__: ::windows::core::IInspectableVtbl,
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
#[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
#[repr(transparent)]
pub struct ISystemBackdropControllerWithTargets(::windows::core::IUnknown);
impl ISystemBackdropControllerWithTargets {
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
    pub fn State(&self) -> ::windows::core::Result<SystemBackdropState> {
        let this = self;
        unsafe {
            let mut result__: SystemBackdropState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).State)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SystemBackdropState>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
    pub fn AddTarget<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::UI::Composition::ICompositionSupportsSystemBackdrop,
        >,
    >(
        &self,
        systembackdroptarget: Param0,
    ) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AddTarget)(
                ::core::mem::transmute_copy(this),
                systembackdroptarget.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
    pub fn RemoveAllTargets(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAllTargets)(
                ::core::mem::transmute_copy(this),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
    pub fn RemoveTarget<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::UI::Composition::ICompositionSupportsSystemBackdrop,
        >,
    >(
        &self,
        systembackdroptarget: Param0,
    ) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RemoveTarget)(
                ::core::mem::transmute_copy(this),
                systembackdroptarget.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
    pub fn ResetProperties(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).ResetProperties)(::core::mem::transmute_copy(
                this,
            ))
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
    pub fn SetPolicySource<
        'a,
        Param0: ::windows::core::IntoParam<'a, CompositionBackdropPolicy>,
    >(
        &self,
        policysource: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetPolicySource)(
                ::core::mem::transmute_copy(this),
                policysource.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
    pub fn StateChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                ISystemBackdropControllerWithTargets,
                ::windows::core::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StateChanged)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
    pub fn RemoveStateChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveStateChanged)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
}
impl ::core::convert::From<ISystemBackdropControllerWithTargets> for ::windows::core::IUnknown {
    fn from(value: ISystemBackdropControllerWithTargets) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISystemBackdropControllerWithTargets> for ::windows::core::IUnknown {
    fn from(value: &ISystemBackdropControllerWithTargets) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for ISystemBackdropControllerWithTargets
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a ISystemBackdropControllerWithTargets
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISystemBackdropControllerWithTargets> for ::windows::core::IInspectable {
    fn from(value: ISystemBackdropControllerWithTargets) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISystemBackdropControllerWithTargets>
    for ::windows::core::IInspectable
{
    fn from(value: &ISystemBackdropControllerWithTargets) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for ISystemBackdropControllerWithTargets
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a ISystemBackdropControllerWithTargets
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISystemBackdropControllerWithTargets {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISystemBackdropControllerWithTargets {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISystemBackdropControllerWithTargets {}
impl ::core::fmt::Debug for ISystemBackdropControllerWithTargets {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISystemBackdropControllerWithTargets")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ISystemBackdropControllerWithTargets {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{9e5734b5-4852-5d63-b20d-1632e053cd60}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ISystemBackdropControllerWithTargets {
    type Vtable = ISystemBackdropControllerWithTargets_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x9e5734b5_4852_5d63_b20d_1632e053cd60);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemBackdropControllerWithTargets_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub State: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut SystemBackdropState,
    ) -> ::windows::core::HRESULT,
    pub AddTarget: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        systembackdroptarget: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub RemoveAllTargets:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveTarget: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        systembackdroptarget: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub ResetProperties:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPolicySource: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        policysource: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub StateChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveStateChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
#[repr(transparent)]
pub struct MicaController(::windows::core::IUnknown);
impl MicaController {
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
            MicaController,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::from_library(b"wuceffectsi.dll\0");
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
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
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
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
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
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
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
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
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
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
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
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
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
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
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
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
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
    pub fn Variant(&self) -> ::windows::core::Result<MicaVariant> {
        let this = &::windows::core::Interface::cast::<IMicaController2>(self)?;
        unsafe {
            let mut result__: MicaVariant = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Variant)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<MicaVariant>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
    pub fn SetVariant(&self, value: MicaVariant) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMicaController2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetVariant)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
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
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
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
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
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
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
    pub fn State(&self) -> ::windows::core::Result<SystemBackdropState> {
        let this = &::windows::core::Interface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            let mut result__: SystemBackdropState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).State)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SystemBackdropState>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
    pub fn AddTarget<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::UI::Composition::ICompositionSupportsSystemBackdrop,
        >,
    >(
        &self,
        systembackdroptarget: Param0,
    ) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AddTarget)(
                ::core::mem::transmute_copy(this),
                systembackdroptarget.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
    pub fn RemoveAllTargets(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAllTargets)(
                ::core::mem::transmute_copy(this),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
    pub fn RemoveTarget<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::UI::Composition::ICompositionSupportsSystemBackdrop,
        >,
    >(
        &self,
        systembackdroptarget: Param0,
    ) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RemoveTarget)(
                ::core::mem::transmute_copy(this),
                systembackdroptarget.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
    pub fn ResetProperties(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).ResetProperties)(::core::mem::transmute_copy(
                this,
            ))
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
    pub fn SetPolicySource<
        'a,
        Param0: ::windows::core::IntoParam<'a, CompositionBackdropPolicy>,
    >(
        &self,
        policysource: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetPolicySource)(
                ::core::mem::transmute_copy(this),
                policysource.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
    pub fn StateChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                ISystemBackdropControllerWithTargets,
                ::windows::core::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StateChanged)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
    pub fn RemoveStateChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveStateChanged)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
            ::windows::core::FactoryCache::from_library(b"wuceffectsi.dll\0");
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
impl ::core::convert::TryFrom<MicaController> for ISystemBackdropControllerWithTargets {
    type Error = ::windows::core::Error;
    fn try_from(value: MicaController) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MicaController> for ISystemBackdropControllerWithTargets {
    type Error = ::windows::core::Error;
    fn try_from(value: &MicaController) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISystemBackdropControllerWithTargets> for MicaController {
    fn into_param(self) -> ::windows::core::Param<'a, ISystemBackdropControllerWithTargets> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISystemBackdropControllerWithTargets> for &MicaController {
    fn into_param(self) -> ::windows::core::Param<'a, ISystemBackdropControllerWithTargets> {
        ::core::convert::TryInto::<ISystemBackdropControllerWithTargets>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MicaController {}
unsafe impl ::core::marker::Sync for MicaController {}
#[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MicaVariant(pub i32);
impl MicaVariant {
    pub const Standard: Self = Self(0i32);
    pub const Alternate: Self = Self(1i32);
}
impl ::core::marker::Copy for MicaVariant {}
impl ::core::clone::Clone for MicaVariant {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MicaVariant {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MicaVariant {
    type Abi = Self;
}
impl ::core::fmt::Debug for MicaVariant {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MicaVariant").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MicaVariant {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Composition.SystemBackdrops.MicaVariant;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SystemBackdropState(pub i32);
impl SystemBackdropState {
    pub const Active: Self = Self(0i32);
    pub const Fallback: Self = Self(1i32);
    pub const HighContrast: Self = Self(2i32);
}
impl ::core::marker::Copy for SystemBackdropState {}
impl ::core::clone::Clone for SystemBackdropState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SystemBackdropState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SystemBackdropState {
    type Abi = Self;
}
impl ::core::fmt::Debug for SystemBackdropState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemBackdropState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SystemBackdropState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Composition.SystemBackdrops.SystemBackdropState;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SystemBackdropTheme(pub i32);
impl SystemBackdropTheme {
    pub const SystemDefault: Self = Self(0i32);
    pub const Light: Self = Self(1i32);
    pub const Dark: Self = Self(2i32);
}
impl ::core::marker::Copy for SystemBackdropTheme {}
impl ::core::clone::Clone for SystemBackdropTheme {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SystemBackdropTheme {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SystemBackdropTheme {
    type Abi = Self;
}
impl ::core::fmt::Debug for SystemBackdropTheme {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemBackdropTheme").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SystemBackdropTheme {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Composition.SystemBackdrops.SystemBackdropTheme;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
