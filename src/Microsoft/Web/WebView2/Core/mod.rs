#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clashing_extern_declarations,
    clippy::all
)]
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
pub struct CoreWebView2(::windows::core::IUnknown);
impl CoreWebView2 {
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Settings(&self) -> ::windows::core::Result<CoreWebView2Settings> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Settings)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2Settings>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Source(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Source)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn BrowserProcessId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BrowserProcessId)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn CanGoBack(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanGoBack)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn CanGoForward(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanGoForward)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn DocumentTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DocumentTitle)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn ContainsFullScreenElement(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContainsFullScreenElement)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).NavigationStarting)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RemoveNavigationStarting<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveNavigationStarting)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).ContentLoading)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RemoveContentLoading<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveContentLoading)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).SourceChanged)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RemoveSourceChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveSourceChanged)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).HistoryChanged)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RemoveHistoryChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveHistoryChanged)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).NavigationCompleted)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RemoveNavigationCompleted<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveNavigationCompleted)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).FrameNavigationStarting)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RemoveFrameNavigationStarting<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveFrameNavigationStarting)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).FrameNavigationCompleted)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RemoveFrameNavigationCompleted<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveFrameNavigationCompleted)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).ScriptDialogOpening)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RemoveScriptDialogOpening<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveScriptDialogOpening)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).PermissionRequested)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RemovePermissionRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemovePermissionRequested)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).ProcessFailed)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RemoveProcessFailed<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveProcessFailed)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).WebMessageReceived)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RemoveWebMessageReceived<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveWebMessageReceived)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).NewWindowRequested)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RemoveNewWindowRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveNewWindowRequested)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).DocumentTitleChanged)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RemoveDocumentTitleChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveDocumentTitleChanged)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).ContainsFullScreenElementChanged)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RemoveContainsFullScreenElementChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveContainsFullScreenElementChanged)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).WebResourceRequested)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RemoveWebResourceRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveWebResourceRequested)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).WindowCloseRequested)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RemoveWindowCloseRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveWindowCloseRequested)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Navigate<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        uri: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Navigate)(
                ::core::mem::transmute_copy(this),
                uri.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn NavigateToString<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        htmlcontent: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).NavigateToString)(
                ::core::mem::transmute_copy(this),
                htmlcontent.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).AddScriptToExecuteOnDocumentCreatedAsync)(
                ::core::mem::transmute_copy(this),
                javascript.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RemoveScriptToExecuteOnDocumentCreated<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        id: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveScriptToExecuteOnDocumentCreated)(
                ::core::mem::transmute_copy(this),
                id.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).ExecuteScriptAsync)(
                ::core::mem::transmute_copy(this),
                javascript.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).CapturePreviewAsync)(
                ::core::mem::transmute_copy(this),
                imageformat,
                imagestream.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Reload(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Reload)(::core::mem::transmute_copy(this))
                .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn PostWebMessageAsJson<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        webmessageasjson: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).PostWebMessageAsJson)(
                ::core::mem::transmute_copy(this),
                webmessageasjson.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn PostWebMessageAsString<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        webmessageasstring: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).PostWebMessageAsString)(
                ::core::mem::transmute_copy(this),
                webmessageasstring.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).CallDevToolsProtocolMethodAsync)(
                ::core::mem::transmute_copy(this),
                methodname.into_param().abi(),
                parametersasjson.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn GoBack(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).GoBack)(::core::mem::transmute_copy(this))
                .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn GoForward(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).GoForward)(::core::mem::transmute_copy(this))
                .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).GetDevToolsProtocolEventReceiver)(
                ::core::mem::transmute_copy(this),
                eventname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<CoreWebView2DevToolsProtocolEventReceiver>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Stop)(::core::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).AddHostObjectToScript)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                rawobject.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RemoveHostObjectFromScript<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveHostObjectFromScript)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn OpenDevToolsWindow(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).OpenDevToolsWindow)(
                ::core::mem::transmute_copy(this),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).AddWebResourceRequestedFilter)(
                ::core::mem::transmute_copy(this),
                uri.into_param().abi(),
                resourcecontext,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).RemoveWebResourceRequestedFilter)(
                ::core::mem::transmute_copy(this),
                uri.into_param().abi(),
                resourcecontext,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn CookieManager(&self) -> ::windows::core::Result<CoreWebView2CookieManager> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2_2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CookieManager)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2CookieManager>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Environment(&self) -> ::windows::core::Result<CoreWebView2Environment> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2_2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Environment)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2Environment>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).WebResourceResponseReceived)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RemoveWebResourceResponseReceived<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2_2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveWebResourceResponseReceived)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).DOMContentLoaded)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RemoveDOMContentLoaded<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2_2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveDOMContentLoaded)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn NavigateWithWebResourceRequest<
        'a,
        Param0: ::windows::core::IntoParam<'a, CoreWebView2WebResourceRequest>,
    >(
        &self,
        request: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2_2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).NavigateWithWebResourceRequest)(
                ::core::mem::transmute_copy(this),
                request.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn IsSuspended(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2_3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSuspended)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn TrySuspendAsync(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2_3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TrySuspendAsync)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Resume(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2_3>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).Resume)(::core::mem::transmute_copy(this))
                .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).SetVirtualHostNameToFolderMapping)(
                ::core::mem::transmute_copy(this),
                hostname.into_param().abi(),
                folderpath.into_param().abi(),
                accesskind,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn ClearVirtualHostNameToFolderMapping<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        hostname: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2_3>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).ClearVirtualHostNameToFolderMapping)(
                ::core::mem::transmute_copy(this),
                hostname.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).FrameCreated)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RemoveFrameCreated<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2_4>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveFrameCreated)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).DownloadStarting)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RemoveDownloadStarting<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2_4>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveDownloadStarting)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).ClientCertificateRequested)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RemoveClientCertificateRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2_5>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveClientCertificateRequested)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn OpenTaskManagerWindow(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2_6>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).OpenTaskManagerWindow)(
                ::core::mem::transmute_copy(this),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).PrintToPdfAsync)(
                ::core::mem::transmute_copy(this),
                resultfilepath.into_param().abi(),
                printsettings.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreWebView2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWebView2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWebView2 {}
impl ::core::fmt::Debug for CoreWebView2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Web.WebView2.Core.CoreWebView2;{3a3f559a-e5e9-5338-bb67-4eb0504a4f14})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreWebView2 {
    type Vtable = ICoreWebView2_Vtbl;
    const IID: ::windows::core::GUID = <ICoreWebView2 as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreWebView2 {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2";
}
impl ::core::convert::From<CoreWebView2> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreWebView2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreWebView2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWebView2> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreWebView2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreWebView2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreWebView2 {}
unsafe impl ::core::marker::Sync for CoreWebView2 {}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
pub struct CoreWebView2AcceleratorKeyPressedEventArgs(::windows::core::IUnknown);
impl CoreWebView2AcceleratorKeyPressedEventArgs {
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn KeyEventKind(&self) -> ::windows::core::Result<CoreWebView2KeyEventKind> {
        let this = self;
        unsafe {
            let mut result__: CoreWebView2KeyEventKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyEventKind)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2KeyEventKind>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn VirtualKey(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VirtualKey)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn KeyEventLParam(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyEventLParam)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn PhysicalKeyStatus(&self) -> ::windows::core::Result<CoreWebView2PhysicalKeyStatus> {
        let this = self;
        unsafe {
            let mut result__: CoreWebView2PhysicalKeyStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PhysicalKeyStatus)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2PhysicalKeyStatus>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Handled)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetHandled)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for CoreWebView2AcceleratorKeyPressedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWebView2AcceleratorKeyPressedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWebView2AcceleratorKeyPressedEventArgs {}
impl ::core::fmt::Debug for CoreWebView2AcceleratorKeyPressedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2AcceleratorKeyPressedEventArgs")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2AcceleratorKeyPressedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2AcceleratorKeyPressedEventArgs;{41a56100-92a5-59d1-9e71-9222e33ae38b})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreWebView2AcceleratorKeyPressedEventArgs {
    type Vtable = ICoreWebView2AcceleratorKeyPressedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        <ICoreWebView2AcceleratorKeyPressedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreWebView2AcceleratorKeyPressedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2AcceleratorKeyPressedEventArgs";
}
impl ::core::convert::From<CoreWebView2AcceleratorKeyPressedEventArgs>
    for ::windows::core::IUnknown
{
    fn from(value: CoreWebView2AcceleratorKeyPressedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2AcceleratorKeyPressedEventArgs>
    for ::windows::core::IUnknown
{
    fn from(value: &CoreWebView2AcceleratorKeyPressedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2AcceleratorKeyPressedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2AcceleratorKeyPressedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWebView2AcceleratorKeyPressedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: CoreWebView2AcceleratorKeyPressedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2AcceleratorKeyPressedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: &CoreWebView2AcceleratorKeyPressedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2AcceleratorKeyPressedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2AcceleratorKeyPressedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreWebView2AcceleratorKeyPressedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2AcceleratorKeyPressedEventArgs {}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CoreWebView2BoundsMode(pub i32);
impl CoreWebView2BoundsMode {
    pub const UseRawPixels: Self = Self(0i32);
    pub const UseRasterizationScale: Self = Self(1i32);
}
impl ::core::marker::Copy for CoreWebView2BoundsMode {}
impl ::core::clone::Clone for CoreWebView2BoundsMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2BoundsMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CoreWebView2BoundsMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for CoreWebView2BoundsMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2BoundsMode")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2BoundsMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2BoundsMode;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CoreWebView2BrowserProcessExitKind(pub i32);
impl CoreWebView2BrowserProcessExitKind {
    pub const Normal: Self = Self(0i32);
    pub const Failed: Self = Self(1i32);
}
impl ::core::marker::Copy for CoreWebView2BrowserProcessExitKind {}
impl ::core::clone::Clone for CoreWebView2BrowserProcessExitKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2BrowserProcessExitKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CoreWebView2BrowserProcessExitKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for CoreWebView2BrowserProcessExitKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2BrowserProcessExitKind")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2BrowserProcessExitKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2BrowserProcessExitKind;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
pub struct CoreWebView2BrowserProcessExitedEventArgs(::windows::core::IUnknown);
impl CoreWebView2BrowserProcessExitedEventArgs {
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn BrowserProcessExitKind(
        &self,
    ) -> ::windows::core::Result<CoreWebView2BrowserProcessExitKind> {
        let this = self;
        unsafe {
            let mut result__: CoreWebView2BrowserProcessExitKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BrowserProcessExitKind)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2BrowserProcessExitKind>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn BrowserProcessId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BrowserProcessId)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreWebView2BrowserProcessExitedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWebView2BrowserProcessExitedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWebView2BrowserProcessExitedEventArgs {}
impl ::core::fmt::Debug for CoreWebView2BrowserProcessExitedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2BrowserProcessExitedEventArgs")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2BrowserProcessExitedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2BrowserProcessExitedEventArgs;{79963f77-1484-5a46-b91f-dfc5c1a0ce14})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreWebView2BrowserProcessExitedEventArgs {
    type Vtable = ICoreWebView2BrowserProcessExitedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        <ICoreWebView2BrowserProcessExitedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreWebView2BrowserProcessExitedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2BrowserProcessExitedEventArgs";
}
impl ::core::convert::From<CoreWebView2BrowserProcessExitedEventArgs>
    for ::windows::core::IUnknown
{
    fn from(value: CoreWebView2BrowserProcessExitedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2BrowserProcessExitedEventArgs>
    for ::windows::core::IUnknown
{
    fn from(value: &CoreWebView2BrowserProcessExitedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2BrowserProcessExitedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2BrowserProcessExitedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWebView2BrowserProcessExitedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: CoreWebView2BrowserProcessExitedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2BrowserProcessExitedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: &CoreWebView2BrowserProcessExitedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2BrowserProcessExitedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2BrowserProcessExitedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreWebView2BrowserProcessExitedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2BrowserProcessExitedEventArgs {}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CoreWebView2CapturePreviewImageFormat(pub i32);
impl CoreWebView2CapturePreviewImageFormat {
    pub const Png: Self = Self(0i32);
    pub const Jpeg: Self = Self(1i32);
}
impl ::core::marker::Copy for CoreWebView2CapturePreviewImageFormat {}
impl ::core::clone::Clone for CoreWebView2CapturePreviewImageFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2CapturePreviewImageFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CoreWebView2CapturePreviewImageFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for CoreWebView2CapturePreviewImageFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2CapturePreviewImageFormat")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2CapturePreviewImageFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2CapturePreviewImageFormat;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
pub struct CoreWebView2ClientCertificate(::windows::core::IUnknown);
impl CoreWebView2ClientCertificate {
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn ToCertificate(
        &self,
    ) -> ::windows::core::Result<::windows::Security::Cryptography::Certificates::Certificate> {
        let this = &::windows::core::Interface::cast::<CoreWebView2ClientCertificate_Manual>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ToCertificate)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Security::Cryptography::Certificates::Certificate>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Subject)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Issuer(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Issuer)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn ValidFrom(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ValidFrom)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn ValidTo(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ValidTo)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn DerEncodedSerialNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DerEncodedSerialNumber)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn PemEncodedIssuerCertificateChain(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IVectorView<::windows::core::HSTRING>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PemEncodedIssuerCertificateChain)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(
                result__,
            )
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Kind(&self) -> ::windows::core::Result<CoreWebView2ClientCertificateKind> {
        let this = self;
        unsafe {
            let mut result__: CoreWebView2ClientCertificateKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2ClientCertificateKind>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn ToPemEncoding(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ToPemEncoding)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreWebView2ClientCertificate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWebView2ClientCertificate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWebView2ClientCertificate {}
impl ::core::fmt::Debug for CoreWebView2ClientCertificate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2ClientCertificate")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2ClientCertificate {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2ClientCertificate;{091b39f2-68df-52b4-8fb0-fd3561af41f2})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreWebView2ClientCertificate {
    type Vtable = ICoreWebView2ClientCertificate_Vtbl;
    const IID: ::windows::core::GUID =
        <ICoreWebView2ClientCertificate as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreWebView2ClientCertificate {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2ClientCertificate";
}
impl ::core::convert::From<CoreWebView2ClientCertificate> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2ClientCertificate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2ClientCertificate> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2ClientCertificate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2ClientCertificate
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2ClientCertificate
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWebView2ClientCertificate> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2ClientCertificate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2ClientCertificate> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2ClientCertificate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2ClientCertificate
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2ClientCertificate
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreWebView2ClientCertificate {}
unsafe impl ::core::marker::Sync for CoreWebView2ClientCertificate {}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CoreWebView2ClientCertificateKind(pub i32);
impl CoreWebView2ClientCertificateKind {
    pub const SmartCard: Self = Self(0i32);
    pub const Pin: Self = Self(1i32);
    pub const Other: Self = Self(2i32);
}
impl ::core::marker::Copy for CoreWebView2ClientCertificateKind {}
impl ::core::clone::Clone for CoreWebView2ClientCertificateKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2ClientCertificateKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CoreWebView2ClientCertificateKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for CoreWebView2ClientCertificateKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2ClientCertificateKind")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2ClientCertificateKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2ClientCertificateKind;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
pub struct CoreWebView2ClientCertificateRequestedEventArgs(::windows::core::IUnknown);
impl CoreWebView2ClientCertificateRequestedEventArgs {
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Host(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Host)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Port(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Port)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn IsProxy(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsProxy)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn AllowedCertificateAuthorities(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IVectorView<::windows::core::HSTRING>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllowedCertificateAuthorities)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(
                result__,
            )
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn MutuallyTrustedCertificates(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IVectorView<CoreWebView2ClientCertificate>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MutuallyTrustedCertificates)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVectorView<
                CoreWebView2ClientCertificate,
            >>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SelectedCertificate(&self) -> ::windows::core::Result<CoreWebView2ClientCertificate> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SelectedCertificate)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2ClientCertificate>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetSelectedCertificate<
        'a,
        Param0: ::windows::core::IntoParam<'a, CoreWebView2ClientCertificate>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetSelectedCertificate)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Cancel(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Cancel)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetCancel(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetCancel)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Handled)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetHandled)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<::windows::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreWebView2ClientCertificateRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWebView2ClientCertificateRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWebView2ClientCertificateRequestedEventArgs {}
impl ::core::fmt::Debug for CoreWebView2ClientCertificateRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2ClientCertificateRequestedEventArgs")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2ClientCertificateRequestedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2ClientCertificateRequestedEventArgs;{93287b55-31f9-55a0-b68b-d9841d7e1bf4})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreWebView2ClientCertificateRequestedEventArgs {
    type Vtable = ICoreWebView2ClientCertificateRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        <ICoreWebView2ClientCertificateRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreWebView2ClientCertificateRequestedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2ClientCertificateRequestedEventArgs";
}
impl ::core::convert::From<CoreWebView2ClientCertificateRequestedEventArgs>
    for ::windows::core::IUnknown
{
    fn from(value: CoreWebView2ClientCertificateRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2ClientCertificateRequestedEventArgs>
    for ::windows::core::IUnknown
{
    fn from(value: &CoreWebView2ClientCertificateRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2ClientCertificateRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2ClientCertificateRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWebView2ClientCertificateRequestedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: CoreWebView2ClientCertificateRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2ClientCertificateRequestedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: &CoreWebView2ClientCertificateRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2ClientCertificateRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2ClientCertificateRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreWebView2ClientCertificateRequestedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2ClientCertificateRequestedEventArgs {}
#[doc(hidden)]
#[repr(transparent)]
pub struct CoreWebView2ClientCertificate_Manual(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for CoreWebView2ClientCertificate_Manual {
    type Vtable = CoreWebView2ClientCertificate_Manual_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xfaefefc2_20c3_5d86_8a74_f6d87d6ff8fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct CoreWebView2ClientCertificate_Manual_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ToCertificate: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
pub struct CoreWebView2CompositionController(::windows::core::IUnknown);
impl CoreWebView2CompositionController {
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RootVisualTarget(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RootVisualTarget)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetRootVisualTarget<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetRootVisualTarget)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).CursorChanged)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RemoveCursorChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveCursorChanged)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).SendMouseInput)(
                ::core::mem::transmute_copy(this),
                eventkind,
                virtualkeys,
                mousedata,
                point.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SendPointerInput<'a, Param1: ::windows::core::IntoParam<'a, CoreWebView2PointerInfo>>(
        &self,
        eventkind: CoreWebView2PointerEventKind,
        pointerinfo: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SendPointerInput)(
                ::core::mem::transmute_copy(this),
                eventkind,
                pointerinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Cursor(&self) -> ::windows::core::Result<::windows::UI::Core::CoreCursor> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Cursor)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreCursor>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn IsVisible(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsVisible)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetIsVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsVisible)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Bounds(&self) -> ::windows::core::Result<::windows::Foundation::Rect> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Bounds)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetBounds<'a, Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Rect>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetBounds)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn ZoomFactor(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ZoomFactor)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetZoomFactor(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetZoomFactor)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn ParentWindow(&self) -> ::windows::core::Result<CoreWebView2ControllerWindowReference> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ParentWindow)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2ControllerWindowReference>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetParentWindow<
        'a,
        Param0: ::windows::core::IntoParam<'a, CoreWebView2ControllerWindowReference>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetParentWindow)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn CoreWebView2(&self) -> ::windows::core::Result<CoreWebView2> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CoreWebView2)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).ZoomFactorChanged)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RemoveZoomFactorChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveZoomFactorChanged)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).MoveFocusRequested)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RemoveMoveFocusRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveMoveFocusRequested)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).GotFocus)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RemoveGotFocus<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveGotFocus)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).LostFocus)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RemoveLostFocus<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveLostFocus)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).AcceleratorKeyPressed)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RemoveAcceleratorKeyPressed<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAcceleratorKeyPressed)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).SetBoundsAndZoomFactor)(
                ::core::mem::transmute_copy(this),
                bounds.into_param().abi(),
                zoomfactor,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn MoveFocus(&self, reason: CoreWebView2MoveFocusReason) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).MoveFocus)(
                ::core::mem::transmute_copy(this),
                reason,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn NotifyParentWindowPositionChanged(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).NotifyParentWindowPositionChanged)(
                ::core::mem::transmute_copy(this),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn DefaultBackgroundColor(&self) -> ::windows::core::Result<::windows::UI::Color> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller2>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DefaultBackgroundColor)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetDefaultBackgroundColor<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::UI::Color>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetDefaultBackgroundColor)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RasterizationScale(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RasterizationScale)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetRasterizationScale(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetRasterizationScale)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn ShouldDetectMonitorScaleChanges(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShouldDetectMonitorScaleChanges)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetShouldDetectMonitorScaleChanges(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetShouldDetectMonitorScaleChanges)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn BoundsMode(&self) -> ::windows::core::Result<CoreWebView2BoundsMode> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            let mut result__: CoreWebView2BoundsMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BoundsMode)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2BoundsMode>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetBoundsMode(&self, value: CoreWebView2BoundsMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetBoundsMode)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).RasterizationScaleChanged)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RemoveRasterizationScaleChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveRasterizationScaleChanged)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for CoreWebView2CompositionController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWebView2CompositionController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWebView2CompositionController {}
impl ::core::fmt::Debug for CoreWebView2CompositionController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2CompositionController")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2CompositionController {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2CompositionController;{4fb8b7b3-4a2e-5787-94b9-cc48c4d364d7})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreWebView2CompositionController {
    type Vtable = ICoreWebView2CompositionController_Vtbl;
    const IID: ::windows::core::GUID =
        <ICoreWebView2CompositionController as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreWebView2CompositionController {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2CompositionController";
}
impl ::core::convert::From<CoreWebView2CompositionController> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2CompositionController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2CompositionController> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2CompositionController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2CompositionController
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2CompositionController
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWebView2CompositionController> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2CompositionController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2CompositionController> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2CompositionController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2CompositionController
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2CompositionController
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWebView2CompositionController> for CoreWebView2Controller {
    fn from(value: CoreWebView2CompositionController) -> Self {
        ::core::convert::From::from(&value)
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
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, CoreWebView2Controller>
    for &CoreWebView2CompositionController
{
    fn into_param(self) -> ::windows::core::Param<'a, CoreWebView2Controller> {
        ::windows::core::Param::Owned(::core::convert::Into::<CoreWebView2Controller>::into(self))
    }
}
unsafe impl ::core::marker::Send for CoreWebView2CompositionController {}
unsafe impl ::core::marker::Sync for CoreWebView2CompositionController {}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
pub struct CoreWebView2ContentLoadingEventArgs(::windows::core::IUnknown);
impl CoreWebView2ContentLoadingEventArgs {
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn IsErrorPage(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsErrorPage)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn NavigationId(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NavigationId)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u64>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreWebView2ContentLoadingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWebView2ContentLoadingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWebView2ContentLoadingEventArgs {}
impl ::core::fmt::Debug for CoreWebView2ContentLoadingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2ContentLoadingEventArgs")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2ContentLoadingEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2ContentLoadingEventArgs;{6cf95373-946c-5dae-9b3e-0fe23d5aa29f})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreWebView2ContentLoadingEventArgs {
    type Vtable = ICoreWebView2ContentLoadingEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        <ICoreWebView2ContentLoadingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreWebView2ContentLoadingEventArgs {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2ContentLoadingEventArgs";
}
impl ::core::convert::From<CoreWebView2ContentLoadingEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2ContentLoadingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2ContentLoadingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2ContentLoadingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2ContentLoadingEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2ContentLoadingEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWebView2ContentLoadingEventArgs> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2ContentLoadingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2ContentLoadingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2ContentLoadingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2ContentLoadingEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2ContentLoadingEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreWebView2ContentLoadingEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2ContentLoadingEventArgs {}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
pub struct CoreWebView2Controller(::windows::core::IUnknown);
impl CoreWebView2Controller {
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn IsVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsVisible)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetIsVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsVisible)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Bounds(&self) -> ::windows::core::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Bounds)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetBounds<'a, Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Rect>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetBounds)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn ZoomFactor(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ZoomFactor)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetZoomFactor(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetZoomFactor)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn ParentWindow(&self) -> ::windows::core::Result<CoreWebView2ControllerWindowReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ParentWindow)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2ControllerWindowReference>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetParentWindow<
        'a,
        Param0: ::windows::core::IntoParam<'a, CoreWebView2ControllerWindowReference>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetParentWindow)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn CoreWebView2(&self) -> ::windows::core::Result<CoreWebView2> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CoreWebView2)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).ZoomFactorChanged)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RemoveZoomFactorChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveZoomFactorChanged)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).MoveFocusRequested)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RemoveMoveFocusRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveMoveFocusRequested)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).GotFocus)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RemoveGotFocus<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveGotFocus)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).LostFocus)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RemoveLostFocus<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveLostFocus)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).AcceleratorKeyPressed)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RemoveAcceleratorKeyPressed<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAcceleratorKeyPressed)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).SetBoundsAndZoomFactor)(
                ::core::mem::transmute_copy(this),
                bounds.into_param().abi(),
                zoomfactor,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn MoveFocus(&self, reason: CoreWebView2MoveFocusReason) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).MoveFocus)(
                ::core::mem::transmute_copy(this),
                reason,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn NotifyParentWindowPositionChanged(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).NotifyParentWindowPositionChanged)(
                ::core::mem::transmute_copy(this),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn DefaultBackgroundColor(&self) -> ::windows::core::Result<::windows::UI::Color> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller2>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DefaultBackgroundColor)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetDefaultBackgroundColor<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::UI::Color>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetDefaultBackgroundColor)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RasterizationScale(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RasterizationScale)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetRasterizationScale(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetRasterizationScale)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn ShouldDetectMonitorScaleChanges(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShouldDetectMonitorScaleChanges)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetShouldDetectMonitorScaleChanges(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetShouldDetectMonitorScaleChanges)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn BoundsMode(&self) -> ::windows::core::Result<CoreWebView2BoundsMode> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            let mut result__: CoreWebView2BoundsMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BoundsMode)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2BoundsMode>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetBoundsMode(&self, value: CoreWebView2BoundsMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetBoundsMode)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).RasterizationScaleChanged)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RemoveRasterizationScaleChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveRasterizationScaleChanged)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for CoreWebView2Controller {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWebView2Controller {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWebView2Controller {}
impl ::core::fmt::Debug for CoreWebView2Controller {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2Controller")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2Controller {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2Controller;{a588121c-53bf-590e-80e5-29d729cbd743})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreWebView2Controller {
    type Vtable = ICoreWebView2Controller_Vtbl;
    const IID: ::windows::core::GUID = <ICoreWebView2Controller as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreWebView2Controller {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2Controller";
}
impl ::core::convert::From<CoreWebView2Controller> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2Controller) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2Controller> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2Controller) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreWebView2Controller {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreWebView2Controller {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWebView2Controller> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2Controller) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2Controller> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2Controller) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreWebView2Controller {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2Controller
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreWebView2Controller {}
unsafe impl ::core::marker::Sync for CoreWebView2Controller {}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
pub struct CoreWebView2ControllerWindowReference(::windows::core::IUnknown);
impl CoreWebView2ControllerWindowReference {
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn WindowHandle(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WindowHandle)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn CoreWindow(&self) -> ::windows::core::Result<::windows::UI::Core::CoreWindow> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CoreWindow)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreWindow>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn CreateFromWindowHandle(
        windowhandle: u64,
    ) -> ::windows::core::Result<CoreWebView2ControllerWindowReference> {
        Self::ICoreWebView2ControllerWindowReferenceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromWindowHandle)(
                ::core::mem::transmute_copy(this),
                windowhandle,
                &mut result__,
            )
            .from_abi::<CoreWebView2ControllerWindowReference>(result__)
        })
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn CreateFromCoreWindow<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::UI::Core::CoreWindow>,
    >(
        corewindow: Param0,
    ) -> ::windows::core::Result<CoreWebView2ControllerWindowReference> {
        Self::ICoreWebView2ControllerWindowReferenceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromCoreWindow)(
                ::core::mem::transmute_copy(this),
                corewindow.into_param().abi(),
                &mut result__,
            )
            .from_abi::<CoreWebView2ControllerWindowReference>(result__)
        })
    }
    #[doc(hidden)]
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
impl ::core::clone::Clone for CoreWebView2ControllerWindowReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWebView2ControllerWindowReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWebView2ControllerWindowReference {}
impl ::core::fmt::Debug for CoreWebView2ControllerWindowReference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2ControllerWindowReference")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2ControllerWindowReference {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2ControllerWindowReference;{0feddad4-48a3-5cc4-9f61-e7adfd1e9c76})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreWebView2ControllerWindowReference {
    type Vtable = ICoreWebView2ControllerWindowReference_Vtbl;
    const IID: ::windows::core::GUID =
        <ICoreWebView2ControllerWindowReference as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreWebView2ControllerWindowReference {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2ControllerWindowReference";
}
impl ::core::convert::From<CoreWebView2ControllerWindowReference> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2ControllerWindowReference) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2ControllerWindowReference> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2ControllerWindowReference) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2ControllerWindowReference
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2ControllerWindowReference
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWebView2ControllerWindowReference>
    for ::windows::core::IInspectable
{
    fn from(value: CoreWebView2ControllerWindowReference) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2ControllerWindowReference>
    for ::windows::core::IInspectable
{
    fn from(value: &CoreWebView2ControllerWindowReference) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2ControllerWindowReference
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2ControllerWindowReference
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreWebView2ControllerWindowReference {}
unsafe impl ::core::marker::Sync for CoreWebView2ControllerWindowReference {}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
pub struct CoreWebView2Cookie(::windows::core::IUnknown);
impl CoreWebView2Cookie {
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Value)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetValue)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Domain(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Domain)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Path)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Expires(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Expires)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetExpires(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetExpires)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn IsHttpOnly(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsHttpOnly)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetIsHttpOnly(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsHttpOnly)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SameSite(&self) -> ::windows::core::Result<CoreWebView2CookieSameSiteKind> {
        let this = self;
        unsafe {
            let mut result__: CoreWebView2CookieSameSiteKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SameSite)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2CookieSameSiteKind>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetSameSite(
        &self,
        value: CoreWebView2CookieSameSiteKind,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetSameSite)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn IsSecure(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSecure)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetIsSecure(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsSecure)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn IsSession(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSession)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreWebView2Cookie {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWebView2Cookie {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWebView2Cookie {}
impl ::core::fmt::Debug for CoreWebView2Cookie {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2Cookie").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2Cookie {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2Cookie;{52f670fe-8ca2-5aad-aedb-25f7903b7038})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreWebView2Cookie {
    type Vtable = ICoreWebView2Cookie_Vtbl;
    const IID: ::windows::core::GUID = <ICoreWebView2Cookie as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreWebView2Cookie {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2Cookie";
}
impl ::core::convert::From<CoreWebView2Cookie> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2Cookie) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2Cookie> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2Cookie) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreWebView2Cookie {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreWebView2Cookie {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWebView2Cookie> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2Cookie) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2Cookie> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2Cookie) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreWebView2Cookie {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreWebView2Cookie {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreWebView2Cookie {}
unsafe impl ::core::marker::Sync for CoreWebView2Cookie {}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
pub struct CoreWebView2CookieManager(::windows::core::IUnknown);
impl CoreWebView2CookieManager {
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).CreateCookie)(
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
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn CopyCookie<'a, Param0: ::windows::core::IntoParam<'a, CoreWebView2Cookie>>(
        &self,
        cookieparam: Param0,
    ) -> ::windows::core::Result<CoreWebView2Cookie> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CopyCookie)(
                ::core::mem::transmute_copy(this),
                cookieparam.into_param().abi(),
                &mut result__,
            )
            .from_abi::<CoreWebView2Cookie>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn AddOrUpdateCookie<'a, Param0: ::windows::core::IntoParam<'a, CoreWebView2Cookie>>(
        &self,
        cookie: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).AddOrUpdateCookie)(
                ::core::mem::transmute_copy(this),
                cookie.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn DeleteCookie<'a, Param0: ::windows::core::IntoParam<'a, CoreWebView2Cookie>>(
        &self,
        cookie: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).DeleteCookie)(
                ::core::mem::transmute_copy(this),
                cookie.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).DeleteCookies)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                uri.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).DeleteCookiesWithDomainAndPath)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                domain.into_param().abi(),
                path.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn DeleteAllCookies(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).DeleteAllCookies)(
                ::core::mem::transmute_copy(this),
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for CoreWebView2CookieManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWebView2CookieManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWebView2CookieManager {}
impl ::core::fmt::Debug for CoreWebView2CookieManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2CookieManager")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2CookieManager {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2CookieManager;{4098f516-adca-5563-aaa5-d7affd847aa3})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreWebView2CookieManager {
    type Vtable = ICoreWebView2CookieManager_Vtbl;
    const IID: ::windows::core::GUID =
        <ICoreWebView2CookieManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreWebView2CookieManager {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2CookieManager";
}
impl ::core::convert::From<CoreWebView2CookieManager> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2CookieManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2CookieManager> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2CookieManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreWebView2CookieManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2CookieManager
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWebView2CookieManager> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2CookieManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2CookieManager> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2CookieManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2CookieManager
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2CookieManager
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreWebView2CookieManager {}
unsafe impl ::core::marker::Sync for CoreWebView2CookieManager {}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CoreWebView2CookieSameSiteKind(pub i32);
impl CoreWebView2CookieSameSiteKind {
    pub const None: Self = Self(0i32);
    pub const Lax: Self = Self(1i32);
    pub const Strict: Self = Self(2i32);
}
impl ::core::marker::Copy for CoreWebView2CookieSameSiteKind {}
impl ::core::clone::Clone for CoreWebView2CookieSameSiteKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2CookieSameSiteKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CoreWebView2CookieSameSiteKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for CoreWebView2CookieSameSiteKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2CookieSameSiteKind")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2CookieSameSiteKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2CookieSameSiteKind;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
pub struct CoreWebView2DOMContentLoadedEventArgs(::windows::core::IUnknown);
impl CoreWebView2DOMContentLoadedEventArgs {
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn NavigationId(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NavigationId)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u64>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreWebView2DOMContentLoadedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWebView2DOMContentLoadedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWebView2DOMContentLoadedEventArgs {}
impl ::core::fmt::Debug for CoreWebView2DOMContentLoadedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2DOMContentLoadedEventArgs")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2DOMContentLoadedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2DOMContentLoadedEventArgs;{c474d0a3-24ac-59fc-b78b-da7562a6a052})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreWebView2DOMContentLoadedEventArgs {
    type Vtable = ICoreWebView2DOMContentLoadedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        <ICoreWebView2DOMContentLoadedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreWebView2DOMContentLoadedEventArgs {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2DOMContentLoadedEventArgs";
}
impl ::core::convert::From<CoreWebView2DOMContentLoadedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2DOMContentLoadedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2DOMContentLoadedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2DOMContentLoadedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2DOMContentLoadedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2DOMContentLoadedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWebView2DOMContentLoadedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: CoreWebView2DOMContentLoadedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2DOMContentLoadedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: &CoreWebView2DOMContentLoadedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2DOMContentLoadedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2DOMContentLoadedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreWebView2DOMContentLoadedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2DOMContentLoadedEventArgs {}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
pub struct CoreWebView2DevToolsProtocolEventReceivedEventArgs(::windows::core::IUnknown);
impl CoreWebView2DevToolsProtocolEventReceivedEventArgs {
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn ParameterObjectAsJson(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ParameterObjectAsJson)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreWebView2DevToolsProtocolEventReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWebView2DevToolsProtocolEventReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWebView2DevToolsProtocolEventReceivedEventArgs {}
impl ::core::fmt::Debug for CoreWebView2DevToolsProtocolEventReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2DevToolsProtocolEventReceivedEventArgs")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2DevToolsProtocolEventReceivedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2DevToolsProtocolEventReceivedEventArgs;{b6a4b41d-fd18-59fa-923a-c57555d960ce})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreWebView2DevToolsProtocolEventReceivedEventArgs {
    type Vtable = ICoreWebView2DevToolsProtocolEventReceivedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        <ICoreWebView2DevToolsProtocolEventReceivedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreWebView2DevToolsProtocolEventReceivedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2DevToolsProtocolEventReceivedEventArgs";
}
impl ::core::convert::From<CoreWebView2DevToolsProtocolEventReceivedEventArgs>
    for ::windows::core::IUnknown
{
    fn from(value: CoreWebView2DevToolsProtocolEventReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2DevToolsProtocolEventReceivedEventArgs>
    for ::windows::core::IUnknown
{
    fn from(value: &CoreWebView2DevToolsProtocolEventReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2DevToolsProtocolEventReceivedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2DevToolsProtocolEventReceivedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWebView2DevToolsProtocolEventReceivedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: CoreWebView2DevToolsProtocolEventReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2DevToolsProtocolEventReceivedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: &CoreWebView2DevToolsProtocolEventReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2DevToolsProtocolEventReceivedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2DevToolsProtocolEventReceivedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreWebView2DevToolsProtocolEventReceivedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2DevToolsProtocolEventReceivedEventArgs {}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
pub struct CoreWebView2DevToolsProtocolEventReceiver(::windows::core::IUnknown);
impl CoreWebView2DevToolsProtocolEventReceiver {
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).DevToolsProtocolEventReceived)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RemoveDevToolsProtocolEventReceived<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveDevToolsProtocolEventReceived)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for CoreWebView2DevToolsProtocolEventReceiver {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWebView2DevToolsProtocolEventReceiver {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWebView2DevToolsProtocolEventReceiver {}
impl ::core::fmt::Debug for CoreWebView2DevToolsProtocolEventReceiver {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2DevToolsProtocolEventReceiver")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2DevToolsProtocolEventReceiver {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2DevToolsProtocolEventReceiver;{b2a2be79-65fc-5537-8715-3d92bf31090b})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreWebView2DevToolsProtocolEventReceiver {
    type Vtable = ICoreWebView2DevToolsProtocolEventReceiver_Vtbl;
    const IID: ::windows::core::GUID =
        <ICoreWebView2DevToolsProtocolEventReceiver as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreWebView2DevToolsProtocolEventReceiver {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2DevToolsProtocolEventReceiver";
}
impl ::core::convert::From<CoreWebView2DevToolsProtocolEventReceiver>
    for ::windows::core::IUnknown
{
    fn from(value: CoreWebView2DevToolsProtocolEventReceiver) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2DevToolsProtocolEventReceiver>
    for ::windows::core::IUnknown
{
    fn from(value: &CoreWebView2DevToolsProtocolEventReceiver) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2DevToolsProtocolEventReceiver
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2DevToolsProtocolEventReceiver
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWebView2DevToolsProtocolEventReceiver>
    for ::windows::core::IInspectable
{
    fn from(value: CoreWebView2DevToolsProtocolEventReceiver) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2DevToolsProtocolEventReceiver>
    for ::windows::core::IInspectable
{
    fn from(value: &CoreWebView2DevToolsProtocolEventReceiver) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2DevToolsProtocolEventReceiver
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2DevToolsProtocolEventReceiver
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreWebView2DevToolsProtocolEventReceiver {}
unsafe impl ::core::marker::Sync for CoreWebView2DevToolsProtocolEventReceiver {}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CoreWebView2DownloadInterruptReason(pub i32);
impl CoreWebView2DownloadInterruptReason {
    pub const None: Self = Self(0i32);
    pub const FileFailed: Self = Self(1i32);
    pub const FileAccessDenied: Self = Self(2i32);
    pub const FileNoSpace: Self = Self(3i32);
    pub const FileNameTooLong: Self = Self(4i32);
    pub const FileTooLarge: Self = Self(5i32);
    pub const FileMalicious: Self = Self(6i32);
    pub const FileTransientError: Self = Self(7i32);
    pub const FileBlockedByPolicy: Self = Self(8i32);
    pub const FileSecurityCheckFailed: Self = Self(9i32);
    pub const FileTooShort: Self = Self(10i32);
    pub const FileHashMismatch: Self = Self(11i32);
    pub const NetworkFailed: Self = Self(12i32);
    pub const NetworkTimeout: Self = Self(13i32);
    pub const NetworkDisconnected: Self = Self(14i32);
    pub const NetworkServerDown: Self = Self(15i32);
    pub const NetworkInvalidRequest: Self = Self(16i32);
    pub const ServerFailed: Self = Self(17i32);
    pub const ServerNoRange: Self = Self(18i32);
    pub const ServerBadContent: Self = Self(19i32);
    pub const ServerUnauthorized: Self = Self(20i32);
    pub const ServerCertificateProblem: Self = Self(21i32);
    pub const ServerForbidden: Self = Self(22i32);
    pub const ServerUnexpectedResponse: Self = Self(23i32);
    pub const ServerContentLengthMismatch: Self = Self(24i32);
    pub const ServerCrossOriginRedirect: Self = Self(25i32);
    pub const UserCanceled: Self = Self(26i32);
    pub const UserShutdown: Self = Self(27i32);
    pub const UserPaused: Self = Self(28i32);
    pub const DownloadProcessCrashed: Self = Self(29i32);
}
impl ::core::marker::Copy for CoreWebView2DownloadInterruptReason {}
impl ::core::clone::Clone for CoreWebView2DownloadInterruptReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2DownloadInterruptReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CoreWebView2DownloadInterruptReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for CoreWebView2DownloadInterruptReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2DownloadInterruptReason")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2DownloadInterruptReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2DownloadInterruptReason;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
pub struct CoreWebView2DownloadOperation(::windows::core::IUnknown);
impl CoreWebView2DownloadOperation {
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Uri(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Uri)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn ContentDisposition(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentDisposition)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn MimeType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MimeType)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn TotalBytesToReceive(&self) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TotalBytesToReceive)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn BytesReceived(&self) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BytesReceived)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn EstimatedEndTime(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EstimatedEndTime)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn ResultFilePath(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResultFilePath)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn State(&self) -> ::windows::core::Result<CoreWebView2DownloadState> {
        let this = self;
        unsafe {
            let mut result__: CoreWebView2DownloadState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).State)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2DownloadState>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn InterruptReason(&self) -> ::windows::core::Result<CoreWebView2DownloadInterruptReason> {
        let this = self;
        unsafe {
            let mut result__: CoreWebView2DownloadInterruptReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InterruptReason)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2DownloadInterruptReason>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn CanResume(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanResume)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).BytesReceivedChanged)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RemoveBytesReceivedChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveBytesReceivedChanged)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).EstimatedEndTimeChanged)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RemoveEstimatedEndTimeChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveEstimatedEndTimeChanged)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).StateChanged)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Cancel(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Cancel)(::core::mem::transmute_copy(this))
                .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Pause(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Pause)(::core::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Resume(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Resume)(::core::mem::transmute_copy(this))
                .ok()
        }
    }
}
impl ::core::clone::Clone for CoreWebView2DownloadOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWebView2DownloadOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWebView2DownloadOperation {}
impl ::core::fmt::Debug for CoreWebView2DownloadOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2DownloadOperation")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2DownloadOperation {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2DownloadOperation;{afe73e6b-e760-5a06-9bf6-1e743c13cd2d})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreWebView2DownloadOperation {
    type Vtable = ICoreWebView2DownloadOperation_Vtbl;
    const IID: ::windows::core::GUID =
        <ICoreWebView2DownloadOperation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreWebView2DownloadOperation {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2DownloadOperation";
}
impl ::core::convert::From<CoreWebView2DownloadOperation> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2DownloadOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2DownloadOperation> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2DownloadOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2DownloadOperation
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2DownloadOperation
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWebView2DownloadOperation> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2DownloadOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2DownloadOperation> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2DownloadOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2DownloadOperation
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2DownloadOperation
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreWebView2DownloadOperation {}
unsafe impl ::core::marker::Sync for CoreWebView2DownloadOperation {}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
pub struct CoreWebView2DownloadStartingEventArgs(::windows::core::IUnknown);
impl CoreWebView2DownloadStartingEventArgs {
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn DownloadOperation(&self) -> ::windows::core::Result<CoreWebView2DownloadOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DownloadOperation)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2DownloadOperation>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Cancel(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Cancel)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetCancel(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetCancel)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn ResultFilePath(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResultFilePath)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetResultFilePath<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetResultFilePath)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Handled)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetHandled)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<::windows::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreWebView2DownloadStartingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWebView2DownloadStartingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWebView2DownloadStartingEventArgs {}
impl ::core::fmt::Debug for CoreWebView2DownloadStartingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2DownloadStartingEventArgs")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2DownloadStartingEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2DownloadStartingEventArgs;{45d982ba-9256-5b35-b023-26a438599110})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreWebView2DownloadStartingEventArgs {
    type Vtable = ICoreWebView2DownloadStartingEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        <ICoreWebView2DownloadStartingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreWebView2DownloadStartingEventArgs {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2DownloadStartingEventArgs";
}
impl ::core::convert::From<CoreWebView2DownloadStartingEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2DownloadStartingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2DownloadStartingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2DownloadStartingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2DownloadStartingEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2DownloadStartingEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWebView2DownloadStartingEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: CoreWebView2DownloadStartingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2DownloadStartingEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: &CoreWebView2DownloadStartingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2DownloadStartingEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2DownloadStartingEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreWebView2DownloadStartingEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2DownloadStartingEventArgs {}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CoreWebView2DownloadState(pub i32);
impl CoreWebView2DownloadState {
    pub const InProgress: Self = Self(0i32);
    pub const Interrupted: Self = Self(1i32);
    pub const Completed: Self = Self(2i32);
}
impl ::core::marker::Copy for CoreWebView2DownloadState {}
impl ::core::clone::Clone for CoreWebView2DownloadState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2DownloadState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CoreWebView2DownloadState {
    type Abi = Self;
}
impl ::core::fmt::Debug for CoreWebView2DownloadState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2DownloadState")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2DownloadState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2DownloadState;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
pub struct CoreWebView2Environment(::windows::core::IUnknown);
impl CoreWebView2Environment {
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn BrowserVersionString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BrowserVersionString)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).NewBrowserVersionAvailable)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RemoveNewBrowserVersionAvailable<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveNewBrowserVersionAvailable)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).CreateCoreWebView2ControllerAsync)(
                ::core::mem::transmute_copy(this),
                parentwindow.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IAsyncOperation<CoreWebView2Controller>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).CreateWebResourceResponse)(
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
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).CreateWebResourceRequest)(
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
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).CreateCoreWebView2CompositionControllerAsync)(
                ::core::mem::transmute_copy(this),
                parentwindow.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IAsyncOperation<CoreWebView2CompositionController>>(
                result__,
            )
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn CreateCoreWebView2PointerInfo(
        &self,
    ) -> ::windows::core::Result<CoreWebView2PointerInfo> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Environment3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateCoreWebView2PointerInfo)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2PointerInfo>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).BrowserProcessExited)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RemoveBrowserProcessExited<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Environment5>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveBrowserProcessExited)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn CreatePrintSettings(&self) -> ::windows::core::Result<CoreWebView2PrintSettings> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Environment6>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreatePrintSettings)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2PrintSettings>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn CreateAsync(
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncOperation<CoreWebView2Environment>>
    {
        Self::ICoreWebView2EnvironmentStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateAsync)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IAsyncOperation<CoreWebView2Environment>>(result__)
        })
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).CreateWithOptionsAsync)(
                ::core::mem::transmute_copy(this),
                browserexecutablefolder.into_param().abi(),
                userdatafolder.into_param().abi(),
                options.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IAsyncOperation<CoreWebView2Environment>>(result__)
        })
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn GetAvailableBrowserVersionString() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICoreWebView2EnvironmentStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAvailableBrowserVersionString)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn GetAvailableBrowserVersionString2<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        browserexecutablefolder: Param0,
    ) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICoreWebView2EnvironmentStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAvailableBrowserVersionString2)(
                ::core::mem::transmute_copy(this),
                browserexecutablefolder.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).CompareBrowserVersionString)(
                ::core::mem::transmute_copy(this),
                browserversionstring1.into_param().abi(),
                browserversionstring2.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        })
    }
    #[doc(hidden)]
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
impl ::core::clone::Clone for CoreWebView2Environment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWebView2Environment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWebView2Environment {}
impl ::core::fmt::Debug for CoreWebView2Environment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2Environment")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2Environment {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2Environment;{d8cc7831-b783-556b-b9ce-899c1e95d585})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreWebView2Environment {
    type Vtable = ICoreWebView2Environment_Vtbl;
    const IID: ::windows::core::GUID =
        <ICoreWebView2Environment as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreWebView2Environment {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2Environment";
}
impl ::core::convert::From<CoreWebView2Environment> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2Environment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2Environment> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2Environment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreWebView2Environment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreWebView2Environment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWebView2Environment> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2Environment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2Environment> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2Environment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreWebView2Environment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2Environment
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreWebView2Environment {}
unsafe impl ::core::marker::Sync for CoreWebView2Environment {}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
pub struct CoreWebView2EnvironmentOptions(::windows::core::IUnknown);
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
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn AdditionalBrowserArguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AdditionalBrowserArguments)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetAdditionalBrowserArguments<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAdditionalBrowserArguments)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Language)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetLanguage)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn TargetCompatibleBrowserVersion(
        &self,
    ) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TargetCompatibleBrowserVersion)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetTargetCompatibleBrowserVersion<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetTargetCompatibleBrowserVersion)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn AllowSingleSignOnUsingOSPrimaryAccount(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllowSingleSignOnUsingOSPrimaryAccount)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetAllowSingleSignOnUsingOSPrimaryAccount(
        &self,
        value: bool,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAllowSingleSignOnUsingOSPrimaryAccount)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for CoreWebView2EnvironmentOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWebView2EnvironmentOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWebView2EnvironmentOptions {}
