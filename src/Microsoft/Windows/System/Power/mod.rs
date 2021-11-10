#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct BatteryStatus(pub i32);
impl BatteryStatus {
    pub const NotPresent: BatteryStatus = BatteryStatus(0i32);
    pub const Discharging: BatteryStatus = BatteryStatus(1i32);
    pub const Idle: BatteryStatus = BatteryStatus(2i32);
    pub const Charging: BatteryStatus = BatteryStatus(3i32);
}
impl ::core::convert::From<i32> for BatteryStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BatteryStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for BatteryStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.System.Power.BatteryStatus;i4)",
    );
}
impl ::windows::core::DefaultType for BatteryStatus {
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
pub struct DisplayStatus(pub i32);
impl DisplayStatus {
    pub const Off: DisplayStatus = DisplayStatus(0i32);
    pub const On: DisplayStatus = DisplayStatus(1i32);
    pub const Dimmed: DisplayStatus = DisplayStatus(2i32);
}
impl ::core::convert::From<i32> for DisplayStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DisplayStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DisplayStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.System.Power.DisplayStatus;i4)",
    );
}
impl ::windows::core::DefaultType for DisplayStatus {
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
pub struct EffectivePowerMode(pub i32);
impl EffectivePowerMode {
    pub const BatterySaver: EffectivePowerMode = EffectivePowerMode(0i32);
    pub const BetterBattery: EffectivePowerMode = EffectivePowerMode(1i32);
    pub const Balanced: EffectivePowerMode = EffectivePowerMode(2i32);
    pub const HighPerformance: EffectivePowerMode = EffectivePowerMode(3i32);
    pub const MaxPerformance: EffectivePowerMode = EffectivePowerMode(4i32);
    pub const GameMode: EffectivePowerMode = EffectivePowerMode(5i32);
    pub const MixedReality: EffectivePowerMode = EffectivePowerMode(6i32);
}
impl ::core::convert::From<i32> for EffectivePowerMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EffectivePowerMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for EffectivePowerMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.System.Power.EffectivePowerMode;i4)",
    );
}
impl ::windows::core::DefaultType for EffectivePowerMode {
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
pub struct EnergySaverStatus(pub i32);
impl EnergySaverStatus {
    pub const Uninitialized: EnergySaverStatus = EnergySaverStatus(0i32);
    pub const Disabled: EnergySaverStatus = EnergySaverStatus(1i32);
    pub const Off: EnergySaverStatus = EnergySaverStatus(2i32);
    pub const On: EnergySaverStatus = EnergySaverStatus(3i32);
}
impl ::core::convert::From<i32> for EnergySaverStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EnergySaverStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for EnergySaverStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.System.Power.EnergySaverStatus;i4)",
    );
}
impl ::windows::core::DefaultType for EnergySaverStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IPowerManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPowerManagerStatics {
    type Vtable = IPowerManagerStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xfa3554cc_be1c_534c_bff8_72df78e9f4a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPowerManagerStatics_abi(
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
        result__: *mut EnergySaverStatus,
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
        result__: *mut BatteryStatus,
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
        result__: *mut PowerSupplyStatus,
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
        result__: *mut i32,
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
        result__: *mut ::windows::Foundation::TimeSpan,
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
        result__: *mut PowerSourceKind,
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
        result__: *mut DisplayStatus,
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
        result__: *mut UserPresenceStatus,
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
        result__: *mut SystemSuspendStatus,
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
pub struct PowerManager {}
impl PowerManager {
    pub fn EnergySaverStatus() -> ::windows::core::Result<EnergySaverStatus> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: EnergySaverStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<EnergySaverStatus>(result__)
        })
    }
    pub fn EnergySaverStatusChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::EventHandler<::windows::core::IInspectable>,
        >,
    >(
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveEnergySaverStatusChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    pub fn BatteryStatus() -> ::windows::core::Result<BatteryStatus> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: BatteryStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<BatteryStatus>(result__)
        })
    }
    pub fn BatteryStatusChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::EventHandler<::windows::core::IInspectable>,
        >,
    >(
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveBatteryStatusChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    pub fn PowerSupplyStatus() -> ::windows::core::Result<PowerSupplyStatus> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: PowerSupplyStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PowerSupplyStatus>(result__)
        })
    }
    pub fn PowerSupplyStatusChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::EventHandler<::windows::core::IInspectable>,
        >,
    >(
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemovePowerSupplyStatusChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    pub fn RemainingChargePercent() -> ::windows::core::Result<i32> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        })
    }
    pub fn RemainingChargePercentChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::EventHandler<::windows::core::IInspectable>,
        >,
    >(
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveRemainingChargePercentChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    pub fn RemainingDischargeTime() -> ::windows::core::Result<::windows::Foundation::TimeSpan> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: ::windows::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::TimeSpan>(result__)
        })
    }
    pub fn RemainingDischargeTimeChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::EventHandler<::windows::core::IInspectable>,
        >,
    >(
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveRemainingDischargeTimeChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    pub fn PowerSourceKind() -> ::windows::core::Result<PowerSourceKind> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: PowerSourceKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PowerSourceKind>(result__)
        })
    }
    pub fn PowerSourceKindChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::EventHandler<::windows::core::IInspectable>,
        >,
    >(
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemovePowerSourceKindChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    pub fn DisplayStatus() -> ::windows::core::Result<DisplayStatus> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: DisplayStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<DisplayStatus>(result__)
        })
    }
    pub fn DisplayStatusChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::EventHandler<::windows::core::IInspectable>,
        >,
    >(
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveDisplayStatusChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).26)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    pub fn SystemIdleStatusChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::EventHandler<::windows::core::IInspectable>,
        >,
    >(
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveSystemIdleStatusChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).28)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    pub fn EffectivePowerMode(
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncOperation<EffectivePowerMode>> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IAsyncOperation<EffectivePowerMode>>(result__)
        })
    }
    pub fn EffectivePowerModeChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::EventHandler<::windows::core::IInspectable>,
        >,
    >(
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveEffectivePowerModeChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).31)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    pub fn UserPresenceStatus() -> ::windows::core::Result<UserPresenceStatus> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: UserPresenceStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).32)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<UserPresenceStatus>(result__)
        })
    }
    pub fn UserPresenceStatusChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::EventHandler<::windows::core::IInspectable>,
        >,
    >(
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveUserPresenceStatusChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).34)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    pub fn SystemSuspendStatus() -> ::windows::core::Result<SystemSuspendStatus> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: SystemSuspendStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).35)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SystemSuspendStatus>(result__)
        })
    }
    pub fn SystemSuspendStatusChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::EventHandler<::windows::core::IInspectable>,
        >,
    >(
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).36)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveSystemSuspendStatusChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).37)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    pub fn IPowerManagerStatics<
        R,
        F: FnOnce(&IPowerManagerStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PowerManager, IPowerManagerStatics> =
            ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for PowerManager {
    const NAME: &'static str = "Microsoft.Windows.System.Power.PowerManager";
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
pub struct PowerSourceKind(pub i32);
impl PowerSourceKind {
    pub const AC: PowerSourceKind = PowerSourceKind(0i32);
    pub const DC: PowerSourceKind = PowerSourceKind(1i32);
}
impl ::core::convert::From<i32> for PowerSourceKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PowerSourceKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PowerSourceKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.System.Power.PowerSourceKind;i4)",
    );
}
impl ::windows::core::DefaultType for PowerSourceKind {
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
pub struct PowerSupplyStatus(pub i32);
impl PowerSupplyStatus {
    pub const NotPresent: PowerSupplyStatus = PowerSupplyStatus(0i32);
    pub const Inadequate: PowerSupplyStatus = PowerSupplyStatus(1i32);
    pub const Adequate: PowerSupplyStatus = PowerSupplyStatus(2i32);
}
impl ::core::convert::From<i32> for PowerSupplyStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PowerSupplyStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PowerSupplyStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.System.Power.PowerSupplyStatus;i4)",
    );
}
impl ::windows::core::DefaultType for PowerSupplyStatus {
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
pub struct SystemSuspendStatus(pub i32);
impl SystemSuspendStatus {
    pub const Uninitialized: SystemSuspendStatus = SystemSuspendStatus(0i32);
    pub const Entering: SystemSuspendStatus = SystemSuspendStatus(1i32);
    pub const AutoResume: SystemSuspendStatus = SystemSuspendStatus(2i32);
    pub const ManualResume: SystemSuspendStatus = SystemSuspendStatus(3i32);
}
impl ::core::convert::From<i32> for SystemSuspendStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SystemSuspendStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for SystemSuspendStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.System.Power.SystemSuspendStatus;i4)",
    );
}
impl ::windows::core::DefaultType for SystemSuspendStatus {
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
pub struct UserPresenceStatus(pub i32);
impl UserPresenceStatus {
    pub const Present: UserPresenceStatus = UserPresenceStatus(0i32);
    pub const Absent: UserPresenceStatus = UserPresenceStatus(1i32);
}
impl ::core::convert::From<i32> for UserPresenceStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for UserPresenceStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for UserPresenceStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.System.Power.UserPresenceStatus;i4)",
    );
}
impl ::windows::core::DefaultType for UserPresenceStatus {
    type DefaultType = Self;
}
