impl serde::Serialize for Alert {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.active_period.is_empty() {
            len += 1;
        }
        if !self.informed_entity.is_empty() {
            len += 1;
        }
        if self.cause.is_some() {
            len += 1;
        }
        if self.effect.is_some() {
            len += 1;
        }
        if self.url.is_some() {
            len += 1;
        }
        if self.header_text.is_some() {
            len += 1;
        }
        if self.description_text.is_some() {
            len += 1;
        }
        if self.tts_header_text.is_some() {
            len += 1;
        }
        if self.tts_description_text.is_some() {
            len += 1;
        }
        if self.severity_level.is_some() {
            len += 1;
        }
        if self.image.is_some() {
            len += 1;
        }
        if self.image_alternative_text.is_some() {
            len += 1;
        }
        if self.cause_detail.is_some() {
            len += 1;
        }
        if self.effect_detail.is_some() {
            len += 1;
        }
        if self.transit_alert_extension.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("transit_realtime.Alert", len)?;
        if !self.active_period.is_empty() {
            struct_ser.serialize_field("activePeriod", &self.active_period)?;
        }
        if !self.informed_entity.is_empty() {
            struct_ser.serialize_field("informedEntity", &self.informed_entity)?;
        }
        if let Some(v) = self.cause.as_ref() {
            let v = alert::Cause::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("cause", &v)?;
        }
        if let Some(v) = self.effect.as_ref() {
            let v = alert::Effect::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("effect", &v)?;
        }
        if let Some(v) = self.url.as_ref() {
            struct_ser.serialize_field("url", v)?;
        }
        if let Some(v) = self.header_text.as_ref() {
            struct_ser.serialize_field("headerText", v)?;
        }
        if let Some(v) = self.description_text.as_ref() {
            struct_ser.serialize_field("descriptionText", v)?;
        }
        if let Some(v) = self.tts_header_text.as_ref() {
            struct_ser.serialize_field("ttsHeaderText", v)?;
        }
        if let Some(v) = self.tts_description_text.as_ref() {
            struct_ser.serialize_field("ttsDescriptionText", v)?;
        }
        if let Some(v) = self.severity_level.as_ref() {
            let v = alert::SeverityLevel::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("severityLevel", &v)?;
        }
        if let Some(v) = self.image.as_ref() {
            struct_ser.serialize_field("image", v)?;
        }
        if let Some(v) = self.image_alternative_text.as_ref() {
            struct_ser.serialize_field("imageAlternativeText", v)?;
        }
        if let Some(v) = self.cause_detail.as_ref() {
            struct_ser.serialize_field("causeDetail", v)?;
        }
        if let Some(v) = self.effect_detail.as_ref() {
            struct_ser.serialize_field("effectDetail", v)?;
        }
        if let Some(v) = self.transit_alert_extension.as_ref() {
            struct_ser.serialize_field("transitAlertExtension", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Alert {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "active_period",
            "activePeriod",
            "informed_entity",
            "informedEntity",
            "cause",
            "effect",
            "url",
            "header_text",
            "headerText",
            "description_text",
            "descriptionText",
            "tts_header_text",
            "ttsHeaderText",
            "tts_description_text",
            "ttsDescriptionText",
            "severity_level",
            "severityLevel",
            "image",
            "image_alternative_text",
            "imageAlternativeText",
            "cause_detail",
            "causeDetail",
            "effect_detail",
            "effectDetail",
            "transit_alert_extension",
            "transitAlertExtension",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ActivePeriod,
            InformedEntity,
            Cause,
            Effect,
            Url,
            HeaderText,
            DescriptionText,
            TtsHeaderText,
            TtsDescriptionText,
            SeverityLevel,
            Image,
            ImageAlternativeText,
            CauseDetail,
            EffectDetail,
            TransitAlertExtension,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "activePeriod" | "active_period" => Ok(GeneratedField::ActivePeriod),
                            "informedEntity" | "informed_entity" => Ok(GeneratedField::InformedEntity),
                            "cause" => Ok(GeneratedField::Cause),
                            "effect" => Ok(GeneratedField::Effect),
                            "url" => Ok(GeneratedField::Url),
                            "headerText" | "header_text" => Ok(GeneratedField::HeaderText),
                            "descriptionText" | "description_text" => Ok(GeneratedField::DescriptionText),
                            "ttsHeaderText" | "tts_header_text" => Ok(GeneratedField::TtsHeaderText),
                            "ttsDescriptionText" | "tts_description_text" => Ok(GeneratedField::TtsDescriptionText),
                            "severityLevel" | "severity_level" => Ok(GeneratedField::SeverityLevel),
                            "image" => Ok(GeneratedField::Image),
                            "imageAlternativeText" | "image_alternative_text" => Ok(GeneratedField::ImageAlternativeText),
                            "causeDetail" | "cause_detail" => Ok(GeneratedField::CauseDetail),
                            "effectDetail" | "effect_detail" => Ok(GeneratedField::EffectDetail),
                            "transitAlertExtension" | "transit_alert_extension" => Ok(GeneratedField::TransitAlertExtension),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Alert;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct transit_realtime.Alert")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Alert, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut active_period__ = None;
                let mut informed_entity__ = None;
                let mut cause__ = None;
                let mut effect__ = None;
                let mut url__ = None;
                let mut header_text__ = None;
                let mut description_text__ = None;
                let mut tts_header_text__ = None;
                let mut tts_description_text__ = None;
                let mut severity_level__ = None;
                let mut image__ = None;
                let mut image_alternative_text__ = None;
                let mut cause_detail__ = None;
                let mut effect_detail__ = None;
                let mut transit_alert_extension__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ActivePeriod => {
                            if active_period__.is_some() {
                                return Err(serde::de::Error::duplicate_field("activePeriod"));
                            }
                            active_period__ = Some(map_.next_value()?);
                        }
                        GeneratedField::InformedEntity => {
                            if informed_entity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("informedEntity"));
                            }
                            informed_entity__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Cause => {
                            if cause__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cause"));
                            }
                            cause__ = map_.next_value::<::std::option::Option<alert::Cause>>()?.map(|x| x as i32);
                        }
                        GeneratedField::Effect => {
                            if effect__.is_some() {
                                return Err(serde::de::Error::duplicate_field("effect"));
                            }
                            effect__ = map_.next_value::<::std::option::Option<alert::Effect>>()?.map(|x| x as i32);
                        }
                        GeneratedField::Url => {
                            if url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            url__ = map_.next_value()?;
                        }
                        GeneratedField::HeaderText => {
                            if header_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headerText"));
                            }
                            header_text__ = map_.next_value()?;
                        }
                        GeneratedField::DescriptionText => {
                            if description_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("descriptionText"));
                            }
                            description_text__ = map_.next_value()?;
                        }
                        GeneratedField::TtsHeaderText => {
                            if tts_header_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ttsHeaderText"));
                            }
                            tts_header_text__ = map_.next_value()?;
                        }
                        GeneratedField::TtsDescriptionText => {
                            if tts_description_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ttsDescriptionText"));
                            }
                            tts_description_text__ = map_.next_value()?;
                        }
                        GeneratedField::SeverityLevel => {
                            if severity_level__.is_some() {
                                return Err(serde::de::Error::duplicate_field("severityLevel"));
                            }
                            severity_level__ = map_.next_value::<::std::option::Option<alert::SeverityLevel>>()?.map(|x| x as i32);
                        }
                        GeneratedField::Image => {
                            if image__.is_some() {
                                return Err(serde::de::Error::duplicate_field("image"));
                            }
                            image__ = map_.next_value()?;
                        }
                        GeneratedField::ImageAlternativeText => {
                            if image_alternative_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("imageAlternativeText"));
                            }
                            image_alternative_text__ = map_.next_value()?;
                        }
                        GeneratedField::CauseDetail => {
                            if cause_detail__.is_some() {
                                return Err(serde::de::Error::duplicate_field("causeDetail"));
                            }
                            cause_detail__ = map_.next_value()?;
                        }
                        GeneratedField::EffectDetail => {
                            if effect_detail__.is_some() {
                                return Err(serde::de::Error::duplicate_field("effectDetail"));
                            }
                            effect_detail__ = map_.next_value()?;
                        }
                        GeneratedField::TransitAlertExtension => {
                            if transit_alert_extension__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transitAlertExtension"));
                            }
                            transit_alert_extension__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Alert {
                    active_period: active_period__.unwrap_or_default(),
                    informed_entity: informed_entity__.unwrap_or_default(),
                    cause: cause__,
                    effect: effect__,
                    url: url__,
                    header_text: header_text__,
                    description_text: description_text__,
                    tts_header_text: tts_header_text__,
                    tts_description_text: tts_description_text__,
                    severity_level: severity_level__,
                    image: image__,
                    image_alternative_text: image_alternative_text__,
                    cause_detail: cause_detail__,
                    effect_detail: effect_detail__,
                    transit_alert_extension: transit_alert_extension__,
                })
            }
        }
        deserializer.deserialize_struct("transit_realtime.Alert", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for alert::Cause {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::UnknownCause => "UNKNOWN_CAUSE",
            Self::OtherCause => "OTHER_CAUSE",
            Self::TechnicalProblem => "TECHNICAL_PROBLEM",
            Self::Strike => "STRIKE",
            Self::Demonstration => "DEMONSTRATION",
            Self::Accident => "ACCIDENT",
            Self::Holiday => "HOLIDAY",
            Self::Weather => "WEATHER",
            Self::Maintenance => "MAINTENANCE",
            Self::Construction => "CONSTRUCTION",
            Self::PoliceActivity => "POLICE_ACTIVITY",
            Self::MedicalEmergency => "MEDICAL_EMERGENCY",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for alert::Cause {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNKNOWN_CAUSE",
            "OTHER_CAUSE",
            "TECHNICAL_PROBLEM",
            "STRIKE",
            "DEMONSTRATION",
            "ACCIDENT",
            "HOLIDAY",
            "WEATHER",
            "MAINTENANCE",
            "CONSTRUCTION",
            "POLICE_ACTIVITY",
            "MEDICAL_EMERGENCY",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = alert::Cause;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "UNKNOWN_CAUSE" => Ok(alert::Cause::UnknownCause),
                    "OTHER_CAUSE" => Ok(alert::Cause::OtherCause),
                    "TECHNICAL_PROBLEM" => Ok(alert::Cause::TechnicalProblem),
                    "STRIKE" => Ok(alert::Cause::Strike),
                    "DEMONSTRATION" => Ok(alert::Cause::Demonstration),
                    "ACCIDENT" => Ok(alert::Cause::Accident),
                    "HOLIDAY" => Ok(alert::Cause::Holiday),
                    "WEATHER" => Ok(alert::Cause::Weather),
                    "MAINTENANCE" => Ok(alert::Cause::Maintenance),
                    "CONSTRUCTION" => Ok(alert::Cause::Construction),
                    "POLICE_ACTIVITY" => Ok(alert::Cause::PoliceActivity),
                    "MEDICAL_EMERGENCY" => Ok(alert::Cause::MedicalEmergency),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for alert::Effect {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::NoService => "NO_SERVICE",
            Self::ReducedService => "REDUCED_SERVICE",
            Self::SignificantDelays => "SIGNIFICANT_DELAYS",
            Self::Detour => "DETOUR",
            Self::AdditionalService => "ADDITIONAL_SERVICE",
            Self::ModifiedService => "MODIFIED_SERVICE",
            Self::OtherEffect => "OTHER_EFFECT",
            Self::UnknownEffect => "UNKNOWN_EFFECT",
            Self::StopMoved => "STOP_MOVED",
            Self::NoEffect => "NO_EFFECT",
            Self::AccessibilityIssue => "ACCESSIBILITY_ISSUE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for alert::Effect {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "NO_SERVICE",
            "REDUCED_SERVICE",
            "SIGNIFICANT_DELAYS",
            "DETOUR",
            "ADDITIONAL_SERVICE",
            "MODIFIED_SERVICE",
            "OTHER_EFFECT",
            "UNKNOWN_EFFECT",
            "STOP_MOVED",
            "NO_EFFECT",
            "ACCESSIBILITY_ISSUE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = alert::Effect;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "NO_SERVICE" => Ok(alert::Effect::NoService),
                    "REDUCED_SERVICE" => Ok(alert::Effect::ReducedService),
                    "SIGNIFICANT_DELAYS" => Ok(alert::Effect::SignificantDelays),
                    "DETOUR" => Ok(alert::Effect::Detour),
                    "ADDITIONAL_SERVICE" => Ok(alert::Effect::AdditionalService),
                    "MODIFIED_SERVICE" => Ok(alert::Effect::ModifiedService),
                    "OTHER_EFFECT" => Ok(alert::Effect::OtherEffect),
                    "UNKNOWN_EFFECT" => Ok(alert::Effect::UnknownEffect),
                    "STOP_MOVED" => Ok(alert::Effect::StopMoved),
                    "NO_EFFECT" => Ok(alert::Effect::NoEffect),
                    "ACCESSIBILITY_ISSUE" => Ok(alert::Effect::AccessibilityIssue),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for alert::SeverityLevel {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::UnknownSeverity => "UNKNOWN_SEVERITY",
            Self::Info => "INFO",
            Self::Warning => "WARNING",
            Self::Severe => "SEVERE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for alert::SeverityLevel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNKNOWN_SEVERITY",
            "INFO",
            "WARNING",
            "SEVERE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = alert::SeverityLevel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "UNKNOWN_SEVERITY" => Ok(alert::SeverityLevel::UnknownSeverity),
                    "INFO" => Ok(alert::SeverityLevel::Info),
                    "WARNING" => Ok(alert::SeverityLevel::Warning),
                    "SEVERE" => Ok(alert::SeverityLevel::Severe),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for EntitySelector {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.agency_id.is_some() {
            len += 1;
        }
        if self.route_id.is_some() {
            len += 1;
        }
        if self.route_type.is_some() {
            len += 1;
        }
        if self.trip.is_some() {
            len += 1;
        }
        if self.stop_id.is_some() {
            len += 1;
        }
        if self.direction_id.is_some() {
            len += 1;
        }
        if self.transit_entity_selector_extension.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("transit_realtime.EntitySelector", len)?;
        if let Some(v) = self.agency_id.as_ref() {
            struct_ser.serialize_field("agencyId", v)?;
        }
        if let Some(v) = self.route_id.as_ref() {
            struct_ser.serialize_field("routeId", v)?;
        }
        if let Some(v) = self.route_type.as_ref() {
            struct_ser.serialize_field("routeType", v)?;
        }
        if let Some(v) = self.trip.as_ref() {
            struct_ser.serialize_field("trip", v)?;
        }
        if let Some(v) = self.stop_id.as_ref() {
            struct_ser.serialize_field("stopId", v)?;
        }
        if let Some(v) = self.direction_id.as_ref() {
            struct_ser.serialize_field("directionId", v)?;
        }
        if let Some(v) = self.transit_entity_selector_extension.as_ref() {
            struct_ser.serialize_field("transitEntitySelectorExtension", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EntitySelector {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "agency_id",
            "agencyId",
            "route_id",
            "routeId",
            "route_type",
            "routeType",
            "trip",
            "stop_id",
            "stopId",
            "direction_id",
            "directionId",
            "transit_entity_selector_extension",
            "transitEntitySelectorExtension",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AgencyId,
            RouteId,
            RouteType,
            Trip,
            StopId,
            DirectionId,
            TransitEntitySelectorExtension,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "agencyId" | "agency_id" => Ok(GeneratedField::AgencyId),
                            "routeId" | "route_id" => Ok(GeneratedField::RouteId),
                            "routeType" | "route_type" => Ok(GeneratedField::RouteType),
                            "trip" => Ok(GeneratedField::Trip),
                            "stopId" | "stop_id" => Ok(GeneratedField::StopId),
                            "directionId" | "direction_id" => Ok(GeneratedField::DirectionId),
                            "transitEntitySelectorExtension" | "transit_entity_selector_extension" => Ok(GeneratedField::TransitEntitySelectorExtension),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EntitySelector;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct transit_realtime.EntitySelector")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EntitySelector, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut agency_id__ = None;
                let mut route_id__ = None;
                let mut route_type__ = None;
                let mut trip__ = None;
                let mut stop_id__ = None;
                let mut direction_id__ = None;
                let mut transit_entity_selector_extension__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AgencyId => {
                            if agency_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("agencyId"));
                            }
                            agency_id__ = map_.next_value()?;
                        }
                        GeneratedField::RouteId => {
                            if route_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routeId"));
                            }
                            route_id__ = map_.next_value()?;
                        }
                        GeneratedField::RouteType => {
                            if route_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routeType"));
                            }
                            route_type__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Trip => {
                            if trip__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trip"));
                            }
                            trip__ = map_.next_value()?;
                        }
                        GeneratedField::StopId => {
                            if stop_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stopId"));
                            }
                            stop_id__ = map_.next_value()?;
                        }
                        GeneratedField::DirectionId => {
                            if direction_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("directionId"));
                            }
                            direction_id__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::TransitEntitySelectorExtension => {
                            if transit_entity_selector_extension__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transitEntitySelectorExtension"));
                            }
                            transit_entity_selector_extension__ = map_.next_value()?;
                        }
                    }
                }
                Ok(EntitySelector {
                    agency_id: agency_id__,
                    route_id: route_id__,
                    route_type: route_type__,
                    trip: trip__,
                    stop_id: stop_id__,
                    direction_id: direction_id__,
                    transit_entity_selector_extension: transit_entity_selector_extension__,
                })
            }
        }
        deserializer.deserialize_struct("transit_realtime.EntitySelector", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FeedEntity {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 1;
        if self.is_deleted.is_some() {
            len += 1;
        }
        if self.trip_update.is_some() {
            len += 1;
        }
        if self.vehicle.is_some() {
            len += 1;
        }
        if self.alert.is_some() {
            len += 1;
        }
        if self.shape.is_some() {
            len += 1;
        }
        if self.stop.is_some() {
            len += 1;
        }
        if self.trip_modifications.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("transit_realtime.FeedEntity", len)?;
        struct_ser.serialize_field("id", &self.id)?;
        if let Some(v) = self.is_deleted.as_ref() {
            struct_ser.serialize_field("isDeleted", v)?;
        }
        if let Some(v) = self.trip_update.as_ref() {
            struct_ser.serialize_field("tripUpdate", v)?;
        }
        if let Some(v) = self.vehicle.as_ref() {
            struct_ser.serialize_field("vehicle", v)?;
        }
        if let Some(v) = self.alert.as_ref() {
            struct_ser.serialize_field("alert", v)?;
        }
        if let Some(v) = self.shape.as_ref() {
            struct_ser.serialize_field("shape", v)?;
        }
        if let Some(v) = self.stop.as_ref() {
            struct_ser.serialize_field("stop", v)?;
        }
        if let Some(v) = self.trip_modifications.as_ref() {
            struct_ser.serialize_field("tripModifications", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FeedEntity {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "is_deleted",
            "isDeleted",
            "trip_update",
            "tripUpdate",
            "vehicle",
            "alert",
            "shape",
            "stop",
            "trip_modifications",
            "tripModifications",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            IsDeleted,
            TripUpdate,
            Vehicle,
            Alert,
            Shape,
            Stop,
            TripModifications,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "isDeleted" | "is_deleted" => Ok(GeneratedField::IsDeleted),
                            "tripUpdate" | "trip_update" => Ok(GeneratedField::TripUpdate),
                            "vehicle" => Ok(GeneratedField::Vehicle),
                            "alert" => Ok(GeneratedField::Alert),
                            "shape" => Ok(GeneratedField::Shape),
                            "stop" => Ok(GeneratedField::Stop),
                            "tripModifications" | "trip_modifications" => Ok(GeneratedField::TripModifications),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FeedEntity;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct transit_realtime.FeedEntity")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FeedEntity, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut is_deleted__ = None;
                let mut trip_update__ = None;
                let mut vehicle__ = None;
                let mut alert__ = None;
                let mut shape__ = None;
                let mut stop__ = None;
                let mut trip_modifications__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsDeleted => {
                            if is_deleted__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isDeleted"));
                            }
                            is_deleted__ = map_.next_value()?;
                        }
                        GeneratedField::TripUpdate => {
                            if trip_update__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tripUpdate"));
                            }
                            trip_update__ = map_.next_value()?;
                        }
                        GeneratedField::Vehicle => {
                            if vehicle__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vehicle"));
                            }
                            vehicle__ = map_.next_value()?;
                        }
                        GeneratedField::Alert => {
                            if alert__.is_some() {
                                return Err(serde::de::Error::duplicate_field("alert"));
                            }
                            alert__ = map_.next_value()?;
                        }
                        GeneratedField::Shape => {
                            if shape__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shape"));
                            }
                            shape__ = map_.next_value()?;
                        }
                        GeneratedField::Stop => {
                            if stop__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stop"));
                            }
                            stop__ = map_.next_value()?;
                        }
                        GeneratedField::TripModifications => {
                            if trip_modifications__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tripModifications"));
                            }
                            trip_modifications__ = map_.next_value()?;
                        }
                    }
                }
                Ok(FeedEntity {
                    id: id__.ok_or_else(|| serde::de::Error::missing_field("id"))?,
                    is_deleted: is_deleted__,
                    trip_update: trip_update__,
                    vehicle: vehicle__,
                    alert: alert__,
                    shape: shape__,
                    stop: stop__,
                    trip_modifications: trip_modifications__,
                })
            }
        }
        deserializer.deserialize_struct("transit_realtime.FeedEntity", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FeedHeader {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 1;
        if self.incrementality.is_some() {
            len += 1;
        }
        if self.timestamp.is_some() {
            len += 1;
        }
        if self.nyct_feed_header.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("transit_realtime.FeedHeader", len)?;
        struct_ser.serialize_field("gtfsRealtimeVersion", &self.gtfs_realtime_version)?;
        if let Some(v) = self.incrementality.as_ref() {
            let v = feed_header::Incrementality::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("incrementality", &v)?;
        }
        if let Some(v) = self.timestamp.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("timestamp", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.nyct_feed_header.as_ref() {
            struct_ser.serialize_field("nyctFeedHeader", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FeedHeader {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "gtfs_realtime_version",
            "gtfsRealtimeVersion",
            "incrementality",
            "timestamp",
            "nyct_feed_header",
            "nyctFeedHeader",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GtfsRealtimeVersion,
            Incrementality,
            Timestamp,
            NyctFeedHeader,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "gtfsRealtimeVersion" | "gtfs_realtime_version" => Ok(GeneratedField::GtfsRealtimeVersion),
                            "incrementality" => Ok(GeneratedField::Incrementality),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            "nyctFeedHeader" | "nyct_feed_header" => Ok(GeneratedField::NyctFeedHeader),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FeedHeader;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct transit_realtime.FeedHeader")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FeedHeader, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut gtfs_realtime_version__ = None;
                let mut incrementality__ = None;
                let mut timestamp__ = None;
                let mut nyct_feed_header__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::GtfsRealtimeVersion => {
                            if gtfs_realtime_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gtfsRealtimeVersion"));
                            }
                            gtfs_realtime_version__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Incrementality => {
                            if incrementality__.is_some() {
                                return Err(serde::de::Error::duplicate_field("incrementality"));
                            }
                            incrementality__ = map_.next_value::<::std::option::Option<feed_header::Incrementality>>()?.map(|x| x as i32);
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::NyctFeedHeader => {
                            if nyct_feed_header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nyctFeedHeader"));
                            }
                            nyct_feed_header__ = map_.next_value()?;
                        }
                    }
                }
                Ok(FeedHeader {
                    gtfs_realtime_version: gtfs_realtime_version__.ok_or_else(|| serde::de::Error::missing_field("gtfsRealtimeVersion"))?,
                    incrementality: incrementality__,
                    timestamp: timestamp__,
                    nyct_feed_header: nyct_feed_header__,
                })
            }
        }
        deserializer.deserialize_struct("transit_realtime.FeedHeader", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for feed_header::Incrementality {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::FullDataset => "FULL_DATASET",
            Self::Differential => "DIFFERENTIAL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for feed_header::Incrementality {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "FULL_DATASET",
            "DIFFERENTIAL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = feed_header::Incrementality;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "FULL_DATASET" => Ok(feed_header::Incrementality::FullDataset),
                    "DIFFERENTIAL" => Ok(feed_header::Incrementality::Differential),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for FeedMessage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 1;
        if !self.entity.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("transit_realtime.FeedMessage", len)?;
        struct_ser.serialize_field("header", &self.header)?;
        if !self.entity.is_empty() {
            struct_ser.serialize_field("entity", &self.entity)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FeedMessage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "entity",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Entity,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "header" => Ok(GeneratedField::Header),
                            "entity" => Ok(GeneratedField::Entity),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FeedMessage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct transit_realtime.FeedMessage")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FeedMessage, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut entity__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map_.next_value()?;
                        }
                        GeneratedField::Entity => {
                            if entity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entity"));
                            }
                            entity__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FeedMessage {
                    header: header__.ok_or_else(|| serde::de::Error::missing_field("header"))?,
                    entity: entity__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("transit_realtime.FeedMessage", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NyctFeedHeader {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 1;
        if !self.trip_replacement_period.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("transit_realtime.NyctFeedHeader", len)?;
        struct_ser.serialize_field("nyctSubwayVersion", &self.nyct_subway_version)?;
        if !self.trip_replacement_period.is_empty() {
            struct_ser.serialize_field("tripReplacementPeriod", &self.trip_replacement_period)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NyctFeedHeader {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "nyct_subway_version",
            "nyctSubwayVersion",
            "trip_replacement_period",
            "tripReplacementPeriod",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NyctSubwayVersion,
            TripReplacementPeriod,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "nyctSubwayVersion" | "nyct_subway_version" => Ok(GeneratedField::NyctSubwayVersion),
                            "tripReplacementPeriod" | "trip_replacement_period" => Ok(GeneratedField::TripReplacementPeriod),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NyctFeedHeader;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct transit_realtime.NyctFeedHeader")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NyctFeedHeader, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut nyct_subway_version__ = None;
                let mut trip_replacement_period__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NyctSubwayVersion => {
                            if nyct_subway_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nyctSubwayVersion"));
                            }
                            nyct_subway_version__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TripReplacementPeriod => {
                            if trip_replacement_period__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tripReplacementPeriod"));
                            }
                            trip_replacement_period__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(NyctFeedHeader {
                    nyct_subway_version: nyct_subway_version__.ok_or_else(|| serde::de::Error::missing_field("nyctSubwayVersion"))?,
                    trip_replacement_period: trip_replacement_period__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("transit_realtime.NyctFeedHeader", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NyctStopTimeUpdate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.scheduled_track.is_some() {
            len += 1;
        }
        if self.actual_track.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("transit_realtime.NyctStopTimeUpdate", len)?;
        if let Some(v) = self.scheduled_track.as_ref() {
            struct_ser.serialize_field("scheduledTrack", v)?;
        }
        if let Some(v) = self.actual_track.as_ref() {
            struct_ser.serialize_field("actualTrack", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NyctStopTimeUpdate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "scheduled_track",
            "scheduledTrack",
            "actual_track",
            "actualTrack",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ScheduledTrack,
            ActualTrack,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "scheduledTrack" | "scheduled_track" => Ok(GeneratedField::ScheduledTrack),
                            "actualTrack" | "actual_track" => Ok(GeneratedField::ActualTrack),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NyctStopTimeUpdate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct transit_realtime.NyctStopTimeUpdate")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NyctStopTimeUpdate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut scheduled_track__ = None;
                let mut actual_track__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ScheduledTrack => {
                            if scheduled_track__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scheduledTrack"));
                            }
                            scheduled_track__ = map_.next_value()?;
                        }
                        GeneratedField::ActualTrack => {
                            if actual_track__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actualTrack"));
                            }
                            actual_track__ = map_.next_value()?;
                        }
                    }
                }
                Ok(NyctStopTimeUpdate {
                    scheduled_track: scheduled_track__,
                    actual_track: actual_track__,
                })
            }
        }
        deserializer.deserialize_struct("transit_realtime.NyctStopTimeUpdate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NyctTripDescriptor {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.train_id.is_some() {
            len += 1;
        }
        if self.is_assigned.is_some() {
            len += 1;
        }
        if self.direction.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("transit_realtime.NyctTripDescriptor", len)?;
        if let Some(v) = self.train_id.as_ref() {
            struct_ser.serialize_field("trainId", v)?;
        }
        if let Some(v) = self.is_assigned.as_ref() {
            struct_ser.serialize_field("isAssigned", v)?;
        }
        if let Some(v) = self.direction.as_ref() {
            let v = nyct_trip_descriptor::Direction::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("direction", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NyctTripDescriptor {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "train_id",
            "trainId",
            "is_assigned",
            "isAssigned",
            "direction",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TrainId,
            IsAssigned,
            Direction,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "trainId" | "train_id" => Ok(GeneratedField::TrainId),
                            "isAssigned" | "is_assigned" => Ok(GeneratedField::IsAssigned),
                            "direction" => Ok(GeneratedField::Direction),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NyctTripDescriptor;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct transit_realtime.NyctTripDescriptor")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NyctTripDescriptor, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut train_id__ = None;
                let mut is_assigned__ = None;
                let mut direction__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TrainId => {
                            if train_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trainId"));
                            }
                            train_id__ = map_.next_value()?;
                        }
                        GeneratedField::IsAssigned => {
                            if is_assigned__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isAssigned"));
                            }
                            is_assigned__ = map_.next_value()?;
                        }
                        GeneratedField::Direction => {
                            if direction__.is_some() {
                                return Err(serde::de::Error::duplicate_field("direction"));
                            }
                            direction__ = map_.next_value::<::std::option::Option<nyct_trip_descriptor::Direction>>()?.map(|x| x as i32);
                        }
                    }
                }
                Ok(NyctTripDescriptor {
                    train_id: train_id__,
                    is_assigned: is_assigned__,
                    direction: direction__,
                })
            }
        }
        deserializer.deserialize_struct("transit_realtime.NyctTripDescriptor", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for nyct_trip_descriptor::Direction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::North => "NORTH",
            Self::East => "EAST",
            Self::South => "SOUTH",
            Self::West => "WEST",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for nyct_trip_descriptor::Direction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "NORTH",
            "EAST",
            "SOUTH",
            "WEST",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = nyct_trip_descriptor::Direction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "NORTH" => Ok(nyct_trip_descriptor::Direction::North),
                    "EAST" => Ok(nyct_trip_descriptor::Direction::East),
                    "SOUTH" => Ok(nyct_trip_descriptor::Direction::South),
                    "WEST" => Ok(nyct_trip_descriptor::Direction::West),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Position {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 2;
        if self.bearing.is_some() {
            len += 1;
        }
        if self.odometer.is_some() {
            len += 1;
        }
        if self.speed.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("transit_realtime.Position", len)?;
        struct_ser.serialize_field("latitude", &self.latitude)?;
        struct_ser.serialize_field("longitude", &self.longitude)?;
        if let Some(v) = self.bearing.as_ref() {
            struct_ser.serialize_field("bearing", v)?;
        }
        if let Some(v) = self.odometer.as_ref() {
            struct_ser.serialize_field("odometer", v)?;
        }
        if let Some(v) = self.speed.as_ref() {
            struct_ser.serialize_field("speed", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Position {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "latitude",
            "longitude",
            "bearing",
            "odometer",
            "speed",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Latitude,
            Longitude,
            Bearing,
            Odometer,
            Speed,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "latitude" => Ok(GeneratedField::Latitude),
                            "longitude" => Ok(GeneratedField::Longitude),
                            "bearing" => Ok(GeneratedField::Bearing),
                            "odometer" => Ok(GeneratedField::Odometer),
                            "speed" => Ok(GeneratedField::Speed),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Position;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct transit_realtime.Position")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Position, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut latitude__ = None;
                let mut longitude__ = None;
                let mut bearing__ = None;
                let mut odometer__ = None;
                let mut speed__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Latitude => {
                            if latitude__.is_some() {
                                return Err(serde::de::Error::duplicate_field("latitude"));
                            }
                            latitude__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Longitude => {
                            if longitude__.is_some() {
                                return Err(serde::de::Error::duplicate_field("longitude"));
                            }
                            longitude__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Bearing => {
                            if bearing__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bearing"));
                            }
                            bearing__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Odometer => {
                            if odometer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("odometer"));
                            }
                            odometer__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Speed => {
                            if speed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("speed"));
                            }
                            speed__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(Position {
                    latitude: latitude__.ok_or_else(|| serde::de::Error::missing_field("latitude"))?,
                    longitude: longitude__.ok_or_else(|| serde::de::Error::missing_field("longitude"))?,
                    bearing: bearing__,
                    odometer: odometer__,
                    speed: speed__,
                })
            }
        }
        deserializer.deserialize_struct("transit_realtime.Position", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReplacementStop {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.travel_time_to_stop.is_some() {
            len += 1;
        }
        if self.stop_id.is_some() {
            len += 1;
        }
        if self.transit_replacement_stop_extension.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("transit_realtime.ReplacementStop", len)?;
        if let Some(v) = self.travel_time_to_stop.as_ref() {
            struct_ser.serialize_field("travelTimeToStop", v)?;
        }
        if let Some(v) = self.stop_id.as_ref() {
            struct_ser.serialize_field("stopId", v)?;
        }
        if let Some(v) = self.transit_replacement_stop_extension.as_ref() {
            struct_ser.serialize_field("transitReplacementStopExtension", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReplacementStop {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "travel_time_to_stop",
            "travelTimeToStop",
            "stop_id",
            "stopId",
            "transit_replacement_stop_extension",
            "transitReplacementStopExtension",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TravelTimeToStop,
            StopId,
            TransitReplacementStopExtension,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "travelTimeToStop" | "travel_time_to_stop" => Ok(GeneratedField::TravelTimeToStop),
                            "stopId" | "stop_id" => Ok(GeneratedField::StopId),
                            "transitReplacementStopExtension" | "transit_replacement_stop_extension" => Ok(GeneratedField::TransitReplacementStopExtension),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReplacementStop;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct transit_realtime.ReplacementStop")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReplacementStop, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut travel_time_to_stop__ = None;
                let mut stop_id__ = None;
                let mut transit_replacement_stop_extension__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TravelTimeToStop => {
                            if travel_time_to_stop__.is_some() {
                                return Err(serde::de::Error::duplicate_field("travelTimeToStop"));
                            }
                            travel_time_to_stop__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::StopId => {
                            if stop_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stopId"));
                            }
                            stop_id__ = map_.next_value()?;
                        }
                        GeneratedField::TransitReplacementStopExtension => {
                            if transit_replacement_stop_extension__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transitReplacementStopExtension"));
                            }
                            transit_replacement_stop_extension__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ReplacementStop {
                    travel_time_to_stop: travel_time_to_stop__,
                    stop_id: stop_id__,
                    transit_replacement_stop_extension: transit_replacement_stop_extension__,
                })
            }
        }
        deserializer.deserialize_struct("transit_realtime.ReplacementStop", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Shape {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.shape_id.is_some() {
            len += 1;
        }
        if self.encoded_polyline.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("transit_realtime.Shape", len)?;
        if let Some(v) = self.shape_id.as_ref() {
            struct_ser.serialize_field("shapeId", v)?;
        }
        if let Some(v) = self.encoded_polyline.as_ref() {
            struct_ser.serialize_field("encodedPolyline", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Shape {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "shape_id",
            "shapeId",
            "encoded_polyline",
            "encodedPolyline",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ShapeId,
            EncodedPolyline,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "shapeId" | "shape_id" => Ok(GeneratedField::ShapeId),
                            "encodedPolyline" | "encoded_polyline" => Ok(GeneratedField::EncodedPolyline),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Shape;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct transit_realtime.Shape")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Shape, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut shape_id__ = None;
                let mut encoded_polyline__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ShapeId => {
                            if shape_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shapeId"));
                            }
                            shape_id__ = map_.next_value()?;
                        }
                        GeneratedField::EncodedPolyline => {
                            if encoded_polyline__.is_some() {
                                return Err(serde::de::Error::duplicate_field("encodedPolyline"));
                            }
                            encoded_polyline__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Shape {
                    shape_id: shape_id__,
                    encoded_polyline: encoded_polyline__,
                })
            }
        }
        deserializer.deserialize_struct("transit_realtime.Shape", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Stop {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.stop_id.is_some() {
            len += 1;
        }
        if self.stop_code.is_some() {
            len += 1;
        }
        if self.stop_name.is_some() {
            len += 1;
        }
        if self.tts_stop_name.is_some() {
            len += 1;
        }
        if self.stop_desc.is_some() {
            len += 1;
        }
        if self.stop_lat.is_some() {
            len += 1;
        }
        if self.stop_lon.is_some() {
            len += 1;
        }
        if self.zone_id.is_some() {
            len += 1;
        }
        if self.stop_url.is_some() {
            len += 1;
        }
        if self.parent_station.is_some() {
            len += 1;
        }
        if self.stop_timezone.is_some() {
            len += 1;
        }
        if self.wheelchair_boarding.is_some() {
            len += 1;
        }
        if self.level_id.is_some() {
            len += 1;
        }
        if self.platform_code.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("transit_realtime.Stop", len)?;
        if let Some(v) = self.stop_id.as_ref() {
            struct_ser.serialize_field("stopId", v)?;
        }
        if let Some(v) = self.stop_code.as_ref() {
            struct_ser.serialize_field("stopCode", v)?;
        }
        if let Some(v) = self.stop_name.as_ref() {
            struct_ser.serialize_field("stopName", v)?;
        }
        if let Some(v) = self.tts_stop_name.as_ref() {
            struct_ser.serialize_field("ttsStopName", v)?;
        }
        if let Some(v) = self.stop_desc.as_ref() {
            struct_ser.serialize_field("stopDesc", v)?;
        }
        if let Some(v) = self.stop_lat.as_ref() {
            struct_ser.serialize_field("stopLat", v)?;
        }
        if let Some(v) = self.stop_lon.as_ref() {
            struct_ser.serialize_field("stopLon", v)?;
        }
        if let Some(v) = self.zone_id.as_ref() {
            struct_ser.serialize_field("zoneId", v)?;
        }
        if let Some(v) = self.stop_url.as_ref() {
            struct_ser.serialize_field("stopUrl", v)?;
        }
        if let Some(v) = self.parent_station.as_ref() {
            struct_ser.serialize_field("parentStation", v)?;
        }
        if let Some(v) = self.stop_timezone.as_ref() {
            struct_ser.serialize_field("stopTimezone", v)?;
        }
        if let Some(v) = self.wheelchair_boarding.as_ref() {
            let v = stop::WheelchairBoarding::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("wheelchairBoarding", &v)?;
        }
        if let Some(v) = self.level_id.as_ref() {
            struct_ser.serialize_field("levelId", v)?;
        }
        if let Some(v) = self.platform_code.as_ref() {
            struct_ser.serialize_field("platformCode", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Stop {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stop_id",
            "stopId",
            "stop_code",
            "stopCode",
            "stop_name",
            "stopName",
            "tts_stop_name",
            "ttsStopName",
            "stop_desc",
            "stopDesc",
            "stop_lat",
            "stopLat",
            "stop_lon",
            "stopLon",
            "zone_id",
            "zoneId",
            "stop_url",
            "stopUrl",
            "parent_station",
            "parentStation",
            "stop_timezone",
            "stopTimezone",
            "wheelchair_boarding",
            "wheelchairBoarding",
            "level_id",
            "levelId",
            "platform_code",
            "platformCode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StopId,
            StopCode,
            StopName,
            TtsStopName,
            StopDesc,
            StopLat,
            StopLon,
            ZoneId,
            StopUrl,
            ParentStation,
            StopTimezone,
            WheelchairBoarding,
            LevelId,
            PlatformCode,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "stopId" | "stop_id" => Ok(GeneratedField::StopId),
                            "stopCode" | "stop_code" => Ok(GeneratedField::StopCode),
                            "stopName" | "stop_name" => Ok(GeneratedField::StopName),
                            "ttsStopName" | "tts_stop_name" => Ok(GeneratedField::TtsStopName),
                            "stopDesc" | "stop_desc" => Ok(GeneratedField::StopDesc),
                            "stopLat" | "stop_lat" => Ok(GeneratedField::StopLat),
                            "stopLon" | "stop_lon" => Ok(GeneratedField::StopLon),
                            "zoneId" | "zone_id" => Ok(GeneratedField::ZoneId),
                            "stopUrl" | "stop_url" => Ok(GeneratedField::StopUrl),
                            "parentStation" | "parent_station" => Ok(GeneratedField::ParentStation),
                            "stopTimezone" | "stop_timezone" => Ok(GeneratedField::StopTimezone),
                            "wheelchairBoarding" | "wheelchair_boarding" => Ok(GeneratedField::WheelchairBoarding),
                            "levelId" | "level_id" => Ok(GeneratedField::LevelId),
                            "platformCode" | "platform_code" => Ok(GeneratedField::PlatformCode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Stop;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct transit_realtime.Stop")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Stop, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stop_id__ = None;
                let mut stop_code__ = None;
                let mut stop_name__ = None;
                let mut tts_stop_name__ = None;
                let mut stop_desc__ = None;
                let mut stop_lat__ = None;
                let mut stop_lon__ = None;
                let mut zone_id__ = None;
                let mut stop_url__ = None;
                let mut parent_station__ = None;
                let mut stop_timezone__ = None;
                let mut wheelchair_boarding__ = None;
                let mut level_id__ = None;
                let mut platform_code__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StopId => {
                            if stop_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stopId"));
                            }
                            stop_id__ = map_.next_value()?;
                        }
                        GeneratedField::StopCode => {
                            if stop_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stopCode"));
                            }
                            stop_code__ = map_.next_value()?;
                        }
                        GeneratedField::StopName => {
                            if stop_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stopName"));
                            }
                            stop_name__ = map_.next_value()?;
                        }
                        GeneratedField::TtsStopName => {
                            if tts_stop_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ttsStopName"));
                            }
                            tts_stop_name__ = map_.next_value()?;
                        }
                        GeneratedField::StopDesc => {
                            if stop_desc__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stopDesc"));
                            }
                            stop_desc__ = map_.next_value()?;
                        }
                        GeneratedField::StopLat => {
                            if stop_lat__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stopLat"));
                            }
                            stop_lat__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::StopLon => {
                            if stop_lon__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stopLon"));
                            }
                            stop_lon__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::ZoneId => {
                            if zone_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("zoneId"));
                            }
                            zone_id__ = map_.next_value()?;
                        }
                        GeneratedField::StopUrl => {
                            if stop_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stopUrl"));
                            }
                            stop_url__ = map_.next_value()?;
                        }
                        GeneratedField::ParentStation => {
                            if parent_station__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parentStation"));
                            }
                            parent_station__ = map_.next_value()?;
                        }
                        GeneratedField::StopTimezone => {
                            if stop_timezone__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stopTimezone"));
                            }
                            stop_timezone__ = map_.next_value()?;
                        }
                        GeneratedField::WheelchairBoarding => {
                            if wheelchair_boarding__.is_some() {
                                return Err(serde::de::Error::duplicate_field("wheelchairBoarding"));
                            }
                            wheelchair_boarding__ = map_.next_value::<::std::option::Option<stop::WheelchairBoarding>>()?.map(|x| x as i32);
                        }
                        GeneratedField::LevelId => {
                            if level_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("levelId"));
                            }
                            level_id__ = map_.next_value()?;
                        }
                        GeneratedField::PlatformCode => {
                            if platform_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("platformCode"));
                            }
                            platform_code__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Stop {
                    stop_id: stop_id__,
                    stop_code: stop_code__,
                    stop_name: stop_name__,
                    tts_stop_name: tts_stop_name__,
                    stop_desc: stop_desc__,
                    stop_lat: stop_lat__,
                    stop_lon: stop_lon__,
                    zone_id: zone_id__,
                    stop_url: stop_url__,
                    parent_station: parent_station__,
                    stop_timezone: stop_timezone__,
                    wheelchair_boarding: wheelchair_boarding__,
                    level_id: level_id__,
                    platform_code: platform_code__,
                })
            }
        }
        deserializer.deserialize_struct("transit_realtime.Stop", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for stop::WheelchairBoarding {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "UNKNOWN",
            Self::Available => "AVAILABLE",
            Self::NotAvailable => "NOT_AVAILABLE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for stop::WheelchairBoarding {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNKNOWN",
            "AVAILABLE",
            "NOT_AVAILABLE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = stop::WheelchairBoarding;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "UNKNOWN" => Ok(stop::WheelchairBoarding::Unknown),
                    "AVAILABLE" => Ok(stop::WheelchairBoarding::Available),
                    "NOT_AVAILABLE" => Ok(stop::WheelchairBoarding::NotAvailable),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for StopSelector {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.stop_sequence.is_some() {
            len += 1;
        }
        if self.stop_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("transit_realtime.StopSelector", len)?;
        if let Some(v) = self.stop_sequence.as_ref() {
            struct_ser.serialize_field("stopSequence", v)?;
        }
        if let Some(v) = self.stop_id.as_ref() {
            struct_ser.serialize_field("stopId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StopSelector {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stop_sequence",
            "stopSequence",
            "stop_id",
            "stopId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StopSequence,
            StopId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "stopSequence" | "stop_sequence" => Ok(GeneratedField::StopSequence),
                            "stopId" | "stop_id" => Ok(GeneratedField::StopId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StopSelector;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct transit_realtime.StopSelector")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StopSelector, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stop_sequence__ = None;
                let mut stop_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StopSequence => {
                            if stop_sequence__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stopSequence"));
                            }
                            stop_sequence__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::StopId => {
                            if stop_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stopId"));
                            }
                            stop_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(StopSelector {
                    stop_sequence: stop_sequence__,
                    stop_id: stop_id__,
                })
            }
        }
        deserializer.deserialize_struct("transit_realtime.StopSelector", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TfnswVehicleDescriptor {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.air_conditioned.is_some() {
            len += 1;
        }
        if self.wheelchair_accessible.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("transit_realtime.TfnswVehicleDescriptor", len)?;
        if let Some(v) = self.air_conditioned.as_ref() {
            struct_ser.serialize_field("airConditioned", v)?;
        }
        if let Some(v) = self.wheelchair_accessible.as_ref() {
            struct_ser.serialize_field("wheelchairAccessible", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TfnswVehicleDescriptor {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "air_conditioned",
            "airConditioned",
            "wheelchair_accessible",
            "wheelchairAccessible",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AirConditioned,
            WheelchairAccessible,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "airConditioned" | "air_conditioned" => Ok(GeneratedField::AirConditioned),
                            "wheelchairAccessible" | "wheelchair_accessible" => Ok(GeneratedField::WheelchairAccessible),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TfnswVehicleDescriptor;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct transit_realtime.TfnswVehicleDescriptor")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TfnswVehicleDescriptor, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut air_conditioned__ = None;
                let mut wheelchair_accessible__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AirConditioned => {
                            if air_conditioned__.is_some() {
                                return Err(serde::de::Error::duplicate_field("airConditioned"));
                            }
                            air_conditioned__ = map_.next_value()?;
                        }
                        GeneratedField::WheelchairAccessible => {
                            if wheelchair_accessible__.is_some() {
                                return Err(serde::de::Error::duplicate_field("wheelchairAccessible"));
                            }
                            wheelchair_accessible__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(TfnswVehicleDescriptor {
                    air_conditioned: air_conditioned__,
                    wheelchair_accessible: wheelchair_accessible__,
                })
            }
        }
        deserializer.deserialize_struct("transit_realtime.TfnswVehicleDescriptor", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TimeRange {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.start.is_some() {
            len += 1;
        }
        if self.end.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("transit_realtime.TimeRange", len)?;
        if let Some(v) = self.start.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("start", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.end.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("end", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TimeRange {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "start",
            "end",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Start,
            End,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "start" => Ok(GeneratedField::Start),
                            "end" => Ok(GeneratedField::End),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TimeRange;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct transit_realtime.TimeRange")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TimeRange, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut start__ = None;
                let mut end__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Start => {
                            if start__.is_some() {
                                return Err(serde::de::Error::duplicate_field("start"));
                            }
                            start__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::End => {
                            if end__.is_some() {
                                return Err(serde::de::Error::duplicate_field("end"));
                            }
                            end__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(TimeRange {
                    start: start__,
                    end: end__,
                })
            }
        }
        deserializer.deserialize_struct("transit_realtime.TimeRange", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TransitAlertExtension {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 1;
        if self.source_name.is_some() {
            len += 1;
        }
        if self.is_service_change_alert.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("transit_realtime.TransitAlertExtension", len)?;
        #[allow(clippy::needless_borrow)]
        struct_ser.serialize_field("createdAt", ToString::to_string(&self.created_at).as_str())?;
        if let Some(v) = self.source_name.as_ref() {
            struct_ser.serialize_field("sourceName", v)?;
        }
        if let Some(v) = self.is_service_change_alert.as_ref() {
            struct_ser.serialize_field("isServiceChangeAlert", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TransitAlertExtension {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "created_at",
            "createdAt",
            "source_name",
            "sourceName",
            "is_service_change_alert",
            "isServiceChangeAlert",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CreatedAt,
            SourceName,
            IsServiceChangeAlert,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "sourceName" | "source_name" => Ok(GeneratedField::SourceName),
                            "isServiceChangeAlert" | "is_service_change_alert" => Ok(GeneratedField::IsServiceChangeAlert),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TransitAlertExtension;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct transit_realtime.TransitAlertExtension")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TransitAlertExtension, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut created_at__ = None;
                let mut source_name__ = None;
                let mut is_service_change_alert__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SourceName => {
                            if source_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceName"));
                            }
                            source_name__ = map_.next_value()?;
                        }
                        GeneratedField::IsServiceChangeAlert => {
                            if is_service_change_alert__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isServiceChangeAlert"));
                            }
                            is_service_change_alert__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TransitAlertExtension {
                    created_at: created_at__.ok_or_else(|| serde::de::Error::missing_field("createdAt"))?,
                    source_name: source_name__,
                    is_service_change_alert: is_service_change_alert__,
                })
            }
        }
        deserializer.deserialize_struct("transit_realtime.TransitAlertExtension", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TransitInformedEntityExtension {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.feed_id.is_some() {
            len += 1;
        }
        if self.global_route_id.is_some() {
            len += 1;
        }
        if self.stable_stop_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("transit_realtime.TransitInformedEntityExtension", len)?;
        if let Some(v) = self.feed_id.as_ref() {
            struct_ser.serialize_field("feedId", v)?;
        }
        if let Some(v) = self.global_route_id.as_ref() {
            struct_ser.serialize_field("globalRouteId", v)?;
        }
        if let Some(v) = self.stable_stop_id.as_ref() {
            struct_ser.serialize_field("stableStopId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TransitInformedEntityExtension {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "feed_id",
            "feedId",
            "global_route_id",
            "globalRouteId",
            "stable_stop_id",
            "stableStopId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FeedId,
            GlobalRouteId,
            StableStopId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "feedId" | "feed_id" => Ok(GeneratedField::FeedId),
                            "globalRouteId" | "global_route_id" => Ok(GeneratedField::GlobalRouteId),
                            "stableStopId" | "stable_stop_id" => Ok(GeneratedField::StableStopId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TransitInformedEntityExtension;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct transit_realtime.TransitInformedEntityExtension")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TransitInformedEntityExtension, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut feed_id__ = None;
                let mut global_route_id__ = None;
                let mut stable_stop_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FeedId => {
                            if feed_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feedId"));
                            }
                            feed_id__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::GlobalRouteId => {
                            if global_route_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("globalRouteId"));
                            }
                            global_route_id__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::StableStopId => {
                            if stable_stop_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stableStopId"));
                            }
                            stable_stop_id__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(TransitInformedEntityExtension {
                    feed_id: feed_id__,
                    global_route_id: global_route_id__,
                    stable_stop_id: stable_stop_id__,
                })
            }
        }
        deserializer.deserialize_struct("transit_realtime.TransitInformedEntityExtension", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TransitModificationExtension {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.trip_ids.is_empty() {
            len += 1;
        }
        if self.detection_time.is_some() {
            len += 1;
        }
        if self.modification_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("transit_realtime.TransitModificationExtension", len)?;
        if !self.trip_ids.is_empty() {
            struct_ser.serialize_field("tripIds", &self.trip_ids)?;
        }
        if let Some(v) = self.detection_time.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("detectionTime", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.modification_id.as_ref() {
            struct_ser.serialize_field("modificationId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TransitModificationExtension {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "trip_ids",
            "tripIds",
            "detection_time",
            "detectionTime",
            "modification_id",
            "modificationId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TripIds,
            DetectionTime,
            ModificationId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tripIds" | "trip_ids" => Ok(GeneratedField::TripIds),
                            "detectionTime" | "detection_time" => Ok(GeneratedField::DetectionTime),
                            "modificationId" | "modification_id" => Ok(GeneratedField::ModificationId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TransitModificationExtension;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct transit_realtime.TransitModificationExtension")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TransitModificationExtension, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut trip_ids__ = None;
                let mut detection_time__ = None;
                let mut modification_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TripIds => {
                            if trip_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tripIds"));
                            }
                            trip_ids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DetectionTime => {
                            if detection_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("detectionTime"));
                            }
                            detection_time__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::ModificationId => {
                            if modification_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("modificationId"));
                            }
                            modification_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TransitModificationExtension {
                    trip_ids: trip_ids__.unwrap_or_default(),
                    detection_time: detection_time__,
                    modification_id: modification_id__,
                })
            }
        }
        deserializer.deserialize_struct("transit_realtime.TransitModificationExtension", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TransitReplacementStopExtension {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.no_through_travel.is_some() {
            len += 1;
        }
        if self.next_shape_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("transit_realtime.TransitReplacementStopExtension", len)?;
        if let Some(v) = self.no_through_travel.as_ref() {
            struct_ser.serialize_field("noThroughTravel", v)?;
        }
        if let Some(v) = self.next_shape_id.as_ref() {
            struct_ser.serialize_field("nextShapeId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TransitReplacementStopExtension {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "no_through_travel",
            "noThroughTravel",
            "next_shape_id",
            "nextShapeId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NoThroughTravel,
            NextShapeId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "noThroughTravel" | "no_through_travel" => Ok(GeneratedField::NoThroughTravel),
                            "nextShapeId" | "next_shape_id" => Ok(GeneratedField::NextShapeId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TransitReplacementStopExtension;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct transit_realtime.TransitReplacementStopExtension")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TransitReplacementStopExtension, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut no_through_travel__ = None;
                let mut next_shape_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NoThroughTravel => {
                            if no_through_travel__.is_some() {
                                return Err(serde::de::Error::duplicate_field("noThroughTravel"));
                            }
                            no_through_travel__ = map_.next_value()?;
                        }
                        GeneratedField::NextShapeId => {
                            if next_shape_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextShapeId"));
                            }
                            next_shape_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TransitReplacementStopExtension {
                    no_through_travel: no_through_travel__,
                    next_shape_id: next_shape_id__,
                })
            }
        }
        deserializer.deserialize_struct("transit_realtime.TransitReplacementStopExtension", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TransitTripDescriptorExtension {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.origin_prediction.is_some() {
            len += 1;
        }
        if self.prediction_made_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("transit_realtime.TransitTripDescriptorExtension", len)?;
        if let Some(v) = self.origin_prediction.as_ref() {
            let v = transit_trip_descriptor_extension::OriginPrediction::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("originPrediction", &v)?;
        }
        if let Some(v) = self.prediction_made_at.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("predictionMadeAt", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TransitTripDescriptorExtension {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "origin_prediction",
            "originPrediction",
            "prediction_made_at",
            "predictionMadeAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OriginPrediction,
            PredictionMadeAt,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "originPrediction" | "origin_prediction" => Ok(GeneratedField::OriginPrediction),
                            "predictionMadeAt" | "prediction_made_at" => Ok(GeneratedField::PredictionMadeAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TransitTripDescriptorExtension;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct transit_realtime.TransitTripDescriptorExtension")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TransitTripDescriptorExtension, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut origin_prediction__ = None;
                let mut prediction_made_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OriginPrediction => {
                            if origin_prediction__.is_some() {
                                return Err(serde::de::Error::duplicate_field("originPrediction"));
                            }
                            origin_prediction__ = map_.next_value::<::std::option::Option<transit_trip_descriptor_extension::OriginPrediction>>()?.map(|x| x as i32);
                        }
                        GeneratedField::PredictionMadeAt => {
                            if prediction_made_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("predictionMadeAt"));
                            }
                            prediction_made_at__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(TransitTripDescriptorExtension {
                    origin_prediction: origin_prediction__,
                    prediction_made_at: prediction_made_at__,
                })
            }
        }
        deserializer.deserialize_struct("transit_realtime.TransitTripDescriptorExtension", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for transit_trip_descriptor_extension::OriginPrediction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::AgencyOriginalData => "AGENCY_ORIGINAL_DATA",
            Self::TransitPredictionEngine => "TRANSIT_PREDICTION_ENGINE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for transit_trip_descriptor_extension::OriginPrediction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "AGENCY_ORIGINAL_DATA",
            "TRANSIT_PREDICTION_ENGINE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = transit_trip_descriptor_extension::OriginPrediction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "AGENCY_ORIGINAL_DATA" => Ok(transit_trip_descriptor_extension::OriginPrediction::AgencyOriginalData),
                    "TRANSIT_PREDICTION_ENGINE" => Ok(transit_trip_descriptor_extension::OriginPrediction::TransitPredictionEngine),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for TransitVehicleDescriptorExtension {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.based_on_crowdsourcing_data.is_some() {
            len += 1;
        }
        if self.transit_vehicle_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("transit_realtime.TransitVehicleDescriptorExtension", len)?;
        if let Some(v) = self.based_on_crowdsourcing_data.as_ref() {
            struct_ser.serialize_field("basedOnCrowdsourcingData", v)?;
        }
        if let Some(v) = self.transit_vehicle_id.as_ref() {
            struct_ser.serialize_field("transitVehicleId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TransitVehicleDescriptorExtension {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "based_on_crowdsourcing_data",
            "basedOnCrowdsourcingData",
            "transit_vehicle_id",
            "transitVehicleId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BasedOnCrowdsourcingData,
            TransitVehicleId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "basedOnCrowdsourcingData" | "based_on_crowdsourcing_data" => Ok(GeneratedField::BasedOnCrowdsourcingData),
                            "transitVehicleId" | "transit_vehicle_id" => Ok(GeneratedField::TransitVehicleId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TransitVehicleDescriptorExtension;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct transit_realtime.TransitVehicleDescriptorExtension")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TransitVehicleDescriptorExtension, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut based_on_crowdsourcing_data__ = None;
                let mut transit_vehicle_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BasedOnCrowdsourcingData => {
                            if based_on_crowdsourcing_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("basedOnCrowdsourcingData"));
                            }
                            based_on_crowdsourcing_data__ = map_.next_value()?;
                        }
                        GeneratedField::TransitVehicleId => {
                            if transit_vehicle_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transitVehicleId"));
                            }
                            transit_vehicle_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TransitVehicleDescriptorExtension {
                    based_on_crowdsourcing_data: based_on_crowdsourcing_data__,
                    transit_vehicle_id: transit_vehicle_id__,
                })
            }
        }
        deserializer.deserialize_struct("transit_realtime.TransitVehicleDescriptorExtension", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TranslatedImage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.localized_image.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("transit_realtime.TranslatedImage", len)?;
        if !self.localized_image.is_empty() {
            struct_ser.serialize_field("localizedImage", &self.localized_image)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TranslatedImage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "localized_image",
            "localizedImage",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LocalizedImage,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "localizedImage" | "localized_image" => Ok(GeneratedField::LocalizedImage),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TranslatedImage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct transit_realtime.TranslatedImage")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TranslatedImage, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut localized_image__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LocalizedImage => {
                            if localized_image__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localizedImage"));
                            }
                            localized_image__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TranslatedImage {
                    localized_image: localized_image__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("transit_realtime.TranslatedImage", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for translated_image::LocalizedImage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 2;
        if self.language.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("transit_realtime.TranslatedImage.LocalizedImage", len)?;
        struct_ser.serialize_field("url", &self.url)?;
        struct_ser.serialize_field("mediaType", &self.media_type)?;
        if let Some(v) = self.language.as_ref() {
            struct_ser.serialize_field("language", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for translated_image::LocalizedImage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "url",
            "media_type",
            "mediaType",
            "language",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Url,
            MediaType,
            Language,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "url" => Ok(GeneratedField::Url),
                            "mediaType" | "media_type" => Ok(GeneratedField::MediaType),
                            "language" => Ok(GeneratedField::Language),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = translated_image::LocalizedImage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct transit_realtime.TranslatedImage.LocalizedImage")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<translated_image::LocalizedImage, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut url__ = None;
                let mut media_type__ = None;
                let mut language__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Url => {
                            if url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MediaType => {
                            if media_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mediaType"));
                            }
                            media_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Language => {
                            if language__.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            language__ = map_.next_value()?;
                        }
                    }
                }
                Ok(translated_image::LocalizedImage {
                    url: url__.ok_or_else(|| serde::de::Error::missing_field("url"))?,
                    media_type: media_type__.ok_or_else(|| serde::de::Error::missing_field("mediaType"))?,
                    language: language__,
                })
            }
        }
        deserializer.deserialize_struct("transit_realtime.TranslatedImage.LocalizedImage", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TranslatedString {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.translation.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("transit_realtime.TranslatedString", len)?;
        if !self.translation.is_empty() {
            struct_ser.serialize_field("translation", &self.translation)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TranslatedString {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "translation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Translation,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "translation" => Ok(GeneratedField::Translation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TranslatedString;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct transit_realtime.TranslatedString")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TranslatedString, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut translation__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Translation => {
                            if translation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("translation"));
                            }
                            translation__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TranslatedString {
                    translation: translation__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("transit_realtime.TranslatedString", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for translated_string::Translation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 1;
        if self.language.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("transit_realtime.TranslatedString.Translation", len)?;
        struct_ser.serialize_field("text", &self.text)?;
        if let Some(v) = self.language.as_ref() {
            struct_ser.serialize_field("language", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for translated_string::Translation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "text",
            "language",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Text,
            Language,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "text" => Ok(GeneratedField::Text),
                            "language" => Ok(GeneratedField::Language),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = translated_string::Translation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct transit_realtime.TranslatedString.Translation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<translated_string::Translation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut text__ = None;
                let mut language__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Text => {
                            if text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Language => {
                            if language__.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            language__ = map_.next_value()?;
                        }
                    }
                }
                Ok(translated_string::Translation {
                    text: text__.ok_or_else(|| serde::de::Error::missing_field("text"))?,
                    language: language__,
                })
            }
        }
        deserializer.deserialize_struct("transit_realtime.TranslatedString.Translation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TripDescriptor {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.trip_id.is_some() {
            len += 1;
        }
        if self.route_id.is_some() {
            len += 1;
        }
        if self.direction_id.is_some() {
            len += 1;
        }
        if self.start_time.is_some() {
            len += 1;
        }
        if self.start_date.is_some() {
            len += 1;
        }
        if self.schedule_relationship.is_some() {
            len += 1;
        }
        if self.modified_trip.is_some() {
            len += 1;
        }
        if self.transit_trip_descriptor_extension.is_some() {
            len += 1;
        }
        if self.nyct_trip_descriptor.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("transit_realtime.TripDescriptor", len)?;
        if let Some(v) = self.trip_id.as_ref() {
            struct_ser.serialize_field("tripId", v)?;
        }
        if let Some(v) = self.route_id.as_ref() {
            struct_ser.serialize_field("routeId", v)?;
        }
        if let Some(v) = self.direction_id.as_ref() {
            struct_ser.serialize_field("directionId", v)?;
        }
        if let Some(v) = self.start_time.as_ref() {
            struct_ser.serialize_field("startTime", v)?;
        }
        if let Some(v) = self.start_date.as_ref() {
            struct_ser.serialize_field("startDate", v)?;
        }
        if let Some(v) = self.schedule_relationship.as_ref() {
            let v = trip_descriptor::ScheduleRelationship::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("scheduleRelationship", &v)?;
        }
        if let Some(v) = self.modified_trip.as_ref() {
            struct_ser.serialize_field("modifiedTrip", v)?;
        }
        if let Some(v) = self.transit_trip_descriptor_extension.as_ref() {
            struct_ser.serialize_field("transitTripDescriptorExtension", v)?;
        }
        if let Some(v) = self.nyct_trip_descriptor.as_ref() {
            struct_ser.serialize_field("nyctTripDescriptor", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TripDescriptor {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "trip_id",
            "tripId",
            "route_id",
            "routeId",
            "direction_id",
            "directionId",
            "start_time",
            "startTime",
            "start_date",
            "startDate",
            "schedule_relationship",
            "scheduleRelationship",
            "modified_trip",
            "modifiedTrip",
            "transit_trip_descriptor_extension",
            "transitTripDescriptorExtension",
            "nyct_trip_descriptor",
            "nyctTripDescriptor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TripId,
            RouteId,
            DirectionId,
            StartTime,
            StartDate,
            ScheduleRelationship,
            ModifiedTrip,
            TransitTripDescriptorExtension,
            NyctTripDescriptor,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tripId" | "trip_id" => Ok(GeneratedField::TripId),
                            "routeId" | "route_id" => Ok(GeneratedField::RouteId),
                            "directionId" | "direction_id" => Ok(GeneratedField::DirectionId),
                            "startTime" | "start_time" => Ok(GeneratedField::StartTime),
                            "startDate" | "start_date" => Ok(GeneratedField::StartDate),
                            "scheduleRelationship" | "schedule_relationship" => Ok(GeneratedField::ScheduleRelationship),
                            "modifiedTrip" | "modified_trip" => Ok(GeneratedField::ModifiedTrip),
                            "transitTripDescriptorExtension" | "transit_trip_descriptor_extension" => Ok(GeneratedField::TransitTripDescriptorExtension),
                            "nyctTripDescriptor" | "nyct_trip_descriptor" => Ok(GeneratedField::NyctTripDescriptor),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TripDescriptor;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct transit_realtime.TripDescriptor")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TripDescriptor, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut trip_id__ = None;
                let mut route_id__ = None;
                let mut direction_id__ = None;
                let mut start_time__ = None;
                let mut start_date__ = None;
                let mut schedule_relationship__ = None;
                let mut modified_trip__ = None;
                let mut transit_trip_descriptor_extension__ = None;
                let mut nyct_trip_descriptor__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TripId => {
                            if trip_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tripId"));
                            }
                            trip_id__ = map_.next_value()?;
                        }
                        GeneratedField::RouteId => {
                            if route_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routeId"));
                            }
                            route_id__ = map_.next_value()?;
                        }
                        GeneratedField::DirectionId => {
                            if direction_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("directionId"));
                            }
                            direction_id__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::StartTime => {
                            if start_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startTime"));
                            }
                            start_time__ = map_.next_value()?;
                        }
                        GeneratedField::StartDate => {
                            if start_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startDate"));
                            }
                            start_date__ = map_.next_value()?;
                        }
                        GeneratedField::ScheduleRelationship => {
                            if schedule_relationship__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scheduleRelationship"));
                            }
                            schedule_relationship__ = map_.next_value::<::std::option::Option<trip_descriptor::ScheduleRelationship>>()?.map(|x| x as i32);
                        }
                        GeneratedField::ModifiedTrip => {
                            if modified_trip__.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifiedTrip"));
                            }
                            modified_trip__ = map_.next_value()?;
                        }
                        GeneratedField::TransitTripDescriptorExtension => {
                            if transit_trip_descriptor_extension__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transitTripDescriptorExtension"));
                            }
                            transit_trip_descriptor_extension__ = map_.next_value()?;
                        }
                        GeneratedField::NyctTripDescriptor => {
                            if nyct_trip_descriptor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nyctTripDescriptor"));
                            }
                            nyct_trip_descriptor__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TripDescriptor {
                    trip_id: trip_id__,
                    route_id: route_id__,
                    direction_id: direction_id__,
                    start_time: start_time__,
                    start_date: start_date__,
                    schedule_relationship: schedule_relationship__,
                    modified_trip: modified_trip__,
                    transit_trip_descriptor_extension: transit_trip_descriptor_extension__,
                    nyct_trip_descriptor: nyct_trip_descriptor__,
                })
            }
        }
        deserializer.deserialize_struct("transit_realtime.TripDescriptor", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for trip_descriptor::ModifiedTripSelector {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.modifications_id.is_some() {
            len += 1;
        }
        if self.affected_trip_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("transit_realtime.TripDescriptor.ModifiedTripSelector", len)?;
        if let Some(v) = self.modifications_id.as_ref() {
            struct_ser.serialize_field("modificationsId", v)?;
        }
        if let Some(v) = self.affected_trip_id.as_ref() {
            struct_ser.serialize_field("affectedTripId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for trip_descriptor::ModifiedTripSelector {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "modifications_id",
            "modificationsId",
            "affected_trip_id",
            "affectedTripId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ModificationsId,
            AffectedTripId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "modificationsId" | "modifications_id" => Ok(GeneratedField::ModificationsId),
                            "affectedTripId" | "affected_trip_id" => Ok(GeneratedField::AffectedTripId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = trip_descriptor::ModifiedTripSelector;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct transit_realtime.TripDescriptor.ModifiedTripSelector")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<trip_descriptor::ModifiedTripSelector, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut modifications_id__ = None;
                let mut affected_trip_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ModificationsId => {
                            if modifications_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("modificationsId"));
                            }
                            modifications_id__ = map_.next_value()?;
                        }
                        GeneratedField::AffectedTripId => {
                            if affected_trip_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("affectedTripId"));
                            }
                            affected_trip_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(trip_descriptor::ModifiedTripSelector {
                    modifications_id: modifications_id__,
                    affected_trip_id: affected_trip_id__,
                })
            }
        }
        deserializer.deserialize_struct("transit_realtime.TripDescriptor.ModifiedTripSelector", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for trip_descriptor::ScheduleRelationship {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Scheduled => "SCHEDULED",
            Self::Added => "ADDED",
            Self::Unscheduled => "UNSCHEDULED",
            Self::Canceled => "CANCELED",
            Self::Replacement => "REPLACEMENT",
            Self::Duplicated => "DUPLICATED",
            Self::Deleted => "DELETED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for trip_descriptor::ScheduleRelationship {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SCHEDULED",
            "ADDED",
            "UNSCHEDULED",
            "CANCELED",
            "REPLACEMENT",
            "DUPLICATED",
            "DELETED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = trip_descriptor::ScheduleRelationship;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "SCHEDULED" => Ok(trip_descriptor::ScheduleRelationship::Scheduled),
                    "ADDED" => Ok(trip_descriptor::ScheduleRelationship::Added),
                    "UNSCHEDULED" => Ok(trip_descriptor::ScheduleRelationship::Unscheduled),
                    "CANCELED" => Ok(trip_descriptor::ScheduleRelationship::Canceled),
                    "REPLACEMENT" => Ok(trip_descriptor::ScheduleRelationship::Replacement),
                    "DUPLICATED" => Ok(trip_descriptor::ScheduleRelationship::Duplicated),
                    "DELETED" => Ok(trip_descriptor::ScheduleRelationship::Deleted),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for TripModifications {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.selected_trips.is_empty() {
            len += 1;
        }
        if !self.start_times.is_empty() {
            len += 1;
        }
        if !self.service_dates.is_empty() {
            len += 1;
        }
        if !self.modifications.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("transit_realtime.TripModifications", len)?;
        if !self.selected_trips.is_empty() {
            struct_ser.serialize_field("selectedTrips", &self.selected_trips)?;
        }
        if !self.start_times.is_empty() {
            struct_ser.serialize_field("startTimes", &self.start_times)?;
        }
        if !self.service_dates.is_empty() {
            struct_ser.serialize_field("serviceDates", &self.service_dates)?;
        }
        if !self.modifications.is_empty() {
            struct_ser.serialize_field("modifications", &self.modifications)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TripModifications {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "selected_trips",
            "selectedTrips",
            "start_times",
            "startTimes",
            "service_dates",
            "serviceDates",
            "modifications",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SelectedTrips,
            StartTimes,
            ServiceDates,
            Modifications,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "selectedTrips" | "selected_trips" => Ok(GeneratedField::SelectedTrips),
                            "startTimes" | "start_times" => Ok(GeneratedField::StartTimes),
                            "serviceDates" | "service_dates" => Ok(GeneratedField::ServiceDates),
                            "modifications" => Ok(GeneratedField::Modifications),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TripModifications;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct transit_realtime.TripModifications")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TripModifications, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut selected_trips__ = None;
                let mut start_times__ = None;
                let mut service_dates__ = None;
                let mut modifications__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SelectedTrips => {
                            if selected_trips__.is_some() {
                                return Err(serde::de::Error::duplicate_field("selectedTrips"));
                            }
                            selected_trips__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StartTimes => {
                            if start_times__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startTimes"));
                            }
                            start_times__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ServiceDates => {
                            if service_dates__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceDates"));
                            }
                            service_dates__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Modifications => {
                            if modifications__.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifications"));
                            }
                            modifications__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TripModifications {
                    selected_trips: selected_trips__.unwrap_or_default(),
                    start_times: start_times__.unwrap_or_default(),
                    service_dates: service_dates__.unwrap_or_default(),
                    modifications: modifications__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("transit_realtime.TripModifications", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for trip_modifications::Modification {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.start_stop_selector.is_some() {
            len += 1;
        }
        if self.end_stop_selector.is_some() {
            len += 1;
        }
        if self.propagated_modification_delay.is_some() {
            len += 1;
        }
        if !self.replacement_stops.is_empty() {
            len += 1;
        }
        if self.service_alert_id.is_some() {
            len += 1;
        }
        if self.last_modified_time.is_some() {
            len += 1;
        }
        if self.transit_modification_extension.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("transit_realtime.TripModifications.Modification", len)?;
        if let Some(v) = self.start_stop_selector.as_ref() {
            struct_ser.serialize_field("startStopSelector", v)?;
        }
        if let Some(v) = self.end_stop_selector.as_ref() {
            struct_ser.serialize_field("endStopSelector", v)?;
        }
        if let Some(v) = self.propagated_modification_delay.as_ref() {
            struct_ser.serialize_field("propagatedModificationDelay", v)?;
        }
        if !self.replacement_stops.is_empty() {
            struct_ser.serialize_field("replacementStops", &self.replacement_stops)?;
        }
        if let Some(v) = self.service_alert_id.as_ref() {
            struct_ser.serialize_field("serviceAlertId", v)?;
        }
        if let Some(v) = self.last_modified_time.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("lastModifiedTime", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.transit_modification_extension.as_ref() {
            struct_ser.serialize_field("transitModificationExtension", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for trip_modifications::Modification {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "start_stop_selector",
            "startStopSelector",
            "end_stop_selector",
            "endStopSelector",
            "propagated_modification_delay",
            "propagatedModificationDelay",
            "replacement_stops",
            "replacementStops",
            "service_alert_id",
            "serviceAlertId",
            "last_modified_time",
            "lastModifiedTime",
            "transit_modification_extension",
            "transitModificationExtension",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StartStopSelector,
            EndStopSelector,
            PropagatedModificationDelay,
            ReplacementStops,
            ServiceAlertId,
            LastModifiedTime,
            TransitModificationExtension,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "startStopSelector" | "start_stop_selector" => Ok(GeneratedField::StartStopSelector),
                            "endStopSelector" | "end_stop_selector" => Ok(GeneratedField::EndStopSelector),
                            "propagatedModificationDelay" | "propagated_modification_delay" => Ok(GeneratedField::PropagatedModificationDelay),
                            "replacementStops" | "replacement_stops" => Ok(GeneratedField::ReplacementStops),
                            "serviceAlertId" | "service_alert_id" => Ok(GeneratedField::ServiceAlertId),
                            "lastModifiedTime" | "last_modified_time" => Ok(GeneratedField::LastModifiedTime),
                            "transitModificationExtension" | "transit_modification_extension" => Ok(GeneratedField::TransitModificationExtension),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = trip_modifications::Modification;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct transit_realtime.TripModifications.Modification")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<trip_modifications::Modification, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut start_stop_selector__ = None;
                let mut end_stop_selector__ = None;
                let mut propagated_modification_delay__ = None;
                let mut replacement_stops__ = None;
                let mut service_alert_id__ = None;
                let mut last_modified_time__ = None;
                let mut transit_modification_extension__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StartStopSelector => {
                            if start_stop_selector__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startStopSelector"));
                            }
                            start_stop_selector__ = map_.next_value()?;
                        }
                        GeneratedField::EndStopSelector => {
                            if end_stop_selector__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endStopSelector"));
                            }
                            end_stop_selector__ = map_.next_value()?;
                        }
                        GeneratedField::PropagatedModificationDelay => {
                            if propagated_modification_delay__.is_some() {
                                return Err(serde::de::Error::duplicate_field("propagatedModificationDelay"));
                            }
                            propagated_modification_delay__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::ReplacementStops => {
                            if replacement_stops__.is_some() {
                                return Err(serde::de::Error::duplicate_field("replacementStops"));
                            }
                            replacement_stops__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ServiceAlertId => {
                            if service_alert_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceAlertId"));
                            }
                            service_alert_id__ = map_.next_value()?;
                        }
                        GeneratedField::LastModifiedTime => {
                            if last_modified_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastModifiedTime"));
                            }
                            last_modified_time__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::TransitModificationExtension => {
                            if transit_modification_extension__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transitModificationExtension"));
                            }
                            transit_modification_extension__ = map_.next_value()?;
                        }
                    }
                }
                Ok(trip_modifications::Modification {
                    start_stop_selector: start_stop_selector__,
                    end_stop_selector: end_stop_selector__,
                    propagated_modification_delay: propagated_modification_delay__,
                    replacement_stops: replacement_stops__.unwrap_or_default(),
                    service_alert_id: service_alert_id__,
                    last_modified_time: last_modified_time__,
                    transit_modification_extension: transit_modification_extension__,
                })
            }
        }
        deserializer.deserialize_struct("transit_realtime.TripModifications.Modification", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for trip_modifications::SelectedTrips {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.trip_ids.is_empty() {
            len += 1;
        }
        if self.shape_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("transit_realtime.TripModifications.SelectedTrips", len)?;
        if !self.trip_ids.is_empty() {
            struct_ser.serialize_field("tripIds", &self.trip_ids)?;
        }
        if let Some(v) = self.shape_id.as_ref() {
            struct_ser.serialize_field("shapeId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for trip_modifications::SelectedTrips {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "trip_ids",
            "tripIds",
            "shape_id",
            "shapeId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TripIds,
            ShapeId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tripIds" | "trip_ids" => Ok(GeneratedField::TripIds),
                            "shapeId" | "shape_id" => Ok(GeneratedField::ShapeId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = trip_modifications::SelectedTrips;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct transit_realtime.TripModifications.SelectedTrips")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<trip_modifications::SelectedTrips, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut trip_ids__ = None;
                let mut shape_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TripIds => {
                            if trip_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tripIds"));
                            }
                            trip_ids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ShapeId => {
                            if shape_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shapeId"));
                            }
                            shape_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(trip_modifications::SelectedTrips {
                    trip_ids: trip_ids__.unwrap_or_default(),
                    shape_id: shape_id__,
                })
            }
        }
        deserializer.deserialize_struct("transit_realtime.TripModifications.SelectedTrips", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TripReplacementPeriod {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.route_id.is_some() {
            len += 1;
        }
        if self.replacement_period.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("transit_realtime.TripReplacementPeriod", len)?;
        if let Some(v) = self.route_id.as_ref() {
            struct_ser.serialize_field("routeId", v)?;
        }
        if let Some(v) = self.replacement_period.as_ref() {
            struct_ser.serialize_field("replacementPeriod", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TripReplacementPeriod {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "route_id",
            "routeId",
            "replacement_period",
            "replacementPeriod",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RouteId,
            ReplacementPeriod,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "routeId" | "route_id" => Ok(GeneratedField::RouteId),
                            "replacementPeriod" | "replacement_period" => Ok(GeneratedField::ReplacementPeriod),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TripReplacementPeriod;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct transit_realtime.TripReplacementPeriod")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TripReplacementPeriod, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut route_id__ = None;
                let mut replacement_period__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RouteId => {
                            if route_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routeId"));
                            }
                            route_id__ = map_.next_value()?;
                        }
                        GeneratedField::ReplacementPeriod => {
                            if replacement_period__.is_some() {
                                return Err(serde::de::Error::duplicate_field("replacementPeriod"));
                            }
                            replacement_period__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TripReplacementPeriod {
                    route_id: route_id__,
                    replacement_period: replacement_period__,
                })
            }
        }
        deserializer.deserialize_struct("transit_realtime.TripReplacementPeriod", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TripUpdate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 1;
        if self.vehicle.is_some() {
            len += 1;
        }
        if !self.stop_time_update.is_empty() {
            len += 1;
        }
        if self.timestamp.is_some() {
            len += 1;
        }
        if self.delay.is_some() {
            len += 1;
        }
        if self.trip_properties.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("transit_realtime.TripUpdate", len)?;
        struct_ser.serialize_field("trip", &self.trip)?;
        if let Some(v) = self.vehicle.as_ref() {
            struct_ser.serialize_field("vehicle", v)?;
        }
        if !self.stop_time_update.is_empty() {
            struct_ser.serialize_field("stopTimeUpdate", &self.stop_time_update)?;
        }
        if let Some(v) = self.timestamp.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("timestamp", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.delay.as_ref() {
            struct_ser.serialize_field("delay", v)?;
        }
        if let Some(v) = self.trip_properties.as_ref() {
            struct_ser.serialize_field("tripProperties", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TripUpdate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "trip",
            "vehicle",
            "stop_time_update",
            "stopTimeUpdate",
            "timestamp",
            "delay",
            "trip_properties",
            "tripProperties",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Trip,
            Vehicle,
            StopTimeUpdate,
            Timestamp,
            Delay,
            TripProperties,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "trip" => Ok(GeneratedField::Trip),
                            "vehicle" => Ok(GeneratedField::Vehicle),
                            "stopTimeUpdate" | "stop_time_update" => Ok(GeneratedField::StopTimeUpdate),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            "delay" => Ok(GeneratedField::Delay),
                            "tripProperties" | "trip_properties" => Ok(GeneratedField::TripProperties),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TripUpdate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct transit_realtime.TripUpdate")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TripUpdate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut trip__ = None;
                let mut vehicle__ = None;
                let mut stop_time_update__ = None;
                let mut timestamp__ = None;
                let mut delay__ = None;
                let mut trip_properties__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Trip => {
                            if trip__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trip"));
                            }
                            trip__ = map_.next_value()?;
                        }
                        GeneratedField::Vehicle => {
                            if vehicle__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vehicle"));
                            }
                            vehicle__ = map_.next_value()?;
                        }
                        GeneratedField::StopTimeUpdate => {
                            if stop_time_update__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stopTimeUpdate"));
                            }
                            stop_time_update__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Delay => {
                            if delay__.is_some() {
                                return Err(serde::de::Error::duplicate_field("delay"));
                            }
                            delay__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::TripProperties => {
                            if trip_properties__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tripProperties"));
                            }
                            trip_properties__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TripUpdate {
                    trip: trip__.ok_or_else(|| serde::de::Error::missing_field("trip"))?,
                    vehicle: vehicle__,
                    stop_time_update: stop_time_update__.unwrap_or_default(),
                    timestamp: timestamp__,
                    delay: delay__,
                    trip_properties: trip_properties__,
                })
            }
        }
        deserializer.deserialize_struct("transit_realtime.TripUpdate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for trip_update::StopTimeEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.delay.is_some() {
            len += 1;
        }
        if self.time.is_some() {
            len += 1;
        }
        if self.uncertainty.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("transit_realtime.TripUpdate.StopTimeEvent", len)?;
        if let Some(v) = self.delay.as_ref() {
            struct_ser.serialize_field("delay", v)?;
        }
        if let Some(v) = self.time.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("time", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.uncertainty.as_ref() {
            struct_ser.serialize_field("uncertainty", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for trip_update::StopTimeEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "delay",
            "time",
            "uncertainty",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Delay,
            Time,
            Uncertainty,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "delay" => Ok(GeneratedField::Delay),
                            "time" => Ok(GeneratedField::Time),
                            "uncertainty" => Ok(GeneratedField::Uncertainty),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = trip_update::StopTimeEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct transit_realtime.TripUpdate.StopTimeEvent")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<trip_update::StopTimeEvent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut delay__ = None;
                let mut time__ = None;
                let mut uncertainty__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Delay => {
                            if delay__.is_some() {
                                return Err(serde::de::Error::duplicate_field("delay"));
                            }
                            delay__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Time => {
                            if time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("time"));
                            }
                            time__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Uncertainty => {
                            if uncertainty__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uncertainty"));
                            }
                            uncertainty__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(trip_update::StopTimeEvent {
                    delay: delay__,
                    time: time__,
                    uncertainty: uncertainty__,
                })
            }
        }
        deserializer.deserialize_struct("transit_realtime.TripUpdate.StopTimeEvent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for trip_update::StopTimeUpdate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.stop_sequence.is_some() {
            len += 1;
        }
        if self.stop_id.is_some() {
            len += 1;
        }
        if self.arrival.is_some() {
            len += 1;
        }
        if self.departure.is_some() {
            len += 1;
        }
        if self.departure_occupancy_status.is_some() {
            len += 1;
        }
        if self.schedule_relationship.is_some() {
            len += 1;
        }
        if self.stop_time_properties.is_some() {
            len += 1;
        }
        if self.transit_stop_time_update_extension.is_some() {
            len += 1;
        }
        if self.nyct_stop_time_update.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("transit_realtime.TripUpdate.StopTimeUpdate", len)?;
        if let Some(v) = self.stop_sequence.as_ref() {
            struct_ser.serialize_field("stopSequence", v)?;
        }
        if let Some(v) = self.stop_id.as_ref() {
            struct_ser.serialize_field("stopId", v)?;
        }
        if let Some(v) = self.arrival.as_ref() {
            struct_ser.serialize_field("arrival", v)?;
        }
        if let Some(v) = self.departure.as_ref() {
            struct_ser.serialize_field("departure", v)?;
        }
        if let Some(v) = self.departure_occupancy_status.as_ref() {
            let v = vehicle_position::OccupancyStatus::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("departureOccupancyStatus", &v)?;
        }
        if let Some(v) = self.schedule_relationship.as_ref() {
            let v = trip_update::stop_time_update::ScheduleRelationship::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("scheduleRelationship", &v)?;
        }
        if let Some(v) = self.stop_time_properties.as_ref() {
            struct_ser.serialize_field("stopTimeProperties", v)?;
        }
        if let Some(v) = self.transit_stop_time_update_extension.as_ref() {
            struct_ser.serialize_field("transitStopTimeUpdateExtension", v)?;
        }
        if let Some(v) = self.nyct_stop_time_update.as_ref() {
            struct_ser.serialize_field("nyctStopTimeUpdate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for trip_update::StopTimeUpdate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stop_sequence",
            "stopSequence",
            "stop_id",
            "stopId",
            "arrival",
            "departure",
            "departure_occupancy_status",
            "departureOccupancyStatus",
            "schedule_relationship",
            "scheduleRelationship",
            "stop_time_properties",
            "stopTimeProperties",
            "transit_stop_time_update_extension",
            "transitStopTimeUpdateExtension",
            "nyct_stop_time_update",
            "nyctStopTimeUpdate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StopSequence,
            StopId,
            Arrival,
            Departure,
            DepartureOccupancyStatus,
            ScheduleRelationship,
            StopTimeProperties,
            TransitStopTimeUpdateExtension,
            NyctStopTimeUpdate,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "stopSequence" | "stop_sequence" => Ok(GeneratedField::StopSequence),
                            "stopId" | "stop_id" => Ok(GeneratedField::StopId),
                            "arrival" => Ok(GeneratedField::Arrival),
                            "departure" => Ok(GeneratedField::Departure),
                            "departureOccupancyStatus" | "departure_occupancy_status" => Ok(GeneratedField::DepartureOccupancyStatus),
                            "scheduleRelationship" | "schedule_relationship" => Ok(GeneratedField::ScheduleRelationship),
                            "stopTimeProperties" | "stop_time_properties" => Ok(GeneratedField::StopTimeProperties),
                            "transitStopTimeUpdateExtension" | "transit_stop_time_update_extension" => Ok(GeneratedField::TransitStopTimeUpdateExtension),
                            "nyctStopTimeUpdate" | "nyct_stop_time_update" => Ok(GeneratedField::NyctStopTimeUpdate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = trip_update::StopTimeUpdate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct transit_realtime.TripUpdate.StopTimeUpdate")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<trip_update::StopTimeUpdate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stop_sequence__ = None;
                let mut stop_id__ = None;
                let mut arrival__ = None;
                let mut departure__ = None;
                let mut departure_occupancy_status__ = None;
                let mut schedule_relationship__ = None;
                let mut stop_time_properties__ = None;
                let mut transit_stop_time_update_extension__ = None;
                let mut nyct_stop_time_update__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StopSequence => {
                            if stop_sequence__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stopSequence"));
                            }
                            stop_sequence__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::StopId => {
                            if stop_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stopId"));
                            }
                            stop_id__ = map_.next_value()?;
                        }
                        GeneratedField::Arrival => {
                            if arrival__.is_some() {
                                return Err(serde::de::Error::duplicate_field("arrival"));
                            }
                            arrival__ = map_.next_value()?;
                        }
                        GeneratedField::Departure => {
                            if departure__.is_some() {
                                return Err(serde::de::Error::duplicate_field("departure"));
                            }
                            departure__ = map_.next_value()?;
                        }
                        GeneratedField::DepartureOccupancyStatus => {
                            if departure_occupancy_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("departureOccupancyStatus"));
                            }
                            departure_occupancy_status__ = map_.next_value::<::std::option::Option<vehicle_position::OccupancyStatus>>()?.map(|x| x as i32);
                        }
                        GeneratedField::ScheduleRelationship => {
                            if schedule_relationship__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scheduleRelationship"));
                            }
                            schedule_relationship__ = map_.next_value::<::std::option::Option<trip_update::stop_time_update::ScheduleRelationship>>()?.map(|x| x as i32);
                        }
                        GeneratedField::StopTimeProperties => {
                            if stop_time_properties__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stopTimeProperties"));
                            }
                            stop_time_properties__ = map_.next_value()?;
                        }
                        GeneratedField::TransitStopTimeUpdateExtension => {
                            if transit_stop_time_update_extension__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transitStopTimeUpdateExtension"));
                            }
                            transit_stop_time_update_extension__ = map_.next_value()?;
                        }
                        GeneratedField::NyctStopTimeUpdate => {
                            if nyct_stop_time_update__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nyctStopTimeUpdate"));
                            }
                            nyct_stop_time_update__ = map_.next_value()?;
                        }
                    }
                }
                Ok(trip_update::StopTimeUpdate {
                    stop_sequence: stop_sequence__,
                    stop_id: stop_id__,
                    arrival: arrival__,
                    departure: departure__,
                    departure_occupancy_status: departure_occupancy_status__,
                    schedule_relationship: schedule_relationship__,
                    stop_time_properties: stop_time_properties__,
                    transit_stop_time_update_extension: transit_stop_time_update_extension__,
                    nyct_stop_time_update: nyct_stop_time_update__,
                })
            }
        }
        deserializer.deserialize_struct("transit_realtime.TripUpdate.StopTimeUpdate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for trip_update::stop_time_update::ScheduleRelationship {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Scheduled => "SCHEDULED",
            Self::Skipped => "SKIPPED",
            Self::NoData => "NO_DATA",
            Self::Unscheduled => "UNSCHEDULED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for trip_update::stop_time_update::ScheduleRelationship {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SCHEDULED",
            "SKIPPED",
            "NO_DATA",
            "UNSCHEDULED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = trip_update::stop_time_update::ScheduleRelationship;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "SCHEDULED" => Ok(trip_update::stop_time_update::ScheduleRelationship::Scheduled),
                    "SKIPPED" => Ok(trip_update::stop_time_update::ScheduleRelationship::Skipped),
                    "NO_DATA" => Ok(trip_update::stop_time_update::ScheduleRelationship::NoData),
                    "UNSCHEDULED" => Ok(trip_update::stop_time_update::ScheduleRelationship::Unscheduled),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for trip_update::stop_time_update::StopTimeProperties {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.assigned_stop_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("transit_realtime.TripUpdate.StopTimeUpdate.StopTimeProperties", len)?;
        if let Some(v) = self.assigned_stop_id.as_ref() {
            struct_ser.serialize_field("assignedStopId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for trip_update::stop_time_update::StopTimeProperties {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "assigned_stop_id",
            "assignedStopId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AssignedStopId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "assignedStopId" | "assigned_stop_id" => Ok(GeneratedField::AssignedStopId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = trip_update::stop_time_update::StopTimeProperties;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct transit_realtime.TripUpdate.StopTimeUpdate.StopTimeProperties")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<trip_update::stop_time_update::StopTimeProperties, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut assigned_stop_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AssignedStopId => {
                            if assigned_stop_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assignedStopId"));
                            }
                            assigned_stop_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(trip_update::stop_time_update::StopTimeProperties {
                    assigned_stop_id: assigned_stop_id__,
                })
            }
        }
        deserializer.deserialize_struct("transit_realtime.TripUpdate.StopTimeUpdate.StopTimeProperties", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for trip_update::TransitStopTimeUpdateExtension {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.should_not_assume_vehicle_is_past.is_some() {
            len += 1;
        }
        if self.prediction_algorithm.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("transit_realtime.TripUpdate.TransitStopTimeUpdateExtension", len)?;
        if let Some(v) = self.should_not_assume_vehicle_is_past.as_ref() {
            struct_ser.serialize_field("shouldNotAssumeVehicleIsPast", v)?;
        }
        if let Some(v) = self.prediction_algorithm.as_ref() {
            let v = trip_update::transit_stop_time_update_extension::PredictionAlgorithm::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("predictionAlgorithm", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for trip_update::TransitStopTimeUpdateExtension {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "should_not_assume_vehicle_is_past",
            "shouldNotAssumeVehicleIsPast",
            "prediction_algorithm",
            "predictionAlgorithm",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ShouldNotAssumeVehicleIsPast,
            PredictionAlgorithm,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "shouldNotAssumeVehicleIsPast" | "should_not_assume_vehicle_is_past" => Ok(GeneratedField::ShouldNotAssumeVehicleIsPast),
                            "predictionAlgorithm" | "prediction_algorithm" => Ok(GeneratedField::PredictionAlgorithm),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = trip_update::TransitStopTimeUpdateExtension;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct transit_realtime.TripUpdate.TransitStopTimeUpdateExtension")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<trip_update::TransitStopTimeUpdateExtension, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut should_not_assume_vehicle_is_past__ = None;
                let mut prediction_algorithm__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ShouldNotAssumeVehicleIsPast => {
                            if should_not_assume_vehicle_is_past__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shouldNotAssumeVehicleIsPast"));
                            }
                            should_not_assume_vehicle_is_past__ = map_.next_value()?;
                        }
                        GeneratedField::PredictionAlgorithm => {
                            if prediction_algorithm__.is_some() {
                                return Err(serde::de::Error::duplicate_field("predictionAlgorithm"));
                            }
                            prediction_algorithm__ = map_.next_value::<::std::option::Option<trip_update::transit_stop_time_update_extension::PredictionAlgorithm>>()?.map(|x| x as i32);
                        }
                    }
                }
                Ok(trip_update::TransitStopTimeUpdateExtension {
                    should_not_assume_vehicle_is_past: should_not_assume_vehicle_is_past__,
                    prediction_algorithm: prediction_algorithm__,
                })
            }
        }
        deserializer.deserialize_struct("transit_realtime.TripUpdate.TransitStopTimeUpdateExtension", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for trip_update::transit_stop_time_update_extension::PredictionAlgorithm {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Ml => "ML",
            Self::Recency => "RECENCY",
            Self::Deterministic => "DETERMINISTIC",
            Self::Propagation => "PROPAGATION",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for trip_update::transit_stop_time_update_extension::PredictionAlgorithm {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ML",
            "RECENCY",
            "DETERMINISTIC",
            "PROPAGATION",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = trip_update::transit_stop_time_update_extension::PredictionAlgorithm;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ML" => Ok(trip_update::transit_stop_time_update_extension::PredictionAlgorithm::Ml),
                    "RECENCY" => Ok(trip_update::transit_stop_time_update_extension::PredictionAlgorithm::Recency),
                    "DETERMINISTIC" => Ok(trip_update::transit_stop_time_update_extension::PredictionAlgorithm::Deterministic),
                    "PROPAGATION" => Ok(trip_update::transit_stop_time_update_extension::PredictionAlgorithm::Propagation),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for trip_update::TripProperties {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.trip_id.is_some() {
            len += 1;
        }
        if self.start_date.is_some() {
            len += 1;
        }
        if self.start_time.is_some() {
            len += 1;
        }
        if self.shape_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("transit_realtime.TripUpdate.TripProperties", len)?;
        if let Some(v) = self.trip_id.as_ref() {
            struct_ser.serialize_field("tripId", v)?;
        }
        if let Some(v) = self.start_date.as_ref() {
            struct_ser.serialize_field("startDate", v)?;
        }
        if let Some(v) = self.start_time.as_ref() {
            struct_ser.serialize_field("startTime", v)?;
        }
        if let Some(v) = self.shape_id.as_ref() {
            struct_ser.serialize_field("shapeId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for trip_update::TripProperties {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "trip_id",
            "tripId",
            "start_date",
            "startDate",
            "start_time",
            "startTime",
            "shape_id",
            "shapeId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TripId,
            StartDate,
            StartTime,
            ShapeId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tripId" | "trip_id" => Ok(GeneratedField::TripId),
                            "startDate" | "start_date" => Ok(GeneratedField::StartDate),
                            "startTime" | "start_time" => Ok(GeneratedField::StartTime),
                            "shapeId" | "shape_id" => Ok(GeneratedField::ShapeId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = trip_update::TripProperties;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct transit_realtime.TripUpdate.TripProperties")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<trip_update::TripProperties, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut trip_id__ = None;
                let mut start_date__ = None;
                let mut start_time__ = None;
                let mut shape_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TripId => {
                            if trip_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tripId"));
                            }
                            trip_id__ = map_.next_value()?;
                        }
                        GeneratedField::StartDate => {
                            if start_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startDate"));
                            }
                            start_date__ = map_.next_value()?;
                        }
                        GeneratedField::StartTime => {
                            if start_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startTime"));
                            }
                            start_time__ = map_.next_value()?;
                        }
                        GeneratedField::ShapeId => {
                            if shape_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shapeId"));
                            }
                            shape_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(trip_update::TripProperties {
                    trip_id: trip_id__,
                    start_date: start_date__,
                    start_time: start_time__,
                    shape_id: shape_id__,
                })
            }
        }
        deserializer.deserialize_struct("transit_realtime.TripUpdate.TripProperties", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VehicleDescriptor {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id.is_some() {
            len += 1;
        }
        if self.label.is_some() {
            len += 1;
        }
        if self.license_plate.is_some() {
            len += 1;
        }
        if self.wheelchair_accessible.is_some() {
            len += 1;
        }
        if self.tfnsw_vehicle_descriptor.is_some() {
            len += 1;
        }
        if self.transit_vehicle_descriptor_extension.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("transit_realtime.VehicleDescriptor", len)?;
        if let Some(v) = self.id.as_ref() {
            struct_ser.serialize_field("id", v)?;
        }
        if let Some(v) = self.label.as_ref() {
            struct_ser.serialize_field("label", v)?;
        }
        if let Some(v) = self.license_plate.as_ref() {
            struct_ser.serialize_field("licensePlate", v)?;
        }
        if let Some(v) = self.wheelchair_accessible.as_ref() {
            let v = vehicle_descriptor::WheelchairAccessible::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("wheelchairAccessible", &v)?;
        }
        if let Some(v) = self.tfnsw_vehicle_descriptor.as_ref() {
            struct_ser.serialize_field("tfnswVehicleDescriptor", v)?;
        }
        if let Some(v) = self.transit_vehicle_descriptor_extension.as_ref() {
            struct_ser.serialize_field("transitVehicleDescriptorExtension", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VehicleDescriptor {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "label",
            "license_plate",
            "licensePlate",
            "wheelchair_accessible",
            "wheelchairAccessible",
            "tfnsw_vehicle_descriptor",
            "tfnswVehicleDescriptor",
            "transit_vehicle_descriptor_extension",
            "transitVehicleDescriptorExtension",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Label,
            LicensePlate,
            WheelchairAccessible,
            TfnswVehicleDescriptor,
            TransitVehicleDescriptorExtension,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "label" => Ok(GeneratedField::Label),
                            "licensePlate" | "license_plate" => Ok(GeneratedField::LicensePlate),
                            "wheelchairAccessible" | "wheelchair_accessible" => Ok(GeneratedField::WheelchairAccessible),
                            "tfnswVehicleDescriptor" | "tfnsw_vehicle_descriptor" => Ok(GeneratedField::TfnswVehicleDescriptor),
                            "transitVehicleDescriptorExtension" | "transit_vehicle_descriptor_extension" => Ok(GeneratedField::TransitVehicleDescriptorExtension),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VehicleDescriptor;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct transit_realtime.VehicleDescriptor")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<VehicleDescriptor, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut label__ = None;
                let mut license_plate__ = None;
                let mut wheelchair_accessible__ = None;
                let mut tfnsw_vehicle_descriptor__ = None;
                let mut transit_vehicle_descriptor_extension__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = map_.next_value()?;
                        }
                        GeneratedField::Label => {
                            if label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("label"));
                            }
                            label__ = map_.next_value()?;
                        }
                        GeneratedField::LicensePlate => {
                            if license_plate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("licensePlate"));
                            }
                            license_plate__ = map_.next_value()?;
                        }
                        GeneratedField::WheelchairAccessible => {
                            if wheelchair_accessible__.is_some() {
                                return Err(serde::de::Error::duplicate_field("wheelchairAccessible"));
                            }
                            wheelchair_accessible__ = map_.next_value::<::std::option::Option<vehicle_descriptor::WheelchairAccessible>>()?.map(|x| x as i32);
                        }
                        GeneratedField::TfnswVehicleDescriptor => {
                            if tfnsw_vehicle_descriptor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tfnswVehicleDescriptor"));
                            }
                            tfnsw_vehicle_descriptor__ = map_.next_value()?;
                        }
                        GeneratedField::TransitVehicleDescriptorExtension => {
                            if transit_vehicle_descriptor_extension__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transitVehicleDescriptorExtension"));
                            }
                            transit_vehicle_descriptor_extension__ = map_.next_value()?;
                        }
                    }
                }
                Ok(VehicleDescriptor {
                    id: id__,
                    label: label__,
                    license_plate: license_plate__,
                    wheelchair_accessible: wheelchair_accessible__,
                    tfnsw_vehicle_descriptor: tfnsw_vehicle_descriptor__,
                    transit_vehicle_descriptor_extension: transit_vehicle_descriptor_extension__,
                })
            }
        }
        deserializer.deserialize_struct("transit_realtime.VehicleDescriptor", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for vehicle_descriptor::WheelchairAccessible {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::NoValue => "NO_VALUE",
            Self::Unknown => "UNKNOWN",
            Self::WheelchairAccessible => "WHEELCHAIR_ACCESSIBLE",
            Self::WheelchairInaccessible => "WHEELCHAIR_INACCESSIBLE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for vehicle_descriptor::WheelchairAccessible {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "NO_VALUE",
            "UNKNOWN",
            "WHEELCHAIR_ACCESSIBLE",
            "WHEELCHAIR_INACCESSIBLE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = vehicle_descriptor::WheelchairAccessible;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "NO_VALUE" => Ok(vehicle_descriptor::WheelchairAccessible::NoValue),
                    "UNKNOWN" => Ok(vehicle_descriptor::WheelchairAccessible::Unknown),
                    "WHEELCHAIR_ACCESSIBLE" => Ok(vehicle_descriptor::WheelchairAccessible::WheelchairAccessible),
                    "WHEELCHAIR_INACCESSIBLE" => Ok(vehicle_descriptor::WheelchairAccessible::WheelchairInaccessible),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for VehiclePosition {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.trip.is_some() {
            len += 1;
        }
        if self.vehicle.is_some() {
            len += 1;
        }
        if self.position.is_some() {
            len += 1;
        }
        if self.current_stop_sequence.is_some() {
            len += 1;
        }
        if self.stop_id.is_some() {
            len += 1;
        }
        if self.current_status.is_some() {
            len += 1;
        }
        if self.timestamp.is_some() {
            len += 1;
        }
        if self.congestion_level.is_some() {
            len += 1;
        }
        if self.occupancy_status.is_some() {
            len += 1;
        }
        if self.occupancy_percentage.is_some() {
            len += 1;
        }
        if !self.multi_carriage_details.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("transit_realtime.VehiclePosition", len)?;
        if let Some(v) = self.trip.as_ref() {
            struct_ser.serialize_field("trip", v)?;
        }
        if let Some(v) = self.vehicle.as_ref() {
            struct_ser.serialize_field("vehicle", v)?;
        }
        if let Some(v) = self.position.as_ref() {
            struct_ser.serialize_field("position", v)?;
        }
        if let Some(v) = self.current_stop_sequence.as_ref() {
            struct_ser.serialize_field("currentStopSequence", v)?;
        }
        if let Some(v) = self.stop_id.as_ref() {
            struct_ser.serialize_field("stopId", v)?;
        }
        if let Some(v) = self.current_status.as_ref() {
            let v = vehicle_position::VehicleStopStatus::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("currentStatus", &v)?;
        }
        if let Some(v) = self.timestamp.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("timestamp", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.congestion_level.as_ref() {
            let v = vehicle_position::CongestionLevel::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("congestionLevel", &v)?;
        }
        if let Some(v) = self.occupancy_status.as_ref() {
            let v = vehicle_position::OccupancyStatus::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("occupancyStatus", &v)?;
        }
        if let Some(v) = self.occupancy_percentage.as_ref() {
            struct_ser.serialize_field("occupancyPercentage", v)?;
        }
        if !self.multi_carriage_details.is_empty() {
            struct_ser.serialize_field("multiCarriageDetails", &self.multi_carriage_details)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VehiclePosition {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "trip",
            "vehicle",
            "position",
            "current_stop_sequence",
            "currentStopSequence",
            "stop_id",
            "stopId",
            "current_status",
            "currentStatus",
            "timestamp",
            "congestion_level",
            "congestionLevel",
            "occupancy_status",
            "occupancyStatus",
            "occupancy_percentage",
            "occupancyPercentage",
            "multi_carriage_details",
            "multiCarriageDetails",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Trip,
            Vehicle,
            Position,
            CurrentStopSequence,
            StopId,
            CurrentStatus,
            Timestamp,
            CongestionLevel,
            OccupancyStatus,
            OccupancyPercentage,
            MultiCarriageDetails,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "trip" => Ok(GeneratedField::Trip),
                            "vehicle" => Ok(GeneratedField::Vehicle),
                            "position" => Ok(GeneratedField::Position),
                            "currentStopSequence" | "current_stop_sequence" => Ok(GeneratedField::CurrentStopSequence),
                            "stopId" | "stop_id" => Ok(GeneratedField::StopId),
                            "currentStatus" | "current_status" => Ok(GeneratedField::CurrentStatus),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            "congestionLevel" | "congestion_level" => Ok(GeneratedField::CongestionLevel),
                            "occupancyStatus" | "occupancy_status" => Ok(GeneratedField::OccupancyStatus),
                            "occupancyPercentage" | "occupancy_percentage" => Ok(GeneratedField::OccupancyPercentage),
                            "multiCarriageDetails" | "multi_carriage_details" => Ok(GeneratedField::MultiCarriageDetails),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VehiclePosition;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct transit_realtime.VehiclePosition")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<VehiclePosition, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut trip__ = None;
                let mut vehicle__ = None;
                let mut position__ = None;
                let mut current_stop_sequence__ = None;
                let mut stop_id__ = None;
                let mut current_status__ = None;
                let mut timestamp__ = None;
                let mut congestion_level__ = None;
                let mut occupancy_status__ = None;
                let mut occupancy_percentage__ = None;
                let mut multi_carriage_details__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Trip => {
                            if trip__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trip"));
                            }
                            trip__ = map_.next_value()?;
                        }
                        GeneratedField::Vehicle => {
                            if vehicle__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vehicle"));
                            }
                            vehicle__ = map_.next_value()?;
                        }
                        GeneratedField::Position => {
                            if position__.is_some() {
                                return Err(serde::de::Error::duplicate_field("position"));
                            }
                            position__ = map_.next_value()?;
                        }
                        GeneratedField::CurrentStopSequence => {
                            if current_stop_sequence__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currentStopSequence"));
                            }
                            current_stop_sequence__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::StopId => {
                            if stop_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stopId"));
                            }
                            stop_id__ = map_.next_value()?;
                        }
                        GeneratedField::CurrentStatus => {
                            if current_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currentStatus"));
                            }
                            current_status__ = map_.next_value::<::std::option::Option<vehicle_position::VehicleStopStatus>>()?.map(|x| x as i32);
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::CongestionLevel => {
                            if congestion_level__.is_some() {
                                return Err(serde::de::Error::duplicate_field("congestionLevel"));
                            }
                            congestion_level__ = map_.next_value::<::std::option::Option<vehicle_position::CongestionLevel>>()?.map(|x| x as i32);
                        }
                        GeneratedField::OccupancyStatus => {
                            if occupancy_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("occupancyStatus"));
                            }
                            occupancy_status__ = map_.next_value::<::std::option::Option<vehicle_position::OccupancyStatus>>()?.map(|x| x as i32);
                        }
                        GeneratedField::OccupancyPercentage => {
                            if occupancy_percentage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("occupancyPercentage"));
                            }
                            occupancy_percentage__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::MultiCarriageDetails => {
                            if multi_carriage_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("multiCarriageDetails"));
                            }
                            multi_carriage_details__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(VehiclePosition {
                    trip: trip__,
                    vehicle: vehicle__,
                    position: position__,
                    current_stop_sequence: current_stop_sequence__,
                    stop_id: stop_id__,
                    current_status: current_status__,
                    timestamp: timestamp__,
                    congestion_level: congestion_level__,
                    occupancy_status: occupancy_status__,
                    occupancy_percentage: occupancy_percentage__,
                    multi_carriage_details: multi_carriage_details__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("transit_realtime.VehiclePosition", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for vehicle_position::CarriageDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id.is_some() {
            len += 1;
        }
        if self.label.is_some() {
            len += 1;
        }
        if self.occupancy_status.is_some() {
            len += 1;
        }
        if self.occupancy_percentage.is_some() {
            len += 1;
        }
        if self.carriage_sequence.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("transit_realtime.VehiclePosition.CarriageDetails", len)?;
        if let Some(v) = self.id.as_ref() {
            struct_ser.serialize_field("id", v)?;
        }
        if let Some(v) = self.label.as_ref() {
            struct_ser.serialize_field("label", v)?;
        }
        if let Some(v) = self.occupancy_status.as_ref() {
            let v = vehicle_position::OccupancyStatus::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("occupancyStatus", &v)?;
        }
        if let Some(v) = self.occupancy_percentage.as_ref() {
            struct_ser.serialize_field("occupancyPercentage", v)?;
        }
        if let Some(v) = self.carriage_sequence.as_ref() {
            struct_ser.serialize_field("carriageSequence", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for vehicle_position::CarriageDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "label",
            "occupancy_status",
            "occupancyStatus",
            "occupancy_percentage",
            "occupancyPercentage",
            "carriage_sequence",
            "carriageSequence",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Label,
            OccupancyStatus,
            OccupancyPercentage,
            CarriageSequence,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "label" => Ok(GeneratedField::Label),
                            "occupancyStatus" | "occupancy_status" => Ok(GeneratedField::OccupancyStatus),
                            "occupancyPercentage" | "occupancy_percentage" => Ok(GeneratedField::OccupancyPercentage),
                            "carriageSequence" | "carriage_sequence" => Ok(GeneratedField::CarriageSequence),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = vehicle_position::CarriageDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct transit_realtime.VehiclePosition.CarriageDetails")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<vehicle_position::CarriageDetails, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut label__ = None;
                let mut occupancy_status__ = None;
                let mut occupancy_percentage__ = None;
                let mut carriage_sequence__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = map_.next_value()?;
                        }
                        GeneratedField::Label => {
                            if label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("label"));
                            }
                            label__ = map_.next_value()?;
                        }
                        GeneratedField::OccupancyStatus => {
                            if occupancy_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("occupancyStatus"));
                            }
                            occupancy_status__ = map_.next_value::<::std::option::Option<vehicle_position::OccupancyStatus>>()?.map(|x| x as i32);
                        }
                        GeneratedField::OccupancyPercentage => {
                            if occupancy_percentage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("occupancyPercentage"));
                            }
                            occupancy_percentage__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::CarriageSequence => {
                            if carriage_sequence__.is_some() {
                                return Err(serde::de::Error::duplicate_field("carriageSequence"));
                            }
                            carriage_sequence__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(vehicle_position::CarriageDetails {
                    id: id__,
                    label: label__,
                    occupancy_status: occupancy_status__,
                    occupancy_percentage: occupancy_percentage__,
                    carriage_sequence: carriage_sequence__,
                })
            }
        }
        deserializer.deserialize_struct("transit_realtime.VehiclePosition.CarriageDetails", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for vehicle_position::CongestionLevel {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::UnknownCongestionLevel => "UNKNOWN_CONGESTION_LEVEL",
            Self::RunningSmoothly => "RUNNING_SMOOTHLY",
            Self::StopAndGo => "STOP_AND_GO",
            Self::Congestion => "CONGESTION",
            Self::SevereCongestion => "SEVERE_CONGESTION",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for vehicle_position::CongestionLevel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNKNOWN_CONGESTION_LEVEL",
            "RUNNING_SMOOTHLY",
            "STOP_AND_GO",
            "CONGESTION",
            "SEVERE_CONGESTION",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = vehicle_position::CongestionLevel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "UNKNOWN_CONGESTION_LEVEL" => Ok(vehicle_position::CongestionLevel::UnknownCongestionLevel),
                    "RUNNING_SMOOTHLY" => Ok(vehicle_position::CongestionLevel::RunningSmoothly),
                    "STOP_AND_GO" => Ok(vehicle_position::CongestionLevel::StopAndGo),
                    "CONGESTION" => Ok(vehicle_position::CongestionLevel::Congestion),
                    "SEVERE_CONGESTION" => Ok(vehicle_position::CongestionLevel::SevereCongestion),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for vehicle_position::OccupancyStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Empty => "EMPTY",
            Self::ManySeatsAvailable => "MANY_SEATS_AVAILABLE",
            Self::FewSeatsAvailable => "FEW_SEATS_AVAILABLE",
            Self::StandingRoomOnly => "STANDING_ROOM_ONLY",
            Self::CrushedStandingRoomOnly => "CRUSHED_STANDING_ROOM_ONLY",
            Self::Full => "FULL",
            Self::NotAcceptingPassengers => "NOT_ACCEPTING_PASSENGERS",
            Self::NoDataAvailable => "NO_DATA_AVAILABLE",
            Self::NotBoardable => "NOT_BOARDABLE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for vehicle_position::OccupancyStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "EMPTY",
            "MANY_SEATS_AVAILABLE",
            "FEW_SEATS_AVAILABLE",
            "STANDING_ROOM_ONLY",
            "CRUSHED_STANDING_ROOM_ONLY",
            "FULL",
            "NOT_ACCEPTING_PASSENGERS",
            "NO_DATA_AVAILABLE",
            "NOT_BOARDABLE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = vehicle_position::OccupancyStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "EMPTY" => Ok(vehicle_position::OccupancyStatus::Empty),
                    "MANY_SEATS_AVAILABLE" => Ok(vehicle_position::OccupancyStatus::ManySeatsAvailable),
                    "FEW_SEATS_AVAILABLE" => Ok(vehicle_position::OccupancyStatus::FewSeatsAvailable),
                    "STANDING_ROOM_ONLY" => Ok(vehicle_position::OccupancyStatus::StandingRoomOnly),
                    "CRUSHED_STANDING_ROOM_ONLY" => Ok(vehicle_position::OccupancyStatus::CrushedStandingRoomOnly),
                    "FULL" => Ok(vehicle_position::OccupancyStatus::Full),
                    "NOT_ACCEPTING_PASSENGERS" => Ok(vehicle_position::OccupancyStatus::NotAcceptingPassengers),
                    "NO_DATA_AVAILABLE" => Ok(vehicle_position::OccupancyStatus::NoDataAvailable),
                    "NOT_BOARDABLE" => Ok(vehicle_position::OccupancyStatus::NotBoardable),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for vehicle_position::VehicleStopStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::IncomingAt => "INCOMING_AT",
            Self::StoppedAt => "STOPPED_AT",
            Self::InTransitTo => "IN_TRANSIT_TO",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for vehicle_position::VehicleStopStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "INCOMING_AT",
            "STOPPED_AT",
            "IN_TRANSIT_TO",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = vehicle_position::VehicleStopStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "INCOMING_AT" => Ok(vehicle_position::VehicleStopStatus::IncomingAt),
                    "STOPPED_AT" => Ok(vehicle_position::VehicleStopStatus::StoppedAt),
                    "IN_TRANSIT_TO" => Ok(vehicle_position::VehicleStopStatus::InTransitTo),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