impl ::core::fmt::Debug for CoreWebView2EnvironmentOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2EnvironmentOptions")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2EnvironmentOptions {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2EnvironmentOptions;{25d6dc39-0062-5735-8b09-a6f535f19e97})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreWebView2EnvironmentOptions {
    type Vtable = ICoreWebView2EnvironmentOptions_Vtbl;
    const IID: ::windows::core::GUID =
        <ICoreWebView2EnvironmentOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreWebView2EnvironmentOptions {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2EnvironmentOptions";
}
impl ::core::convert::From<CoreWebView2EnvironmentOptions> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2EnvironmentOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2EnvironmentOptions> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2EnvironmentOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2EnvironmentOptions
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2EnvironmentOptions
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWebView2EnvironmentOptions> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2EnvironmentOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2EnvironmentOptions> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2EnvironmentOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2EnvironmentOptions
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2EnvironmentOptions
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreWebView2EnvironmentOptions {}
unsafe impl ::core::marker::Sync for CoreWebView2EnvironmentOptions {}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
pub struct CoreWebView2Frame(::windows::core::IUnknown);
impl CoreWebView2Frame {
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).NameChanged)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RemoveNameChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveNameChanged)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).Destroyed)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RemoveDestroyed<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveDestroyed)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RemoveHostObjectFromScript<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveHostObjectFromScript)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn IsDestroyed(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDestroyed)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreWebView2Frame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWebView2Frame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWebView2Frame {}
impl ::core::fmt::Debug for CoreWebView2Frame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2Frame").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2Frame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Web.WebView2.Core.CoreWebView2Frame;{02ffcbf9-19e7-5bb8-8273-346420fb1503})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreWebView2Frame {
    type Vtable = ICoreWebView2Frame_Vtbl;
    const IID: ::windows::core::GUID = <ICoreWebView2Frame as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreWebView2Frame {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2Frame";
}
impl ::core::convert::From<CoreWebView2Frame> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2Frame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2Frame> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2Frame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreWebView2Frame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreWebView2Frame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWebView2Frame> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2Frame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2Frame> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2Frame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreWebView2Frame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreWebView2Frame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreWebView2Frame {}
unsafe impl ::core::marker::Sync for CoreWebView2Frame {}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
pub struct CoreWebView2FrameCreatedEventArgs(::windows::core::IUnknown);
impl CoreWebView2FrameCreatedEventArgs {
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Frame(&self) -> ::windows::core::Result<CoreWebView2Frame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Frame)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2Frame>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreWebView2FrameCreatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWebView2FrameCreatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWebView2FrameCreatedEventArgs {}
impl ::core::fmt::Debug for CoreWebView2FrameCreatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2FrameCreatedEventArgs")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2FrameCreatedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2FrameCreatedEventArgs;{527b01b8-fc6d-5543-8dce-96cdfdb32c81})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreWebView2FrameCreatedEventArgs {
    type Vtable = ICoreWebView2FrameCreatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        <ICoreWebView2FrameCreatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreWebView2FrameCreatedEventArgs {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2FrameCreatedEventArgs";
}
impl ::core::convert::From<CoreWebView2FrameCreatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2FrameCreatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2FrameCreatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2FrameCreatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2FrameCreatedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2FrameCreatedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWebView2FrameCreatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2FrameCreatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2FrameCreatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2FrameCreatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2FrameCreatedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2FrameCreatedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreWebView2FrameCreatedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2FrameCreatedEventArgs {}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
pub struct CoreWebView2FrameInfo(::windows::core::IUnknown);
impl CoreWebView2FrameInfo {
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Source(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Source)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreWebView2FrameInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWebView2FrameInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWebView2FrameInfo {}
impl ::core::fmt::Debug for CoreWebView2FrameInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2FrameInfo")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2FrameInfo {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2FrameInfo;{f9b82e06-73f3-513b-bc2c-445ddedba976})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreWebView2FrameInfo {
    type Vtable = ICoreWebView2FrameInfo_Vtbl;
    const IID: ::windows::core::GUID = <ICoreWebView2FrameInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreWebView2FrameInfo {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2FrameInfo";
}
impl ::core::convert::From<CoreWebView2FrameInfo> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2FrameInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2FrameInfo> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2FrameInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreWebView2FrameInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreWebView2FrameInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWebView2FrameInfo> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2FrameInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2FrameInfo> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2FrameInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreWebView2FrameInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2FrameInfo
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreWebView2FrameInfo {}
unsafe impl ::core::marker::Sync for CoreWebView2FrameInfo {}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CoreWebView2HostResourceAccessKind(pub i32);
impl CoreWebView2HostResourceAccessKind {
    pub const Deny: Self = Self(0i32);
    pub const Allow: Self = Self(1i32);
    pub const DenyCors: Self = Self(2i32);
}
impl ::core::marker::Copy for CoreWebView2HostResourceAccessKind {}
impl ::core::clone::Clone for CoreWebView2HostResourceAccessKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2HostResourceAccessKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CoreWebView2HostResourceAccessKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for CoreWebView2HostResourceAccessKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2HostResourceAccessKind")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2HostResourceAccessKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2HostResourceAccessKind;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
pub struct CoreWebView2HttpHeadersCollectionIterator(::windows::core::IUnknown);
impl CoreWebView2HttpHeadersCollectionIterator {
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).Current)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IKeyValuePair<
                ::windows::core::HSTRING,
                ::windows::core::HSTRING,
            >>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).HasCurrent)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).MoveNext)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn GetMany(
        &self,
        items: &mut [::core::option::Option<
            ::windows::Foundation::Collections::IKeyValuePair<
                ::windows::core::HSTRING,
                ::windows::core::HSTRING,
            >,
        >],
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
            (::windows::core::Interface::vtable(this).GetMany)(
                ::core::mem::transmute_copy(this),
                items.len() as u32,
                ::core::mem::transmute_copy(&items),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreWebView2HttpHeadersCollectionIterator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWebView2HttpHeadersCollectionIterator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWebView2HttpHeadersCollectionIterator {}
impl ::core::fmt::Debug for CoreWebView2HttpHeadersCollectionIterator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2HttpHeadersCollectionIterator")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2HttpHeadersCollectionIterator {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2HttpHeadersCollectionIterator;{adf264ee-d980-5f48-a60e-8705de046608})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreWebView2HttpHeadersCollectionIterator {
    type Vtable = ICoreWebView2HttpHeadersCollectionIterator_Vtbl;
    const IID: ::windows::core::GUID =
        <ICoreWebView2HttpHeadersCollectionIterator as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreWebView2HttpHeadersCollectionIterator {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2HttpHeadersCollectionIterator";
}
impl ::core::convert::From<CoreWebView2HttpHeadersCollectionIterator>
    for ::windows::core::IUnknown
{
    fn from(value: CoreWebView2HttpHeadersCollectionIterator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2HttpHeadersCollectionIterator>
    for ::windows::core::IUnknown
{
    fn from(value: &CoreWebView2HttpHeadersCollectionIterator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2HttpHeadersCollectionIterator
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2HttpHeadersCollectionIterator
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWebView2HttpHeadersCollectionIterator>
    for ::windows::core::IInspectable
{
    fn from(value: CoreWebView2HttpHeadersCollectionIterator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2HttpHeadersCollectionIterator>
    for ::windows::core::IInspectable
{
    fn from(value: &CoreWebView2HttpHeadersCollectionIterator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2HttpHeadersCollectionIterator
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2HttpHeadersCollectionIterator
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
pub struct CoreWebView2HttpRequestHeaders(::windows::core::IUnknown);
impl CoreWebView2HttpRequestHeaders {
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn GetHeader<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetHeader)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn GetHeaders<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<CoreWebView2HttpHeadersCollectionIterator> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetHeaders)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<CoreWebView2HttpHeadersCollectionIterator>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Contains<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Contains)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).SetHeader)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RemoveHeader<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveHeader)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).First)(
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
impl ::core::clone::Clone for CoreWebView2HttpRequestHeaders {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWebView2HttpRequestHeaders {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWebView2HttpRequestHeaders {}
impl ::core::fmt::Debug for CoreWebView2HttpRequestHeaders {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2HttpRequestHeaders")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2HttpRequestHeaders {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2HttpRequestHeaders;{dc2226c7-3515-55bb-bcb2-57b78f86b91d})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreWebView2HttpRequestHeaders {
    type Vtable = ICoreWebView2HttpRequestHeaders_Vtbl;
    const IID: ::windows::core::GUID =
        <ICoreWebView2HttpRequestHeaders as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreWebView2HttpRequestHeaders {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2HttpRequestHeaders";
}
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
impl ::core::convert::From<CoreWebView2HttpRequestHeaders> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2HttpRequestHeaders) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2HttpRequestHeaders> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2HttpRequestHeaders) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2HttpRequestHeaders
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2HttpRequestHeaders
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWebView2HttpRequestHeaders> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2HttpRequestHeaders) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2HttpRequestHeaders> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2HttpRequestHeaders) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2HttpRequestHeaders
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2HttpRequestHeaders
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
pub struct CoreWebView2HttpResponseHeaders(::windows::core::IUnknown);
impl CoreWebView2HttpResponseHeaders {
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).AppendHeader)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Contains<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Contains)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn GetHeader<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetHeader)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn GetHeaders<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<CoreWebView2HttpHeadersCollectionIterator> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetHeaders)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<CoreWebView2HttpHeadersCollectionIterator>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            (::windows::core::Interface::vtable(this).First)(
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
impl ::core::clone::Clone for CoreWebView2HttpResponseHeaders {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWebView2HttpResponseHeaders {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWebView2HttpResponseHeaders {}
impl ::core::fmt::Debug for CoreWebView2HttpResponseHeaders {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2HttpResponseHeaders")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2HttpResponseHeaders {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2HttpResponseHeaders;{f3d383e9-747f-5574-8662-9a6b920cecd4})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreWebView2HttpResponseHeaders {
    type Vtable = ICoreWebView2HttpResponseHeaders_Vtbl;
    const IID: ::windows::core::GUID =
        <ICoreWebView2HttpResponseHeaders as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreWebView2HttpResponseHeaders {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2HttpResponseHeaders";
}
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
impl ::core::convert::From<CoreWebView2HttpResponseHeaders> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2HttpResponseHeaders) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2HttpResponseHeaders> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2HttpResponseHeaders) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2HttpResponseHeaders
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2HttpResponseHeaders
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWebView2HttpResponseHeaders> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2HttpResponseHeaders) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2HttpResponseHeaders> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2HttpResponseHeaders) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2HttpResponseHeaders
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2HttpResponseHeaders
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CoreWebView2KeyEventKind(pub i32);
impl CoreWebView2KeyEventKind {
    pub const KeyDown: Self = Self(0i32);
    pub const KeyUp: Self = Self(1i32);
    pub const SystemKeyDown: Self = Self(2i32);
    pub const SystemKeyUp: Self = Self(3i32);
}
impl ::core::marker::Copy for CoreWebView2KeyEventKind {}
impl ::core::clone::Clone for CoreWebView2KeyEventKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2KeyEventKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CoreWebView2KeyEventKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for CoreWebView2KeyEventKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2KeyEventKind")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2KeyEventKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2KeyEventKind;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CoreWebView2MouseEventKind(pub i32);
impl CoreWebView2MouseEventKind {
    pub const HorizontalWheel: Self = Self(526i32);
    pub const LeftButtonDoubleClick: Self = Self(515i32);
    pub const LeftButtonDown: Self = Self(513i32);
    pub const LeftButtonUp: Self = Self(514i32);
    pub const Leave: Self = Self(675i32);
    pub const MiddleButtonDoubleClick: Self = Self(521i32);
    pub const MiddleButtonDown: Self = Self(519i32);
    pub const MiddleButtonUp: Self = Self(520i32);
    pub const Move: Self = Self(512i32);
    pub const RightButtonDoubleClick: Self = Self(518i32);
    pub const RightButtonDown: Self = Self(516i32);
    pub const RightButtonUp: Self = Self(517i32);
    pub const Wheel: Self = Self(522i32);
    pub const XButtonDoubleClick: Self = Self(525i32);
    pub const XButtonDown: Self = Self(523i32);
    pub const XButtonUp: Self = Self(524i32);
}
impl ::core::marker::Copy for CoreWebView2MouseEventKind {}
impl ::core::clone::Clone for CoreWebView2MouseEventKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2MouseEventKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CoreWebView2MouseEventKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for CoreWebView2MouseEventKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2MouseEventKind")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2MouseEventKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2MouseEventKind;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CoreWebView2MouseEventVirtualKeys(pub u32);
impl CoreWebView2MouseEventVirtualKeys {
    pub const None: Self = Self(0u32);
    pub const LeftButton: Self = Self(1u32);
    pub const RightButton: Self = Self(2u32);
    pub const Shift: Self = Self(4u32);
    pub const Control: Self = Self(8u32);
    pub const MiddleButton: Self = Self(16u32);
    pub const XButton1: Self = Self(32u32);
    pub const XButton2: Self = Self(64u32);
}
impl ::core::marker::Copy for CoreWebView2MouseEventVirtualKeys {}
impl ::core::clone::Clone for CoreWebView2MouseEventVirtualKeys {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2MouseEventVirtualKeys {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CoreWebView2MouseEventVirtualKeys {
    type Abi = Self;
}
impl ::core::fmt::Debug for CoreWebView2MouseEventVirtualKeys {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2MouseEventVirtualKeys")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for CoreWebView2MouseEventVirtualKeys {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CoreWebView2MouseEventVirtualKeys {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CoreWebView2MouseEventVirtualKeys {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CoreWebView2MouseEventVirtualKeys {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CoreWebView2MouseEventVirtualKeys {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2MouseEventVirtualKeys {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2MouseEventVirtualKeys;u4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CoreWebView2MoveFocusReason(pub i32);
impl CoreWebView2MoveFocusReason {
    pub const Programmatic: Self = Self(0i32);
    pub const Next: Self = Self(1i32);
    pub const Previous: Self = Self(2i32);
}
impl ::core::marker::Copy for CoreWebView2MoveFocusReason {}
impl ::core::clone::Clone for CoreWebView2MoveFocusReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2MoveFocusReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CoreWebView2MoveFocusReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for CoreWebView2MoveFocusReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2MoveFocusReason")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2MoveFocusReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2MoveFocusReason;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
pub struct CoreWebView2MoveFocusRequestedEventArgs(::windows::core::IUnknown);
impl CoreWebView2MoveFocusRequestedEventArgs {
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Reason(&self) -> ::windows::core::Result<CoreWebView2MoveFocusReason> {
        let this = self;
        unsafe {
            let mut result__: CoreWebView2MoveFocusReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Reason)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2MoveFocusReason>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Handled)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetHandled)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for CoreWebView2MoveFocusRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWebView2MoveFocusRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWebView2MoveFocusRequestedEventArgs {}
impl ::core::fmt::Debug for CoreWebView2MoveFocusRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2MoveFocusRequestedEventArgs")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2MoveFocusRequestedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2MoveFocusRequestedEventArgs;{2e29103b-ecdd-5c1d-b288-3f066d608919})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreWebView2MoveFocusRequestedEventArgs {
    type Vtable = ICoreWebView2MoveFocusRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        <ICoreWebView2MoveFocusRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreWebView2MoveFocusRequestedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2MoveFocusRequestedEventArgs";
}
impl ::core::convert::From<CoreWebView2MoveFocusRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2MoveFocusRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2MoveFocusRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2MoveFocusRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2MoveFocusRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2MoveFocusRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWebView2MoveFocusRequestedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: CoreWebView2MoveFocusRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2MoveFocusRequestedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: &CoreWebView2MoveFocusRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2MoveFocusRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2MoveFocusRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreWebView2MoveFocusRequestedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2MoveFocusRequestedEventArgs {}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
pub struct CoreWebView2NavigationCompletedEventArgs(::windows::core::IUnknown);
impl CoreWebView2NavigationCompletedEventArgs {
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn IsSuccess(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSuccess)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn WebErrorStatus(&self) -> ::windows::core::Result<CoreWebView2WebErrorStatus> {
        let this = self;
        unsafe {
            let mut result__: CoreWebView2WebErrorStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WebErrorStatus)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2WebErrorStatus>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn NavigationId(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NavigationId)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u64>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreWebView2NavigationCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWebView2NavigationCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWebView2NavigationCompletedEventArgs {}
impl ::core::fmt::Debug for CoreWebView2NavigationCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2NavigationCompletedEventArgs")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2NavigationCompletedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2NavigationCompletedEventArgs;{4865e238-036a-5664-95a3-447ec44cf498})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreWebView2NavigationCompletedEventArgs {
    type Vtable = ICoreWebView2NavigationCompletedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        <ICoreWebView2NavigationCompletedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreWebView2NavigationCompletedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2NavigationCompletedEventArgs";
}
impl ::core::convert::From<CoreWebView2NavigationCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2NavigationCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2NavigationCompletedEventArgs>
    for ::windows::core::IUnknown
{
    fn from(value: &CoreWebView2NavigationCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2NavigationCompletedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2NavigationCompletedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWebView2NavigationCompletedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: CoreWebView2NavigationCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2NavigationCompletedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: &CoreWebView2NavigationCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2NavigationCompletedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2NavigationCompletedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreWebView2NavigationCompletedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2NavigationCompletedEventArgs {}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
pub struct CoreWebView2NavigationStartingEventArgs(::windows::core::IUnknown);
impl CoreWebView2NavigationStartingEventArgs {
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Uri(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Uri)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn IsUserInitiated(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsUserInitiated)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn IsRedirected(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsRedirected)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn RequestHeaders(&self) -> ::windows::core::Result<CoreWebView2HttpRequestHeaders> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestHeaders)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2HttpRequestHeaders>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Cancel(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Cancel)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetCancel(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetCancel)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn NavigationId(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NavigationId)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u64>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreWebView2NavigationStartingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWebView2NavigationStartingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWebView2NavigationStartingEventArgs {}
impl ::core::fmt::Debug for CoreWebView2NavigationStartingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2NavigationStartingEventArgs")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2NavigationStartingEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2NavigationStartingEventArgs;{548d23d3-fea3-5616-bd05-ae08066c86d3})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreWebView2NavigationStartingEventArgs {
    type Vtable = ICoreWebView2NavigationStartingEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        <ICoreWebView2NavigationStartingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreWebView2NavigationStartingEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2NavigationStartingEventArgs";
}
impl ::core::convert::From<CoreWebView2NavigationStartingEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2NavigationStartingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2NavigationStartingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2NavigationStartingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2NavigationStartingEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2NavigationStartingEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWebView2NavigationStartingEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: CoreWebView2NavigationStartingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2NavigationStartingEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: &CoreWebView2NavigationStartingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2NavigationStartingEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2NavigationStartingEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreWebView2NavigationStartingEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2NavigationStartingEventArgs {}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
pub struct CoreWebView2NewWindowRequestedEventArgs(::windows::core::IUnknown);
impl CoreWebView2NewWindowRequestedEventArgs {
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Uri(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Uri)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn NewWindow(&self) -> ::windows::core::Result<CoreWebView2> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NewWindow)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetNewWindow<'a, Param0: ::windows::core::IntoParam<'a, CoreWebView2>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetNewWindow)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Handled)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetHandled)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn IsUserInitiated(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsUserInitiated)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn WindowFeatures(&self) -> ::windows::core::Result<CoreWebView2WindowFeatures> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WindowFeatures)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2WindowFeatures>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<::windows::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Deferral>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this =
            &::windows::core::Interface::cast::<ICoreWebView2NewWindowRequestedEventArgs2>(self)?;
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
}
impl ::core::clone::Clone for CoreWebView2NewWindowRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWebView2NewWindowRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWebView2NewWindowRequestedEventArgs {}
impl ::core::fmt::Debug for CoreWebView2NewWindowRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2NewWindowRequestedEventArgs")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2NewWindowRequestedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2NewWindowRequestedEventArgs;{e6e013ba-aec8-532e-9ac9-1590af7b25ec})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreWebView2NewWindowRequestedEventArgs {
    type Vtable = ICoreWebView2NewWindowRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        <ICoreWebView2NewWindowRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreWebView2NewWindowRequestedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2NewWindowRequestedEventArgs";
}
impl ::core::convert::From<CoreWebView2NewWindowRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2NewWindowRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2NewWindowRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2NewWindowRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2NewWindowRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2NewWindowRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWebView2NewWindowRequestedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: CoreWebView2NewWindowRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2NewWindowRequestedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: &CoreWebView2NewWindowRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2NewWindowRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2NewWindowRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreWebView2NewWindowRequestedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2NewWindowRequestedEventArgs {}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CoreWebView2PermissionKind(pub i32);
impl CoreWebView2PermissionKind {
    pub const UnknownPermission: Self = Self(0i32);
    pub const Microphone: Self = Self(1i32);
    pub const Camera: Self = Self(2i32);
    pub const Geolocation: Self = Self(3i32);
    pub const Notifications: Self = Self(4i32);
    pub const OtherSensors: Self = Self(5i32);
    pub const ClipboardRead: Self = Self(6i32);
}
impl ::core::marker::Copy for CoreWebView2PermissionKind {}
impl ::core::clone::Clone for CoreWebView2PermissionKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2PermissionKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CoreWebView2PermissionKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for CoreWebView2PermissionKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2PermissionKind")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2PermissionKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2PermissionKind;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
pub struct CoreWebView2PermissionRequestedEventArgs(::windows::core::IUnknown);
impl CoreWebView2PermissionRequestedEventArgs {
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Uri(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Uri)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn PermissionKind(&self) -> ::windows::core::Result<CoreWebView2PermissionKind> {
        let this = self;
        unsafe {
            let mut result__: CoreWebView2PermissionKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PermissionKind)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2PermissionKind>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn IsUserInitiated(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsUserInitiated)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn State(&self) -> ::windows::core::Result<CoreWebView2PermissionState> {
        let this = self;
        unsafe {
            let mut result__: CoreWebView2PermissionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).State)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2PermissionState>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetState(&self, value: CoreWebView2PermissionState) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetState)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<::windows::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreWebView2PermissionRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWebView2PermissionRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWebView2PermissionRequestedEventArgs {}
impl ::core::fmt::Debug for CoreWebView2PermissionRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2PermissionRequestedEventArgs")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2PermissionRequestedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2PermissionRequestedEventArgs;{118bdd9b-cef1-5910-929e-c1a321328239})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreWebView2PermissionRequestedEventArgs {
    type Vtable = ICoreWebView2PermissionRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        <ICoreWebView2PermissionRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreWebView2PermissionRequestedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2PermissionRequestedEventArgs";
}
impl ::core::convert::From<CoreWebView2PermissionRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2PermissionRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2PermissionRequestedEventArgs>
    for ::windows::core::IUnknown
{
    fn from(value: &CoreWebView2PermissionRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2PermissionRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2PermissionRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWebView2PermissionRequestedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: CoreWebView2PermissionRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2PermissionRequestedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: &CoreWebView2PermissionRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2PermissionRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2PermissionRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreWebView2PermissionRequestedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2PermissionRequestedEventArgs {}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CoreWebView2PermissionState(pub i32);
impl CoreWebView2PermissionState {
    pub const Default: Self = Self(0i32);
    pub const Allow: Self = Self(1i32);
    pub const Deny: Self = Self(2i32);
}
impl ::core::marker::Copy for CoreWebView2PermissionState {}
impl ::core::clone::Clone for CoreWebView2PermissionState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2PermissionState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CoreWebView2PermissionState {
    type Abi = Self;
}
impl ::core::fmt::Debug for CoreWebView2PermissionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2PermissionState")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2PermissionState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2PermissionState;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
#[doc = "*Required features: 'Web_WebView2_Core'*"]
pub struct CoreWebView2PhysicalKeyStatus {
    pub RepeatCount: u32,
    pub ScanCode: u32,
    pub IsExtendedKey: i32,
    pub IsMenuKeyDown: i32,
    pub WasKeyDown: i32,
    pub IsKeyReleased: i32,
}
impl ::core::marker::Copy for CoreWebView2PhysicalKeyStatus {}
impl ::core::clone::Clone for CoreWebView2PhysicalKeyStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CoreWebView2PhysicalKeyStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CoreWebView2PhysicalKeyStatus")
            .field("RepeatCount", &self.RepeatCount)
            .field("ScanCode", &self.ScanCode)
            .field("IsExtendedKey", &self.IsExtendedKey)
            .field("IsMenuKeyDown", &self.IsMenuKeyDown)
            .field("WasKeyDown", &self.WasKeyDown)
            .field("IsKeyReleased", &self.IsKeyReleased)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for CoreWebView2PhysicalKeyStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2PhysicalKeyStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"struct(Microsoft.Web.WebView2.Core.CoreWebView2PhysicalKeyStatus;u4;u4;i4;i4;i4;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for CoreWebView2PhysicalKeyStatus {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            ::windows::core::memcmp(
                self as *const _ as _,
                other as *const _ as _,
                core::mem::size_of::<CoreWebView2PhysicalKeyStatus>(),
            ) == 0
        }
    }
}
impl ::core::cmp::Eq for CoreWebView2PhysicalKeyStatus {}
impl ::core::default::Default for CoreWebView2PhysicalKeyStatus {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CoreWebView2PointerEventKind(pub i32);
impl CoreWebView2PointerEventKind {
    pub const Activate: Self = Self(587i32);
    pub const Down: Self = Self(582i32);
    pub const Enter: Self = Self(585i32);
    pub const Leave: Self = Self(586i32);
    pub const Up: Self = Self(583i32);
    pub const Update: Self = Self(581i32);
}
impl ::core::marker::Copy for CoreWebView2PointerEventKind {}
impl ::core::clone::Clone for CoreWebView2PointerEventKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2PointerEventKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CoreWebView2PointerEventKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for CoreWebView2PointerEventKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2PointerEventKind")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2PointerEventKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2PointerEventKind;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
pub struct CoreWebView2PointerInfo(::windows::core::IUnknown);
impl CoreWebView2PointerInfo {
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn PointerKind(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerKind)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetPointerKind(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetPointerKind)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn PointerId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerId)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetPointerId(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetPointerId)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn FrameId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FrameId)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetFrameId(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFrameId)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn PointerFlags(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerFlags)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetPointerFlags(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetPointerFlags)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn PointerDeviceRect(&self) -> ::windows::core::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerDeviceRect)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetPointerDeviceRect<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Rect>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetPointerDeviceRect)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn DisplayRect(&self) -> ::windows::core::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayRect)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetDisplayRect<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Rect>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetDisplayRect)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn PixelLocation(&self) -> ::windows::core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PixelLocation)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetPixelLocation<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Point>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetPixelLocation)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn HimetricLocation(&self) -> ::windows::core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HimetricLocation)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetHimetricLocation<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Point>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetHimetricLocation)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn PixelLocationRaw(&self) -> ::windows::core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PixelLocationRaw)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetPixelLocationRaw<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Point>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetPixelLocationRaw)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn HimetricLocationRaw(&self) -> ::windows::core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HimetricLocationRaw)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetHimetricLocationRaw<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Point>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetHimetricLocationRaw)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Time(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Time)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetTime(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetTime)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn HistoryCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HistoryCount)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetHistoryCount(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetHistoryCount)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn InputData(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InputData)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetInputData(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetInputData)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn KeyStates(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyStates)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetKeyStates(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetKeyStates)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn PerformanceCount(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PerformanceCount)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetPerformanceCount(&self, value: u64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetPerformanceCount)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn ButtonChangeKind(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ButtonChangeKind)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetButtonChangeKind(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetButtonChangeKind)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn PenFlags(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PenFlags)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetPenFlags(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetPenFlags)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn PenMask(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PenMask)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetPenMask(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetPenMask)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn PenPressure(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PenPressure)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetPenPressure(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetPenPressure)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn PenRotation(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PenRotation)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetPenRotation(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetPenRotation)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn PenTiltX(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PenTiltX)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetPenTiltX(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetPenTiltX)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn PenTiltY(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PenTiltY)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetPenTiltY(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetPenTiltY)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn TouchFlags(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TouchFlags)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetTouchFlags(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetTouchFlags)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn TouchMask(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TouchMask)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetTouchMask(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetTouchMask)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn TouchContact(&self) -> ::windows::core::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TouchContact)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetTouchContact<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Rect>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetTouchContact)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn TouchContactRaw(&self) -> ::windows::core::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TouchContactRaw)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetTouchContactRaw<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Rect>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetTouchContactRaw)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn TouchOrientation(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TouchOrientation)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetTouchOrientation(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetTouchOrientation)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn TouchPressure(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TouchPressure)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetTouchPressure(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetTouchPressure)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for CoreWebView2PointerInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWebView2PointerInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWebView2PointerInfo {}
impl ::core::fmt::Debug for CoreWebView2PointerInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2PointerInfo")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2PointerInfo {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2PointerInfo;{c3860e0d-c018-5a84-bc06-9f8f7b275dff})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreWebView2PointerInfo {
    type Vtable = ICoreWebView2PointerInfo_Vtbl;
    const IID: ::windows::core::GUID =
        <ICoreWebView2PointerInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreWebView2PointerInfo {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2PointerInfo";
}
impl ::core::convert::From<CoreWebView2PointerInfo> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2PointerInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2PointerInfo> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2PointerInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreWebView2PointerInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreWebView2PointerInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWebView2PointerInfo> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2PointerInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2PointerInfo> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2PointerInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreWebView2PointerInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2PointerInfo
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreWebView2PointerInfo {}
unsafe impl ::core::marker::Sync for CoreWebView2PointerInfo {}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CoreWebView2PrintOrientation(pub i32);
impl CoreWebView2PrintOrientation {
    pub const Portrait: Self = Self(0i32);
    pub const Landscape: Self = Self(1i32);
}
impl ::core::marker::Copy for CoreWebView2PrintOrientation {}
impl ::core::clone::Clone for CoreWebView2PrintOrientation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2PrintOrientation {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CoreWebView2PrintOrientation {
    type Abi = Self;
}
impl ::core::fmt::Debug for CoreWebView2PrintOrientation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2PrintOrientation")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2PrintOrientation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2PrintOrientation;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
pub struct CoreWebView2PrintSettings(::windows::core::IUnknown);
impl CoreWebView2PrintSettings {
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Orientation(&self) -> ::windows::core::Result<CoreWebView2PrintOrientation> {
        let this = self;
        unsafe {
            let mut result__: CoreWebView2PrintOrientation = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Orientation)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2PrintOrientation>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetOrientation(
        &self,
        value: CoreWebView2PrintOrientation,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetOrientation)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn ScaleFactor(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ScaleFactor)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetScaleFactor(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetScaleFactor)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn PageWidth(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PageWidth)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetPageWidth(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetPageWidth)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn PageHeight(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PageHeight)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetPageHeight(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetPageHeight)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn MarginTop(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MarginTop)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetMarginTop(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetMarginTop)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn MarginBottom(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MarginBottom)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetMarginBottom(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetMarginBottom)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn MarginLeft(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MarginLeft)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetMarginLeft(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetMarginLeft)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn MarginRight(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MarginRight)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetMarginRight(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetMarginRight)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn ShouldPrintBackgrounds(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShouldPrintBackgrounds)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetShouldPrintBackgrounds(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetShouldPrintBackgrounds)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn ShouldPrintSelectionOnly(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShouldPrintSelectionOnly)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetShouldPrintSelectionOnly(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetShouldPrintSelectionOnly)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn ShouldPrintHeaderAndFooter(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShouldPrintHeaderAndFooter)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetShouldPrintHeaderAndFooter(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetShouldPrintHeaderAndFooter)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn HeaderTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HeaderTitle)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetHeaderTitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetHeaderTitle)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn FooterUri(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FooterUri)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetFooterUri<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFooterUri)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for CoreWebView2PrintSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWebView2PrintSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWebView2PrintSettings {}
