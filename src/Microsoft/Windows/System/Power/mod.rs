#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clashing_extern_declarations,
    clippy::all
)]
#[doc = "*Required features: `\"Windows_System_Power\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BatteryStatus(pub i32);
impl BatteryStatus {
    pub const NotPresent: Self = Self(0i32);
    pub const Discharging: Self = Self(1i32);
    pub const Idle: Self = Self(2i32);
    pub const Charging: Self = Self(3i32);
}
impl ::core::marker::Copy for BatteryStatus {}
impl ::core::clone::Clone for BatteryStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BatteryStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BatteryStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for BatteryStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BatteryStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BatteryStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.System.Power.BatteryStatus;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Windows_System_Power\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DisplayStatus(pub i32);
impl DisplayStatus {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Dimmed: Self = Self(2i32);
}
impl ::core::marker::Copy for DisplayStatus {}
impl ::core::clone::Clone for DisplayStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DisplayStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for DisplayStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.System.Power.DisplayStatus;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Windows_System_Power\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EffectivePowerMode(pub i32);
impl EffectivePowerMode {
    pub const BatterySaver: Self = Self(0i32);
    pub const BetterBattery: Self = Self(1i32);
    pub const Balanced: Self = Self(2i32);
    pub const HighPerformance: Self = Self(3i32);
    pub const MaxPerformance: Self = Self(4i32);
    pub const GameMode: Self = Self(5i32);
    pub const MixedReality: Self = Self(6i32);
}
impl ::core::marker::Copy for EffectivePowerMode {}
impl ::core::clone::Clone for EffectivePowerMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EffectivePowerMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EffectivePowerMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for EffectivePowerMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EffectivePowerMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EffectivePowerMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.System.Power.EffectivePowerMode;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Windows_System_Power\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EnergySaverStatus(pub i32);
impl EnergySaverStatus {
    pub const Uninitialized: Self = Self(0i32);
    pub const Disabled: Self = Self(1i32);
    pub const Off: Self = Self(2i32);
    pub const On: Self = Self(3i32);
}
impl ::core::marker::Copy for EnergySaverStatus {}
impl ::core::clone::Clone for EnergySaverStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EnergySaverStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EnergySaverStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for EnergySaverStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EnergySaverStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EnergySaverStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.System.Power.EnergySaverStatus;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPowerManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPowerManagerStatics {
    type Vtable = IPowerManagerStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xfa3554cc_be1c_534c_bff8_72df78e9f4a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPowerManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub EnergySaverStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut EnergySaverStatus,
    ) -> ::windows::core::HRESULT,
    pub EnergySaverStatusChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveEnergySaverStatusChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub BatteryStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut BatteryStatus,
    ) -> ::windows::core::HRESULT,
    pub BatteryStatusChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveBatteryStatusChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub PowerSupplyStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut PowerSupplyStatus,
    ) -> ::windows::core::HRESULT,
    pub PowerSupplyStatusChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemovePowerSupplyStatusChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemainingChargePercent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub RemainingChargePercentChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveRemainingChargePercentChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    )
        -> ::windows::core::HRESULT,
    pub RemainingDischargeTime: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::TimeSpan,
    ) -> ::windows::core::HRESULT,
    pub RemainingDischargeTimeChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveRemainingDischargeTimeChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    )
        -> ::windows::core::HRESULT,
    pub PowerSourceKind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut PowerSourceKind,
    ) -> ::windows::core::HRESULT,
    pub PowerSourceKindChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemovePowerSourceKindChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub DisplayStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut DisplayStatus,
    ) -> ::windows::core::HRESULT,
    pub DisplayStatusChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveDisplayStatusChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub SystemIdleStatusChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveSystemIdleStatusChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub EffectivePowerMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub EffectivePowerModeChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveEffectivePowerModeChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub UserPresenceStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut UserPresenceStatus,
    ) -> ::windows::core::HRESULT,
    pub UserPresenceStatusChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveUserPresenceStatusChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub SystemSuspendStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut SystemSuspendStatus,
    ) -> ::windows::core::HRESULT,
    pub SystemSuspendStatusChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveSystemSuspendStatusChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    )
        -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Windows_System_Power\"`*"]
