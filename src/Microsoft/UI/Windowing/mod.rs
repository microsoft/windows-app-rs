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
pub struct AppWindow(pub ::windows::core::IInspectable);
impl AppWindow {
    pub fn Id(&self) -> ::windows::core::Result<super::WindowId> {
        let this = self;
        unsafe {
            let mut result__: super::WindowId = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::WindowId>(result__)
        }
    }
    pub fn IsShownInSwitchers(&self) -> ::windows::core::Result<bool> {
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
    pub fn SetIsShownInSwitchers(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsVisible(&self) -> ::windows::core::Result<bool> {
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
    pub fn OwnerWindowId(&self) -> ::windows::core::Result<super::WindowId> {
        let this = self;
        unsafe {
            let mut result__: super::WindowId = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::WindowId>(result__)
        }
    }
    pub fn Position(&self) -> ::windows::core::Result<::windows::Graphics::PointInt32> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Graphics::PointInt32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Graphics::PointInt32>(result__)
        }
    }
    pub fn Presenter(&self) -> ::windows::core::Result<AppWindowPresenter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AppWindowPresenter>(result__)
        }
    }
    pub fn Size(&self) -> ::windows::core::Result<::windows::Graphics::SizeInt32> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Graphics::SizeInt32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Graphics::SizeInt32>(result__)
        }
    }
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetTitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn TitleBar(&self) -> ::windows::core::Result<AppWindowTitleBar> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AppWindowTitleBar>(result__)
        }
    }
    pub fn Destroy(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Hide(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Move<'a, Param0: ::windows::core::IntoParam<'a, ::windows::Graphics::PointInt32>>(
        &self,
        position: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                position.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn MoveAndResize<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Graphics::RectInt32>,
    >(
        &self,
        rect: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                rect.into_param().abi(),
            )
            .ok()
        }
    }
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
            (::windows::core::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                rect.into_param().abi(),
                displayarea.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Resize<'a, Param0: ::windows::core::IntoParam<'a, ::windows::Graphics::SizeInt32>>(
        &self,
        size: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                size.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn SetIcon<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        iconpath: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                iconpath.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn SetIconWithIconId<'a, Param0: ::windows::core::IntoParam<'a, super::IconId>>(
        &self,
        iconid: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).24)(
                ::core::mem::transmute_copy(this),
                iconid.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn SetPresenter<'a, Param0: ::windows::core::IntoParam<'a, AppWindowPresenter>>(
        &self,
        appwindowpresenter: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                appwindowpresenter.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn SetPresenterByKind(
        &self,
        appwindowpresenterkind: AppWindowPresenterKind,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).26)(
                ::core::mem::transmute_copy(this),
                appwindowpresenterkind,
            )
            .ok()
        }
    }
    pub fn Show(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn ShowWithActivation(&self, activatewindow: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).28)(
                ::core::mem::transmute_copy(this),
                activatewindow,
            )
            .ok()
        }
    }
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
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).30)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
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
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveClosing<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).32)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
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
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDestroying<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).34)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Create() -> ::windows::core::Result<AppWindow> {
        Self::IAppWindowStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AppWindow>(result__)
        })
    }
    pub fn CreateWithPresenter<'a, Param0: ::windows::core::IntoParam<'a, AppWindowPresenter>>(
        appwindowpresenter: Param0,
    ) -> ::windows::core::Result<AppWindow> {
        Self::IAppWindowStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                appwindowpresenter.into_param().abi(),
                &mut result__,
            )
            .from_abi::<AppWindow>(result__)
        })
    }
    pub fn CreateWithPresenterAndOwner<
        'a,
        Param0: ::windows::core::IntoParam<'a, AppWindowPresenter>,
        Param1: ::windows::core::IntoParam<'a, super::WindowId>,
    >(
        appwindowpresenter: Param0,
        ownerwindowid: Param1,
    ) -> ::windows::core::Result<AppWindow> {
        Self::IAppWindowStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                appwindowpresenter.into_param().abi(),
                ownerwindowid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<AppWindow>(result__)
        })
    }
    pub fn GetFromWindowId<'a, Param0: ::windows::core::IntoParam<'a, super::WindowId>>(
        windowid: Param0,
    ) -> ::windows::core::Result<AppWindow> {
        Self::IAppWindowStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                windowid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<AppWindow>(result__)
        })
    }
    pub fn IAppWindowStatics<R, F: FnOnce(&IAppWindowStatics) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppWindow, IAppWindowStatics> =
            ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AppWindow {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Windowing.AppWindow;{cfa788b3-643b-5c5e-ad4e-321d48a82acd})",
    );
}
unsafe impl ::windows::core::Interface for AppWindow {
    type Vtable = IAppWindow_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xcfa788b3_643b_5c5e_ad4e_321d48a82acd);
}
impl ::windows::core::RuntimeName for AppWindow {
    const NAME: &'static str = "Microsoft.UI.Windowing.AppWindow";
}
impl ::core::convert::From<AppWindow> for ::windows::core::IUnknown {
    fn from(value: AppWindow) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppWindow> for ::windows::core::IUnknown {
    fn from(value: &AppWindow) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppWindow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppWindow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppWindow> for ::windows::core::IInspectable {
    fn from(value: AppWindow) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppWindow> for ::windows::core::IInspectable {
    fn from(value: &AppWindow) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppWindow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppWindow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppWindow {}
unsafe impl ::core::marker::Sync for AppWindow {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct AppWindowChangedEventArgs(pub ::windows::core::IInspectable);
impl AppWindowChangedEventArgs {
    pub fn DidPositionChange(&self) -> ::windows::core::Result<bool> {
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
    pub fn DidPresenterChange(&self) -> ::windows::core::Result<bool> {
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
    pub fn DidSizeChange(&self) -> ::windows::core::Result<bool> {
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
    pub fn DidVisibilityChange(&self) -> ::windows::core::Result<bool> {
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
}
unsafe impl ::windows::core::RuntimeType for AppWindowChangedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Windowing.AppWindowChangedEventArgs;{2182bc5d-fdac-5c3e-bf37-7d8d684e9d1d})" ) ;
}
unsafe impl ::windows::core::Interface for AppWindowChangedEventArgs {
    type Vtable = IAppWindowChangedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x2182bc5d_fdac_5c3e_bf37_7d8d684e9d1d);
}
impl ::windows::core::RuntimeName for AppWindowChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Windowing.AppWindowChangedEventArgs";
}
impl ::core::convert::From<AppWindowChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppWindowChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppWindowChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppWindowChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppWindowChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a AppWindowChangedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppWindowChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppWindowChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppWindowChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppWindowChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for AppWindowChangedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a AppWindowChangedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppWindowChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppWindowChangedEventArgs {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct AppWindowClosingEventArgs(pub ::windows::core::IInspectable);
impl AppWindowClosingEventArgs {
    pub fn Cancel(&self) -> ::windows::core::Result<bool> {
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
    pub fn SetCancel(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppWindowClosingEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Windowing.AppWindowClosingEventArgs;{0e09d90b-2261-590b-9ad1-8504991d8754})" ) ;
}
unsafe impl ::windows::core::Interface for AppWindowClosingEventArgs {
    type Vtable = IAppWindowClosingEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x0e09d90b_2261_590b_9ad1_8504991d8754);
}
impl ::windows::core::RuntimeName for AppWindowClosingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Windowing.AppWindowClosingEventArgs";
}
impl ::core::convert::From<AppWindowClosingEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppWindowClosingEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppWindowClosingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppWindowClosingEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppWindowClosingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a AppWindowClosingEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppWindowClosingEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppWindowClosingEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppWindowClosingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppWindowClosingEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for AppWindowClosingEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a AppWindowClosingEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppWindowClosingEventArgs {}
unsafe impl ::core::marker::Sync for AppWindowClosingEventArgs {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct AppWindowPresenter(pub ::windows::core::IInspectable);
impl AppWindowPresenter {
    pub fn Kind(&self) -> ::windows::core::Result<AppWindowPresenterKind> {
        let this = self;
        unsafe {
            let mut result__: AppWindowPresenterKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AppWindowPresenterKind>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppWindowPresenter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Windowing.AppWindowPresenter;{bc3042c2-c6c6-5632-8989-ff0ec6d3b40d})",
    );
}
unsafe impl ::windows::core::Interface for AppWindowPresenter {
    type Vtable = IAppWindowPresenter_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xbc3042c2_c6c6_5632_8989_ff0ec6d3b40d);
}
impl ::windows::core::RuntimeName for AppWindowPresenter {
    const NAME: &'static str = "Microsoft.UI.Windowing.AppWindowPresenter";
}
impl ::core::convert::From<AppWindowPresenter> for ::windows::core::IUnknown {
    fn from(value: AppWindowPresenter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppWindowPresenter> for ::windows::core::IUnknown {
    fn from(value: &AppWindowPresenter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppWindowPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppWindowPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppWindowPresenter> for ::windows::core::IInspectable {
    fn from(value: AppWindowPresenter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppWindowPresenter> for ::windows::core::IInspectable {
    fn from(value: &AppWindowPresenter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppWindowPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppWindowPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppWindowPresenter {}
unsafe impl ::core::marker::Sync for AppWindowPresenter {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct AppWindowPresenterKind(pub i32);
impl AppWindowPresenterKind {
    pub const Default: AppWindowPresenterKind = AppWindowPresenterKind(0i32);
    pub const CompactOverlay: AppWindowPresenterKind = AppWindowPresenterKind(1i32);
    pub const FullScreen: AppWindowPresenterKind = AppWindowPresenterKind(2i32);
    pub const Overlapped: AppWindowPresenterKind = AppWindowPresenterKind(3i32);
}
impl ::core::convert::From<i32> for AppWindowPresenterKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppWindowPresenterKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AppWindowPresenterKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Windowing.AppWindowPresenterKind;i4)",
    );
}
impl ::windows::core::DefaultType for AppWindowPresenterKind {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct AppWindowTitleBar(pub ::windows::core::IInspectable);
impl AppWindowTitleBar {
    pub fn BackgroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IReference<::windows::UI::Color>>(result__)
        }
    }
    pub fn SetBackgroundColor<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::IReference<::windows::UI::Color>>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ButtonBackgroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IReference<::windows::UI::Color>>(result__)
        }
    }
    pub fn SetButtonBackgroundColor<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::IReference<::windows::UI::Color>>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ButtonForegroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IReference<::windows::UI::Color>>(result__)
        }
    }
    pub fn SetButtonForegroundColor<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::IReference<::windows::UI::Color>>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ButtonHoverBackgroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IReference<::windows::UI::Color>>(result__)
        }
    }
    pub fn SetButtonHoverBackgroundColor<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::IReference<::windows::UI::Color>>,
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
    pub fn ButtonHoverForegroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IReference<::windows::UI::Color>>(result__)
        }
    }
    pub fn SetButtonHoverForegroundColor<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::IReference<::windows::UI::Color>>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ButtonInactiveBackgroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IReference<::windows::UI::Color>>(result__)
        }
    }
    pub fn SetButtonInactiveBackgroundColor<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::IReference<::windows::UI::Color>>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ButtonInactiveForegroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IReference<::windows::UI::Color>>(result__)
        }
    }
    pub fn SetButtonInactiveForegroundColor<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::IReference<::windows::UI::Color>>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ButtonPressedBackgroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IReference<::windows::UI::Color>>(result__)
        }
    }
    pub fn SetButtonPressedBackgroundColor<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::IReference<::windows::UI::Color>>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ButtonPressedForegroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IReference<::windows::UI::Color>>(result__)
        }
    }
    pub fn SetButtonPressedForegroundColor<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::IReference<::windows::UI::Color>>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ExtendsContentIntoTitleBar(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetExtendsContentIntoTitleBar(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn ForegroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IReference<::windows::UI::Color>>(result__)
        }
    }
    pub fn SetForegroundColor<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::IReference<::windows::UI::Color>>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).27)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Height(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn IconShowOptions(&self) -> ::windows::core::Result<IconShowOptions> {
        let this = self;
        unsafe {
            let mut result__: IconShowOptions = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IconShowOptions>(result__)
        }
    }
    pub fn SetIconShowOptions(&self, value: IconShowOptions) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn InactiveBackgroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IReference<::windows::UI::Color>>(result__)
        }
    }
    pub fn SetInactiveBackgroundColor<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::IReference<::windows::UI::Color>>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).32)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn InactiveForegroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IReference<::windows::UI::Color>>(result__)
        }
    }
    pub fn SetInactiveForegroundColor<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::IReference<::windows::UI::Color>>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).34)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn LeftInset(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).35)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn RightInset(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).36)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn ResetToDefault(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).37)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn SetDragRectangles(
        &self,
        value: &[<::windows::Graphics::RectInt32 as ::windows::core::DefaultType>::DefaultType],
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).38)(
                ::core::mem::transmute_copy(this),
                value.len() as u32,
                ::core::mem::transmute(value.as_ptr()),
            )
            .ok()
        }
    }
    pub fn IsCustomizationSupported() -> ::windows::core::Result<bool> {
        Self::IAppWindowTitleBarStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
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
unsafe impl ::windows::core::RuntimeType for AppWindowTitleBar {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Windowing.AppWindowTitleBar;{5574efa2-c91c-5700-a363-539c71a7aaf4})",
    );
}
unsafe impl ::windows::core::Interface for AppWindowTitleBar {
    type Vtable = IAppWindowTitleBar_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x5574efa2_c91c_5700_a363_539c71a7aaf4);
}
impl ::windows::core::RuntimeName for AppWindowTitleBar {
    const NAME: &'static str = "Microsoft.UI.Windowing.AppWindowTitleBar";
}
impl ::core::convert::From<AppWindowTitleBar> for ::windows::core::IUnknown {
    fn from(value: AppWindowTitleBar) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppWindowTitleBar> for ::windows::core::IUnknown {
    fn from(value: &AppWindowTitleBar) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppWindowTitleBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppWindowTitleBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppWindowTitleBar> for ::windows::core::IInspectable {
    fn from(value: AppWindowTitleBar) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppWindowTitleBar> for ::windows::core::IInspectable {
    fn from(value: &AppWindowTitleBar) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppWindowTitleBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppWindowTitleBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppWindowTitleBar {}
unsafe impl ::core::marker::Sync for AppWindowTitleBar {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CompactOverlayPresenter(pub ::windows::core::IInspectable);
impl CompactOverlayPresenter {
    pub fn InitialSize(&self) -> ::windows::core::Result<CompactOverlaySize> {
        let this = self;
        unsafe {
            let mut result__: CompactOverlaySize = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CompactOverlaySize>(result__)
        }
    }
    pub fn SetInitialSize(&self, value: CompactOverlaySize) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn Create() -> ::windows::core::Result<CompactOverlayPresenter> {
        Self::ICompactOverlayPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CompactOverlayPresenter>(result__)
        })
    }
    pub fn Kind(&self) -> ::windows::core::Result<AppWindowPresenterKind> {
        let this = &::windows::core::Interface::cast::<IAppWindowPresenter>(self)?;
        unsafe {
            let mut result__: AppWindowPresenterKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AppWindowPresenterKind>(result__)
        }
    }
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
unsafe impl ::windows::core::RuntimeType for CompactOverlayPresenter {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Windowing.CompactOverlayPresenter;{efeb0812-6fc7-5b7d-bd92-cc8f9a6454c9})" ) ;
}
unsafe impl ::windows::core::Interface for CompactOverlayPresenter {
    type Vtable = ICompactOverlayPresenter_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xefeb0812_6fc7_5b7d_bd92_cc8f9a6454c9);
}
impl ::windows::core::RuntimeName for CompactOverlayPresenter {
    const NAME: &'static str = "Microsoft.UI.Windowing.CompactOverlayPresenter";
}
impl ::core::convert::From<CompactOverlayPresenter> for ::windows::core::IUnknown {
    fn from(value: CompactOverlayPresenter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CompactOverlayPresenter> for ::windows::core::IUnknown {
    fn from(value: &CompactOverlayPresenter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CompactOverlayPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CompactOverlayPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CompactOverlayPresenter> for ::windows::core::IInspectable {
    fn from(value: CompactOverlayPresenter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CompactOverlayPresenter> for ::windows::core::IInspectable {
    fn from(value: &CompactOverlayPresenter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CompactOverlayPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CompactOverlayPresenter
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<CompactOverlayPresenter> for AppWindowPresenter {
    fn from(value: CompactOverlayPresenter) -> Self {
        ::core::convert::Into::<AppWindowPresenter>::into(&value)
    }
}
impl ::core::convert::From<&CompactOverlayPresenter> for AppWindowPresenter {
    fn from(value: &CompactOverlayPresenter) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, AppWindowPresenter> for CompactOverlayPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, AppWindowPresenter> {
        ::windows::core::Param::Owned(::core::convert::Into::<AppWindowPresenter>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, AppWindowPresenter> for &CompactOverlayPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, AppWindowPresenter> {
        ::windows::core::Param::Owned(::core::convert::Into::<AppWindowPresenter>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for CompactOverlayPresenter {}
unsafe impl ::core::marker::Sync for CompactOverlayPresenter {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CompactOverlaySize(pub i32);
impl CompactOverlaySize {
    pub const Small: CompactOverlaySize = CompactOverlaySize(0i32);
    pub const Medium: CompactOverlaySize = CompactOverlaySize(1i32);
    pub const Large: CompactOverlaySize = CompactOverlaySize(2i32);
}
impl ::core::convert::From<i32> for CompactOverlaySize {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CompactOverlaySize {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CompactOverlaySize {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Windowing.CompactOverlaySize;i4)",
    );
}
impl ::windows::core::DefaultType for CompactOverlaySize {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct DisplayArea(pub ::windows::core::IInspectable);
impl DisplayArea {
    pub fn DisplayId(&self) -> ::windows::core::Result<super::DisplayId> {
        let this = self;
        unsafe {
            let mut result__: super::DisplayId = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DisplayId>(result__)
        }
    }
    pub fn IsPrimary(&self) -> ::windows::core::Result<bool> {
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
    pub fn OuterBounds(&self) -> ::windows::core::Result<::windows::Graphics::RectInt32> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Graphics::RectInt32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Graphics::RectInt32>(result__)
        }
    }
    pub fn WorkArea(&self) -> ::windows::core::Result<::windows::Graphics::RectInt32> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Graphics::RectInt32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Graphics::RectInt32>(result__)
        }
    }
    pub fn Primary() -> ::windows::core::Result<DisplayArea> {
        Self::IDisplayAreaStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<DisplayArea>(result__)
        })
    }
    pub fn CreateWatcher() -> ::windows::core::Result<DisplayAreaWatcher> {
        Self::IDisplayAreaStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<DisplayAreaWatcher>(result__)
        })
    }
    pub fn FindAll(
    ) -> ::windows::core::Result<::windows::Foundation::Collections::IVectorView<DisplayArea>> {
        Self::IDisplayAreaStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVectorView<DisplayArea>>(result__)
        })
    }
    pub fn GetFromWindowId<'a, Param0: ::windows::core::IntoParam<'a, super::WindowId>>(
        windowid: Param0,
        displayareafallback: DisplayAreaFallback,
    ) -> ::windows::core::Result<DisplayArea> {
        Self::IDisplayAreaStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                windowid.into_param().abi(),
                displayareafallback,
                &mut result__,
            )
            .from_abi::<DisplayArea>(result__)
        })
    }
    pub fn GetFromPoint<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Graphics::PointInt32>,
    >(
        point: Param0,
        displayareafallback: DisplayAreaFallback,
    ) -> ::windows::core::Result<DisplayArea> {
        Self::IDisplayAreaStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                point.into_param().abi(),
                displayareafallback,
                &mut result__,
            )
            .from_abi::<DisplayArea>(result__)
        })
    }
    pub fn GetFromRect<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Graphics::RectInt32>,
    >(
        rect: Param0,
        displayareafallback: DisplayAreaFallback,
    ) -> ::windows::core::Result<DisplayArea> {
        Self::IDisplayAreaStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                rect.into_param().abi(),
                displayareafallback,
                &mut result__,
            )
            .from_abi::<DisplayArea>(result__)
        })
    }
    pub fn IDisplayAreaStatics<R, F: FnOnce(&IDisplayAreaStatics) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DisplayArea, IDisplayAreaStatics> =
            ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayArea {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Windowing.DisplayArea;{5c7e0537-b621-5579-bcae-a84aa8746167})",
    );
}
unsafe impl ::windows::core::Interface for DisplayArea {
    type Vtable = IDisplayArea_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x5c7e0537_b621_5579_bcae_a84aa8746167);
}
impl ::windows::core::RuntimeName for DisplayArea {
    const NAME: &'static str = "Microsoft.UI.Windowing.DisplayArea";
}
impl ::core::convert::From<DisplayArea> for ::windows::core::IUnknown {
    fn from(value: DisplayArea) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DisplayArea> for ::windows::core::IUnknown {
    fn from(value: &DisplayArea) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayArea {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayArea {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DisplayArea> for ::windows::core::IInspectable {
    fn from(value: DisplayArea) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DisplayArea> for ::windows::core::IInspectable {
    fn from(value: &DisplayArea) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayArea {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayArea {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DisplayArea {}
unsafe impl ::core::marker::Sync for DisplayArea {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DisplayAreaFallback(pub i32);
impl DisplayAreaFallback {
    pub const None: DisplayAreaFallback = DisplayAreaFallback(0i32);
    pub const Primary: DisplayAreaFallback = DisplayAreaFallback(1i32);
    pub const Nearest: DisplayAreaFallback = DisplayAreaFallback(2i32);
}
impl ::core::convert::From<i32> for DisplayAreaFallback {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DisplayAreaFallback {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DisplayAreaFallback {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Windowing.DisplayAreaFallback;i4)",
    );
}
impl ::windows::core::DefaultType for DisplayAreaFallback {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct DisplayAreaWatcher(pub ::windows::core::IInspectable);
impl DisplayAreaWatcher {
    pub fn Status(&self) -> ::windows::core::Result<DisplayAreaWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__: DisplayAreaWatcherStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<DisplayAreaWatcherStatus>(result__)
        }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok()
        }
    }
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
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAdded<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
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
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveEnumerationCompleted<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
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
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveRemoved<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
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
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveStopped<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
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
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveUpdated<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayAreaWatcher {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Windowing.DisplayAreaWatcher;{83f6562f-d3a0-548b-8e4f-a99be3d95c9c})",
    );
}
unsafe impl ::windows::core::Interface for DisplayAreaWatcher {
    type Vtable = IDisplayAreaWatcher_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x83f6562f_d3a0_548b_8e4f_a99be3d95c9c);
}
impl ::windows::core::RuntimeName for DisplayAreaWatcher {
    const NAME: &'static str = "Microsoft.UI.Windowing.DisplayAreaWatcher";
}
impl ::core::convert::From<DisplayAreaWatcher> for ::windows::core::IUnknown {
    fn from(value: DisplayAreaWatcher) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DisplayAreaWatcher> for ::windows::core::IUnknown {
    fn from(value: &DisplayAreaWatcher) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayAreaWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayAreaWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DisplayAreaWatcher> for ::windows::core::IInspectable {
    fn from(value: DisplayAreaWatcher) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DisplayAreaWatcher> for ::windows::core::IInspectable {
    fn from(value: &DisplayAreaWatcher) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayAreaWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayAreaWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DisplayAreaWatcher {}
unsafe impl ::core::marker::Sync for DisplayAreaWatcher {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DisplayAreaWatcherStatus(pub i32);
impl DisplayAreaWatcherStatus {
    pub const Created: DisplayAreaWatcherStatus = DisplayAreaWatcherStatus(0i32);
    pub const Started: DisplayAreaWatcherStatus = DisplayAreaWatcherStatus(1i32);
    pub const EnumerationCompleted: DisplayAreaWatcherStatus = DisplayAreaWatcherStatus(2i32);
    pub const Stopping: DisplayAreaWatcherStatus = DisplayAreaWatcherStatus(3i32);
    pub const Stopped: DisplayAreaWatcherStatus = DisplayAreaWatcherStatus(4i32);
    pub const Aborted: DisplayAreaWatcherStatus = DisplayAreaWatcherStatus(5i32);
}
impl ::core::convert::From<i32> for DisplayAreaWatcherStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DisplayAreaWatcherStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DisplayAreaWatcherStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Windowing.DisplayAreaWatcherStatus;i4)",
    );
}
impl ::windows::core::DefaultType for DisplayAreaWatcherStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct FullScreenPresenter(pub ::windows::core::IInspectable);
impl FullScreenPresenter {
    pub fn Create() -> ::windows::core::Result<FullScreenPresenter> {
        Self::IFullScreenPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<FullScreenPresenter>(result__)
        })
    }
    pub fn Kind(&self) -> ::windows::core::Result<AppWindowPresenterKind> {
        let this = &::windows::core::Interface::cast::<IAppWindowPresenter>(self)?;
        unsafe {
            let mut result__: AppWindowPresenterKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AppWindowPresenterKind>(result__)
        }
    }
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
unsafe impl ::windows::core::RuntimeType for FullScreenPresenter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Windowing.FullScreenPresenter;{fa9141fd-b8dd-5da1-8b2b-7cdadb76f593})",
    );
}
unsafe impl ::windows::core::Interface for FullScreenPresenter {
    type Vtable = IFullScreenPresenter_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xfa9141fd_b8dd_5da1_8b2b_7cdadb76f593);
}
impl ::windows::core::RuntimeName for FullScreenPresenter {
    const NAME: &'static str = "Microsoft.UI.Windowing.FullScreenPresenter";
}
impl ::core::convert::From<FullScreenPresenter> for ::windows::core::IUnknown {
    fn from(value: FullScreenPresenter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FullScreenPresenter> for ::windows::core::IUnknown {
    fn from(value: &FullScreenPresenter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FullScreenPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FullScreenPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FullScreenPresenter> for ::windows::core::IInspectable {
    fn from(value: FullScreenPresenter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FullScreenPresenter> for ::windows::core::IInspectable {
    fn from(value: &FullScreenPresenter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FullScreenPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FullScreenPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<FullScreenPresenter> for AppWindowPresenter {
    fn from(value: FullScreenPresenter) -> Self {
        ::core::convert::Into::<AppWindowPresenter>::into(&value)
    }
}
impl ::core::convert::From<&FullScreenPresenter> for AppWindowPresenter {
    fn from(value: &FullScreenPresenter) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, AppWindowPresenter> for FullScreenPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, AppWindowPresenter> {
        ::windows::core::Param::Owned(::core::convert::Into::<AppWindowPresenter>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, AppWindowPresenter> for &FullScreenPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, AppWindowPresenter> {
        ::windows::core::Param::Owned(::core::convert::Into::<AppWindowPresenter>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for FullScreenPresenter {}
unsafe impl ::core::marker::Sync for FullScreenPresenter {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppWindow(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppWindow {
    type Vtable = IAppWindow_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xcfa788b3_643b_5c5e_ad4e_321d48a82acd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindow_abi(
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
        result__: *mut super::WindowId,
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
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut super::WindowId,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::Graphics::PointInt32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::Graphics::SizeInt32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        position: ::windows::Graphics::PointInt32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        rect: ::windows::Graphics::RectInt32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        rect: ::windows::Graphics::RectInt32,
        displayarea: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        size: ::windows::Graphics::SizeInt32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iconpath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iconid: super::IconId,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        appwindowpresenter: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        appwindowpresenterkind: AppWindowPresenterKind,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        activatewindow: bool,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppWindowChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppWindowChangedEventArgs {
    type Vtable = IAppWindowChangedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x2182bc5d_fdac_5c3e_bf37_7d8d684e9d1d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowChangedEventArgs_abi(
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
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppWindowClosingEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppWindowClosingEventArgs {
    type Vtable = IAppWindowClosingEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x0e09d90b_2261_590b_9ad1_8504991d8754);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowClosingEventArgs_abi(
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
        value: bool,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppWindowPresenter(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppWindowPresenter {
    type Vtable = IAppWindowPresenter_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xbc3042c2_c6c6_5632_8989_ff0ec6d3b40d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowPresenter_abi(
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
        result__: *mut AppWindowPresenterKind,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppWindowPresenterFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppWindowPresenterFactory {
    type Vtable = IAppWindowPresenterFactory_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x62082e3c_1368_5238_90d1_e932dc718a82);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowPresenterFactory_abi(
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
pub struct IAppWindowStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppWindowStatics {
    type Vtable = IAppWindowStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x3c315c24_d540_5d72_b518_b226b83627cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowStatics_abi(
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
        appwindowpresenter: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        appwindowpresenter: ::windows::core::RawPtr,
        ownerwindowid: super::WindowId,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        windowid: super::WindowId,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppWindowTitleBar(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppWindowTitleBar {
    type Vtable = IAppWindowTitleBar_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x5574efa2_c91c_5700_a363_539c71a7aaf4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowTitleBar_abi(
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
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
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
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut IconShowOptions,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: IconShowOptions,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value_array_size: u32,
        value: *const ::windows::Graphics::RectInt32,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppWindowTitleBarStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppWindowTitleBarStatics {
    type Vtable = IAppWindowTitleBarStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x9e1da52e_8b15_54d6_a886_f7b9f9d930b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowTitleBarStatics_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICompactOverlayPresenter(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICompactOverlayPresenter {
    type Vtable = ICompactOverlayPresenter_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xefeb0812_6fc7_5b7d_bd92_cc8f9a6454c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompactOverlayPresenter_abi(
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
        result__: *mut CompactOverlaySize,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: CompactOverlaySize,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICompactOverlayPresenterStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICompactOverlayPresenterStatics {
    type Vtable = ICompactOverlayPresenterStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xeab93186_4f6a_52f9_8c03_da57a1522f6e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompactOverlayPresenterStatics_abi(
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
pub struct IDisplayArea(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDisplayArea {
    type Vtable = IDisplayArea_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x5c7e0537_b621_5579_bcae_a84aa8746167);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayArea_abi(
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
        result__: *mut super::DisplayId,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::Graphics::RectInt32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::Graphics::RectInt32,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayAreaStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDisplayAreaStatics {
    type Vtable = IDisplayAreaStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x02ab4926_211e_5d49_8e4b_2af193daed09);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayAreaStatics_abi(
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
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        windowid: super::WindowId,
        displayareafallback: DisplayAreaFallback,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        point: ::windows::Graphics::PointInt32,
        displayareafallback: DisplayAreaFallback,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        rect: ::windows::Graphics::RectInt32,
        displayareafallback: DisplayAreaFallback,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayAreaWatcher(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDisplayAreaWatcher {
    type Vtable = IDisplayAreaWatcher_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x83f6562f_d3a0_548b_8e4f_a99be3d95c9c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayAreaWatcher_abi(
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
        result__: *mut DisplayAreaWatcherStatus,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFullScreenPresenter(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFullScreenPresenter {
    type Vtable = IFullScreenPresenter_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xfa9141fd_b8dd_5da1_8b2b_7cdadb76f593);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFullScreenPresenter_abi(
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
pub struct IFullScreenPresenterStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFullScreenPresenterStatics {
    type Vtable = IFullScreenPresenterStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x2ec0d2c1_e086_55bb_a3b2_44942e231c67);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFullScreenPresenterStatics_abi(
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
pub struct IOverlappedPresenter(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IOverlappedPresenter {
    type Vtable = IOverlappedPresenter_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x21693970_4f4c_5172_9e9d_682a2d174884);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOverlappedPresenter_abi(
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
        value: bool,
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
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: bool,
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
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut OverlappedPresenterState,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        hasborder: bool,
        hastitlebar: bool,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IOverlappedPresenterStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IOverlappedPresenterStatics {
    type Vtable = IOverlappedPresenterStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x997225e4_7b00_5aee_a4be_d4068d1999e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOverlappedPresenterStatics_abi(
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
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct IconShowOptions(pub i32);
impl IconShowOptions {
    pub const ShowIconAndSystemMenu: IconShowOptions = IconShowOptions(0i32);
    pub const HideIconAndSystemMenu: IconShowOptions = IconShowOptions(1i32);
}
impl ::core::convert::From<i32> for IconShowOptions {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for IconShowOptions {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for IconShowOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Windowing.IconShowOptions;i4)",
    );
}
impl ::windows::core::DefaultType for IconShowOptions {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct OverlappedPresenter(pub ::windows::core::IInspectable);
impl OverlappedPresenter {
    pub fn HasBorder(&self) -> ::windows::core::Result<bool> {
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
    pub fn HasTitleBar(&self) -> ::windows::core::Result<bool> {
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
    pub fn IsAlwaysOnTop(&self) -> ::windows::core::Result<bool> {
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
    pub fn SetIsAlwaysOnTop(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsMaximizable(&self) -> ::windows::core::Result<bool> {
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
    pub fn SetIsMaximizable(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsMinimizable(&self) -> ::windows::core::Result<bool> {
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
    pub fn SetIsMinimizable(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsModal(&self) -> ::windows::core::Result<bool> {
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
    pub fn SetIsModal(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsResizable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsResizable(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn State(&self) -> ::windows::core::Result<OverlappedPresenterState> {
        let this = self;
        unsafe {
            let mut result__: OverlappedPresenterState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<OverlappedPresenterState>(result__)
        }
    }
    pub fn Maximize(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Minimize(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Restore(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn SetBorderAndTitleBar(
        &self,
        hasborder: bool,
        hastitlebar: bool,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                hasborder,
                hastitlebar,
            )
            .ok()
        }
    }
    pub fn Create() -> ::windows::core::Result<OverlappedPresenter> {
        Self::IOverlappedPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<OverlappedPresenter>(result__)
        })
    }
    pub fn CreateForContextMenu() -> ::windows::core::Result<OverlappedPresenter> {
        Self::IOverlappedPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<OverlappedPresenter>(result__)
        })
    }
    pub fn CreateForDialog() -> ::windows::core::Result<OverlappedPresenter> {
        Self::IOverlappedPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<OverlappedPresenter>(result__)
        })
    }
    pub fn CreateForToolWindow() -> ::windows::core::Result<OverlappedPresenter> {
        Self::IOverlappedPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<OverlappedPresenter>(result__)
        })
    }
    pub fn Kind(&self) -> ::windows::core::Result<AppWindowPresenterKind> {
        let this = &::windows::core::Interface::cast::<IAppWindowPresenter>(self)?;
        unsafe {
            let mut result__: AppWindowPresenterKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AppWindowPresenterKind>(result__)
        }
    }
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
}
unsafe impl ::windows::core::RuntimeType for OverlappedPresenter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Windowing.OverlappedPresenter;{21693970-4f4c-5172-9e9d-682a2d174884})",
    );
}
unsafe impl ::windows::core::Interface for OverlappedPresenter {
    type Vtable = IOverlappedPresenter_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x21693970_4f4c_5172_9e9d_682a2d174884);
}
impl ::windows::core::RuntimeName for OverlappedPresenter {
    const NAME: &'static str = "Microsoft.UI.Windowing.OverlappedPresenter";
}
impl ::core::convert::From<OverlappedPresenter> for ::windows::core::IUnknown {
    fn from(value: OverlappedPresenter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&OverlappedPresenter> for ::windows::core::IUnknown {
    fn from(value: &OverlappedPresenter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for OverlappedPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a OverlappedPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<OverlappedPresenter> for ::windows::core::IInspectable {
    fn from(value: OverlappedPresenter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&OverlappedPresenter> for ::windows::core::IInspectable {
    fn from(value: &OverlappedPresenter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for OverlappedPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a OverlappedPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<OverlappedPresenter> for AppWindowPresenter {
    fn from(value: OverlappedPresenter) -> Self {
        ::core::convert::Into::<AppWindowPresenter>::into(&value)
    }
}
impl ::core::convert::From<&OverlappedPresenter> for AppWindowPresenter {
    fn from(value: &OverlappedPresenter) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, AppWindowPresenter> for OverlappedPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, AppWindowPresenter> {
        ::windows::core::Param::Owned(::core::convert::Into::<AppWindowPresenter>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, AppWindowPresenter> for &OverlappedPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, AppWindowPresenter> {
        ::windows::core::Param::Owned(::core::convert::Into::<AppWindowPresenter>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for OverlappedPresenter {}
unsafe impl ::core::marker::Sync for OverlappedPresenter {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct OverlappedPresenterState(pub i32);
impl OverlappedPresenterState {
    pub const Maximized: OverlappedPresenterState = OverlappedPresenterState(0i32);
    pub const Minimized: OverlappedPresenterState = OverlappedPresenterState(1i32);
    pub const Restored: OverlappedPresenterState = OverlappedPresenterState(2i32);
}
impl ::core::convert::From<i32> for OverlappedPresenterState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for OverlappedPresenterState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for OverlappedPresenterState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Windowing.OverlappedPresenterState;i4)",
    );
}
impl ::windows::core::DefaultType for OverlappedPresenterState {
    type DefaultType = Self;
}
