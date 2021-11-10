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
pub struct AccessKeyDisplayDismissedEventArgs(pub ::windows::core::IInspectable);
impl AccessKeyDisplayDismissedEventArgs {
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
            AccessKeyDisplayDismissedEventArgs,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AccessKeyDisplayDismissedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Input.AccessKeyDisplayDismissedEventArgs;{125a83d8-7f86-5ea9-9063-b9407e644587})" ) ;
}
unsafe impl ::windows::core::Interface for AccessKeyDisplayDismissedEventArgs {
    type Vtable = IAccessKeyDisplayDismissedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x125a83d8_7f86_5ea9_9063_b9407e644587);
}
impl ::windows::core::RuntimeName for AccessKeyDisplayDismissedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.AccessKeyDisplayDismissedEventArgs";
}
impl ::core::convert::From<AccessKeyDisplayDismissedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AccessKeyDisplayDismissedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AccessKeyDisplayDismissedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AccessKeyDisplayDismissedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for AccessKeyDisplayDismissedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a AccessKeyDisplayDismissedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AccessKeyDisplayDismissedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AccessKeyDisplayDismissedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AccessKeyDisplayDismissedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AccessKeyDisplayDismissedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for AccessKeyDisplayDismissedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a AccessKeyDisplayDismissedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AccessKeyDisplayDismissedEventArgs {}
unsafe impl ::core::marker::Sync for AccessKeyDisplayDismissedEventArgs {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct AccessKeyDisplayRequestedEventArgs(pub ::windows::core::IInspectable);
impl AccessKeyDisplayRequestedEventArgs {
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
            AccessKeyDisplayRequestedEventArgs,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn PressedKeys(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
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
unsafe impl ::windows::core::RuntimeType for AccessKeyDisplayRequestedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Input.AccessKeyDisplayRequestedEventArgs;{c4ed84d8-2b27-59b1-9cf0-7f9164de58cb})" ) ;
}
unsafe impl ::windows::core::Interface for AccessKeyDisplayRequestedEventArgs {
    type Vtable = IAccessKeyDisplayRequestedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc4ed84d8_2b27_59b1_9cf0_7f9164de58cb);
}
impl ::windows::core::RuntimeName for AccessKeyDisplayRequestedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.AccessKeyDisplayRequestedEventArgs";
}
impl ::core::convert::From<AccessKeyDisplayRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AccessKeyDisplayRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AccessKeyDisplayRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AccessKeyDisplayRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for AccessKeyDisplayRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a AccessKeyDisplayRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AccessKeyDisplayRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AccessKeyDisplayRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AccessKeyDisplayRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AccessKeyDisplayRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for AccessKeyDisplayRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a AccessKeyDisplayRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AccessKeyDisplayRequestedEventArgs {}
unsafe impl ::core::marker::Sync for AccessKeyDisplayRequestedEventArgs {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct AccessKeyInvokedEventArgs(pub ::windows::core::IInspectable);
impl AccessKeyInvokedEventArgs {
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
            AccessKeyInvokedEventArgs,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
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
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AccessKeyInvokedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Input.AccessKeyInvokedEventArgs;{d00c11a4-f9fb-5707-9692-98b80bb8546d})" ) ;
}
unsafe impl ::windows::core::Interface for AccessKeyInvokedEventArgs {
    type Vtable = IAccessKeyInvokedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xd00c11a4_f9fb_5707_9692_98b80bb8546d);
}
impl ::windows::core::RuntimeName for AccessKeyInvokedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.AccessKeyInvokedEventArgs";
}
impl ::core::convert::From<AccessKeyInvokedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AccessKeyInvokedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AccessKeyInvokedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AccessKeyInvokedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AccessKeyInvokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a AccessKeyInvokedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AccessKeyInvokedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AccessKeyInvokedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AccessKeyInvokedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AccessKeyInvokedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for AccessKeyInvokedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a AccessKeyInvokedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AccessKeyInvokedEventArgs {}
unsafe impl ::core::marker::Sync for AccessKeyInvokedEventArgs {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct AccessKeyManager(pub ::windows::core::IInspectable);
impl AccessKeyManager {
    pub fn IsDisplayModeEnabled() -> ::windows::core::Result<bool> {
        Self::IAccessKeyManagerStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn AreKeyTipsEnabled() -> ::windows::core::Result<bool> {
        Self::IAccessKeyManagerStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetAreKeyTipsEnabled(value: bool) -> ::windows::core::Result<()> {
        Self::IAccessKeyManagerStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value)
                .ok()
        })
    }
    pub fn IsDisplayModeEnabledChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                ::windows::core::IInspectable,
                ::windows::core::IInspectable,
            >,
        >,
    >(
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        Self::IAccessKeyManagerStatics(|this| unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveIsDisplayModeEnabledChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::core::Result<()> {
        Self::IAccessKeyManagerStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    pub fn ExitDisplayMode() -> ::windows::core::Result<()> {
        Self::IAccessKeyManagerStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this)).ok()
        })
    }
    pub fn IAccessKeyManagerStatics<
        R,
        F: FnOnce(&IAccessKeyManagerStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            AccessKeyManager,
            IAccessKeyManagerStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AccessKeyManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Input.AccessKeyManager;{8f2a4402-a635-53dc-bc17-da911eabaade})",
    );
}
unsafe impl ::windows::core::Interface for AccessKeyManager {
    type Vtable = IAccessKeyManager_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x8f2a4402_a635_53dc_bc17_da911eabaade);
}
impl ::windows::core::RuntimeName for AccessKeyManager {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.AccessKeyManager";
}
impl ::core::convert::From<AccessKeyManager> for ::windows::core::IUnknown {
    fn from(value: AccessKeyManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AccessKeyManager> for ::windows::core::IUnknown {
    fn from(value: &AccessKeyManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AccessKeyManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AccessKeyManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AccessKeyManager> for ::windows::core::IInspectable {
    fn from(value: AccessKeyManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AccessKeyManager> for ::windows::core::IInspectable {
    fn from(value: &AccessKeyManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AccessKeyManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AccessKeyManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AccessKeyManager {}
unsafe impl ::core::marker::Sync for AccessKeyManager {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CanExecuteRequestedEventArgs(pub ::windows::core::IInspectable);
impl CanExecuteRequestedEventArgs {
    pub fn Parameter(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
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
    pub fn CanExecute(&self) -> ::windows::core::Result<bool> {
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
    pub fn SetCanExecute(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CanExecuteRequestedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Input.CanExecuteRequestedEventArgs;{e4bf6d7d-f6eb-53ca-a2d4-c741ec871e38})" ) ;
}
unsafe impl ::windows::core::Interface for CanExecuteRequestedEventArgs {
    type Vtable = ICanExecuteRequestedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xe4bf6d7d_f6eb_53ca_a2d4_c741ec871e38);
}
impl ::windows::core::RuntimeName for CanExecuteRequestedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.CanExecuteRequestedEventArgs";
}
impl ::core::convert::From<CanExecuteRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CanExecuteRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CanExecuteRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CanExecuteRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CanExecuteRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CanExecuteRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CanExecuteRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CanExecuteRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CanExecuteRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CanExecuteRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CanExecuteRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CanExecuteRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CanExecuteRequestedEventArgs {}
unsafe impl ::core::marker::Sync for CanExecuteRequestedEventArgs {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CharacterReceivedRoutedEventArgs(pub ::windows::core::IInspectable);
impl CharacterReceivedRoutedEventArgs {
    pub fn Character(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u16>(result__)
        }
    }
    pub fn KeyStatus(&self) -> ::windows::core::Result<::windows::UI::Core::CorePhysicalKeyStatus> {
        let this = self;
        unsafe {
            let mut result__: ::windows::UI::Core::CorePhysicalKeyStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CorePhysicalKeyStatus>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
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
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn OriginalSource(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CharacterReceivedRoutedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Input.CharacterReceivedRoutedEventArgs;{e26ca5bb-34c3-5c1e-9a16-00b80b07a899})" ) ;
}
unsafe impl ::windows::core::Interface for CharacterReceivedRoutedEventArgs {
    type Vtable = ICharacterReceivedRoutedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xe26ca5bb_34c3_5c1e_9a16_00b80b07a899);
}
impl ::windows::core::RuntimeName for CharacterReceivedRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.CharacterReceivedRoutedEventArgs";
}
impl ::core::convert::From<CharacterReceivedRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CharacterReceivedRoutedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CharacterReceivedRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CharacterReceivedRoutedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CharacterReceivedRoutedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CharacterReceivedRoutedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CharacterReceivedRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CharacterReceivedRoutedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CharacterReceivedRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CharacterReceivedRoutedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CharacterReceivedRoutedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CharacterReceivedRoutedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<CharacterReceivedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: CharacterReceivedRoutedEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&CharacterReceivedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &CharacterReceivedRoutedEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs>
    for CharacterReceivedRoutedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs>
    for &CharacterReceivedRoutedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for CharacterReceivedRoutedEventArgs {}
unsafe impl ::core::marker::Sync for CharacterReceivedRoutedEventArgs {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct ContextRequestedEventArgs(pub ::windows::core::IInspectable);
impl ContextRequestedEventArgs {
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
            ContextRequestedEventArgs,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
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
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn TryGetPosition<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(
        &self,
        relativeto: Param0,
        point: &mut ::windows::Foundation::Point,
    ) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                relativeto.into_param().abi(),
                point,
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn OriginalSource(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ContextRequestedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Input.ContextRequestedEventArgs;{bcedcb98-77b5-53c0-802e-fd52f3806e51})" ) ;
}
unsafe impl ::windows::core::Interface for ContextRequestedEventArgs {
    type Vtable = IContextRequestedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xbcedcb98_77b5_53c0_802e_fd52f3806e51);
}
impl ::windows::core::RuntimeName for ContextRequestedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.ContextRequestedEventArgs";
}
impl ::core::convert::From<ContextRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ContextRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ContextRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ContextRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContextRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a ContextRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ContextRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ContextRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ContextRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ContextRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for ContextRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a ContextRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ContextRequestedEventArgs> for super::RoutedEventArgs {
    fn from(value: ContextRequestedEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&ContextRequestedEventArgs> for super::RoutedEventArgs {
    fn from(value: &ContextRequestedEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for ContextRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &ContextRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for ContextRequestedEventArgs {}
unsafe impl ::core::marker::Sync for ContextRequestedEventArgs {}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct DoubleTappedEventHandler(::windows::core::IUnknown);
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
impl DoubleTappedEventHandler {
    pub fn new<
        F: FnMut(
                &::core::option::Option<::windows::core::IInspectable>,
                &::core::option::Option<DoubleTappedRoutedEventArgs>,
            ) -> ::windows::core::Result<()>
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = DoubleTappedEventHandler_box::<F> {
            vtable: &DoubleTappedEventHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn Invoke<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param1: ::windows::core::IntoParam<'a, DoubleTappedRoutedEventArgs>,
    >(
        &self,
        sender: Param0,
        e: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).3)(
                ::core::mem::transmute_copy(this),
                sender.into_param().abi(),
                e.into_param().abi(),
            )
            .ok()
        }
    }
}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
unsafe impl ::windows::core::RuntimeType for DoubleTappedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"delegate({f7a501b9-e277-5611-87b0-0e0607622183})",
    );
}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
unsafe impl ::windows::core::Interface for DoubleTappedEventHandler {
    type Vtable = DoubleTappedEventHandler_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xf7a501b9_e277_5611_87b0_0e0607622183);
}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
#[repr(C)]
#[doc(hidden)]
pub struct DoubleTappedEventHandler_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        sender: ::windows::core::RawPtr,
        e: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
#[repr(C)]
struct DoubleTappedEventHandler_box<
    F: FnMut(
            &::core::option::Option<::windows::core::IInspectable>,
            &::core::option::Option<DoubleTappedRoutedEventArgs>,
        ) -> ::windows::core::Result<()>
        + 'static,
> {
    vtable: *const DoubleTappedEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
impl<
        F: FnMut(
                &::core::option::Option<::windows::core::IInspectable>,
                &::core::option::Option<DoubleTappedRoutedEventArgs>,
            ) -> ::windows::core::Result<()>
            + 'static,
    > DoubleTappedEventHandler_box<F>
{
    const VTABLE: DoubleTappedEventHandler_abi = DoubleTappedEventHandler_abi(
        Self::QueryInterface,
        Self::AddRef,
        Self::Release,
        Self::Invoke,
    );
    unsafe extern "system" fn QueryInterface(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<DoubleTappedEventHandler as ::windows::core::Interface>::IID
            || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID
            || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::core::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: ::windows::core::RawPtr,
        sender: ::windows::core::RawPtr,
        e: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ( ( * this ) . invoke ) ( & * ( & sender as * const < :: windows :: core :: IInspectable as :: windows :: core :: Abi > :: Abi as * const < :: windows :: core :: IInspectable as :: windows :: core :: DefaultType > :: DefaultType ) , & * ( & e as * const < DoubleTappedRoutedEventArgs as :: windows :: core :: Abi > :: Abi as * const < DoubleTappedRoutedEventArgs as :: windows :: core :: DefaultType > :: DefaultType ) , ) . into ( )
    }
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct DoubleTappedRoutedEventArgs(pub ::windows::core::IInspectable);
impl DoubleTappedRoutedEventArgs {
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
            DoubleTappedRoutedEventArgs,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "UI_Input")]
    pub fn PointerDeviceType(
        &self,
    ) -> ::windows::core::Result<super::super::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Input::PointerDeviceType>(result__)
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
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn GetPosition<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(
        &self,
        relativeto: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                relativeto.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Point>(result__)
        }
    }
    pub fn OriginalSource(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DoubleTappedRoutedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Input.DoubleTappedRoutedEventArgs;{32b9549d-11d8-53a5-a953-02409537a11f})" ) ;
}
unsafe impl ::windows::core::Interface for DoubleTappedRoutedEventArgs {
    type Vtable = IDoubleTappedRoutedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x32b9549d_11d8_53a5_a953_02409537a11f);
}
impl ::windows::core::RuntimeName for DoubleTappedRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.DoubleTappedRoutedEventArgs";
}
impl ::core::convert::From<DoubleTappedRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: DoubleTappedRoutedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DoubleTappedRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DoubleTappedRoutedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DoubleTappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a DoubleTappedRoutedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DoubleTappedRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: DoubleTappedRoutedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DoubleTappedRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DoubleTappedRoutedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for DoubleTappedRoutedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a DoubleTappedRoutedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<DoubleTappedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: DoubleTappedRoutedEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&DoubleTappedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &DoubleTappedRoutedEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for DoubleTappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &DoubleTappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for DoubleTappedRoutedEventArgs {}
unsafe impl ::core::marker::Sync for DoubleTappedRoutedEventArgs {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct ExecuteRequestedEventArgs(pub ::windows::core::IInspectable);
impl ExecuteRequestedEventArgs {
    pub fn Parameter(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
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
}
unsafe impl ::windows::core::RuntimeType for ExecuteRequestedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Input.ExecuteRequestedEventArgs;{e1a9fd0c-34d0-5ae2-8f5d-377e7a8a2708})" ) ;
}
unsafe impl ::windows::core::Interface for ExecuteRequestedEventArgs {
    type Vtable = IExecuteRequestedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xe1a9fd0c_34d0_5ae2_8f5d_377e7a8a2708);
}
impl ::windows::core::RuntimeName for ExecuteRequestedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.ExecuteRequestedEventArgs";
}
impl ::core::convert::From<ExecuteRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ExecuteRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ExecuteRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ExecuteRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ExecuteRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a ExecuteRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ExecuteRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ExecuteRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ExecuteRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ExecuteRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for ExecuteRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a ExecuteRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ExecuteRequestedEventArgs {}
unsafe impl ::core::marker::Sync for ExecuteRequestedEventArgs {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct FindNextElementOptions(pub ::windows::core::IInspectable);
impl FindNextElementOptions {
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
            FindNextElementOptions,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SearchRoot(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetSearchRoot<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
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
    pub fn ExclusionRect(&self) -> ::windows::core::Result<::windows::Foundation::Rect> {
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
    pub fn SetExclusionRect<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Rect>,
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
    pub fn HintRect(&self) -> ::windows::core::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Rect>(result__)
        }
    }
    pub fn SetHintRect<'a, Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Rect>>(
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
    pub fn XYFocusNavigationStrategyOverride(
        &self,
    ) -> ::windows::core::Result<XYFocusNavigationStrategyOverride> {
        let this = self;
        unsafe {
            let mut result__: XYFocusNavigationStrategyOverride = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<XYFocusNavigationStrategyOverride>(result__)
        }
    }
    pub fn SetXYFocusNavigationStrategyOverride(
        &self,
        value: XYFocusNavigationStrategyOverride,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for FindNextElementOptions {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Input.FindNextElementOptions;{7f88e76b-7417-5447-aed4-2fabd291bdc6})" ) ;
}
unsafe impl ::windows::core::Interface for FindNextElementOptions {
    type Vtable = IFindNextElementOptions_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x7f88e76b_7417_5447_aed4_2fabd291bdc6);
}
impl ::windows::core::RuntimeName for FindNextElementOptions {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.FindNextElementOptions";
}
impl ::core::convert::From<FindNextElementOptions> for ::windows::core::IUnknown {
    fn from(value: FindNextElementOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FindNextElementOptions> for ::windows::core::IUnknown {
    fn from(value: &FindNextElementOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FindNextElementOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FindNextElementOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FindNextElementOptions> for ::windows::core::IInspectable {
    fn from(value: FindNextElementOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FindNextElementOptions> for ::windows::core::IInspectable {
    fn from(value: &FindNextElementOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FindNextElementOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a FindNextElementOptions
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for FindNextElementOptions {}
unsafe impl ::core::marker::Sync for FindNextElementOptions {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct FocusInputDeviceKind(pub i32);
impl FocusInputDeviceKind {
    pub const None: FocusInputDeviceKind = FocusInputDeviceKind(0i32);
    pub const Mouse: FocusInputDeviceKind = FocusInputDeviceKind(1i32);
    pub const Touch: FocusInputDeviceKind = FocusInputDeviceKind(2i32);
    pub const Pen: FocusInputDeviceKind = FocusInputDeviceKind(3i32);
    pub const Keyboard: FocusInputDeviceKind = FocusInputDeviceKind(4i32);
    pub const GameController: FocusInputDeviceKind = FocusInputDeviceKind(5i32);
}
impl ::core::convert::From<i32> for FocusInputDeviceKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for FocusInputDeviceKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for FocusInputDeviceKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Input.FocusInputDeviceKind;i4)",
    );
}
impl ::windows::core::DefaultType for FocusInputDeviceKind {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct FocusManager(pub ::windows::core::IInspectable);
impl FocusManager {
    #[cfg(feature = "UI_Dispatching")]
    pub fn GotFocus<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::EventHandler<FocusManagerGotFocusEventArgs>,
        >,
    >(
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveGotFocus<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::core::Result<()> {
        Self::IFocusManagerStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn LostFocus<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::EventHandler<FocusManagerLostFocusEventArgs>,
        >,
    >(
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveLostFocus<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::core::Result<()> {
        Self::IFocusManagerStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GettingFocus<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventHandler<GettingFocusEventArgs>>,
    >(
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveGettingFocus<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::core::Result<()> {
        Self::IFocusManagerStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn LosingFocus<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventHandler<LosingFocusEventArgs>>,
    >(
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveLosingFocus<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::core::Result<()> {
        Self::IFocusManagerStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn TryFocusAsync<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: super::FocusState,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncOperation<FocusMovementResult>> {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IAsyncOperation<FocusMovementResult>>(result__)
        })
    }
    pub fn TryMoveFocusAsync(
        focusnavigationdirection: FocusNavigationDirection,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncOperation<FocusMovementResult>> {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                focusnavigationdirection,
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IAsyncOperation<FocusMovementResult>>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn TryMoveFocusWithOptionsAsync<
        'a,
        Param1: ::windows::core::IntoParam<'a, FindNextElementOptions>,
    >(
        focusnavigationdirection: FocusNavigationDirection,
        focusnavigationoptions: Param1,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncOperation<FocusMovementResult>> {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                focusnavigationdirection,
                focusnavigationoptions.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IAsyncOperation<FocusMovementResult>>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn TryMoveFocusWithOptions<
        'a,
        Param1: ::windows::core::IntoParam<'a, FindNextElementOptions>,
    >(
        focusnavigationdirection: FocusNavigationDirection,
        focusnavigationoptions: Param1,
    ) -> ::windows::core::Result<bool> {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                focusnavigationdirection,
                focusnavigationoptions.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn FindNextElement(
        focusnavigationdirection: FocusNavigationDirection,
    ) -> ::windows::core::Result<super::DependencyObject> {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                focusnavigationdirection,
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn FindFirstFocusableElement<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        searchscope: Param0,
    ) -> ::windows::core::Result<super::DependencyObject> {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                searchscope.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn FindLastFocusableElement<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        searchscope: Param0,
    ) -> ::windows::core::Result<super::DependencyObject> {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                searchscope.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn FindNextElementWithOptions<
        'a,
        Param1: ::windows::core::IntoParam<'a, FindNextElementOptions>,
    >(
        focusnavigationdirection: FocusNavigationDirection,
        focusnavigationoptions: Param1,
    ) -> ::windows::core::Result<super::DependencyObject> {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                focusnavigationdirection,
                focusnavigationoptions.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        })
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn FindNextFocusableElement(
        focusnavigationdirection: FocusNavigationDirection,
    ) -> ::windows::core::Result<super::UIElement> {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                focusnavigationdirection,
                &mut result__,
            )
            .from_abi::<super::UIElement>(result__)
        })
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn FindNextFocusableElementWithHint<
        'a,
        Param1: ::windows::core::IntoParam<'a, ::windows::Foundation::Rect>,
    >(
        focusnavigationdirection: FocusNavigationDirection,
        hintrect: Param1,
    ) -> ::windows::core::Result<super::UIElement> {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                focusnavigationdirection,
                hintrect.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::UIElement>(result__)
        })
    }
    pub fn TryMoveFocus(
        focusnavigationdirection: FocusNavigationDirection,
    ) -> ::windows::core::Result<bool> {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(
                ::core::mem::transmute_copy(this),
                focusnavigationdirection,
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn GetFocusedElement() -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn GetFocusedElementWithRoot<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::XamlRoot>,
    >(
        xamlroot: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(
                ::core::mem::transmute_copy(this),
                xamlroot.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn IFocusManagerStatics<
        R,
        F: FnOnce(&IFocusManagerStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FocusManager, IFocusManagerStatics> =
            ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for FocusManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Input.FocusManager;{9fd07bc5-d2d4-53fe-a31a-846de8b7a257})",
    );
}
unsafe impl ::windows::core::Interface for FocusManager {
    type Vtable = IFocusManager_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x9fd07bc5_d2d4_53fe_a31a_846de8b7a257);
}
impl ::windows::core::RuntimeName for FocusManager {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.FocusManager";
}
impl ::core::convert::From<FocusManager> for ::windows::core::IUnknown {
    fn from(value: FocusManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FocusManager> for ::windows::core::IUnknown {
    fn from(value: &FocusManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FocusManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FocusManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FocusManager> for ::windows::core::IInspectable {
    fn from(value: FocusManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FocusManager> for ::windows::core::IInspectable {
    fn from(value: &FocusManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FocusManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FocusManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for FocusManager {}
unsafe impl ::core::marker::Sync for FocusManager {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct FocusManagerGotFocusEventArgs(pub ::windows::core::IInspectable);
impl FocusManagerGotFocusEventArgs {
    #[cfg(feature = "UI_Dispatching")]
    pub fn NewFocusedElement(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn CorrelationId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::GUID>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for FocusManagerGotFocusEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Input.FocusManagerGotFocusEventArgs;{50aca341-4519-59cf-83b1-c9c45cfdb816})" ) ;
}
unsafe impl ::windows::core::Interface for FocusManagerGotFocusEventArgs {
    type Vtable = IFocusManagerGotFocusEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x50aca341_4519_59cf_83b1_c9c45cfdb816);
}
impl ::windows::core::RuntimeName for FocusManagerGotFocusEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.FocusManagerGotFocusEventArgs";
}
impl ::core::convert::From<FocusManagerGotFocusEventArgs> for ::windows::core::IUnknown {
    fn from(value: FocusManagerGotFocusEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FocusManagerGotFocusEventArgs> for ::windows::core::IUnknown {
    fn from(value: &FocusManagerGotFocusEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for FocusManagerGotFocusEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a FocusManagerGotFocusEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FocusManagerGotFocusEventArgs> for ::windows::core::IInspectable {
    fn from(value: FocusManagerGotFocusEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FocusManagerGotFocusEventArgs> for ::windows::core::IInspectable {
    fn from(value: &FocusManagerGotFocusEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for FocusManagerGotFocusEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a FocusManagerGotFocusEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for FocusManagerGotFocusEventArgs {}
unsafe impl ::core::marker::Sync for FocusManagerGotFocusEventArgs {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct FocusManagerLostFocusEventArgs(pub ::windows::core::IInspectable);
impl FocusManagerLostFocusEventArgs {
    #[cfg(feature = "UI_Dispatching")]
    pub fn OldFocusedElement(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn CorrelationId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::GUID>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for FocusManagerLostFocusEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Input.FocusManagerLostFocusEventArgs;{fdaf2c3f-a22e-5902-abce-b60758fbed1e})" ) ;
}
unsafe impl ::windows::core::Interface for FocusManagerLostFocusEventArgs {
    type Vtable = IFocusManagerLostFocusEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xfdaf2c3f_a22e_5902_abce_b60758fbed1e);
}
impl ::windows::core::RuntimeName for FocusManagerLostFocusEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.FocusManagerLostFocusEventArgs";
}
impl ::core::convert::From<FocusManagerLostFocusEventArgs> for ::windows::core::IUnknown {
    fn from(value: FocusManagerLostFocusEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FocusManagerLostFocusEventArgs> for ::windows::core::IUnknown {
    fn from(value: &FocusManagerLostFocusEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for FocusManagerLostFocusEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a FocusManagerLostFocusEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FocusManagerLostFocusEventArgs> for ::windows::core::IInspectable {
    fn from(value: FocusManagerLostFocusEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FocusManagerLostFocusEventArgs> for ::windows::core::IInspectable {
    fn from(value: &FocusManagerLostFocusEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for FocusManagerLostFocusEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a FocusManagerLostFocusEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for FocusManagerLostFocusEventArgs {}
unsafe impl ::core::marker::Sync for FocusManagerLostFocusEventArgs {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct FocusMovementResult(pub ::windows::core::IInspectable);
impl FocusMovementResult {
    pub fn Succeeded(&self) -> ::windows::core::Result<bool> {
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
unsafe impl ::windows::core::RuntimeType for FocusMovementResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Input.FocusMovementResult;{a46259fd-3edd-554b-a188-0a47b71e4e1a})",
    );
}
unsafe impl ::windows::core::Interface for FocusMovementResult {
    type Vtable = IFocusMovementResult_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa46259fd_3edd_554b_a188_0a47b71e4e1a);
}
impl ::windows::core::RuntimeName for FocusMovementResult {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.FocusMovementResult";
}
impl ::core::convert::From<FocusMovementResult> for ::windows::core::IUnknown {
    fn from(value: FocusMovementResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FocusMovementResult> for ::windows::core::IUnknown {
    fn from(value: &FocusMovementResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FocusMovementResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FocusMovementResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FocusMovementResult> for ::windows::core::IInspectable {
    fn from(value: FocusMovementResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FocusMovementResult> for ::windows::core::IInspectable {
    fn from(value: &FocusMovementResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FocusMovementResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FocusMovementResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for FocusMovementResult {}
unsafe impl ::core::marker::Sync for FocusMovementResult {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct FocusNavigationDirection(pub i32);
impl FocusNavigationDirection {
    pub const Next: FocusNavigationDirection = FocusNavigationDirection(0i32);
    pub const Previous: FocusNavigationDirection = FocusNavigationDirection(1i32);
    pub const Up: FocusNavigationDirection = FocusNavigationDirection(2i32);
    pub const Down: FocusNavigationDirection = FocusNavigationDirection(3i32);
    pub const Left: FocusNavigationDirection = FocusNavigationDirection(4i32);
    pub const Right: FocusNavigationDirection = FocusNavigationDirection(5i32);
    pub const None: FocusNavigationDirection = FocusNavigationDirection(6i32);
}
impl ::core::convert::From<i32> for FocusNavigationDirection {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for FocusNavigationDirection {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for FocusNavigationDirection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Input.FocusNavigationDirection;i4)",
    );
}
impl ::windows::core::DefaultType for FocusNavigationDirection {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct GettingFocusEventArgs(pub ::windows::core::IInspectable);
impl GettingFocusEventArgs {
    #[cfg(feature = "UI_Dispatching")]
    pub fn OldFocusedElement(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn NewFocusedElement(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetNewFocusedElement<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
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
    pub fn FocusState(&self) -> ::windows::core::Result<super::FocusState> {
        let this = self;
        unsafe {
            let mut result__: super::FocusState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::FocusState>(result__)
        }
    }
    pub fn Direction(&self) -> ::windows::core::Result<FocusNavigationDirection> {
        let this = self;
        unsafe {
            let mut result__: FocusNavigationDirection = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<FocusNavigationDirection>(result__)
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
    pub fn InputDevice(&self) -> ::windows::core::Result<FocusInputDeviceKind> {
        let this = self;
        unsafe {
            let mut result__: FocusInputDeviceKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<FocusInputDeviceKind>(result__)
        }
    }
    pub fn Cancel(&self) -> ::windows::core::Result<bool> {
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
    pub fn SetCancel(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn CorrelationId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn TryCancel(&self) -> ::windows::core::Result<bool> {
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
    #[cfg(feature = "UI_Dispatching")]
    pub fn TrySetNewFocusedElement<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn OriginalSource(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GettingFocusEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Input.GettingFocusEventArgs;{37fd3af0-bd3c-5bf5-a9cd-71a1e87af950})",
    );
}
unsafe impl ::windows::core::Interface for GettingFocusEventArgs {
    type Vtable = IGettingFocusEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x37fd3af0_bd3c_5bf5_a9cd_71a1e87af950);
}
impl ::windows::core::RuntimeName for GettingFocusEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.GettingFocusEventArgs";
}
impl ::core::convert::From<GettingFocusEventArgs> for ::windows::core::IUnknown {
    fn from(value: GettingFocusEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GettingFocusEventArgs> for ::windows::core::IUnknown {
    fn from(value: &GettingFocusEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GettingFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GettingFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GettingFocusEventArgs> for ::windows::core::IInspectable {
    fn from(value: GettingFocusEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GettingFocusEventArgs> for ::windows::core::IInspectable {
    fn from(value: &GettingFocusEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GettingFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a GettingFocusEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<GettingFocusEventArgs> for super::RoutedEventArgs {
    fn from(value: GettingFocusEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&GettingFocusEventArgs> for super::RoutedEventArgs {
    fn from(value: &GettingFocusEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for GettingFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &GettingFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for GettingFocusEventArgs {}
unsafe impl ::core::marker::Sync for GettingFocusEventArgs {}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct HoldingEventHandler(::windows::core::IUnknown);
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
impl HoldingEventHandler {
    pub fn new<
        F: FnMut(
                &::core::option::Option<::windows::core::IInspectable>,
                &::core::option::Option<HoldingRoutedEventArgs>,
            ) -> ::windows::core::Result<()>
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = HoldingEventHandler_box::<F> {
            vtable: &HoldingEventHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn Invoke<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param1: ::windows::core::IntoParam<'a, HoldingRoutedEventArgs>,
    >(
        &self,
        sender: Param0,
        e: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).3)(
                ::core::mem::transmute_copy(this),
                sender.into_param().abi(),
                e.into_param().abi(),
            )
            .ok()
        }
    }
}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
unsafe impl ::windows::core::RuntimeType for HoldingEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"delegate({fe23c5bd-4984-56b6-b92b-fc9d1216b24e})",
    );
}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
unsafe impl ::windows::core::Interface for HoldingEventHandler {
    type Vtable = HoldingEventHandler_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xfe23c5bd_4984_56b6_b92b_fc9d1216b24e);
}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
#[repr(C)]
#[doc(hidden)]
pub struct HoldingEventHandler_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        sender: ::windows::core::RawPtr,
        e: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
#[repr(C)]
struct HoldingEventHandler_box<
    F: FnMut(
            &::core::option::Option<::windows::core::IInspectable>,
            &::core::option::Option<HoldingRoutedEventArgs>,
        ) -> ::windows::core::Result<()>
        + 'static,
> {
    vtable: *const HoldingEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
impl<
        F: FnMut(
                &::core::option::Option<::windows::core::IInspectable>,
                &::core::option::Option<HoldingRoutedEventArgs>,
            ) -> ::windows::core::Result<()>
            + 'static,
    > HoldingEventHandler_box<F>
{
    const VTABLE: HoldingEventHandler_abi = HoldingEventHandler_abi(
        Self::QueryInterface,
        Self::AddRef,
        Self::Release,
        Self::Invoke,
    );
    unsafe extern "system" fn QueryInterface(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<HoldingEventHandler as ::windows::core::Interface>::IID
            || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID
            || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::core::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: ::windows::core::RawPtr,
        sender: ::windows::core::RawPtr,
        e: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ( ( * this ) . invoke ) ( & * ( & sender as * const < :: windows :: core :: IInspectable as :: windows :: core :: Abi > :: Abi as * const < :: windows :: core :: IInspectable as :: windows :: core :: DefaultType > :: DefaultType ) , & * ( & e as * const < HoldingRoutedEventArgs as :: windows :: core :: Abi > :: Abi as * const < HoldingRoutedEventArgs as :: windows :: core :: DefaultType > :: DefaultType ) , ) . into ( )
    }
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct HoldingRoutedEventArgs(pub ::windows::core::IInspectable);
impl HoldingRoutedEventArgs {
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
            HoldingRoutedEventArgs,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "UI_Input")]
    pub fn PointerDeviceType(
        &self,
    ) -> ::windows::core::Result<super::super::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Input::PointerDeviceType>(result__)
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn HoldingState(&self) -> ::windows::core::Result<super::super::Input::HoldingState> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::HoldingState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Input::HoldingState>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
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
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn GetPosition<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(
        &self,
        relativeto: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                relativeto.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Point>(result__)
        }
    }
    pub fn OriginalSource(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for HoldingRoutedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Input.HoldingRoutedEventArgs;{8272a4b2-2221-551e-b0bb-16e29138ab20})" ) ;
}
unsafe impl ::windows::core::Interface for HoldingRoutedEventArgs {
    type Vtable = IHoldingRoutedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x8272a4b2_2221_551e_b0bb_16e29138ab20);
}
impl ::windows::core::RuntimeName for HoldingRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.HoldingRoutedEventArgs";
}
impl ::core::convert::From<HoldingRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: HoldingRoutedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HoldingRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &HoldingRoutedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HoldingRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HoldingRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HoldingRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: HoldingRoutedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HoldingRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &HoldingRoutedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HoldingRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a HoldingRoutedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<HoldingRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: HoldingRoutedEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&HoldingRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &HoldingRoutedEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for HoldingRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &HoldingRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for HoldingRoutedEventArgs {}
unsafe impl ::core::marker::Sync for HoldingRoutedEventArgs {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAccessKeyDisplayDismissedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAccessKeyDisplayDismissedEventArgs {
    type Vtable = IAccessKeyDisplayDismissedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x125a83d8_7f86_5ea9_9063_b9407e644587);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessKeyDisplayDismissedEventArgs_abi(
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
pub struct IAccessKeyDisplayRequestedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAccessKeyDisplayRequestedEventArgs {
    type Vtable = IAccessKeyDisplayRequestedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc4ed84d8_2b27_59b1_9cf0_7f9164de58cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessKeyDisplayRequestedEventArgs_abi(
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
pub struct IAccessKeyInvokedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAccessKeyInvokedEventArgs {
    type Vtable = IAccessKeyInvokedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xd00c11a4_f9fb_5707_9692_98b80bb8546d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessKeyInvokedEventArgs_abi(
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
pub struct IAccessKeyManager(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAccessKeyManager {
    type Vtable = IAccessKeyManager_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x8f2a4402_a635_53dc_bc17_da911eabaade);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessKeyManager_abi(
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
pub struct IAccessKeyManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAccessKeyManagerStatics {
    type Vtable = IAccessKeyManagerStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x3375aef7_742f_5e84_b76f_c187e08253bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessKeyManagerStatics_abi(
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
        value: bool,
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICanExecuteRequestedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICanExecuteRequestedEventArgs {
    type Vtable = ICanExecuteRequestedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xe4bf6d7d_f6eb_53ca_a2d4_c741ec871e38);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanExecuteRequestedEventArgs_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICharacterReceivedRoutedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICharacterReceivedRoutedEventArgs {
    type Vtable = ICharacterReceivedRoutedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xe26ca5bb_34c3_5c1e_9a16_00b80b07a899);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICharacterReceivedRoutedEventArgs_abi(
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
        result__: *mut u16,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::UI::Core::CorePhysicalKeyStatus,
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
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct ICommand(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICommand {
    type Vtable = ICommand_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xe5af3542_ca67_4081_995b_709dd13792df);
}
impl ICommand {
    pub fn CanExecuteChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::EventHandler<::windows::core::IInspectable>,
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
    pub fn RemoveCanExecuteChanged<
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
    pub fn CanExecute<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(
        &self,
        parameter: Param0,
    ) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                parameter.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn Execute<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(
        &self,
        parameter: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                parameter.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ICommand {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{e5af3542-ca67-4081-995b-709dd13792df}");
}
impl ::core::convert::From<ICommand> for ::windows::core::IUnknown {
    fn from(value: ICommand) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ICommand> for ::windows::core::IUnknown {
    fn from(value: &ICommand) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ICommand> for ::windows::core::IInspectable {
    fn from(value: ICommand) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICommand> for ::windows::core::IInspectable {
    fn from(value: &ICommand) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommand_abi(
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
        parameter: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        parameter: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContextRequestedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IContextRequestedEventArgs {
    type Vtable = IContextRequestedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xbcedcb98_77b5_53c0_802e_fd52f3806e51);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContextRequestedEventArgs_abi(
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
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        relativeto: ::windows::core::RawPtr,
        point: *mut ::windows::Foundation::Point,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    )))]
    usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDoubleTappedRoutedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDoubleTappedRoutedEventArgs {
    type Vtable = IDoubleTappedRoutedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x32b9549d_11d8_53a5_a953_02409537a11f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDoubleTappedRoutedEventArgs_abi(
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
    #[cfg(feature = "UI_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut super::super::Input::PointerDeviceType,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Input"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        relativeto: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    )))]
    usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IExecuteRequestedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IExecuteRequestedEventArgs {
    type Vtable = IExecuteRequestedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xe1a9fd0c_34d0_5ae2_8f5d_377e7a8a2708);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExecuteRequestedEventArgs_abi(
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
pub struct IFindNextElementOptions(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFindNextElementOptions {
    type Vtable = IFindNextElementOptions_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x7f88e76b_7417_5447_aed4_2fabd291bdc6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFindNextElementOptions_abi(
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
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
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
        result__: *mut XYFocusNavigationStrategyOverride,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: XYFocusNavigationStrategyOverride,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFocusManager(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFocusManager {
    type Vtable = IFocusManager_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x9fd07bc5_d2d4_53fe_a31a_846de8b7a257);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManager_abi(
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
pub struct IFocusManagerGotFocusEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFocusManagerGotFocusEventArgs {
    type Vtable = IFocusManagerGotFocusEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x50aca341_4519_59cf_83b1_c9c45cfdb816);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManagerGotFocusEventArgs_abi(
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
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFocusManagerLostFocusEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFocusManagerLostFocusEventArgs {
    type Vtable = IFocusManagerLostFocusEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xfdaf2c3f_a22e_5902_abce_b60758fbed1e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManagerLostFocusEventArgs_abi(
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
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFocusManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFocusManagerStatics {
    type Vtable = IFocusManagerStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xe73dce04_e23a_5fb3_96ab_7df04c51dff2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManagerStatics_abi(
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
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: super::FocusState,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        focusnavigationdirection: FocusNavigationDirection,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        focusnavigationdirection: FocusNavigationDirection,
        focusnavigationoptions: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        focusnavigationdirection: FocusNavigationDirection,
        focusnavigationoptions: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        focusnavigationdirection: FocusNavigationDirection,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        searchscope: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        searchscope: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        focusnavigationdirection: FocusNavigationDirection,
        focusnavigationoptions: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        focusnavigationdirection: FocusNavigationDirection,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    )))]
    usize,
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        focusnavigationdirection: FocusNavigationDirection,
        hintrect: ::windows::Foundation::Rect,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        focusnavigationdirection: FocusNavigationDirection,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        xamlroot: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    )))]
    usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFocusMovementResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFocusMovementResult {
    type Vtable = IFocusMovementResult_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa46259fd_3edd_554b_a188_0a47b71e4e1a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusMovementResult_abi(
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
pub struct IGettingFocusEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGettingFocusEventArgs {
    type Vtable = IGettingFocusEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x37fd3af0_bd3c_5bf5_a9cd_71a1e87af950);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGettingFocusEventArgs_abi(
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
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut super::FocusState,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut FocusNavigationDirection,
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
        result__: *mut FocusInputDeviceKind,
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
        result__: *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHoldingRoutedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IHoldingRoutedEventArgs {
    type Vtable = IHoldingRoutedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x8272a4b2_2221_551e_b0bb_16e29138ab20);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHoldingRoutedEventArgs_abi(
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
    #[cfg(feature = "UI_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut super::super::Input::PointerDeviceType,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Input"))] usize,
    #[cfg(feature = "UI_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut super::super::Input::HoldingState,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Input"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        relativeto: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    )))]
    usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInertiaExpansionBehavior(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInertiaExpansionBehavior {
    type Vtable = IInertiaExpansionBehavior_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xd60029b7_f0cd_5aea_abe5_7410d09118c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInertiaExpansionBehavior_abi(
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
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: f64,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInertiaRotationBehavior(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInertiaRotationBehavior {
    type Vtable = IInertiaRotationBehavior_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x27b4bd03_9149_5691_bce5_fa33b32c4a81);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInertiaRotationBehavior_abi(
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
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: f64,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInertiaTranslationBehavior(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInertiaTranslationBehavior {
    type Vtable = IInertiaTranslationBehavior_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xd4f91cf5_3317_5914_b25a_ea6ee55b96d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInertiaTranslationBehavior_abi(
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
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: f64,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInputScope(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInputScope {
    type Vtable = IInputScope_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x76ea58b1_e910_5176_9147_695cc95e7da2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputScope_abi(
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
pub struct IInputScopeName(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInputScopeName {
    type Vtable = IInputScopeName_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xee99a66d_28d0_53cb_82ee_1b6ee58bcc35);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputScopeName_abi(
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
        result__: *mut InputScopeNameValue,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: InputScopeNameValue,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInputScopeNameFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInputScopeNameFactory {
    type Vtable = IInputScopeNameFactory_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xfeec2efd_bc09_5cd6_9b47_6d35d1d87c61);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputScopeNameFactory_abi(
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
        namevalue: InputScopeNameValue,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKeyRoutedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IKeyRoutedEventArgs {
    type Vtable = IKeyRoutedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xee357007_a2d6_5c75_9431_05fd66ec7915);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyRoutedEventArgs_abi(
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
        result__: *mut ::windows::System::VirtualKey,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::UI::Core::CorePhysicalKeyStatus,
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
        result__: *mut ::windows::System::VirtualKey,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKeyboardAccelerator(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IKeyboardAccelerator {
    type Vtable = IKeyboardAccelerator_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x6f8bf1e2_4e91_5cf9_a6be_4770caf3d770);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyboardAccelerator_abi(
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
        result__: *mut ::windows::System::VirtualKey,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::System::VirtualKey,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::System::VirtualKeyModifiers,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::System::VirtualKeyModifiers,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKeyboardAcceleratorFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IKeyboardAcceleratorFactory {
    type Vtable = IKeyboardAcceleratorFactory_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xca1d410a_af2a_51b9_a1de_6c0af9f3b598);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyboardAcceleratorFactory_abi(
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
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        baseinterface: ::windows::core::RawPtr,
        innerinterface: *mut ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKeyboardAcceleratorInvokedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IKeyboardAcceleratorInvokedEventArgs {
    type Vtable = IKeyboardAcceleratorInvokedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x62c9fdb0_b574_527d_97eb_5c7f674441e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyboardAcceleratorInvokedEventArgs_abi(
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
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKeyboardAcceleratorStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IKeyboardAcceleratorStatics {
    type Vtable = IKeyboardAcceleratorStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x73e674ca_73f4_5e77_b8d6_ff7852a63b0b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyboardAcceleratorStatics_abi(
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
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILosingFocusEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILosingFocusEventArgs {
    type Vtable = ILosingFocusEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xfa0e5ffa_2b1b_52f8_bb66_e35f51e73cf3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILosingFocusEventArgs_abi(
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
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut super::FocusState,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut FocusNavigationDirection,
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
        result__: *mut FocusInputDeviceKind,
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
        result__: *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IManipulationCompletedRoutedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IManipulationCompletedRoutedEventArgs {
    type Vtable = IManipulationCompletedRoutedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xe3be9e4e_c5fb_5859_a81d_ce12fc3a2f4d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationCompletedRoutedEventArgs_abi(
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
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut super::super::Input::ManipulationDelta,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Input"))] usize,
    #[cfg(feature = "UI_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut super::super::Input::ManipulationVelocities,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Input"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut super::super::Input::PointerDeviceType,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Input"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IManipulationDeltaRoutedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IManipulationDeltaRoutedEventArgs {
    type Vtable = IManipulationDeltaRoutedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x51369745_960f_54ac_93fa_763d22910dea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationDeltaRoutedEventArgs_abi(
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
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut super::super::Input::ManipulationDelta,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Input"))] usize,
    #[cfg(feature = "UI_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut super::super::Input::ManipulationDelta,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Input"))] usize,
    #[cfg(feature = "UI_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut super::super::Input::ManipulationVelocities,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Input"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut super::super::Input::PointerDeviceType,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Input"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IManipulationInertiaStartingRoutedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IManipulationInertiaStartingRoutedEventArgs {
    type Vtable = IManipulationInertiaStartingRoutedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x17d510be_5514_5952_9afd_959b60ab9394);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationInertiaStartingRoutedEventArgs_abi(
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
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    )))]
    usize,
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
    #[cfg(feature = "UI_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut super::super::Input::PointerDeviceType,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Input"))] usize,
    #[cfg(feature = "UI_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut super::super::Input::ManipulationDelta,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Input"))] usize,
    #[cfg(feature = "UI_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut super::super::Input::ManipulationDelta,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Input"))] usize,
    #[cfg(feature = "UI_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut super::super::Input::ManipulationVelocities,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Input"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IManipulationPivot(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IManipulationPivot {
    type Vtable = IManipulationPivot_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x286baba4_313d_507c_adc5_f739732cea27);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationPivot_abi(
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
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: f64,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IManipulationPivotFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IManipulationPivotFactory {
    type Vtable = IManipulationPivotFactory_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x67143ccd_ea6c_5fe2_bef2_adcbd7af52fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationPivotFactory_abi(
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
        center: ::windows::Foundation::Point,
        radius: f64,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IManipulationStartedRoutedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IManipulationStartedRoutedEventArgs {
    type Vtable = IManipulationStartedRoutedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x61857950_5821_5652_9fdf_c6277c5886f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationStartedRoutedEventArgs_abi(
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
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut super::super::Input::PointerDeviceType,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Input"))] usize,
    #[cfg(feature = "UI_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut super::super::Input::ManipulationDelta,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Input"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IManipulationStartedRoutedEventArgsFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IManipulationStartedRoutedEventArgsFactory {
    type Vtable = IManipulationStartedRoutedEventArgsFactory_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x5681b0de_3fa7_503e_9c46_a80339760292);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationStartedRoutedEventArgsFactory_abi(
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
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        baseinterface: ::windows::core::RawPtr,
        innerinterface: *mut ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    )))]
    usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IManipulationStartingRoutedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IManipulationStartingRoutedEventArgs {
    type Vtable = IManipulationStartingRoutedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x93a99f86_f5a0_5326_91b0_851c897af79f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationStartingRoutedEventArgs_abi(
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
        result__: *mut ManipulationModes,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ManipulationModes,
    ) -> ::windows::core::HRESULT,
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    )))]
    usize,
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    )))]
    usize,
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct INoFocusCandidateFoundEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INoFocusCandidateFoundEventArgs {
    type Vtable = INoFocusCandidateFoundEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa2d7153a_cd2a_59cb_a574_ac82e30b9201);
}
#[repr(C)]
#[doc(hidden)]
pub struct INoFocusCandidateFoundEventArgs_abi(
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
        result__: *mut FocusNavigationDirection,
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
        result__: *mut FocusInputDeviceKind,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPointer(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPointer {
    type Vtable = IPointer_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x1f9afbf5_11a3_5e68_aa1b_72febfa0ab23);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointer_abi(
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
    #[cfg(feature = "UI_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut super::super::Input::PointerDeviceType,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Input"))] usize,
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
pub struct IPointerRoutedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPointerRoutedEventArgs {
    type Vtable = IPointerRoutedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x66e78a9a_1bec_5f92_b1a1_ea6334ee511c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerRoutedEventArgs_abi(
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
    #[cfg(feature = "UI_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Input"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::System::VirtualKeyModifiers,
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
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        relativeto: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    )))]
    usize,
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        relativeto: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    )))]
    usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProcessKeyboardAcceleratorEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IProcessKeyboardAcceleratorEventArgs {
    type Vtable = IProcessKeyboardAcceleratorEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x9be0d058_3d26_5811_b50a_3bb80ca766c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessKeyboardAcceleratorEventArgs_abi(
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
        result__: *mut ::windows::System::VirtualKey,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::System::VirtualKeyModifiers,
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
pub struct IRightTappedRoutedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRightTappedRoutedEventArgs {
    type Vtable = IRightTappedRoutedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x3972fafb_2915_5c62_bb6b_54ad84ff400d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRightTappedRoutedEventArgs_abi(
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
    #[cfg(feature = "UI_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut super::super::Input::PointerDeviceType,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Input"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        relativeto: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    )))]
    usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStandardUICommand(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStandardUICommand {
    type Vtable = IStandardUICommand_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x5f395d50_5449_59ab_9cb2_4e3700033f03);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStandardUICommand_abi(
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
        result__: *mut StandardUICommandKind,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: StandardUICommandKind,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStandardUICommandFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStandardUICommandFactory {
    type Vtable = IStandardUICommandFactory_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x5800f099_3746_5bcf_b1ce_af3d6bf8e83f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStandardUICommandFactory_abi(
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
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        baseinterface: ::windows::core::RawPtr,
        innerinterface: *mut ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        kind: StandardUICommandKind,
        baseinterface: ::windows::core::RawPtr,
        innerinterface: *mut ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStandardUICommandStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStandardUICommandStatics {
    type Vtable = IStandardUICommandStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xab80c197_85cc_5d36_81aa_156cd63be31a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStandardUICommandStatics_abi(
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
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITappedRoutedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITappedRoutedEventArgs {
    type Vtable = ITappedRoutedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x73f74b8c_3709_547e_8e0c_51c03c89126a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITappedRoutedEventArgs_abi(
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
    #[cfg(feature = "UI_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut super::super::Input::PointerDeviceType,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Input"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        relativeto: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    )))]
    usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlUICommand(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlUICommand {
    type Vtable = IXamlUICommand_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa457f2cb_51e0_541c_9c42_dd1dcbdf58fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUICommand_abi(
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
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Media"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Media"
    )))]
    usize,
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Media"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Media"
    )))]
    usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
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
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Media"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Media"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Media"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Media"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlUICommandFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlUICommandFactory {
    type Vtable = IXamlUICommandFactory_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xf1f80a20_0e31_5505_8bc3_cdd1f0947f1d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUICommandFactory_abi(
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
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Media"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        baseinterface: ::windows::core::RawPtr,
        innerinterface: *mut ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Media"
    )))]
    usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlUICommandStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlUICommandStatics {
    type Vtable = IXamlUICommandStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x981dbda6_cdcb_5e35_b24b_c4f60ba148d9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUICommandStatics_abi(
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
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
);
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct InertiaExpansionBehavior(pub ::windows::core::IInspectable);
impl InertiaExpansionBehavior {
    pub fn DesiredDeceleration(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetDesiredDeceleration(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn DesiredExpansion(&self) -> ::windows::core::Result<f64> {
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
    pub fn SetDesiredExpansion(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for InertiaExpansionBehavior {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Input.InertiaExpansionBehavior;{d60029b7-f0cd-5aea-abe5-7410d09118c6})" ) ;
}
unsafe impl ::windows::core::Interface for InertiaExpansionBehavior {
    type Vtable = IInertiaExpansionBehavior_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xd60029b7_f0cd_5aea_abe5_7410d09118c6);
}
impl ::windows::core::RuntimeName for InertiaExpansionBehavior {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.InertiaExpansionBehavior";
}
impl ::core::convert::From<InertiaExpansionBehavior> for ::windows::core::IUnknown {
    fn from(value: InertiaExpansionBehavior) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InertiaExpansionBehavior> for ::windows::core::IUnknown {
    fn from(value: &InertiaExpansionBehavior) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InertiaExpansionBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a InertiaExpansionBehavior
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InertiaExpansionBehavior> for ::windows::core::IInspectable {
    fn from(value: InertiaExpansionBehavior) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InertiaExpansionBehavior> for ::windows::core::IInspectable {
    fn from(value: &InertiaExpansionBehavior) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for InertiaExpansionBehavior
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a InertiaExpansionBehavior
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for InertiaExpansionBehavior {}
unsafe impl ::core::marker::Sync for InertiaExpansionBehavior {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct InertiaRotationBehavior(pub ::windows::core::IInspectable);
impl InertiaRotationBehavior {
    pub fn DesiredDeceleration(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetDesiredDeceleration(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn DesiredRotation(&self) -> ::windows::core::Result<f64> {
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
    pub fn SetDesiredRotation(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for InertiaRotationBehavior {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Input.InertiaRotationBehavior;{27b4bd03-9149-5691-bce5-fa33b32c4a81})" ) ;
}
unsafe impl ::windows::core::Interface for InertiaRotationBehavior {
    type Vtable = IInertiaRotationBehavior_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x27b4bd03_9149_5691_bce5_fa33b32c4a81);
}
impl ::windows::core::RuntimeName for InertiaRotationBehavior {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.InertiaRotationBehavior";
}
impl ::core::convert::From<InertiaRotationBehavior> for ::windows::core::IUnknown {
    fn from(value: InertiaRotationBehavior) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InertiaRotationBehavior> for ::windows::core::IUnknown {
    fn from(value: &InertiaRotationBehavior) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InertiaRotationBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InertiaRotationBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InertiaRotationBehavior> for ::windows::core::IInspectable {
    fn from(value: InertiaRotationBehavior) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InertiaRotationBehavior> for ::windows::core::IInspectable {
    fn from(value: &InertiaRotationBehavior) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InertiaRotationBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a InertiaRotationBehavior
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for InertiaRotationBehavior {}
unsafe impl ::core::marker::Sync for InertiaRotationBehavior {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct InertiaTranslationBehavior(pub ::windows::core::IInspectable);
impl InertiaTranslationBehavior {
    pub fn DesiredDeceleration(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetDesiredDeceleration(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn DesiredDisplacement(&self) -> ::windows::core::Result<f64> {
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
    pub fn SetDesiredDisplacement(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for InertiaTranslationBehavior {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Input.InertiaTranslationBehavior;{d4f91cf5-3317-5914-b25a-ea6ee55b96d0})" ) ;
}
unsafe impl ::windows::core::Interface for InertiaTranslationBehavior {
    type Vtable = IInertiaTranslationBehavior_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xd4f91cf5_3317_5914_b25a_ea6ee55b96d0);
}
impl ::windows::core::RuntimeName for InertiaTranslationBehavior {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.InertiaTranslationBehavior";
}
impl ::core::convert::From<InertiaTranslationBehavior> for ::windows::core::IUnknown {
    fn from(value: InertiaTranslationBehavior) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InertiaTranslationBehavior> for ::windows::core::IUnknown {
    fn from(value: &InertiaTranslationBehavior) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InertiaTranslationBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a InertiaTranslationBehavior
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InertiaTranslationBehavior> for ::windows::core::IInspectable {
    fn from(value: InertiaTranslationBehavior) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InertiaTranslationBehavior> for ::windows::core::IInspectable {
    fn from(value: &InertiaTranslationBehavior) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for InertiaTranslationBehavior
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a InertiaTranslationBehavior
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for InertiaTranslationBehavior {}
unsafe impl ::core::marker::Sync for InertiaTranslationBehavior {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct InputScope(pub ::windows::core::IInspectable);
impl InputScope {
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
            InputScope,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Names(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Collections::IVector<InputScopeName>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVector<InputScopeName>>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ClearValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ReadLocalValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for InputScope {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Input.InputScope;{76ea58b1-e910-5176-9147-695cc95e7da2})",
    );
}
unsafe impl ::windows::core::Interface for InputScope {
    type Vtable = IInputScope_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x76ea58b1_e910_5176_9147_695cc95e7da2);
}
impl ::windows::core::RuntimeName for InputScope {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.InputScope";
}
impl ::core::convert::From<InputScope> for ::windows::core::IUnknown {
    fn from(value: InputScope) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InputScope> for ::windows::core::IUnknown {
    fn from(value: &InputScope) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InputScope {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InputScope {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InputScope> for ::windows::core::IInspectable {
    fn from(value: InputScope) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InputScope> for ::windows::core::IInspectable {
    fn from(value: &InputScope) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InputScope {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InputScope {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<InputScope> for super::DependencyObject {
    fn from(value: InputScope) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&InputScope> for super::DependencyObject {
    fn from(value: &InputScope) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for InputScope {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &InputScope {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for InputScope {}
unsafe impl ::core::marker::Sync for InputScope {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct InputScopeName(pub ::windows::core::IInspectable);
impl InputScopeName {
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
            InputScopeName,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn NameValue(&self) -> ::windows::core::Result<InputScopeNameValue> {
        let this = self;
        unsafe {
            let mut result__: InputScopeNameValue = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InputScopeNameValue>(result__)
        }
    }
    pub fn SetNameValue(&self, value: InputScopeNameValue) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn CreateInstance(
        namevalue: InputScopeNameValue,
    ) -> ::windows::core::Result<InputScopeName> {
        Self::IInputScopeNameFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                namevalue,
                &mut result__,
            )
            .from_abi::<InputScopeName>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ClearValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ReadLocalValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn IInputScopeNameFactory<
        R,
        F: FnOnce(&IInputScopeNameFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<InputScopeName, IInputScopeNameFactory> =
            ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for InputScopeName {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Input.InputScopeName;{ee99a66d-28d0-53cb-82ee-1b6ee58bcc35})",
    );
}
unsafe impl ::windows::core::Interface for InputScopeName {
    type Vtable = IInputScopeName_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xee99a66d_28d0_53cb_82ee_1b6ee58bcc35);
}
impl ::windows::core::RuntimeName for InputScopeName {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.InputScopeName";
}
impl ::core::convert::From<InputScopeName> for ::windows::core::IUnknown {
    fn from(value: InputScopeName) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InputScopeName> for ::windows::core::IUnknown {
    fn from(value: &InputScopeName) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InputScopeName {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InputScopeName {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InputScopeName> for ::windows::core::IInspectable {
    fn from(value: InputScopeName) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InputScopeName> for ::windows::core::IInspectable {
    fn from(value: &InputScopeName) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InputScopeName {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InputScopeName {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<InputScopeName> for super::DependencyObject {
    fn from(value: InputScopeName) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&InputScopeName> for super::DependencyObject {
    fn from(value: &InputScopeName) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for InputScopeName {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &InputScopeName {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for InputScopeName {}
unsafe impl ::core::marker::Sync for InputScopeName {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct InputScopeNameValue(pub i32);
impl InputScopeNameValue {
    pub const Default: InputScopeNameValue = InputScopeNameValue(0i32);
    pub const Url: InputScopeNameValue = InputScopeNameValue(1i32);
    pub const EmailSmtpAddress: InputScopeNameValue = InputScopeNameValue(5i32);
    pub const PersonalFullName: InputScopeNameValue = InputScopeNameValue(7i32);
    pub const CurrencyAmountAndSymbol: InputScopeNameValue = InputScopeNameValue(20i32);
    pub const CurrencyAmount: InputScopeNameValue = InputScopeNameValue(21i32);
    pub const DateMonthNumber: InputScopeNameValue = InputScopeNameValue(23i32);
    pub const DateDayNumber: InputScopeNameValue = InputScopeNameValue(24i32);
    pub const DateYear: InputScopeNameValue = InputScopeNameValue(25i32);
    pub const Digits: InputScopeNameValue = InputScopeNameValue(28i32);
    pub const Number: InputScopeNameValue = InputScopeNameValue(29i32);
    pub const Password: InputScopeNameValue = InputScopeNameValue(31i32);
    pub const TelephoneNumber: InputScopeNameValue = InputScopeNameValue(32i32);
    pub const TelephoneCountryCode: InputScopeNameValue = InputScopeNameValue(33i32);
    pub const TelephoneAreaCode: InputScopeNameValue = InputScopeNameValue(34i32);
    pub const TelephoneLocalNumber: InputScopeNameValue = InputScopeNameValue(35i32);
    pub const TimeHour: InputScopeNameValue = InputScopeNameValue(37i32);
    pub const TimeMinutesOrSeconds: InputScopeNameValue = InputScopeNameValue(38i32);
    pub const NumberFullWidth: InputScopeNameValue = InputScopeNameValue(39i32);
    pub const AlphanumericHalfWidth: InputScopeNameValue = InputScopeNameValue(40i32);
    pub const AlphanumericFullWidth: InputScopeNameValue = InputScopeNameValue(41i32);
    pub const Hiragana: InputScopeNameValue = InputScopeNameValue(44i32);
    pub const KatakanaHalfWidth: InputScopeNameValue = InputScopeNameValue(45i32);
    pub const KatakanaFullWidth: InputScopeNameValue = InputScopeNameValue(46i32);
    pub const Hanja: InputScopeNameValue = InputScopeNameValue(47i32);
    pub const HangulHalfWidth: InputScopeNameValue = InputScopeNameValue(48i32);
    pub const HangulFullWidth: InputScopeNameValue = InputScopeNameValue(49i32);
    pub const Search: InputScopeNameValue = InputScopeNameValue(50i32);
    pub const Formula: InputScopeNameValue = InputScopeNameValue(51i32);
    pub const SearchIncremental: InputScopeNameValue = InputScopeNameValue(52i32);
    pub const ChineseHalfWidth: InputScopeNameValue = InputScopeNameValue(53i32);
    pub const ChineseFullWidth: InputScopeNameValue = InputScopeNameValue(54i32);
    pub const NativeScript: InputScopeNameValue = InputScopeNameValue(55i32);
    pub const Text: InputScopeNameValue = InputScopeNameValue(57i32);
    pub const Chat: InputScopeNameValue = InputScopeNameValue(58i32);
    pub const NameOrPhoneNumber: InputScopeNameValue = InputScopeNameValue(59i32);
    pub const EmailNameOrAddress: InputScopeNameValue = InputScopeNameValue(60i32);
    pub const Maps: InputScopeNameValue = InputScopeNameValue(62i32);
    pub const NumericPassword: InputScopeNameValue = InputScopeNameValue(63i32);
    pub const NumericPin: InputScopeNameValue = InputScopeNameValue(64i32);
    pub const AlphanumericPin: InputScopeNameValue = InputScopeNameValue(65i32);
    pub const FormulaNumber: InputScopeNameValue = InputScopeNameValue(67i32);
    pub const ChatWithoutEmoji: InputScopeNameValue = InputScopeNameValue(68i32);
}
impl ::core::convert::From<i32> for InputScopeNameValue {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for InputScopeNameValue {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for InputScopeNameValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Input.InputScopeNameValue;i4)",
    );
}
impl ::windows::core::DefaultType for InputScopeNameValue {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct KeyEventHandler(::windows::core::IUnknown);
impl KeyEventHandler {
    pub fn new<
        F: FnMut(
                &::core::option::Option<::windows::core::IInspectable>,
                &::core::option::Option<KeyRoutedEventArgs>,
            ) -> ::windows::core::Result<()>
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = KeyEventHandler_box::<F> {
            vtable: &KeyEventHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param1: ::windows::core::IntoParam<'a, KeyRoutedEventArgs>,
    >(
        &self,
        sender: Param0,
        e: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).3)(
                ::core::mem::transmute_copy(this),
                sender.into_param().abi(),
                e.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for KeyEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"delegate({db68e7cc-9a2b-527d-9989-25284daccc03})",
    );
}
unsafe impl ::windows::core::Interface for KeyEventHandler {
    type Vtable = KeyEventHandler_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xdb68e7cc_9a2b_527d_9989_25284daccc03);
}
#[repr(C)]
#[doc(hidden)]
pub struct KeyEventHandler_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        sender: ::windows::core::RawPtr,
        e: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(C)]
struct KeyEventHandler_box<
    F: FnMut(
            &::core::option::Option<::windows::core::IInspectable>,
            &::core::option::Option<KeyRoutedEventArgs>,
        ) -> ::windows::core::Result<()>
        + 'static,
> {
    vtable: *const KeyEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<
        F: FnMut(
                &::core::option::Option<::windows::core::IInspectable>,
                &::core::option::Option<KeyRoutedEventArgs>,
            ) -> ::windows::core::Result<()>
            + 'static,
    > KeyEventHandler_box<F>
{
    const VTABLE: KeyEventHandler_abi = KeyEventHandler_abi(
        Self::QueryInterface,
        Self::AddRef,
        Self::Release,
        Self::Invoke,
    );
    unsafe extern "system" fn QueryInterface(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<KeyEventHandler as ::windows::core::Interface>::IID
            || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID
            || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::core::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: ::windows::core::RawPtr,
        sender: ::windows::core::RawPtr,
        e: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ( ( * this ) . invoke ) ( & * ( & sender as * const < :: windows :: core :: IInspectable as :: windows :: core :: Abi > :: Abi as * const < :: windows :: core :: IInspectable as :: windows :: core :: DefaultType > :: DefaultType ) , & * ( & e as * const < KeyRoutedEventArgs as :: windows :: core :: Abi > :: Abi as * const < KeyRoutedEventArgs as :: windows :: core :: DefaultType > :: DefaultType ) , ) . into ( )
    }
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct KeyRoutedEventArgs(pub ::windows::core::IInspectable);
impl KeyRoutedEventArgs {
    pub fn Key(&self) -> ::windows::core::Result<::windows::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__: ::windows::System::VirtualKey = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::System::VirtualKey>(result__)
        }
    }
    pub fn KeyStatus(&self) -> ::windows::core::Result<::windows::UI::Core::CorePhysicalKeyStatus> {
        let this = self;
        unsafe {
            let mut result__: ::windows::UI::Core::CorePhysicalKeyStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CorePhysicalKeyStatus>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
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
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn OriginalKey(&self) -> ::windows::core::Result<::windows::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__: ::windows::System::VirtualKey = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::System::VirtualKey>(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
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
    pub fn OriginalSource(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for KeyRoutedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Input.KeyRoutedEventArgs;{ee357007-a2d6-5c75-9431-05fd66ec7915})",
    );
}
unsafe impl ::windows::core::Interface for KeyRoutedEventArgs {
    type Vtable = IKeyRoutedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xee357007_a2d6_5c75_9431_05fd66ec7915);
}
impl ::windows::core::RuntimeName for KeyRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.KeyRoutedEventArgs";
}
impl ::core::convert::From<KeyRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: KeyRoutedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&KeyRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &KeyRoutedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for KeyRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a KeyRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<KeyRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: KeyRoutedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&KeyRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &KeyRoutedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for KeyRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a KeyRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<KeyRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: KeyRoutedEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&KeyRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &KeyRoutedEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for KeyRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &KeyRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for KeyRoutedEventArgs {}
unsafe impl ::core::marker::Sync for KeyRoutedEventArgs {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct KeyTipPlacementMode(pub i32);
impl KeyTipPlacementMode {
    pub const Auto: KeyTipPlacementMode = KeyTipPlacementMode(0i32);
    pub const Bottom: KeyTipPlacementMode = KeyTipPlacementMode(1i32);
    pub const Top: KeyTipPlacementMode = KeyTipPlacementMode(2i32);
    pub const Left: KeyTipPlacementMode = KeyTipPlacementMode(3i32);
    pub const Right: KeyTipPlacementMode = KeyTipPlacementMode(4i32);
    pub const Center: KeyTipPlacementMode = KeyTipPlacementMode(5i32);
    pub const Hidden: KeyTipPlacementMode = KeyTipPlacementMode(6i32);
}
impl ::core::convert::From<i32> for KeyTipPlacementMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for KeyTipPlacementMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for KeyTipPlacementMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Input.KeyTipPlacementMode;i4)",
    );
}
impl ::windows::core::DefaultType for KeyTipPlacementMode {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct KeyboardAccelerator(pub ::windows::core::IInspectable);
impl KeyboardAccelerator {
    pub fn Key(&self) -> ::windows::core::Result<::windows::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__: ::windows::System::VirtualKey = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::System::VirtualKey>(result__)
        }
    }
    pub fn SetKey(&self, value: ::windows::System::VirtualKey) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn Modifiers(&self) -> ::windows::core::Result<::windows::System::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__: ::windows::System::VirtualKeyModifiers = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::System::VirtualKeyModifiers>(result__)
        }
    }
    pub fn SetModifiers(
        &self,
        value: ::windows::System::VirtualKeyModifiers,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
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
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetScopeOwner<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
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
    #[cfg(feature = "UI_Dispatching")]
    pub fn Invoked<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                KeyboardAccelerator,
                KeyboardAcceleratorInvokedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveInvoked<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn KeyProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IKeyboardAcceleratorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ModifiersProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IKeyboardAcceleratorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn IsEnabledProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IKeyboardAcceleratorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ScopeOwnerProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IKeyboardAcceleratorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn new() -> ::windows::core::Result<KeyboardAccelerator> {
        Self::IKeyboardAcceleratorFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                ::core::ptr::null_mut(),
                &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _,
                &mut result__,
            )
            .from_abi::<KeyboardAccelerator>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ClearValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ReadLocalValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn IKeyboardAcceleratorStatics<
        R,
        F: FnOnce(&IKeyboardAcceleratorStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            KeyboardAccelerator,
            IKeyboardAcceleratorStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IKeyboardAcceleratorFactory<
        R,
        F: FnOnce(&IKeyboardAcceleratorFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            KeyboardAccelerator,
            IKeyboardAcceleratorFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for KeyboardAccelerator {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Input.KeyboardAccelerator;{6f8bf1e2-4e91-5cf9-a6be-4770caf3d770})",
    );
}
unsafe impl ::windows::core::Interface for KeyboardAccelerator {
    type Vtable = IKeyboardAccelerator_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x6f8bf1e2_4e91_5cf9_a6be_4770caf3d770);
}
impl ::windows::core::RuntimeName for KeyboardAccelerator {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.KeyboardAccelerator";
}
impl ::core::convert::From<KeyboardAccelerator> for ::windows::core::IUnknown {
    fn from(value: KeyboardAccelerator) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&KeyboardAccelerator> for ::windows::core::IUnknown {
    fn from(value: &KeyboardAccelerator) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for KeyboardAccelerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a KeyboardAccelerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<KeyboardAccelerator> for ::windows::core::IInspectable {
    fn from(value: KeyboardAccelerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&KeyboardAccelerator> for ::windows::core::IInspectable {
    fn from(value: &KeyboardAccelerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for KeyboardAccelerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a KeyboardAccelerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<KeyboardAccelerator> for super::DependencyObject {
    fn from(value: KeyboardAccelerator) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&KeyboardAccelerator> for super::DependencyObject {
    fn from(value: &KeyboardAccelerator) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for KeyboardAccelerator {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &KeyboardAccelerator {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for KeyboardAccelerator {}
unsafe impl ::core::marker::Sync for KeyboardAccelerator {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct KeyboardAcceleratorInvokedEventArgs(pub ::windows::core::IInspectable);
impl KeyboardAcceleratorInvokedEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
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
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn Element(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn KeyboardAccelerator(&self) -> ::windows::core::Result<KeyboardAccelerator> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<KeyboardAccelerator>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for KeyboardAcceleratorInvokedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Input.KeyboardAcceleratorInvokedEventArgs;{62c9fdb0-b574-527d-97eb-5c7f674441e0})" ) ;
}
unsafe impl ::windows::core::Interface for KeyboardAcceleratorInvokedEventArgs {
    type Vtable = IKeyboardAcceleratorInvokedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x62c9fdb0_b574_527d_97eb_5c7f674441e0);
}
impl ::windows::core::RuntimeName for KeyboardAcceleratorInvokedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.KeyboardAcceleratorInvokedEventArgs";
}
impl ::core::convert::From<KeyboardAcceleratorInvokedEventArgs> for ::windows::core::IUnknown {
    fn from(value: KeyboardAcceleratorInvokedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&KeyboardAcceleratorInvokedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &KeyboardAcceleratorInvokedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for KeyboardAcceleratorInvokedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a KeyboardAcceleratorInvokedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<KeyboardAcceleratorInvokedEventArgs> for ::windows::core::IInspectable {
    fn from(value: KeyboardAcceleratorInvokedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&KeyboardAcceleratorInvokedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &KeyboardAcceleratorInvokedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for KeyboardAcceleratorInvokedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a KeyboardAcceleratorInvokedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for KeyboardAcceleratorInvokedEventArgs {}
unsafe impl ::core::marker::Sync for KeyboardAcceleratorInvokedEventArgs {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct KeyboardAcceleratorPlacementMode(pub i32);
impl KeyboardAcceleratorPlacementMode {
    pub const Auto: KeyboardAcceleratorPlacementMode = KeyboardAcceleratorPlacementMode(0i32);
    pub const Hidden: KeyboardAcceleratorPlacementMode = KeyboardAcceleratorPlacementMode(1i32);
}
impl ::core::convert::From<i32> for KeyboardAcceleratorPlacementMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for KeyboardAcceleratorPlacementMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for KeyboardAcceleratorPlacementMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Input.KeyboardAcceleratorPlacementMode;i4)",
    );
}
impl ::windows::core::DefaultType for KeyboardAcceleratorPlacementMode {
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
pub struct KeyboardNavigationMode(pub i32);
impl KeyboardNavigationMode {
    pub const Local: KeyboardNavigationMode = KeyboardNavigationMode(0i32);
    pub const Cycle: KeyboardNavigationMode = KeyboardNavigationMode(1i32);
    pub const Once: KeyboardNavigationMode = KeyboardNavigationMode(2i32);
}
impl ::core::convert::From<i32> for KeyboardNavigationMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for KeyboardNavigationMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for KeyboardNavigationMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Input.KeyboardNavigationMode;i4)",
    );
}
impl ::windows::core::DefaultType for KeyboardNavigationMode {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct LosingFocusEventArgs(pub ::windows::core::IInspectable);
impl LosingFocusEventArgs {
    #[cfg(feature = "UI_Dispatching")]
    pub fn OldFocusedElement(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn NewFocusedElement(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetNewFocusedElement<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
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
    pub fn FocusState(&self) -> ::windows::core::Result<super::FocusState> {
        let this = self;
        unsafe {
            let mut result__: super::FocusState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::FocusState>(result__)
        }
    }
    pub fn Direction(&self) -> ::windows::core::Result<FocusNavigationDirection> {
        let this = self;
        unsafe {
            let mut result__: FocusNavigationDirection = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<FocusNavigationDirection>(result__)
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
    pub fn InputDevice(&self) -> ::windows::core::Result<FocusInputDeviceKind> {
        let this = self;
        unsafe {
            let mut result__: FocusInputDeviceKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<FocusInputDeviceKind>(result__)
        }
    }
    pub fn Cancel(&self) -> ::windows::core::Result<bool> {
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
    pub fn SetCancel(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn CorrelationId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn TryCancel(&self) -> ::windows::core::Result<bool> {
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
    #[cfg(feature = "UI_Dispatching")]
    pub fn TrySetNewFocusedElement<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn OriginalSource(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for LosingFocusEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Input.LosingFocusEventArgs;{fa0e5ffa-2b1b-52f8-bb66-e35f51e73cf3})",
    );
}
unsafe impl ::windows::core::Interface for LosingFocusEventArgs {
    type Vtable = ILosingFocusEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xfa0e5ffa_2b1b_52f8_bb66_e35f51e73cf3);
}
impl ::windows::core::RuntimeName for LosingFocusEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.LosingFocusEventArgs";
}
impl ::core::convert::From<LosingFocusEventArgs> for ::windows::core::IUnknown {
    fn from(value: LosingFocusEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LosingFocusEventArgs> for ::windows::core::IUnknown {
    fn from(value: &LosingFocusEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LosingFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LosingFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LosingFocusEventArgs> for ::windows::core::IInspectable {
    fn from(value: LosingFocusEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LosingFocusEventArgs> for ::windows::core::IInspectable {
    fn from(value: &LosingFocusEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LosingFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a LosingFocusEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<LosingFocusEventArgs> for super::RoutedEventArgs {
    fn from(value: LosingFocusEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&LosingFocusEventArgs> for super::RoutedEventArgs {
    fn from(value: &LosingFocusEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for LosingFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &LosingFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for LosingFocusEventArgs {}
unsafe impl ::core::marker::Sync for LosingFocusEventArgs {}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct ManipulationCompletedEventHandler(::windows::core::IUnknown);
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
impl ManipulationCompletedEventHandler {
    pub fn new<
        F: FnMut(
                &::core::option::Option<::windows::core::IInspectable>,
                &::core::option::Option<ManipulationCompletedRoutedEventArgs>,
            ) -> ::windows::core::Result<()>
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = ManipulationCompletedEventHandler_box::<F> {
            vtable: &ManipulationCompletedEventHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn Invoke<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param1: ::windows::core::IntoParam<'a, ManipulationCompletedRoutedEventArgs>,
    >(
        &self,
        sender: Param0,
        e: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).3)(
                ::core::mem::transmute_copy(this),
                sender.into_param().abi(),
                e.into_param().abi(),
            )
            .ok()
        }
    }
}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
unsafe impl ::windows::core::RuntimeType for ManipulationCompletedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"delegate({d51df8db-71cd-5bfd-8426-767218ee55ec})",
    );
}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
unsafe impl ::windows::core::Interface for ManipulationCompletedEventHandler {
    type Vtable = ManipulationCompletedEventHandler_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xd51df8db_71cd_5bfd_8426_767218ee55ec);
}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
#[repr(C)]
#[doc(hidden)]
pub struct ManipulationCompletedEventHandler_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        sender: ::windows::core::RawPtr,
        e: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
#[repr(C)]
struct ManipulationCompletedEventHandler_box<
    F: FnMut(
            &::core::option::Option<::windows::core::IInspectable>,
            &::core::option::Option<ManipulationCompletedRoutedEventArgs>,
        ) -> ::windows::core::Result<()>
        + 'static,
> {
    vtable: *const ManipulationCompletedEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
impl<
        F: FnMut(
                &::core::option::Option<::windows::core::IInspectable>,
                &::core::option::Option<ManipulationCompletedRoutedEventArgs>,
            ) -> ::windows::core::Result<()>
            + 'static,
    > ManipulationCompletedEventHandler_box<F>
{
    const VTABLE: ManipulationCompletedEventHandler_abi = ManipulationCompletedEventHandler_abi(
        Self::QueryInterface,
        Self::AddRef,
        Self::Release,
        Self::Invoke,
    );
    unsafe extern "system" fn QueryInterface(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid
            == &<ManipulationCompletedEventHandler as ::windows::core::Interface>::IID
            || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID
            || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::core::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: ::windows::core::RawPtr,
        sender: ::windows::core::RawPtr,
        e: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ( ( * this ) . invoke ) ( & * ( & sender as * const < :: windows :: core :: IInspectable as :: windows :: core :: Abi > :: Abi as * const < :: windows :: core :: IInspectable as :: windows :: core :: DefaultType > :: DefaultType ) , & * ( & e as * const < ManipulationCompletedRoutedEventArgs as :: windows :: core :: Abi > :: Abi as * const < ManipulationCompletedRoutedEventArgs as :: windows :: core :: DefaultType > :: DefaultType ) , ) . into ( )
    }
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct ManipulationCompletedRoutedEventArgs(pub ::windows::core::IInspectable);
impl ManipulationCompletedRoutedEventArgs {
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
            ManipulationCompletedRoutedEventArgs,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn Container(&self) -> ::windows::core::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::UIElement>(result__)
        }
    }
    pub fn Position(&self) -> ::windows::core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Point>(result__)
        }
    }
    pub fn IsInertial(&self) -> ::windows::core::Result<bool> {
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
    #[cfg(feature = "UI_Input")]
    pub fn Cumulative(&self) -> ::windows::core::Result<super::super::Input::ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::ManipulationDelta = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Input::ManipulationDelta>(result__)
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn Velocities(
        &self,
    ) -> ::windows::core::Result<super::super::Input::ManipulationVelocities> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::ManipulationVelocities = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Input::ManipulationVelocities>(result__)
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
    #[cfg(feature = "UI_Input")]
    pub fn PointerDeviceType(
        &self,
    ) -> ::windows::core::Result<super::super::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Input::PointerDeviceType>(result__)
        }
    }
    pub fn OriginalSource(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ManipulationCompletedRoutedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Input.ManipulationCompletedRoutedEventArgs;{e3be9e4e-c5fb-5859-a81d-ce12fc3a2f4d})" ) ;
}
unsafe impl ::windows::core::Interface for ManipulationCompletedRoutedEventArgs {
    type Vtable = IManipulationCompletedRoutedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xe3be9e4e_c5fb_5859_a81d_ce12fc3a2f4d);
}
impl ::windows::core::RuntimeName for ManipulationCompletedRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.ManipulationCompletedRoutedEventArgs";
}
impl ::core::convert::From<ManipulationCompletedRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ManipulationCompletedRoutedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ManipulationCompletedRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ManipulationCompletedRoutedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for ManipulationCompletedRoutedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a ManipulationCompletedRoutedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ManipulationCompletedRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ManipulationCompletedRoutedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ManipulationCompletedRoutedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: &ManipulationCompletedRoutedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for ManipulationCompletedRoutedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a ManipulationCompletedRoutedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ManipulationCompletedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: ManipulationCompletedRoutedEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&ManipulationCompletedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &ManipulationCompletedRoutedEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs>
    for ManipulationCompletedRoutedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs>
    for &ManipulationCompletedRoutedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for ManipulationCompletedRoutedEventArgs {}
unsafe impl ::core::marker::Sync for ManipulationCompletedRoutedEventArgs {}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct ManipulationDeltaEventHandler(::windows::core::IUnknown);
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
impl ManipulationDeltaEventHandler {
    pub fn new<
        F: FnMut(
                &::core::option::Option<::windows::core::IInspectable>,
                &::core::option::Option<ManipulationDeltaRoutedEventArgs>,
            ) -> ::windows::core::Result<()>
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = ManipulationDeltaEventHandler_box::<F> {
            vtable: &ManipulationDeltaEventHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn Invoke<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param1: ::windows::core::IntoParam<'a, ManipulationDeltaRoutedEventArgs>,
    >(
        &self,
        sender: Param0,
        e: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).3)(
                ::core::mem::transmute_copy(this),
                sender.into_param().abi(),
                e.into_param().abi(),
            )
            .ok()
        }
    }
}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
unsafe impl ::windows::core::RuntimeType for ManipulationDeltaEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"delegate({83f2d4ce-105f-5392-a38a-b7467b7c2ea5})",
    );
}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
unsafe impl ::windows::core::Interface for ManipulationDeltaEventHandler {
    type Vtable = ManipulationDeltaEventHandler_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x83f2d4ce_105f_5392_a38a_b7467b7c2ea5);
}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
#[repr(C)]
#[doc(hidden)]
pub struct ManipulationDeltaEventHandler_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        sender: ::windows::core::RawPtr,
        e: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
#[repr(C)]
struct ManipulationDeltaEventHandler_box<
    F: FnMut(
            &::core::option::Option<::windows::core::IInspectable>,
            &::core::option::Option<ManipulationDeltaRoutedEventArgs>,
        ) -> ::windows::core::Result<()>
        + 'static,
> {
    vtable: *const ManipulationDeltaEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
impl<
        F: FnMut(
                &::core::option::Option<::windows::core::IInspectable>,
                &::core::option::Option<ManipulationDeltaRoutedEventArgs>,
            ) -> ::windows::core::Result<()>
            + 'static,
    > ManipulationDeltaEventHandler_box<F>
{
    const VTABLE: ManipulationDeltaEventHandler_abi = ManipulationDeltaEventHandler_abi(
        Self::QueryInterface,
        Self::AddRef,
        Self::Release,
        Self::Invoke,
    );
    unsafe extern "system" fn QueryInterface(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<ManipulationDeltaEventHandler as ::windows::core::Interface>::IID
            || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID
            || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::core::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: ::windows::core::RawPtr,
        sender: ::windows::core::RawPtr,
        e: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ( ( * this ) . invoke ) ( & * ( & sender as * const < :: windows :: core :: IInspectable as :: windows :: core :: Abi > :: Abi as * const < :: windows :: core :: IInspectable as :: windows :: core :: DefaultType > :: DefaultType ) , & * ( & e as * const < ManipulationDeltaRoutedEventArgs as :: windows :: core :: Abi > :: Abi as * const < ManipulationDeltaRoutedEventArgs as :: windows :: core :: DefaultType > :: DefaultType ) , ) . into ( )
    }
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct ManipulationDeltaRoutedEventArgs(pub ::windows::core::IInspectable);
impl ManipulationDeltaRoutedEventArgs {
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
            ManipulationDeltaRoutedEventArgs,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn Container(&self) -> ::windows::core::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::UIElement>(result__)
        }
    }
    pub fn Position(&self) -> ::windows::core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Point>(result__)
        }
    }
    pub fn IsInertial(&self) -> ::windows::core::Result<bool> {
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
    #[cfg(feature = "UI_Input")]
    pub fn Delta(&self) -> ::windows::core::Result<super::super::Input::ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::ManipulationDelta = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Input::ManipulationDelta>(result__)
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn Cumulative(&self) -> ::windows::core::Result<super::super::Input::ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::ManipulationDelta = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Input::ManipulationDelta>(result__)
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn Velocities(
        &self,
    ) -> ::windows::core::Result<super::super::Input::ManipulationVelocities> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::ManipulationVelocities = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Input::ManipulationVelocities>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
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
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn PointerDeviceType(
        &self,
    ) -> ::windows::core::Result<super::super::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Input::PointerDeviceType>(result__)
        }
    }
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn OriginalSource(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ManipulationDeltaRoutedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Input.ManipulationDeltaRoutedEventArgs;{51369745-960f-54ac-93fa-763d22910dea})" ) ;
}
unsafe impl ::windows::core::Interface for ManipulationDeltaRoutedEventArgs {
    type Vtable = IManipulationDeltaRoutedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x51369745_960f_54ac_93fa_763d22910dea);
}
impl ::windows::core::RuntimeName for ManipulationDeltaRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.ManipulationDeltaRoutedEventArgs";
}
impl ::core::convert::From<ManipulationDeltaRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ManipulationDeltaRoutedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ManipulationDeltaRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ManipulationDeltaRoutedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for ManipulationDeltaRoutedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a ManipulationDeltaRoutedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ManipulationDeltaRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ManipulationDeltaRoutedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ManipulationDeltaRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ManipulationDeltaRoutedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for ManipulationDeltaRoutedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a ManipulationDeltaRoutedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ManipulationDeltaRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: ManipulationDeltaRoutedEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&ManipulationDeltaRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &ManipulationDeltaRoutedEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs>
    for ManipulationDeltaRoutedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs>
    for &ManipulationDeltaRoutedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for ManipulationDeltaRoutedEventArgs {}
unsafe impl ::core::marker::Sync for ManipulationDeltaRoutedEventArgs {}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct ManipulationInertiaStartingEventHandler(::windows::core::IUnknown);
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
impl ManipulationInertiaStartingEventHandler {
    pub fn new<
        F: FnMut(
                &::core::option::Option<::windows::core::IInspectable>,
                &::core::option::Option<ManipulationInertiaStartingRoutedEventArgs>,
            ) -> ::windows::core::Result<()>
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = ManipulationInertiaStartingEventHandler_box::<F> {
            vtable: &ManipulationInertiaStartingEventHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn Invoke<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param1: ::windows::core::IntoParam<'a, ManipulationInertiaStartingRoutedEventArgs>,
    >(
        &self,
        sender: Param0,
        e: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).3)(
                ::core::mem::transmute_copy(this),
                sender.into_param().abi(),
                e.into_param().abi(),
            )
            .ok()
        }
    }
}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
unsafe impl ::windows::core::RuntimeType for ManipulationInertiaStartingEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"delegate({5de296bd-6f1c-5f60-9180-10705282576c})",
    );
}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
unsafe impl ::windows::core::Interface for ManipulationInertiaStartingEventHandler {
    type Vtable = ManipulationInertiaStartingEventHandler_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x5de296bd_6f1c_5f60_9180_10705282576c);
}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
#[repr(C)]
#[doc(hidden)]
pub struct ManipulationInertiaStartingEventHandler_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        sender: ::windows::core::RawPtr,
        e: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
#[repr(C)]
struct ManipulationInertiaStartingEventHandler_box<
    F: FnMut(
            &::core::option::Option<::windows::core::IInspectable>,
            &::core::option::Option<ManipulationInertiaStartingRoutedEventArgs>,
        ) -> ::windows::core::Result<()>
        + 'static,
> {
    vtable: *const ManipulationInertiaStartingEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
impl<
        F: FnMut(
                &::core::option::Option<::windows::core::IInspectable>,
                &::core::option::Option<ManipulationInertiaStartingRoutedEventArgs>,
            ) -> ::windows::core::Result<()>
            + 'static,
    > ManipulationInertiaStartingEventHandler_box<F>
{
    const VTABLE: ManipulationInertiaStartingEventHandler_abi =
        ManipulationInertiaStartingEventHandler_abi(
            Self::QueryInterface,
            Self::AddRef,
            Self::Release,
            Self::Invoke,
        );
    unsafe extern "system" fn QueryInterface(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid
            == &<ManipulationInertiaStartingEventHandler as ::windows::core::Interface>::IID
            || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID
            || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::core::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: ::windows::core::RawPtr,
        sender: ::windows::core::RawPtr,
        e: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ( ( * this ) . invoke ) ( & * ( & sender as * const < :: windows :: core :: IInspectable as :: windows :: core :: Abi > :: Abi as * const < :: windows :: core :: IInspectable as :: windows :: core :: DefaultType > :: DefaultType ) , & * ( & e as * const < ManipulationInertiaStartingRoutedEventArgs as :: windows :: core :: Abi > :: Abi as * const < ManipulationInertiaStartingRoutedEventArgs as :: windows :: core :: DefaultType > :: DefaultType ) , ) . into ( )
    }
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct ManipulationInertiaStartingRoutedEventArgs(pub ::windows::core::IInspectable);
impl ManipulationInertiaStartingRoutedEventArgs {
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
            ManipulationInertiaStartingRoutedEventArgs,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn Container(&self) -> ::windows::core::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::UIElement>(result__)
        }
    }
    pub fn ExpansionBehavior(&self) -> ::windows::core::Result<InertiaExpansionBehavior> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InertiaExpansionBehavior>(result__)
        }
    }
    pub fn SetExpansionBehavior<
        'a,
        Param0: ::windows::core::IntoParam<'a, InertiaExpansionBehavior>,
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
    pub fn RotationBehavior(&self) -> ::windows::core::Result<InertiaRotationBehavior> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InertiaRotationBehavior>(result__)
        }
    }
    pub fn SetRotationBehavior<
        'a,
        Param0: ::windows::core::IntoParam<'a, InertiaRotationBehavior>,
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
    pub fn TranslationBehavior(&self) -> ::windows::core::Result<InertiaTranslationBehavior> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InertiaTranslationBehavior>(result__)
        }
    }
    pub fn SetTranslationBehavior<
        'a,
        Param0: ::windows::core::IntoParam<'a, InertiaTranslationBehavior>,
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
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
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
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn PointerDeviceType(
        &self,
    ) -> ::windows::core::Result<super::super::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Input::PointerDeviceType>(result__)
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn Delta(&self) -> ::windows::core::Result<super::super::Input::ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::ManipulationDelta = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Input::ManipulationDelta>(result__)
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn Cumulative(&self) -> ::windows::core::Result<super::super::Input::ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::ManipulationDelta = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Input::ManipulationDelta>(result__)
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn Velocities(
        &self,
    ) -> ::windows::core::Result<super::super::Input::ManipulationVelocities> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::ManipulationVelocities = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Input::ManipulationVelocities>(result__)
        }
    }
    pub fn OriginalSource(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ManipulationInertiaStartingRoutedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Input.ManipulationInertiaStartingRoutedEventArgs;{17d510be-5514-5952-9afd-959b60ab9394})" ) ;
}
unsafe impl ::windows::core::Interface for ManipulationInertiaStartingRoutedEventArgs {
    type Vtable = IManipulationInertiaStartingRoutedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x17d510be_5514_5952_9afd_959b60ab9394);
}
impl ::windows::core::RuntimeName for ManipulationInertiaStartingRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.ManipulationInertiaStartingRoutedEventArgs";
}
impl ::core::convert::From<ManipulationInertiaStartingRoutedEventArgs>
    for ::windows::core::IUnknown
{
    fn from(value: ManipulationInertiaStartingRoutedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ManipulationInertiaStartingRoutedEventArgs>
    for ::windows::core::IUnknown
{
    fn from(value: &ManipulationInertiaStartingRoutedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for ManipulationInertiaStartingRoutedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a ManipulationInertiaStartingRoutedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ManipulationInertiaStartingRoutedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: ManipulationInertiaStartingRoutedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ManipulationInertiaStartingRoutedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: &ManipulationInertiaStartingRoutedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for ManipulationInertiaStartingRoutedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a ManipulationInertiaStartingRoutedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ManipulationInertiaStartingRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: ManipulationInertiaStartingRoutedEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&ManipulationInertiaStartingRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &ManipulationInertiaStartingRoutedEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs>
    for ManipulationInertiaStartingRoutedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs>
    for &ManipulationInertiaStartingRoutedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for ManipulationInertiaStartingRoutedEventArgs {}
unsafe impl ::core::marker::Sync for ManipulationInertiaStartingRoutedEventArgs {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct ManipulationModes(pub u32);
impl ManipulationModes {
    pub const None: ManipulationModes = ManipulationModes(0u32);
    pub const TranslateX: ManipulationModes = ManipulationModes(1u32);
    pub const TranslateY: ManipulationModes = ManipulationModes(2u32);
    pub const TranslateRailsX: ManipulationModes = ManipulationModes(4u32);
    pub const TranslateRailsY: ManipulationModes = ManipulationModes(8u32);
    pub const Rotate: ManipulationModes = ManipulationModes(16u32);
    pub const Scale: ManipulationModes = ManipulationModes(32u32);
    pub const TranslateInertia: ManipulationModes = ManipulationModes(64u32);
    pub const RotateInertia: ManipulationModes = ManipulationModes(128u32);
    pub const ScaleInertia: ManipulationModes = ManipulationModes(256u32);
    pub const All: ManipulationModes = ManipulationModes(65535u32);
    pub const System: ManipulationModes = ManipulationModes(65536u32);
}
impl ::core::convert::From<u32> for ManipulationModes {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ManipulationModes {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ManipulationModes {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Input.ManipulationModes;u4)",
    );
}
impl ::windows::core::DefaultType for ManipulationModes {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for ManipulationModes {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for ManipulationModes {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for ManipulationModes {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for ManipulationModes {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for ManipulationModes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct ManipulationPivot(pub ::windows::core::IInspectable);
impl ManipulationPivot {
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
            ManipulationPivot,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Center(&self) -> ::windows::core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Point>(result__)
        }
    }
    pub fn SetCenter<'a, Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Point>>(
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
    pub fn Radius(&self) -> ::windows::core::Result<f64> {
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
    pub fn SetRadius(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn CreateInstanceWithCenterAndRadius<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Point>,
    >(
        center: Param0,
        radius: f64,
    ) -> ::windows::core::Result<ManipulationPivot> {
        Self::IManipulationPivotFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                center.into_param().abi(),
                radius,
                &mut result__,
            )
            .from_abi::<ManipulationPivot>(result__)
        })
    }
    pub fn IManipulationPivotFactory<
        R,
        F: FnOnce(&IManipulationPivotFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            ManipulationPivot,
            IManipulationPivotFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ManipulationPivot {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Input.ManipulationPivot;{286baba4-313d-507c-adc5-f739732cea27})",
    );
}
unsafe impl ::windows::core::Interface for ManipulationPivot {
    type Vtable = IManipulationPivot_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x286baba4_313d_507c_adc5_f739732cea27);
}
impl ::windows::core::RuntimeName for ManipulationPivot {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.ManipulationPivot";
}
impl ::core::convert::From<ManipulationPivot> for ::windows::core::IUnknown {
    fn from(value: ManipulationPivot) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ManipulationPivot> for ::windows::core::IUnknown {
    fn from(value: &ManipulationPivot) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ManipulationPivot {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ManipulationPivot {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ManipulationPivot> for ::windows::core::IInspectable {
    fn from(value: ManipulationPivot) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ManipulationPivot> for ::windows::core::IInspectable {
    fn from(value: &ManipulationPivot) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ManipulationPivot {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ManipulationPivot {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ManipulationPivot {}
unsafe impl ::core::marker::Sync for ManipulationPivot {}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct ManipulationStartedEventHandler(::windows::core::IUnknown);
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
impl ManipulationStartedEventHandler {
    pub fn new<
        F: FnMut(
                &::core::option::Option<::windows::core::IInspectable>,
                &::core::option::Option<ManipulationStartedRoutedEventArgs>,
            ) -> ::windows::core::Result<()>
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = ManipulationStartedEventHandler_box::<F> {
            vtable: &ManipulationStartedEventHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn Invoke<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param1: ::windows::core::IntoParam<'a, ManipulationStartedRoutedEventArgs>,
    >(
        &self,
        sender: Param0,
        e: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).3)(
                ::core::mem::transmute_copy(this),
                sender.into_param().abi(),
                e.into_param().abi(),
            )
            .ok()
        }
    }
}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
unsafe impl ::windows::core::RuntimeType for ManipulationStartedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"delegate({41060669-304c-53ac-9d43-bc311235aae4})",
    );
}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
unsafe impl ::windows::core::Interface for ManipulationStartedEventHandler {
    type Vtable = ManipulationStartedEventHandler_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x41060669_304c_53ac_9d43_bc311235aae4);
}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
#[repr(C)]
#[doc(hidden)]
pub struct ManipulationStartedEventHandler_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        sender: ::windows::core::RawPtr,
        e: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
#[repr(C)]
struct ManipulationStartedEventHandler_box<
    F: FnMut(
            &::core::option::Option<::windows::core::IInspectable>,
            &::core::option::Option<ManipulationStartedRoutedEventArgs>,
        ) -> ::windows::core::Result<()>
        + 'static,
> {
    vtable: *const ManipulationStartedEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
impl<
        F: FnMut(
                &::core::option::Option<::windows::core::IInspectable>,
                &::core::option::Option<ManipulationStartedRoutedEventArgs>,
            ) -> ::windows::core::Result<()>
            + 'static,
    > ManipulationStartedEventHandler_box<F>
{
    const VTABLE: ManipulationStartedEventHandler_abi = ManipulationStartedEventHandler_abi(
        Self::QueryInterface,
        Self::AddRef,
        Self::Release,
        Self::Invoke,
    );
    unsafe extern "system" fn QueryInterface(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<ManipulationStartedEventHandler as ::windows::core::Interface>::IID
            || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID
            || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::core::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: ::windows::core::RawPtr,
        sender: ::windows::core::RawPtr,
        e: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ( ( * this ) . invoke ) ( & * ( & sender as * const < :: windows :: core :: IInspectable as :: windows :: core :: Abi > :: Abi as * const < :: windows :: core :: IInspectable as :: windows :: core :: DefaultType > :: DefaultType ) , & * ( & e as * const < ManipulationStartedRoutedEventArgs as :: windows :: core :: Abi > :: Abi as * const < ManipulationStartedRoutedEventArgs as :: windows :: core :: DefaultType > :: DefaultType ) , ) . into ( )
    }
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct ManipulationStartedRoutedEventArgs(pub ::windows::core::IInspectable);
impl ManipulationStartedRoutedEventArgs {
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn Container(&self) -> ::windows::core::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::UIElement>(result__)
        }
    }
    pub fn Position(&self) -> ::windows::core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Point>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
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
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn PointerDeviceType(
        &self,
    ) -> ::windows::core::Result<super::super::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Input::PointerDeviceType>(result__)
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn Cumulative(&self) -> ::windows::core::Result<super::super::Input::ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::ManipulationDelta = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Input::ManipulationDelta>(result__)
        }
    }
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn new() -> ::windows::core::Result<ManipulationStartedRoutedEventArgs> {
        Self::IManipulationStartedRoutedEventArgsFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                ::core::ptr::null_mut(),
                &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _,
                &mut result__,
            )
            .from_abi::<ManipulationStartedRoutedEventArgs>(result__)
        })
    }
    pub fn OriginalSource(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn IManipulationStartedRoutedEventArgsFactory<
        R,
        F: FnOnce(&IManipulationStartedRoutedEventArgsFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            ManipulationStartedRoutedEventArgs,
            IManipulationStartedRoutedEventArgsFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ManipulationStartedRoutedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Input.ManipulationStartedRoutedEventArgs;{61857950-5821-5652-9fdf-c6277c5886f5})" ) ;
}
unsafe impl ::windows::core::Interface for ManipulationStartedRoutedEventArgs {
    type Vtable = IManipulationStartedRoutedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x61857950_5821_5652_9fdf_c6277c5886f5);
}
impl ::windows::core::RuntimeName for ManipulationStartedRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.ManipulationStartedRoutedEventArgs";
}
impl ::core::convert::From<ManipulationStartedRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ManipulationStartedRoutedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ManipulationStartedRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ManipulationStartedRoutedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for ManipulationStartedRoutedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a ManipulationStartedRoutedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ManipulationStartedRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ManipulationStartedRoutedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ManipulationStartedRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ManipulationStartedRoutedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for ManipulationStartedRoutedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a ManipulationStartedRoutedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ManipulationStartedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: ManipulationStartedRoutedEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&ManipulationStartedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &ManipulationStartedRoutedEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs>
    for ManipulationStartedRoutedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs>
    for &ManipulationStartedRoutedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for ManipulationStartedRoutedEventArgs {}
unsafe impl ::core::marker::Sync for ManipulationStartedRoutedEventArgs {}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct ManipulationStartingEventHandler(::windows::core::IUnknown);
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
impl ManipulationStartingEventHandler {
    pub fn new<
        F: FnMut(
                &::core::option::Option<::windows::core::IInspectable>,
                &::core::option::Option<ManipulationStartingRoutedEventArgs>,
            ) -> ::windows::core::Result<()>
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = ManipulationStartingEventHandler_box::<F> {
            vtable: &ManipulationStartingEventHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn Invoke<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param1: ::windows::core::IntoParam<'a, ManipulationStartingRoutedEventArgs>,
    >(
        &self,
        sender: Param0,
        e: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).3)(
                ::core::mem::transmute_copy(this),
                sender.into_param().abi(),
                e.into_param().abi(),
            )
            .ok()
        }
    }
}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
unsafe impl ::windows::core::RuntimeType for ManipulationStartingEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"delegate({44f528f1-f0e4-505c-a0bb-0c4839b29df5})",
    );
}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
unsafe impl ::windows::core::Interface for ManipulationStartingEventHandler {
    type Vtable = ManipulationStartingEventHandler_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x44f528f1_f0e4_505c_a0bb_0c4839b29df5);
}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
#[repr(C)]
#[doc(hidden)]
pub struct ManipulationStartingEventHandler_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        sender: ::windows::core::RawPtr,
        e: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
#[repr(C)]
struct ManipulationStartingEventHandler_box<
    F: FnMut(
            &::core::option::Option<::windows::core::IInspectable>,
            &::core::option::Option<ManipulationStartingRoutedEventArgs>,
        ) -> ::windows::core::Result<()>
        + 'static,
> {
    vtable: *const ManipulationStartingEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
impl<
        F: FnMut(
                &::core::option::Option<::windows::core::IInspectable>,
                &::core::option::Option<ManipulationStartingRoutedEventArgs>,
            ) -> ::windows::core::Result<()>
            + 'static,
    > ManipulationStartingEventHandler_box<F>
{
    const VTABLE: ManipulationStartingEventHandler_abi = ManipulationStartingEventHandler_abi(
        Self::QueryInterface,
        Self::AddRef,
        Self::Release,
        Self::Invoke,
    );
    unsafe extern "system" fn QueryInterface(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid
            == &<ManipulationStartingEventHandler as ::windows::core::Interface>::IID
            || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID
            || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::core::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: ::windows::core::RawPtr,
        sender: ::windows::core::RawPtr,
        e: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ( ( * this ) . invoke ) ( & * ( & sender as * const < :: windows :: core :: IInspectable as :: windows :: core :: Abi > :: Abi as * const < :: windows :: core :: IInspectable as :: windows :: core :: DefaultType > :: DefaultType ) , & * ( & e as * const < ManipulationStartingRoutedEventArgs as :: windows :: core :: Abi > :: Abi as * const < ManipulationStartingRoutedEventArgs as :: windows :: core :: DefaultType > :: DefaultType ) , ) . into ( )
    }
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct ManipulationStartingRoutedEventArgs(pub ::windows::core::IInspectable);
impl ManipulationStartingRoutedEventArgs {
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
            ManipulationStartingRoutedEventArgs,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Mode(&self) -> ::windows::core::Result<ManipulationModes> {
        let this = self;
        unsafe {
            let mut result__: ManipulationModes = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ManipulationModes>(result__)
        }
    }
    pub fn SetMode(&self, value: ManipulationModes) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn Container(&self) -> ::windows::core::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::UIElement>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn SetContainer<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(
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
    pub fn Pivot(&self) -> ::windows::core::Result<ManipulationPivot> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ManipulationPivot>(result__)
        }
    }
    pub fn SetPivot<'a, Param0: ::windows::core::IntoParam<'a, ManipulationPivot>>(
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
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
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
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn OriginalSource(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ManipulationStartingRoutedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Input.ManipulationStartingRoutedEventArgs;{93a99f86-f5a0-5326-91b0-851c897af79f})" ) ;
}
unsafe impl ::windows::core::Interface for ManipulationStartingRoutedEventArgs {
    type Vtable = IManipulationStartingRoutedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x93a99f86_f5a0_5326_91b0_851c897af79f);
}
impl ::windows::core::RuntimeName for ManipulationStartingRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.ManipulationStartingRoutedEventArgs";
}
impl ::core::convert::From<ManipulationStartingRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ManipulationStartingRoutedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ManipulationStartingRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ManipulationStartingRoutedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for ManipulationStartingRoutedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a ManipulationStartingRoutedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ManipulationStartingRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ManipulationStartingRoutedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ManipulationStartingRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ManipulationStartingRoutedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for ManipulationStartingRoutedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a ManipulationStartingRoutedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ManipulationStartingRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: ManipulationStartingRoutedEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&ManipulationStartingRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &ManipulationStartingRoutedEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs>
    for ManipulationStartingRoutedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs>
    for &ManipulationStartingRoutedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for ManipulationStartingRoutedEventArgs {}
unsafe impl ::core::marker::Sync for ManipulationStartingRoutedEventArgs {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct NoFocusCandidateFoundEventArgs(pub ::windows::core::IInspectable);
impl NoFocusCandidateFoundEventArgs {
    pub fn Direction(&self) -> ::windows::core::Result<FocusNavigationDirection> {
        let this = self;
        unsafe {
            let mut result__: FocusNavigationDirection = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<FocusNavigationDirection>(result__)
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
    pub fn InputDevice(&self) -> ::windows::core::Result<FocusInputDeviceKind> {
        let this = self;
        unsafe {
            let mut result__: FocusInputDeviceKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<FocusInputDeviceKind>(result__)
        }
    }
    pub fn OriginalSource(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for NoFocusCandidateFoundEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Input.NoFocusCandidateFoundEventArgs;{a2d7153a-cd2a-59cb-a574-ac82e30b9201})" ) ;
}
unsafe impl ::windows::core::Interface for NoFocusCandidateFoundEventArgs {
    type Vtable = INoFocusCandidateFoundEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa2d7153a_cd2a_59cb_a574_ac82e30b9201);
}
impl ::windows::core::RuntimeName for NoFocusCandidateFoundEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.NoFocusCandidateFoundEventArgs";
}
impl ::core::convert::From<NoFocusCandidateFoundEventArgs> for ::windows::core::IUnknown {
    fn from(value: NoFocusCandidateFoundEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&NoFocusCandidateFoundEventArgs> for ::windows::core::IUnknown {
    fn from(value: &NoFocusCandidateFoundEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for NoFocusCandidateFoundEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a NoFocusCandidateFoundEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<NoFocusCandidateFoundEventArgs> for ::windows::core::IInspectable {
    fn from(value: NoFocusCandidateFoundEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&NoFocusCandidateFoundEventArgs> for ::windows::core::IInspectable {
    fn from(value: &NoFocusCandidateFoundEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for NoFocusCandidateFoundEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a NoFocusCandidateFoundEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<NoFocusCandidateFoundEventArgs> for super::RoutedEventArgs {
    fn from(value: NoFocusCandidateFoundEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&NoFocusCandidateFoundEventArgs> for super::RoutedEventArgs {
    fn from(value: &NoFocusCandidateFoundEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for NoFocusCandidateFoundEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs>
    for &NoFocusCandidateFoundEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for NoFocusCandidateFoundEventArgs {}
unsafe impl ::core::marker::Sync for NoFocusCandidateFoundEventArgs {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct Pointer(pub ::windows::core::IInspectable);
impl Pointer {
    pub fn PointerId(&self) -> ::windows::core::Result<u32> {
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
    #[cfg(feature = "UI_Input")]
    pub fn PointerDeviceType(
        &self,
    ) -> ::windows::core::Result<super::super::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Input::PointerDeviceType>(result__)
        }
    }
    pub fn IsInContact(&self) -> ::windows::core::Result<bool> {
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
    pub fn IsInRange(&self) -> ::windows::core::Result<bool> {
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
unsafe impl ::windows::core::RuntimeType for Pointer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Input.Pointer;{1f9afbf5-11a3-5e68-aa1b-72febfa0ab23})",
    );
}
unsafe impl ::windows::core::Interface for Pointer {
    type Vtable = IPointer_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x1f9afbf5_11a3_5e68_aa1b_72febfa0ab23);
}
impl ::windows::core::RuntimeName for Pointer {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.Pointer";
}
impl ::core::convert::From<Pointer> for ::windows::core::IUnknown {
    fn from(value: Pointer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Pointer> for ::windows::core::IUnknown {
    fn from(value: &Pointer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Pointer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Pointer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Pointer> for ::windows::core::IInspectable {
    fn from(value: Pointer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Pointer> for ::windows::core::IInspectable {
    fn from(value: &Pointer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Pointer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Pointer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for Pointer {}
unsafe impl ::core::marker::Sync for Pointer {}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct PointerEventHandler(::windows::core::IUnknown);
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
impl PointerEventHandler {
    pub fn new<
        F: FnMut(
                &::core::option::Option<::windows::core::IInspectable>,
                &::core::option::Option<PointerRoutedEventArgs>,
            ) -> ::windows::core::Result<()>
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = PointerEventHandler_box::<F> {
            vtable: &PointerEventHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn Invoke<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param1: ::windows::core::IntoParam<'a, PointerRoutedEventArgs>,
    >(
        &self,
        sender: Param0,
        e: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).3)(
                ::core::mem::transmute_copy(this),
                sender.into_param().abi(),
                e.into_param().abi(),
            )
            .ok()
        }
    }
}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
unsafe impl ::windows::core::RuntimeType for PointerEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"delegate({a48a71e1-8bb4-5597-9e31-903a3f6a04fb})",
    );
}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
unsafe impl ::windows::core::Interface for PointerEventHandler {
    type Vtable = PointerEventHandler_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa48a71e1_8bb4_5597_9e31_903a3f6a04fb);
}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
#[repr(C)]
#[doc(hidden)]
pub struct PointerEventHandler_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        sender: ::windows::core::RawPtr,
        e: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
#[repr(C)]
struct PointerEventHandler_box<
    F: FnMut(
            &::core::option::Option<::windows::core::IInspectable>,
            &::core::option::Option<PointerRoutedEventArgs>,
        ) -> ::windows::core::Result<()>
        + 'static,
> {
    vtable: *const PointerEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
impl<
        F: FnMut(
                &::core::option::Option<::windows::core::IInspectable>,
                &::core::option::Option<PointerRoutedEventArgs>,
            ) -> ::windows::core::Result<()>
            + 'static,
    > PointerEventHandler_box<F>
{
    const VTABLE: PointerEventHandler_abi = PointerEventHandler_abi(
        Self::QueryInterface,
        Self::AddRef,
        Self::Release,
        Self::Invoke,
    );
    unsafe extern "system" fn QueryInterface(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<PointerEventHandler as ::windows::core::Interface>::IID
            || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID
            || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::core::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: ::windows::core::RawPtr,
        sender: ::windows::core::RawPtr,
        e: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ( ( * this ) . invoke ) ( & * ( & sender as * const < :: windows :: core :: IInspectable as :: windows :: core :: Abi > :: Abi as * const < :: windows :: core :: IInspectable as :: windows :: core :: DefaultType > :: DefaultType ) , & * ( & e as * const < PointerRoutedEventArgs as :: windows :: core :: Abi > :: Abi as * const < PointerRoutedEventArgs as :: windows :: core :: DefaultType > :: DefaultType ) , ) . into ( )
    }
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct PointerRoutedEventArgs(pub ::windows::core::IInspectable);
impl PointerRoutedEventArgs {
    #[cfg(feature = "UI_Input")]
    pub fn Pointer(&self) -> ::windows::core::Result<Pointer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<Pointer>(result__)
        }
    }
    pub fn KeyModifiers(&self) -> ::windows::core::Result<::windows::System::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__: ::windows::System::VirtualKeyModifiers = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::System::VirtualKeyModifiers>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
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
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsGenerated(&self) -> ::windows::core::Result<bool> {
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
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn GetCurrentPoint<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(
        &self,
        relativeto: Param0,
    ) -> ::windows::core::Result<super::super::Input::PointerPoint> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                relativeto.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Input::PointerPoint>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn GetIntermediatePoints<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(
        &self,
        relativeto: Param0,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IVector<super::super::Input::PointerPoint>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                relativeto.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVector<
                super::super::Input::PointerPoint,
            >>(result__)
        }
    }
    pub fn OriginalSource(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PointerRoutedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Input.PointerRoutedEventArgs;{66e78a9a-1bec-5f92-b1a1-ea6334ee511c})" ) ;
}
unsafe impl ::windows::core::Interface for PointerRoutedEventArgs {
    type Vtable = IPointerRoutedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x66e78a9a_1bec_5f92_b1a1_ea6334ee511c);
}
impl ::windows::core::RuntimeName for PointerRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.PointerRoutedEventArgs";
}
impl ::core::convert::From<PointerRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PointerRoutedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PointerRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PointerRoutedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PointerRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PointerRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PointerRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PointerRoutedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PointerRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PointerRoutedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PointerRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a PointerRoutedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PointerRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: PointerRoutedEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&PointerRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &PointerRoutedEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for PointerRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &PointerRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for PointerRoutedEventArgs {}
unsafe impl ::core::marker::Sync for PointerRoutedEventArgs {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct ProcessKeyboardAcceleratorEventArgs(pub ::windows::core::IInspectable);
impl ProcessKeyboardAcceleratorEventArgs {
    pub fn Key(&self) -> ::windows::core::Result<::windows::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__: ::windows::System::VirtualKey = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::System::VirtualKey>(result__)
        }
    }
    pub fn Modifiers(&self) -> ::windows::core::Result<::windows::System::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__: ::windows::System::VirtualKeyModifiers = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::System::VirtualKeyModifiers>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
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
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ProcessKeyboardAcceleratorEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Input.ProcessKeyboardAcceleratorEventArgs;{9be0d058-3d26-5811-b50a-3bb80ca766c9})" ) ;
}
unsafe impl ::windows::core::Interface for ProcessKeyboardAcceleratorEventArgs {
    type Vtable = IProcessKeyboardAcceleratorEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x9be0d058_3d26_5811_b50a_3bb80ca766c9);
}
impl ::windows::core::RuntimeName for ProcessKeyboardAcceleratorEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.ProcessKeyboardAcceleratorEventArgs";
}
impl ::core::convert::From<ProcessKeyboardAcceleratorEventArgs> for ::windows::core::IUnknown {
    fn from(value: ProcessKeyboardAcceleratorEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ProcessKeyboardAcceleratorEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ProcessKeyboardAcceleratorEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for ProcessKeyboardAcceleratorEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a ProcessKeyboardAcceleratorEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ProcessKeyboardAcceleratorEventArgs> for ::windows::core::IInspectable {
    fn from(value: ProcessKeyboardAcceleratorEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ProcessKeyboardAcceleratorEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ProcessKeyboardAcceleratorEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for ProcessKeyboardAcceleratorEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a ProcessKeyboardAcceleratorEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ProcessKeyboardAcceleratorEventArgs {}
unsafe impl ::core::marker::Sync for ProcessKeyboardAcceleratorEventArgs {}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct RightTappedEventHandler(::windows::core::IUnknown);
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
impl RightTappedEventHandler {
    pub fn new<
        F: FnMut(
                &::core::option::Option<::windows::core::IInspectable>,
                &::core::option::Option<RightTappedRoutedEventArgs>,
            ) -> ::windows::core::Result<()>
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = RightTappedEventHandler_box::<F> {
            vtable: &RightTappedEventHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn Invoke<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param1: ::windows::core::IntoParam<'a, RightTappedRoutedEventArgs>,
    >(
        &self,
        sender: Param0,
        e: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).3)(
                ::core::mem::transmute_copy(this),
                sender.into_param().abi(),
                e.into_param().abi(),
            )
            .ok()
        }
    }
}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
unsafe impl ::windows::core::RuntimeType for RightTappedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"delegate({5070e32f-3dc7-56cf-8fdd-de1b40d0b472})",
    );
}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
unsafe impl ::windows::core::Interface for RightTappedEventHandler {
    type Vtable = RightTappedEventHandler_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x5070e32f_3dc7_56cf_8fdd_de1b40d0b472);
}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
#[repr(C)]
#[doc(hidden)]
pub struct RightTappedEventHandler_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        sender: ::windows::core::RawPtr,
        e: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
#[repr(C)]
struct RightTappedEventHandler_box<
    F: FnMut(
            &::core::option::Option<::windows::core::IInspectable>,
            &::core::option::Option<RightTappedRoutedEventArgs>,
        ) -> ::windows::core::Result<()>
        + 'static,
> {
    vtable: *const RightTappedEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
impl<
        F: FnMut(
                &::core::option::Option<::windows::core::IInspectable>,
                &::core::option::Option<RightTappedRoutedEventArgs>,
            ) -> ::windows::core::Result<()>
            + 'static,
    > RightTappedEventHandler_box<F>
{
    const VTABLE: RightTappedEventHandler_abi = RightTappedEventHandler_abi(
        Self::QueryInterface,
        Self::AddRef,
        Self::Release,
        Self::Invoke,
    );
    unsafe extern "system" fn QueryInterface(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<RightTappedEventHandler as ::windows::core::Interface>::IID
            || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID
            || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::core::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: ::windows::core::RawPtr,
        sender: ::windows::core::RawPtr,
        e: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ( ( * this ) . invoke ) ( & * ( & sender as * const < :: windows :: core :: IInspectable as :: windows :: core :: Abi > :: Abi as * const < :: windows :: core :: IInspectable as :: windows :: core :: DefaultType > :: DefaultType ) , & * ( & e as * const < RightTappedRoutedEventArgs as :: windows :: core :: Abi > :: Abi as * const < RightTappedRoutedEventArgs as :: windows :: core :: DefaultType > :: DefaultType ) , ) . into ( )
    }
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct RightTappedRoutedEventArgs(pub ::windows::core::IInspectable);
impl RightTappedRoutedEventArgs {
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
            RightTappedRoutedEventArgs,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "UI_Input")]
    pub fn PointerDeviceType(
        &self,
    ) -> ::windows::core::Result<super::super::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Input::PointerDeviceType>(result__)
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
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn GetPosition<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(
        &self,
        relativeto: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                relativeto.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Point>(result__)
        }
    }
    pub fn OriginalSource(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for RightTappedRoutedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Input.RightTappedRoutedEventArgs;{3972fafb-2915-5c62-bb6b-54ad84ff400d})" ) ;
}
unsafe impl ::windows::core::Interface for RightTappedRoutedEventArgs {
    type Vtable = IRightTappedRoutedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x3972fafb_2915_5c62_bb6b_54ad84ff400d);
}
impl ::windows::core::RuntimeName for RightTappedRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.RightTappedRoutedEventArgs";
}
impl ::core::convert::From<RightTappedRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: RightTappedRoutedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RightTappedRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &RightTappedRoutedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RightTappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a RightTappedRoutedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RightTappedRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: RightTappedRoutedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RightTappedRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &RightTappedRoutedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for RightTappedRoutedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a RightTappedRoutedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<RightTappedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: RightTappedRoutedEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&RightTappedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &RightTappedRoutedEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for RightTappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &RightTappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for RightTappedRoutedEventArgs {}
unsafe impl ::core::marker::Sync for RightTappedRoutedEventArgs {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct StandardUICommand(pub ::windows::core::IInspectable);
impl StandardUICommand {
    pub fn Kind(&self) -> ::windows::core::Result<StandardUICommandKind> {
        let this = self;
        unsafe {
            let mut result__: StandardUICommandKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<StandardUICommandKind>(result__)
        }
    }
    pub fn SetKind(&self, value: StandardUICommandKind) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn KindProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IStandardUICommandStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn new() -> ::windows::core::Result<StandardUICommand> {
        Self::IStandardUICommandFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                ::core::ptr::null_mut(),
                &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _,
                &mut result__,
            )
            .from_abi::<StandardUICommand>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn CreateInstanceWithKind(
        kind: StandardUICommandKind,
    ) -> ::windows::core::Result<StandardUICommand> {
        Self::IStandardUICommandFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                kind,
                ::core::ptr::null_mut(),
                &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _,
                &mut result__,
            )
            .from_abi::<StandardUICommand>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ClearValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ReadLocalValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn CanExecuteChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::EventHandler<::windows::core::IInspectable>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICommand>(self)?;
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
    pub fn RemoveCanExecuteChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommand>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn CanExecute<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(
        &self,
        parameter: Param0,
    ) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommand>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                parameter.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn Execute<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(
        &self,
        parameter: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommand>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                parameter.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXamlUICommand>(self)?;
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
    pub fn SetLabel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXamlUICommand>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Media"
    ))]
    pub fn IconSource(&self) -> ::windows::core::Result<super::Controls::IconSource> {
        let this = &::windows::core::Interface::cast::<IXamlUICommand>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Controls::IconSource>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Media"
    ))]
    pub fn SetIconSource<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Controls::IconSource>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXamlUICommand>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn KeyboardAccelerators(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Collections::IVector<KeyboardAccelerator>>
    {
        let this = &::windows::core::Interface::cast::<IXamlUICommand>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVector<KeyboardAccelerator>>(result__)
        }
    }
    pub fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXamlUICommand>(self)?;
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
    pub fn SetAccessKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXamlUICommand>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXamlUICommand>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXamlUICommand>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Command(&self) -> ::windows::core::Result<ICommand> {
        let this = &::windows::core::Interface::cast::<IXamlUICommand>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ICommand>(result__)
        }
    }
    pub fn SetCommand<'a, Param0: ::windows::core::IntoParam<'a, ICommand>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXamlUICommand>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ExecuteRequested<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<XamlUICommand, ExecuteRequestedEventArgs>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IXamlUICommand>(self)?;
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
    pub fn RemoveExecuteRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXamlUICommand>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Media"
    ))]
    pub fn CanExecuteRequested<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<XamlUICommand, CanExecuteRequestedEventArgs>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IXamlUICommand>(self)?;
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
    pub fn RemoveCanExecuteRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXamlUICommand>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn NotifyCanExecuteChanged(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXamlUICommand>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn IStandardUICommandStatics<
        R,
        F: FnOnce(&IStandardUICommandStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            StandardUICommand,
            IStandardUICommandStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IStandardUICommandFactory<
        R,
        F: FnOnce(&IStandardUICommandFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            StandardUICommand,
            IStandardUICommandFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for StandardUICommand {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Input.StandardUICommand;{5f395d50-5449-59ab-9cb2-4e3700033f03})",
    );
}
unsafe impl ::windows::core::Interface for StandardUICommand {
    type Vtable = IStandardUICommand_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x5f395d50_5449_59ab_9cb2_4e3700033f03);
}
impl ::windows::core::RuntimeName for StandardUICommand {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.StandardUICommand";
}
impl ::core::convert::From<StandardUICommand> for ::windows::core::IUnknown {
    fn from(value: StandardUICommand) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&StandardUICommand> for ::windows::core::IUnknown {
    fn from(value: &StandardUICommand) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StandardUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StandardUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<StandardUICommand> for ::windows::core::IInspectable {
    fn from(value: StandardUICommand) -> Self {
        value.0
    }
}
impl ::core::convert::From<&StandardUICommand> for ::windows::core::IInspectable {
    fn from(value: &StandardUICommand) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StandardUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StandardUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<StandardUICommand> for ICommand {
    type Error = ::windows::core::Error;
    fn try_from(value: StandardUICommand) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StandardUICommand> for ICommand {
    type Error = ::windows::core::Error;
    fn try_from(value: &StandardUICommand) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICommand> for StandardUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ICommand> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICommand> for &StandardUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ICommand> {
        ::core::convert::TryInto::<ICommand>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<StandardUICommand> for XamlUICommand {
    fn from(value: StandardUICommand) -> Self {
        ::core::convert::Into::<XamlUICommand>::into(&value)
    }
}
impl ::core::convert::From<&StandardUICommand> for XamlUICommand {
    fn from(value: &StandardUICommand) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, XamlUICommand> for StandardUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, XamlUICommand> {
        ::windows::core::Param::Owned(::core::convert::Into::<XamlUICommand>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, XamlUICommand> for &StandardUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, XamlUICommand> {
        ::windows::core::Param::Owned(::core::convert::Into::<XamlUICommand>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<StandardUICommand> for super::DependencyObject {
    fn from(value: StandardUICommand) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&StandardUICommand> for super::DependencyObject {
    fn from(value: &StandardUICommand) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for StandardUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &StandardUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for StandardUICommand {}
unsafe impl ::core::marker::Sync for StandardUICommand {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct StandardUICommandKind(pub i32);
impl StandardUICommandKind {
    pub const None: StandardUICommandKind = StandardUICommandKind(0i32);
    pub const Cut: StandardUICommandKind = StandardUICommandKind(1i32);
    pub const Copy: StandardUICommandKind = StandardUICommandKind(2i32);
    pub const Paste: StandardUICommandKind = StandardUICommandKind(3i32);
    pub const SelectAll: StandardUICommandKind = StandardUICommandKind(4i32);
    pub const Delete: StandardUICommandKind = StandardUICommandKind(5i32);
    pub const Share: StandardUICommandKind = StandardUICommandKind(6i32);
    pub const Save: StandardUICommandKind = StandardUICommandKind(7i32);
    pub const Open: StandardUICommandKind = StandardUICommandKind(8i32);
    pub const Close: StandardUICommandKind = StandardUICommandKind(9i32);
    pub const Pause: StandardUICommandKind = StandardUICommandKind(10i32);
    pub const Play: StandardUICommandKind = StandardUICommandKind(11i32);
    pub const Stop: StandardUICommandKind = StandardUICommandKind(12i32);
    pub const Forward: StandardUICommandKind = StandardUICommandKind(13i32);
    pub const Backward: StandardUICommandKind = StandardUICommandKind(14i32);
    pub const Undo: StandardUICommandKind = StandardUICommandKind(15i32);
    pub const Redo: StandardUICommandKind = StandardUICommandKind(16i32);
}
impl ::core::convert::From<i32> for StandardUICommandKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for StandardUICommandKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for StandardUICommandKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Input.StandardUICommandKind;i4)",
    );
}
impl ::windows::core::DefaultType for StandardUICommandKind {
    type DefaultType = Self;
}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct TappedEventHandler(::windows::core::IUnknown);
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
impl TappedEventHandler {
    pub fn new<
        F: FnMut(
                &::core::option::Option<::windows::core::IInspectable>,
                &::core::option::Option<TappedRoutedEventArgs>,
            ) -> ::windows::core::Result<()>
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = TappedEventHandler_box::<F> {
            vtable: &TappedEventHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn Invoke<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param1: ::windows::core::IntoParam<'a, TappedRoutedEventArgs>,
    >(
        &self,
        sender: Param0,
        e: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).3)(
                ::core::mem::transmute_copy(this),
                sender.into_param().abi(),
                e.into_param().abi(),
            )
            .ok()
        }
    }
}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
unsafe impl ::windows::core::RuntimeType for TappedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"delegate({b60074f3-125b-534e-8f9c-9769bd3f0f64})",
    );
}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
unsafe impl ::windows::core::Interface for TappedEventHandler {
    type Vtable = TappedEventHandler_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xb60074f3_125b_534e_8f9c_9769bd3f0f64);
}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
#[repr(C)]
#[doc(hidden)]
pub struct TappedEventHandler_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        sender: ::windows::core::RawPtr,
        e: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
#[repr(C)]
struct TappedEventHandler_box<
    F: FnMut(
            &::core::option::Option<::windows::core::IInspectable>,
            &::core::option::Option<TappedRoutedEventArgs>,
        ) -> ::windows::core::Result<()>
        + 'static,
> {
    vtable: *const TappedEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
#[cfg(all(
    feature = "UI_Composition",
    feature = "UI_Dispatching",
    feature = "UI_Input",
    feature = "UI_Xaml_Automation",
    feature = "UI_Xaml_Automation_Peers",
    feature = "UI_Xaml_Automation_Provider",
    feature = "UI_Xaml_Controls",
    feature = "UI_Xaml_Controls_Primitives",
    feature = "UI_Xaml_Data",
    feature = "UI_Xaml_Media",
    feature = "UI_Xaml_Media_Animation",
    feature = "UI_Xaml_Media_Imaging",
    feature = "UI_Xaml_Media_Media3D"
))]
impl<
        F: FnMut(
                &::core::option::Option<::windows::core::IInspectable>,
                &::core::option::Option<TappedRoutedEventArgs>,
            ) -> ::windows::core::Result<()>
            + 'static,
    > TappedEventHandler_box<F>
{
    const VTABLE: TappedEventHandler_abi = TappedEventHandler_abi(
        Self::QueryInterface,
        Self::AddRef,
        Self::Release,
        Self::Invoke,
    );
    unsafe extern "system" fn QueryInterface(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<TappedEventHandler as ::windows::core::Interface>::IID
            || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID
            || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::core::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: ::windows::core::RawPtr,
        sender: ::windows::core::RawPtr,
        e: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ( ( * this ) . invoke ) ( & * ( & sender as * const < :: windows :: core :: IInspectable as :: windows :: core :: Abi > :: Abi as * const < :: windows :: core :: IInspectable as :: windows :: core :: DefaultType > :: DefaultType ) , & * ( & e as * const < TappedRoutedEventArgs as :: windows :: core :: Abi > :: Abi as * const < TappedRoutedEventArgs as :: windows :: core :: DefaultType > :: DefaultType ) , ) . into ( )
    }
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct TappedRoutedEventArgs(pub ::windows::core::IInspectable);
impl TappedRoutedEventArgs {
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
            TappedRoutedEventArgs,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "UI_Input")]
    pub fn PointerDeviceType(
        &self,
    ) -> ::windows::core::Result<super::super::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Input::PointerDeviceType>(result__)
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
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn GetPosition<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(
        &self,
        relativeto: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                relativeto.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Point>(result__)
        }
    }
    pub fn OriginalSource(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for TappedRoutedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Input.TappedRoutedEventArgs;{73f74b8c-3709-547e-8e0c-51c03c89126a})",
    );
}
unsafe impl ::windows::core::Interface for TappedRoutedEventArgs {
    type Vtable = ITappedRoutedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x73f74b8c_3709_547e_8e0c_51c03c89126a);
}
impl ::windows::core::RuntimeName for TappedRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.TappedRoutedEventArgs";
}
impl ::core::convert::From<TappedRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: TappedRoutedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TappedRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &TappedRoutedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TappedRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: TappedRoutedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TappedRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &TappedRoutedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a TappedRoutedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<TappedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: TappedRoutedEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&TappedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &TappedRoutedEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for TappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &TappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for TappedRoutedEventArgs {}
unsafe impl ::core::marker::Sync for TappedRoutedEventArgs {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct XYFocusKeyboardNavigationMode(pub i32);
impl XYFocusKeyboardNavigationMode {
    pub const Auto: XYFocusKeyboardNavigationMode = XYFocusKeyboardNavigationMode(0i32);
    pub const Enabled: XYFocusKeyboardNavigationMode = XYFocusKeyboardNavigationMode(1i32);
    pub const Disabled: XYFocusKeyboardNavigationMode = XYFocusKeyboardNavigationMode(2i32);
}
impl ::core::convert::From<i32> for XYFocusKeyboardNavigationMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for XYFocusKeyboardNavigationMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for XYFocusKeyboardNavigationMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Input.XYFocusKeyboardNavigationMode;i4)",
    );
}
impl ::windows::core::DefaultType for XYFocusKeyboardNavigationMode {
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
pub struct XYFocusNavigationStrategy(pub i32);
impl XYFocusNavigationStrategy {
    pub const Auto: XYFocusNavigationStrategy = XYFocusNavigationStrategy(0i32);
    pub const Projection: XYFocusNavigationStrategy = XYFocusNavigationStrategy(1i32);
    pub const NavigationDirectionDistance: XYFocusNavigationStrategy =
        XYFocusNavigationStrategy(2i32);
    pub const RectilinearDistance: XYFocusNavigationStrategy = XYFocusNavigationStrategy(3i32);
}
impl ::core::convert::From<i32> for XYFocusNavigationStrategy {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for XYFocusNavigationStrategy {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for XYFocusNavigationStrategy {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Input.XYFocusNavigationStrategy;i4)",
    );
}
impl ::windows::core::DefaultType for XYFocusNavigationStrategy {
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
pub struct XYFocusNavigationStrategyOverride(pub i32);
impl XYFocusNavigationStrategyOverride {
    pub const None: XYFocusNavigationStrategyOverride = XYFocusNavigationStrategyOverride(0i32);
    pub const Auto: XYFocusNavigationStrategyOverride = XYFocusNavigationStrategyOverride(1i32);
    pub const Projection: XYFocusNavigationStrategyOverride =
        XYFocusNavigationStrategyOverride(2i32);
    pub const NavigationDirectionDistance: XYFocusNavigationStrategyOverride =
        XYFocusNavigationStrategyOverride(3i32);
    pub const RectilinearDistance: XYFocusNavigationStrategyOverride =
        XYFocusNavigationStrategyOverride(4i32);
}
impl ::core::convert::From<i32> for XYFocusNavigationStrategyOverride {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for XYFocusNavigationStrategyOverride {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for XYFocusNavigationStrategyOverride {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Input.XYFocusNavigationStrategyOverride;i4)",
    );
}
impl ::windows::core::DefaultType for XYFocusNavigationStrategyOverride {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct XamlUICommand(pub ::windows::core::IInspectable);
impl XamlUICommand {
    pub fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
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
    pub fn SetLabel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
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
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Media"
    ))]
    pub fn IconSource(&self) -> ::windows::core::Result<super::Controls::IconSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Controls::IconSource>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Media"
    ))]
    pub fn SetIconSource<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Controls::IconSource>,
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
    #[cfg(feature = "UI_Dispatching")]
    pub fn KeyboardAccelerators(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Collections::IVector<KeyboardAccelerator>>
    {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVector<KeyboardAccelerator>>(result__)
        }
    }
    pub fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
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
    pub fn SetAccessKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
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
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Command(&self) -> ::windows::core::Result<ICommand> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ICommand>(result__)
        }
    }
    pub fn SetCommand<'a, Param0: ::windows::core::IntoParam<'a, ICommand>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ExecuteRequested<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<XamlUICommand, ExecuteRequestedEventArgs>,
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
    pub fn RemoveExecuteRequested<
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
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Media"
    ))]
    pub fn CanExecuteRequested<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<XamlUICommand, CanExecuteRequestedEventArgs>,
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
    pub fn RemoveCanExecuteRequested<
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
    pub fn NotifyCanExecuteChanged(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn CanExecuteChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::EventHandler<::windows::core::IInspectable>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICommand>(self)?;
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
    pub fn RemoveCanExecuteChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommand>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn CanExecute<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(
        &self,
        parameter: Param0,
    ) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommand>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                parameter.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn Execute<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(
        &self,
        parameter: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommand>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                parameter.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn LabelProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IXamlUICommandStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn IconSourceProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IXamlUICommandStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn KeyboardAcceleratorsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IXamlUICommandStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn AccessKeyProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IXamlUICommandStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DescriptionProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IXamlUICommandStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn CommandProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IXamlUICommandStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Media"
    ))]
    pub fn new() -> ::windows::core::Result<XamlUICommand> {
        Self::IXamlUICommandFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                ::core::ptr::null_mut(),
                &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _,
                &mut result__,
            )
            .from_abi::<XamlUICommand>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ClearValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ReadLocalValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn IXamlUICommandStatics<
        R,
        F: FnOnce(&IXamlUICommandStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<XamlUICommand, IXamlUICommandStatics> =
            ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IXamlUICommandFactory<
        R,
        F: FnOnce(&IXamlUICommandFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<XamlUICommand, IXamlUICommandFactory> =
            ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for XamlUICommand {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Input.XamlUICommand;{a457f2cb-51e0-541c-9c42-dd1dcbdf58fb})",
    );
}
unsafe impl ::windows::core::Interface for XamlUICommand {
    type Vtable = IXamlUICommand_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa457f2cb_51e0_541c_9c42_dd1dcbdf58fb);
}
impl ::windows::core::RuntimeName for XamlUICommand {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.XamlUICommand";
}
impl ::core::convert::From<XamlUICommand> for ::windows::core::IUnknown {
    fn from(value: XamlUICommand) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XamlUICommand> for ::windows::core::IUnknown {
    fn from(value: &XamlUICommand) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for XamlUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a XamlUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XamlUICommand> for ::windows::core::IInspectable {
    fn from(value: XamlUICommand) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XamlUICommand> for ::windows::core::IInspectable {
    fn from(value: &XamlUICommand) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for XamlUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a XamlUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<XamlUICommand> for ICommand {
    type Error = ::windows::core::Error;
    fn try_from(value: XamlUICommand) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XamlUICommand> for ICommand {
    type Error = ::windows::core::Error;
    fn try_from(value: &XamlUICommand) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICommand> for XamlUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ICommand> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICommand> for &XamlUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ICommand> {
        ::core::convert::TryInto::<ICommand>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<XamlUICommand> for super::DependencyObject {
    fn from(value: XamlUICommand) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&XamlUICommand> for super::DependencyObject {
    fn from(value: &XamlUICommand) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for XamlUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &XamlUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for XamlUICommand {}
unsafe impl ::core::marker::Sync for XamlUICommand {}
