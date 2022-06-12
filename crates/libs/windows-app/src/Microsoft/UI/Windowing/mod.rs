#[doc = "*Required features: `\"UI_Windowing\"`*"]
#[repr(transparent)]
pub struct AppWindow(::windows::core::IUnknown);
impl AppWindow {
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<super::WindowId> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::WindowId>::zeroed();
            (::windows::core::Interface::vtable(this).Id)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::WindowId>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn IsShownInSwitchers(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).IsShownInSwitchers)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn SetIsShownInSwitchers(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsShownInSwitchers)(
                ::windows::core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn IsVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).IsVisible)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn OwnerWindowId(&self) -> ::windows::core::Result<super::WindowId> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::WindowId>::zeroed();
            (::windows::core::Interface::vtable(this).OwnerWindowId)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::WindowId>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn Position(&self) -> ::windows::core::Result<::windows::Graphics::PointInt32> {
        let this = self;
        unsafe {
            let mut result__ =
                ::core::mem::MaybeUninit::<::windows::Graphics::PointInt32>::zeroed();
            (::windows::core::Interface::vtable(this).Position)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Graphics::PointInt32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn Presenter(&self) -> ::windows::core::Result<AppWindowPresenter> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).Presenter)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppWindowPresenter>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn Size(&self) -> ::windows::core::Result<::windows::Graphics::SizeInt32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::Graphics::SizeInt32>::zeroed();
            (::windows::core::Interface::vtable(this).Size)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Graphics::SizeInt32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<
                ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
            >::zeroed();
            (::windows::core::Interface::vtable(this).Title)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn SetTitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetTitle)(
                ::windows::core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn TitleBar(&self) -> ::windows::core::Result<AppWindowTitleBar> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).TitleBar)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppWindowTitleBar>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn Destroy(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Destroy)(::windows::core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn Hide(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Hide)(::windows::core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn Move<'a, Param0: ::windows::core::IntoParam<'a, ::windows::Graphics::PointInt32>>(
        &self,
        position: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Move)(
                ::windows::core::Interface::as_raw(this),
                position.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn MoveAndResize<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Graphics::RectInt32>,
    >(
        &self,
        rect: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).MoveAndResize)(
                ::windows::core::Interface::as_raw(this),
                rect.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn MoveAndResizeRelativeToDisplayArea<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Graphics::RectInt32>,
        Param1: ::windows::core::IntoParam<'a, DisplayArea>,
    >(
        &self,
        rect: Param0,
        displayarea: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).MoveAndResizeRelativeToDisplayArea)(
                ::windows::core::Interface::as_raw(this),
                rect.into_param().abi(),
                displayarea.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn Resize<'a, Param0: ::windows::core::IntoParam<'a, ::windows::Graphics::SizeInt32>>(
        &self,
        size: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Resize)(
                ::windows::core::Interface::as_raw(this),
                size.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn SetIcon<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        iconpath: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIcon)(
                ::windows::core::Interface::as_raw(this),
                iconpath.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn SetIconWithIconId<'a, Param0: ::windows::core::IntoParam<'a, super::IconId>>(
        &self,
        iconid: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIconWithIconId)(
                ::windows::core::Interface::as_raw(this),
                iconid.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn SetPresenter<'a, Param0: ::windows::core::IntoParam<'a, AppWindowPresenter>>(
        &self,
        appwindowpresenter: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetPresenter)(
                ::windows::core::Interface::as_raw(this),
                appwindowpresenter.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn SetPresenterByKind(
        &self,
        appwindowpresenterkind: AppWindowPresenterKind,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetPresenterByKind)(
                ::windows::core::Interface::as_raw(this),
                appwindowpresenterkind,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn Show(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Show)(::windows::core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn ShowWithActivation(&self, activatewindow: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).ShowWithActivation)(
                ::windows::core::Interface::as_raw(this),
                activatewindow,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn Changed<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<AppWindow, AppWindowChangedEventArgs>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ =
                ::core::mem::MaybeUninit::<::windows::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).Changed)(
                ::windows::core::Interface::as_raw(this),
                handler.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn RemoveChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveChanged)(
                ::windows::core::Interface::as_raw(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn Closing<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<AppWindow, AppWindowClosingEventArgs>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ =
                ::core::mem::MaybeUninit::<::windows::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).Closing)(
                ::windows::core::Interface::as_raw(this),
                handler.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn RemoveClosing<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveClosing)(
                ::windows::core::Interface::as_raw(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn Destroying<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<AppWindow, ::windows::core::IInspectable>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ =
                ::core::mem::MaybeUninit::<::windows::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).Destroying)(
                ::windows::core::Interface::as_raw(this),
                handler.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn RemoveDestroying<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveDestroying)(
                ::windows::core::Interface::as_raw(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn ClientSize(&self) -> ::windows::core::Result<::windows::Graphics::SizeInt32> {
        let this = &::windows::core::Interface::cast::<IAppWindow2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::Graphics::SizeInt32>::zeroed();
            (::windows::core::Interface::vtable(this).ClientSize)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Graphics::SizeInt32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn MoveInZOrderAtBottom(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppWindow2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).MoveInZOrderAtBottom)(
                ::windows::core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn MoveInZOrderAtTop(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppWindow2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).MoveInZOrderAtTop)(
                ::windows::core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn MoveInZOrderBelow<'a, Param0: ::windows::core::IntoParam<'a, super::WindowId>>(
        &self,
        windowid: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppWindow2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).MoveInZOrderBelow)(
                ::windows::core::Interface::as_raw(this),
                windowid.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn ResizeClient<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Graphics::SizeInt32>,
    >(
        &self,
        size: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppWindow2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).ResizeClient)(
                ::windows::core::Interface::as_raw(this),
                size.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn ShowOnceWithRequestedStartupState(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppWindow2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).ShowOnceWithRequestedStartupState)(
                ::windows::core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn Create() -> ::windows::core::Result<AppWindow> {
        Self::IAppWindowStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).Create)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppWindow>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn CreateWithPresenter<'a, Param0: ::windows::core::IntoParam<'a, AppWindowPresenter>>(
        appwindowpresenter: Param0,
    ) -> ::windows::core::Result<AppWindow> {
        Self::IAppWindowStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithPresenter)(
                ::windows::core::Interface::as_raw(this),
                appwindowpresenter.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppWindow>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn CreateWithPresenterAndOwner<
        'a,
        Param0: ::windows::core::IntoParam<'a, AppWindowPresenter>,
        Param1: ::windows::core::IntoParam<'a, super::WindowId>,
    >(
        appwindowpresenter: Param0,
        ownerwindowid: Param1,
    ) -> ::windows::core::Result<AppWindow> {
        Self::IAppWindowStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithPresenterAndOwner)(
                ::windows::core::Interface::as_raw(this),
                appwindowpresenter.into_param().abi(),
                ownerwindowid.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppWindow>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn GetFromWindowId<'a, Param0: ::windows::core::IntoParam<'a, super::WindowId>>(
        windowid: Param0,
    ) -> ::windows::core::Result<AppWindow> {
        Self::IAppWindowStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).GetFromWindowId)(
                ::windows::core::Interface::as_raw(this),
                windowid.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppWindow>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppWindowStatics<R, F: FnOnce(&IAppWindowStatics) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppWindow, IAppWindowStatics> =
            ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AppWindow {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppWindow {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppWindow {}
impl ::core::fmt::Debug for AppWindow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindow").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppWindow {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Windowing.AppWindow;{cfa788b3-643b-5c5e-ad4e-321d48a82acd})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppWindow {
    type Vtable = IAppWindow_Vtbl;
    const IID: ::windows::core::GUID = <IAppWindow as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppWindow {
    const NAME: &'static str = "Microsoft.UI.Windowing.AppWindow";
}
impl ::core::convert::From<AppWindow> for ::windows::core::IUnknown {
    fn from(value: AppWindow) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindow> for ::windows::core::IUnknown {
    fn from(value: &AppWindow) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppWindow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppWindow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppWindow> for ::windows::core::IInspectable {
    fn from(value: AppWindow) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindow> for ::windows::core::IInspectable {
    fn from(value: &AppWindow) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppWindow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppWindow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppWindow {}
unsafe impl ::core::marker::Sync for AppWindow {}
#[doc = "*Required features: `\"UI_Windowing\"`*"]
#[repr(transparent)]
pub struct AppWindowChangedEventArgs(::windows::core::IUnknown);
impl AppWindowChangedEventArgs {
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn DidPositionChange(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).DidPositionChange)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn DidPresenterChange(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).DidPresenterChange)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn DidSizeChange(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).DidSizeChange)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn DidVisibilityChange(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).DidVisibilityChange)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn DidZOrderChange(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppWindowChangedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).DidZOrderChange)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn IsZOrderAtBottom(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppWindowChangedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).IsZOrderAtBottom)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn IsZOrderAtTop(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppWindowChangedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).IsZOrderAtTop)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn ZOrderBelowWindowId(&self) -> ::windows::core::Result<super::WindowId> {
        let this = &::windows::core::Interface::cast::<IAppWindowChangedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::WindowId>::zeroed();
            (::windows::core::Interface::vtable(this).ZOrderBelowWindowId)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::WindowId>(result__)
        }
    }
}
impl ::core::clone::Clone for AppWindowChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppWindowChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppWindowChangedEventArgs {}
impl ::core::fmt::Debug for AppWindowChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowChangedEventArgs")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppWindowChangedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Windowing.AppWindowChangedEventArgs;{2182bc5d-fdac-5c3e-bf37-7d8d684e9d1d})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppWindowChangedEventArgs {
    type Vtable = IAppWindowChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        <IAppWindowChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppWindowChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Windowing.AppWindowChangedEventArgs";
}
impl ::core::convert::From<AppWindowChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppWindowChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindowChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppWindowChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppWindowChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a AppWindowChangedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppWindowChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppWindowChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindowChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppWindowChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for AppWindowChangedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a AppWindowChangedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppWindowChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppWindowChangedEventArgs {}
#[doc = "*Required features: `\"UI_Windowing\"`*"]
#[repr(transparent)]
pub struct AppWindowClosingEventArgs(::windows::core::IUnknown);
impl AppWindowClosingEventArgs {
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn Cancel(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).Cancel)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn SetCancel(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetCancel)(
                ::windows::core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for AppWindowClosingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppWindowClosingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppWindowClosingEventArgs {}
impl ::core::fmt::Debug for AppWindowClosingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowClosingEventArgs")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppWindowClosingEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Windowing.AppWindowClosingEventArgs;{0e09d90b-2261-590b-9ad1-8504991d8754})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppWindowClosingEventArgs {
    type Vtable = IAppWindowClosingEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        <IAppWindowClosingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppWindowClosingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Windowing.AppWindowClosingEventArgs";
}
impl ::core::convert::From<AppWindowClosingEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppWindowClosingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindowClosingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppWindowClosingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppWindowClosingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a AppWindowClosingEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppWindowClosingEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppWindowClosingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindowClosingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppWindowClosingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for AppWindowClosingEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a AppWindowClosingEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppWindowClosingEventArgs {}
unsafe impl ::core::marker::Sync for AppWindowClosingEventArgs {}
#[doc = "*Required features: `\"UI_Windowing\"`*"]
#[repr(transparent)]
pub struct AppWindowPresenter(::windows::core::IUnknown);
impl AppWindowPresenter {
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<AppWindowPresenterKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppWindowPresenterKind>::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppWindowPresenterKind>(result__)
        }
    }
}
impl ::core::clone::Clone for AppWindowPresenter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppWindowPresenter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppWindowPresenter {}
impl ::core::fmt::Debug for AppWindowPresenter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowPresenter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppWindowPresenter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Windowing.AppWindowPresenter;{bc3042c2-c6c6-5632-8989-ff0ec6d3b40d})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppWindowPresenter {
    type Vtable = IAppWindowPresenter_Vtbl;
    const IID: ::windows::core::GUID = <IAppWindowPresenter as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppWindowPresenter {
    const NAME: &'static str = "Microsoft.UI.Windowing.AppWindowPresenter";
}
impl ::core::convert::From<AppWindowPresenter> for ::windows::core::IUnknown {
    fn from(value: AppWindowPresenter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindowPresenter> for ::windows::core::IUnknown {
    fn from(value: &AppWindowPresenter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppWindowPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppWindowPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppWindowPresenter> for ::windows::core::IInspectable {
    fn from(value: AppWindowPresenter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindowPresenter> for ::windows::core::IInspectable {
    fn from(value: &AppWindowPresenter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppWindowPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppWindowPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppWindowPresenter {}
unsafe impl ::core::marker::Sync for AppWindowPresenter {}
#[doc = "*Required features: `\"UI_Windowing\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppWindowPresenterKind(pub i32);
impl AppWindowPresenterKind {
    pub const Default: Self = Self(0i32);
    pub const CompactOverlay: Self = Self(1i32);
    pub const FullScreen: Self = Self(2i32);
    pub const Overlapped: Self = Self(3i32);
}
impl ::core::marker::Copy for AppWindowPresenterKind {}
impl ::core::clone::Clone for AppWindowPresenterKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppWindowPresenterKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppWindowPresenterKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppWindowPresenterKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowPresenterKind")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppWindowPresenterKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Windowing.AppWindowPresenterKind;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Windowing\"`*"]
#[repr(transparent)]
pub struct AppWindowTitleBar(::windows::core::IUnknown);
impl AppWindowTitleBar {
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn BackgroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).BackgroundColor)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::IReference<::windows::UI::Color>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn SetBackgroundColor<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::IReference<::windows::UI::Color>>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetBackgroundColor)(
                ::windows::core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn ButtonBackgroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).ButtonBackgroundColor)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::IReference<::windows::UI::Color>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn SetButtonBackgroundColor<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::IReference<::windows::UI::Color>>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetButtonBackgroundColor)(
                ::windows::core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn ButtonForegroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).ButtonForegroundColor)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::IReference<::windows::UI::Color>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn SetButtonForegroundColor<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::IReference<::windows::UI::Color>>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetButtonForegroundColor)(
                ::windows::core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn ButtonHoverBackgroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).ButtonHoverBackgroundColor)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::IReference<::windows::UI::Color>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn SetButtonHoverBackgroundColor<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::IReference<::windows::UI::Color>>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetButtonHoverBackgroundColor)(
                ::windows::core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn ButtonHoverForegroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).ButtonHoverForegroundColor)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::IReference<::windows::UI::Color>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn SetButtonHoverForegroundColor<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::IReference<::windows::UI::Color>>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetButtonHoverForegroundColor)(
                ::windows::core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn ButtonInactiveBackgroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).ButtonInactiveBackgroundColor)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::IReference<::windows::UI::Color>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn SetButtonInactiveBackgroundColor<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::IReference<::windows::UI::Color>>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetButtonInactiveBackgroundColor)(
                ::windows::core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn ButtonInactiveForegroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).ButtonInactiveForegroundColor)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::IReference<::windows::UI::Color>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn SetButtonInactiveForegroundColor<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::IReference<::windows::UI::Color>>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetButtonInactiveForegroundColor)(
                ::windows::core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn ButtonPressedBackgroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).ButtonPressedBackgroundColor)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::IReference<::windows::UI::Color>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn SetButtonPressedBackgroundColor<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::IReference<::windows::UI::Color>>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetButtonPressedBackgroundColor)(
                ::windows::core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn ButtonPressedForegroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).ButtonPressedForegroundColor)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::IReference<::windows::UI::Color>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn SetButtonPressedForegroundColor<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::IReference<::windows::UI::Color>>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetButtonPressedForegroundColor)(
                ::windows::core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn ExtendsContentIntoTitleBar(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).ExtendsContentIntoTitleBar)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn SetExtendsContentIntoTitleBar(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetExtendsContentIntoTitleBar)(
                ::windows::core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn ForegroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).ForegroundColor)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::IReference<::windows::UI::Color>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn SetForegroundColor<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::IReference<::windows::UI::Color>>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetForegroundColor)(
                ::windows::core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn Height(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows::core::Interface::vtable(this).Height)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn IconShowOptions(&self) -> ::windows::core::Result<IconShowOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<IconShowOptions>::zeroed();
            (::windows::core::Interface::vtable(this).IconShowOptions)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<IconShowOptions>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn SetIconShowOptions(&self, value: IconShowOptions) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIconShowOptions)(
                ::windows::core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn InactiveBackgroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).InactiveBackgroundColor)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::IReference<::windows::UI::Color>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn SetInactiveBackgroundColor<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::IReference<::windows::UI::Color>>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetInactiveBackgroundColor)(
                ::windows::core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn InactiveForegroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).InactiveForegroundColor)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::IReference<::windows::UI::Color>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn SetInactiveForegroundColor<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::IReference<::windows::UI::Color>>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetInactiveForegroundColor)(
                ::windows::core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn LeftInset(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows::core::Interface::vtable(this).LeftInset)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn RightInset(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows::core::Interface::vtable(this).RightInset)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn ResetToDefault(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).ResetToDefault)(
                ::windows::core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn SetDragRectangles(
        &self,
        value: &[::windows::Graphics::RectInt32],
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetDragRectangles)(
                ::windows::core::Interface::as_raw(this),
                value.len() as u32,
                ::core::mem::transmute(value.as_ptr()),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn PreferredHeightOption(&self) -> ::windows::core::Result<TitleBarHeightOption> {
        let this = &::windows::core::Interface::cast::<IAppWindowTitleBar2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<TitleBarHeightOption>::zeroed();
            (::windows::core::Interface::vtable(this).PreferredHeightOption)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TitleBarHeightOption>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn SetPreferredHeightOption(
        &self,
        value: TitleBarHeightOption,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppWindowTitleBar2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetPreferredHeightOption)(
                ::windows::core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn IsCustomizationSupported() -> ::windows::core::Result<bool> {
        Self::IAppWindowTitleBarStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).IsCustomizationSupported)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppWindowTitleBarStatics<
        R,
        F: FnOnce(&IAppWindowTitleBarStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            AppWindowTitleBar,
            IAppWindowTitleBarStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AppWindowTitleBar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppWindowTitleBar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppWindowTitleBar {}
impl ::core::fmt::Debug for AppWindowTitleBar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowTitleBar").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppWindowTitleBar {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Windowing.AppWindowTitleBar;{5574efa2-c91c-5700-a363-539c71a7aaf4})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppWindowTitleBar {
    type Vtable = IAppWindowTitleBar_Vtbl;
    const IID: ::windows::core::GUID = <IAppWindowTitleBar as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppWindowTitleBar {
    const NAME: &'static str = "Microsoft.UI.Windowing.AppWindowTitleBar";
}
impl ::core::convert::From<AppWindowTitleBar> for ::windows::core::IUnknown {
    fn from(value: AppWindowTitleBar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindowTitleBar> for ::windows::core::IUnknown {
    fn from(value: &AppWindowTitleBar) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppWindowTitleBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppWindowTitleBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppWindowTitleBar> for ::windows::core::IInspectable {
    fn from(value: AppWindowTitleBar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindowTitleBar> for ::windows::core::IInspectable {
    fn from(value: &AppWindowTitleBar) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppWindowTitleBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppWindowTitleBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppWindowTitleBar {}
unsafe impl ::core::marker::Sync for AppWindowTitleBar {}
#[doc = "*Required features: `\"UI_Windowing\"`*"]
#[repr(transparent)]
pub struct CompactOverlayPresenter(::windows::core::IUnknown);
impl CompactOverlayPresenter {
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<AppWindowPresenterKind> {
        let this = &::windows::core::Interface::cast::<IAppWindowPresenter>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppWindowPresenterKind>::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppWindowPresenterKind>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn InitialSize(&self) -> ::windows::core::Result<CompactOverlaySize> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<CompactOverlaySize>::zeroed();
            (::windows::core::Interface::vtable(this).InitialSize)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<CompactOverlaySize>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn SetInitialSize(&self, value: CompactOverlaySize) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetInitialSize)(
                ::windows::core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn Create() -> ::windows::core::Result<CompactOverlayPresenter> {
        Self::ICompactOverlayPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).Create)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<CompactOverlayPresenter>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICompactOverlayPresenterStatics<
        R,
        F: FnOnce(&ICompactOverlayPresenterStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            CompactOverlayPresenter,
            ICompactOverlayPresenterStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CompactOverlayPresenter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CompactOverlayPresenter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CompactOverlayPresenter {}
impl ::core::fmt::Debug for CompactOverlayPresenter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CompactOverlayPresenter")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CompactOverlayPresenter {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Windowing.CompactOverlayPresenter;{efeb0812-6fc7-5b7d-bd92-cc8f9a6454c9})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CompactOverlayPresenter {
    type Vtable = ICompactOverlayPresenter_Vtbl;
    const IID: ::windows::core::GUID =
        <ICompactOverlayPresenter as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CompactOverlayPresenter {
    const NAME: &'static str = "Microsoft.UI.Windowing.CompactOverlayPresenter";
}
impl ::core::convert::From<CompactOverlayPresenter> for ::windows::core::IUnknown {
    fn from(value: CompactOverlayPresenter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CompactOverlayPresenter> for ::windows::core::IUnknown {
    fn from(value: &CompactOverlayPresenter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CompactOverlayPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CompactOverlayPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CompactOverlayPresenter> for ::windows::core::IInspectable {
    fn from(value: CompactOverlayPresenter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CompactOverlayPresenter> for ::windows::core::IInspectable {
    fn from(value: &CompactOverlayPresenter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CompactOverlayPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CompactOverlayPresenter
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CompactOverlayPresenter> for AppWindowPresenter {
    fn from(value: CompactOverlayPresenter) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CompactOverlayPresenter> for AppWindowPresenter {
    fn from(value: &CompactOverlayPresenter) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, AppWindowPresenter> for CompactOverlayPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, AppWindowPresenter> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, AppWindowPresenter> for &CompactOverlayPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, AppWindowPresenter> {
        ::windows::core::Param::Owned(::core::convert::Into::<AppWindowPresenter>::into(self))
    }
}
unsafe impl ::core::marker::Send for CompactOverlayPresenter {}
unsafe impl ::core::marker::Sync for CompactOverlayPresenter {}
#[doc = "*Required features: `\"UI_Windowing\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CompactOverlaySize(pub i32);
impl CompactOverlaySize {
    pub const Small: Self = Self(0i32);
    pub const Medium: Self = Self(1i32);
    pub const Large: Self = Self(2i32);
}
impl ::core::marker::Copy for CompactOverlaySize {}
impl ::core::clone::Clone for CompactOverlaySize {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CompactOverlaySize {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CompactOverlaySize {
    type Abi = Self;
}
impl ::core::fmt::Debug for CompactOverlaySize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CompactOverlaySize").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CompactOverlaySize {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Windowing.CompactOverlaySize;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Windowing\"`*"]
#[repr(transparent)]
pub struct DisplayArea(::windows::core::IUnknown);
impl DisplayArea {
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn DisplayId(&self) -> ::windows::core::Result<super::DisplayId> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::DisplayId>::zeroed();
            (::windows::core::Interface::vtable(this).DisplayId)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DisplayId>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn IsPrimary(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).IsPrimary)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn OuterBounds(&self) -> ::windows::core::Result<::windows::Graphics::RectInt32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::Graphics::RectInt32>::zeroed();
            (::windows::core::Interface::vtable(this).OuterBounds)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Graphics::RectInt32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn WorkArea(&self) -> ::windows::core::Result<::windows::Graphics::RectInt32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::Graphics::RectInt32>::zeroed();
            (::windows::core::Interface::vtable(this).WorkArea)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Graphics::RectInt32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn Primary() -> ::windows::core::Result<DisplayArea> {
        Self::IDisplayAreaStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).Primary)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<DisplayArea>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn CreateWatcher() -> ::windows::core::Result<DisplayAreaWatcher> {
        Self::IDisplayAreaStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).CreateWatcher)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<DisplayAreaWatcher>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn FindAll(
    ) -> ::windows::core::Result<::windows::Foundation::Collections::IVectorView<DisplayArea>> {
        Self::IDisplayAreaStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).FindAll)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Collections::IVectorView<DisplayArea>>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn GetFromWindowId<'a, Param0: ::windows::core::IntoParam<'a, super::WindowId>>(
        windowid: Param0,
        displayareafallback: DisplayAreaFallback,
    ) -> ::windows::core::Result<DisplayArea> {
        Self::IDisplayAreaStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).GetFromWindowId)(
                ::windows::core::Interface::as_raw(this),
                windowid.into_param().abi(),
                displayareafallback,
                result__.as_mut_ptr(),
            )
            .from_abi::<DisplayArea>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn GetFromPoint<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Graphics::PointInt32>,
    >(
        point: Param0,
        displayareafallback: DisplayAreaFallback,
    ) -> ::windows::core::Result<DisplayArea> {
        Self::IDisplayAreaStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).GetFromPoint)(
                ::windows::core::Interface::as_raw(this),
                point.into_param().abi(),
                displayareafallback,
                result__.as_mut_ptr(),
            )
            .from_abi::<DisplayArea>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn GetFromRect<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Graphics::RectInt32>,
    >(
        rect: Param0,
        displayareafallback: DisplayAreaFallback,
    ) -> ::windows::core::Result<DisplayArea> {
        Self::IDisplayAreaStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).GetFromRect)(
                ::windows::core::Interface::as_raw(this),
                rect.into_param().abi(),
                displayareafallback,
                result__.as_mut_ptr(),
            )
            .from_abi::<DisplayArea>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDisplayAreaStatics<R, F: FnOnce(&IDisplayAreaStatics) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DisplayArea, IDisplayAreaStatics> =
            ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DisplayArea {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DisplayArea {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DisplayArea {}
impl ::core::fmt::Debug for DisplayArea {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayArea").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayArea {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Windowing.DisplayArea;{5c7e0537-b621-5579-bcae-a84aa8746167})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DisplayArea {
    type Vtable = IDisplayArea_Vtbl;
    const IID: ::windows::core::GUID = <IDisplayArea as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DisplayArea {
    const NAME: &'static str = "Microsoft.UI.Windowing.DisplayArea";
}
impl ::core::convert::From<DisplayArea> for ::windows::core::IUnknown {
    fn from(value: DisplayArea) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayArea> for ::windows::core::IUnknown {
    fn from(value: &DisplayArea) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayArea {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayArea {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DisplayArea> for ::windows::core::IInspectable {
    fn from(value: DisplayArea) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayArea> for ::windows::core::IInspectable {
    fn from(value: &DisplayArea) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayArea {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayArea {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DisplayArea {}
unsafe impl ::core::marker::Sync for DisplayArea {}
#[doc = "*Required features: `\"UI_Windowing\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DisplayAreaFallback(pub i32);
impl DisplayAreaFallback {
    pub const None: Self = Self(0i32);
    pub const Primary: Self = Self(1i32);
    pub const Nearest: Self = Self(2i32);
}
impl ::core::marker::Copy for DisplayAreaFallback {}
impl ::core::clone::Clone for DisplayAreaFallback {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayAreaFallback {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DisplayAreaFallback {
    type Abi = Self;
}
impl ::core::fmt::Debug for DisplayAreaFallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayAreaFallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayAreaFallback {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Windowing.DisplayAreaFallback;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Windowing\"`*"]
#[repr(transparent)]
pub struct DisplayAreaWatcher(::windows::core::IUnknown);
impl DisplayAreaWatcher {
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<DisplayAreaWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<DisplayAreaWatcherStatus>::zeroed();
            (::windows::core::Interface::vtable(this).Status)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<DisplayAreaWatcherStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Start)(::windows::core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Stop)(::windows::core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn Added<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<DisplayAreaWatcher, DisplayArea>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ =
                ::core::mem::MaybeUninit::<::windows::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).Added)(
                ::windows::core::Interface::as_raw(this),
                handler.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn RemoveAdded<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAdded)(
                ::windows::core::Interface::as_raw(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn EnumerationCompleted<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                DisplayAreaWatcher,
                ::windows::core::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ =
                ::core::mem::MaybeUninit::<::windows::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).EnumerationCompleted)(
                ::windows::core::Interface::as_raw(this),
                handler.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn RemoveEnumerationCompleted<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveEnumerationCompleted)(
                ::windows::core::Interface::as_raw(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn Removed<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<DisplayAreaWatcher, DisplayArea>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ =
                ::core::mem::MaybeUninit::<::windows::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).Removed)(
                ::windows::core::Interface::as_raw(this),
                handler.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn RemoveRemoved<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveRemoved)(
                ::windows::core::Interface::as_raw(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn Stopped<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                DisplayAreaWatcher,
                ::windows::core::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ =
                ::core::mem::MaybeUninit::<::windows::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).Stopped)(
                ::windows::core::Interface::as_raw(this),
                handler.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn RemoveStopped<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveStopped)(
                ::windows::core::Interface::as_raw(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn Updated<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<DisplayAreaWatcher, DisplayArea>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ =
                ::core::mem::MaybeUninit::<::windows::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).Updated)(
                ::windows::core::Interface::as_raw(this),
                handler.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn RemoveUpdated<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveUpdated)(
                ::windows::core::Interface::as_raw(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for DisplayAreaWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DisplayAreaWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DisplayAreaWatcher {}
impl ::core::fmt::Debug for DisplayAreaWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayAreaWatcher").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayAreaWatcher {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Windowing.DisplayAreaWatcher;{83f6562f-d3a0-548b-8e4f-a99be3d95c9c})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DisplayAreaWatcher {
    type Vtable = IDisplayAreaWatcher_Vtbl;
    const IID: ::windows::core::GUID = <IDisplayAreaWatcher as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DisplayAreaWatcher {
    const NAME: &'static str = "Microsoft.UI.Windowing.DisplayAreaWatcher";
}
impl ::core::convert::From<DisplayAreaWatcher> for ::windows::core::IUnknown {
    fn from(value: DisplayAreaWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayAreaWatcher> for ::windows::core::IUnknown {
    fn from(value: &DisplayAreaWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayAreaWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayAreaWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DisplayAreaWatcher> for ::windows::core::IInspectable {
    fn from(value: DisplayAreaWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayAreaWatcher> for ::windows::core::IInspectable {
    fn from(value: &DisplayAreaWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayAreaWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayAreaWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DisplayAreaWatcher {}
unsafe impl ::core::marker::Sync for DisplayAreaWatcher {}
#[doc = "*Required features: `\"UI_Windowing\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DisplayAreaWatcherStatus(pub i32);
impl DisplayAreaWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for DisplayAreaWatcherStatus {}
impl ::core::clone::Clone for DisplayAreaWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayAreaWatcherStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DisplayAreaWatcherStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for DisplayAreaWatcherStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayAreaWatcherStatus")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayAreaWatcherStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Windowing.DisplayAreaWatcherStatus;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Windowing\"`*"]
#[repr(transparent)]
pub struct FullScreenPresenter(::windows::core::IUnknown);
impl FullScreenPresenter {
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<AppWindowPresenterKind> {
        let this = &::windows::core::Interface::cast::<IAppWindowPresenter>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppWindowPresenterKind>::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppWindowPresenterKind>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn Create() -> ::windows::core::Result<FullScreenPresenter> {
        Self::IFullScreenPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).Create)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<FullScreenPresenter>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IFullScreenPresenterStatics<
        R,
        F: FnOnce(&IFullScreenPresenterStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            FullScreenPresenter,
            IFullScreenPresenterStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for FullScreenPresenter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FullScreenPresenter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FullScreenPresenter {}
impl ::core::fmt::Debug for FullScreenPresenter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FullScreenPresenter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FullScreenPresenter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Windowing.FullScreenPresenter;{fa9141fd-b8dd-5da1-8b2b-7cdadb76f593})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FullScreenPresenter {
    type Vtable = IFullScreenPresenter_Vtbl;
    const IID: ::windows::core::GUID = <IFullScreenPresenter as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FullScreenPresenter {
    const NAME: &'static str = "Microsoft.UI.Windowing.FullScreenPresenter";
}
impl ::core::convert::From<FullScreenPresenter> for ::windows::core::IUnknown {
    fn from(value: FullScreenPresenter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FullScreenPresenter> for ::windows::core::IUnknown {
    fn from(value: &FullScreenPresenter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FullScreenPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FullScreenPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FullScreenPresenter> for ::windows::core::IInspectable {
    fn from(value: FullScreenPresenter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FullScreenPresenter> for ::windows::core::IInspectable {
    fn from(value: &FullScreenPresenter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FullScreenPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FullScreenPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FullScreenPresenter> for AppWindowPresenter {
    fn from(value: FullScreenPresenter) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&FullScreenPresenter> for AppWindowPresenter {
    fn from(value: &FullScreenPresenter) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, AppWindowPresenter> for FullScreenPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, AppWindowPresenter> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, AppWindowPresenter> for &FullScreenPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, AppWindowPresenter> {
        ::windows::core::Param::Owned(::core::convert::Into::<AppWindowPresenter>::into(self))
    }
}
unsafe impl ::core::marker::Send for FullScreenPresenter {}
unsafe impl ::core::marker::Sync for FullScreenPresenter {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindow(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindow {
    type Vtable = IAppWindow_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xcfa788b3_643b_5c5e_ad4e_321d48a82acd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindow_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::WindowId,
    ) -> ::windows::core::HRESULT,
    pub IsShownInSwitchers: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetIsShownInSwitchers: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub IsVisible: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub OwnerWindowId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::WindowId,
    ) -> ::windows::core::HRESULT,
    pub Position: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Graphics::PointInt32,
    ) -> ::windows::core::HRESULT,
    pub Presenter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub Size: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Graphics::SizeInt32,
    ) -> ::windows::core::HRESULT,
    pub Title: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub TitleBar: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub Destroy:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Hide: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Move: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        position: ::windows::Graphics::PointInt32,
    ) -> ::windows::core::HRESULT,
    pub MoveAndResize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        rect: ::windows::Graphics::RectInt32,
    ) -> ::windows::core::HRESULT,
    pub MoveAndResizeRelativeToDisplayArea: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        rect: ::windows::Graphics::RectInt32,
        displayarea: *mut ::core::ffi::c_void,
    )
        -> ::windows::core::HRESULT,
    pub Resize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        size: ::windows::Graphics::SizeInt32,
    ) -> ::windows::core::HRESULT,
    pub SetIcon: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        iconpath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetIconWithIconId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        iconid: super::IconId,
    ) -> ::windows::core::HRESULT,
    pub SetPresenter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        appwindowpresenter: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetPresenterByKind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        appwindowpresenterkind: AppWindowPresenterKind,
    ) -> ::windows::core::HRESULT,
    pub Show: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ShowWithActivation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        activatewindow: bool,
    ) -> ::windows::core::HRESULT,
    pub Changed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub Closing: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveClosing: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub Destroying: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveDestroying: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindow2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindow2 {
    type Vtable = IAppWindow2_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x6cd41292_794c_5cac_8961_210d012c6ebc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindow2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ClientSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Graphics::SizeInt32,
    ) -> ::windows::core::HRESULT,
    pub MoveInZOrderAtBottom:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MoveInZOrderAtTop:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MoveInZOrderBelow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        windowid: super::WindowId,
    ) -> ::windows::core::HRESULT,
    pub ResizeClient: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        size: ::windows::Graphics::SizeInt32,
    ) -> ::windows::core::HRESULT,
    pub ShowOnceWithRequestedStartupState:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindowChangedEventArgs {
    type Vtable = IAppWindowChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x2182bc5d_fdac_5c3e_bf37_7d8d684e9d1d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub DidPositionChange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub DidPresenterChange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub DidSizeChange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub DidVisibilityChange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowChangedEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindowChangedEventArgs2 {
    type Vtable = IAppWindowChangedEventArgs2_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa773ab4c_a5ec_50e8_98ac_247fe6cd4227);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowChangedEventArgs2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub DidZOrderChange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub IsZOrderAtBottom: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub IsZOrderAtTop: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub ZOrderBelowWindowId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::WindowId,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowClosingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindowClosingEventArgs {
    type Vtable = IAppWindowClosingEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x0e09d90b_2261_590b_9ad1_8504991d8754);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowClosingEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Cancel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowPresenter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindowPresenter {
    type Vtable = IAppWindowPresenter_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xbc3042c2_c6c6_5632_8989_ff0ec6d3b40d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowPresenter_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Kind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut AppWindowPresenterKind,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowPresenterFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindowPresenterFactory {
    type Vtable = IAppWindowPresenterFactory_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x62082e3c_1368_5238_90d1_e932dc718a82);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowPresenterFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindowStatics {
    type Vtable = IAppWindowStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x3c315c24_d540_5d72_b518_b226b83627cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CreateWithPresenter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        appwindowpresenter: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CreateWithPresenterAndOwner: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        appwindowpresenter: *mut ::core::ffi::c_void,
        ownerwindowid: super::WindowId,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetFromWindowId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        windowid: super::WindowId,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowTitleBar(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindowTitleBar {
    type Vtable = IAppWindowTitleBar_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x5574efa2_c91c_5700_a363_539c71a7aaf4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowTitleBar_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub BackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetBackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub ButtonBackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetButtonBackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub ButtonForegroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetButtonForegroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub ButtonHoverBackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetButtonHoverBackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub ButtonHoverForegroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetButtonHoverForegroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub ButtonInactiveBackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetButtonInactiveBackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    )
        -> ::windows::core::HRESULT,
    pub ButtonInactiveForegroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetButtonInactiveForegroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    )
        -> ::windows::core::HRESULT,
    pub ButtonPressedBackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetButtonPressedBackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub ButtonPressedForegroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetButtonPressedForegroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub ExtendsContentIntoTitleBar: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetExtendsContentIntoTitleBar: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub ForegroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetForegroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub Height: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub IconShowOptions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut IconShowOptions,
    ) -> ::windows::core::HRESULT,
    pub SetIconShowOptions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: IconShowOptions,
    ) -> ::windows::core::HRESULT,
    pub InactiveBackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetInactiveBackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub InactiveForegroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetInactiveForegroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub LeftInset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub RightInset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub ResetToDefault:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDragRectangles: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value_array_size: u32,
        value: *const ::windows::Graphics::RectInt32,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowTitleBar2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindowTitleBar2 {
    type Vtable = IAppWindowTitleBar2_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x86faed38_748a_5b4b_9ccf_3ba0496c9041);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowTitleBar2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub PreferredHeightOption: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut TitleBarHeightOption,
    ) -> ::windows::core::HRESULT,
    pub SetPreferredHeightOption: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: TitleBarHeightOption,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowTitleBarStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindowTitleBarStatics {
    type Vtable = IAppWindowTitleBarStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x9e1da52e_8b15_54d6_a886_f7b9f9d930b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowTitleBarStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub IsCustomizationSupported: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompactOverlayPresenter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICompactOverlayPresenter {
    type Vtable = ICompactOverlayPresenter_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xefeb0812_6fc7_5b7d_bd92_cc8f9a6454c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompactOverlayPresenter_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub InitialSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CompactOverlaySize,
    ) -> ::windows::core::HRESULT,
    pub SetInitialSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CompactOverlaySize,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompactOverlayPresenterStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICompactOverlayPresenterStatics {
    type Vtable = ICompactOverlayPresenterStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xeab93186_4f6a_52f9_8c03_da57a1522f6e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompactOverlayPresenterStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayArea(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayArea {
    type Vtable = IDisplayArea_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x5c7e0537_b621_5579_bcae_a84aa8746167);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayArea_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub DisplayId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::DisplayId,
    ) -> ::windows::core::HRESULT,
    pub IsPrimary: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub OuterBounds: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Graphics::RectInt32,
    ) -> ::windows::core::HRESULT,
    pub WorkArea: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Graphics::RectInt32,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayAreaStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayAreaStatics {
    type Vtable = IDisplayAreaStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x02ab4926_211e_5d49_8e4b_2af193daed09);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayAreaStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Primary: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CreateWatcher: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub FindAll: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetFromWindowId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        windowid: super::WindowId,
        displayareafallback: DisplayAreaFallback,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetFromPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        point: ::windows::Graphics::PointInt32,
        displayareafallback: DisplayAreaFallback,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetFromRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        rect: ::windows::Graphics::RectInt32,
        displayareafallback: DisplayAreaFallback,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayAreaWatcher(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayAreaWatcher {
    type Vtable = IDisplayAreaWatcher_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x83f6562f_d3a0_548b_8e4f_a99be3d95c9c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayAreaWatcher_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut DisplayAreaWatcherStatus,
    ) -> ::windows::core::HRESULT,
    pub Start:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Added: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveAdded: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub EnumerationCompleted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub Removed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveRemoved: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub Stopped: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveStopped: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub Updated: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveUpdated: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFullScreenPresenter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFullScreenPresenter {
    type Vtable = IFullScreenPresenter_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xfa9141fd_b8dd_5da1_8b2b_7cdadb76f593);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFullScreenPresenter_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFullScreenPresenterStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFullScreenPresenterStatics {
    type Vtable = IFullScreenPresenterStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x2ec0d2c1_e086_55bb_a3b2_44942e231c67);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFullScreenPresenterStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOverlappedPresenter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOverlappedPresenter {
    type Vtable = IOverlappedPresenter_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x21693970_4f4c_5172_9e9d_682a2d174884);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOverlappedPresenter_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub HasBorder: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub HasTitleBar: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub IsAlwaysOnTop: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetIsAlwaysOnTop: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub IsMaximizable: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetIsMaximizable: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub IsMinimizable: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetIsMinimizable: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub IsModal: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetIsModal: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub IsResizable: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetIsResizable: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut OverlappedPresenterState,
    ) -> ::windows::core::HRESULT,
    pub Maximize:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Minimize:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Restore:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetBorderAndTitleBar: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        hasborder: bool,
        hastitlebar: bool,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOverlappedPresenter2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOverlappedPresenter2 {
    type Vtable = IOverlappedPresenter2_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x5c6ccd93_4244_5cd2_b355_ed5ea34df730);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOverlappedPresenter2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub MinimizeWithActivation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        activatewindow: bool,
    ) -> ::windows::core::HRESULT,
    pub RestoreWithActivation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        activatewindow: bool,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOverlappedPresenterStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOverlappedPresenterStatics {
    type Vtable = IOverlappedPresenterStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x997225e4_7b00_5aee_a4be_d4068d1999e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOverlappedPresenterStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CreateForContextMenu: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CreateForDialog: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CreateForToolWindow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOverlappedPresenterStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOverlappedPresenterStatics2 {
    type Vtable = IOverlappedPresenterStatics2_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xed5c4f92_32f4_5d15_80d0_b2a5efa04d39);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOverlappedPresenterStatics2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub RequestedStartupState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut OverlappedPresenterState,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Windowing\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IconShowOptions(pub i32);
impl IconShowOptions {
    pub const ShowIconAndSystemMenu: Self = Self(0i32);
    pub const HideIconAndSystemMenu: Self = Self(1i32);
}
impl ::core::marker::Copy for IconShowOptions {}
impl ::core::clone::Clone for IconShowOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IconShowOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IconShowOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for IconShowOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IconShowOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IconShowOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Windowing.IconShowOptions;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Windowing\"`*"]
#[repr(transparent)]
pub struct OverlappedPresenter(::windows::core::IUnknown);
impl OverlappedPresenter {
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<AppWindowPresenterKind> {
        let this = &::windows::core::Interface::cast::<IAppWindowPresenter>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppWindowPresenterKind>::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppWindowPresenterKind>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn HasBorder(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).HasBorder)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn HasTitleBar(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).HasTitleBar)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn IsAlwaysOnTop(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).IsAlwaysOnTop)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn SetIsAlwaysOnTop(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsAlwaysOnTop)(
                ::windows::core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn IsMaximizable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).IsMaximizable)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn SetIsMaximizable(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsMaximizable)(
                ::windows::core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn IsMinimizable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).IsMinimizable)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn SetIsMinimizable(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsMinimizable)(
                ::windows::core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn IsModal(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).IsModal)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn SetIsModal(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsModal)(
                ::windows::core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn IsResizable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).IsResizable)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn SetIsResizable(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsResizable)(
                ::windows::core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn State(&self) -> ::windows::core::Result<OverlappedPresenterState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<OverlappedPresenterState>::zeroed();
            (::windows::core::Interface::vtable(this).State)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<OverlappedPresenterState>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn Maximize(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Maximize)(::windows::core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn Minimize(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Minimize)(::windows::core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn Restore(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Restore)(::windows::core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn SetBorderAndTitleBar(
        &self,
        hasborder: bool,
        hastitlebar: bool,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetBorderAndTitleBar)(
                ::windows::core::Interface::as_raw(this),
                hasborder,
                hastitlebar,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn MinimizeWithActivation(&self, activatewindow: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IOverlappedPresenter2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).MinimizeWithActivation)(
                ::windows::core::Interface::as_raw(this),
                activatewindow,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn RestoreWithActivation(&self, activatewindow: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IOverlappedPresenter2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RestoreWithActivation)(
                ::windows::core::Interface::as_raw(this),
                activatewindow,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn Create() -> ::windows::core::Result<OverlappedPresenter> {
        Self::IOverlappedPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).Create)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<OverlappedPresenter>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn CreateForContextMenu() -> ::windows::core::Result<OverlappedPresenter> {
        Self::IOverlappedPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).CreateForContextMenu)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<OverlappedPresenter>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn CreateForDialog() -> ::windows::core::Result<OverlappedPresenter> {
        Self::IOverlappedPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).CreateForDialog)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<OverlappedPresenter>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn CreateForToolWindow() -> ::windows::core::Result<OverlappedPresenter> {
        Self::IOverlappedPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).CreateForToolWindow)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<OverlappedPresenter>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Windowing\"`*"]
    pub fn RequestedStartupState() -> ::windows::core::Result<OverlappedPresenterState> {
        Self::IOverlappedPresenterStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<OverlappedPresenterState>::zeroed();
            (::windows::core::Interface::vtable(this).RequestedStartupState)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<OverlappedPresenterState>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IOverlappedPresenterStatics<
        R,
        F: FnOnce(&IOverlappedPresenterStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            OverlappedPresenter,
            IOverlappedPresenterStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IOverlappedPresenterStatics2<
        R,
        F: FnOnce(&IOverlappedPresenterStatics2) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            OverlappedPresenter,
            IOverlappedPresenterStatics2,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for OverlappedPresenter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for OverlappedPresenter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OverlappedPresenter {}
impl ::core::fmt::Debug for OverlappedPresenter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OverlappedPresenter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for OverlappedPresenter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Windowing.OverlappedPresenter;{21693970-4f4c-5172-9e9d-682a2d174884})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for OverlappedPresenter {
    type Vtable = IOverlappedPresenter_Vtbl;
    const IID: ::windows::core::GUID = <IOverlappedPresenter as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for OverlappedPresenter {
    const NAME: &'static str = "Microsoft.UI.Windowing.OverlappedPresenter";
}
impl ::core::convert::From<OverlappedPresenter> for ::windows::core::IUnknown {
    fn from(value: OverlappedPresenter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OverlappedPresenter> for ::windows::core::IUnknown {
    fn from(value: &OverlappedPresenter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for OverlappedPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a OverlappedPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<OverlappedPresenter> for ::windows::core::IInspectable {
    fn from(value: OverlappedPresenter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OverlappedPresenter> for ::windows::core::IInspectable {
    fn from(value: &OverlappedPresenter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for OverlappedPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a OverlappedPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<OverlappedPresenter> for AppWindowPresenter {
    fn from(value: OverlappedPresenter) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&OverlappedPresenter> for AppWindowPresenter {
    fn from(value: &OverlappedPresenter) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, AppWindowPresenter> for OverlappedPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, AppWindowPresenter> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, AppWindowPresenter> for &OverlappedPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, AppWindowPresenter> {
        ::windows::core::Param::Owned(::core::convert::Into::<AppWindowPresenter>::into(self))
    }
}
unsafe impl ::core::marker::Send for OverlappedPresenter {}
unsafe impl ::core::marker::Sync for OverlappedPresenter {}
#[doc = "*Required features: `\"UI_Windowing\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct OverlappedPresenterState(pub i32);
impl OverlappedPresenterState {
    pub const Maximized: Self = Self(0i32);
    pub const Minimized: Self = Self(1i32);
    pub const Restored: Self = Self(2i32);
}
impl ::core::marker::Copy for OverlappedPresenterState {}
impl ::core::clone::Clone for OverlappedPresenterState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OverlappedPresenterState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for OverlappedPresenterState {
    type Abi = Self;
}
impl ::core::fmt::Debug for OverlappedPresenterState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OverlappedPresenterState")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for OverlappedPresenterState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Windowing.OverlappedPresenterState;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Windowing\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TitleBarHeightOption(pub i32);
impl TitleBarHeightOption {
    pub const Standard: Self = Self(0i32);
    pub const Tall: Self = Self(1i32);
}
impl ::core::marker::Copy for TitleBarHeightOption {}
impl ::core::clone::Clone for TitleBarHeightOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TitleBarHeightOption {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TitleBarHeightOption {
    type Abi = Self;
}
impl ::core::fmt::Debug for TitleBarHeightOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TitleBarHeightOption")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TitleBarHeightOption {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Windowing.TitleBarHeightOption;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
