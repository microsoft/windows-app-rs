#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clashing_extern_declarations,
    clippy::all
)]
#[doc = "*Required features: `\"UI_Input_Experimental\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ExpFocusChangedDirection(pub i32);
impl ExpFocusChangedDirection {
    pub const None: Self = Self(0i32);
    pub const Next: Self = Self(1i32);
    pub const Previous: Self = Self(2i32);
    pub const Up: Self = Self(3i32);
    pub const Down: Self = Self(4i32);
    pub const Left: Self = Self(5i32);
    pub const Right: Self = Self(6i32);
}
impl ::core::marker::Copy for ExpFocusChangedDirection {}
impl ::core::clone::Clone for ExpFocusChangedDirection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ExpFocusChangedDirection {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ExpFocusChangedDirection {
    type Abi = Self;
}
impl ::core::fmt::Debug for ExpFocusChangedDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExpFocusChangedDirection")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ExpFocusChangedDirection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Input.Experimental.ExpFocusChangedDirection;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Input_Experimental\"`*"]
#[repr(transparent)]
pub struct ExpFocusChangedEventArgs(::windows::core::IUnknown);
impl ExpFocusChangedEventArgs {
    #[doc = "*Required features: `\"UI_Input_Experimental\"`*"]
    pub fn Direction(&self) -> ::windows::core::Result<ExpFocusChangedDirection> {
        let this = self;
        unsafe {
            let mut result__: ExpFocusChangedDirection = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Direction)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ExpFocusChangedDirection>(result__)
        }
    }
}
impl ::core::clone::Clone for ExpFocusChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ExpFocusChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ExpFocusChangedEventArgs {}
impl ::core::fmt::Debug for ExpFocusChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExpFocusChangedEventArgs")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ExpFocusChangedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Input.Experimental.ExpFocusChangedEventArgs;{714933b6-ba0d-58bc-97a9-bcc89a0b3431})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ExpFocusChangedEventArgs {
    type Vtable = IExpFocusChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        <IExpFocusChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ExpFocusChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.Experimental.ExpFocusChangedEventArgs";
}
impl ::core::convert::From<ExpFocusChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ExpFocusChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExpFocusChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ExpFocusChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ExpFocusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a ExpFocusChangedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ExpFocusChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ExpFocusChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExpFocusChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ExpFocusChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for ExpFocusChangedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a ExpFocusChangedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ExpFocusChangedEventArgs {}
unsafe impl ::core::marker::Sync for ExpFocusChangedEventArgs {}
#[doc = "*Required features: `\"UI_Input_Experimental\"`*"]
#[repr(transparent)]
pub struct ExpFocusController(::windows::core::IUnknown);
impl ExpFocusController {
    #[doc = "*Required features: `\"UI_Input_Experimental\"`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input_Experimental\"`*"]
    pub fn HasFocus(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HasFocus)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input_Experimental\"`*"]
    pub fn TrySetFocus(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TrySetFocus)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input_Experimental\"`*"]
    pub fn DepartFocus<'a, Param0: ::windows::core::IntoParam<'a, ExpFocusNavigationRequest>>(
        &self,
        request: Param0,
    ) -> ::windows::core::Result<ExpFocusNavigationResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DepartFocus)(
                ::core::mem::transmute_copy(this),
                request.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ExpFocusNavigationResult>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input_Experimental\"`*"]
    pub fn NavigateFocusRequested<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                ExpFocusController,
                ExpNavigateFocusRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NavigateFocusRequested)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input_Experimental\"`*"]
    pub fn RemoveNavigateFocusRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveNavigateFocusRequested)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input_Experimental\"`*"]
    pub fn GetForInputSite<'a, Param0: ::windows::core::IntoParam<'a, ExpInputSite>>(
        inputsite: Param0,
    ) -> ::windows::core::Result<ExpFocusController> {
        Self::IExpFocusControllerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForInputSite)(
                ::core::mem::transmute_copy(this),
                inputsite.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ExpFocusController>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Input_Experimental\"`, `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<IExpInputObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input_Experimental\"`*"]
    pub fn InputSite(&self) -> ::windows::core::Result<ExpInputSite> {
        let this = &::windows::core::Interface::cast::<IExpInputObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InputSite)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ExpInputSite>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IExpFocusControllerStatics<
        R,
        F: FnOnce(&IExpFocusControllerStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            ExpFocusController,
            IExpFocusControllerStatics,
        > = ::windows::core::FactoryCache::from_library(b"Microsoft.UI.Input.dll\0");
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ExpFocusController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ExpFocusController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ExpFocusController {}
impl ::core::fmt::Debug for ExpFocusController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExpFocusController").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ExpFocusController {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Input.Experimental.ExpFocusController;{fd86e2d9-1550-59ba-8b4a-973c7251cd2c})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ExpFocusController {
    type Vtable = IExpFocusController_Vtbl;
    const IID: ::windows::core::GUID = <IExpFocusController as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ExpFocusController {
    const NAME: &'static str = "Microsoft.UI.Input.Experimental.ExpFocusController";
}
impl ::core::convert::From<ExpFocusController> for ::windows::core::IUnknown {
    fn from(value: ExpFocusController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExpFocusController> for ::windows::core::IUnknown {
    fn from(value: &ExpFocusController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ExpFocusController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ExpFocusController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ExpFocusController> for ::windows::core::IInspectable {
    fn from(value: ExpFocusController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExpFocusController> for ::windows::core::IInspectable {
    fn from(value: &ExpFocusController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ExpFocusController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ExpFocusController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ExpFocusController> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: ExpFocusController) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ExpFocusController> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &ExpFocusController) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable> for ExpFocusController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable> for &ExpFocusController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::core::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<ExpFocusController> for ExpInputObject {
    fn from(value: ExpFocusController) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ExpFocusController> for ExpInputObject {
    fn from(value: &ExpFocusController) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ExpInputObject> for ExpFocusController {
    fn into_param(self) -> ::windows::core::Param<'a, ExpInputObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ExpInputObject> for &ExpFocusController {
    fn into_param(self) -> ::windows::core::Param<'a, ExpInputObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<ExpInputObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for ExpFocusController {}
unsafe impl ::core::marker::Sync for ExpFocusController {}
#[doc = "*Required features: `\"UI_Input_Experimental\"`*"]
#[repr(transparent)]
pub struct ExpFocusNavigationHost(::windows::core::IUnknown);
impl ExpFocusNavigationHost {
    #[doc = "*Required features: `\"UI_Input_Experimental\"`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input_Experimental\"`*"]
    pub fn NavigateFocus<'a, Param0: ::windows::core::IntoParam<'a, ExpFocusNavigationRequest>>(
        &self,
        request: Param0,
    ) -> ::windows::core::Result<ExpFocusNavigationResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NavigateFocus)(
                ::core::mem::transmute_copy(this),
                request.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ExpFocusNavigationResult>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input_Experimental\"`*"]
    pub fn TakeFocusRequested<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                ExpFocusNavigationHost,
                ExpNavigateFocusRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TakeFocusRequested)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input_Experimental\"`*"]
    pub fn RemoveTakeFocusRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveTakeFocusRequested)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input_Experimental\"`*"]
    pub fn GetForInputSite<'a, Param0: ::windows::core::IntoParam<'a, ExpInputSite>>(
        inputsite: Param0,
    ) -> ::windows::core::Result<ExpFocusNavigationHost> {
        Self::IExpFocusNavigationHostStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForInputSite)(
                ::core::mem::transmute_copy(this),
                inputsite.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ExpFocusNavigationHost>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Input_Experimental\"`, `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<IExpInputObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input_Experimental\"`*"]
    pub fn InputSite(&self) -> ::windows::core::Result<ExpInputSite> {
        let this = &::windows::core::Interface::cast::<IExpInputObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InputSite)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ExpInputSite>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IExpFocusNavigationHostStatics<
        R,
        F: FnOnce(&IExpFocusNavigationHostStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            ExpFocusNavigationHost,
            IExpFocusNavigationHostStatics,
        > = ::windows::core::FactoryCache::from_library(b"Microsoft.UI.Input.dll\0");
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ExpFocusNavigationHost {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ExpFocusNavigationHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ExpFocusNavigationHost {}
impl ::core::fmt::Debug for ExpFocusNavigationHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExpFocusNavigationHost")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ExpFocusNavigationHost {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Input.Experimental.ExpFocusNavigationHost;{49b02d03-b0d8-55db-985a-48611e47e838})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ExpFocusNavigationHost {
    type Vtable = IExpFocusNavigationHost_Vtbl;
    const IID: ::windows::core::GUID = <IExpFocusNavigationHost as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ExpFocusNavigationHost {
    const NAME: &'static str = "Microsoft.UI.Input.Experimental.ExpFocusNavigationHost";
}
impl ::core::convert::From<ExpFocusNavigationHost> for ::windows::core::IUnknown {
    fn from(value: ExpFocusNavigationHost) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExpFocusNavigationHost> for ::windows::core::IUnknown {
    fn from(value: &ExpFocusNavigationHost) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ExpFocusNavigationHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ExpFocusNavigationHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ExpFocusNavigationHost> for ::windows::core::IInspectable {
    fn from(value: ExpFocusNavigationHost) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExpFocusNavigationHost> for ::windows::core::IInspectable {
    fn from(value: &ExpFocusNavigationHost) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ExpFocusNavigationHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a ExpFocusNavigationHost
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ExpFocusNavigationHost> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: ExpFocusNavigationHost) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ExpFocusNavigationHost> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &ExpFocusNavigationHost) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable>
    for ExpFocusNavigationHost
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable>
    for &ExpFocusNavigationHost
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::core::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<ExpFocusNavigationHost> for ExpInputObject {
    fn from(value: ExpFocusNavigationHost) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ExpFocusNavigationHost> for ExpInputObject {
    fn from(value: &ExpFocusNavigationHost) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ExpInputObject> for ExpFocusNavigationHost {
    fn into_param(self) -> ::windows::core::Param<'a, ExpInputObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ExpInputObject> for &ExpFocusNavigationHost {
    fn into_param(self) -> ::windows::core::Param<'a, ExpInputObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<ExpInputObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for ExpFocusNavigationHost {}
unsafe impl ::core::marker::Sync for ExpFocusNavigationHost {}
#[doc = "*Required features: `\"UI_Input_Experimental\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ExpFocusNavigationReason(pub i32);
impl ExpFocusNavigationReason {
    pub const Programmatic: Self = Self(0i32);
    pub const Restore: Self = Self(1i32);
    pub const First: Self = Self(2i32);
    pub const Last: Self = Self(3i32);
    pub const Left: Self = Self(4i32);
    pub const Up: Self = Self(5i32);
    pub const Right: Self = Self(6i32);
    pub const Down: Self = Self(7i32);
}
impl ::core::marker::Copy for ExpFocusNavigationReason {}
impl ::core::clone::Clone for ExpFocusNavigationReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ExpFocusNavigationReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ExpFocusNavigationReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for ExpFocusNavigationReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExpFocusNavigationReason")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ExpFocusNavigationReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Input.Experimental.ExpFocusNavigationReason;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Input_Experimental\"`*"]
#[repr(transparent)]
pub struct ExpFocusNavigationRequest(::windows::core::IUnknown);
impl ExpFocusNavigationRequest {
    #[doc = "*Required features: `\"UI_Input_Experimental\"`*"]
    pub fn CorrelationId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CorrelationId)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input_Experimental\"`*"]
    pub fn HintRect(&self) -> ::windows::core::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HintRect)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input_Experimental\"`*"]
    pub fn Reason(&self) -> ::windows::core::Result<ExpFocusNavigationReason> {
        let this = self;
        unsafe {
            let mut result__: ExpFocusNavigationReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Reason)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ExpFocusNavigationReason>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input_Experimental\"`*"]
    pub fn CreateFocusNavigationRequestReasonAndHintRect<
        'a,
        Param1: ::windows::core::IntoParam<'a, ::windows::Foundation::Rect>,
    >(
        reason: ExpFocusNavigationReason,
        hintrect: Param1,
    ) -> ::windows::core::Result<ExpFocusNavigationRequest> {
        Self::IExpFocusNavigationRequestStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            ( :: windows :: core :: Interface :: vtable ( this ) . CreateFocusNavigationRequestReasonAndHintRect ) ( :: core :: mem :: transmute_copy ( this ) , reason , hintrect . into_param ( ) . abi ( ) , & mut result__ ) . from_abi :: < ExpFocusNavigationRequest > ( result__ )
        })
    }
    #[doc = "*Required features: `\"UI_Input_Experimental\"`*"]
    pub fn CreateFocusNavigationRequestReasonHintRectAndId<
        'a,
        Param1: ::windows::core::IntoParam<'a, ::windows::Foundation::Rect>,
        Param2: ::windows::core::IntoParam<'a, ::windows::core::GUID>,
    >(
        reason: ExpFocusNavigationReason,
        hintrect: Param1,
        correlationid: Param2,
    ) -> ::windows::core::Result<ExpFocusNavigationRequest> {
        Self::IExpFocusNavigationRequestStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this)
                .CreateFocusNavigationRequestReasonHintRectAndId)(
                ::core::mem::transmute_copy(this),
                reason,
                hintrect.into_param().abi(),
                correlationid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ExpFocusNavigationRequest>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Input_Experimental\"`*"]
    pub fn CreateFocusNavigationRequestWithReason(
        reason: ExpFocusNavigationReason,
    ) -> ::windows::core::Result<ExpFocusNavigationRequest> {
        Self::IExpFocusNavigationRequestStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFocusNavigationRequestWithReason)(
                ::core::mem::transmute_copy(this),
                reason,
                &mut result__,
            )
            .from_abi::<ExpFocusNavigationRequest>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IExpFocusNavigationRequestStatics<
        R,
        F: FnOnce(&IExpFocusNavigationRequestStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            ExpFocusNavigationRequest,
            IExpFocusNavigationRequestStatics,
        > = ::windows::core::FactoryCache::from_library(b"Microsoft.UI.Input.dll\0");
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ExpFocusNavigationRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ExpFocusNavigationRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ExpFocusNavigationRequest {}
impl ::core::fmt::Debug for ExpFocusNavigationRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExpFocusNavigationRequest")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ExpFocusNavigationRequest {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Input.Experimental.ExpFocusNavigationRequest;{4de7c4d0-6f4b-5176-9ae2-346d3318db4e})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ExpFocusNavigationRequest {
    type Vtable = IExpFocusNavigationRequest_Vtbl;
    const IID: ::windows::core::GUID =
        <IExpFocusNavigationRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ExpFocusNavigationRequest {
    const NAME: &'static str = "Microsoft.UI.Input.Experimental.ExpFocusNavigationRequest";
}
impl ::core::convert::From<ExpFocusNavigationRequest> for ::windows::core::IUnknown {
    fn from(value: ExpFocusNavigationRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExpFocusNavigationRequest> for ::windows::core::IUnknown {
    fn from(value: &ExpFocusNavigationRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ExpFocusNavigationRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a ExpFocusNavigationRequest
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ExpFocusNavigationRequest> for ::windows::core::IInspectable {
    fn from(value: ExpFocusNavigationRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExpFocusNavigationRequest> for ::windows::core::IInspectable {
    fn from(value: &ExpFocusNavigationRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for ExpFocusNavigationRequest
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a ExpFocusNavigationRequest
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ExpFocusNavigationRequest {}
unsafe impl ::core::marker::Sync for ExpFocusNavigationRequest {}
#[doc = "*Required features: `\"UI_Input_Experimental\"`*"]
#[repr(transparent)]
pub struct ExpFocusNavigationResult(::windows::core::IUnknown);
impl ExpFocusNavigationResult {
    #[doc = "*Required features: `\"UI_Input_Experimental\"`*"]
    pub fn WasMoved(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WasMoved)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input_Experimental\"`*"]
    pub fn SetWasMoved(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetWasMoved)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for ExpFocusNavigationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ExpFocusNavigationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ExpFocusNavigationResult {}
impl ::core::fmt::Debug for ExpFocusNavigationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExpFocusNavigationResult")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ExpFocusNavigationResult {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Input.Experimental.ExpFocusNavigationResult;{33007903-6cd1-54a2-a5d3-5724ef673c6c})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ExpFocusNavigationResult {
    type Vtable = IExpFocusNavigationResult_Vtbl;
    const IID: ::windows::core::GUID =
        <IExpFocusNavigationResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ExpFocusNavigationResult {
    const NAME: &'static str = "Microsoft.UI.Input.Experimental.ExpFocusNavigationResult";
}
impl ::core::convert::From<ExpFocusNavigationResult> for ::windows::core::IUnknown {
    fn from(value: ExpFocusNavigationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExpFocusNavigationResult> for ::windows::core::IUnknown {
    fn from(value: &ExpFocusNavigationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ExpFocusNavigationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a ExpFocusNavigationResult
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ExpFocusNavigationResult> for ::windows::core::IInspectable {
    fn from(value: ExpFocusNavigationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExpFocusNavigationResult> for ::windows::core::IInspectable {
    fn from(value: &ExpFocusNavigationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for ExpFocusNavigationResult
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a ExpFocusNavigationResult
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ExpFocusNavigationResult {}
unsafe impl ::core::marker::Sync for ExpFocusNavigationResult {}
#[doc = "*Required features: `\"UI_Input_Experimental\"`*"]
#[repr(transparent)]
pub struct ExpIndependentPointerInputObserver(::windows::core::IUnknown);
impl ExpIndependentPointerInputObserver {
    #[doc = "*Required features: `\"UI_Input_Experimental\"`, `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn CreateForVisual<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::Composition::Visual>,
    >(
        visual: Param0,
        devicetypes: ::windows::UI::Core::CoreInputDeviceTypes,
    ) -> ::windows::core::Result<ExpIndependentPointerInputObserver> {
        Self::IExpIndependentPointerInputObserverStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateForVisual)(
                ::core::mem::transmute_copy(this),
                visual.into_param().abi(),
                devicetypes,
                &mut result__,
            )
            .from_abi::<ExpIndependentPointerInputObserver>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IExpIndependentPointerInputObserverStatics<
        R,
        F: FnOnce(&IExpIndependentPointerInputObserverStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            ExpIndependentPointerInputObserver,
            IExpIndependentPointerInputObserverStatics,
        > = ::windows::core::FactoryCache::from_library(b"Microsoft.UI.Input.dll\0");
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ExpIndependentPointerInputObserver {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ExpIndependentPointerInputObserver {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ExpIndependentPointerInputObserver {}
impl ::core::fmt::Debug for ExpIndependentPointerInputObserver {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExpIndependentPointerInputObserver")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ExpIndependentPointerInputObserver {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Input.Experimental.ExpIndependentPointerInputObserver;{197ccc46-6940-56aa-bc0b-db9bff9d5f85})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ExpIndependentPointerInputObserver {
    type Vtable = IExpIndependentPointerInputObserver_Vtbl;
    const IID: ::windows::core::GUID =
        <IExpIndependentPointerInputObserver as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ExpIndependentPointerInputObserver {
    const NAME: &'static str = "Microsoft.UI.Input.Experimental.ExpIndependentPointerInputObserver";
}
impl ::core::convert::From<ExpIndependentPointerInputObserver> for ::windows::core::IUnknown {
    fn from(value: ExpIndependentPointerInputObserver) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExpIndependentPointerInputObserver> for ::windows::core::IUnknown {
    fn from(value: &ExpIndependentPointerInputObserver) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for ExpIndependentPointerInputObserver
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a ExpIndependentPointerInputObserver
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ExpIndependentPointerInputObserver> for ::windows::core::IInspectable {
    fn from(value: ExpIndependentPointerInputObserver) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExpIndependentPointerInputObserver> for ::windows::core::IInspectable {
    fn from(value: &ExpIndependentPointerInputObserver) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for ExpIndependentPointerInputObserver
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a ExpIndependentPointerInputObserver
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ExpIndependentPointerInputObserver {}
unsafe impl ::core::marker::Sync for ExpIndependentPointerInputObserver {}
#[doc = "*Required features: `\"UI_Input_Experimental\"`*"]
#[repr(transparent)]
pub struct ExpInputObject(::windows::core::IUnknown);
impl ExpInputObject {
    #[doc = "*Required features: `\"UI_Input_Experimental\"`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input_Experimental\"`, `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input_Experimental\"`*"]
    pub fn InputSite(&self) -> ::windows::core::Result<ExpInputSite> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InputSite)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ExpInputSite>(result__)
        }
    }
}
impl ::core::clone::Clone for ExpInputObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ExpInputObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ExpInputObject {}
impl ::core::fmt::Debug for ExpInputObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExpInputObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ExpInputObject {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Input.Experimental.ExpInputObject;{a9f18af5-34b0-5227-b981-1ec408d730da})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ExpInputObject {
    type Vtable = IExpInputObject_Vtbl;
    const IID: ::windows::core::GUID = <IExpInputObject as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ExpInputObject {
    const NAME: &'static str = "Microsoft.UI.Input.Experimental.ExpInputObject";
}
impl ::core::convert::From<ExpInputObject> for ::windows::core::IUnknown {
    fn from(value: ExpInputObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExpInputObject> for ::windows::core::IUnknown {
    fn from(value: &ExpInputObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ExpInputObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ExpInputObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ExpInputObject> for ::windows::core::IInspectable {
    fn from(value: ExpInputObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExpInputObject> for ::windows::core::IInspectable {
    fn from(value: &ExpInputObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ExpInputObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ExpInputObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ExpInputObject> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: ExpInputObject) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ExpInputObject> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &ExpInputObject) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable> for ExpInputObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable> for &ExpInputObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::core::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ExpInputObject {}
unsafe impl ::core::marker::Sync for ExpInputObject {}
#[doc = "*Required features: `\"UI_Input_Experimental\"`*"]
#[repr(transparent)]
pub struct ExpInputSite(::windows::core::IUnknown);
impl ExpInputSite {
    #[doc = "*Required features: `\"UI_Input_Experimental\"`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok()
        }
    }
}
impl ::core::clone::Clone for ExpInputSite {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ExpInputSite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ExpInputSite {}
impl ::core::fmt::Debug for ExpInputSite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExpInputSite").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ExpInputSite {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Input.Experimental.ExpInputSite;{6b707b95-bbe8-5131-a6d7-b11c26cb7cb6})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ExpInputSite {
    type Vtable = IExpInputSite_Vtbl;
    const IID: ::windows::core::GUID = <IExpInputSite as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ExpInputSite {
    const NAME: &'static str = "Microsoft.UI.Input.Experimental.ExpInputSite";
}
impl ::core::convert::From<ExpInputSite> for ::windows::core::IUnknown {
    fn from(value: ExpInputSite) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExpInputSite> for ::windows::core::IUnknown {
    fn from(value: &ExpInputSite) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ExpInputSite {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ExpInputSite {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ExpInputSite> for ::windows::core::IInspectable {
    fn from(value: ExpInputSite) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExpInputSite> for ::windows::core::IInspectable {
    fn from(value: &ExpInputSite) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ExpInputSite {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ExpInputSite {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ExpInputSite> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: ExpInputSite) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ExpInputSite> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &ExpInputSite) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable> for ExpInputSite {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable> for &ExpInputSite {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::core::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ExpInputSite {}
unsafe impl ::core::marker::Sync for ExpInputSite {}
#[doc = "*Required features: `\"UI_Input_Experimental\"`*"]
#[repr(transparent)]
pub struct ExpNavigateFocusRequestedEventArgs(::windows::core::IUnknown);
impl ExpNavigateFocusRequestedEventArgs {
    #[doc = "*Required features: `\"UI_Input_Experimental\"`*"]
    pub fn Request(&self) -> ::windows::core::Result<ExpFocusNavigationRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Request)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ExpFocusNavigationRequest>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input_Experimental\"`*"]
    pub fn WasMoved(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WasMoved)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input_Experimental\"`*"]
    pub fn SetWasMoved(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetWasMoved)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for ExpNavigateFocusRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ExpNavigateFocusRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ExpNavigateFocusRequestedEventArgs {}
impl ::core::fmt::Debug for ExpNavigateFocusRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExpNavigateFocusRequestedEventArgs")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ExpNavigateFocusRequestedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Input.Experimental.ExpNavigateFocusRequestedEventArgs;{563b3f71-eca0-5652-b748-13c6d6794cee})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ExpNavigateFocusRequestedEventArgs {
    type Vtable = IExpNavigateFocusRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        <IExpNavigateFocusRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ExpNavigateFocusRequestedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.Experimental.ExpNavigateFocusRequestedEventArgs";
}
impl ::core::convert::From<ExpNavigateFocusRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ExpNavigateFocusRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExpNavigateFocusRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ExpNavigateFocusRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for ExpNavigateFocusRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a ExpNavigateFocusRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ExpNavigateFocusRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ExpNavigateFocusRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExpNavigateFocusRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ExpNavigateFocusRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for ExpNavigateFocusRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a ExpNavigateFocusRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ExpNavigateFocusRequestedEventArgs {}
unsafe impl ::core::marker::Sync for ExpNavigateFocusRequestedEventArgs {}
#[doc = "*Required features: `\"UI_Input_Experimental\"`*"]
#[repr(transparent)]
pub struct ExpPointerPoint(::windows::core::IUnknown);
impl ExpPointerPoint {
    #[doc = "*Required features: `\"UI_Input_Experimental\"`*"]
    pub fn GetCurrentPoint(pointerid: u32) -> ::windows::core::Result<super::PointerPoint> {
        Self::IExpPointerPointStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCurrentPoint)(
                ::core::mem::transmute_copy(this),
                pointerid,
                &mut result__,
            )
            .from_abi::<super::PointerPoint>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Input_Experimental\"`*"]
    pub fn GetCurrentPointTransformed<
        'a,
        Param1: ::windows::core::IntoParam<'a, super::IPointerPointTransform>,
    >(
        pointerid: u32,
        transform: Param1,
    ) -> ::windows::core::Result<super::PointerPoint> {
        Self::IExpPointerPointStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCurrentPointTransformed)(
                ::core::mem::transmute_copy(this),
                pointerid,
                transform.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::PointerPoint>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Input_Experimental\"`*"]
    pub fn GetIntermediatePoints(
        pointerid: u32,
    ) -> ::windows::core::Result<::windows::Foundation::Collections::IVector<super::PointerPoint>>
    {
        Self::IExpPointerPointStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetIntermediatePoints)(
                ::core::mem::transmute_copy(this),
                pointerid,
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVector<super::PointerPoint>>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Input_Experimental\"`*"]
    pub fn GetIntermediatePointsTransformed<
        'a,
        Param1: ::windows::core::IntoParam<'a, super::IPointerPointTransform>,
    >(
        pointerid: u32,
        transform: Param1,
    ) -> ::windows::core::Result<::windows::Foundation::Collections::IVector<super::PointerPoint>>
    {
        Self::IExpPointerPointStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetIntermediatePointsTransformed)(
                ::core::mem::transmute_copy(this),
                pointerid,
                transform.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVector<super::PointerPoint>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IExpPointerPointStatics<
        R,
        F: FnOnce(&IExpPointerPointStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ExpPointerPoint, IExpPointerPointStatics> =
            ::windows::core::FactoryCache::from_library(b"Microsoft.UI.Input.dll\0");
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ExpPointerPoint {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ExpPointerPoint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ExpPointerPoint {}
impl ::core::fmt::Debug for ExpPointerPoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExpPointerPoint").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ExpPointerPoint {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Input.Experimental.ExpPointerPoint;{f9dc0c95-f286-5a1c-b038-7ff84a4e4af5})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ExpPointerPoint {
    type Vtable = IExpPointerPoint_Vtbl;
    const IID: ::windows::core::GUID = <IExpPointerPoint as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ExpPointerPoint {
    const NAME: &'static str = "Microsoft.UI.Input.Experimental.ExpPointerPoint";
}
impl ::core::convert::From<ExpPointerPoint> for ::windows::core::IUnknown {
    fn from(value: ExpPointerPoint) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExpPointerPoint> for ::windows::core::IUnknown {
    fn from(value: &ExpPointerPoint) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ExpPointerPoint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ExpPointerPoint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ExpPointerPoint> for ::windows::core::IInspectable {
    fn from(value: ExpPointerPoint) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExpPointerPoint> for ::windows::core::IInspectable {
    fn from(value: &ExpPointerPoint) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ExpPointerPoint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ExpPointerPoint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ExpPointerPoint {}
unsafe impl ::core::marker::Sync for ExpPointerPoint {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IExpFocusChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IExpFocusChangedEventArgs {
    type Vtable = IExpFocusChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x714933b6_ba0d_58bc_97a9_bcc89a0b3431);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpFocusChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Direction: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ExpFocusChangedDirection,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IExpFocusController(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IExpFocusController {
    type Vtable = IExpFocusController_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xfd86e2d9_1550_59ba_8b4a_973c7251cd2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpFocusController_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub HasFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub TrySetFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub DepartFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        request: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub NavigateFocusRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveNavigateFocusRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IExpFocusControllerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IExpFocusControllerStatics {
    type Vtable = IExpFocusControllerStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xf7d9123f_9364_566b_b6f0_19c49ed142b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpFocusControllerStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub GetForInputSite: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        inputsite: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IExpFocusNavigationHost(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IExpFocusNavigationHost {
    type Vtable = IExpFocusNavigationHost_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x49b02d03_b0d8_55db_985a_48611e47e838);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpFocusNavigationHost_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub NavigateFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        request: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub TakeFocusRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveTakeFocusRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IExpFocusNavigationHostStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IExpFocusNavigationHostStatics {
    type Vtable = IExpFocusNavigationHostStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x7634e568_2177_50d9_9d95_18e3caca2b40);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpFocusNavigationHostStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub GetForInputSite: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        inputsite: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IExpFocusNavigationRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IExpFocusNavigationRequest {
    type Vtable = IExpFocusNavigationRequest_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4de7c4d0_6f4b_5176_9ae2_346d3318db4e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpFocusNavigationRequest_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CorrelationId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
    pub HintRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    pub Reason: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ExpFocusNavigationReason,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IExpFocusNavigationRequestStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IExpFocusNavigationRequestStatics {
    type Vtable = IExpFocusNavigationRequestStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x1cc934bb_ec15_5faa_af75_afe45d0ce1a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpFocusNavigationRequestStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CreateFocusNavigationRequestReasonAndHintRect:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            reason: ExpFocusNavigationReason,
            hintrect: ::windows::Foundation::Rect,
            result__: *mut ::windows::core::RawPtr,
        ) -> ::windows::core::HRESULT,
    pub CreateFocusNavigationRequestReasonHintRectAndId:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            reason: ExpFocusNavigationReason,
            hintrect: ::windows::Foundation::Rect,
            correlationid: ::windows::core::GUID,
            result__: *mut ::windows::core::RawPtr,
        ) -> ::windows::core::HRESULT,
    pub CreateFocusNavigationRequestWithReason:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            reason: ExpFocusNavigationReason,
            result__: *mut ::windows::core::RawPtr,
        ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IExpFocusNavigationResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IExpFocusNavigationResult {
    type Vtable = IExpFocusNavigationResult_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x33007903_6cd1_54a2_a5d3_5724ef673c6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpFocusNavigationResult_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub WasMoved: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetWasMoved: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IExpIndependentPointerInputObserver(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IExpIndependentPointerInputObserver {
    type Vtable = IExpIndependentPointerInputObserver_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x197ccc46_6940_56aa_bc0b_db9bff9d5f85);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpIndependentPointerInputObserver_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IExpIndependentPointerInputObserverStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IExpIndependentPointerInputObserverStatics {
    type Vtable = IExpIndependentPointerInputObserverStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4a2aa824_939a_5da6_b46d_4706a932d53b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpIndependentPointerInputObserverStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "UI_Composition")]
    pub CreateForVisual: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        visual: ::windows::core::RawPtr,
        devicetypes: ::windows::UI::Core::CoreInputDeviceTypes,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CreateForVisual: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IExpInputObject(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IExpInputObject {
    type Vtable = IExpInputObject_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa9f18af5_34b0_5227_b981_1ec408d730da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpInputObject_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "UI_Dispatching")]
    pub DispatcherQueue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))]
    DispatcherQueue: usize,
    pub InputSite: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IExpInputObjectFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IExpInputObjectFactory {
    type Vtable = IExpInputObjectFactory_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x759669eb_ea25_551b_9b8a_4fed55c93c8d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpInputObjectFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IExpInputSite(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IExpInputSite {
    type Vtable = IExpInputSite_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x6b707b95_bbe8_5131_a6d7_b11c26cb7cb6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpInputSite_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IExpNavigateFocusRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IExpNavigateFocusRequestedEventArgs {
    type Vtable = IExpNavigateFocusRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x563b3f71_eca0_5652_b748_13c6d6794cee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpNavigateFocusRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Request: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub WasMoved: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetWasMoved: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IExpPointerPoint(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IExpPointerPoint {
    type Vtable = IExpPointerPoint_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xf9dc0c95_f286_5a1c_b038_7ff84a4e4af5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpPointerPoint_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IExpPointerPointStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IExpPointerPointStatics {
    type Vtable = IExpPointerPointStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xfae05d62_8e52_5bf5_a577_ff07bb15e031);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpPointerPointStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub GetCurrentPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        pointerid: u32,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetCurrentPointTransformed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        pointerid: u32,
        transform: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetIntermediatePoints: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        pointerid: u32,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetIntermediatePointsTransformed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        pointerid: u32,
        transform: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    )
        -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
