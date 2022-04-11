#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clashing_extern_declarations,
    clippy::all
)]
#[doc = "*Required features: `\"UI_Composition_Core\"`*"]
#[repr(transparent)]
pub struct CompositorController(::windows::core::IUnknown);
impl CompositorController {
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
            CompositorController,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::from_library(b"dcompi.dll\0");
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Composition_Core\"`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Core\"`*"]
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Compositor)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Core\"`*"]
    pub fn Commit(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Commit)(::core::mem::transmute_copy(this))
                .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Core\"`*"]
    pub fn EnsurePreviousCommitCompletedAsync(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EnsurePreviousCommitCompletedAsync)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Core\"`*"]
    pub fn CommitNeeded<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                CompositorController,
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
            (::windows::core::Interface::vtable(this).CommitNeeded)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Core\"`*"]
    pub fn RemoveCommitNeeded<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveCommitNeeded)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for CompositorController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CompositorController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CompositorController {}
impl ::core::fmt::Debug for CompositorController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CompositorController")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CompositorController {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Core.CompositorController;{cc107cdc-558f-5d1a-96a5-a735ac04386b})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CompositorController {
    type Vtable = ICompositorController_Vtbl;
    const IID: ::windows::core::GUID = <ICompositorController as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CompositorController {
    const NAME: &'static str = "Microsoft.UI.Composition.Core.CompositorController";
}
impl ::core::convert::From<CompositorController> for ::windows::core::IUnknown {
    fn from(value: CompositorController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CompositorController> for ::windows::core::IUnknown {
    fn from(value: &CompositorController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CompositorController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CompositorController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CompositorController> for ::windows::core::IInspectable {
    fn from(value: CompositorController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CompositorController> for ::windows::core::IInspectable {
    fn from(value: &CompositorController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CompositorController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CompositorController
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<CompositorController> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CompositorController) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CompositorController> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CompositorController) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable> for CompositorController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable>
    for &CompositorController
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::core::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for CompositorController {}
unsafe impl ::core::marker::Sync for CompositorController {}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompositorController(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICompositorController {
    type Vtable = ICompositorController_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xcc107cdc_558f_5d1a_96a5_a735ac04386b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositorController_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Compositor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub Commit:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnsurePreviousCommitCompletedAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    )
        -> ::windows::core::HRESULT,
    pub CommitNeeded: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveCommitNeeded: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