pub struct PowerManager {}
impl PowerManager {
    #[doc = "*Required features: `\"Windows_System_Power\"`*"]
    pub fn EnergySaverStatus() -> ::windows::core::Result<EnergySaverStatus> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: EnergySaverStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EnergySaverStatus)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<EnergySaverStatus>(result__)
        })
    }
    #[doc = "*Required features: `\"Windows_System_Power\"`*"]
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
            (::windows::core::Interface::vtable(this).EnergySaverStatusChanged)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Windows_System_Power\"`*"]
    pub fn RemoveEnergySaverStatusChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).RemoveEnergySaverStatusChanged)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"Windows_System_Power\"`*"]
    pub fn BatteryStatus() -> ::windows::core::Result<BatteryStatus> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: BatteryStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BatteryStatus)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<BatteryStatus>(result__)
        })
    }
    #[doc = "*Required features: `\"Windows_System_Power\"`*"]
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
            (::windows::core::Interface::vtable(this).BatteryStatusChanged)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Windows_System_Power\"`*"]
    pub fn RemoveBatteryStatusChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).RemoveBatteryStatusChanged)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"Windows_System_Power\"`*"]
    pub fn PowerSupplyStatus() -> ::windows::core::Result<PowerSupplyStatus> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: PowerSupplyStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PowerSupplyStatus)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PowerSupplyStatus>(result__)
        })
    }
    #[doc = "*Required features: `\"Windows_System_Power\"`*"]
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
            (::windows::core::Interface::vtable(this).PowerSupplyStatusChanged)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Windows_System_Power\"`*"]
    pub fn RemovePowerSupplyStatusChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).RemovePowerSupplyStatusChanged)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"Windows_System_Power\"`*"]
    pub fn RemainingChargePercent() -> ::windows::core::Result<i32> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RemainingChargePercent)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `\"Windows_System_Power\"`*"]
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
            (::windows::core::Interface::vtable(this).RemainingChargePercentChanged)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Windows_System_Power\"`*"]
    pub fn RemoveRemainingChargePercentChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).RemoveRemainingChargePercentChanged)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"Windows_System_Power\"`*"]
    pub fn RemainingDischargeTime() -> ::windows::core::Result<::windows::Foundation::TimeSpan> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: ::windows::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RemainingDischargeTime)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::TimeSpan>(result__)
        })
    }
    #[doc = "*Required features: `\"Windows_System_Power\"`*"]
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
            (::windows::core::Interface::vtable(this).RemainingDischargeTimeChanged)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Windows_System_Power\"`*"]
    pub fn RemoveRemainingDischargeTimeChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).RemoveRemainingDischargeTimeChanged)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"Windows_System_Power\"`*"]
    pub fn PowerSourceKind() -> ::windows::core::Result<PowerSourceKind> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: PowerSourceKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PowerSourceKind)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PowerSourceKind>(result__)
        })
    }
    #[doc = "*Required features: `\"Windows_System_Power\"`*"]
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
            (::windows::core::Interface::vtable(this).PowerSourceKindChanged)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Windows_System_Power\"`*"]
    pub fn RemovePowerSourceKindChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).RemovePowerSourceKindChanged)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"Windows_System_Power\"`*"]
    pub fn DisplayStatus() -> ::windows::core::Result<DisplayStatus> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: DisplayStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayStatus)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<DisplayStatus>(result__)
        })
    }
    #[doc = "*Required features: `\"Windows_System_Power\"`*"]
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
            (::windows::core::Interface::vtable(this).DisplayStatusChanged)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Windows_System_Power\"`*"]
    pub fn RemoveDisplayStatusChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).RemoveDisplayStatusChanged)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"Windows_System_Power\"`*"]
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
            (::windows::core::Interface::vtable(this).SystemIdleStatusChanged)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Windows_System_Power\"`*"]
    pub fn RemoveSystemIdleStatusChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).RemoveSystemIdleStatusChanged)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"Windows_System_Power\"`*"]
    pub fn EffectivePowerMode(
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncOperation<EffectivePowerMode>> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EffectivePowerMode)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IAsyncOperation<EffectivePowerMode>>(result__)
        })
    }
    #[doc = "*Required features: `\"Windows_System_Power\"`*"]
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
            (::windows::core::Interface::vtable(this).EffectivePowerModeChanged)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Windows_System_Power\"`*"]
    pub fn RemoveEffectivePowerModeChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).RemoveEffectivePowerModeChanged)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"Windows_System_Power\"`*"]
    pub fn UserPresenceStatus() -> ::windows::core::Result<UserPresenceStatus> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: UserPresenceStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UserPresenceStatus)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<UserPresenceStatus>(result__)
        })
    }
    #[doc = "*Required features: `\"Windows_System_Power\"`*"]
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
            (::windows::core::Interface::vtable(this).UserPresenceStatusChanged)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Windows_System_Power\"`*"]
    pub fn RemoveUserPresenceStatusChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).RemoveUserPresenceStatusChanged)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"Windows_System_Power\"`*"]
    pub fn SystemSuspendStatus() -> ::windows::core::Result<SystemSuspendStatus> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: SystemSuspendStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SystemSuspendStatus)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SystemSuspendStatus>(result__)
        })
    }
    #[doc = "*Required features: `\"Windows_System_Power\"`*"]
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
            (::windows::core::Interface::vtable(this).SystemSuspendStatusChanged)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Windows_System_Power\"`*"]
    pub fn RemoveSystemSuspendStatusChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).RemoveSystemSuspendStatusChanged)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc(hidden)]
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
#[doc = "*Required features: `\"Windows_System_Power\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PowerSourceKind(pub i32);
impl PowerSourceKind {
    pub const AC: Self = Self(0i32);
    pub const DC: Self = Self(1i32);
}
impl ::core::marker::Copy for PowerSourceKind {}
impl ::core::clone::Clone for PowerSourceKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PowerSourceKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PowerSourceKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for PowerSourceKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PowerSourceKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PowerSourceKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.System.Power.PowerSourceKind;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Windows_System_Power\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PowerSupplyStatus(pub i32);
impl PowerSupplyStatus {
    pub const NotPresent: Self = Self(0i32);
    pub const Inadequate: Self = Self(1i32);
    pub const Adequate: Self = Self(2i32);
}
impl ::core::marker::Copy for PowerSupplyStatus {}
impl ::core::clone::Clone for PowerSupplyStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PowerSupplyStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PowerSupplyStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for PowerSupplyStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PowerSupplyStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PowerSupplyStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.System.Power.PowerSupplyStatus;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Windows_System_Power\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SystemSuspendStatus(pub i32);
impl SystemSuspendStatus {
    pub const Uninitialized: Self = Self(0i32);
    pub const Entering: Self = Self(1i32);
    pub const AutoResume: Self = Self(2i32);
    pub const ManualResume: Self = Self(3i32);
}
impl ::core::marker::Copy for SystemSuspendStatus {}
impl ::core::clone::Clone for SystemSuspendStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SystemSuspendStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SystemSuspendStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for SystemSuspendStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemSuspendStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SystemSuspendStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.System.Power.SystemSuspendStatus;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Windows_System_Power\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UserPresenceStatus(pub i32);
impl UserPresenceStatus {
    pub const Present: Self = Self(0i32);
    pub const Absent: Self = Self(1i32);
}
impl ::core::marker::Copy for UserPresenceStatus {}
impl ::core::clone::Clone for UserPresenceStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserPresenceStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UserPresenceStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserPresenceStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserPresenceStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserPresenceStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.System.Power.UserPresenceStatus;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