impl ::core::fmt::Debug for CoreWebView2PrintSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2PrintSettings")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2PrintSettings {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2PrintSettings;{9c75c8c0-ef3d-58a8-9a8c-18eed9fd0f16})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreWebView2PrintSettings {
    type Vtable = ICoreWebView2PrintSettings_Vtbl;
    const IID: ::windows::core::GUID =
        <ICoreWebView2PrintSettings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreWebView2PrintSettings {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2PrintSettings";
}
impl ::core::convert::From<CoreWebView2PrintSettings> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2PrintSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2PrintSettings> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2PrintSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreWebView2PrintSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2PrintSettings
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWebView2PrintSettings> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2PrintSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2PrintSettings> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2PrintSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2PrintSettings
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2PrintSettings
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreWebView2PrintSettings {}
unsafe impl ::core::marker::Sync for CoreWebView2PrintSettings {}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
pub struct CoreWebView2ProcessFailedEventArgs(::windows::core::IUnknown);
impl CoreWebView2ProcessFailedEventArgs {
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn ProcessFailedKind(&self) -> ::windows::core::Result<CoreWebView2ProcessFailedKind> {
        let this = self;
        unsafe {
            let mut result__: CoreWebView2ProcessFailedKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProcessFailedKind)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2ProcessFailedKind>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Reason(&self) -> ::windows::core::Result<CoreWebView2ProcessFailedReason> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2ProcessFailedEventArgs2>(self)?;
        unsafe {
            let mut result__: CoreWebView2ProcessFailedReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Reason)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2ProcessFailedReason>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn ExitCode(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2ProcessFailedEventArgs2>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExitCode)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn ProcessDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2ProcessFailedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProcessDescription)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn FrameInfosForFailedProcess(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IVectorView<CoreWebView2FrameInfo>,
    > {
        let this = &::windows::core::Interface::cast::<ICoreWebView2ProcessFailedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FrameInfosForFailedProcess)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVectorView<CoreWebView2FrameInfo>>(
                result__,
            )
        }
    }
}
impl ::core::clone::Clone for CoreWebView2ProcessFailedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWebView2ProcessFailedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWebView2ProcessFailedEventArgs {}
impl ::core::fmt::Debug for CoreWebView2ProcessFailedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2ProcessFailedEventArgs")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2ProcessFailedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2ProcessFailedEventArgs;{25a8f8c9-d944-539d-afa3-24172b48ef47})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreWebView2ProcessFailedEventArgs {
    type Vtable = ICoreWebView2ProcessFailedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        <ICoreWebView2ProcessFailedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreWebView2ProcessFailedEventArgs {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2ProcessFailedEventArgs";
}
impl ::core::convert::From<CoreWebView2ProcessFailedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2ProcessFailedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2ProcessFailedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2ProcessFailedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2ProcessFailedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2ProcessFailedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWebView2ProcessFailedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2ProcessFailedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2ProcessFailedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2ProcessFailedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2ProcessFailedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2ProcessFailedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreWebView2ProcessFailedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2ProcessFailedEventArgs {}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CoreWebView2ProcessFailedKind(pub i32);
impl CoreWebView2ProcessFailedKind {
    pub const BrowserProcessExited: Self = Self(0i32);
    pub const RenderProcessExited: Self = Self(1i32);
    pub const RenderProcessUnresponsive: Self = Self(2i32);
    pub const FrameRenderProcessExited: Self = Self(3i32);
    pub const UtilityProcessExited: Self = Self(4i32);
    pub const SandboxHelperProcessExited: Self = Self(5i32);
    pub const GpuProcessExited: Self = Self(6i32);
    pub const PpapiPluginProcessExited: Self = Self(7i32);
    pub const PpapiBrokerProcessExited: Self = Self(8i32);
    pub const UnknownProcessExited: Self = Self(9i32);
}
impl ::core::marker::Copy for CoreWebView2ProcessFailedKind {}
impl ::core::clone::Clone for CoreWebView2ProcessFailedKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2ProcessFailedKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CoreWebView2ProcessFailedKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for CoreWebView2ProcessFailedKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2ProcessFailedKind")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2ProcessFailedKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2ProcessFailedKind;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CoreWebView2ProcessFailedReason(pub i32);
impl CoreWebView2ProcessFailedReason {
    pub const Unexpected: Self = Self(0i32);
    pub const Unresponsive: Self = Self(1i32);
    pub const Terminated: Self = Self(2i32);
    pub const Crashed: Self = Self(3i32);
    pub const LaunchFailed: Self = Self(4i32);
    pub const OutOfMemory: Self = Self(5i32);
}
impl ::core::marker::Copy for CoreWebView2ProcessFailedReason {}
impl ::core::clone::Clone for CoreWebView2ProcessFailedReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2ProcessFailedReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CoreWebView2ProcessFailedReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for CoreWebView2ProcessFailedReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2ProcessFailedReason")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2ProcessFailedReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2ProcessFailedReason;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CoreWebView2ScriptDialogKind(pub i32);
impl CoreWebView2ScriptDialogKind {
    pub const Alert: Self = Self(0i32);
    pub const Confirm: Self = Self(1i32);
    pub const Prompt: Self = Self(2i32);
    pub const Beforeunload: Self = Self(3i32);
}
impl ::core::marker::Copy for CoreWebView2ScriptDialogKind {}
impl ::core::clone::Clone for CoreWebView2ScriptDialogKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2ScriptDialogKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CoreWebView2ScriptDialogKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for CoreWebView2ScriptDialogKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2ScriptDialogKind")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2ScriptDialogKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2ScriptDialogKind;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
pub struct CoreWebView2ScriptDialogOpeningEventArgs(::windows::core::IUnknown);
impl CoreWebView2ScriptDialogOpeningEventArgs {
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Uri(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Uri)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Kind(&self) -> ::windows::core::Result<CoreWebView2ScriptDialogKind> {
        let this = self;
        unsafe {
            let mut result__: CoreWebView2ScriptDialogKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2ScriptDialogKind>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Message)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn DefaultText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DefaultText)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn ResultText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResultText)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetResultText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetResultText)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Accept(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Accept)(::core::mem::transmute_copy(this))
                .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<::windows::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreWebView2ScriptDialogOpeningEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWebView2ScriptDialogOpeningEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWebView2ScriptDialogOpeningEventArgs {}
