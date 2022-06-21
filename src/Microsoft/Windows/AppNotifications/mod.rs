#[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
#[repr(transparent)]
pub struct AppNotification(::windows::core::IUnknown);
impl AppNotification {
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
    pub fn Tag(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<
                ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
            >::zeroed();
            (::windows::core::Interface::vtable(this).Tag)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
    pub fn SetTag<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetTag)(
                ::windows::core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
    pub fn Group(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<
                ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
            >::zeroed();
            (::windows::core::Interface::vtable(this).Group)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
    pub fn SetGroup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetGroup)(
                ::windows::core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows::core::Interface::vtable(this).Id)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
    pub fn Payload(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<
                ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
            >::zeroed();
            (::windows::core::Interface::vtable(this).Payload)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
    pub fn Progress(&self) -> ::windows::core::Result<AppNotificationProgressData> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).Progress)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationProgressData>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
    pub fn SetProgress<'a, Param0: ::windows::core::IntoParam<'a, AppNotificationProgressData>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetProgress)(
                ::windows::core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
    pub fn Expiration(&self) -> ::windows::core::Result<::windows::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ =
                ::core::mem::MaybeUninit::<::windows::Foundation::DateTime>::zeroed();
            (::windows::core::Interface::vtable(this).Expiration)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
    pub fn SetExpiration<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::DateTime>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetExpiration)(
                ::windows::core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
    pub fn ExpiresOnReboot(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).ExpiresOnReboot)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
    pub fn SetExpiresOnReboot(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetExpiresOnReboot)(
                ::windows::core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
    pub fn Priority(&self) -> ::windows::core::Result<AppNotificationPriority> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppNotificationPriority>::zeroed();
            (::windows::core::Interface::vtable(this).Priority)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationPriority>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
    pub fn SetPriority(&self, value: AppNotificationPriority) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetPriority)(
                ::windows::core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
    pub fn SuppressDisplay(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).SuppressDisplay)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
    pub fn SetSuppressDisplay(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetSuppressDisplay)(
                ::windows::core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
    pub fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        payload: Param0,
    ) -> ::windows::core::Result<AppNotification> {
        Self::IAppNotificationFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(
                ::windows::core::Interface::as_raw(this),
                payload.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotification>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppNotificationFactory<
        R,
        F: FnOnce(&IAppNotificationFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppNotification, IAppNotificationFactory> =
            ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AppNotification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppNotification {}
impl ::core::fmt::Debug for AppNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppNotification").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppNotification {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.AppNotifications.AppNotification;{373a6917-4116-5657-936a-15f99afdd667})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppNotification {
    type Vtable = IAppNotification_Vtbl;
    const IID: ::windows::core::GUID = <IAppNotification as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppNotification {
    const NAME: &'static str = "Microsoft.Windows.AppNotifications.AppNotification";
}
impl ::core::convert::From<AppNotification> for ::windows::core::IUnknown {
    fn from(value: AppNotification) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppNotification> for ::windows::core::IUnknown {
    fn from(value: &AppNotification) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppNotification> for ::windows::core::IInspectable {
    fn from(value: AppNotification) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppNotification> for ::windows::core::IInspectable {
    fn from(value: &AppNotification) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppNotification {}
unsafe impl ::core::marker::Sync for AppNotification {}
#[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
#[repr(transparent)]
pub struct AppNotificationActivatedEventArgs(::windows::core::IUnknown);
impl AppNotificationActivatedEventArgs {
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
    pub fn Argument(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<
                ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
            >::zeroed();
            (::windows::core::Interface::vtable(this).Argument)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
    pub fn UserInput(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IMap<
            ::windows::core::HSTRING,
            ::windows::core::HSTRING,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).UserInput)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Collections::IMap<
                ::windows::core::HSTRING,
                ::windows::core::HSTRING,
            >>(result__)
        }
    }
}
impl ::core::clone::Clone for AppNotificationActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppNotificationActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppNotificationActivatedEventArgs {}
impl ::core::fmt::Debug for AppNotificationActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppNotificationActivatedEventArgs")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppNotificationActivatedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.AppNotifications.AppNotificationActivatedEventArgs;{7a8afaf9-31cb-51d5-82be-db6bd5878b77})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppNotificationActivatedEventArgs {
    type Vtable = IAppNotificationActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        <IAppNotificationActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppNotificationActivatedEventArgs {
    const NAME: &'static str =
        "Microsoft.Windows.AppNotifications.AppNotificationActivatedEventArgs";
}
impl ::core::convert::From<AppNotificationActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppNotificationActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppNotificationActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppNotificationActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for AppNotificationActivatedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a AppNotificationActivatedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppNotificationActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppNotificationActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppNotificationActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppNotificationActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for AppNotificationActivatedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a AppNotificationActivatedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppNotificationActivatedEventArgs {}
unsafe impl ::core::marker::Sync for AppNotificationActivatedEventArgs {}
#[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
#[repr(transparent)]
pub struct AppNotificationManager(::windows::core::IUnknown);
impl AppNotificationManager {
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
    pub fn Register(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Register)(::windows::core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
    pub fn Unregister(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Unregister)(
                ::windows::core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
    pub fn UnregisterAll(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).UnregisterAll)(
                ::windows::core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
    pub fn NotificationInvoked<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                AppNotificationManager,
                AppNotificationActivatedEventArgs,
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
            (::windows::core::Interface::vtable(this).NotificationInvoked)(
                ::windows::core::Interface::as_raw(this),
                handler.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
    pub fn RemoveNotificationInvoked<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveNotificationInvoked)(
                ::windows::core::Interface::as_raw(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
    pub fn Show<'a, Param0: ::windows::core::IntoParam<'a, AppNotification>>(
        &self,
        notification: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Show)(
                ::windows::core::Interface::as_raw(this),
                notification.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
    pub fn UpdateAsync<
        'a,
        Param0: ::windows::core::IntoParam<'a, AppNotificationProgressData>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        data: Param0,
        tag: Param1,
        group: Param2,
    ) -> ::windows::core::Result<
        ::windows::Foundation::IAsyncOperation<AppNotificationProgressResult>,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).UpdateAsync)(
                ::windows::core::Interface::as_raw(this),
                data.into_param().abi(),
                tag.into_param().abi(),
                group.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::IAsyncOperation<AppNotificationProgressResult>>(
                result__,
            )
        }
    }
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
    pub fn UpdateAsync2<
        'a,
        Param0: ::windows::core::IntoParam<'a, AppNotificationProgressData>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        data: Param0,
        tag: Param1,
    ) -> ::windows::core::Result<
        ::windows::Foundation::IAsyncOperation<AppNotificationProgressResult>,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).UpdateAsync2)(
                ::windows::core::Interface::as_raw(this),
                data.into_param().abi(),
                tag.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::IAsyncOperation<AppNotificationProgressResult>>(
                result__,
            )
        }
    }
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
    pub fn Setting(&self) -> ::windows::core::Result<AppNotificationSetting> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppNotificationSetting>::zeroed();
            (::windows::core::Interface::vtable(this).Setting)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationSetting>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
    pub fn RemoveByIdAsync(
        &self,
        notificationid: u32,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).RemoveByIdAsync)(
                ::windows::core::Interface::as_raw(this),
                notificationid,
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
    pub fn RemoveByTagAsync<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        tag: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).RemoveByTagAsync)(
                ::windows::core::Interface::as_raw(this),
                tag.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
    pub fn RemoveByTagAndGroupAsync<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        tag: Param0,
        group: Param1,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).RemoveByTagAndGroupAsync)(
                ::windows::core::Interface::as_raw(this),
                tag.into_param().abi(),
                group.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
    pub fn RemoveByGroupAsync<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        group: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).RemoveByGroupAsync)(
                ::windows::core::Interface::as_raw(this),
                group.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
    pub fn RemoveAllAsync(&self) -> ::windows::core::Result<::windows::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).RemoveAllAsync)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
    pub fn GetAllAsync(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::IAsyncOperation<
            ::windows::Foundation::Collections::IVector<AppNotification>,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).GetAllAsync)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::IAsyncOperation<
                ::windows::Foundation::Collections::IVector<AppNotification>,
            >>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
    pub fn Default() -> ::windows::core::Result<AppNotificationManager> {
        Self::IAppNotificationManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).Default)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationManager>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppNotificationManagerStatics<
        R,
        F: FnOnce(&IAppNotificationManagerStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            AppNotificationManager,
            IAppNotificationManagerStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AppNotificationManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppNotificationManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppNotificationManager {}
impl ::core::fmt::Debug for AppNotificationManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppNotificationManager")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppNotificationManager {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.AppNotifications.AppNotificationManager;{55129688-b4bd-550b-ae6b-c24061954d91})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppNotificationManager {
    type Vtable = IAppNotificationManager_Vtbl;
    const IID: ::windows::core::GUID = <IAppNotificationManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppNotificationManager {
    const NAME: &'static str = "Microsoft.Windows.AppNotifications.AppNotificationManager";
}
impl ::core::convert::From<AppNotificationManager> for ::windows::core::IUnknown {
    fn from(value: AppNotificationManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppNotificationManager> for ::windows::core::IUnknown {
    fn from(value: &AppNotificationManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppNotificationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppNotificationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppNotificationManager> for ::windows::core::IInspectable {
    fn from(value: AppNotificationManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppNotificationManager> for ::windows::core::IInspectable {
    fn from(value: &AppNotificationManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppNotificationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a AppNotificationManager
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppNotificationManager {}
unsafe impl ::core::marker::Sync for AppNotificationManager {}
#[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppNotificationPriority(pub i32);
impl AppNotificationPriority {
    pub const Default: Self = Self(0i32);
    pub const High: Self = Self(1i32);
}
impl ::core::marker::Copy for AppNotificationPriority {}
impl ::core::clone::Clone for AppNotificationPriority {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppNotificationPriority {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppNotificationPriority {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppNotificationPriority {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppNotificationPriority")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppNotificationPriority {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.AppNotifications.AppNotificationPriority;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
#[repr(transparent)]
pub struct AppNotificationProgressData(::windows::core::IUnknown);
impl AppNotificationProgressData {
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
    pub fn SequenceNumber(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows::core::Interface::vtable(this).SequenceNumber)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
    pub fn SetSequenceNumber(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetSequenceNumber)(
                ::windows::core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
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
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
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
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
    pub fn Value(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows::core::Interface::vtable(this).Value)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
    pub fn SetValue(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetValue)(
                ::windows::core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
    pub fn ValueStringOverride(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<
                ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
            >::zeroed();
            (::windows::core::Interface::vtable(this).ValueStringOverride)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
    pub fn SetValueStringOverride<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetValueStringOverride)(
                ::windows::core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<
                ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
            >::zeroed();
            (::windows::core::Interface::vtable(this).Status)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
    pub fn SetStatus<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetStatus)(
                ::windows::core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
    pub fn CreateInstance(
        sequencenumber: u32,
    ) -> ::windows::core::Result<AppNotificationProgressData> {
        Self::IAppNotificationProgressDataFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(
                ::windows::core::Interface::as_raw(this),
                sequencenumber,
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationProgressData>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppNotificationProgressDataFactory<
        R,
        F: FnOnce(&IAppNotificationProgressDataFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            AppNotificationProgressData,
            IAppNotificationProgressDataFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AppNotificationProgressData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppNotificationProgressData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppNotificationProgressData {}
impl ::core::fmt::Debug for AppNotificationProgressData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppNotificationProgressData")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppNotificationProgressData {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.AppNotifications.AppNotificationProgressData;{4debfac0-809d-55cb-b879-924821876b97})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppNotificationProgressData {
    type Vtable = IAppNotificationProgressData_Vtbl;
    const IID: ::windows::core::GUID =
        <IAppNotificationProgressData as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppNotificationProgressData {
    const NAME: &'static str = "Microsoft.Windows.AppNotifications.AppNotificationProgressData";
}
impl ::core::convert::From<AppNotificationProgressData> for ::windows::core::IUnknown {
    fn from(value: AppNotificationProgressData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppNotificationProgressData> for ::windows::core::IUnknown {
    fn from(value: &AppNotificationProgressData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppNotificationProgressData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a AppNotificationProgressData
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppNotificationProgressData> for ::windows::core::IInspectable {
    fn from(value: AppNotificationProgressData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppNotificationProgressData> for ::windows::core::IInspectable {
    fn from(value: &AppNotificationProgressData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for AppNotificationProgressData
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a AppNotificationProgressData
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppNotificationProgressData {}
unsafe impl ::core::marker::Sync for AppNotificationProgressData {}
#[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppNotificationProgressResult(pub i32);
impl AppNotificationProgressResult {
    pub const Succeeded: Self = Self(0i32);
    pub const AppNotificationNotFound: Self = Self(1i32);
}
impl ::core::marker::Copy for AppNotificationProgressResult {}
impl ::core::clone::Clone for AppNotificationProgressResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppNotificationProgressResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppNotificationProgressResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppNotificationProgressResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppNotificationProgressResult")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppNotificationProgressResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.AppNotifications.AppNotificationProgressResult;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppNotificationSetting(pub i32);
impl AppNotificationSetting {
    pub const Enabled: Self = Self(0i32);
    pub const DisabledForApplication: Self = Self(1i32);
    pub const DisabledForUser: Self = Self(2i32);
    pub const DisabledByGroupPolicy: Self = Self(3i32);
    pub const DisabledByManifest: Self = Self(4i32);
}
impl ::core::marker::Copy for AppNotificationSetting {}
impl ::core::clone::Clone for AppNotificationSetting {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppNotificationSetting {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppNotificationSetting {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppNotificationSetting {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppNotificationSetting")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppNotificationSetting {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.AppNotifications.AppNotificationSetting;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppNotification(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppNotification {
    type Vtable = IAppNotification_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x373a6917_4116_5657_936a_15f99afdd667);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotification_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Tag: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetTag: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub Group: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetGroup: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub Id: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub Payload: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub Progress: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub SetProgress: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub Expiration: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::DateTime,
    ) -> ::windows::core::HRESULT,
    pub SetExpiration: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::DateTime,
    ) -> ::windows::core::HRESULT,
    pub ExpiresOnReboot: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetExpiresOnReboot: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub Priority: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut AppNotificationPriority,
    ) -> ::windows::core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: AppNotificationPriority,
    ) -> ::windows::core::HRESULT,
    pub SuppressDisplay: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetSuppressDisplay: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppNotificationActivatedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppNotificationActivatedEventArgs {
    type Vtable = IAppNotificationActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x7a8afaf9_31cb_51d5_82be_db6bd5878b77);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Argument: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub UserInput: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppNotificationFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppNotificationFactory {
    type Vtable = IAppNotificationFactory_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x9ffee485_184a_5c65_87a9_c1d94469dbe7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        payload: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppNotificationManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppNotificationManager {
    type Vtable = IAppNotificationManager_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x55129688_b4bd_550b_ae6b_c24061954d91);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationManager_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Register:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Unregister:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UnregisterAll:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NotificationInvoked: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveNotificationInvoked: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub Show: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        notification: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub UpdateAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        data: ::windows::core::RawPtr,
        tag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        group: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub UpdateAsync2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        data: ::windows::core::RawPtr,
        tag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub Setting: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut AppNotificationSetting,
    ) -> ::windows::core::HRESULT,
    pub RemoveByIdAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        notificationid: u32,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub RemoveByTagAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        tag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub RemoveByTagAndGroupAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        tag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        group: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub RemoveByGroupAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        group: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub RemoveAllAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetAllAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppNotificationManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppNotificationManagerStatics {
    type Vtable = IAppNotificationManagerStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x6cfc0d8d_84a3_5592_b4c6_e3e7e7c680e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Default: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppNotificationProgressData(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppNotificationProgressData {
    type Vtable = IAppNotificationProgressData_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4debfac0_809d_55cb_b879_924821876b97);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationProgressData_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub SequenceNumber: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub SetSequenceNumber: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: u32,
    ) -> ::windows::core::HRESULT,
    pub Title: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows::core::HRESULT,
    pub ValueStringOverride: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetValueStringOverride: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppNotificationProgressDataFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppNotificationProgressDataFactory {
    type Vtable = IAppNotificationProgressDataFactory_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc08e4a0f_3a75_55d6_8c3e_14f03ae46046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationProgressDataFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sequencenumber: u32,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
