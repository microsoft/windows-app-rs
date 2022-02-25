pub trait IPointerPointTransform_Impl: Sized {
    fn Inverse(&self) -> ::windows::core::Result<IPointerPointTransform>;
    fn TryTransform(
        &self,
        inpoint: &::windows::Foundation::Point,
        outpoint: &mut ::windows::Foundation::Point,
    ) -> ::windows::core::Result<bool>;
    fn TryTransformBounds(
        &self,
        inrect: &::windows::Foundation::Rect,
        outrect: &mut ::windows::Foundation::Rect,
    ) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for IPointerPointTransform {
    const NAME: &'static str = "Microsoft.UI.Input.IPointerPointTransform";
}
impl IPointerPointTransform_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl,
        Impl: IPointerPointTransform_Impl,
        const OFFSET: isize,
    >() -> IPointerPointTransform_Vtbl {
        unsafe extern "system" fn Inverse<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IPointerPointTransform_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::core::RawPtr,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Inverse() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryTransform<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IPointerPointTransform_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            inpoint: ::windows::Foundation::Point,
            outpoint: *mut ::windows::Foundation::Point,
            result__: *mut bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TryTransform(
                ::core::mem::transmute(&inpoint),
                ::core::mem::transmute_copy(&outpoint),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryTransformBounds<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IPointerPointTransform_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            inrect: ::windows::Foundation::Rect,
            outrect: *mut ::windows::Foundation::Rect,
            result__: *mut bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TryTransformBounds(
                ::core::mem::transmute(&inrect),
                ::core::mem::transmute_copy(&outrect),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPointerPointTransform, OFFSET>(
            ),
            Inverse: Inverse::<Identity, Impl, OFFSET>,
            TryTransform: TryTransform::<Identity, Impl, OFFSET>,
            TryTransformBounds: TryTransformBounds::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPointerPointTransform as ::windows::core::Interface>::IID
    }
}