impl ::core::fmt::Debug for CoreWebView2ScriptDialogOpeningEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2ScriptDialogOpeningEventArgs")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2ScriptDialogOpeningEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2ScriptDialogOpeningEventArgs;{a4315212-c7eb-568a-86e4-c61e31ba6cda})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreWebView2ScriptDialogOpeningEventArgs {
    type Vtable = ICoreWebView2ScriptDialogOpeningEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        <ICoreWebView2ScriptDialogOpeningEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreWebView2ScriptDialogOpeningEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2ScriptDialogOpeningEventArgs";
}
impl ::core::convert::From<CoreWebView2ScriptDialogOpeningEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2ScriptDialogOpeningEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2ScriptDialogOpeningEventArgs>
    for ::windows::core::IUnknown
{
    fn from(value: &CoreWebView2ScriptDialogOpeningEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2ScriptDialogOpeningEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2ScriptDialogOpeningEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWebView2ScriptDialogOpeningEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: CoreWebView2ScriptDialogOpeningEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2ScriptDialogOpeningEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: &CoreWebView2ScriptDialogOpeningEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2ScriptDialogOpeningEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2ScriptDialogOpeningEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreWebView2ScriptDialogOpeningEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2ScriptDialogOpeningEventArgs {}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
pub struct CoreWebView2Settings(::windows::core::IUnknown);
impl CoreWebView2Settings {
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn IsScriptEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsScriptEnabled)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetIsScriptEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsScriptEnabled)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn IsWebMessageEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsWebMessageEnabled)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetIsWebMessageEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsWebMessageEnabled)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn AreDefaultScriptDialogsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AreDefaultScriptDialogsEnabled)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetAreDefaultScriptDialogsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAreDefaultScriptDialogsEnabled)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn IsStatusBarEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsStatusBarEnabled)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetIsStatusBarEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsStatusBarEnabled)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn AreDevToolsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AreDevToolsEnabled)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetAreDevToolsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAreDevToolsEnabled)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn AreDefaultContextMenusEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AreDefaultContextMenusEnabled)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetAreDefaultContextMenusEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAreDefaultContextMenusEnabled)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn AreHostObjectsAllowed(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AreHostObjectsAllowed)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetAreHostObjectsAllowed(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAreHostObjectsAllowed)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn IsZoomControlEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsZoomControlEnabled)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetIsZoomControlEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsZoomControlEnabled)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn IsBuiltInErrorPageEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsBuiltInErrorPageEnabled)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetIsBuiltInErrorPageEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsBuiltInErrorPageEnabled)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn UserAgent(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Settings2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UserAgent)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetUserAgent<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Settings2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetUserAgent)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn AreBrowserAcceleratorKeysEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Settings3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AreBrowserAcceleratorKeysEnabled)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetAreBrowserAcceleratorKeysEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Settings3>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAreBrowserAcceleratorKeysEnabled)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn IsPasswordAutosaveEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Settings4>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPasswordAutosaveEnabled)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetIsPasswordAutosaveEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Settings4>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsPasswordAutosaveEnabled)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn IsGeneralAutofillEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Settings4>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsGeneralAutofillEnabled)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetIsGeneralAutofillEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Settings4>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsGeneralAutofillEnabled)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn IsPinchZoomEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Settings5>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPinchZoomEnabled)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetIsPinchZoomEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Settings5>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsPinchZoomEnabled)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn IsSwipeNavigationEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Settings6>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSwipeNavigationEnabled)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetIsSwipeNavigationEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Settings6>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsSwipeNavigationEnabled)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn HostObjectDispatchAdapter(
        &self,
    ) -> ::windows::core::Result<ICoreWebView2DispatchAdapter> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Settings_Manual>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HostObjectDispatchAdapter)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ICoreWebView2DispatchAdapter>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetHostObjectDispatchAdapter<
        'a,
        Param0: ::windows::core::IntoParam<'a, ICoreWebView2DispatchAdapter>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWebView2Settings_Manual>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetHostObjectDispatchAdapter)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for CoreWebView2Settings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWebView2Settings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWebView2Settings {}
