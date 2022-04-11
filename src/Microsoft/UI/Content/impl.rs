pub trait IContentSiteBridge_Impl: Sized + ::windows::Foundation::IClosable_Impl {
    fn IsConnected(&self) -> ::windows::core::Result<bool>;
    fn OverrideScale(&self) -> ::windows::core::Result<f32>;
    fn SetOverrideScale(&self, value: f32) -> ::windows::core::Result<()>;
    fn Connect(
        &self,
        content: &::core::option::Option<ContentIsland>,
    ) -> ::windows::core::Result<()>;
    fn TryCreatePopupSiteBridge(&self) -> ::windows::core::Result<PopupWindowSiteBridge>;
}
impl ::windows::core::RuntimeName for IContentSiteBridge {
    const NAME: &'static str = "Microsoft.UI.Content.IContentSiteBridge";
}
impl IContentSiteBridge_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl,
        Impl: IContentSiteBridge_Impl,
        const OFFSET: isize,
    >() -> IContentSiteBridge_Vtbl {
        unsafe extern "system" fn IsConnected<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IContentSiteBridge_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsConnected() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OverrideScale<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IContentSiteBridge_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut f32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OverrideScale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOverrideScale<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IContentSiteBridge_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: f32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOverrideScale(value).into()
        }
        unsafe extern "system" fn Connect<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IContentSiteBridge_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            content: ::windows::core::RawPtr,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Connect(::core::mem::transmute(&content)).into()
        }
        unsafe extern "system" fn TryCreatePopupSiteBridge<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IContentSiteBridge_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::core::RawPtr,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TryCreatePopupSiteBridge() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, IContentSiteBridge, OFFSET>(
            ),
            IsConnected: IsConnected::<Identity, Impl, OFFSET>,
            OverrideScale: OverrideScale::<Identity, Impl, OFFSET>,
            SetOverrideScale: SetOverrideScale::<Identity, Impl, OFFSET>,
            Connect: Connect::<Identity, Impl, OFFSET>,
            TryCreatePopupSiteBridge: TryCreatePopupSiteBridge::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContentSiteBridge as ::windows::core::Interface>::IID
    }
}
pub trait IContentWindowBridge_Impl: Sized {
    fn CurrentOrientation(&self) -> ::windows::core::Result<ContentDisplayOrientations>;
    fn DisplayId(&self) -> ::windows::core::Result<super::DisplayId>;
    fn DisplayScale(&self) -> ::windows::core::Result<f32>;
    fn NativeOrientation(&self) -> ::windows::core::Result<ContentDisplayOrientations>;
    fn WindowId(&self) -> ::windows::core::Result<super::WindowId>;
    fn SettingsChanged(
        &self,
        handler: &::core::option::Option<
            ::windows::Foundation::TypedEventHandler<
                IContentWindowBridge,
                ::windows::core::IInspectable,
            >,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken>;
    fn RemoveSettingsChanged(
        &self,
        token: &::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()>;
    fn StateChanged(
        &self,
        handler: &::core::option::Option<
            ::windows::Foundation::TypedEventHandler<
                IContentWindowBridge,
                ::windows::core::IInspectable,
            >,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken>;
    fn RemoveStateChanged(
        &self,
        token: &::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()>;
    fn ThemeChanged(
        &self,
        handler: &::core::option::Option<
            ::windows::Foundation::TypedEventHandler<
                IContentWindowBridge,
                ::windows::core::IInspectable,
            >,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken>;
    fn RemoveThemeChanged(
        &self,
        token: &::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IContentWindowBridge {
    const NAME: &'static str = "Microsoft.UI.Content.IContentWindowBridge";
}
impl IContentWindowBridge_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl,
        Impl: IContentWindowBridge_Impl,
        const OFFSET: isize,
    >() -> IContentWindowBridge_Vtbl {
        unsafe extern "system" fn CurrentOrientation<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IContentWindowBridge_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ContentDisplayOrientations,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentOrientation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayId<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IContentWindowBridge_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut super::DisplayId,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DisplayId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayScale<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IContentWindowBridge_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut f32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DisplayScale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NativeOrientation<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IContentWindowBridge_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ContentDisplayOrientations,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NativeOrientation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WindowId<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IContentWindowBridge_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut super::WindowId,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).WindowId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SettingsChanged<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IContentWindowBridge_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            handler: ::windows::core::RawPtr,
            result__: *mut ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SettingsChanged(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSettingsChanged<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IContentWindowBridge_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            token: ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .RemoveSettingsChanged(::core::mem::transmute(&token))
                .into()
        }
        unsafe extern "system" fn StateChanged<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IContentWindowBridge_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            handler: ::windows::core::RawPtr,
            result__: *mut ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StateChanged(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStateChanged<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IContentWindowBridge_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            token: ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .RemoveStateChanged(::core::mem::transmute(&token))
                .into()
        }
        unsafe extern "system" fn ThemeChanged<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IContentWindowBridge_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            handler: ::windows::core::RawPtr,
            result__: *mut ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ThemeChanged(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveThemeChanged<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IContentWindowBridge_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            token: ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .RemoveThemeChanged(::core::mem::transmute(&token))
                .into()
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, IContentWindowBridge, OFFSET>(
            ),
            CurrentOrientation: CurrentOrientation::<Identity, Impl, OFFSET>,
            DisplayId: DisplayId::<Identity, Impl, OFFSET>,
            DisplayScale: DisplayScale::<Identity, Impl, OFFSET>,
            NativeOrientation: NativeOrientation::<Identity, Impl, OFFSET>,
            WindowId: WindowId::<Identity, Impl, OFFSET>,
            SettingsChanged: SettingsChanged::<Identity, Impl, OFFSET>,
            RemoveSettingsChanged: RemoveSettingsChanged::<Identity, Impl, OFFSET>,
            StateChanged: StateChanged::<Identity, Impl, OFFSET>,
            RemoveStateChanged: RemoveStateChanged::<Identity, Impl, OFFSET>,
            ThemeChanged: ThemeChanged::<Identity, Impl, OFFSET>,
            RemoveThemeChanged: RemoveThemeChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContentWindowBridge as ::windows::core::Interface>::IID
    }
}
