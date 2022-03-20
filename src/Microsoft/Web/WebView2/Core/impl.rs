pub trait ICoreWebView2DispatchAdapter_Impl: Sized {
    fn WrapNamedObject(
        &self,
        name: &::windows::core::HSTRING,
        adapter: &::core::option::Option<ICoreWebView2DispatchAdapter>,
    ) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn WrapObject(
        &self,
        unwrapped: &::core::option::Option<::windows::core::IInspectable>,
        adapter: &::core::option::Option<ICoreWebView2DispatchAdapter>,
    ) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn UnwrapObject(
        &self,
        wrapped: &::core::option::Option<::windows::core::IInspectable>,
    ) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn Clean(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICoreWebView2DispatchAdapter {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.ICoreWebView2DispatchAdapter";
}
impl ICoreWebView2DispatchAdapter_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl,
        Impl: ICoreWebView2DispatchAdapter_Impl,
        const OFFSET: isize,
    >() -> ICoreWebView2DispatchAdapter_Vtbl {
        unsafe extern "system" fn WrapNamedObject<
            Identity: ::windows::core::IUnknownImpl,
            Impl: ICoreWebView2DispatchAdapter_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
            adapter: ::windows::core::RawPtr,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).WrapNamedObject(
                ::core::mem::transmute(&name),
                ::core::mem::transmute(&adapter),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WrapObject<
            Identity: ::windows::core::IUnknownImpl,
            Impl: ICoreWebView2DispatchAdapter_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            unwrapped: *mut ::core::ffi::c_void,
            adapter: ::windows::core::RawPtr,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).WrapObject(
                ::core::mem::transmute(&unwrapped),
                ::core::mem::transmute(&adapter),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnwrapObject<
            Identity: ::windows::core::IUnknownImpl,
            Impl: ICoreWebView2DispatchAdapter_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            wrapped: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UnwrapObject(::core::mem::transmute(&wrapped)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clean<
            Identity: ::windows::core::IUnknownImpl,
            Impl: ICoreWebView2DispatchAdapter_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Clean().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<
                Identity,
                ICoreWebView2DispatchAdapter,
                OFFSET,
            >(),
            WrapNamedObject: WrapNamedObject::<Identity, Impl, OFFSET>,
            WrapObject: WrapObject::<Identity, Impl, OFFSET>,
            UnwrapObject: UnwrapObject::<Identity, Impl, OFFSET>,
            Clean: Clean::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreWebView2DispatchAdapter as ::windows::core::Interface>::IID
    }
}