impl ::core::fmt::Debug for CoreWebView2Settings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2Settings")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2Settings {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2Settings;{003b325e-74cd-52dd-8024-ebb8be38e48e})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreWebView2Settings {
    type Vtable = ICoreWebView2Settings_Vtbl;
    const IID: ::windows::core::GUID = <ICoreWebView2Settings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreWebView2Settings {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2Settings";
}
impl ::core::convert::From<CoreWebView2Settings> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2Settings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2Settings> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2Settings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreWebView2Settings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreWebView2Settings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWebView2Settings> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2Settings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2Settings> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2Settings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreWebView2Settings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2Settings
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreWebView2Settings {}
unsafe impl ::core::marker::Sync for CoreWebView2Settings {}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
pub struct CoreWebView2SourceChangedEventArgs(::windows::core::IUnknown);
impl CoreWebView2SourceChangedEventArgs {
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn IsNewDocument(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsNewDocument)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreWebView2SourceChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWebView2SourceChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWebView2SourceChangedEventArgs {}
impl ::core::fmt::Debug for CoreWebView2SourceChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2SourceChangedEventArgs")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2SourceChangedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2SourceChangedEventArgs;{ca437b2c-6a18-5552-b749-b198f8cc34d9})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreWebView2SourceChangedEventArgs {
    type Vtable = ICoreWebView2SourceChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        <ICoreWebView2SourceChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreWebView2SourceChangedEventArgs {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2SourceChangedEventArgs";
}
impl ::core::convert::From<CoreWebView2SourceChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2SourceChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2SourceChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2SourceChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2SourceChangedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2SourceChangedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWebView2SourceChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2SourceChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2SourceChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2SourceChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2SourceChangedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2SourceChangedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreWebView2SourceChangedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2SourceChangedEventArgs {}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CoreWebView2WebErrorStatus(pub i32);
impl CoreWebView2WebErrorStatus {
    pub const Unknown: Self = Self(0i32);
    pub const CertificateCommonNameIsIncorrect: Self = Self(1i32);
    pub const CertificateExpired: Self = Self(2i32);
    pub const ClientCertificateContainsErrors: Self = Self(3i32);
    pub const CertificateRevoked: Self = Self(4i32);
    pub const CertificateIsInvalid: Self = Self(5i32);
    pub const ServerUnreachable: Self = Self(6i32);
    pub const Timeout: Self = Self(7i32);
    pub const ErrorHttpInvalidServerResponse: Self = Self(8i32);
    pub const ConnectionAborted: Self = Self(9i32);
    pub const ConnectionReset: Self = Self(10i32);
    pub const Disconnected: Self = Self(11i32);
    pub const CannotConnect: Self = Self(12i32);
    pub const HostNameNotResolved: Self = Self(13i32);
    pub const OperationCanceled: Self = Self(14i32);
    pub const RedirectFailed: Self = Self(15i32);
    pub const UnexpectedError: Self = Self(16i32);
}
impl ::core::marker::Copy for CoreWebView2WebErrorStatus {}
impl ::core::clone::Clone for CoreWebView2WebErrorStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2WebErrorStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CoreWebView2WebErrorStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for CoreWebView2WebErrorStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2WebErrorStatus")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2WebErrorStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2WebErrorStatus;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
pub struct CoreWebView2WebMessageReceivedEventArgs(::windows::core::IUnknown);
impl CoreWebView2WebMessageReceivedEventArgs {
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Source(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Source)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn WebMessageAsJson(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WebMessageAsJson)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn TryGetWebMessageAsString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryGetWebMessageAsString)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreWebView2WebMessageReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWebView2WebMessageReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWebView2WebMessageReceivedEventArgs {}
impl ::core::fmt::Debug for CoreWebView2WebMessageReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2WebMessageReceivedEventArgs")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2WebMessageReceivedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2WebMessageReceivedEventArgs;{eb066159-b725-5d5b-adc8-f5d7b9290304})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreWebView2WebMessageReceivedEventArgs {
    type Vtable = ICoreWebView2WebMessageReceivedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        <ICoreWebView2WebMessageReceivedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreWebView2WebMessageReceivedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2WebMessageReceivedEventArgs";
}
impl ::core::convert::From<CoreWebView2WebMessageReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2WebMessageReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2WebMessageReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2WebMessageReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2WebMessageReceivedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2WebMessageReceivedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWebView2WebMessageReceivedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: CoreWebView2WebMessageReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2WebMessageReceivedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: &CoreWebView2WebMessageReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2WebMessageReceivedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2WebMessageReceivedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreWebView2WebMessageReceivedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2WebMessageReceivedEventArgs {}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CoreWebView2WebResourceContext(pub i32);
impl CoreWebView2WebResourceContext {
    pub const All: Self = Self(0i32);
    pub const Document: Self = Self(1i32);
    pub const Stylesheet: Self = Self(2i32);
    pub const Image: Self = Self(3i32);
    pub const Media: Self = Self(4i32);
    pub const Font: Self = Self(5i32);
    pub const Script: Self = Self(6i32);
    pub const XmlHttpRequest: Self = Self(7i32);
    pub const Fetch: Self = Self(8i32);
    pub const TextTrack: Self = Self(9i32);
    pub const EventSource: Self = Self(10i32);
    pub const Websocket: Self = Self(11i32);
    pub const Manifest: Self = Self(12i32);
    pub const SignedExchange: Self = Self(13i32);
    pub const Ping: Self = Self(14i32);
    pub const CspViolationReport: Self = Self(15i32);
    pub const Other: Self = Self(16i32);
}
impl ::core::marker::Copy for CoreWebView2WebResourceContext {}
impl ::core::clone::Clone for CoreWebView2WebResourceContext {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2WebResourceContext {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CoreWebView2WebResourceContext {
    type Abi = Self;
}
impl ::core::fmt::Debug for CoreWebView2WebResourceContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2WebResourceContext")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2WebResourceContext {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2WebResourceContext;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
pub struct CoreWebView2WebResourceRequest(::windows::core::IUnknown);
impl CoreWebView2WebResourceRequest {
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Uri(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Uri)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetUri<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetUri)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Method(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Method)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetMethod<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetMethod)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Content(
        &self,
    ) -> ::windows::core::Result<::windows::Storage::Streams::IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Content)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Storage::Streams::IRandomAccessStream>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetContent<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Storage::Streams::IRandomAccessStream>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetContent)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Headers(&self) -> ::windows::core::Result<CoreWebView2HttpRequestHeaders> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Headers)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2HttpRequestHeaders>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreWebView2WebResourceRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWebView2WebResourceRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWebView2WebResourceRequest {}
