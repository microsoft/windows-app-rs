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
pub struct CoreWebView2(pub ::windows::core::IInspectable);
impl CoreWebView2 {
    pub fn Settings(&self) -> ::windows::core::Result<CoreWebView2Settings> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2Settings>(result__)
        }
    }
    pub fn Source(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
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
    pub fn BrowserProcessId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn CanGoBack(&self) -> ::windows::core::Result<bool> {
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
    pub fn CanGoForward(&self) -> ::windows::core::Result<bool> {
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
    pub fn DocumentTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ContainsFullScreenElement(&self) -> ::windows::core::Result<bool> {
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
    pub fn NavigationStarting<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2NavigationStartingEventArgs,
            >,
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
    pub fn RemoveNavigationStarting<
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
    pub fn ContentLoading<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2ContentLoadingEventArgs,
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
    pub fn RemoveContentLoading<
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
    pub fn SourceChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2SourceChangedEventArgs,
            >,
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
    pub fn RemoveSourceChanged<
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
    pub fn HistoryChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<CoreWebView2, ::windows::core::IInspectable>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveHistoryChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn NavigationCompleted<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2NavigationCompletedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveNavigationCompleted<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FrameNavigationStarting<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2NavigationStartingEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveFrameNavigationStarting<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).24)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FrameNavigationCompleted<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2NavigationCompletedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveFrameNavigationCompleted<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).26)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ScriptDialogOpening<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2ScriptDialogOpeningEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveScriptDialogOpening<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).28)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn PermissionRequested<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2PermissionRequestedEventArgs,
            >,
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
    pub fn RemovePermissionRequested<
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
    pub fn ProcessFailed<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2ProcessFailedEventArgs,
            >,
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
    pub fn RemoveProcessFailed<
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
    pub fn WebMessageReceived<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2WebMessageReceivedEventArgs,
            >,
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
    pub fn RemoveWebMessageReceived<
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
    pub fn NewWindowRequested<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2NewWindowRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).35)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveNewWindowRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).36)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn DocumentTitleChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<CoreWebView2, ::windows::core::IInspectable>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).37)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDocumentTitleChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).38)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ContainsFullScreenElementChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<CoreWebView2, ::windows::core::IInspectable>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).39)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveContainsFullScreenElementChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).40)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn WebResourceRequested<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2WebResourceRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).41)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveWebResourceRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).42)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn WindowCloseRequested<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<CoreWebView2, ::windows::core::IInspectable>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).43)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveWindowCloseRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).44)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Navigate<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        uri: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).45)(
                ::core::mem::transmute_copy(this),
                uri.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn NavigateToString<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        htmlcontent: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).46)(
                ::core::mem::transmute_copy(this),
                htmlcontent.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn AddScriptToExecuteOnDocumentCreatedAsync<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        javascript: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncOperation<::windows::core::HSTRING>>
    {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).47)(
                ::core::mem::transmute_copy(this),
                javascript.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    pub fn RemoveScriptToExecuteOnDocumentCreated<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        id: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).48)(
                ::core::mem::transmute_copy(this),
                id.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ExecuteScriptAsync<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        javascript: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncOperation<::windows::core::HSTRING>>
    {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).49)(
                ::core::mem::transmute_copy(this),
                javascript.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    pub fn CapturePreviewAsync<
        'a,
        Param1: ::windows::core::IntoParam<'a, ::windows::Storage::Streams::IRandomAccessStream>,
    >(
        &self,
        imageformat: CoreWebView2CapturePreviewImageFormat,
        imagestream: Param1,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).50)(
                ::core::mem::transmute_copy(this),
                imageformat,
                imagestream.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn Reload(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).51)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn PostWebMessageAsJson<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        webmessageasjson: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).52)(
                ::core::mem::transmute_copy(this),
                webmessageasjson.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn PostWebMessageAsString<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        webmessageasstring: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).53)(
                ::core::mem::transmute_copy(this),
                webmessageasstring.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn CallDevToolsProtocolMethodAsync<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        methodname: Param0,
        parametersasjson: Param1,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncOperation<::windows::core::HSTRING>>
    {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).54)(
                ::core::mem::transmute_copy(this),
                methodname.into_param().abi(),
                parametersasjson.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    pub fn GoBack(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).55)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn GoForward(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).56)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn GetDevToolsProtocolEventReceiver<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        eventname: Param0,
    ) -> ::windows::core::Result<CoreWebView2DevToolsProtocolEventReceiver> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).57)(
                ::core::mem::transmute_copy(this),
                eventname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<CoreWebView2DevToolsProtocolEventReceiver>(result__)
        }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).58)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn AddHostObjectToScript<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        name: Param0,
        rawobject: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).59)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                rawobject.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveHostObjectFromScript<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).60)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn OpenDevToolsWindow(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).61)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn AddWebResourceRequestedFilter<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        uri: Param0,
        resourcecontext: CoreWebView2WebResourceContext,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).62)(
                ::core::mem::transmute_copy(this),
                uri.into_param().abi(),
                resourcecontext,
            )
            .ok()
        }
    }
    pub fn RemoveWebResourceRequestedFilter<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        uri: Param0,
        resourcecontext: CoreWebView2WebResourceContext,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).63)(
                ::core::mem::transmute_copy(this),
                uri.into_param().abi(),
                resourcecontext,
            )
            .ok()
        }
    }
    pub fn CookieManager(&self) -> ::windows::core::Result<CoreWebView2CookieManager> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2_2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2CookieManager>(result__)
        }
    }
    pub fn Environment(&self) -> ::windows::core::Result<CoreWebView2Environment> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2_2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2Environment>(result__)
        }
    }
    pub fn WebResourceResponseReceived<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2WebResourceResponseReceivedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2_2>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveWebResourceResponseReceived<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2_2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn DOMContentLoaded<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2DOMContentLoadedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2_2>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDOMContentLoaded<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2_2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn NavigateWithWebResourceRequest<
        'a,
        Param0: ::windows::core::IntoParam<'a, CoreWebView2WebResourceRequest>,
    >(
        &self,
        request: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2_2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                request.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IsSuspended(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2_3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn TrySuspendAsync(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2_3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn Resume(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2_3>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn SetVirtualHostNameToFolderMapping<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        hostname: Param0,
        folderpath: Param1,
        accesskind: CoreWebView2HostResourceAccessKind,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2_3>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                hostname.into_param().abi(),
                folderpath.into_param().abi(),
                accesskind,
            )
            .ok()
        }
    }
    pub fn ClearVirtualHostNameToFolderMapping<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        hostname: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2_3>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                hostname.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FrameCreated<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2FrameCreatedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2_4>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveFrameCreated<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2_4>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn DownloadStarting<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2DownloadStartingEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2_4>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDownloadStarting<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2_4>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ClientCertificateRequested<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2ClientCertificateRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2_5>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveClientCertificateRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2_5>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn OpenTaskManagerWindow(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2_6>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn PrintToPdfAsync<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, CoreWebView2PrintSettings>,
    >(
        &self,
        resultfilepath: Param0,
        printsettings: Param1,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2_7>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                resultfilepath.into_param().abi(),
                printsettings.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Web.WebView2.Core.CoreWebView2;{3a3f559a-e5e9-5338-bb67-4eb0504a4f14})",
    );
}
unsafe impl ::windows::core::Interface for CoreWebView2 {
    type Vtable = ICoreWebView2_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x3a3f559a_e5e9_5338_bb67_4eb0504a4f14);
}
impl ::windows::core::RuntimeName for CoreWebView2 {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2";
}
impl ::core::convert::From<CoreWebView2> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWebView2> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreWebView2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreWebView2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWebView2> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWebView2> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreWebView2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreWebView2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreWebView2 {}
unsafe impl ::core::marker::Sync for CoreWebView2 {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CoreWebView2AcceleratorKeyPressedEventArgs(pub ::windows::core::IInspectable);
impl CoreWebView2AcceleratorKeyPressedEventArgs {
    pub fn KeyEventKind(&self) -> ::windows::core::Result<CoreWebView2KeyEventKind> {
        let this = self;
        unsafe {
            let mut result__: CoreWebView2KeyEventKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2KeyEventKind>(result__)
        }
    }
    pub fn VirtualKey(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn KeyEventLParam(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn PhysicalKeyStatus(&self) -> ::windows::core::Result<CoreWebView2PhysicalKeyStatus> {
        let this = self;
        unsafe {
            let mut result__: CoreWebView2PhysicalKeyStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2PhysicalKeyStatus>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
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
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2AcceleratorKeyPressedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2AcceleratorKeyPressedEventArgs;{41a56100-92a5-59d1-9e71-9222e33ae38b})" ) ;
}
unsafe impl ::windows::core::Interface for CoreWebView2AcceleratorKeyPressedEventArgs {
    type Vtable = ICoreWebView2AcceleratorKeyPressedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x41a56100_92a5_59d1_9e71_9222e33ae38b);
}
impl ::windows::core::RuntimeName for CoreWebView2AcceleratorKeyPressedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2AcceleratorKeyPressedEventArgs";
}
impl ::core::convert::From<CoreWebView2AcceleratorKeyPressedEventArgs>
    for ::windows::core::IUnknown
{
    fn from(value: CoreWebView2AcceleratorKeyPressedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWebView2AcceleratorKeyPressedEventArgs>
    for ::windows::core::IUnknown
{
    fn from(value: &CoreWebView2AcceleratorKeyPressedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2AcceleratorKeyPressedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2AcceleratorKeyPressedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWebView2AcceleratorKeyPressedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: CoreWebView2AcceleratorKeyPressedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWebView2AcceleratorKeyPressedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: &CoreWebView2AcceleratorKeyPressedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2AcceleratorKeyPressedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2AcceleratorKeyPressedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreWebView2AcceleratorKeyPressedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2AcceleratorKeyPressedEventArgs {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CoreWebView2BoundsMode(pub i32);
impl CoreWebView2BoundsMode {
    pub const UseRawPixels: CoreWebView2BoundsMode = CoreWebView2BoundsMode(0i32);
    pub const UseRasterizationScale: CoreWebView2BoundsMode = CoreWebView2BoundsMode(1i32);
}
impl ::core::convert::From<i32> for CoreWebView2BoundsMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CoreWebView2BoundsMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2BoundsMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2BoundsMode;i4)",
    );
}
impl ::windows::core::DefaultType for CoreWebView2BoundsMode {
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
pub struct CoreWebView2BrowserProcessExitKind(pub i32);
impl CoreWebView2BrowserProcessExitKind {
    pub const Normal: CoreWebView2BrowserProcessExitKind = CoreWebView2BrowserProcessExitKind(0i32);
    pub const Failed: CoreWebView2BrowserProcessExitKind = CoreWebView2BrowserProcessExitKind(1i32);
}
impl ::core::convert::From<i32> for CoreWebView2BrowserProcessExitKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CoreWebView2BrowserProcessExitKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2BrowserProcessExitKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2BrowserProcessExitKind;i4)",
    );
}
impl ::windows::core::DefaultType for CoreWebView2BrowserProcessExitKind {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CoreWebView2BrowserProcessExitedEventArgs(pub ::windows::core::IInspectable);
impl CoreWebView2BrowserProcessExitedEventArgs {
    pub fn BrowserProcessExitKind(
        &self,
    ) -> ::windows::core::Result<CoreWebView2BrowserProcessExitKind> {
        let this = self;
        unsafe {
            let mut result__: CoreWebView2BrowserProcessExitKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2BrowserProcessExitKind>(result__)
        }
    }
    pub fn BrowserProcessId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2BrowserProcessExitedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2BrowserProcessExitedEventArgs;{79963f77-1484-5a46-b91f-dfc5c1a0ce14})" ) ;
}
unsafe impl ::windows::core::Interface for CoreWebView2BrowserProcessExitedEventArgs {
    type Vtable = ICoreWebView2BrowserProcessExitedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x79963f77_1484_5a46_b91f_dfc5c1a0ce14);
}
impl ::windows::core::RuntimeName for CoreWebView2BrowserProcessExitedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2BrowserProcessExitedEventArgs";
}
impl ::core::convert::From<CoreWebView2BrowserProcessExitedEventArgs>
    for ::windows::core::IUnknown
{
    fn from(value: CoreWebView2BrowserProcessExitedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWebView2BrowserProcessExitedEventArgs>
    for ::windows::core::IUnknown
{
    fn from(value: &CoreWebView2BrowserProcessExitedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2BrowserProcessExitedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2BrowserProcessExitedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWebView2BrowserProcessExitedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: CoreWebView2BrowserProcessExitedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWebView2BrowserProcessExitedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: &CoreWebView2BrowserProcessExitedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2BrowserProcessExitedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2BrowserProcessExitedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreWebView2BrowserProcessExitedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2BrowserProcessExitedEventArgs {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CoreWebView2CapturePreviewImageFormat(pub i32);
impl CoreWebView2CapturePreviewImageFormat {
    pub const Png: CoreWebView2CapturePreviewImageFormat =
        CoreWebView2CapturePreviewImageFormat(0i32);
    pub const Jpeg: CoreWebView2CapturePreviewImageFormat =
        CoreWebView2CapturePreviewImageFormat(1i32);
}
impl ::core::convert::From<i32> for CoreWebView2CapturePreviewImageFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CoreWebView2CapturePreviewImageFormat {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2CapturePreviewImageFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2CapturePreviewImageFormat;i4)",
    );
}
impl ::windows::core::DefaultType for CoreWebView2CapturePreviewImageFormat {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CoreWebView2ClientCertificate(pub ::windows::core::IInspectable);
impl CoreWebView2ClientCertificate {
    pub fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
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
    pub fn Issuer(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
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
    pub fn ValidFrom(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn ValidTo(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn DerEncodedSerialNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn PemEncodedIssuerCertificateChain(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IVectorView<::windows::core::HSTRING>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(
                result__,
            )
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<CoreWebView2ClientCertificateKind> {
        let this = self;
        unsafe {
            let mut result__: CoreWebView2ClientCertificateKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2ClientCertificateKind>(result__)
        }
    }
    pub fn ToPemEncoding(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
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
    pub fn ToCertificate(
        &self,
    ) -> ::windows::core::Result<::windows::Security::Cryptography::Certificates::Certificate> {
        let this = &::windows::core::Interface::cast::<CoreWebView2ClientCertificate_Manual>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Security::Cryptography::Certificates::Certificate>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2ClientCertificate {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2ClientCertificate;{091b39f2-68df-52b4-8fb0-fd3561af41f2})" ) ;
}
unsafe impl ::windows::core::Interface for CoreWebView2ClientCertificate {
    type Vtable = ICoreWebView2ClientCertificate_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x091b39f2_68df_52b4_8fb0_fd3561af41f2);
}
impl ::windows::core::RuntimeName for CoreWebView2ClientCertificate {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2ClientCertificate";
}
impl ::core::convert::From<CoreWebView2ClientCertificate> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2ClientCertificate) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWebView2ClientCertificate> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2ClientCertificate) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2ClientCertificate
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2ClientCertificate
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWebView2ClientCertificate> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2ClientCertificate) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWebView2ClientCertificate> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2ClientCertificate) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2ClientCertificate
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2ClientCertificate
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreWebView2ClientCertificate {}
unsafe impl ::core::marker::Sync for CoreWebView2ClientCertificate {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CoreWebView2ClientCertificateKind(pub i32);
impl CoreWebView2ClientCertificateKind {
    pub const SmartCard: CoreWebView2ClientCertificateKind =
        CoreWebView2ClientCertificateKind(0i32);
    pub const Pin: CoreWebView2ClientCertificateKind = CoreWebView2ClientCertificateKind(1i32);
    pub const Other: CoreWebView2ClientCertificateKind = CoreWebView2ClientCertificateKind(2i32);
}
impl ::core::convert::From<i32> for CoreWebView2ClientCertificateKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CoreWebView2ClientCertificateKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2ClientCertificateKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2ClientCertificateKind;i4)",
    );
}
impl ::windows::core::DefaultType for CoreWebView2ClientCertificateKind {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CoreWebView2ClientCertificateRequestedEventArgs(pub ::windows::core::IInspectable);
impl CoreWebView2ClientCertificateRequestedEventArgs {
    pub fn Host(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
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
    pub fn Port(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn IsProxy(&self) -> ::windows::core::Result<bool> {
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
    pub fn AllowedCertificateAuthorities(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IVectorView<::windows::core::HSTRING>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(
                result__,
            )
        }
    }
    pub fn MutuallyTrustedCertificates(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IVectorView<CoreWebView2ClientCertificate>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVectorView<
                CoreWebView2ClientCertificate,
            >>(result__)
        }
    }
    pub fn SelectedCertificate(&self) -> ::windows::core::Result<CoreWebView2ClientCertificate> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2ClientCertificate>(result__)
        }
    }
    pub fn SetSelectedCertificate<
        'a,
        Param0: ::windows::core::IntoParam<'a, CoreWebView2ClientCertificate>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Cancel(&self) -> ::windows::core::Result<bool> {
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
    pub fn SetCancel(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn GetDeferral(&self) -> ::windows::core::Result<::windows::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2ClientCertificateRequestedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2ClientCertificateRequestedEventArgs;{93287b55-31f9-55a0-b68b-d9841d7e1bf4})" ) ;
}
unsafe impl ::windows::core::Interface for CoreWebView2ClientCertificateRequestedEventArgs {
    type Vtable = ICoreWebView2ClientCertificateRequestedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x93287b55_31f9_55a0_b68b_d9841d7e1bf4);
}
impl ::windows::core::RuntimeName for CoreWebView2ClientCertificateRequestedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2ClientCertificateRequestedEventArgs";
}
impl ::core::convert::From<CoreWebView2ClientCertificateRequestedEventArgs>
    for ::windows::core::IUnknown
{
    fn from(value: CoreWebView2ClientCertificateRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWebView2ClientCertificateRequestedEventArgs>
    for ::windows::core::IUnknown
{
    fn from(value: &CoreWebView2ClientCertificateRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2ClientCertificateRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2ClientCertificateRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWebView2ClientCertificateRequestedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: CoreWebView2ClientCertificateRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWebView2ClientCertificateRequestedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: &CoreWebView2ClientCertificateRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2ClientCertificateRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2ClientCertificateRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreWebView2ClientCertificateRequestedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2ClientCertificateRequestedEventArgs {}
#[repr(transparent)]
#[doc(hidden)]
pub struct CoreWebView2ClientCertificate_Manual(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for CoreWebView2ClientCertificate_Manual {
    type Vtable = CoreWebView2ClientCertificate_Manual_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xfaefefc2_20c3_5d86_8a74_f6d87d6ff8fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct CoreWebView2ClientCertificate_Manual_abi(
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
pub struct CoreWebView2CompositionController(pub ::windows::core::IInspectable);
impl CoreWebView2CompositionController {
    pub fn RootVisualTarget(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
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
    pub fn SetRootVisualTarget<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
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
    pub fn CursorChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2CompositionController,
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
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveCursorChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn SendMouseInput<
        'a,
        Param3: ::windows::core::IntoParam<'a, ::windows::Foundation::Point>,
    >(
        &self,
        eventkind: CoreWebView2MouseEventKind,
        virtualkeys: CoreWebView2MouseEventVirtualKeys,
        mousedata: u32,
        point: Param3,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                eventkind,
                virtualkeys,
                mousedata,
                point.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn SendPointerInput<'a, Param1: ::windows::core::IntoParam<'a, CoreWebView2PointerInfo>>(
        &self,
        eventkind: CoreWebView2PointerEventKind,
        pointerinfo: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                eventkind,
                pointerinfo.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Cursor(&self) -> ::windows::core::Result<::windows::UI::Core::CoreCursor> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreCursor>(result__)
        }
    }
    pub fn IsVisible(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn Bounds(&self) -> ::windows::core::Result<::windows::Foundation::Rect> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Rect>(result__)
        }
    }
    pub fn SetBounds<'a, Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Rect>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ZoomFactor(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetZoomFactor(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn ParentWindow(&self) -> ::windows::core::Result<CoreWebView2ControllerWindowReference> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2ControllerWindowReference>(result__)
        }
    }
    pub fn SetParentWindow<
        'a,
        Param0: ::windows::core::IntoParam<'a, CoreWebView2ControllerWindowReference>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn CoreWebView2(&self) -> ::windows::core::Result<CoreWebView2> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2>(result__)
        }
    }
    pub fn ZoomFactorChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2Controller,
                ::windows::core::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller>(self)?;
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
    pub fn RemoveZoomFactorChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn MoveFocusRequested<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2Controller,
                CoreWebView2MoveFocusRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller>(self)?;
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
    pub fn RemoveMoveFocusRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn GotFocus<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2Controller,
                ::windows::core::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveGotFocus<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn LostFocus<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2Controller,
                ::windows::core::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveLostFocus<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn AcceleratorKeyPressed<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2Controller,
                CoreWebView2AcceleratorKeyPressedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAcceleratorKeyPressed<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).24)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn SetBoundsAndZoomFactor<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Rect>,
    >(
        &self,
        bounds: Param0,
        zoomfactor: f64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                bounds.into_param().abi(),
                zoomfactor,
            )
            .ok()
        }
    }
    pub fn MoveFocus(&self, reason: CoreWebView2MoveFocusReason) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), reason)
                .ok()
        }
    }
    pub fn NotifyParentWindowPositionChanged(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn DefaultBackgroundColor(&self) -> ::windows::core::Result<::windows::UI::Color> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller2>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        }
    }
    pub fn SetDefaultBackgroundColor<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::UI::Color>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn RasterizationScale(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetRasterizationScale(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn ShouldDetectMonitorScaleChanges(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetShouldDetectMonitorScaleChanges(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn BoundsMode(&self) -> ::windows::core::Result<CoreWebView2BoundsMode> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            let mut result__: CoreWebView2BoundsMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2BoundsMode>(result__)
        }
    }
    pub fn SetBoundsMode(&self, value: CoreWebView2BoundsMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn RasterizationScaleChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2Controller,
                ::windows::core::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveRasterizationScaleChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2CompositionController {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2CompositionController;{4fb8b7b3-4a2e-5787-94b9-cc48c4d364d7})" ) ;
}
unsafe impl ::windows::core::Interface for CoreWebView2CompositionController {
    type Vtable = ICoreWebView2CompositionController_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4fb8b7b3_4a2e_5787_94b9_cc48c4d364d7);
}
impl ::windows::core::RuntimeName for CoreWebView2CompositionController {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2CompositionController";
}
impl ::core::convert::From<CoreWebView2CompositionController> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2CompositionController) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWebView2CompositionController> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2CompositionController) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2CompositionController
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2CompositionController
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWebView2CompositionController> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2CompositionController) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWebView2CompositionController> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2CompositionController) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2CompositionController
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2CompositionController
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<CoreWebView2CompositionController> for CoreWebView2Controller {
    fn from(value: CoreWebView2CompositionController) -> Self {
        ::core::convert::Into::<CoreWebView2Controller>::into(&value)
    }
}
impl ::core::convert::From<&CoreWebView2CompositionController> for CoreWebView2Controller {
    fn from(value: &CoreWebView2CompositionController) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, CoreWebView2Controller>
    for CoreWebView2CompositionController
{
    fn into_param(self) -> ::windows::core::Param<'a, CoreWebView2Controller> {
        ::windows::core::Param::Owned(::core::convert::Into::<CoreWebView2Controller>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, CoreWebView2Controller>
    for &CoreWebView2CompositionController
{
    fn into_param(self) -> ::windows::core::Param<'a, CoreWebView2Controller> {
        ::windows::core::Param::Owned(::core::convert::Into::<CoreWebView2Controller>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for CoreWebView2CompositionController {}
unsafe impl ::core::marker::Sync for CoreWebView2CompositionController {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CoreWebView2ContentLoadingEventArgs(pub ::windows::core::IInspectable);
impl CoreWebView2ContentLoadingEventArgs {
    pub fn IsErrorPage(&self) -> ::windows::core::Result<bool> {
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
    pub fn NavigationId(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u64>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2ContentLoadingEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2ContentLoadingEventArgs;{6cf95373-946c-5dae-9b3e-0fe23d5aa29f})" ) ;
}
unsafe impl ::windows::core::Interface for CoreWebView2ContentLoadingEventArgs {
    type Vtable = ICoreWebView2ContentLoadingEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x6cf95373_946c_5dae_9b3e_0fe23d5aa29f);
}
impl ::windows::core::RuntimeName for CoreWebView2ContentLoadingEventArgs {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2ContentLoadingEventArgs";
}
impl ::core::convert::From<CoreWebView2ContentLoadingEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2ContentLoadingEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWebView2ContentLoadingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2ContentLoadingEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2ContentLoadingEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2ContentLoadingEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWebView2ContentLoadingEventArgs> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2ContentLoadingEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWebView2ContentLoadingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2ContentLoadingEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2ContentLoadingEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2ContentLoadingEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreWebView2ContentLoadingEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2ContentLoadingEventArgs {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CoreWebView2Controller(pub ::windows::core::IInspectable);
impl CoreWebView2Controller {
    pub fn IsVisible(&self) -> ::windows::core::Result<bool> {
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
    pub fn SetIsVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn Bounds(&self) -> ::windows::core::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Rect>(result__)
        }
    }
    pub fn SetBounds<'a, Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Rect>>(
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
    pub fn ZoomFactor(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetZoomFactor(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn ParentWindow(&self) -> ::windows::core::Result<CoreWebView2ControllerWindowReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2ControllerWindowReference>(result__)
        }
    }
    pub fn SetParentWindow<
        'a,
        Param0: ::windows::core::IntoParam<'a, CoreWebView2ControllerWindowReference>,
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
    pub fn CoreWebView2(&self) -> ::windows::core::Result<CoreWebView2> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2>(result__)
        }
    }
    pub fn ZoomFactorChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2Controller,
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
    pub fn RemoveZoomFactorChanged<
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
    pub fn MoveFocusRequested<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2Controller,
                CoreWebView2MoveFocusRequestedEventArgs,
            >,
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
    pub fn RemoveMoveFocusRequested<
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
    pub fn GotFocus<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2Controller,
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
            (::windows::core::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveGotFocus<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn LostFocus<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2Controller,
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
            (::windows::core::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveLostFocus<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn AcceleratorKeyPressed<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2Controller,
                CoreWebView2AcceleratorKeyPressedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAcceleratorKeyPressed<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).24)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn SetBoundsAndZoomFactor<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Rect>,
    >(
        &self,
        bounds: Param0,
        zoomfactor: f64,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                bounds.into_param().abi(),
                zoomfactor,
            )
            .ok()
        }
    }
    pub fn MoveFocus(&self, reason: CoreWebView2MoveFocusReason) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), reason)
                .ok()
        }
    }
    pub fn NotifyParentWindowPositionChanged(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn DefaultBackgroundColor(&self) -> ::windows::core::Result<::windows::UI::Color> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller2>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        }
    }
    pub fn SetDefaultBackgroundColor<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::UI::Color>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn RasterizationScale(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetRasterizationScale(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn ShouldDetectMonitorScaleChanges(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetShouldDetectMonitorScaleChanges(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn BoundsMode(&self) -> ::windows::core::Result<CoreWebView2BoundsMode> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            let mut result__: CoreWebView2BoundsMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2BoundsMode>(result__)
        }
    }
    pub fn SetBoundsMode(&self, value: CoreWebView2BoundsMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn RasterizationScaleChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2Controller,
                ::windows::core::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveRasterizationScaleChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2Controller {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2Controller;{a588121c-53bf-590e-80e5-29d729cbd743})" ) ;
}
unsafe impl ::windows::core::Interface for CoreWebView2Controller {
    type Vtable = ICoreWebView2Controller_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa588121c_53bf_590e_80e5_29d729cbd743);
}
impl ::windows::core::RuntimeName for CoreWebView2Controller {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2Controller";
}
impl ::core::convert::From<CoreWebView2Controller> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2Controller) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWebView2Controller> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2Controller) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreWebView2Controller {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreWebView2Controller {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWebView2Controller> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2Controller) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWebView2Controller> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2Controller) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreWebView2Controller {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2Controller
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreWebView2Controller {}
unsafe impl ::core::marker::Sync for CoreWebView2Controller {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CoreWebView2ControllerWindowReference(pub ::windows::core::IInspectable);
impl CoreWebView2ControllerWindowReference {
    pub fn WindowHandle(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u64>(result__)
        }
    }
    pub fn CoreWindow(&self) -> ::windows::core::Result<::windows::UI::Core::CoreWindow> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreWindow>(result__)
        }
    }
    pub fn CreateFromWindowHandle(
        windowhandle: u64,
    ) -> ::windows::core::Result<CoreWebView2ControllerWindowReference> {
        Self::ICoreWebView2ControllerWindowReferenceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                windowhandle,
                &mut result__,
            )
            .from_abi::<CoreWebView2ControllerWindowReference>(result__)
        })
    }
    pub fn CreateFromCoreWindow<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::UI::Core::CoreWindow>,
    >(
        corewindow: Param0,
    ) -> ::windows::core::Result<CoreWebView2ControllerWindowReference> {
        Self::ICoreWebView2ControllerWindowReferenceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                corewindow.into_param().abi(),
                &mut result__,
            )
            .from_abi::<CoreWebView2ControllerWindowReference>(result__)
        })
    }
    pub fn ICoreWebView2ControllerWindowReferenceStatics<
        R,
        F: FnOnce(&ICoreWebView2ControllerWindowReferenceStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            CoreWebView2ControllerWindowReference,
            ICoreWebView2ControllerWindowReferenceStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2ControllerWindowReference {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2ControllerWindowReference;{0feddad4-48a3-5cc4-9f61-e7adfd1e9c76})" ) ;
}
unsafe impl ::windows::core::Interface for CoreWebView2ControllerWindowReference {
    type Vtable = ICoreWebView2ControllerWindowReference_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x0feddad4_48a3_5cc4_9f61_e7adfd1e9c76);
}
impl ::windows::core::RuntimeName for CoreWebView2ControllerWindowReference {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2ControllerWindowReference";
}
impl ::core::convert::From<CoreWebView2ControllerWindowReference> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2ControllerWindowReference) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWebView2ControllerWindowReference> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2ControllerWindowReference) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2ControllerWindowReference
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2ControllerWindowReference
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWebView2ControllerWindowReference>
    for ::windows::core::IInspectable
{
    fn from(value: CoreWebView2ControllerWindowReference) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWebView2ControllerWindowReference>
    for ::windows::core::IInspectable
{
    fn from(value: &CoreWebView2ControllerWindowReference) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2ControllerWindowReference
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2ControllerWindowReference
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreWebView2ControllerWindowReference {}
unsafe impl ::core::marker::Sync for CoreWebView2ControllerWindowReference {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CoreWebView2Cookie(pub ::windows::core::IInspectable);
impl CoreWebView2Cookie {
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
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
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
    pub fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Domain(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
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
    pub fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Expires(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetExpires(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsHttpOnly(&self) -> ::windows::core::Result<bool> {
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
    pub fn SetIsHttpOnly(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn SameSite(&self) -> ::windows::core::Result<CoreWebView2CookieSameSiteKind> {
        let this = self;
        unsafe {
            let mut result__: CoreWebView2CookieSameSiteKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2CookieSameSiteKind>(result__)
        }
    }
    pub fn SetSameSite(
        &self,
        value: CoreWebView2CookieSameSiteKind,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsSecure(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsSecure(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsSession(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2Cookie {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2Cookie;{52f670fe-8ca2-5aad-aedb-25f7903b7038})" ) ;
}
unsafe impl ::windows::core::Interface for CoreWebView2Cookie {
    type Vtable = ICoreWebView2Cookie_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x52f670fe_8ca2_5aad_aedb_25f7903b7038);
}
impl ::windows::core::RuntimeName for CoreWebView2Cookie {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2Cookie";
}
impl ::core::convert::From<CoreWebView2Cookie> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2Cookie) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWebView2Cookie> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2Cookie) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreWebView2Cookie {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreWebView2Cookie {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWebView2Cookie> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2Cookie) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWebView2Cookie> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2Cookie) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreWebView2Cookie {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreWebView2Cookie {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreWebView2Cookie {}
unsafe impl ::core::marker::Sync for CoreWebView2Cookie {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CoreWebView2CookieManager(pub ::windows::core::IInspectable);
impl CoreWebView2CookieManager {
    pub fn CreateCookie<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        name: Param0,
        value: Param1,
        domain: Param2,
        path: Param3,
    ) -> ::windows::core::Result<CoreWebView2Cookie> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                value.into_param().abi(),
                domain.into_param().abi(),
                path.into_param().abi(),
                &mut result__,
            )
            .from_abi::<CoreWebView2Cookie>(result__)
        }
    }
    pub fn CopyCookie<'a, Param0: ::windows::core::IntoParam<'a, CoreWebView2Cookie>>(
        &self,
        cookieparam: Param0,
    ) -> ::windows::core::Result<CoreWebView2Cookie> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                cookieparam.into_param().abi(),
                &mut result__,
            )
            .from_abi::<CoreWebView2Cookie>(result__)
        }
    }
    pub fn AddOrUpdateCookie<'a, Param0: ::windows::core::IntoParam<'a, CoreWebView2Cookie>>(
        &self,
        cookie: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                cookie.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn DeleteCookie<'a, Param0: ::windows::core::IntoParam<'a, CoreWebView2Cookie>>(
        &self,
        cookie: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                cookie.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn DeleteCookies<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        name: Param0,
        uri: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                uri.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn DeleteCookiesWithDomainAndPath<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        name: Param0,
        domain: Param1,
        path: Param2,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                domain.into_param().abi(),
                path.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn DeleteAllCookies(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2CookieManager {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2CookieManager;{4098f516-adca-5563-aaa5-d7affd847aa3})" ) ;
}
unsafe impl ::windows::core::Interface for CoreWebView2CookieManager {
    type Vtable = ICoreWebView2CookieManager_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4098f516_adca_5563_aaa5_d7affd847aa3);
}
impl ::windows::core::RuntimeName for CoreWebView2CookieManager {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2CookieManager";
}
impl ::core::convert::From<CoreWebView2CookieManager> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2CookieManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWebView2CookieManager> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2CookieManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreWebView2CookieManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2CookieManager
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWebView2CookieManager> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2CookieManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWebView2CookieManager> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2CookieManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2CookieManager
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2CookieManager
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreWebView2CookieManager {}
unsafe impl ::core::marker::Sync for CoreWebView2CookieManager {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CoreWebView2CookieSameSiteKind(pub i32);
impl CoreWebView2CookieSameSiteKind {
    pub const None: CoreWebView2CookieSameSiteKind = CoreWebView2CookieSameSiteKind(0i32);
    pub const Lax: CoreWebView2CookieSameSiteKind = CoreWebView2CookieSameSiteKind(1i32);
    pub const Strict: CoreWebView2CookieSameSiteKind = CoreWebView2CookieSameSiteKind(2i32);
}
impl ::core::convert::From<i32> for CoreWebView2CookieSameSiteKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CoreWebView2CookieSameSiteKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2CookieSameSiteKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2CookieSameSiteKind;i4)",
    );
}
impl ::windows::core::DefaultType for CoreWebView2CookieSameSiteKind {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CoreWebView2DOMContentLoadedEventArgs(pub ::windows::core::IInspectable);
impl CoreWebView2DOMContentLoadedEventArgs {
    pub fn NavigationId(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u64>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2DOMContentLoadedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2DOMContentLoadedEventArgs;{c474d0a3-24ac-59fc-b78b-da7562a6a052})" ) ;
}
unsafe impl ::windows::core::Interface for CoreWebView2DOMContentLoadedEventArgs {
    type Vtable = ICoreWebView2DOMContentLoadedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc474d0a3_24ac_59fc_b78b_da7562a6a052);
}
impl ::windows::core::RuntimeName for CoreWebView2DOMContentLoadedEventArgs {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2DOMContentLoadedEventArgs";
}
impl ::core::convert::From<CoreWebView2DOMContentLoadedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2DOMContentLoadedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWebView2DOMContentLoadedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2DOMContentLoadedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2DOMContentLoadedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2DOMContentLoadedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWebView2DOMContentLoadedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: CoreWebView2DOMContentLoadedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWebView2DOMContentLoadedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: &CoreWebView2DOMContentLoadedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2DOMContentLoadedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2DOMContentLoadedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreWebView2DOMContentLoadedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2DOMContentLoadedEventArgs {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CoreWebView2DevToolsProtocolEventReceivedEventArgs(pub ::windows::core::IInspectable);
impl CoreWebView2DevToolsProtocolEventReceivedEventArgs {
    pub fn ParameterObjectAsJson(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
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
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2DevToolsProtocolEventReceivedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2DevToolsProtocolEventReceivedEventArgs;{b6a4b41d-fd18-59fa-923a-c57555d960ce})" ) ;
}
unsafe impl ::windows::core::Interface for CoreWebView2DevToolsProtocolEventReceivedEventArgs {
    type Vtable = ICoreWebView2DevToolsProtocolEventReceivedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xb6a4b41d_fd18_59fa_923a_c57555d960ce);
}
impl ::windows::core::RuntimeName for CoreWebView2DevToolsProtocolEventReceivedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2DevToolsProtocolEventReceivedEventArgs";
}
impl ::core::convert::From<CoreWebView2DevToolsProtocolEventReceivedEventArgs>
    for ::windows::core::IUnknown
{
    fn from(value: CoreWebView2DevToolsProtocolEventReceivedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWebView2DevToolsProtocolEventReceivedEventArgs>
    for ::windows::core::IUnknown
{
    fn from(value: &CoreWebView2DevToolsProtocolEventReceivedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2DevToolsProtocolEventReceivedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2DevToolsProtocolEventReceivedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWebView2DevToolsProtocolEventReceivedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: CoreWebView2DevToolsProtocolEventReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWebView2DevToolsProtocolEventReceivedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: &CoreWebView2DevToolsProtocolEventReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2DevToolsProtocolEventReceivedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2DevToolsProtocolEventReceivedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreWebView2DevToolsProtocolEventReceivedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2DevToolsProtocolEventReceivedEventArgs {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CoreWebView2DevToolsProtocolEventReceiver(pub ::windows::core::IInspectable);
impl CoreWebView2DevToolsProtocolEventReceiver {
    pub fn DevToolsProtocolEventReceived<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2DevToolsProtocolEventReceivedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDevToolsProtocolEventReceived<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2DevToolsProtocolEventReceiver {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2DevToolsProtocolEventReceiver;{b2a2be79-65fc-5537-8715-3d92bf31090b})" ) ;
}
unsafe impl ::windows::core::Interface for CoreWebView2DevToolsProtocolEventReceiver {
    type Vtable = ICoreWebView2DevToolsProtocolEventReceiver_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xb2a2be79_65fc_5537_8715_3d92bf31090b);
}
impl ::windows::core::RuntimeName for CoreWebView2DevToolsProtocolEventReceiver {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2DevToolsProtocolEventReceiver";
}
impl ::core::convert::From<CoreWebView2DevToolsProtocolEventReceiver>
    for ::windows::core::IUnknown
{
    fn from(value: CoreWebView2DevToolsProtocolEventReceiver) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWebView2DevToolsProtocolEventReceiver>
    for ::windows::core::IUnknown
{
    fn from(value: &CoreWebView2DevToolsProtocolEventReceiver) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2DevToolsProtocolEventReceiver
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2DevToolsProtocolEventReceiver
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWebView2DevToolsProtocolEventReceiver>
    for ::windows::core::IInspectable
{
    fn from(value: CoreWebView2DevToolsProtocolEventReceiver) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWebView2DevToolsProtocolEventReceiver>
    for ::windows::core::IInspectable
{
    fn from(value: &CoreWebView2DevToolsProtocolEventReceiver) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2DevToolsProtocolEventReceiver
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2DevToolsProtocolEventReceiver
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreWebView2DevToolsProtocolEventReceiver {}
unsafe impl ::core::marker::Sync for CoreWebView2DevToolsProtocolEventReceiver {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CoreWebView2DownloadInterruptReason(pub i32);
impl CoreWebView2DownloadInterruptReason {
    pub const None: CoreWebView2DownloadInterruptReason = CoreWebView2DownloadInterruptReason(0i32);
    pub const FileFailed: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(1i32);
    pub const FileAccessDenied: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(2i32);
    pub const FileNoSpace: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(3i32);
    pub const FileNameTooLong: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(4i32);
    pub const FileTooLarge: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(5i32);
    pub const FileMalicious: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(6i32);
    pub const FileTransientError: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(7i32);
    pub const FileBlockedByPolicy: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(8i32);
    pub const FileSecurityCheckFailed: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(9i32);
    pub const FileTooShort: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(10i32);
    pub const FileHashMismatch: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(11i32);
    pub const NetworkFailed: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(12i32);
    pub const NetworkTimeout: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(13i32);
    pub const NetworkDisconnected: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(14i32);
    pub const NetworkServerDown: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(15i32);
    pub const NetworkInvalidRequest: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(16i32);
    pub const ServerFailed: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(17i32);
    pub const ServerNoRange: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(18i32);
    pub const ServerBadContent: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(19i32);
    pub const ServerUnauthorized: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(20i32);
    pub const ServerCertificateProblem: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(21i32);
    pub const ServerForbidden: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(22i32);
    pub const ServerUnexpectedResponse: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(23i32);
    pub const ServerContentLengthMismatch: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(24i32);
    pub const ServerCrossOriginRedirect: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(25i32);
    pub const UserCanceled: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(26i32);
    pub const UserShutdown: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(27i32);
    pub const UserPaused: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(28i32);
    pub const DownloadProcessCrashed: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(29i32);
}
impl ::core::convert::From<i32> for CoreWebView2DownloadInterruptReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CoreWebView2DownloadInterruptReason {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2DownloadInterruptReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2DownloadInterruptReason;i4)",
    );
}
impl ::windows::core::DefaultType for CoreWebView2DownloadInterruptReason {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CoreWebView2DownloadOperation(pub ::windows::core::IInspectable);
impl CoreWebView2DownloadOperation {
    pub fn Uri(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
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
    pub fn ContentDisposition(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
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
    pub fn MimeType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
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
    pub fn TotalBytesToReceive(&self) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    pub fn BytesReceived(&self) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    pub fn EstimatedEndTime(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ResultFilePath(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
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
    pub fn State(&self) -> ::windows::core::Result<CoreWebView2DownloadState> {
        let this = self;
        unsafe {
            let mut result__: CoreWebView2DownloadState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2DownloadState>(result__)
        }
    }
    pub fn InterruptReason(&self) -> ::windows::core::Result<CoreWebView2DownloadInterruptReason> {
        let this = self;
        unsafe {
            let mut result__: CoreWebView2DownloadInterruptReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2DownloadInterruptReason>(result__)
        }
    }
    pub fn CanResume(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn BytesReceivedChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2DownloadOperation,
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
            (::windows::core::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveBytesReceivedChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn EstimatedEndTimeChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2DownloadOperation,
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
            (::windows::core::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveEstimatedEndTimeChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StateChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2DownloadOperation,
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
            (::windows::core::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveStateChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Cancel(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Pause(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Resume(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2DownloadOperation {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2DownloadOperation;{afe73e6b-e760-5a06-9bf6-1e743c13cd2d})" ) ;
}
unsafe impl ::windows::core::Interface for CoreWebView2DownloadOperation {
    type Vtable = ICoreWebView2DownloadOperation_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xafe73e6b_e760_5a06_9bf6_1e743c13cd2d);
}
impl ::windows::core::RuntimeName for CoreWebView2DownloadOperation {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2DownloadOperation";
}
impl ::core::convert::From<CoreWebView2DownloadOperation> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2DownloadOperation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWebView2DownloadOperation> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2DownloadOperation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2DownloadOperation
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2DownloadOperation
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWebView2DownloadOperation> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2DownloadOperation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWebView2DownloadOperation> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2DownloadOperation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2DownloadOperation
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2DownloadOperation
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreWebView2DownloadOperation {}
unsafe impl ::core::marker::Sync for CoreWebView2DownloadOperation {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CoreWebView2DownloadStartingEventArgs(pub ::windows::core::IInspectable);
impl CoreWebView2DownloadStartingEventArgs {
    pub fn DownloadOperation(&self) -> ::windows::core::Result<CoreWebView2DownloadOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2DownloadOperation>(result__)
        }
    }
    pub fn Cancel(&self) -> ::windows::core::Result<bool> {
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
    pub fn SetCancel(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn ResultFilePath(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
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
    pub fn SetResultFilePath<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
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
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn GetDeferral(&self) -> ::windows::core::Result<::windows::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2DownloadStartingEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2DownloadStartingEventArgs;{45d982ba-9256-5b35-b023-26a438599110})" ) ;
}
unsafe impl ::windows::core::Interface for CoreWebView2DownloadStartingEventArgs {
    type Vtable = ICoreWebView2DownloadStartingEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x45d982ba_9256_5b35_b023_26a438599110);
}
impl ::windows::core::RuntimeName for CoreWebView2DownloadStartingEventArgs {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2DownloadStartingEventArgs";
}
impl ::core::convert::From<CoreWebView2DownloadStartingEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2DownloadStartingEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWebView2DownloadStartingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2DownloadStartingEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2DownloadStartingEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2DownloadStartingEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWebView2DownloadStartingEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: CoreWebView2DownloadStartingEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWebView2DownloadStartingEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: &CoreWebView2DownloadStartingEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2DownloadStartingEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2DownloadStartingEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreWebView2DownloadStartingEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2DownloadStartingEventArgs {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CoreWebView2DownloadState(pub i32);
impl CoreWebView2DownloadState {
    pub const InProgress: CoreWebView2DownloadState = CoreWebView2DownloadState(0i32);
    pub const Interrupted: CoreWebView2DownloadState = CoreWebView2DownloadState(1i32);
    pub const Completed: CoreWebView2DownloadState = CoreWebView2DownloadState(2i32);
}
impl ::core::convert::From<i32> for CoreWebView2DownloadState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CoreWebView2DownloadState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2DownloadState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2DownloadState;i4)",
    );
}
impl ::windows::core::DefaultType for CoreWebView2DownloadState {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CoreWebView2Environment(pub ::windows::core::IInspectable);
impl CoreWebView2Environment {
    pub fn BrowserVersionString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
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
    pub fn NewBrowserVersionAvailable<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2Environment,
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
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveNewBrowserVersionAvailable<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn CreateCoreWebView2ControllerAsync<
        'a,
        Param0: ::windows::core::IntoParam<'a, CoreWebView2ControllerWindowReference>,
    >(
        &self,
        parentwindow: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncOperation<CoreWebView2Controller>>
    {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                parentwindow.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IAsyncOperation<CoreWebView2Controller>>(result__)
        }
    }
    pub fn CreateWebResourceResponse<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Storage::Streams::IRandomAccessStream>,
        Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        content: Param0,
        statuscode: i32,
        reasonphrase: Param2,
        headers: Param3,
    ) -> ::windows::core::Result<CoreWebView2WebResourceResponse> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                content.into_param().abi(),
                statuscode,
                reasonphrase.into_param().abi(),
                headers.into_param().abi(),
                &mut result__,
            )
            .from_abi::<CoreWebView2WebResourceResponse>(result__)
        }
    }
    pub fn CreateWebResourceRequest<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param2: ::windows::core::IntoParam<'a, ::windows::Storage::Streams::IRandomAccessStream>,
        Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        uri: Param0,
        method: Param1,
        postdata: Param2,
        headers: Param3,
    ) -> ::windows::core::Result<CoreWebView2WebResourceRequest> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Environment2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                uri.into_param().abi(),
                method.into_param().abi(),
                postdata.into_param().abi(),
                headers.into_param().abi(),
                &mut result__,
            )
            .from_abi::<CoreWebView2WebResourceRequest>(result__)
        }
    }
    pub fn CreateCoreWebView2CompositionControllerAsync<
        'a,
        Param0: ::windows::core::IntoParam<'a, CoreWebView2ControllerWindowReference>,
    >(
        &self,
        parentwindow: Param0,
    ) -> ::windows::core::Result<
        ::windows::Foundation::IAsyncOperation<CoreWebView2CompositionController>,
    > {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Environment3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                parentwindow.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IAsyncOperation<CoreWebView2CompositionController>>(
                result__,
            )
        }
    }
    pub fn CreateCoreWebView2PointerInfo(
        &self,
    ) -> ::windows::core::Result<CoreWebView2PointerInfo> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Environment3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2PointerInfo>(result__)
        }
    }
    pub fn BrowserProcessExited<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2Environment,
                CoreWebView2BrowserProcessExitedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Environment5>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveBrowserProcessExited<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Environment5>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn CreatePrintSettings(&self) -> ::windows::core::Result<CoreWebView2PrintSettings> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Environment6>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2PrintSettings>(result__)
        }
    }
    pub fn CreateAsync(
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncOperation<CoreWebView2Environment>>
    {
        Self::ICoreWebView2EnvironmentStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IAsyncOperation<CoreWebView2Environment>>(result__)
        })
    }
    pub fn CreateWithOptionsAsync<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param2: ::windows::core::IntoParam<'a, CoreWebView2EnvironmentOptions>,
    >(
        browserexecutablefolder: Param0,
        userdatafolder: Param1,
        options: Param2,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncOperation<CoreWebView2Environment>>
    {
        Self::ICoreWebView2EnvironmentStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                browserexecutablefolder.into_param().abi(),
                userdatafolder.into_param().abi(),
                options.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IAsyncOperation<CoreWebView2Environment>>(result__)
        })
    }
    pub fn GetAvailableBrowserVersionString() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICoreWebView2EnvironmentStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn GetAvailableBrowserVersionString2<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        browserexecutablefolder: Param0,
    ) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICoreWebView2EnvironmentStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                browserexecutablefolder.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn CompareBrowserVersionString<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        browserversionstring1: Param0,
        browserversionstring2: Param1,
    ) -> ::windows::core::Result<i32> {
        Self::ICoreWebView2EnvironmentStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                browserversionstring1.into_param().abi(),
                browserversionstring2.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        })
    }
    pub fn ICoreWebView2EnvironmentStatics<
        R,
        F: FnOnce(&ICoreWebView2EnvironmentStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            CoreWebView2Environment,
            ICoreWebView2EnvironmentStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2Environment {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2Environment;{d8cc7831-b783-556b-b9ce-899c1e95d585})" ) ;
}
unsafe impl ::windows::core::Interface for CoreWebView2Environment {
    type Vtable = ICoreWebView2Environment_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xd8cc7831_b783_556b_b9ce_899c1e95d585);
}
impl ::windows::core::RuntimeName for CoreWebView2Environment {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2Environment";
}
impl ::core::convert::From<CoreWebView2Environment> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2Environment) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWebView2Environment> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2Environment) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreWebView2Environment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreWebView2Environment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWebView2Environment> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2Environment) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWebView2Environment> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2Environment) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreWebView2Environment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2Environment
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreWebView2Environment {}
unsafe impl ::core::marker::Sync for CoreWebView2Environment {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CoreWebView2EnvironmentOptions(pub ::windows::core::IInspectable);
impl CoreWebView2EnvironmentOptions {
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
            CoreWebView2EnvironmentOptions,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn AdditionalBrowserArguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
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
    pub fn SetAdditionalBrowserArguments<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
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
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
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
    pub fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
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
    pub fn TargetCompatibleBrowserVersion(
        &self,
    ) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetTargetCompatibleBrowserVersion<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
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
    pub fn AllowSingleSignOnUsingOSPrimaryAccount(&self) -> ::windows::core::Result<bool> {
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
    pub fn SetAllowSingleSignOnUsingOSPrimaryAccount(
        &self,
        value: bool,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2EnvironmentOptions {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2EnvironmentOptions;{25d6dc39-0062-5735-8b09-a6f535f19e97})" ) ;
}
unsafe impl ::windows::core::Interface for CoreWebView2EnvironmentOptions {
    type Vtable = ICoreWebView2EnvironmentOptions_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x25d6dc39_0062_5735_8b09_a6f535f19e97);
}
impl ::windows::core::RuntimeName for CoreWebView2EnvironmentOptions {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2EnvironmentOptions";
}
impl ::core::convert::From<CoreWebView2EnvironmentOptions> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2EnvironmentOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWebView2EnvironmentOptions> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2EnvironmentOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2EnvironmentOptions
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2EnvironmentOptions
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWebView2EnvironmentOptions> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2EnvironmentOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWebView2EnvironmentOptions> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2EnvironmentOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2EnvironmentOptions
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2EnvironmentOptions
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreWebView2EnvironmentOptions {}
unsafe impl ::core::marker::Sync for CoreWebView2EnvironmentOptions {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CoreWebView2Frame(pub ::windows::core::IInspectable);
impl CoreWebView2Frame {
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
    pub fn NameChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2Frame,
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
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveNameChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Destroyed<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2Frame,
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
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDestroyed<
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
    pub fn RemoveHostObjectFromScript<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IsDestroyed(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2Frame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Web.WebView2.Core.CoreWebView2Frame;{02ffcbf9-19e7-5bb8-8273-346420fb1503})",
    );
}
unsafe impl ::windows::core::Interface for CoreWebView2Frame {
    type Vtable = ICoreWebView2Frame_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x02ffcbf9_19e7_5bb8_8273_346420fb1503);
}
impl ::windows::core::RuntimeName for CoreWebView2Frame {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2Frame";
}
impl ::core::convert::From<CoreWebView2Frame> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2Frame) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWebView2Frame> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2Frame) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreWebView2Frame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreWebView2Frame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWebView2Frame> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2Frame) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWebView2Frame> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2Frame) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreWebView2Frame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreWebView2Frame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreWebView2Frame {}
unsafe impl ::core::marker::Sync for CoreWebView2Frame {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CoreWebView2FrameCreatedEventArgs(pub ::windows::core::IInspectable);
impl CoreWebView2FrameCreatedEventArgs {
    pub fn Frame(&self) -> ::windows::core::Result<CoreWebView2Frame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2Frame>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2FrameCreatedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2FrameCreatedEventArgs;{527b01b8-fc6d-5543-8dce-96cdfdb32c81})" ) ;
}
unsafe impl ::windows::core::Interface for CoreWebView2FrameCreatedEventArgs {
    type Vtable = ICoreWebView2FrameCreatedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x527b01b8_fc6d_5543_8dce_96cdfdb32c81);
}
impl ::windows::core::RuntimeName for CoreWebView2FrameCreatedEventArgs {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2FrameCreatedEventArgs";
}
impl ::core::convert::From<CoreWebView2FrameCreatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2FrameCreatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWebView2FrameCreatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2FrameCreatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2FrameCreatedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2FrameCreatedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWebView2FrameCreatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2FrameCreatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWebView2FrameCreatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2FrameCreatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2FrameCreatedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2FrameCreatedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreWebView2FrameCreatedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2FrameCreatedEventArgs {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CoreWebView2FrameInfo(pub ::windows::core::IInspectable);
impl CoreWebView2FrameInfo {
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
    pub fn Source(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
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
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2FrameInfo {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2FrameInfo;{f9b82e06-73f3-513b-bc2c-445ddedba976})" ) ;
}
unsafe impl ::windows::core::Interface for CoreWebView2FrameInfo {
    type Vtable = ICoreWebView2FrameInfo_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xf9b82e06_73f3_513b_bc2c_445ddedba976);
}
impl ::windows::core::RuntimeName for CoreWebView2FrameInfo {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2FrameInfo";
}
impl ::core::convert::From<CoreWebView2FrameInfo> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2FrameInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWebView2FrameInfo> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2FrameInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreWebView2FrameInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreWebView2FrameInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWebView2FrameInfo> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2FrameInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWebView2FrameInfo> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2FrameInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreWebView2FrameInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2FrameInfo
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreWebView2FrameInfo {}
unsafe impl ::core::marker::Sync for CoreWebView2FrameInfo {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CoreWebView2HostResourceAccessKind(pub i32);
impl CoreWebView2HostResourceAccessKind {
    pub const Deny: CoreWebView2HostResourceAccessKind = CoreWebView2HostResourceAccessKind(0i32);
    pub const Allow: CoreWebView2HostResourceAccessKind = CoreWebView2HostResourceAccessKind(1i32);
    pub const DenyCors: CoreWebView2HostResourceAccessKind =
        CoreWebView2HostResourceAccessKind(2i32);
}
impl ::core::convert::From<i32> for CoreWebView2HostResourceAccessKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CoreWebView2HostResourceAccessKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2HostResourceAccessKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2HostResourceAccessKind;i4)",
    );
}
impl ::windows::core::DefaultType for CoreWebView2HostResourceAccessKind {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CoreWebView2HttpHeadersCollectionIterator(pub ::windows::core::IInspectable);
impl CoreWebView2HttpHeadersCollectionIterator {
    pub fn Current(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IKeyValuePair<
            ::windows::core::HSTRING,
            ::windows::core::HSTRING,
        >,
    > {
        let this = &::windows::core::Interface::cast::<
            ::windows::Foundation::Collections::IIterator<
                ::windows::Foundation::Collections::IKeyValuePair<
                    ::windows::core::HSTRING,
                    ::windows::core::HSTRING,
                >,
            >,
        >(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IKeyValuePair<
                ::windows::core::HSTRING,
                ::windows::core::HSTRING,
            >>(result__)
        }
    }
    pub fn HasCurrent(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<
            ::windows::Foundation::Collections::IIterator<
                ::windows::Foundation::Collections::IKeyValuePair<
                    ::windows::core::HSTRING,
                    ::windows::core::HSTRING,
                >,
            >,
        >(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn MoveNext(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<
            ::windows::Foundation::Collections::IIterator<
                ::windows::Foundation::Collections::IKeyValuePair<
                    ::windows::core::HSTRING,
                    ::windows::core::HSTRING,
                >,
            >,
        >(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn GetMany(
        &self,
        items: &mut [<::windows::Foundation::Collections::IKeyValuePair<
            ::windows::core::HSTRING,
            ::windows::core::HSTRING,
        > as ::windows::core::DefaultType>::DefaultType],
    ) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<
            ::windows::Foundation::Collections::IIterator<
                ::windows::Foundation::Collections::IKeyValuePair<
                    ::windows::core::HSTRING,
                    ::windows::core::HSTRING,
                >,
            >,
        >(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                items.len() as u32,
                ::core::mem::transmute_copy(&items),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2HttpHeadersCollectionIterator {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2HttpHeadersCollectionIterator;{adf264ee-d980-5f48-a60e-8705de046608})" ) ;
}
unsafe impl ::windows::core::Interface for CoreWebView2HttpHeadersCollectionIterator {
    type Vtable = ICoreWebView2HttpHeadersCollectionIterator_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xadf264ee_d980_5f48_a60e_8705de046608);
}
impl ::windows::core::RuntimeName for CoreWebView2HttpHeadersCollectionIterator {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2HttpHeadersCollectionIterator";
}
impl ::core::convert::From<CoreWebView2HttpHeadersCollectionIterator>
    for ::windows::core::IUnknown
{
    fn from(value: CoreWebView2HttpHeadersCollectionIterator) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWebView2HttpHeadersCollectionIterator>
    for ::windows::core::IUnknown
{
    fn from(value: &CoreWebView2HttpHeadersCollectionIterator) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2HttpHeadersCollectionIterator
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2HttpHeadersCollectionIterator
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWebView2HttpHeadersCollectionIterator>
    for ::windows::core::IInspectable
{
    fn from(value: CoreWebView2HttpHeadersCollectionIterator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWebView2HttpHeadersCollectionIterator>
    for ::windows::core::IInspectable
{
    fn from(value: &CoreWebView2HttpHeadersCollectionIterator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2HttpHeadersCollectionIterator
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2HttpHeadersCollectionIterator
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<CoreWebView2HttpHeadersCollectionIterator>
    for ::windows::Foundation::Collections::IIterator<
        ::windows::Foundation::Collections::IKeyValuePair<
            ::windows::core::HSTRING,
            ::windows::core::HSTRING,
        >,
    >
{
    type Error = ::windows::core::Error;
    fn try_from(value: CoreWebView2HttpHeadersCollectionIterator) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CoreWebView2HttpHeadersCollectionIterator>
    for ::windows::Foundation::Collections::IIterator<
        ::windows::Foundation::Collections::IKeyValuePair<
            ::windows::core::HSTRING,
            ::windows::core::HSTRING,
        >,
    >
{
    type Error = ::windows::core::Error;
    fn try_from(
        value: &CoreWebView2HttpHeadersCollectionIterator,
    ) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a>
    ::windows::core::IntoParam<
        'a,
        ::windows::Foundation::Collections::IIterator<
            ::windows::Foundation::Collections::IKeyValuePair<
                ::windows::core::HSTRING,
                ::windows::core::HSTRING,
            >,
        >,
    > for CoreWebView2HttpHeadersCollectionIterator
{
    fn into_param(
        self,
    ) -> ::windows::core::Param<
        'a,
        ::windows::Foundation::Collections::IIterator<
            ::windows::Foundation::Collections::IKeyValuePair<
                ::windows::core::HSTRING,
                ::windows::core::HSTRING,
            >,
        >,
    > {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a>
    ::windows::core::IntoParam<
        'a,
        ::windows::Foundation::Collections::IIterator<
            ::windows::Foundation::Collections::IKeyValuePair<
                ::windows::core::HSTRING,
                ::windows::core::HSTRING,
            >,
        >,
    > for &CoreWebView2HttpHeadersCollectionIterator
{
    fn into_param(
        self,
    ) -> ::windows::core::Param<
        'a,
        ::windows::Foundation::Collections::IIterator<
            ::windows::Foundation::Collections::IKeyValuePair<
                ::windows::core::HSTRING,
                ::windows::core::HSTRING,
            >,
        >,
    > {
        ::core::convert::TryInto::<
            ::windows::Foundation::Collections::IIterator<
                ::windows::Foundation::Collections::IKeyValuePair<
                    ::windows::core::HSTRING,
                    ::windows::core::HSTRING,
                >,
            >,
        >::try_into(self)
        .map(::windows::core::Param::Owned)
        .unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for CoreWebView2HttpHeadersCollectionIterator {}
unsafe impl ::core::marker::Sync for CoreWebView2HttpHeadersCollectionIterator {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CoreWebView2HttpRequestHeaders(pub ::windows::core::IInspectable);
impl CoreWebView2HttpRequestHeaders {
    pub fn GetHeader<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn GetHeaders<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<CoreWebView2HttpHeadersCollectionIterator> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<CoreWebView2HttpHeadersCollectionIterator>(result__)
        }
    }
    pub fn Contains<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetHeader<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        name: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveHeader<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn First(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IIterator<
            ::windows::Foundation::Collections::IKeyValuePair<
                ::windows::core::HSTRING,
                ::windows::core::HSTRING,
            >,
        >,
    > {
        let this = &::windows::core::Interface::cast::<
            ::windows::Foundation::Collections::IIterable<
                ::windows::Foundation::Collections::IKeyValuePair<
                    ::windows::core::HSTRING,
                    ::windows::core::HSTRING,
                >,
            >,
        >(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IIterator<
                ::windows::Foundation::Collections::IKeyValuePair<
                    ::windows::core::HSTRING,
                    ::windows::core::HSTRING,
                >,
            >>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2HttpRequestHeaders {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2HttpRequestHeaders;{dc2226c7-3515-55bb-bcb2-57b78f86b91d})" ) ;
}
unsafe impl ::windows::core::Interface for CoreWebView2HttpRequestHeaders {
    type Vtable = ICoreWebView2HttpRequestHeaders_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xdc2226c7_3515_55bb_bcb2_57b78f86b91d);
}
impl ::windows::core::RuntimeName for CoreWebView2HttpRequestHeaders {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2HttpRequestHeaders";
}
impl ::core::convert::From<CoreWebView2HttpRequestHeaders> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2HttpRequestHeaders) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWebView2HttpRequestHeaders> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2HttpRequestHeaders) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2HttpRequestHeaders
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2HttpRequestHeaders
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWebView2HttpRequestHeaders> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2HttpRequestHeaders) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWebView2HttpRequestHeaders> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2HttpRequestHeaders) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2HttpRequestHeaders
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2HttpRequestHeaders
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<CoreWebView2HttpRequestHeaders>
    for ::windows::Foundation::Collections::IIterable<
        ::windows::Foundation::Collections::IKeyValuePair<
            ::windows::core::HSTRING,
            ::windows::core::HSTRING,
        >,
    >
{
    type Error = ::windows::core::Error;
    fn try_from(value: CoreWebView2HttpRequestHeaders) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CoreWebView2HttpRequestHeaders>
    for ::windows::Foundation::Collections::IIterable<
        ::windows::Foundation::Collections::IKeyValuePair<
            ::windows::core::HSTRING,
            ::windows::core::HSTRING,
        >,
    >
{
    type Error = ::windows::core::Error;
    fn try_from(value: &CoreWebView2HttpRequestHeaders) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a>
    ::windows::core::IntoParam<
        'a,
        ::windows::Foundation::Collections::IIterable<
            ::windows::Foundation::Collections::IKeyValuePair<
                ::windows::core::HSTRING,
                ::windows::core::HSTRING,
            >,
        >,
    > for CoreWebView2HttpRequestHeaders
{
    fn into_param(
        self,
    ) -> ::windows::core::Param<
        'a,
        ::windows::Foundation::Collections::IIterable<
            ::windows::Foundation::Collections::IKeyValuePair<
                ::windows::core::HSTRING,
                ::windows::core::HSTRING,
            >,
        >,
    > {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a>
    ::windows::core::IntoParam<
        'a,
        ::windows::Foundation::Collections::IIterable<
            ::windows::Foundation::Collections::IKeyValuePair<
                ::windows::core::HSTRING,
                ::windows::core::HSTRING,
            >,
        >,
    > for &CoreWebView2HttpRequestHeaders
{
    fn into_param(
        self,
    ) -> ::windows::core::Param<
        'a,
        ::windows::Foundation::Collections::IIterable<
            ::windows::Foundation::Collections::IKeyValuePair<
                ::windows::core::HSTRING,
                ::windows::core::HSTRING,
            >,
        >,
    > {
        ::core::convert::TryInto::<
            ::windows::Foundation::Collections::IIterable<
                ::windows::Foundation::Collections::IKeyValuePair<
                    ::windows::core::HSTRING,
                    ::windows::core::HSTRING,
                >,
            >,
        >::try_into(self)
        .map(::windows::core::Param::Owned)
        .unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for CoreWebView2HttpRequestHeaders {}
unsafe impl ::core::marker::Sync for CoreWebView2HttpRequestHeaders {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for CoreWebView2HttpRequestHeaders {
    type Item = ::windows::Foundation::Collections::IKeyValuePair<
        ::windows::core::HSTRING,
        ::windows::core::HSTRING,
    >;
    type IntoIter = ::windows::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &CoreWebView2HttpRequestHeaders {
    type Item = ::windows::Foundation::Collections::IKeyValuePair<
        ::windows::core::HSTRING,
        ::windows::core::HSTRING,
    >;
    type IntoIter = ::windows::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CoreWebView2HttpResponseHeaders(pub ::windows::core::IInspectable);
impl CoreWebView2HttpResponseHeaders {
    pub fn AppendHeader<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        name: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Contains<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn GetHeader<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn GetHeaders<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<CoreWebView2HttpHeadersCollectionIterator> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<CoreWebView2HttpHeadersCollectionIterator>(result__)
        }
    }
    pub fn First(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IIterator<
            ::windows::Foundation::Collections::IKeyValuePair<
                ::windows::core::HSTRING,
                ::windows::core::HSTRING,
            >,
        >,
    > {
        let this = &::windows::core::Interface::cast::<
            ::windows::Foundation::Collections::IIterable<
                ::windows::Foundation::Collections::IKeyValuePair<
                    ::windows::core::HSTRING,
                    ::windows::core::HSTRING,
                >,
            >,
        >(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IIterator<
                ::windows::Foundation::Collections::IKeyValuePair<
                    ::windows::core::HSTRING,
                    ::windows::core::HSTRING,
                >,
            >>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2HttpResponseHeaders {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2HttpResponseHeaders;{f3d383e9-747f-5574-8662-9a6b920cecd4})" ) ;
}
unsafe impl ::windows::core::Interface for CoreWebView2HttpResponseHeaders {
    type Vtable = ICoreWebView2HttpResponseHeaders_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xf3d383e9_747f_5574_8662_9a6b920cecd4);
}
impl ::windows::core::RuntimeName for CoreWebView2HttpResponseHeaders {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2HttpResponseHeaders";
}
impl ::core::convert::From<CoreWebView2HttpResponseHeaders> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2HttpResponseHeaders) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWebView2HttpResponseHeaders> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2HttpResponseHeaders) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2HttpResponseHeaders
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2HttpResponseHeaders
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWebView2HttpResponseHeaders> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2HttpResponseHeaders) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWebView2HttpResponseHeaders> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2HttpResponseHeaders) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2HttpResponseHeaders
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2HttpResponseHeaders
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<CoreWebView2HttpResponseHeaders>
    for ::windows::Foundation::Collections::IIterable<
        ::windows::Foundation::Collections::IKeyValuePair<
            ::windows::core::HSTRING,
            ::windows::core::HSTRING,
        >,
    >
{
    type Error = ::windows::core::Error;
    fn try_from(value: CoreWebView2HttpResponseHeaders) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CoreWebView2HttpResponseHeaders>
    for ::windows::Foundation::Collections::IIterable<
        ::windows::Foundation::Collections::IKeyValuePair<
            ::windows::core::HSTRING,
            ::windows::core::HSTRING,
        >,
    >
{
    type Error = ::windows::core::Error;
    fn try_from(value: &CoreWebView2HttpResponseHeaders) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a>
    ::windows::core::IntoParam<
        'a,
        ::windows::Foundation::Collections::IIterable<
            ::windows::Foundation::Collections::IKeyValuePair<
                ::windows::core::HSTRING,
                ::windows::core::HSTRING,
            >,
        >,
    > for CoreWebView2HttpResponseHeaders
{
    fn into_param(
        self,
    ) -> ::windows::core::Param<
        'a,
        ::windows::Foundation::Collections::IIterable<
            ::windows::Foundation::Collections::IKeyValuePair<
                ::windows::core::HSTRING,
                ::windows::core::HSTRING,
            >,
        >,
    > {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a>
    ::windows::core::IntoParam<
        'a,
        ::windows::Foundation::Collections::IIterable<
            ::windows::Foundation::Collections::IKeyValuePair<
                ::windows::core::HSTRING,
                ::windows::core::HSTRING,
            >,
        >,
    > for &CoreWebView2HttpResponseHeaders
{
    fn into_param(
        self,
    ) -> ::windows::core::Param<
        'a,
        ::windows::Foundation::Collections::IIterable<
            ::windows::Foundation::Collections::IKeyValuePair<
                ::windows::core::HSTRING,
                ::windows::core::HSTRING,
            >,
        >,
    > {
        ::core::convert::TryInto::<
            ::windows::Foundation::Collections::IIterable<
                ::windows::Foundation::Collections::IKeyValuePair<
                    ::windows::core::HSTRING,
                    ::windows::core::HSTRING,
                >,
            >,
        >::try_into(self)
        .map(::windows::core::Param::Owned)
        .unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for CoreWebView2HttpResponseHeaders {}
unsafe impl ::core::marker::Sync for CoreWebView2HttpResponseHeaders {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for CoreWebView2HttpResponseHeaders {
    type Item = ::windows::Foundation::Collections::IKeyValuePair<
        ::windows::core::HSTRING,
        ::windows::core::HSTRING,
    >;
    type IntoIter = ::windows::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &CoreWebView2HttpResponseHeaders {
    type Item = ::windows::Foundation::Collections::IKeyValuePair<
        ::windows::core::HSTRING,
        ::windows::core::HSTRING,
    >;
    type IntoIter = ::windows::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
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
pub struct CoreWebView2KeyEventKind(pub i32);
impl CoreWebView2KeyEventKind {
    pub const KeyDown: CoreWebView2KeyEventKind = CoreWebView2KeyEventKind(0i32);
    pub const KeyUp: CoreWebView2KeyEventKind = CoreWebView2KeyEventKind(1i32);
    pub const SystemKeyDown: CoreWebView2KeyEventKind = CoreWebView2KeyEventKind(2i32);
    pub const SystemKeyUp: CoreWebView2KeyEventKind = CoreWebView2KeyEventKind(3i32);
}
impl ::core::convert::From<i32> for CoreWebView2KeyEventKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CoreWebView2KeyEventKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2KeyEventKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2KeyEventKind;i4)",
    );
}
impl ::windows::core::DefaultType for CoreWebView2KeyEventKind {
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
pub struct CoreWebView2MouseEventKind(pub i32);
impl CoreWebView2MouseEventKind {
    pub const HorizontalWheel: CoreWebView2MouseEventKind = CoreWebView2MouseEventKind(526i32);
    pub const LeftButtonDoubleClick: CoreWebView2MouseEventKind =
        CoreWebView2MouseEventKind(515i32);
    pub const LeftButtonDown: CoreWebView2MouseEventKind = CoreWebView2MouseEventKind(513i32);
    pub const LeftButtonUp: CoreWebView2MouseEventKind = CoreWebView2MouseEventKind(514i32);
    pub const Leave: CoreWebView2MouseEventKind = CoreWebView2MouseEventKind(675i32);
    pub const MiddleButtonDoubleClick: CoreWebView2MouseEventKind =
        CoreWebView2MouseEventKind(521i32);
    pub const MiddleButtonDown: CoreWebView2MouseEventKind = CoreWebView2MouseEventKind(519i32);
    pub const MiddleButtonUp: CoreWebView2MouseEventKind = CoreWebView2MouseEventKind(520i32);
    pub const Move: CoreWebView2MouseEventKind = CoreWebView2MouseEventKind(512i32);
    pub const RightButtonDoubleClick: CoreWebView2MouseEventKind =
        CoreWebView2MouseEventKind(518i32);
    pub const RightButtonDown: CoreWebView2MouseEventKind = CoreWebView2MouseEventKind(516i32);
    pub const RightButtonUp: CoreWebView2MouseEventKind = CoreWebView2MouseEventKind(517i32);
    pub const Wheel: CoreWebView2MouseEventKind = CoreWebView2MouseEventKind(522i32);
    pub const XButtonDoubleClick: CoreWebView2MouseEventKind = CoreWebView2MouseEventKind(525i32);
    pub const XButtonDown: CoreWebView2MouseEventKind = CoreWebView2MouseEventKind(523i32);
    pub const XButtonUp: CoreWebView2MouseEventKind = CoreWebView2MouseEventKind(524i32);
}
impl ::core::convert::From<i32> for CoreWebView2MouseEventKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CoreWebView2MouseEventKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2MouseEventKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2MouseEventKind;i4)",
    );
}
impl ::windows::core::DefaultType for CoreWebView2MouseEventKind {
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
pub struct CoreWebView2MouseEventVirtualKeys(pub u32);
impl CoreWebView2MouseEventVirtualKeys {
    pub const None: CoreWebView2MouseEventVirtualKeys = CoreWebView2MouseEventVirtualKeys(0u32);
    pub const LeftButton: CoreWebView2MouseEventVirtualKeys =
        CoreWebView2MouseEventVirtualKeys(1u32);
    pub const RightButton: CoreWebView2MouseEventVirtualKeys =
        CoreWebView2MouseEventVirtualKeys(2u32);
    pub const Shift: CoreWebView2MouseEventVirtualKeys = CoreWebView2MouseEventVirtualKeys(4u32);
    pub const Control: CoreWebView2MouseEventVirtualKeys = CoreWebView2MouseEventVirtualKeys(8u32);
    pub const MiddleButton: CoreWebView2MouseEventVirtualKeys =
        CoreWebView2MouseEventVirtualKeys(16u32);
    pub const XButton1: CoreWebView2MouseEventVirtualKeys =
        CoreWebView2MouseEventVirtualKeys(32u32);
    pub const XButton2: CoreWebView2MouseEventVirtualKeys =
        CoreWebView2MouseEventVirtualKeys(64u32);
}
impl ::core::convert::From<u32> for CoreWebView2MouseEventVirtualKeys {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CoreWebView2MouseEventVirtualKeys {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2MouseEventVirtualKeys {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2MouseEventVirtualKeys;u4)",
    );
}
impl ::windows::core::DefaultType for CoreWebView2MouseEventVirtualKeys {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for CoreWebView2MouseEventVirtualKeys {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for CoreWebView2MouseEventVirtualKeys {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for CoreWebView2MouseEventVirtualKeys {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for CoreWebView2MouseEventVirtualKeys {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for CoreWebView2MouseEventVirtualKeys {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
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
pub struct CoreWebView2MoveFocusReason(pub i32);
impl CoreWebView2MoveFocusReason {
    pub const Programmatic: CoreWebView2MoveFocusReason = CoreWebView2MoveFocusReason(0i32);
    pub const Next: CoreWebView2MoveFocusReason = CoreWebView2MoveFocusReason(1i32);
    pub const Previous: CoreWebView2MoveFocusReason = CoreWebView2MoveFocusReason(2i32);
}
impl ::core::convert::From<i32> for CoreWebView2MoveFocusReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CoreWebView2MoveFocusReason {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2MoveFocusReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2MoveFocusReason;i4)",
    );
}
impl ::windows::core::DefaultType for CoreWebView2MoveFocusReason {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CoreWebView2MoveFocusRequestedEventArgs(pub ::windows::core::IInspectable);
impl CoreWebView2MoveFocusRequestedEventArgs {
    pub fn Reason(&self) -> ::windows::core::Result<CoreWebView2MoveFocusReason> {
        let this = self;
        unsafe {
            let mut result__: CoreWebView2MoveFocusReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2MoveFocusReason>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
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
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2MoveFocusRequestedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2MoveFocusRequestedEventArgs;{2e29103b-ecdd-5c1d-b288-3f066d608919})" ) ;
}
unsafe impl ::windows::core::Interface for CoreWebView2MoveFocusRequestedEventArgs {
    type Vtable = ICoreWebView2MoveFocusRequestedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x2e29103b_ecdd_5c1d_b288_3f066d608919);
}
impl ::windows::core::RuntimeName for CoreWebView2MoveFocusRequestedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2MoveFocusRequestedEventArgs";
}
impl ::core::convert::From<CoreWebView2MoveFocusRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2MoveFocusRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWebView2MoveFocusRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2MoveFocusRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2MoveFocusRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2MoveFocusRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWebView2MoveFocusRequestedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: CoreWebView2MoveFocusRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWebView2MoveFocusRequestedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: &CoreWebView2MoveFocusRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2MoveFocusRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2MoveFocusRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreWebView2MoveFocusRequestedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2MoveFocusRequestedEventArgs {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CoreWebView2NavigationCompletedEventArgs(pub ::windows::core::IInspectable);
impl CoreWebView2NavigationCompletedEventArgs {
    pub fn IsSuccess(&self) -> ::windows::core::Result<bool> {
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
    pub fn WebErrorStatus(&self) -> ::windows::core::Result<CoreWebView2WebErrorStatus> {
        let this = self;
        unsafe {
            let mut result__: CoreWebView2WebErrorStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2WebErrorStatus>(result__)
        }
    }
    pub fn NavigationId(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u64>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2NavigationCompletedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2NavigationCompletedEventArgs;{4865e238-036a-5664-95a3-447ec44cf498})" ) ;
}
unsafe impl ::windows::core::Interface for CoreWebView2NavigationCompletedEventArgs {
    type Vtable = ICoreWebView2NavigationCompletedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4865e238_036a_5664_95a3_447ec44cf498);
}
impl ::windows::core::RuntimeName for CoreWebView2NavigationCompletedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2NavigationCompletedEventArgs";
}
impl ::core::convert::From<CoreWebView2NavigationCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2NavigationCompletedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWebView2NavigationCompletedEventArgs>
    for ::windows::core::IUnknown
{
    fn from(value: &CoreWebView2NavigationCompletedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2NavigationCompletedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2NavigationCompletedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWebView2NavigationCompletedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: CoreWebView2NavigationCompletedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWebView2NavigationCompletedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: &CoreWebView2NavigationCompletedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2NavigationCompletedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2NavigationCompletedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreWebView2NavigationCompletedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2NavigationCompletedEventArgs {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CoreWebView2NavigationStartingEventArgs(pub ::windows::core::IInspectable);
impl CoreWebView2NavigationStartingEventArgs {
    pub fn Uri(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
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
    pub fn IsUserInitiated(&self) -> ::windows::core::Result<bool> {
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
    pub fn IsRedirected(&self) -> ::windows::core::Result<bool> {
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
    pub fn RequestHeaders(&self) -> ::windows::core::Result<CoreWebView2HttpRequestHeaders> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2HttpRequestHeaders>(result__)
        }
    }
    pub fn Cancel(&self) -> ::windows::core::Result<bool> {
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
    pub fn SetCancel(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn NavigationId(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u64>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2NavigationStartingEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2NavigationStartingEventArgs;{548d23d3-fea3-5616-bd05-ae08066c86d3})" ) ;
}
unsafe impl ::windows::core::Interface for CoreWebView2NavigationStartingEventArgs {
    type Vtable = ICoreWebView2NavigationStartingEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x548d23d3_fea3_5616_bd05_ae08066c86d3);
}
impl ::windows::core::RuntimeName for CoreWebView2NavigationStartingEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2NavigationStartingEventArgs";
}
impl ::core::convert::From<CoreWebView2NavigationStartingEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2NavigationStartingEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWebView2NavigationStartingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2NavigationStartingEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2NavigationStartingEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2NavigationStartingEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWebView2NavigationStartingEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: CoreWebView2NavigationStartingEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWebView2NavigationStartingEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: &CoreWebView2NavigationStartingEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2NavigationStartingEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2NavigationStartingEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreWebView2NavigationStartingEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2NavigationStartingEventArgs {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CoreWebView2NewWindowRequestedEventArgs(pub ::windows::core::IInspectable);
impl CoreWebView2NewWindowRequestedEventArgs {
    pub fn Uri(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
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
    pub fn NewWindow(&self) -> ::windows::core::Result<CoreWebView2> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2>(result__)
        }
    }
    pub fn SetNewWindow<'a, Param0: ::windows::core::IntoParam<'a, CoreWebView2>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
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
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsUserInitiated(&self) -> ::windows::core::Result<bool> {
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
    pub fn WindowFeatures(&self) -> ::windows::core::Result<CoreWebView2WindowFeatures> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2WindowFeatures>(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows::core::Result<::windows::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Deferral>(result__)
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this =
            &::windows::core::Interface::cast::<ICoreWebView2NewWindowRequestedEventArgs2>(self)?;
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
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2NewWindowRequestedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2NewWindowRequestedEventArgs;{e6e013ba-aec8-532e-9ac9-1590af7b25ec})" ) ;
}
unsafe impl ::windows::core::Interface for CoreWebView2NewWindowRequestedEventArgs {
    type Vtable = ICoreWebView2NewWindowRequestedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xe6e013ba_aec8_532e_9ac9_1590af7b25ec);
}
impl ::windows::core::RuntimeName for CoreWebView2NewWindowRequestedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2NewWindowRequestedEventArgs";
}
impl ::core::convert::From<CoreWebView2NewWindowRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2NewWindowRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWebView2NewWindowRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2NewWindowRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2NewWindowRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2NewWindowRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWebView2NewWindowRequestedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: CoreWebView2NewWindowRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWebView2NewWindowRequestedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: &CoreWebView2NewWindowRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2NewWindowRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2NewWindowRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreWebView2NewWindowRequestedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2NewWindowRequestedEventArgs {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CoreWebView2PermissionKind(pub i32);
impl CoreWebView2PermissionKind {
    pub const UnknownPermission: CoreWebView2PermissionKind = CoreWebView2PermissionKind(0i32);
    pub const Microphone: CoreWebView2PermissionKind = CoreWebView2PermissionKind(1i32);
    pub const Camera: CoreWebView2PermissionKind = CoreWebView2PermissionKind(2i32);
    pub const Geolocation: CoreWebView2PermissionKind = CoreWebView2PermissionKind(3i32);
    pub const Notifications: CoreWebView2PermissionKind = CoreWebView2PermissionKind(4i32);
    pub const OtherSensors: CoreWebView2PermissionKind = CoreWebView2PermissionKind(5i32);
    pub const ClipboardRead: CoreWebView2PermissionKind = CoreWebView2PermissionKind(6i32);
}
impl ::core::convert::From<i32> for CoreWebView2PermissionKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CoreWebView2PermissionKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2PermissionKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2PermissionKind;i4)",
    );
}
impl ::windows::core::DefaultType for CoreWebView2PermissionKind {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CoreWebView2PermissionRequestedEventArgs(pub ::windows::core::IInspectable);
impl CoreWebView2PermissionRequestedEventArgs {
    pub fn Uri(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
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
    pub fn PermissionKind(&self) -> ::windows::core::Result<CoreWebView2PermissionKind> {
        let this = self;
        unsafe {
            let mut result__: CoreWebView2PermissionKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2PermissionKind>(result__)
        }
    }
    pub fn IsUserInitiated(&self) -> ::windows::core::Result<bool> {
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
    pub fn State(&self) -> ::windows::core::Result<CoreWebView2PermissionState> {
        let this = self;
        unsafe {
            let mut result__: CoreWebView2PermissionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2PermissionState>(result__)
        }
    }
    pub fn SetState(&self, value: CoreWebView2PermissionState) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn GetDeferral(&self) -> ::windows::core::Result<::windows::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2PermissionRequestedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2PermissionRequestedEventArgs;{118bdd9b-cef1-5910-929e-c1a321328239})" ) ;
}
unsafe impl ::windows::core::Interface for CoreWebView2PermissionRequestedEventArgs {
    type Vtable = ICoreWebView2PermissionRequestedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x118bdd9b_cef1_5910_929e_c1a321328239);
}
impl ::windows::core::RuntimeName for CoreWebView2PermissionRequestedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2PermissionRequestedEventArgs";
}
impl ::core::convert::From<CoreWebView2PermissionRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2PermissionRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWebView2PermissionRequestedEventArgs>
    for ::windows::core::IUnknown
{
    fn from(value: &CoreWebView2PermissionRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2PermissionRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2PermissionRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWebView2PermissionRequestedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: CoreWebView2PermissionRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWebView2PermissionRequestedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: &CoreWebView2PermissionRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2PermissionRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2PermissionRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreWebView2PermissionRequestedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2PermissionRequestedEventArgs {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CoreWebView2PermissionState(pub i32);
impl CoreWebView2PermissionState {
    pub const Default: CoreWebView2PermissionState = CoreWebView2PermissionState(0i32);
    pub const Allow: CoreWebView2PermissionState = CoreWebView2PermissionState(1i32);
    pub const Deny: CoreWebView2PermissionState = CoreWebView2PermissionState(2i32);
}
impl ::core::convert::From<i32> for CoreWebView2PermissionState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CoreWebView2PermissionState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2PermissionState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2PermissionState;i4)",
    );
}
impl ::windows::core::DefaultType for CoreWebView2PermissionState {
    type DefaultType = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct CoreWebView2PhysicalKeyStatus {
    pub RepeatCount: u32,
    pub ScanCode: u32,
    pub IsExtendedKey: i32,
    pub IsMenuKeyDown: i32,
    pub WasKeyDown: i32,
    pub IsKeyReleased: i32,
}
impl CoreWebView2PhysicalKeyStatus {}
impl ::core::default::Default for CoreWebView2PhysicalKeyStatus {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for CoreWebView2PhysicalKeyStatus {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CoreWebView2PhysicalKeyStatus")
            .field("RepeatCount", &self.RepeatCount)
            .field("ScanCode", &self.ScanCode)
            .field("IsExtendedKey", &self.IsExtendedKey)
            .field("IsMenuKeyDown", &self.IsMenuKeyDown)
            .field("WasKeyDown", &self.WasKeyDown)
            .field("IsKeyReleased", &self.IsKeyReleased)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CoreWebView2PhysicalKeyStatus {
    fn eq(&self, other: &Self) -> bool {
        self.RepeatCount == other.RepeatCount
            && self.ScanCode == other.ScanCode
            && self.IsExtendedKey == other.IsExtendedKey
            && self.IsMenuKeyDown == other.IsMenuKeyDown
            && self.WasKeyDown == other.WasKeyDown
            && self.IsKeyReleased == other.IsKeyReleased
    }
}
impl ::core::cmp::Eq for CoreWebView2PhysicalKeyStatus {}
unsafe impl ::windows::core::Abi for CoreWebView2PhysicalKeyStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2PhysicalKeyStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"struct(Microsoft.Web.WebView2.Core.CoreWebView2PhysicalKeyStatus;u4;u4;i4;i4;i4;i4)",
    );
}
impl ::windows::core::DefaultType for CoreWebView2PhysicalKeyStatus {
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
pub struct CoreWebView2PointerEventKind(pub i32);
impl CoreWebView2PointerEventKind {
    pub const Activate: CoreWebView2PointerEventKind = CoreWebView2PointerEventKind(587i32);
    pub const Down: CoreWebView2PointerEventKind = CoreWebView2PointerEventKind(582i32);
    pub const Enter: CoreWebView2PointerEventKind = CoreWebView2PointerEventKind(585i32);
    pub const Leave: CoreWebView2PointerEventKind = CoreWebView2PointerEventKind(586i32);
    pub const Up: CoreWebView2PointerEventKind = CoreWebView2PointerEventKind(583i32);
    pub const Update: CoreWebView2PointerEventKind = CoreWebView2PointerEventKind(581i32);
}
impl ::core::convert::From<i32> for CoreWebView2PointerEventKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CoreWebView2PointerEventKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2PointerEventKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2PointerEventKind;i4)",
    );
}
impl ::windows::core::DefaultType for CoreWebView2PointerEventKind {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CoreWebView2PointerInfo(pub ::windows::core::IInspectable);
impl CoreWebView2PointerInfo {
    pub fn PointerKind(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn SetPointerKind(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn PointerId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn SetPointerId(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn FrameId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn SetFrameId(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn PointerFlags(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn SetPointerFlags(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn PointerDeviceRect(&self) -> ::windows::core::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Rect>(result__)
        }
    }
    pub fn SetPointerDeviceRect<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Rect>,
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
    pub fn DisplayRect(&self) -> ::windows::core::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Rect>(result__)
        }
    }
    pub fn SetDisplayRect<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Rect>,
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
    pub fn PixelLocation(&self) -> ::windows::core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Point>(result__)
        }
    }
    pub fn SetPixelLocation<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Point>,
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
    pub fn HimetricLocation(&self) -> ::windows::core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Point>(result__)
        }
    }
    pub fn SetHimetricLocation<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Point>,
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
    pub fn PixelLocationRaw(&self) -> ::windows::core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Point>(result__)
        }
    }
    pub fn SetPixelLocationRaw<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Point>,
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
    pub fn HimetricLocationRaw(&self) -> ::windows::core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Point>(result__)
        }
    }
    pub fn SetHimetricLocationRaw<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Point>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Time(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn SetTime(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn HistoryCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn SetHistoryCount(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn InputData(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetInputData(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn KeyStates(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).32)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn SetKeyStates(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).33)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn PerformanceCount(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).34)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u64>(result__)
        }
    }
    pub fn SetPerformanceCount(&self, value: u64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).35)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn ButtonChangeKind(&self) -> ::windows::core::Result<i32> {
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
    pub fn SetButtonChangeKind(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).37)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn PenFlags(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).38)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn SetPenFlags(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).39)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn PenMask(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).40)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn SetPenMask(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).41)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn PenPressure(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).42)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn SetPenPressure(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).43)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn PenRotation(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).44)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn SetPenRotation(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).45)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn PenTiltX(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).46)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetPenTiltX(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).47)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn PenTiltY(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).48)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetPenTiltY(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).49)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn TouchFlags(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).50)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn SetTouchFlags(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).51)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn TouchMask(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).52)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn SetTouchMask(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).53)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn TouchContact(&self) -> ::windows::core::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).54)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Rect>(result__)
        }
    }
    pub fn SetTouchContact<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Rect>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).55)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn TouchContactRaw(&self) -> ::windows::core::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).56)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Rect>(result__)
        }
    }
    pub fn SetTouchContactRaw<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Rect>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).57)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn TouchOrientation(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).58)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn SetTouchOrientation(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).59)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn TouchPressure(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).60)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn SetTouchPressure(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).61)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2PointerInfo {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2PointerInfo;{c3860e0d-c018-5a84-bc06-9f8f7b275dff})" ) ;
}
unsafe impl ::windows::core::Interface for CoreWebView2PointerInfo {
    type Vtable = ICoreWebView2PointerInfo_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc3860e0d_c018_5a84_bc06_9f8f7b275dff);
}
impl ::windows::core::RuntimeName for CoreWebView2PointerInfo {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2PointerInfo";
}
impl ::core::convert::From<CoreWebView2PointerInfo> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2PointerInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWebView2PointerInfo> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2PointerInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreWebView2PointerInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreWebView2PointerInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWebView2PointerInfo> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2PointerInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWebView2PointerInfo> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2PointerInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreWebView2PointerInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2PointerInfo
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreWebView2PointerInfo {}
unsafe impl ::core::marker::Sync for CoreWebView2PointerInfo {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CoreWebView2PrintOrientation(pub i32);
impl CoreWebView2PrintOrientation {
    pub const Portrait: CoreWebView2PrintOrientation = CoreWebView2PrintOrientation(0i32);
    pub const Landscape: CoreWebView2PrintOrientation = CoreWebView2PrintOrientation(1i32);
}
impl ::core::convert::From<i32> for CoreWebView2PrintOrientation {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CoreWebView2PrintOrientation {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2PrintOrientation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2PrintOrientation;i4)",
    );
}
impl ::windows::core::DefaultType for CoreWebView2PrintOrientation {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CoreWebView2PrintSettings(pub ::windows::core::IInspectable);
impl CoreWebView2PrintSettings {
    pub fn Orientation(&self) -> ::windows::core::Result<CoreWebView2PrintOrientation> {
        let this = self;
        unsafe {
            let mut result__: CoreWebView2PrintOrientation = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2PrintOrientation>(result__)
        }
    }
    pub fn SetOrientation(
        &self,
        value: CoreWebView2PrintOrientation,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn ScaleFactor(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetScaleFactor(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn PageWidth(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetPageWidth(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn PageHeight(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetPageHeight(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn MarginTop(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetMarginTop(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn MarginBottom(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetMarginBottom(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn MarginLeft(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetMarginLeft(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn MarginRight(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetMarginRight(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn ShouldPrintBackgrounds(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetShouldPrintBackgrounds(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn ShouldPrintSelectionOnly(&self) -> ::windows::core::Result<bool> {
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
    pub fn SetShouldPrintSelectionOnly(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn ShouldPrintHeaderAndFooter(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetShouldPrintHeaderAndFooter(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn HeaderTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetHeaderTitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).29)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FooterUri(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetFooterUri<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).31)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2PrintSettings {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2PrintSettings;{9c75c8c0-ef3d-58a8-9a8c-18eed9fd0f16})" ) ;
}
unsafe impl ::windows::core::Interface for CoreWebView2PrintSettings {
    type Vtable = ICoreWebView2PrintSettings_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x9c75c8c0_ef3d_58a8_9a8c_18eed9fd0f16);
}
impl ::windows::core::RuntimeName for CoreWebView2PrintSettings {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2PrintSettings";
}
impl ::core::convert::From<CoreWebView2PrintSettings> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2PrintSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWebView2PrintSettings> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2PrintSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreWebView2PrintSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2PrintSettings
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWebView2PrintSettings> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2PrintSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWebView2PrintSettings> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2PrintSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2PrintSettings
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2PrintSettings
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreWebView2PrintSettings {}
unsafe impl ::core::marker::Sync for CoreWebView2PrintSettings {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CoreWebView2ProcessFailedEventArgs(pub ::windows::core::IInspectable);
impl CoreWebView2ProcessFailedEventArgs {
    pub fn ProcessFailedKind(&self) -> ::windows::core::Result<CoreWebView2ProcessFailedKind> {
        let this = self;
        unsafe {
            let mut result__: CoreWebView2ProcessFailedKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2ProcessFailedKind>(result__)
        }
    }
    pub fn Reason(&self) -> ::windows::core::Result<CoreWebView2ProcessFailedReason> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2ProcessFailedEventArgs2>(self)?;
        unsafe {
            let mut result__: CoreWebView2ProcessFailedReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2ProcessFailedReason>(result__)
        }
    }
    pub fn ExitCode(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2ProcessFailedEventArgs2>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn ProcessDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2ProcessFailedEventArgs2>(self)?;
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
    pub fn FrameInfosForFailedProcess(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IVectorView<CoreWebView2FrameInfo>,
    > {
        let this = &::windows::core::Interface::cast::<ICoreWebView2ProcessFailedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVectorView<CoreWebView2FrameInfo>>(
                result__,
            )
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2ProcessFailedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2ProcessFailedEventArgs;{25a8f8c9-d944-539d-afa3-24172b48ef47})" ) ;
}
unsafe impl ::windows::core::Interface for CoreWebView2ProcessFailedEventArgs {
    type Vtable = ICoreWebView2ProcessFailedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x25a8f8c9_d944_539d_afa3_24172b48ef47);
}
impl ::windows::core::RuntimeName for CoreWebView2ProcessFailedEventArgs {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2ProcessFailedEventArgs";
}
impl ::core::convert::From<CoreWebView2ProcessFailedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2ProcessFailedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWebView2ProcessFailedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2ProcessFailedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2ProcessFailedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2ProcessFailedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWebView2ProcessFailedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2ProcessFailedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWebView2ProcessFailedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2ProcessFailedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2ProcessFailedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2ProcessFailedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreWebView2ProcessFailedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2ProcessFailedEventArgs {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CoreWebView2ProcessFailedKind(pub i32);
impl CoreWebView2ProcessFailedKind {
    pub const BrowserProcessExited: CoreWebView2ProcessFailedKind =
        CoreWebView2ProcessFailedKind(0i32);
    pub const RenderProcessExited: CoreWebView2ProcessFailedKind =
        CoreWebView2ProcessFailedKind(1i32);
    pub const RenderProcessUnresponsive: CoreWebView2ProcessFailedKind =
        CoreWebView2ProcessFailedKind(2i32);
    pub const FrameRenderProcessExited: CoreWebView2ProcessFailedKind =
        CoreWebView2ProcessFailedKind(3i32);
    pub const UtilityProcessExited: CoreWebView2ProcessFailedKind =
        CoreWebView2ProcessFailedKind(4i32);
    pub const SandboxHelperProcessExited: CoreWebView2ProcessFailedKind =
        CoreWebView2ProcessFailedKind(5i32);
    pub const GpuProcessExited: CoreWebView2ProcessFailedKind = CoreWebView2ProcessFailedKind(6i32);
    pub const PpapiPluginProcessExited: CoreWebView2ProcessFailedKind =
        CoreWebView2ProcessFailedKind(7i32);
    pub const PpapiBrokerProcessExited: CoreWebView2ProcessFailedKind =
        CoreWebView2ProcessFailedKind(8i32);
    pub const UnknownProcessExited: CoreWebView2ProcessFailedKind =
        CoreWebView2ProcessFailedKind(9i32);
}
impl ::core::convert::From<i32> for CoreWebView2ProcessFailedKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CoreWebView2ProcessFailedKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2ProcessFailedKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2ProcessFailedKind;i4)",
    );
}
impl ::windows::core::DefaultType for CoreWebView2ProcessFailedKind {
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
pub struct CoreWebView2ProcessFailedReason(pub i32);
impl CoreWebView2ProcessFailedReason {
    pub const Unexpected: CoreWebView2ProcessFailedReason = CoreWebView2ProcessFailedReason(0i32);
    pub const Unresponsive: CoreWebView2ProcessFailedReason = CoreWebView2ProcessFailedReason(1i32);
    pub const Terminated: CoreWebView2ProcessFailedReason = CoreWebView2ProcessFailedReason(2i32);
    pub const Crashed: CoreWebView2ProcessFailedReason = CoreWebView2ProcessFailedReason(3i32);
    pub const LaunchFailed: CoreWebView2ProcessFailedReason = CoreWebView2ProcessFailedReason(4i32);
    pub const OutOfMemory: CoreWebView2ProcessFailedReason = CoreWebView2ProcessFailedReason(5i32);
}
impl ::core::convert::From<i32> for CoreWebView2ProcessFailedReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CoreWebView2ProcessFailedReason {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2ProcessFailedReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2ProcessFailedReason;i4)",
    );
}
impl ::windows::core::DefaultType for CoreWebView2ProcessFailedReason {
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
pub struct CoreWebView2ScriptDialogKind(pub i32);
impl CoreWebView2ScriptDialogKind {
    pub const Alert: CoreWebView2ScriptDialogKind = CoreWebView2ScriptDialogKind(0i32);
    pub const Confirm: CoreWebView2ScriptDialogKind = CoreWebView2ScriptDialogKind(1i32);
    pub const Prompt: CoreWebView2ScriptDialogKind = CoreWebView2ScriptDialogKind(2i32);
    pub const Beforeunload: CoreWebView2ScriptDialogKind = CoreWebView2ScriptDialogKind(3i32);
}
impl ::core::convert::From<i32> for CoreWebView2ScriptDialogKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CoreWebView2ScriptDialogKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2ScriptDialogKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2ScriptDialogKind;i4)",
    );
}
impl ::windows::core::DefaultType for CoreWebView2ScriptDialogKind {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CoreWebView2ScriptDialogOpeningEventArgs(pub ::windows::core::IInspectable);
impl CoreWebView2ScriptDialogOpeningEventArgs {
    pub fn Uri(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
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
    pub fn Kind(&self) -> ::windows::core::Result<CoreWebView2ScriptDialogKind> {
        let this = self;
        unsafe {
            let mut result__: CoreWebView2ScriptDialogKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2ScriptDialogKind>(result__)
        }
    }
    pub fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
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
    pub fn DefaultText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
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
    pub fn ResultText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetResultText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
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
    pub fn Accept(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn GetDeferral(&self) -> ::windows::core::Result<::windows::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2ScriptDialogOpeningEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2ScriptDialogOpeningEventArgs;{a4315212-c7eb-568a-86e4-c61e31ba6cda})" ) ;
}
unsafe impl ::windows::core::Interface for CoreWebView2ScriptDialogOpeningEventArgs {
    type Vtable = ICoreWebView2ScriptDialogOpeningEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa4315212_c7eb_568a_86e4_c61e31ba6cda);
}
impl ::windows::core::RuntimeName for CoreWebView2ScriptDialogOpeningEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2ScriptDialogOpeningEventArgs";
}
impl ::core::convert::From<CoreWebView2ScriptDialogOpeningEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2ScriptDialogOpeningEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWebView2ScriptDialogOpeningEventArgs>
    for ::windows::core::IUnknown
{
    fn from(value: &CoreWebView2ScriptDialogOpeningEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2ScriptDialogOpeningEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2ScriptDialogOpeningEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWebView2ScriptDialogOpeningEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: CoreWebView2ScriptDialogOpeningEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWebView2ScriptDialogOpeningEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: &CoreWebView2ScriptDialogOpeningEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2ScriptDialogOpeningEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2ScriptDialogOpeningEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreWebView2ScriptDialogOpeningEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2ScriptDialogOpeningEventArgs {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CoreWebView2Settings(pub ::windows::core::IInspectable);
impl CoreWebView2Settings {
    pub fn IsScriptEnabled(&self) -> ::windows::core::Result<bool> {
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
    pub fn SetIsScriptEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsWebMessageEnabled(&self) -> ::windows::core::Result<bool> {
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
    pub fn SetIsWebMessageEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn AreDefaultScriptDialogsEnabled(&self) -> ::windows::core::Result<bool> {
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
    pub fn SetAreDefaultScriptDialogsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsStatusBarEnabled(&self) -> ::windows::core::Result<bool> {
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
    pub fn SetIsStatusBarEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn AreDevToolsEnabled(&self) -> ::windows::core::Result<bool> {
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
    pub fn SetAreDevToolsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn AreDefaultContextMenusEnabled(&self) -> ::windows::core::Result<bool> {
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
    pub fn SetAreDefaultContextMenusEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn AreHostObjectsAllowed(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAreHostObjectsAllowed(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsZoomControlEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsZoomControlEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsBuiltInErrorPageEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsBuiltInErrorPageEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn UserAgent(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Settings2>(self)?;
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
    pub fn SetUserAgent<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Settings2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn AreBrowserAcceleratorKeysEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Settings3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAreBrowserAcceleratorKeysEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Settings3>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsPasswordAutosaveEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Settings4>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsPasswordAutosaveEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Settings4>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsGeneralAutofillEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Settings4>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsGeneralAutofillEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Settings4>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsPinchZoomEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Settings5>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsPinchZoomEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Settings5>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsSwipeNavigationEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Settings6>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsSwipeNavigationEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Settings6>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn HostObjectDispatchAdapter(
        &self,
    ) -> ::windows::core::Result<ICoreWebView2DispatchAdapter> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Settings_Manual>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ICoreWebView2DispatchAdapter>(result__)
        }
    }
    pub fn SetHostObjectDispatchAdapter<
        'a,
        Param0: ::windows::core::IntoParam<'a, ICoreWebView2DispatchAdapter>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Settings_Manual>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2Settings {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2Settings;{003b325e-74cd-52dd-8024-ebb8be38e48e})" ) ;
}
unsafe impl ::windows::core::Interface for CoreWebView2Settings {
    type Vtable = ICoreWebView2Settings_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x003b325e_74cd_52dd_8024_ebb8be38e48e);
}
impl ::windows::core::RuntimeName for CoreWebView2Settings {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2Settings";
}
impl ::core::convert::From<CoreWebView2Settings> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2Settings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWebView2Settings> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2Settings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreWebView2Settings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreWebView2Settings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWebView2Settings> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2Settings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWebView2Settings> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2Settings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreWebView2Settings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2Settings
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreWebView2Settings {}
unsafe impl ::core::marker::Sync for CoreWebView2Settings {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CoreWebView2SourceChangedEventArgs(pub ::windows::core::IInspectable);
impl CoreWebView2SourceChangedEventArgs {
    pub fn IsNewDocument(&self) -> ::windows::core::Result<bool> {
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
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2SourceChangedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2SourceChangedEventArgs;{ca437b2c-6a18-5552-b749-b198f8cc34d9})" ) ;
}
unsafe impl ::windows::core::Interface for CoreWebView2SourceChangedEventArgs {
    type Vtable = ICoreWebView2SourceChangedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xca437b2c_6a18_5552_b749_b198f8cc34d9);
}
impl ::windows::core::RuntimeName for CoreWebView2SourceChangedEventArgs {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2SourceChangedEventArgs";
}
impl ::core::convert::From<CoreWebView2SourceChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2SourceChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWebView2SourceChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2SourceChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2SourceChangedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2SourceChangedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWebView2SourceChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2SourceChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWebView2SourceChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2SourceChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2SourceChangedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2SourceChangedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreWebView2SourceChangedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2SourceChangedEventArgs {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CoreWebView2WebErrorStatus(pub i32);
impl CoreWebView2WebErrorStatus {
    pub const Unknown: CoreWebView2WebErrorStatus = CoreWebView2WebErrorStatus(0i32);
    pub const CertificateCommonNameIsIncorrect: CoreWebView2WebErrorStatus =
        CoreWebView2WebErrorStatus(1i32);
    pub const CertificateExpired: CoreWebView2WebErrorStatus = CoreWebView2WebErrorStatus(2i32);
    pub const ClientCertificateContainsErrors: CoreWebView2WebErrorStatus =
        CoreWebView2WebErrorStatus(3i32);
    pub const CertificateRevoked: CoreWebView2WebErrorStatus = CoreWebView2WebErrorStatus(4i32);
    pub const CertificateIsInvalid: CoreWebView2WebErrorStatus = CoreWebView2WebErrorStatus(5i32);
    pub const ServerUnreachable: CoreWebView2WebErrorStatus = CoreWebView2WebErrorStatus(6i32);
    pub const Timeout: CoreWebView2WebErrorStatus = CoreWebView2WebErrorStatus(7i32);
    pub const ErrorHttpInvalidServerResponse: CoreWebView2WebErrorStatus =
        CoreWebView2WebErrorStatus(8i32);
    pub const ConnectionAborted: CoreWebView2WebErrorStatus = CoreWebView2WebErrorStatus(9i32);
    pub const ConnectionReset: CoreWebView2WebErrorStatus = CoreWebView2WebErrorStatus(10i32);
    pub const Disconnected: CoreWebView2WebErrorStatus = CoreWebView2WebErrorStatus(11i32);
    pub const CannotConnect: CoreWebView2WebErrorStatus = CoreWebView2WebErrorStatus(12i32);
    pub const HostNameNotResolved: CoreWebView2WebErrorStatus = CoreWebView2WebErrorStatus(13i32);
    pub const OperationCanceled: CoreWebView2WebErrorStatus = CoreWebView2WebErrorStatus(14i32);
    pub const RedirectFailed: CoreWebView2WebErrorStatus = CoreWebView2WebErrorStatus(15i32);
    pub const UnexpectedError: CoreWebView2WebErrorStatus = CoreWebView2WebErrorStatus(16i32);
}
impl ::core::convert::From<i32> for CoreWebView2WebErrorStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CoreWebView2WebErrorStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2WebErrorStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2WebErrorStatus;i4)",
    );
}
impl ::windows::core::DefaultType for CoreWebView2WebErrorStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CoreWebView2WebMessageReceivedEventArgs(pub ::windows::core::IInspectable);
impl CoreWebView2WebMessageReceivedEventArgs {
    pub fn Source(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
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
    pub fn WebMessageAsJson(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
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
    pub fn TryGetWebMessageAsString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
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
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2WebMessageReceivedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2WebMessageReceivedEventArgs;{eb066159-b725-5d5b-adc8-f5d7b9290304})" ) ;
}
unsafe impl ::windows::core::Interface for CoreWebView2WebMessageReceivedEventArgs {
    type Vtable = ICoreWebView2WebMessageReceivedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xeb066159_b725_5d5b_adc8_f5d7b9290304);
}
impl ::windows::core::RuntimeName for CoreWebView2WebMessageReceivedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2WebMessageReceivedEventArgs";
}
impl ::core::convert::From<CoreWebView2WebMessageReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2WebMessageReceivedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWebView2WebMessageReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2WebMessageReceivedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2WebMessageReceivedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2WebMessageReceivedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWebView2WebMessageReceivedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: CoreWebView2WebMessageReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWebView2WebMessageReceivedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: &CoreWebView2WebMessageReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2WebMessageReceivedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2WebMessageReceivedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreWebView2WebMessageReceivedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2WebMessageReceivedEventArgs {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CoreWebView2WebResourceContext(pub i32);
impl CoreWebView2WebResourceContext {
    pub const All: CoreWebView2WebResourceContext = CoreWebView2WebResourceContext(0i32);
    pub const Document: CoreWebView2WebResourceContext = CoreWebView2WebResourceContext(1i32);
    pub const Stylesheet: CoreWebView2WebResourceContext = CoreWebView2WebResourceContext(2i32);
    pub const Image: CoreWebView2WebResourceContext = CoreWebView2WebResourceContext(3i32);
    pub const Media: CoreWebView2WebResourceContext = CoreWebView2WebResourceContext(4i32);
    pub const Font: CoreWebView2WebResourceContext = CoreWebView2WebResourceContext(5i32);
    pub const Script: CoreWebView2WebResourceContext = CoreWebView2WebResourceContext(6i32);
    pub const XmlHttpRequest: CoreWebView2WebResourceContext = CoreWebView2WebResourceContext(7i32);
    pub const Fetch: CoreWebView2WebResourceContext = CoreWebView2WebResourceContext(8i32);
    pub const TextTrack: CoreWebView2WebResourceContext = CoreWebView2WebResourceContext(9i32);
    pub const EventSource: CoreWebView2WebResourceContext = CoreWebView2WebResourceContext(10i32);
    pub const Websocket: CoreWebView2WebResourceContext = CoreWebView2WebResourceContext(11i32);
    pub const Manifest: CoreWebView2WebResourceContext = CoreWebView2WebResourceContext(12i32);
    pub const SignedExchange: CoreWebView2WebResourceContext =
        CoreWebView2WebResourceContext(13i32);
    pub const Ping: CoreWebView2WebResourceContext = CoreWebView2WebResourceContext(14i32);
    pub const CspViolationReport: CoreWebView2WebResourceContext =
        CoreWebView2WebResourceContext(15i32);
    pub const Other: CoreWebView2WebResourceContext = CoreWebView2WebResourceContext(16i32);
}
impl ::core::convert::From<i32> for CoreWebView2WebResourceContext {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CoreWebView2WebResourceContext {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2WebResourceContext {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2WebResourceContext;i4)",
    );
}
impl ::windows::core::DefaultType for CoreWebView2WebResourceContext {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CoreWebView2WebResourceRequest(pub ::windows::core::IInspectable);
impl CoreWebView2WebResourceRequest {
    pub fn Uri(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
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
    pub fn SetUri<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
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
    pub fn Method(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
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
    pub fn SetMethod<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
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
    pub fn Content(
        &self,
    ) -> ::windows::core::Result<::windows::Storage::Streams::IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Storage::Streams::IRandomAccessStream>(result__)
        }
    }
    pub fn SetContent<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Storage::Streams::IRandomAccessStream>,
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
    pub fn Headers(&self) -> ::windows::core::Result<CoreWebView2HttpRequestHeaders> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2HttpRequestHeaders>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2WebResourceRequest {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2WebResourceRequest;{5c742259-67d2-5df2-8382-0f201b4d7197})" ) ;
}
unsafe impl ::windows::core::Interface for CoreWebView2WebResourceRequest {
    type Vtable = ICoreWebView2WebResourceRequest_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x5c742259_67d2_5df2_8382_0f201b4d7197);
}
impl ::windows::core::RuntimeName for CoreWebView2WebResourceRequest {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2WebResourceRequest";
}
impl ::core::convert::From<CoreWebView2WebResourceRequest> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2WebResourceRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWebView2WebResourceRequest> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2WebResourceRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2WebResourceRequest
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2WebResourceRequest
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWebView2WebResourceRequest> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2WebResourceRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWebView2WebResourceRequest> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2WebResourceRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2WebResourceRequest
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2WebResourceRequest
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreWebView2WebResourceRequest {}
unsafe impl ::core::marker::Sync for CoreWebView2WebResourceRequest {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CoreWebView2WebResourceRequestedEventArgs(pub ::windows::core::IInspectable);
impl CoreWebView2WebResourceRequestedEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<CoreWebView2WebResourceRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2WebResourceRequest>(result__)
        }
    }
    pub fn Response(&self) -> ::windows::core::Result<CoreWebView2WebResourceResponse> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2WebResourceResponse>(result__)
        }
    }
    pub fn SetResponse<
        'a,
        Param0: ::windows::core::IntoParam<'a, CoreWebView2WebResourceResponse>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ResourceContext(&self) -> ::windows::core::Result<CoreWebView2WebResourceContext> {
        let this = self;
        unsafe {
            let mut result__: CoreWebView2WebResourceContext = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2WebResourceContext>(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows::core::Result<::windows::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2WebResourceRequestedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2WebResourceRequestedEventArgs;{577f1fc4-c943-54a9-9700-bd469b48bd41})" ) ;
}
unsafe impl ::windows::core::Interface for CoreWebView2WebResourceRequestedEventArgs {
    type Vtable = ICoreWebView2WebResourceRequestedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x577f1fc4_c943_54a9_9700_bd469b48bd41);
}
impl ::windows::core::RuntimeName for CoreWebView2WebResourceRequestedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2WebResourceRequestedEventArgs";
}
impl ::core::convert::From<CoreWebView2WebResourceRequestedEventArgs>
    for ::windows::core::IUnknown
{
    fn from(value: CoreWebView2WebResourceRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWebView2WebResourceRequestedEventArgs>
    for ::windows::core::IUnknown
{
    fn from(value: &CoreWebView2WebResourceRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2WebResourceRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2WebResourceRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWebView2WebResourceRequestedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: CoreWebView2WebResourceRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWebView2WebResourceRequestedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: &CoreWebView2WebResourceRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2WebResourceRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2WebResourceRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreWebView2WebResourceRequestedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2WebResourceRequestedEventArgs {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CoreWebView2WebResourceResponse(pub ::windows::core::IInspectable);
impl CoreWebView2WebResourceResponse {
    pub fn Content(
        &self,
    ) -> ::windows::core::Result<::windows::Storage::Streams::IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Storage::Streams::IRandomAccessStream>(result__)
        }
    }
    pub fn SetContent<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Storage::Streams::IRandomAccessStream>,
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
    pub fn Headers(&self) -> ::windows::core::Result<CoreWebView2HttpResponseHeaders> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2HttpResponseHeaders>(result__)
        }
    }
    pub fn StatusCode(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetStatusCode(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn ReasonPhrase(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetReasonPhrase<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2WebResourceResponse {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2WebResourceResponse;{14621923-e485-5f44-8f5d-bd4243bc398f})" ) ;
}
unsafe impl ::windows::core::Interface for CoreWebView2WebResourceResponse {
    type Vtable = ICoreWebView2WebResourceResponse_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x14621923_e485_5f44_8f5d_bd4243bc398f);
}
impl ::windows::core::RuntimeName for CoreWebView2WebResourceResponse {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2WebResourceResponse";
}
impl ::core::convert::From<CoreWebView2WebResourceResponse> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2WebResourceResponse) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWebView2WebResourceResponse> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2WebResourceResponse) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2WebResourceResponse
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2WebResourceResponse
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWebView2WebResourceResponse> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2WebResourceResponse) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWebView2WebResourceResponse> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2WebResourceResponse) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2WebResourceResponse
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2WebResourceResponse
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreWebView2WebResourceResponse {}
unsafe impl ::core::marker::Sync for CoreWebView2WebResourceResponse {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CoreWebView2WebResourceResponseReceivedEventArgs(pub ::windows::core::IInspectable);
impl CoreWebView2WebResourceResponseReceivedEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<CoreWebView2WebResourceRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2WebResourceRequest>(result__)
        }
    }
    pub fn Response(&self) -> ::windows::core::Result<CoreWebView2WebResourceResponseView> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2WebResourceResponseView>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2WebResourceResponseReceivedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2WebResourceResponseReceivedEventArgs;{12424671-9711-54f4-bcdf-5f307add6ec2})" ) ;
}
unsafe impl ::windows::core::Interface for CoreWebView2WebResourceResponseReceivedEventArgs {
    type Vtable = ICoreWebView2WebResourceResponseReceivedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x12424671_9711_54f4_bcdf_5f307add6ec2);
}
impl ::windows::core::RuntimeName for CoreWebView2WebResourceResponseReceivedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2WebResourceResponseReceivedEventArgs";
}
impl ::core::convert::From<CoreWebView2WebResourceResponseReceivedEventArgs>
    for ::windows::core::IUnknown
{
    fn from(value: CoreWebView2WebResourceResponseReceivedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWebView2WebResourceResponseReceivedEventArgs>
    for ::windows::core::IUnknown
{
    fn from(value: &CoreWebView2WebResourceResponseReceivedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2WebResourceResponseReceivedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2WebResourceResponseReceivedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWebView2WebResourceResponseReceivedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: CoreWebView2WebResourceResponseReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWebView2WebResourceResponseReceivedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: &CoreWebView2WebResourceResponseReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2WebResourceResponseReceivedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2WebResourceResponseReceivedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreWebView2WebResourceResponseReceivedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2WebResourceResponseReceivedEventArgs {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CoreWebView2WebResourceResponseView(pub ::windows::core::IInspectable);
impl CoreWebView2WebResourceResponseView {
    pub fn Headers(&self) -> ::windows::core::Result<CoreWebView2HttpResponseHeaders> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2HttpResponseHeaders>(result__)
        }
    }
    pub fn StatusCode(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn ReasonPhrase(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
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
    pub fn GetContentAsync(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::IAsyncOperation<::windows::Storage::Streams::IRandomAccessStream>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IAsyncOperation<
                ::windows::Storage::Streams::IRandomAccessStream,
            >>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2WebResourceResponseView {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2WebResourceResponseView;{33ee060b-b578-5698-b541-fef87fe7fe72})" ) ;
}
unsafe impl ::windows::core::Interface for CoreWebView2WebResourceResponseView {
    type Vtable = ICoreWebView2WebResourceResponseView_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x33ee060b_b578_5698_b541_fef87fe7fe72);
}
impl ::windows::core::RuntimeName for CoreWebView2WebResourceResponseView {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2WebResourceResponseView";
}
impl ::core::convert::From<CoreWebView2WebResourceResponseView> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2WebResourceResponseView) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWebView2WebResourceResponseView> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2WebResourceResponseView) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2WebResourceResponseView
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2WebResourceResponseView
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWebView2WebResourceResponseView> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2WebResourceResponseView) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWebView2WebResourceResponseView> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2WebResourceResponseView) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2WebResourceResponseView
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2WebResourceResponseView
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreWebView2WebResourceResponseView {}
unsafe impl ::core::marker::Sync for CoreWebView2WebResourceResponseView {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CoreWebView2WindowFeatures(pub ::windows::core::IInspectable);
impl CoreWebView2WindowFeatures {
    pub fn HasPosition(&self) -> ::windows::core::Result<bool> {
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
    pub fn HasSize(&self) -> ::windows::core::Result<bool> {
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
    pub fn Left(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn Top(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn Height(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn Width(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn ShouldDisplayMenuBar(&self) -> ::windows::core::Result<bool> {
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
    pub fn ShouldDisplayStatus(&self) -> ::windows::core::Result<bool> {
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
    pub fn ShouldDisplayToolbar(&self) -> ::windows::core::Result<bool> {
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
    pub fn ShouldDisplayScrollBars(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2WindowFeatures {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2WindowFeatures;{ee8686d6-056f-5e06-824f-4e2a24c1c1d6})" ) ;
}
unsafe impl ::windows::core::Interface for CoreWebView2WindowFeatures {
    type Vtable = ICoreWebView2WindowFeatures_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xee8686d6_056f_5e06_824f_4e2a24c1c1d6);
}
impl ::windows::core::RuntimeName for CoreWebView2WindowFeatures {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2WindowFeatures";
}
impl ::core::convert::From<CoreWebView2WindowFeatures> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2WindowFeatures) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWebView2WindowFeatures> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2WindowFeatures) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreWebView2WindowFeatures {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2WindowFeatures
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWebView2WindowFeatures> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2WindowFeatures) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWebView2WindowFeatures> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2WindowFeatures) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2WindowFeatures
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2WindowFeatures
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreWebView2WindowFeatures {}
unsafe impl ::core::marker::Sync for CoreWebView2WindowFeatures {}
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWebView2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2 {
    type Vtable = ICoreWebView2_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x3a3f559a_e5e9_5338_bb67_4eb0504a4f14);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2_abi(
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
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut u32,
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
        result__: *mut bool,
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
        uri: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        htmlcontent: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        javascript: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        javascript: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        imageformat: CoreWebView2CapturePreviewImageFormat,
        imagestream: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        webmessageasjson: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        webmessageasstring: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        methodname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        parametersasjson: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        eventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        rawobject: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        uri: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        resourcecontext: CoreWebView2WebResourceContext,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        uri: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        resourcecontext: CoreWebView2WebResourceContext,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWebView2AcceleratorKeyPressedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2AcceleratorKeyPressedEventArgs {
    type Vtable = ICoreWebView2AcceleratorKeyPressedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x41a56100_92a5_59d1_9e71_9222e33ae38b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2AcceleratorKeyPressedEventArgs_abi(
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
        result__: *mut CoreWebView2KeyEventKind,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut CoreWebView2PhysicalKeyStatus,
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
pub struct ICoreWebView2BrowserProcessExitedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2BrowserProcessExitedEventArgs {
    type Vtable = ICoreWebView2BrowserProcessExitedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x79963f77_1484_5a46_b91f_dfc5c1a0ce14);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2BrowserProcessExitedEventArgs_abi(
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
        result__: *mut CoreWebView2BrowserProcessExitKind,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWebView2ClientCertificate(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2ClientCertificate {
    type Vtable = ICoreWebView2ClientCertificate_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x091b39f2_68df_52b4_8fb0_fd3561af41f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2ClientCertificate_abi(
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
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
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
        result__: *mut CoreWebView2ClientCertificateKind,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWebView2ClientCertificateRequestedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2ClientCertificateRequestedEventArgs {
    type Vtable = ICoreWebView2ClientCertificateRequestedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x93287b55_31f9_55a0_b68b_d9841d7e1bf4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2ClientCertificateRequestedEventArgs_abi(
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
        result__: *mut i32,
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWebView2CompositionController(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2CompositionController {
    type Vtable = ICoreWebView2CompositionController_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4fb8b7b3_4a2e_5787_94b9_cc48c4d364d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2CompositionController_abi(
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
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        eventkind: CoreWebView2MouseEventKind,
        virtualkeys: CoreWebView2MouseEventVirtualKeys,
        mousedata: u32,
        point: ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        eventkind: CoreWebView2PointerEventKind,
        pointerinfo: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWebView2CompositionController2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2CompositionController2 {
    type Vtable = ICoreWebView2CompositionController2_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x8cef61b9_fa55_547d_aae6_7bcaed4249a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2CompositionController2_abi(
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
pub struct ICoreWebView2CompositionControllerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2CompositionControllerStatics {
    type Vtable = ICoreWebView2CompositionControllerStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4df0ab1f_7f2a_573b_b81a_b9b531224736);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2CompositionControllerStatics_abi(
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
pub struct ICoreWebView2ContentLoadingEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2ContentLoadingEventArgs {
    type Vtable = ICoreWebView2ContentLoadingEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x6cf95373_946c_5dae_9b3e_0fe23d5aa29f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2ContentLoadingEventArgs_abi(
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
        result__: *mut u64,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWebView2Controller(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2Controller {
    type Vtable = ICoreWebView2Controller_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa588121c_53bf_590e_80e5_29d729cbd743);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Controller_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: f64,
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
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        bounds: ::windows::Foundation::Rect,
        zoomfactor: f64,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        reason: CoreWebView2MoveFocusReason,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWebView2Controller2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2Controller2 {
    type Vtable = ICoreWebView2Controller2_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x0069c40b_2e8a_513f_9d9d_e0c2b64f7200);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Controller2_abi(
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
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWebView2Controller3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2Controller3 {
    type Vtable = ICoreWebView2Controller3_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xe5bae214_791a_5d13_9b76_a257d9fda2ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Controller3_abi(
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
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: f64,
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
        result__: *mut CoreWebView2BoundsMode,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: CoreWebView2BoundsMode,
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
pub struct ICoreWebView2ControllerFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2ControllerFactory {
    type Vtable = ICoreWebView2ControllerFactory_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x270b2c5b_c3a9_53d8_a5ca_262ea9ea62e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2ControllerFactory_abi(
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
pub struct ICoreWebView2ControllerWindowReference(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2ControllerWindowReference {
    type Vtable = ICoreWebView2ControllerWindowReference_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x0feddad4_48a3_5cc4_9f61_e7adfd1e9c76);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2ControllerWindowReference_abi(
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
        result__: *mut u64,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWebView2ControllerWindowReferenceStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2ControllerWindowReferenceStatics {
    type Vtable = ICoreWebView2ControllerWindowReferenceStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xddf6ebf1_ebc6_5a34_9008_661c3a2eb767);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2ControllerWindowReferenceStatics_abi(
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
        windowhandle: u64,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        corewindow: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWebView2Cookie(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2Cookie {
    type Vtable = ICoreWebView2Cookie_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x52f670fe_8ca2_5aad_aedb_25f7903b7038);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Cookie_abi(
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
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: f64,
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
        result__: *mut CoreWebView2CookieSameSiteKind,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: CoreWebView2CookieSameSiteKind,
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWebView2CookieManager(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2CookieManager {
    type Vtable = ICoreWebView2CookieManager_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4098f516_adca_5563_aaa5_d7affd847aa3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2CookieManager_abi(
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
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        domain: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        cookieparam: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        cookie: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        cookie: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        uri: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        domain: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWebView2DOMContentLoadedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2DOMContentLoadedEventArgs {
    type Vtable = ICoreWebView2DOMContentLoadedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc474d0a3_24ac_59fc_b78b_da7562a6a052);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2DOMContentLoadedEventArgs_abi(
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
        result__: *mut u64,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWebView2DevToolsProtocolEventReceivedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2DevToolsProtocolEventReceivedEventArgs {
    type Vtable = ICoreWebView2DevToolsProtocolEventReceivedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xb6a4b41d_fd18_59fa_923a_c57555d960ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2DevToolsProtocolEventReceivedEventArgs_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWebView2DevToolsProtocolEventReceiver(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2DevToolsProtocolEventReceiver {
    type Vtable = ICoreWebView2DevToolsProtocolEventReceiver_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xb2a2be79_65fc_5537_8715_3d92bf31090b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2DevToolsProtocolEventReceiver_abi(
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
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct ICoreWebView2DispatchAdapter(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2DispatchAdapter {
    type Vtable = ICoreWebView2DispatchAdapter_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x7888a42d_18f3_5966_80cb_8cc25351bd0a);
}
impl ICoreWebView2DispatchAdapter {
    pub fn WrapNamedObject<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, ICoreWebView2DispatchAdapter>,
    >(
        &self,
        name: Param0,
        adapter: Param1,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                adapter.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn WrapObject<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param1: ::windows::core::IntoParam<'a, ICoreWebView2DispatchAdapter>,
    >(
        &self,
        unwrapped: Param0,
        adapter: Param1,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                unwrapped.into_param().abi(),
                adapter.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn UnwrapObject<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        wrapped: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                wrapped.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn Clean(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ICoreWebView2DispatchAdapter {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{7888a42d-18f3-5966-80cb-8cc25351bd0a}");
}
impl ::core::convert::From<ICoreWebView2DispatchAdapter> for ::windows::core::IUnknown {
    fn from(value: ICoreWebView2DispatchAdapter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ICoreWebView2DispatchAdapter> for ::windows::core::IUnknown {
    fn from(value: &ICoreWebView2DispatchAdapter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for ICoreWebView2DispatchAdapter
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a ICoreWebView2DispatchAdapter
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ICoreWebView2DispatchAdapter> for ::windows::core::IInspectable {
    fn from(value: ICoreWebView2DispatchAdapter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICoreWebView2DispatchAdapter> for ::windows::core::IInspectable {
    fn from(value: &ICoreWebView2DispatchAdapter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for ICoreWebView2DispatchAdapter
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a ICoreWebView2DispatchAdapter
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2DispatchAdapter_abi(
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
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        adapter: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        unwrapped: ::windows::core::RawPtr,
        adapter: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        wrapped: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWebView2DownloadOperation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2DownloadOperation {
    type Vtable = ICoreWebView2DownloadOperation_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xafe73e6b_e760_5a06_9bf6_1e743c13cd2d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2DownloadOperation_abi(
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
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut i64,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut i64,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut CoreWebView2DownloadState,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut CoreWebView2DownloadInterruptReason,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut bool,
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
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWebView2DownloadStartingEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2DownloadStartingEventArgs {
    type Vtable = ICoreWebView2DownloadStartingEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x45d982ba_9256_5b35_b023_26a438599110);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2DownloadStartingEventArgs_abi(
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
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: bool,
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWebView2Environment(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2Environment {
    type Vtable = ICoreWebView2Environment_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xd8cc7831_b783_556b_b9ce_899c1e95d585);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Environment_abi(
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
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        parentwindow: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        content: ::windows::core::RawPtr,
        statuscode: i32,
        reasonphrase: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        headers: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWebView2Environment2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2Environment2 {
    type Vtable = ICoreWebView2Environment2_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x0b634668_1017_5fc7_9921_f1f51866a8c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Environment2_abi(
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
        uri: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        method: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        postdata: ::windows::core::RawPtr,
        headers: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWebView2Environment3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2Environment3 {
    type Vtable = ICoreWebView2Environment3_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x5e33f46c_c0b9_5126_8840_17f9c11b3a8a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Environment3_abi(
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
        parentwindow: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWebView2Environment4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2Environment4 {
    type Vtable = ICoreWebView2Environment4_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x6db697da_eebd_5818_8790_1fe57ef319e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Environment4_abi(
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
pub struct ICoreWebView2Environment5(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2Environment5 {
    type Vtable = ICoreWebView2Environment5_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xf33399af_e4d3_59dc_ac38_8397aadcedb1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Environment5_abi(
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
pub struct ICoreWebView2Environment6(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2Environment6 {
    type Vtable = ICoreWebView2Environment6_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x965d538f_5958_5d98_8972_f622021df402);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Environment6_abi(
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
pub struct ICoreWebView2EnvironmentOptions(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2EnvironmentOptions {
    type Vtable = ICoreWebView2EnvironmentOptions_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x25d6dc39_0062_5735_8b09_a6f535f19e97);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2EnvironmentOptions_abi(
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
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
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
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
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
pub struct ICoreWebView2EnvironmentOptions_Manual(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2EnvironmentOptions_Manual {
    type Vtable = ICoreWebView2EnvironmentOptions_Manual_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x1f104443_ea93_5a37_b791_34e6a31172ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2EnvironmentOptions_Manual_abi(
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
pub struct ICoreWebView2EnvironmentStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2EnvironmentStatics {
    type Vtable = ICoreWebView2EnvironmentStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x0e33f804_f20b_5635_8491_162aaa27517b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2EnvironmentStatics_abi(
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
        browserexecutablefolder: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        userdatafolder: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        options: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        browserexecutablefolder: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        browserversionstring1: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        browserversionstring2: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWebView2Frame(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2Frame {
    type Vtable = ICoreWebView2Frame_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x02ffcbf9_19e7_5bb8_8273_346420fb1503);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Frame_abi(
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
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWebView2FrameCreatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2FrameCreatedEventArgs {
    type Vtable = ICoreWebView2FrameCreatedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x527b01b8_fc6d_5543_8dce_96cdfdb32c81);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2FrameCreatedEventArgs_abi(
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
pub struct ICoreWebView2FrameInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2FrameInfo {
    type Vtable = ICoreWebView2FrameInfo_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xf9b82e06_73f3_513b_bc2c_445ddedba976);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2FrameInfo_abi(
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
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWebView2HttpHeadersCollectionIterator(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2HttpHeadersCollectionIterator {
    type Vtable = ICoreWebView2HttpHeadersCollectionIterator_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xadf264ee_d980_5f48_a60e_8705de046608);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2HttpHeadersCollectionIterator_abi(
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
pub struct ICoreWebView2HttpRequestHeaders(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2HttpRequestHeaders {
    type Vtable = ICoreWebView2HttpRequestHeaders_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xdc2226c7_3515_55bb_bcb2_57b78f86b91d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2HttpRequestHeaders_abi(
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
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWebView2HttpResponseHeaders(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2HttpResponseHeaders {
    type Vtable = ICoreWebView2HttpResponseHeaders_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xf3d383e9_747f_5574_8662_9a6b920cecd4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2HttpResponseHeaders_abi(
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
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWebView2MoveFocusRequestedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2MoveFocusRequestedEventArgs {
    type Vtable = ICoreWebView2MoveFocusRequestedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x2e29103b_ecdd_5c1d_b288_3f066d608919);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2MoveFocusRequestedEventArgs_abi(
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
        result__: *mut CoreWebView2MoveFocusReason,
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
pub struct ICoreWebView2NavigationCompletedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2NavigationCompletedEventArgs {
    type Vtable = ICoreWebView2NavigationCompletedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4865e238_036a_5664_95a3_447ec44cf498);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2NavigationCompletedEventArgs_abi(
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
        result__: *mut CoreWebView2WebErrorStatus,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut u64,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWebView2NavigationStartingEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2NavigationStartingEventArgs {
    type Vtable = ICoreWebView2NavigationStartingEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x548d23d3_fea3_5616_bd05_ae08066c86d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2NavigationStartingEventArgs_abi(
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
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut u64,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWebView2NewWindowRequestedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2NewWindowRequestedEventArgs {
    type Vtable = ICoreWebView2NewWindowRequestedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xe6e013ba_aec8_532e_9ac9_1590af7b25ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2NewWindowRequestedEventArgs_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWebView2NewWindowRequestedEventArgs2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2NewWindowRequestedEventArgs2 {
    type Vtable = ICoreWebView2NewWindowRequestedEventArgs2_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xf4806259_e63a_5c0b_a02c_5f10e11094f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2NewWindowRequestedEventArgs2_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWebView2PermissionRequestedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2PermissionRequestedEventArgs {
    type Vtable = ICoreWebView2PermissionRequestedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x118bdd9b_cef1_5910_929e_c1a321328239);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2PermissionRequestedEventArgs_abi(
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
        result__: *mut CoreWebView2PermissionKind,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut CoreWebView2PermissionState,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: CoreWebView2PermissionState,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWebView2PointerInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2PointerInfo {
    type Vtable = ICoreWebView2PointerInfo_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc3860e0d_c018_5a84_bc06_9f8f7b275dff);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2PointerInfo_abi(
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
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: u32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: u32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: u32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: u32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: u32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: u32,
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
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: u32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut u64,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: u64,
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
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: u32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: u32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: u32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: u32,
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
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: u32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: u32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: u32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: u32,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWebView2PrintSettings(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2PrintSettings {
    type Vtable = ICoreWebView2PrintSettings_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x9c75c8c0_ef3d_58a8_9a8c_18eed9fd0f16);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2PrintSettings_abi(
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
        result__: *mut CoreWebView2PrintOrientation,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: CoreWebView2PrintOrientation,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: f64,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: f64,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: f64,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: f64,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: f64,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: f64,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: f64,
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
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
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
pub struct ICoreWebView2ProcessFailedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2ProcessFailedEventArgs {
    type Vtable = ICoreWebView2ProcessFailedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x25a8f8c9_d944_539d_afa3_24172b48ef47);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2ProcessFailedEventArgs_abi(
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
        result__: *mut CoreWebView2ProcessFailedKind,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWebView2ProcessFailedEventArgs2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2ProcessFailedEventArgs2 {
    type Vtable = ICoreWebView2ProcessFailedEventArgs2_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc5d9c952_b456_5dc7_9f76_fde967484af5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2ProcessFailedEventArgs2_abi(
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
        result__: *mut CoreWebView2ProcessFailedReason,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWebView2ScriptDialogOpeningEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2ScriptDialogOpeningEventArgs {
    type Vtable = ICoreWebView2ScriptDialogOpeningEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa4315212_c7eb_568a_86e4_c61e31ba6cda);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2ScriptDialogOpeningEventArgs_abi(
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
        result__: *mut CoreWebView2ScriptDialogKind,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWebView2Settings(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2Settings {
    type Vtable = ICoreWebView2Settings_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x003b325e_74cd_52dd_8024_ebb8be38e48e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Settings_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWebView2Settings2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2Settings2 {
    type Vtable = ICoreWebView2Settings2_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x377d3480_fdb2_56e7_bade_507d352887e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Settings2_abi(
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
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWebView2Settings3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2Settings3 {
    type Vtable = ICoreWebView2Settings3_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x52200f01_5309_5b2e_a03c_3d2677591940);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Settings3_abi(
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
pub struct ICoreWebView2Settings4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2Settings4 {
    type Vtable = ICoreWebView2Settings4_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xd6a955f0_daef_5a6a_a6f6_c72f0ede7620);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Settings4_abi(
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
pub struct ICoreWebView2Settings5(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2Settings5 {
    type Vtable = ICoreWebView2Settings5_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xafc42b23_4839_5d73_acf7_e0335631abf5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Settings5_abi(
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
pub struct ICoreWebView2Settings6(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2Settings6 {
    type Vtable = ICoreWebView2Settings6_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x3fe4ae85_0540_5bf1_b4d9_99ec57aa64f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Settings6_abi(
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
pub struct ICoreWebView2Settings_Manual(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2Settings_Manual {
    type Vtable = ICoreWebView2Settings_Manual_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x0a538c87_e000_511c_87ca_ded3413d03da);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Settings_Manual_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWebView2SourceChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2SourceChangedEventArgs {
    type Vtable = ICoreWebView2SourceChangedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xca437b2c_6a18_5552_b749_b198f8cc34d9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2SourceChangedEventArgs_abi(
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
pub struct ICoreWebView2WebMessageReceivedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2WebMessageReceivedEventArgs {
    type Vtable = ICoreWebView2WebMessageReceivedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xeb066159_b725_5d5b_adc8_f5d7b9290304);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2WebMessageReceivedEventArgs_abi(
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
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWebView2WebResourceRequest(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2WebResourceRequest {
    type Vtable = ICoreWebView2WebResourceRequest_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x5c742259_67d2_5df2_8382_0f201b4d7197);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2WebResourceRequest_abi(
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
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
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
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWebView2WebResourceRequestedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2WebResourceRequestedEventArgs {
    type Vtable = ICoreWebView2WebResourceRequestedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x577f1fc4_c943_54a9_9700_bd469b48bd41);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2WebResourceRequestedEventArgs_abi(
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
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut CoreWebView2WebResourceContext,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWebView2WebResourceResponse(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2WebResourceResponse {
    type Vtable = ICoreWebView2WebResourceResponse_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x14621923_e485_5f44_8f5d_bd4243bc398f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2WebResourceResponse_abi(
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
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: i32,
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
pub struct ICoreWebView2WebResourceResponseReceivedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2WebResourceResponseReceivedEventArgs {
    type Vtable = ICoreWebView2WebResourceResponseReceivedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x12424671_9711_54f4_bcdf_5f307add6ec2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2WebResourceResponseReceivedEventArgs_abi(
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
pub struct ICoreWebView2WebResourceResponseView(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2WebResourceResponseView {
    type Vtable = ICoreWebView2WebResourceResponseView_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x33ee060b_b578_5698_b541_fef87fe7fe72);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2WebResourceResponseView_abi(
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
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWebView2WindowFeatures(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2WindowFeatures {
    type Vtable = ICoreWebView2WindowFeatures_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xee8686d6_056f_5e06_824f_4e2a24c1c1d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2WindowFeatures_abi(
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
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut u32,
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
pub struct ICoreWebView2_2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2_2 {
    type Vtable = ICoreWebView2_2_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x578cb133_2873_5408_bd9e_389bbe9fa7fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2_2_abi(
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
        request: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWebView2_3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2_3 {
    type Vtable = ICoreWebView2_3_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa8c76ae7_6170_5dfe_8f00_79cd76a9b4d9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2_3_abi(
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
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        hostname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        folderpath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        accesskind: CoreWebView2HostResourceAccessKind,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        hostname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWebView2_4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2_4 {
    type Vtable = ICoreWebView2_4_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4ac595ce_1502_5775_b2c8_22c11a369c25);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2_4_abi(
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
pub struct ICoreWebView2_5(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2_5 {
    type Vtable = ICoreWebView2_5_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xdd6af643_220c_5dc6_b0a8_22c41e472595);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2_5_abi(
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
pub struct ICoreWebView2_6(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2_6 {
    type Vtable = ICoreWebView2_6_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x92b34b96_853d_5bb6_ac52_30297ce805f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2_6_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWebView2_7(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWebView2_7 {
    type Vtable = ICoreWebView2_7_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xf9b7107a_2e09_5596_a033_911ba12315f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2_7_abi(
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
        resultfilepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        printsettings: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
