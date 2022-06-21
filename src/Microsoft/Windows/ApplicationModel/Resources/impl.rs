pub trait IResourceContext_Impl: Sized {
    fn QualifierValues(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IMap<
            ::windows::core::HSTRING,
            ::windows::core::HSTRING,
        >,
    >;
}
impl ::windows::core::RuntimeName for IResourceContext {
    const NAME: &'static str = "Microsoft.Windows.ApplicationModel.Resources.IResourceContext";
}
impl IResourceContext_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IResourceContext_Impl,
        const OFFSET: isize,
    >() -> IResourceContext_Vtbl {
        unsafe extern "system" fn QualifierValues<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IResourceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::core::RawPtr,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QualifierValues() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, IResourceContext, OFFSET>(),
            QualifierValues: QualifierValues::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResourceContext as ::windows::core::Interface>::IID
    }
}
pub trait IResourceManager_Impl: Sized {
    fn MainResourceMap(&self) -> ::windows::core::Result<ResourceMap>;
    fn CreateResourceContext(&self) -> ::windows::core::Result<ResourceContext>;
    fn ResourceNotFound(
        &self,
        handler: &::core::option::Option<
            ::windows::Foundation::TypedEventHandler<ResourceManager, ResourceNotFoundEventArgs>,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken>;
    fn RemoveResourceNotFound(
        &self,
        token: &::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IResourceManager {
    const NAME: &'static str = "Microsoft.Windows.ApplicationModel.Resources.IResourceManager";
}
impl IResourceManager_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IResourceManager_Impl,
        const OFFSET: isize,
    >() -> IResourceManager_Vtbl {
        unsafe extern "system" fn MainResourceMap<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IResourceManager_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::core::RawPtr,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MainResourceMap() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateResourceContext<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IResourceManager_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::core::RawPtr,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateResourceContext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResourceNotFound<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IResourceManager_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            handler: ::windows::core::RawPtr,
            result__: *mut ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ResourceNotFound(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveResourceNotFound<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IResourceManager_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            token: ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveResourceNotFound(::core::mem::transmute(&token))
                .into()
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, IResourceManager, OFFSET>(),
            MainResourceMap: MainResourceMap::<Identity, Impl, OFFSET>,
            CreateResourceContext: CreateResourceContext::<Identity, Impl, OFFSET>,
            ResourceNotFound: ResourceNotFound::<Identity, Impl, OFFSET>,
            RemoveResourceNotFound: RemoveResourceNotFound::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResourceManager as ::windows::core::Interface>::IID
    }
}