impl ::core::fmt::Debug for CoreWebView2WebResourceRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2WebResourceRequest")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2WebResourceRequest {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2WebResourceRequest;{5c742259-67d2-5df2-8382-0f201b4d7197})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreWebView2WebResourceRequest {
    type Vtable = ICoreWebView2WebResourceRequest_Vtbl;
    const IID: ::windows::core::GUID =
        <ICoreWebView2WebResourceRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreWebView2WebResourceRequest {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2WebResourceRequest";
}
impl ::core::convert::From<CoreWebView2WebResourceRequest> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2WebResourceRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2WebResourceRequest> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2WebResourceRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2WebResourceRequest
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2WebResourceRequest
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWebView2WebResourceRequest> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2WebResourceRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2WebResourceRequest> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2WebResourceRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2WebResourceRequest
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2WebResourceRequest
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreWebView2WebResourceRequest {}
unsafe impl ::core::marker::Sync for CoreWebView2WebResourceRequest {}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
pub struct CoreWebView2WebResourceRequestedEventArgs(::windows::core::IUnknown);
impl CoreWebView2WebResourceRequestedEventArgs {
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Request(&self) -> ::windows::core::Result<CoreWebView2WebResourceRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Request)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2WebResourceRequest>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Response(&self) -> ::windows::core::Result<CoreWebView2WebResourceResponse> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Response)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2WebResourceResponse>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetResponse<
        'a,
        Param0: ::windows::core::IntoParam<'a, CoreWebView2WebResourceResponse>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetResponse)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn ResourceContext(&self) -> ::windows::core::Result<CoreWebView2WebResourceContext> {
        let this = self;
        unsafe {
            let mut result__: CoreWebView2WebResourceContext = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResourceContext)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2WebResourceContext>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<::windows::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreWebView2WebResourceRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWebView2WebResourceRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWebView2WebResourceRequestedEventArgs {}
