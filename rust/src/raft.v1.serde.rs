// @generated
impl serde::Serialize for ConfChange {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.change_type != 0 {
            len += 1;
        }
        if self.node_id != 0 {
            len += 1;
        }
        if !self.context.is_empty() {
            len += 1;
        }
        if self.id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("raft.v1.ConfChange", len)?;
        if self.change_type != 0 {
            let v = ConfChangeType::from_i32(self.change_type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.change_type)))?;
            struct_ser.serialize_field("changeType", &v)?;
        }
        if self.node_id != 0 {
            struct_ser.serialize_field("nodeId", ToString::to_string(&self.node_id).as_str())?;
        }
        if !self.context.is_empty() {
            struct_ser.serialize_field("context", pbjson::private::base64::encode(&self.context).as_str())?;
        }
        if self.id != 0 {
            struct_ser.serialize_field("id", ToString::to_string(&self.id).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ConfChange {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "change_type",
            "changeType",
            "node_id",
            "nodeId",
            "context",
            "id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChangeType,
            NodeId,
            Context,
            Id,
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
                            "changeType" | "change_type" => Ok(GeneratedField::ChangeType),
                            "nodeId" | "node_id" => Ok(GeneratedField::NodeId),
                            "context" => Ok(GeneratedField::Context),
                            "id" => Ok(GeneratedField::Id),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConfChange;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct raft.v1.ConfChange")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ConfChange, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut change_type__ = None;
                let mut node_id__ = None;
                let mut context__ = None;
                let mut id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ChangeType => {
                            if change_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("changeType"));
                            }
                            change_type__ = Some(map.next_value::<ConfChangeType>()? as i32);
                        }
                        GeneratedField::NodeId => {
                            if node_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nodeId"));
                            }
                            node_id__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Context => {
                            if context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("context"));
                            }
                            context__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ConfChange {
                    change_type: change_type__.unwrap_or_default(),
                    node_id: node_id__.unwrap_or_default(),
                    context: context__.unwrap_or_default(),
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("raft.v1.ConfChange", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ConfChangeSingle {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.change_type != 0 {
            len += 1;
        }
        if self.node_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("raft.v1.ConfChangeSingle", len)?;
        if self.change_type != 0 {
            let v = ConfChangeType::from_i32(self.change_type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.change_type)))?;
            struct_ser.serialize_field("changeType", &v)?;
        }
        if self.node_id != 0 {
            struct_ser.serialize_field("nodeId", ToString::to_string(&self.node_id).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ConfChangeSingle {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "change_type",
            "changeType",
            "node_id",
            "nodeId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChangeType,
            NodeId,
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
                            "changeType" | "change_type" => Ok(GeneratedField::ChangeType),
                            "nodeId" | "node_id" => Ok(GeneratedField::NodeId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConfChangeSingle;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct raft.v1.ConfChangeSingle")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ConfChangeSingle, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut change_type__ = None;
                let mut node_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ChangeType => {
                            if change_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("changeType"));
                            }
                            change_type__ = Some(map.next_value::<ConfChangeType>()? as i32);
                        }
                        GeneratedField::NodeId => {
                            if node_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nodeId"));
                            }
                            node_id__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ConfChangeSingle {
                    change_type: change_type__.unwrap_or_default(),
                    node_id: node_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("raft.v1.ConfChangeSingle", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ConfChangeTransition {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Auto => "Auto",
            Self::Implicit => "Implicit",
            Self::Explicit => "Explicit",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ConfChangeTransition {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "Auto",
            "Implicit",
            "Explicit",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConfChangeTransition;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(ConfChangeTransition::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(ConfChangeTransition::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "Auto" => Ok(ConfChangeTransition::Auto),
                    "Implicit" => Ok(ConfChangeTransition::Implicit),
                    "Explicit" => Ok(ConfChangeTransition::Explicit),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ConfChangeType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::AddNode => "AddNode",
            Self::RemoveNode => "RemoveNode",
            Self::AddLearnerNode => "AddLearnerNode",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ConfChangeType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "AddNode",
            "RemoveNode",
            "AddLearnerNode",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConfChangeType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(ConfChangeType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(ConfChangeType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "AddNode" => Ok(ConfChangeType::AddNode),
                    "RemoveNode" => Ok(ConfChangeType::RemoveNode),
                    "AddLearnerNode" => Ok(ConfChangeType::AddLearnerNode),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ConfChangeV2 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.transition != 0 {
            len += 1;
        }
        if !self.changes.is_empty() {
            len += 1;
        }
        if !self.context.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("raft.v1.ConfChangeV2", len)?;
        if self.transition != 0 {
            let v = ConfChangeTransition::from_i32(self.transition)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.transition)))?;
            struct_ser.serialize_field("transition", &v)?;
        }
        if !self.changes.is_empty() {
            struct_ser.serialize_field("changes", &self.changes)?;
        }
        if !self.context.is_empty() {
            struct_ser.serialize_field("context", pbjson::private::base64::encode(&self.context).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ConfChangeV2 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "transition",
            "changes",
            "context",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Transition,
            Changes,
            Context,
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
                            "transition" => Ok(GeneratedField::Transition),
                            "changes" => Ok(GeneratedField::Changes),
                            "context" => Ok(GeneratedField::Context),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConfChangeV2;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct raft.v1.ConfChangeV2")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ConfChangeV2, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut transition__ = None;
                let mut changes__ = None;
                let mut context__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Transition => {
                            if transition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transition"));
                            }
                            transition__ = Some(map.next_value::<ConfChangeTransition>()? as i32);
                        }
                        GeneratedField::Changes => {
                            if changes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("changes"));
                            }
                            changes__ = Some(map.next_value()?);
                        }
                        GeneratedField::Context => {
                            if context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("context"));
                            }
                            context__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ConfChangeV2 {
                    transition: transition__.unwrap_or_default(),
                    changes: changes__.unwrap_or_default(),
                    context: context__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("raft.v1.ConfChangeV2", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ConfState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.voters.is_empty() {
            len += 1;
        }
        if !self.learners.is_empty() {
            len += 1;
        }
        if !self.voters_outgoing.is_empty() {
            len += 1;
        }
        if !self.learners_next.is_empty() {
            len += 1;
        }
        if self.auto_leave {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("raft.v1.ConfState", len)?;
        if !self.voters.is_empty() {
            struct_ser.serialize_field("voters", &self.voters.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if !self.learners.is_empty() {
            struct_ser.serialize_field("learners", &self.learners.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if !self.voters_outgoing.is_empty() {
            struct_ser.serialize_field("votersOutgoing", &self.voters_outgoing.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if !self.learners_next.is_empty() {
            struct_ser.serialize_field("learnersNext", &self.learners_next.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if self.auto_leave {
            struct_ser.serialize_field("autoLeave", &self.auto_leave)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ConfState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "voters",
            "learners",
            "voters_outgoing",
            "votersOutgoing",
            "learners_next",
            "learnersNext",
            "auto_leave",
            "autoLeave",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Voters,
            Learners,
            VotersOutgoing,
            LearnersNext,
            AutoLeave,
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
                            "voters" => Ok(GeneratedField::Voters),
                            "learners" => Ok(GeneratedField::Learners),
                            "votersOutgoing" | "voters_outgoing" => Ok(GeneratedField::VotersOutgoing),
                            "learnersNext" | "learners_next" => Ok(GeneratedField::LearnersNext),
                            "autoLeave" | "auto_leave" => Ok(GeneratedField::AutoLeave),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConfState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct raft.v1.ConfState")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ConfState, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut voters__ = None;
                let mut learners__ = None;
                let mut voters_outgoing__ = None;
                let mut learners_next__ = None;
                let mut auto_leave__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Voters => {
                            if voters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("voters"));
                            }
                            voters__ = 
                                Some(map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Learners => {
                            if learners__.is_some() {
                                return Err(serde::de::Error::duplicate_field("learners"));
                            }
                            learners__ = 
                                Some(map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::VotersOutgoing => {
                            if voters_outgoing__.is_some() {
                                return Err(serde::de::Error::duplicate_field("votersOutgoing"));
                            }
                            voters_outgoing__ = 
                                Some(map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::LearnersNext => {
                            if learners_next__.is_some() {
                                return Err(serde::de::Error::duplicate_field("learnersNext"));
                            }
                            learners_next__ = 
                                Some(map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::AutoLeave => {
                            if auto_leave__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoLeave"));
                            }
                            auto_leave__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ConfState {
                    voters: voters__.unwrap_or_default(),
                    learners: learners__.unwrap_or_default(),
                    voters_outgoing: voters_outgoing__.unwrap_or_default(),
                    learners_next: learners_next__.unwrap_or_default(),
                    auto_leave: auto_leave__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("raft.v1.ConfState", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Entry {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.entry_type != 0 {
            len += 1;
        }
        if self.term != 0 {
            len += 1;
        }
        if self.index != 0 {
            len += 1;
        }
        if !self.data.is_empty() {
            len += 1;
        }
        if !self.context.is_empty() {
            len += 1;
        }
        if self.sync_log {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("raft.v1.Entry", len)?;
        if self.entry_type != 0 {
            let v = EntryType::from_i32(self.entry_type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.entry_type)))?;
            struct_ser.serialize_field("entryType", &v)?;
        }
        if self.term != 0 {
            struct_ser.serialize_field("term", ToString::to_string(&self.term).as_str())?;
        }
        if self.index != 0 {
            struct_ser.serialize_field("index", ToString::to_string(&self.index).as_str())?;
        }
        if !self.data.is_empty() {
            struct_ser.serialize_field("data", pbjson::private::base64::encode(&self.data).as_str())?;
        }
        if !self.context.is_empty() {
            struct_ser.serialize_field("context", pbjson::private::base64::encode(&self.context).as_str())?;
        }
        if self.sync_log {
            struct_ser.serialize_field("syncLog", &self.sync_log)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Entry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entry_type",
            "entryType",
            "term",
            "index",
            "data",
            "context",
            "sync_log",
            "syncLog",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EntryType,
            Term,
            Index,
            Data,
            Context,
            SyncLog,
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
                            "entryType" | "entry_type" => Ok(GeneratedField::EntryType),
                            "term" => Ok(GeneratedField::Term),
                            "index" => Ok(GeneratedField::Index),
                            "data" => Ok(GeneratedField::Data),
                            "context" => Ok(GeneratedField::Context),
                            "syncLog" | "sync_log" => Ok(GeneratedField::SyncLog),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Entry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct raft.v1.Entry")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Entry, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entry_type__ = None;
                let mut term__ = None;
                let mut index__ = None;
                let mut data__ = None;
                let mut context__ = None;
                let mut sync_log__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::EntryType => {
                            if entry_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entryType"));
                            }
                            entry_type__ = Some(map.next_value::<EntryType>()? as i32);
                        }
                        GeneratedField::Term => {
                            if term__.is_some() {
                                return Err(serde::de::Error::duplicate_field("term"));
                            }
                            term__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Index => {
                            if index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            index__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Context => {
                            if context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("context"));
                            }
                            context__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SyncLog => {
                            if sync_log__.is_some() {
                                return Err(serde::de::Error::duplicate_field("syncLog"));
                            }
                            sync_log__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Entry {
                    entry_type: entry_type__.unwrap_or_default(),
                    term: term__.unwrap_or_default(),
                    index: index__.unwrap_or_default(),
                    data: data__.unwrap_or_default(),
                    context: context__.unwrap_or_default(),
                    sync_log: sync_log__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("raft.v1.Entry", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EntryType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::EntryNormal => "EntryNormal",
            Self::EntryConfChange => "EntryConfChange",
            Self::EntryConfChangeV2 => "EntryConfChangeV2",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for EntryType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "EntryNormal",
            "EntryConfChange",
            "EntryConfChangeV2",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EntryType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(EntryType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(EntryType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "EntryNormal" => Ok(EntryType::EntryNormal),
                    "EntryConfChange" => Ok(EntryType::EntryConfChange),
                    "EntryConfChangeV2" => Ok(EntryType::EntryConfChangeV2),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for HardState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.term != 0 {
            len += 1;
        }
        if self.vote != 0 {
            len += 1;
        }
        if self.commit != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("raft.v1.HardState", len)?;
        if self.term != 0 {
            struct_ser.serialize_field("term", ToString::to_string(&self.term).as_str())?;
        }
        if self.vote != 0 {
            struct_ser.serialize_field("vote", ToString::to_string(&self.vote).as_str())?;
        }
        if self.commit != 0 {
            struct_ser.serialize_field("commit", ToString::to_string(&self.commit).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HardState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "term",
            "vote",
            "commit",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Term,
            Vote,
            Commit,
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
                            "term" => Ok(GeneratedField::Term),
                            "vote" => Ok(GeneratedField::Vote),
                            "commit" => Ok(GeneratedField::Commit),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HardState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct raft.v1.HardState")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HardState, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut term__ = None;
                let mut vote__ = None;
                let mut commit__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Term => {
                            if term__.is_some() {
                                return Err(serde::de::Error::duplicate_field("term"));
                            }
                            term__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Vote => {
                            if vote__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vote"));
                            }
                            vote__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Commit => {
                            if commit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commit"));
                            }
                            commit__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(HardState {
                    term: term__.unwrap_or_default(),
                    vote: vote__.unwrap_or_default(),
                    commit: commit__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("raft.v1.HardState", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Message {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.msg_type != 0 {
            len += 1;
        }
        if self.to != 0 {
            len += 1;
        }
        if self.from != 0 {
            len += 1;
        }
        if self.term != 0 {
            len += 1;
        }
        if self.log_term != 0 {
            len += 1;
        }
        if self.index != 0 {
            len += 1;
        }
        if !self.entries.is_empty() {
            len += 1;
        }
        if self.commit != 0 {
            len += 1;
        }
        if self.commit_term != 0 {
            len += 1;
        }
        if self.snapshot.is_some() {
            len += 1;
        }
        if self.request_snapshot != 0 {
            len += 1;
        }
        if self.reject {
            len += 1;
        }
        if self.reject_hint != 0 {
            len += 1;
        }
        if !self.context.is_empty() {
            len += 1;
        }
        if self.deprecated_priority != 0 {
            len += 1;
        }
        if self.priority != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("raft.v1.Message", len)?;
        if self.msg_type != 0 {
            let v = MessageType::from_i32(self.msg_type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.msg_type)))?;
            struct_ser.serialize_field("msgType", &v)?;
        }
        if self.to != 0 {
            struct_ser.serialize_field("to", ToString::to_string(&self.to).as_str())?;
        }
        if self.from != 0 {
            struct_ser.serialize_field("from", ToString::to_string(&self.from).as_str())?;
        }
        if self.term != 0 {
            struct_ser.serialize_field("term", ToString::to_string(&self.term).as_str())?;
        }
        if self.log_term != 0 {
            struct_ser.serialize_field("logTerm", ToString::to_string(&self.log_term).as_str())?;
        }
        if self.index != 0 {
            struct_ser.serialize_field("index", ToString::to_string(&self.index).as_str())?;
        }
        if !self.entries.is_empty() {
            struct_ser.serialize_field("entries", &self.entries)?;
        }
        if self.commit != 0 {
            struct_ser.serialize_field("commit", ToString::to_string(&self.commit).as_str())?;
        }
        if self.commit_term != 0 {
            struct_ser.serialize_field("commitTerm", ToString::to_string(&self.commit_term).as_str())?;
        }
        if let Some(v) = self.snapshot.as_ref() {
            struct_ser.serialize_field("snapshot", v)?;
        }
        if self.request_snapshot != 0 {
            struct_ser.serialize_field("requestSnapshot", ToString::to_string(&self.request_snapshot).as_str())?;
        }
        if self.reject {
            struct_ser.serialize_field("reject", &self.reject)?;
        }
        if self.reject_hint != 0 {
            struct_ser.serialize_field("rejectHint", ToString::to_string(&self.reject_hint).as_str())?;
        }
        if !self.context.is_empty() {
            struct_ser.serialize_field("context", pbjson::private::base64::encode(&self.context).as_str())?;
        }
        if self.deprecated_priority != 0 {
            struct_ser.serialize_field("deprecatedPriority", ToString::to_string(&self.deprecated_priority).as_str())?;
        }
        if self.priority != 0 {
            struct_ser.serialize_field("priority", ToString::to_string(&self.priority).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Message {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "msg_type",
            "msgType",
            "to",
            "from",
            "term",
            "log_term",
            "logTerm",
            "index",
            "entries",
            "commit",
            "commit_term",
            "commitTerm",
            "snapshot",
            "request_snapshot",
            "requestSnapshot",
            "reject",
            "reject_hint",
            "rejectHint",
            "context",
            "deprecated_priority",
            "deprecatedPriority",
            "priority",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MsgType,
            To,
            From,
            Term,
            LogTerm,
            Index,
            Entries,
            Commit,
            CommitTerm,
            Snapshot,
            RequestSnapshot,
            Reject,
            RejectHint,
            Context,
            DeprecatedPriority,
            Priority,
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
                            "msgType" | "msg_type" => Ok(GeneratedField::MsgType),
                            "to" => Ok(GeneratedField::To),
                            "from" => Ok(GeneratedField::From),
                            "term" => Ok(GeneratedField::Term),
                            "logTerm" | "log_term" => Ok(GeneratedField::LogTerm),
                            "index" => Ok(GeneratedField::Index),
                            "entries" => Ok(GeneratedField::Entries),
                            "commit" => Ok(GeneratedField::Commit),
                            "commitTerm" | "commit_term" => Ok(GeneratedField::CommitTerm),
                            "snapshot" => Ok(GeneratedField::Snapshot),
                            "requestSnapshot" | "request_snapshot" => Ok(GeneratedField::RequestSnapshot),
                            "reject" => Ok(GeneratedField::Reject),
                            "rejectHint" | "reject_hint" => Ok(GeneratedField::RejectHint),
                            "context" => Ok(GeneratedField::Context),
                            "deprecatedPriority" | "deprecated_priority" => Ok(GeneratedField::DeprecatedPriority),
                            "priority" => Ok(GeneratedField::Priority),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Message;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct raft.v1.Message")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Message, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut msg_type__ = None;
                let mut to__ = None;
                let mut from__ = None;
                let mut term__ = None;
                let mut log_term__ = None;
                let mut index__ = None;
                let mut entries__ = None;
                let mut commit__ = None;
                let mut commit_term__ = None;
                let mut snapshot__ = None;
                let mut request_snapshot__ = None;
                let mut reject__ = None;
                let mut reject_hint__ = None;
                let mut context__ = None;
                let mut deprecated_priority__ = None;
                let mut priority__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MsgType => {
                            if msg_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("msgType"));
                            }
                            msg_type__ = Some(map.next_value::<MessageType>()? as i32);
                        }
                        GeneratedField::To => {
                            if to__.is_some() {
                                return Err(serde::de::Error::duplicate_field("to"));
                            }
                            to__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::From => {
                            if from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("from"));
                            }
                            from__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Term => {
                            if term__.is_some() {
                                return Err(serde::de::Error::duplicate_field("term"));
                            }
                            term__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::LogTerm => {
                            if log_term__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logTerm"));
                            }
                            log_term__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Index => {
                            if index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            index__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Entries => {
                            if entries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entries"));
                            }
                            entries__ = Some(map.next_value()?);
                        }
                        GeneratedField::Commit => {
                            if commit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commit"));
                            }
                            commit__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CommitTerm => {
                            if commit_term__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commitTerm"));
                            }
                            commit_term__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Snapshot => {
                            if snapshot__.is_some() {
                                return Err(serde::de::Error::duplicate_field("snapshot"));
                            }
                            snapshot__ = map.next_value()?;
                        }
                        GeneratedField::RequestSnapshot => {
                            if request_snapshot__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestSnapshot"));
                            }
                            request_snapshot__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Reject => {
                            if reject__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reject"));
                            }
                            reject__ = Some(map.next_value()?);
                        }
                        GeneratedField::RejectHint => {
                            if reject_hint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rejectHint"));
                            }
                            reject_hint__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Context => {
                            if context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("context"));
                            }
                            context__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DeprecatedPriority => {
                            if deprecated_priority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deprecatedPriority"));
                            }
                            deprecated_priority__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Priority => {
                            if priority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            priority__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Message {
                    msg_type: msg_type__.unwrap_or_default(),
                    to: to__.unwrap_or_default(),
                    from: from__.unwrap_or_default(),
                    term: term__.unwrap_or_default(),
                    log_term: log_term__.unwrap_or_default(),
                    index: index__.unwrap_or_default(),
                    entries: entries__.unwrap_or_default(),
                    commit: commit__.unwrap_or_default(),
                    commit_term: commit_term__.unwrap_or_default(),
                    snapshot: snapshot__,
                    request_snapshot: request_snapshot__.unwrap_or_default(),
                    reject: reject__.unwrap_or_default(),
                    reject_hint: reject_hint__.unwrap_or_default(),
                    context: context__.unwrap_or_default(),
                    deprecated_priority: deprecated_priority__.unwrap_or_default(),
                    priority: priority__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("raft.v1.Message", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MessageType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::MsgHup => "MsgHup",
            Self::MsgBeat => "MsgBeat",
            Self::MsgPropose => "MsgPropose",
            Self::MsgAppend => "MsgAppend",
            Self::MsgAppendResponse => "MsgAppendResponse",
            Self::MsgRequestVote => "MsgRequestVote",
            Self::MsgRequestVoteResponse => "MsgRequestVoteResponse",
            Self::MsgSnapshot => "MsgSnapshot",
            Self::MsgHeartbeat => "MsgHeartbeat",
            Self::MsgHeartbeatResponse => "MsgHeartbeatResponse",
            Self::MsgUnreachable => "MsgUnreachable",
            Self::MsgSnapStatus => "MsgSnapStatus",
            Self::MsgCheckQuorum => "MsgCheckQuorum",
            Self::MsgTransferLeader => "MsgTransferLeader",
            Self::MsgTimeoutNow => "MsgTimeoutNow",
            Self::MsgReadIndex => "MsgReadIndex",
            Self::MsgReadIndexResp => "MsgReadIndexResp",
            Self::MsgRequestPreVote => "MsgRequestPreVote",
            Self::MsgRequestPreVoteResponse => "MsgRequestPreVoteResponse",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for MessageType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "MsgHup",
            "MsgBeat",
            "MsgPropose",
            "MsgAppend",
            "MsgAppendResponse",
            "MsgRequestVote",
            "MsgRequestVoteResponse",
            "MsgSnapshot",
            "MsgHeartbeat",
            "MsgHeartbeatResponse",
            "MsgUnreachable",
            "MsgSnapStatus",
            "MsgCheckQuorum",
            "MsgTransferLeader",
            "MsgTimeoutNow",
            "MsgReadIndex",
            "MsgReadIndexResp",
            "MsgRequestPreVote",
            "MsgRequestPreVoteResponse",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MessageType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(MessageType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(MessageType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "MsgHup" => Ok(MessageType::MsgHup),
                    "MsgBeat" => Ok(MessageType::MsgBeat),
                    "MsgPropose" => Ok(MessageType::MsgPropose),
                    "MsgAppend" => Ok(MessageType::MsgAppend),
                    "MsgAppendResponse" => Ok(MessageType::MsgAppendResponse),
                    "MsgRequestVote" => Ok(MessageType::MsgRequestVote),
                    "MsgRequestVoteResponse" => Ok(MessageType::MsgRequestVoteResponse),
                    "MsgSnapshot" => Ok(MessageType::MsgSnapshot),
                    "MsgHeartbeat" => Ok(MessageType::MsgHeartbeat),
                    "MsgHeartbeatResponse" => Ok(MessageType::MsgHeartbeatResponse),
                    "MsgUnreachable" => Ok(MessageType::MsgUnreachable),
                    "MsgSnapStatus" => Ok(MessageType::MsgSnapStatus),
                    "MsgCheckQuorum" => Ok(MessageType::MsgCheckQuorum),
                    "MsgTransferLeader" => Ok(MessageType::MsgTransferLeader),
                    "MsgTimeoutNow" => Ok(MessageType::MsgTimeoutNow),
                    "MsgReadIndex" => Ok(MessageType::MsgReadIndex),
                    "MsgReadIndexResp" => Ok(MessageType::MsgReadIndexResp),
                    "MsgRequestPreVote" => Ok(MessageType::MsgRequestPreVote),
                    "MsgRequestPreVoteResponse" => Ok(MessageType::MsgRequestPreVoteResponse),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Snapshot {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.data.is_empty() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("raft.v1.Snapshot", len)?;
        if !self.data.is_empty() {
            struct_ser.serialize_field("data", pbjson::private::base64::encode(&self.data).as_str())?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Snapshot {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "data",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Data,
            Metadata,
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
                            "data" => Ok(GeneratedField::Data),
                            "metadata" => Ok(GeneratedField::Metadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Snapshot;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct raft.v1.Snapshot")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Snapshot, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut data__ = None;
                let mut metadata__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                    }
                }
                Ok(Snapshot {
                    data: data__.unwrap_or_default(),
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("raft.v1.Snapshot", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SnapshotMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.conf_state.is_some() {
            len += 1;
        }
        if self.index != 0 {
            len += 1;
        }
        if self.term != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("raft.v1.SnapshotMetadata", len)?;
        if let Some(v) = self.conf_state.as_ref() {
            struct_ser.serialize_field("confState", v)?;
        }
        if self.index != 0 {
            struct_ser.serialize_field("index", ToString::to_string(&self.index).as_str())?;
        }
        if self.term != 0 {
            struct_ser.serialize_field("term", ToString::to_string(&self.term).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SnapshotMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "conf_state",
            "confState",
            "index",
            "term",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ConfState,
            Index,
            Term,
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
                            "confState" | "conf_state" => Ok(GeneratedField::ConfState),
                            "index" => Ok(GeneratedField::Index),
                            "term" => Ok(GeneratedField::Term),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SnapshotMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct raft.v1.SnapshotMetadata")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SnapshotMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut conf_state__ = None;
                let mut index__ = None;
                let mut term__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ConfState => {
                            if conf_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("confState"));
                            }
                            conf_state__ = map.next_value()?;
                        }
                        GeneratedField::Index => {
                            if index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            index__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Term => {
                            if term__.is_some() {
                                return Err(serde::de::Error::duplicate_field("term"));
                            }
                            term__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(SnapshotMetadata {
                    conf_state: conf_state__,
                    index: index__.unwrap_or_default(),
                    term: term__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("raft.v1.SnapshotMetadata", FIELDS, GeneratedVisitor)
    }
}