impl ::core::fmt::Debug for CoreWebView2WebResourceRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2WebResourceRequestedEventArgs")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2WebResourceRequestedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2WebResourceRequestedEventArgs;{577f1fc4-c943-54a9-9700-bd469b48bd41})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreWebView2WebResourceRequestedEventArgs {
    type Vtable = ICoreWebView2WebResourceRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        <ICoreWebView2WebResourceRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreWebView2WebResourceRequestedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2WebResourceRequestedEventArgs";
}
impl ::core::convert::From<CoreWebView2WebResourceRequestedEventArgs>
    for ::windows::core::IUnknown
{
    fn from(value: CoreWebView2WebResourceRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2WebResourceRequestedEventArgs>
    for ::windows::core::IUnknown
{
    fn from(value: &CoreWebView2WebResourceRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2WebResourceRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2WebResourceRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWebView2WebResourceRequestedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: CoreWebView2WebResourceRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2WebResourceRequestedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: &CoreWebView2WebResourceRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2WebResourceRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2WebResourceRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreWebView2WebResourceRequestedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2WebResourceRequestedEventArgs {}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
pub struct CoreWebView2WebResourceResponse(::windows::core::IUnknown);
impl CoreWebView2WebResourceResponse {
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Content(
        &self,
    ) -> ::windows::core::Result<::windows::Storage::Streams::IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Content)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Storage::Streams::IRandomAccessStream>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetContent<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Storage::Streams::IRandomAccessStream>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetContent)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Headers(&self) -> ::windows::core::Result<CoreWebView2HttpResponseHeaders> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Headers)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2HttpResponseHeaders>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn StatusCode(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StatusCode)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetStatusCode(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetStatusCode)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn ReasonPhrase(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReasonPhrase)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn SetReasonPhrase<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetReasonPhrase)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for CoreWebView2WebResourceResponse {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWebView2WebResourceResponse {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWebView2WebResourceResponse {}
impl ::core::fmt::Debug for CoreWebView2WebResourceResponse {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2WebResourceResponse")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2WebResourceResponse {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2WebResourceResponse;{14621923-e485-5f44-8f5d-bd4243bc398f})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreWebView2WebResourceResponse {
    type Vtable = ICoreWebView2WebResourceResponse_Vtbl;
    const IID: ::windows::core::GUID =
        <ICoreWebView2WebResourceResponse as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreWebView2WebResourceResponse {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2WebResourceResponse";
}
impl ::core::convert::From<CoreWebView2WebResourceResponse> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2WebResourceResponse) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2WebResourceResponse> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2WebResourceResponse) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2WebResourceResponse
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2WebResourceResponse
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWebView2WebResourceResponse> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2WebResourceResponse) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2WebResourceResponse> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2WebResourceResponse) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2WebResourceResponse
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2WebResourceResponse
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreWebView2WebResourceResponse {}
unsafe impl ::core::marker::Sync for CoreWebView2WebResourceResponse {}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
pub struct CoreWebView2WebResourceResponseReceivedEventArgs(::windows::core::IUnknown);
impl CoreWebView2WebResourceResponseReceivedEventArgs {
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Request(&self) -> ::windows::core::Result<CoreWebView2WebResourceRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Request)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2WebResourceRequest>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Response(&self) -> ::windows::core::Result<CoreWebView2WebResourceResponseView> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Response)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2WebResourceResponseView>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreWebView2WebResourceResponseReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWebView2WebResourceResponseReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWebView2WebResourceResponseReceivedEventArgs {}
impl ::core::fmt::Debug for CoreWebView2WebResourceResponseReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2WebResourceResponseReceivedEventArgs")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2WebResourceResponseReceivedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2WebResourceResponseReceivedEventArgs;{12424671-9711-54f4-bcdf-5f307add6ec2})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreWebView2WebResourceResponseReceivedEventArgs {
    type Vtable = ICoreWebView2WebResourceResponseReceivedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        <ICoreWebView2WebResourceResponseReceivedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreWebView2WebResourceResponseReceivedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2WebResourceResponseReceivedEventArgs";
}
impl ::core::convert::From<CoreWebView2WebResourceResponseReceivedEventArgs>
    for ::windows::core::IUnknown
{
    fn from(value: CoreWebView2WebResourceResponseReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2WebResourceResponseReceivedEventArgs>
    for ::windows::core::IUnknown
{
    fn from(value: &CoreWebView2WebResourceResponseReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2WebResourceResponseReceivedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2WebResourceResponseReceivedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWebView2WebResourceResponseReceivedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: CoreWebView2WebResourceResponseReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2WebResourceResponseReceivedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: &CoreWebView2WebResourceResponseReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2WebResourceResponseReceivedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2WebResourceResponseReceivedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreWebView2WebResourceResponseReceivedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2WebResourceResponseReceivedEventArgs {}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
pub struct CoreWebView2WebResourceResponseView(::windows::core::IUnknown);
impl CoreWebView2WebResourceResponseView {
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Headers(&self) -> ::windows::core::Result<CoreWebView2HttpResponseHeaders> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Headers)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2HttpResponseHeaders>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn StatusCode(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StatusCode)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn ReasonPhrase(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReasonPhrase)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn GetContentAsync(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::IAsyncOperation<::windows::Storage::Streams::IRandomAccessStream>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetContentAsync)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IAsyncOperation<
                ::windows::Storage::Streams::IRandomAccessStream,
            >>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreWebView2WebResourceResponseView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWebView2WebResourceResponseView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWebView2WebResourceResponseView {}
impl ::core::fmt::Debug for CoreWebView2WebResourceResponseView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2WebResourceResponseView")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2WebResourceResponseView {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2WebResourceResponseView;{33ee060b-b578-5698-b541-fef87fe7fe72})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreWebView2WebResourceResponseView {
    type Vtable = ICoreWebView2WebResourceResponseView_Vtbl;
    const IID: ::windows::core::GUID =
        <ICoreWebView2WebResourceResponseView as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreWebView2WebResourceResponseView {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2WebResourceResponseView";
}
impl ::core::convert::From<CoreWebView2WebResourceResponseView> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2WebResourceResponseView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2WebResourceResponseView> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2WebResourceResponseView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWebView2WebResourceResponseView
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2WebResourceResponseView
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWebView2WebResourceResponseView> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2WebResourceResponseView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2WebResourceResponseView> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2WebResourceResponseView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2WebResourceResponseView
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2WebResourceResponseView
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreWebView2WebResourceResponseView {}
unsafe impl ::core::marker::Sync for CoreWebView2WebResourceResponseView {}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
pub struct CoreWebView2WindowFeatures(::windows::core::IUnknown);
impl CoreWebView2WindowFeatures {
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn HasPosition(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HasPosition)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn HasSize(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HasSize)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Left(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Left)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Top(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Top)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Height(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Height)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Width(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Width)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn ShouldDisplayMenuBar(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShouldDisplayMenuBar)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn ShouldDisplayStatus(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShouldDisplayStatus)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn ShouldDisplayToolbar(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShouldDisplayToolbar)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn ShouldDisplayScrollBars(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShouldDisplayScrollBars)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreWebView2WindowFeatures {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWebView2WindowFeatures {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWebView2WindowFeatures {}
impl ::core::fmt::Debug for CoreWebView2WindowFeatures {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2WindowFeatures")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWebView2WindowFeatures {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2WindowFeatures;{ee8686d6-056f-5e06-824f-4e2a24c1c1d6})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreWebView2WindowFeatures {
    type Vtable = ICoreWebView2WindowFeatures_Vtbl;
    const IID: ::windows::core::GUID =
        <ICoreWebView2WindowFeatures as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreWebView2WindowFeatures {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2WindowFeatures";
}
impl ::core::convert::From<CoreWebView2WindowFeatures> for ::windows::core::IUnknown {
    fn from(value: CoreWebView2WindowFeatures) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2WindowFeatures> for ::windows::core::IUnknown {
    fn from(value: &CoreWebView2WindowFeatures) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreWebView2WindowFeatures {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWebView2WindowFeatures
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWebView2WindowFeatures> for ::windows::core::IInspectable {
    fn from(value: CoreWebView2WindowFeatures) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWebView2WindowFeatures> for ::windows::core::IInspectable {
    fn from(value: &CoreWebView2WindowFeatures) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWebView2WindowFeatures
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWebView2WindowFeatures
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreWebView2WindowFeatures {}
unsafe impl ::core::marker::Sync for CoreWebView2WindowFeatures {}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2 {
    type Vtable = ICoreWebView2_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x3a3f559a_e5e9_5338_bb67_4eb0504a4f14);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Settings: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub Source: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub BrowserProcessId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub CanGoBack: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub CanGoForward: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub DocumentTitle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub ContainsFullScreenElement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub NavigationStarting: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveNavigationStarting: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub ContentLoading: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveContentLoading: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub SourceChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveSourceChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub HistoryChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveHistoryChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub NavigationCompleted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveNavigationCompleted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub FrameNavigationStarting: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveFrameNavigationStarting: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub FrameNavigationCompleted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveFrameNavigationCompleted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub ScriptDialogOpening: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveScriptDialogOpening: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub PermissionRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemovePermissionRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub ProcessFailed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveProcessFailed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub WebMessageReceived: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveWebMessageReceived: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub NewWindowRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveNewWindowRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub DocumentTitleChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveDocumentTitleChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub ContainsFullScreenElementChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    )
        -> ::windows::core::HRESULT,
    pub RemoveContainsFullScreenElementChanged:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            token: ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows::core::HRESULT,
    pub WebResourceRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveWebResourceRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub WindowCloseRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveWindowCloseRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub Navigate: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        uri: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub NavigateToString: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        htmlcontent: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub AddScriptToExecuteOnDocumentCreatedAsync:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            javascript: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
            result__: *mut ::windows::core::RawPtr,
        ) -> ::windows::core::HRESULT,
    pub RemoveScriptToExecuteOnDocumentCreated:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        ) -> ::windows::core::HRESULT,
    pub ExecuteScriptAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        javascript: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub CapturePreviewAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        imageformat: CoreWebView2CapturePreviewImageFormat,
        imagestream: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub Reload:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PostWebMessageAsJson: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        webmessageasjson: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub PostWebMessageAsString: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        webmessageasstring: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub CallDevToolsProtocolMethodAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        methodname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        parametersasjson: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GoBack:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GoForward:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDevToolsProtocolEventReceiver: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        eventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    )
        -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddHostObjectToScript: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        rawobject: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub RemoveHostObjectFromScript: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub OpenDevToolsWindow:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddWebResourceRequestedFilter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        uri: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        resourcecontext: CoreWebView2WebResourceContext,
    ) -> ::windows::core::HRESULT,
    pub RemoveWebResourceRequestedFilter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        uri: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        resourcecontext: CoreWebView2WebResourceContext,
    )
        -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2AcceleratorKeyPressedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2AcceleratorKeyPressedEventArgs {
    type Vtable = ICoreWebView2AcceleratorKeyPressedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x41a56100_92a5_59d1_9e71_9222e33ae38b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2AcceleratorKeyPressedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub KeyEventKind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2KeyEventKind,
    ) -> ::windows::core::HRESULT,
    pub VirtualKey: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub KeyEventLParam: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub PhysicalKeyStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2PhysicalKeyStatus,
    ) -> ::windows::core::HRESULT,
    pub Handled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2BrowserProcessExitedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2BrowserProcessExitedEventArgs {
    type Vtable = ICoreWebView2BrowserProcessExitedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x79963f77_1484_5a46_b91f_dfc5c1a0ce14);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2BrowserProcessExitedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub BrowserProcessExitKind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2BrowserProcessExitKind,
    ) -> ::windows::core::HRESULT,
    pub BrowserProcessId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2ClientCertificate(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2ClientCertificate {
    type Vtable = ICoreWebView2ClientCertificate_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x091b39f2_68df_52b4_8fb0_fd3561af41f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2ClientCertificate_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Subject: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub Issuer: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub ValidFrom: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub ValidTo: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub DerEncodedSerialNumber: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub PemEncodedIssuerCertificateChain: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    )
        -> ::windows::core::HRESULT,
    pub Kind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2ClientCertificateKind,
    ) -> ::windows::core::HRESULT,
    pub ToPemEncoding: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2ClientCertificateRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2ClientCertificateRequestedEventArgs {
    type Vtable = ICoreWebView2ClientCertificateRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x93287b55_31f9_55a0_b68b_d9841d7e1bf4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2ClientCertificateRequestedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Host: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub Port: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub IsProxy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub AllowedCertificateAuthorities: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub MutuallyTrustedCertificates: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub SelectedCertificate: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub SetSelectedCertificate: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub Handled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2CompositionController(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2CompositionController {
    type Vtable = ICoreWebView2CompositionController_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4fb8b7b3_4a2e_5787_94b9_cc48c4d364d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2CompositionController_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub RootVisualTarget: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetRootVisualTarget: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CursorChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveCursorChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub SendMouseInput: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        eventkind: CoreWebView2MouseEventKind,
        virtualkeys: CoreWebView2MouseEventVirtualKeys,
        mousedata: u32,
        point: ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
    pub SendPointerInput: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        eventkind: CoreWebView2PointerEventKind,
        pointerinfo: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub Cursor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2CompositionController2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2CompositionController2 {
    type Vtable = ICoreWebView2CompositionController2_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x8cef61b9_fa55_547d_aae6_7bcaed4249a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2CompositionController2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2CompositionControllerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2CompositionControllerStatics {
    type Vtable = ICoreWebView2CompositionControllerStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4df0ab1f_7f2a_573b_b81a_b9b531224736);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2CompositionControllerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2ContentLoadingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2ContentLoadingEventArgs {
    type Vtable = ICoreWebView2ContentLoadingEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x6cf95373_946c_5dae_9b3e_0fe23d5aa29f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2ContentLoadingEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsErrorPage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub NavigationId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u64,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2Controller(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2Controller {
    type Vtable = ICoreWebView2Controller_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa588121c_53bf_590e_80e5_29d729cbd743);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Controller_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsVisible: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetIsVisible: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub Bounds: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    pub SetBounds: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    pub ZoomFactor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub SetZoomFactor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows::core::HRESULT,
    pub ParentWindow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub SetParentWindow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub CoreWebView2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub ZoomFactorChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveZoomFactorChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub MoveFocusRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveMoveFocusRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub GotFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveGotFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub LostFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveLostFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub AcceleratorKeyPressed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveAcceleratorKeyPressed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub SetBoundsAndZoomFactor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        bounds: ::windows::Foundation::Rect,
        zoomfactor: f64,
    ) -> ::windows::core::HRESULT,
    pub MoveFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        reason: CoreWebView2MoveFocusReason,
    ) -> ::windows::core::HRESULT,
    pub NotifyParentWindowPositionChanged:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Close:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2Controller2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2Controller2 {
    type Vtable = ICoreWebView2Controller2_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x0069c40b_2e8a_513f_9d9d_e0c2b64f7200);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Controller2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DefaultBackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub SetDefaultBackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2Controller3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2Controller3 {
    type Vtable = ICoreWebView2Controller3_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xe5bae214_791a_5d13_9b76_a257d9fda2ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Controller3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub RasterizationScale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub SetRasterizationScale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows::core::HRESULT,
    pub ShouldDetectMonitorScaleChanges: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetShouldDetectMonitorScaleChanges: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    )
        -> ::windows::core::HRESULT,
    pub BoundsMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2BoundsMode,
    ) -> ::windows::core::HRESULT,
    pub SetBoundsMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CoreWebView2BoundsMode,
    ) -> ::windows::core::HRESULT,
    pub RasterizationScaleChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveRasterizationScaleChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2ControllerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2ControllerFactory {
    type Vtable = ICoreWebView2ControllerFactory_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x270b2c5b_c3a9_53d8_a5ca_262ea9ea62e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2ControllerFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2ControllerWindowReference(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2ControllerWindowReference {
    type Vtable = ICoreWebView2ControllerWindowReference_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x0feddad4_48a3_5cc4_9f61_e7adfd1e9c76);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2ControllerWindowReference_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub WindowHandle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u64,
    ) -> ::windows::core::HRESULT,
    pub CoreWindow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2ControllerWindowReferenceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2ControllerWindowReferenceStatics {
    type Vtable = ICoreWebView2ControllerWindowReferenceStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xddf6ebf1_ebc6_5a34_9008_661c3a2eb767);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2ControllerWindowReferenceStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateFromWindowHandle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        windowhandle: u64,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub CreateFromCoreWindow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        corewindow: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2Cookie(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2Cookie {
    type Vtable = ICoreWebView2Cookie_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x52f670fe_8ca2_5aad_aedb_25f7903b7038);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Cookie_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub Domain: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub Path: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub Expires: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub SetExpires: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows::core::HRESULT,
    pub IsHttpOnly: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetIsHttpOnly: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub SameSite: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2CookieSameSiteKind,
    ) -> ::windows::core::HRESULT,
    pub SetSameSite: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CoreWebView2CookieSameSiteKind,
    ) -> ::windows::core::HRESULT,
    pub IsSecure: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetIsSecure: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub IsSession: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2CookieManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2CookieManager {
    type Vtable = ICoreWebView2CookieManager_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4098f516_adca_5563_aaa5_d7affd847aa3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2CookieManager_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateCookie: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        domain: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub CopyCookie: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        cookieparam: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub AddOrUpdateCookie: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        cookie: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub DeleteCookie: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        cookie: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub DeleteCookies: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        uri: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub DeleteCookiesWithDomainAndPath: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        domain: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub DeleteAllCookies:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2DOMContentLoadedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2DOMContentLoadedEventArgs {
    type Vtable = ICoreWebView2DOMContentLoadedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc474d0a3_24ac_59fc_b78b_da7562a6a052);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2DOMContentLoadedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub NavigationId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u64,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2DevToolsProtocolEventReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2DevToolsProtocolEventReceivedEventArgs {
    type Vtable = ICoreWebView2DevToolsProtocolEventReceivedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xb6a4b41d_fd18_59fa_923a_c57555d960ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2DevToolsProtocolEventReceivedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ParameterObjectAsJson: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2DevToolsProtocolEventReceiver(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2DevToolsProtocolEventReceiver {
    type Vtable = ICoreWebView2DevToolsProtocolEventReceiver_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xb2a2be79_65fc_5537_8715_3d92bf31090b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2DevToolsProtocolEventReceiver_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DevToolsProtocolEventReceived: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveDevToolsProtocolEventReceived: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    )
        -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Web_WebView2_Core'*"]
#[repr(transparent)]
pub struct ICoreWebView2DispatchAdapter(::windows::core::IUnknown);
impl ICoreWebView2DispatchAdapter {
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WrapNamedObject)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                adapter.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
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
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WrapObject)(
                ::core::mem::transmute_copy(this),
                unwrapped.into_param().abi(),
                adapter.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn UnwrapObject<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        wrapped: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UnwrapObject)(
                ::core::mem::transmute_copy(this),
                wrapped.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: 'Web_WebView2_Core'*"]
    pub fn Clean(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Clean)(::core::mem::transmute_copy(this)).ok()
        }
    }
}
impl ::core::convert::From<ICoreWebView2DispatchAdapter> for ::windows::core::IUnknown {
    fn from(value: ICoreWebView2DispatchAdapter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICoreWebView2DispatchAdapter> for ::windows::core::IUnknown {
    fn from(value: &ICoreWebView2DispatchAdapter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for ICoreWebView2DispatchAdapter
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a ICoreWebView2DispatchAdapter
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICoreWebView2DispatchAdapter> for ::windows::core::IInspectable {
    fn from(value: ICoreWebView2DispatchAdapter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICoreWebView2DispatchAdapter> for ::windows::core::IInspectable {
    fn from(value: &ICoreWebView2DispatchAdapter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for ICoreWebView2DispatchAdapter
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a ICoreWebView2DispatchAdapter
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICoreWebView2DispatchAdapter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICoreWebView2DispatchAdapter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICoreWebView2DispatchAdapter {}
impl ::core::fmt::Debug for ICoreWebView2DispatchAdapter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICoreWebView2DispatchAdapter")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ICoreWebView2DispatchAdapter {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{7888a42d-18f3-5966-80cb-8cc25351bd0a}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ICoreWebView2DispatchAdapter {
    type Vtable = ICoreWebView2DispatchAdapter_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x7888a42d_18f3_5966_80cb_8cc25351bd0a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2DispatchAdapter_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub WrapNamedObject: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        adapter: ::windows::core::RawPtr,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub WrapObject: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        unwrapped: *mut ::core::ffi::c_void,
        adapter: ::windows::core::RawPtr,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub UnwrapObject: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        wrapped: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub Clean:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2DownloadOperation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2DownloadOperation {
    type Vtable = ICoreWebView2DownloadOperation_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xafe73e6b_e760_5a06_9bf6_1e743c13cd2d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2DownloadOperation_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Uri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub ContentDisposition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub MimeType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub TotalBytesToReceive: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i64,
    ) -> ::windows::core::HRESULT,
    pub BytesReceived: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i64,
    ) -> ::windows::core::HRESULT,
    pub EstimatedEndTime: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub ResultFilePath: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2DownloadState,
    ) -> ::windows::core::HRESULT,
    pub InterruptReason: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2DownloadInterruptReason,
    ) -> ::windows::core::HRESULT,
    pub CanResume: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub BytesReceivedChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveBytesReceivedChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub EstimatedEndTimeChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveEstimatedEndTimeChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
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
    pub Cancel:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Pause:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Resume:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2DownloadStartingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2DownloadStartingEventArgs {
    type Vtable = ICoreWebView2DownloadStartingEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x45d982ba_9256_5b35_b023_26a438599110);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2DownloadStartingEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DownloadOperation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub ResultFilePath: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetResultFilePath: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub Handled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2Environment(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2Environment {
    type Vtable = ICoreWebView2Environment_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xd8cc7831_b783_556b_b9ce_899c1e95d585);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Environment_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub BrowserVersionString: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub NewBrowserVersionAvailable: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveNewBrowserVersionAvailable: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    )
        -> ::windows::core::HRESULT,
    pub CreateCoreWebView2ControllerAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        parentwindow: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    )
        -> ::windows::core::HRESULT,
    pub CreateWebResourceResponse: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        content: ::windows::core::RawPtr,
        statuscode: i32,
        reasonphrase: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        headers: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2Environment2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2Environment2 {
    type Vtable = ICoreWebView2Environment2_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x0b634668_1017_5fc7_9921_f1f51866a8c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Environment2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateWebResourceRequest: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        uri: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        method: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        postdata: ::windows::core::RawPtr,
        headers: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2Environment3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2Environment3 {
    type Vtable = ICoreWebView2Environment3_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x5e33f46c_c0b9_5126_8840_17f9c11b3a8a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Environment3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateCoreWebView2CompositionControllerAsync:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            parentwindow: ::windows::core::RawPtr,
            result__: *mut ::windows::core::RawPtr,
        ) -> ::windows::core::HRESULT,
    pub CreateCoreWebView2PointerInfo: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2Environment4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2Environment4 {
    type Vtable = ICoreWebView2Environment4_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x6db697da_eebd_5818_8790_1fe57ef319e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Environment4_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2Environment5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2Environment5 {
    type Vtable = ICoreWebView2Environment5_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xf33399af_e4d3_59dc_ac38_8397aadcedb1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Environment5_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub BrowserProcessExited: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveBrowserProcessExited: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2Environment6(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2Environment6 {
    type Vtable = ICoreWebView2Environment6_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x965d538f_5958_5d98_8972_f622021df402);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Environment6_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreatePrintSettings: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2EnvironmentOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2EnvironmentOptions {
    type Vtable = ICoreWebView2EnvironmentOptions_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x25d6dc39_0062_5735_8b09_a6f535f19e97);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2EnvironmentOptions_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AdditionalBrowserArguments: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetAdditionalBrowserArguments: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub Language: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub TargetCompatibleBrowserVersion: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetTargetCompatibleBrowserVersion: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    )
        -> ::windows::core::HRESULT,
    pub AllowSingleSignOnUsingOSPrimaryAccount:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows::core::HRESULT,
    pub SetAllowSingleSignOnUsingOSPrimaryAccount:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            value: bool,
        ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2EnvironmentOptions_Manual(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2EnvironmentOptions_Manual {
    type Vtable = ICoreWebView2EnvironmentOptions_Manual_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x1f104443_ea93_5a37_b791_34e6a31172ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2EnvironmentOptions_Manual_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2EnvironmentStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2EnvironmentStatics {
    type Vtable = ICoreWebView2EnvironmentStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x0e33f804_f20b_5635_8491_162aaa27517b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2EnvironmentStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub CreateWithOptionsAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        browserexecutablefolder: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        userdatafolder: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        options: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetAvailableBrowserVersionString: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    )
        -> ::windows::core::HRESULT,
    pub GetAvailableBrowserVersionString2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        browserexecutablefolder: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    )
        -> ::windows::core::HRESULT,
    pub CompareBrowserVersionString: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        browserversionstring1: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        browserversionstring2: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2Frame(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2Frame {
    type Vtable = ICoreWebView2Frame_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x02ffcbf9_19e7_5bb8_8273_346420fb1503);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Frame_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub NameChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveNameChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub Destroyed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveDestroyed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveHostObjectFromScript: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub IsDestroyed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2FrameCreatedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2FrameCreatedEventArgs {
    type Vtable = ICoreWebView2FrameCreatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x527b01b8_fc6d_5543_8dce_96cdfdb32c81);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2FrameCreatedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Frame: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2FrameInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2FrameInfo {
    type Vtable = ICoreWebView2FrameInfo_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xf9b82e06_73f3_513b_bc2c_445ddedba976);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2FrameInfo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub Source: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2HttpHeadersCollectionIterator(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2HttpHeadersCollectionIterator {
    type Vtable = ICoreWebView2HttpHeadersCollectionIterator_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xadf264ee_d980_5f48_a60e_8705de046608);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2HttpHeadersCollectionIterator_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2HttpRequestHeaders(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2HttpRequestHeaders {
    type Vtable = ICoreWebView2HttpRequestHeaders_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xdc2226c7_3515_55bb_bcb2_57b78f86b91d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2HttpRequestHeaders_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetHeader: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub GetHeaders: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub Contains: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetHeader: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub RemoveHeader: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2HttpResponseHeaders(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2HttpResponseHeaders {
    type Vtable = ICoreWebView2HttpResponseHeaders_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xf3d383e9_747f_5574_8662_9a6b920cecd4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2HttpResponseHeaders_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AppendHeader: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub Contains: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub GetHeader: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub GetHeaders: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2MoveFocusRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2MoveFocusRequestedEventArgs {
    type Vtable = ICoreWebView2MoveFocusRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x2e29103b_ecdd_5c1d_b288_3f066d608919);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2MoveFocusRequestedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Reason: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2MoveFocusReason,
    ) -> ::windows::core::HRESULT,
    pub Handled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2NavigationCompletedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2NavigationCompletedEventArgs {
    type Vtable = ICoreWebView2NavigationCompletedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4865e238_036a_5664_95a3_447ec44cf498);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2NavigationCompletedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsSuccess: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub WebErrorStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2WebErrorStatus,
    ) -> ::windows::core::HRESULT,
    pub NavigationId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u64,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2NavigationStartingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2NavigationStartingEventArgs {
    type Vtable = ICoreWebView2NavigationStartingEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x548d23d3_fea3_5616_bd05_ae08066c86d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2NavigationStartingEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Uri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub IsUserInitiated: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub IsRedirected: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub RequestHeaders: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub NavigationId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u64,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2NewWindowRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2NewWindowRequestedEventArgs {
    type Vtable = ICoreWebView2NewWindowRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xe6e013ba_aec8_532e_9ac9_1590af7b25ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2NewWindowRequestedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Uri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub NewWindow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub SetNewWindow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub Handled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub IsUserInitiated: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub WindowFeatures: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2NewWindowRequestedEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2NewWindowRequestedEventArgs2 {
    type Vtable = ICoreWebView2NewWindowRequestedEventArgs2_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xf4806259_e63a_5c0b_a02c_5f10e11094f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2NewWindowRequestedEventArgs2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2PermissionRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2PermissionRequestedEventArgs {
    type Vtable = ICoreWebView2PermissionRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x118bdd9b_cef1_5910_929e_c1a321328239);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2PermissionRequestedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Uri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub PermissionKind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2PermissionKind,
    ) -> ::windows::core::HRESULT,
    pub IsUserInitiated: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2PermissionState,
    ) -> ::windows::core::HRESULT,
    pub SetState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CoreWebView2PermissionState,
    ) -> ::windows::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2PointerInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2PointerInfo {
    type Vtable = ICoreWebView2PointerInfo_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc3860e0d_c018_5a84_bc06_9f8f7b275dff);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2PointerInfo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub PointerKind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub SetPointerKind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: u32,
    ) -> ::windows::core::HRESULT,
    pub PointerId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub SetPointerId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: u32,
    ) -> ::windows::core::HRESULT,
    pub FrameId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub SetFrameId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: u32,
    ) -> ::windows::core::HRESULT,
    pub PointerFlags: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub SetPointerFlags: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: u32,
    ) -> ::windows::core::HRESULT,
    pub PointerDeviceRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    pub SetPointerDeviceRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    pub DisplayRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    pub SetDisplayRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    pub PixelLocation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
    pub SetPixelLocation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
    pub HimetricLocation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
    pub SetHimetricLocation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
    pub PixelLocationRaw: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
    pub SetPixelLocationRaw: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
    pub HimetricLocationRaw: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
    pub SetHimetricLocationRaw: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
    pub Time: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub SetTime: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: u32,
    ) -> ::windows::core::HRESULT,
    pub HistoryCount: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub SetHistoryCount: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: u32,
    ) -> ::windows::core::HRESULT,
    pub InputData: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub SetInputData: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows::core::HRESULT,
    pub KeyStates: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub SetKeyStates: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: u32,
    ) -> ::windows::core::HRESULT,
    pub PerformanceCount: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u64,
    ) -> ::windows::core::HRESULT,
    pub SetPerformanceCount: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: u64,
    ) -> ::windows::core::HRESULT,
    pub ButtonChangeKind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub SetButtonChangeKind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows::core::HRESULT,
    pub PenFlags: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub SetPenFlags: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: u32,
    ) -> ::windows::core::HRESULT,
    pub PenMask: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub SetPenMask: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: u32,
    ) -> ::windows::core::HRESULT,
    pub PenPressure: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub SetPenPressure: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: u32,
    ) -> ::windows::core::HRESULT,
    pub PenRotation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub SetPenRotation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: u32,
    ) -> ::windows::core::HRESULT,
    pub PenTiltX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub SetPenTiltX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows::core::HRESULT,
    pub PenTiltY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub SetPenTiltY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows::core::HRESULT,
    pub TouchFlags: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub SetTouchFlags: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: u32,
    ) -> ::windows::core::HRESULT,
    pub TouchMask: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub SetTouchMask: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: u32,
    ) -> ::windows::core::HRESULT,
    pub TouchContact: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    pub SetTouchContact: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    pub TouchContactRaw: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    pub SetTouchContactRaw: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    pub TouchOrientation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub SetTouchOrientation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: u32,
    ) -> ::windows::core::HRESULT,
    pub TouchPressure: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub SetTouchPressure: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: u32,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2PrintSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2PrintSettings {
    type Vtable = ICoreWebView2PrintSettings_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x9c75c8c0_ef3d_58a8_9a8c_18eed9fd0f16);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2PrintSettings_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Orientation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2PrintOrientation,
    ) -> ::windows::core::HRESULT,
    pub SetOrientation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CoreWebView2PrintOrientation,
    ) -> ::windows::core::HRESULT,
    pub ScaleFactor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub SetScaleFactor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows::core::HRESULT,
    pub PageWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub SetPageWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows::core::HRESULT,
    pub PageHeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub SetPageHeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows::core::HRESULT,
    pub MarginTop: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub SetMarginTop: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows::core::HRESULT,
    pub MarginBottom: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub SetMarginBottom: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows::core::HRESULT,
    pub MarginLeft: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub SetMarginLeft: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows::core::HRESULT,
    pub MarginRight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub SetMarginRight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows::core::HRESULT,
    pub ShouldPrintBackgrounds: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetShouldPrintBackgrounds: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub ShouldPrintSelectionOnly: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetShouldPrintSelectionOnly: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub ShouldPrintHeaderAndFooter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetShouldPrintHeaderAndFooter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub HeaderTitle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetHeaderTitle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub FooterUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetFooterUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2ProcessFailedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2ProcessFailedEventArgs {
    type Vtable = ICoreWebView2ProcessFailedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x25a8f8c9_d944_539d_afa3_24172b48ef47);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2ProcessFailedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ProcessFailedKind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2ProcessFailedKind,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2ProcessFailedEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2ProcessFailedEventArgs2 {
    type Vtable = ICoreWebView2ProcessFailedEventArgs2_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc5d9c952_b456_5dc7_9f76_fde967484af5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2ProcessFailedEventArgs2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Reason: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2ProcessFailedReason,
    ) -> ::windows::core::HRESULT,
    pub ExitCode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub ProcessDescription: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub FrameInfosForFailedProcess: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2ScriptDialogOpeningEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2ScriptDialogOpeningEventArgs {
    type Vtable = ICoreWebView2ScriptDialogOpeningEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa4315212_c7eb_568a_86e4_c61e31ba6cda);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2ScriptDialogOpeningEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Uri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub Kind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2ScriptDialogKind,
    ) -> ::windows::core::HRESULT,
    pub Message: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub DefaultText: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub ResultText: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetResultText: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub Accept:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2Settings(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2Settings {
    type Vtable = ICoreWebView2Settings_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x003b325e_74cd_52dd_8024_ebb8be38e48e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Settings_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsScriptEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetIsScriptEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub IsWebMessageEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetIsWebMessageEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub AreDefaultScriptDialogsEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetAreDefaultScriptDialogsEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    )
        -> ::windows::core::HRESULT,
    pub IsStatusBarEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetIsStatusBarEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub AreDevToolsEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetAreDevToolsEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub AreDefaultContextMenusEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetAreDefaultContextMenusEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    )
        -> ::windows::core::HRESULT,
    pub AreHostObjectsAllowed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetAreHostObjectsAllowed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub IsZoomControlEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetIsZoomControlEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub IsBuiltInErrorPageEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetIsBuiltInErrorPageEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2Settings2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2Settings2 {
    type Vtable = ICoreWebView2Settings2_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x377d3480_fdb2_56e7_bade_507d352887e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Settings2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub UserAgent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetUserAgent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2Settings3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2Settings3 {
    type Vtable = ICoreWebView2Settings3_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x52200f01_5309_5b2e_a03c_3d2677591940);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Settings3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AreBrowserAcceleratorKeysEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    )
        -> ::windows::core::HRESULT,
    pub SetAreBrowserAcceleratorKeysEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    )
        -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2Settings4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2Settings4 {
    type Vtable = ICoreWebView2Settings4_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xd6a955f0_daef_5a6a_a6f6_c72f0ede7620);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Settings4_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsPasswordAutosaveEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetIsPasswordAutosaveEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub IsGeneralAutofillEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetIsGeneralAutofillEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2Settings5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2Settings5 {
    type Vtable = ICoreWebView2Settings5_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xafc42b23_4839_5d73_acf7_e0335631abf5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Settings5_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsPinchZoomEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetIsPinchZoomEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2Settings6(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2Settings6 {
    type Vtable = ICoreWebView2Settings6_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x3fe4ae85_0540_5bf1_b4d9_99ec57aa64f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Settings6_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsSwipeNavigationEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetIsSwipeNavigationEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2Settings_Manual(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2Settings_Manual {
    type Vtable = ICoreWebView2Settings_Manual_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x0a538c87_e000_511c_87ca_ded3413d03da);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Settings_Manual_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub HostObjectDispatchAdapter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub SetHostObjectDispatchAdapter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2SourceChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2SourceChangedEventArgs {
    type Vtable = ICoreWebView2SourceChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xca437b2c_6a18_5552_b749_b198f8cc34d9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2SourceChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsNewDocument: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2WebMessageReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2WebMessageReceivedEventArgs {
    type Vtable = ICoreWebView2WebMessageReceivedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xeb066159_b725_5d5b_adc8_f5d7b9290304);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2WebMessageReceivedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Source: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub WebMessageAsJson: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub TryGetWebMessageAsString: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2WebResourceRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2WebResourceRequest {
    type Vtable = ICoreWebView2WebResourceRequest_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x5c742259_67d2_5df2_8382_0f201b4d7197);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2WebResourceRequest_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Uri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub Method: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetMethod: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub Content: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub SetContent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub Headers: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2WebResourceRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2WebResourceRequestedEventArgs {
    type Vtable = ICoreWebView2WebResourceRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x577f1fc4_c943_54a9_9700_bd469b48bd41);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2WebResourceRequestedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Request: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub Response: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub SetResponse: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub ResourceContext: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2WebResourceContext,
    ) -> ::windows::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2WebResourceResponse(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2WebResourceResponse {
    type Vtable = ICoreWebView2WebResourceResponse_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x14621923_e485_5f44_8f5d_bd4243bc398f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2WebResourceResponse_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Content: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub SetContent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub Headers: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub SetStatusCode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows::core::HRESULT,
    pub ReasonPhrase: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetReasonPhrase: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2WebResourceResponseReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2WebResourceResponseReceivedEventArgs {
    type Vtable = ICoreWebView2WebResourceResponseReceivedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x12424671_9711_54f4_bcdf_5f307add6ec2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2WebResourceResponseReceivedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Request: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub Response: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2WebResourceResponseView(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2WebResourceResponseView {
    type Vtable = ICoreWebView2WebResourceResponseView_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x33ee060b_b578_5698_b541_fef87fe7fe72);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2WebResourceResponseView_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Headers: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub ReasonPhrase: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub GetContentAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2WindowFeatures(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2WindowFeatures {
    type Vtable = ICoreWebView2WindowFeatures_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xee8686d6_056f_5e06_824f_4e2a24c1c1d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2WindowFeatures_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub HasPosition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub HasSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub Left: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub Top: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub Height: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub Width: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub ShouldDisplayMenuBar: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub ShouldDisplayStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub ShouldDisplayToolbar: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub ShouldDisplayScrollBars: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2_2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2_2 {
    type Vtable = ICoreWebView2_2_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x578cb133_2873_5408_bd9e_389bbe9fa7fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2_2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CookieManager: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub Environment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub WebResourceResponseReceived: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveWebResourceResponseReceived: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    )
        -> ::windows::core::HRESULT,
    pub DOMContentLoaded: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveDOMContentLoaded: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub NavigateWithWebResourceRequest: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        request: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2_3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2_3 {
    type Vtable = ICoreWebView2_3_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa8c76ae7_6170_5dfe_8f00_79cd76a9b4d9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2_3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsSuspended: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub TrySuspendAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub Resume:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetVirtualHostNameToFolderMapping: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        hostname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        folderpath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        accesskind: CoreWebView2HostResourceAccessKind,
    )
        -> ::windows::core::HRESULT,
    pub ClearVirtualHostNameToFolderMapping: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        hostname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    )
        -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2_4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2_4 {
    type Vtable = ICoreWebView2_4_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4ac595ce_1502_5775_b2c8_22c11a369c25);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2_4_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub FrameCreated: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveFrameCreated: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub DownloadStarting: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveDownloadStarting: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2_5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2_5 {
    type Vtable = ICoreWebView2_5_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xdd6af643_220c_5dc6_b0a8_22c41e472595);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2_5_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ClientCertificateRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveClientCertificateRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    )
        -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2_6(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2_6 {
    type Vtable = ICoreWebView2_6_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x92b34b96_853d_5bb6_ac52_30297ce805f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2_6_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub OpenTaskManagerWindow:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWebView2_7(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWebView2_7 {
    type Vtable = ICoreWebView2_7_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xf9b7107a_2e09_5596_a033_911ba12315f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2_7_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub PrintToPdfAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resultfilepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        printsettings: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
