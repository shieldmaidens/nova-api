// @generated
impl serde::Serialize for AppendEntriesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.vote.is_some() {
            len += 1;
        }
        if self.prev_log_id.is_some() {
            len += 1;
        }
        if !self.entries.is_empty() {
            len += 1;
        }
        if self.leader_commit.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("raft.v1.AppendEntriesRequest", len)?;
        if let Some(v) = self.vote.as_ref() {
            struct_ser.serialize_field("vote", v)?;
        }
        if let Some(v) = self.prev_log_id.as_ref() {
            struct_ser.serialize_field("prevLogId", v)?;
        }
        if !self.entries.is_empty() {
            struct_ser.serialize_field("entries", &self.entries)?;
        }
        if let Some(v) = self.leader_commit.as_ref() {
            struct_ser.serialize_field("leaderCommit", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AppendEntriesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "vote",
            "prev_log_id",
            "prevLogId",
            "entries",
            "leader_commit",
            "leaderCommit",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Vote,
            PrevLogId,
            Entries,
            LeaderCommit,
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
                            "vote" => Ok(GeneratedField::Vote),
                            "prevLogId" | "prev_log_id" => Ok(GeneratedField::PrevLogId),
                            "entries" => Ok(GeneratedField::Entries),
                            "leaderCommit" | "leader_commit" => Ok(GeneratedField::LeaderCommit),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AppendEntriesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct raft.v1.AppendEntriesRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AppendEntriesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut vote__ = None;
                let mut prev_log_id__ = None;
                let mut entries__ = None;
                let mut leader_commit__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Vote => {
                            if vote__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vote"));
                            }
                            vote__ = map.next_value()?;
                        }
                        GeneratedField::PrevLogId => {
                            if prev_log_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prevLogId"));
                            }
                            prev_log_id__ = map.next_value()?;
                        }
                        GeneratedField::Entries => {
                            if entries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entries"));
                            }
                            entries__ = Some(map.next_value()?);
                        }
                        GeneratedField::LeaderCommit => {
                            if leader_commit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("leaderCommit"));
                            }
                            leader_commit__ = map.next_value()?;
                        }
                    }
                }
                Ok(AppendEntriesRequest {
                    vote: vote__,
                    prev_log_id: prev_log_id__,
                    entries: entries__.unwrap_or_default(),
                    leader_commit: leader_commit__,
                })
            }
        }
        deserializer.deserialize_struct("raft.v1.AppendEntriesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AppendEntriesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.response.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("raft.v1.AppendEntriesResponse", len)?;
        if let Some(v) = self.response.as_ref() {
            match v {
                append_entries_response::Response::Success(v) => {
                    struct_ser.serialize_field("success", v)?;
                }
                append_entries_response::Response::Conflict(v) => {
                    struct_ser.serialize_field("conflict", v)?;
                }
                append_entries_response::Response::HigherVote(v) => {
                    struct_ser.serialize_field("higherVote", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AppendEntriesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "success",
            "conflict",
            "higher_vote",
            "higherVote",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Success,
            Conflict,
            HigherVote,
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
                            "success" => Ok(GeneratedField::Success),
                            "conflict" => Ok(GeneratedField::Conflict),
                            "higherVote" | "higher_vote" => Ok(GeneratedField::HigherVote),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AppendEntriesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct raft.v1.AppendEntriesResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AppendEntriesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut response__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Success => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("success"));
                            }
                            response__ = map.next_value::<::std::option::Option<_>>()?.map(append_entries_response::Response::Success)
;
                        }
                        GeneratedField::Conflict => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("conflict"));
                            }
                            response__ = map.next_value::<::std::option::Option<_>>()?.map(append_entries_response::Response::Conflict)
;
                        }
                        GeneratedField::HigherVote => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("higherVote"));
                            }
                            response__ = map.next_value::<::std::option::Option<_>>()?.map(append_entries_response::Response::HigherVote)
;
                        }
                    }
                }
                Ok(AppendEntriesResponse {
                    response: response__,
                })
            }
        }
        deserializer.deserialize_struct("raft.v1.AppendEntriesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Blank {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("raft.v1.Blank", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Blank {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Blank;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct raft.v1.Blank")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Blank, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(Blank {
                })
            }
        }
        deserializer.deserialize_struct("raft.v1.Blank", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ColumnFamilies {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.column_families.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("raft.v1.ColumnFamilies", len)?;
        if !self.column_families.is_empty() {
            struct_ser.serialize_field("columnFamilies", &self.column_families)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ColumnFamilies {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "column_families",
            "columnFamilies",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ColumnFamilies,
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
                            "columnFamilies" | "column_families" => Ok(GeneratedField::ColumnFamilies),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ColumnFamilies;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct raft.v1.ColumnFamilies")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ColumnFamilies, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut column_families__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ColumnFamilies => {
                            if column_families__.is_some() {
                                return Err(serde::de::Error::duplicate_field("columnFamilies"));
                            }
                            column_families__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ColumnFamilies {
                    column_families: column_families__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("raft.v1.ColumnFamilies", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ColumnFamilyDescriptor {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.range != 0 {
            len += 1;
        }
        if self.shard != 0 {
            len += 1;
        }
        if self.r#type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("raft.v1.ColumnFamilyDescriptor", len)?;
        if self.range != 0 {
            struct_ser.serialize_field("range", ToString::to_string(&self.range).as_str())?;
        }
        if self.shard != 0 {
            struct_ser.serialize_field("shard", ToString::to_string(&self.shard).as_str())?;
        }
        if self.r#type != 0 {
            let v = ColumnFamilyType::from_i32(self.r#type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ColumnFamilyDescriptor {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "range",
            "shard",
            "type",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Range,
            Shard,
            Type,
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
                            "range" => Ok(GeneratedField::Range),
                            "shard" => Ok(GeneratedField::Shard),
                            "type" => Ok(GeneratedField::Type),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ColumnFamilyDescriptor;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct raft.v1.ColumnFamilyDescriptor")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ColumnFamilyDescriptor, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut range__ = None;
                let mut shard__ = None;
                let mut r#type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Range => {
                            if range__.is_some() {
                                return Err(serde::de::Error::duplicate_field("range"));
                            }
                            range__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Shard => {
                            if shard__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shard"));
                            }
                            shard__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map.next_value::<ColumnFamilyType>()? as i32);
                        }
                    }
                }
                Ok(ColumnFamilyDescriptor {
                    range: range__.unwrap_or_default(),
                    shard: shard__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("raft.v1.ColumnFamilyDescriptor", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ColumnFamilyType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "COLUMN_FAMILY_TYPE_UNSPECIFIED",
            Self::Config => "COLUMN_FAMILY_TYPE_CONFIG",
            Self::RaftLog => "COLUMN_FAMILY_TYPE_RAFT_LOG",
            Self::Data => "COLUMN_FAMILY_TYPE_DATA",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ColumnFamilyType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "COLUMN_FAMILY_TYPE_UNSPECIFIED",
            "COLUMN_FAMILY_TYPE_CONFIG",
            "COLUMN_FAMILY_TYPE_RAFT_LOG",
            "COLUMN_FAMILY_TYPE_DATA",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ColumnFamilyType;

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
                    .and_then(ColumnFamilyType::from_i32)
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
                    .and_then(ColumnFamilyType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "COLUMN_FAMILY_TYPE_UNSPECIFIED" => Ok(ColumnFamilyType::Unspecified),
                    "COLUMN_FAMILY_TYPE_CONFIG" => Ok(ColumnFamilyType::Config),
                    "COLUMN_FAMILY_TYPE_RAFT_LOG" => Ok(ColumnFamilyType::RaftLog),
                    "COLUMN_FAMILY_TYPE_DATA" => Ok(ColumnFamilyType::Data),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for CommittedLeaderId {
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
        if self.node_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("raft.v1.CommittedLeaderId", len)?;
        if self.term != 0 {
            struct_ser.serialize_field("term", ToString::to_string(&self.term).as_str())?;
        }
        if self.node_id != 0 {
            struct_ser.serialize_field("nodeId", ToString::to_string(&self.node_id).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CommittedLeaderId {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "term",
            "node_id",
            "nodeId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Term,
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
                            "term" => Ok(GeneratedField::Term),
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
            type Value = CommittedLeaderId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct raft.v1.CommittedLeaderId")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CommittedLeaderId, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut term__ = None;
                let mut node_id__ = None;
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
                Ok(CommittedLeaderId {
                    term: term__.unwrap_or_default(),
                    node_id: node_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("raft.v1.CommittedLeaderId", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Conflict {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("raft.v1.Conflict", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Conflict {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Conflict;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct raft.v1.Conflict")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Conflict, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(Conflict {
                })
            }
        }
        deserializer.deserialize_struct("raft.v1.Conflict", FIELDS, GeneratedVisitor)
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
        if self.log_id.is_some() {
            len += 1;
        }
        if self.payload.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("raft.v1.Entry", len)?;
        if let Some(v) = self.log_id.as_ref() {
            struct_ser.serialize_field("logId", v)?;
        }
        if let Some(v) = self.payload.as_ref() {
            match v {
                entry::Payload::Blank(v) => {
                    struct_ser.serialize_field("blank", v)?;
                }
                entry::Payload::Normal(v) => {
                    struct_ser.serialize_field("normal", v)?;
                }
                entry::Payload::Membership(v) => {
                    struct_ser.serialize_field("membership", v)?;
                }
            }
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
            "log_id",
            "logId",
            "blank",
            "normal",
            "membership",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LogId,
            Blank,
            Normal,
            Membership,
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
                            "logId" | "log_id" => Ok(GeneratedField::LogId),
                            "blank" => Ok(GeneratedField::Blank),
                            "normal" => Ok(GeneratedField::Normal),
                            "membership" => Ok(GeneratedField::Membership),
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
                let mut log_id__ = None;
                let mut payload__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::LogId => {
                            if log_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logId"));
                            }
                            log_id__ = map.next_value()?;
                        }
                        GeneratedField::Blank => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blank"));
                            }
                            payload__ = map.next_value::<::std::option::Option<_>>()?.map(entry::Payload::Blank)
;
                        }
                        GeneratedField::Normal => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("normal"));
                            }
                            payload__ = map.next_value::<::std::option::Option<_>>()?.map(entry::Payload::Normal)
;
                        }
                        GeneratedField::Membership => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("membership"));
                            }
                            payload__ = map.next_value::<::std::option::Option<_>>()?.map(entry::Payload::Membership)
;
                        }
                    }
                }
                Ok(Entry {
                    log_id: log_id__,
                    payload: payload__,
                })
            }
        }
        deserializer.deserialize_struct("raft.v1.Entry", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HostNode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.addr.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("raft.v1.HostNode", len)?;
        if !self.addr.is_empty() {
            struct_ser.serialize_field("addr", &self.addr)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostNode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "addr",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Addr,
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
                            "addr" => Ok(GeneratedField::Addr),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostNode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct raft.v1.HostNode")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HostNode, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut addr__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Addr => {
                            if addr__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addr"));
                            }
                            addr__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(HostNode {
                    addr: addr__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("raft.v1.HostNode", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InstallSnapshotRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.vote.is_some() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        if self.offset != 0 {
            len += 1;
        }
        if !self.data.is_empty() {
            len += 1;
        }
        if self.done {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("raft.v1.InstallSnapshotRequest", len)?;
        if let Some(v) = self.vote.as_ref() {
            struct_ser.serialize_field("vote", v)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if self.offset != 0 {
            struct_ser.serialize_field("offset", ToString::to_string(&self.offset).as_str())?;
        }
        if !self.data.is_empty() {
            struct_ser.serialize_field("data", pbjson::private::base64::encode(&self.data).as_str())?;
        }
        if self.done {
            struct_ser.serialize_field("done", &self.done)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InstallSnapshotRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "vote",
            "metadata",
            "offset",
            "data",
            "done",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Vote,
            Metadata,
            Offset,
            Data,
            Done,
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
                            "vote" => Ok(GeneratedField::Vote),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "offset" => Ok(GeneratedField::Offset),
                            "data" => Ok(GeneratedField::Data),
                            "done" => Ok(GeneratedField::Done),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InstallSnapshotRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct raft.v1.InstallSnapshotRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<InstallSnapshotRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut vote__ = None;
                let mut metadata__ = None;
                let mut offset__ = None;
                let mut data__ = None;
                let mut done__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Vote => {
                            if vote__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vote"));
                            }
                            vote__ = map.next_value()?;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                        GeneratedField::Offset => {
                            if offset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offset"));
                            }
                            offset__ = 
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
                        GeneratedField::Done => {
                            if done__.is_some() {
                                return Err(serde::de::Error::duplicate_field("done"));
                            }
                            done__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(InstallSnapshotRequest {
                    vote: vote__,
                    metadata: metadata__,
                    offset: offset__.unwrap_or_default(),
                    data: data__.unwrap_or_default(),
                    done: done__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("raft.v1.InstallSnapshotRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InstallSnapshotResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.vote.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("raft.v1.InstallSnapshotResponse", len)?;
        if let Some(v) = self.vote.as_ref() {
            struct_ser.serialize_field("vote", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InstallSnapshotResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "vote",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Vote,
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
                            "vote" => Ok(GeneratedField::Vote),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InstallSnapshotResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct raft.v1.InstallSnapshotResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<InstallSnapshotResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut vote__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Vote => {
                            if vote__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vote"));
                            }
                            vote__ = map.next_value()?;
                        }
                    }
                }
                Ok(InstallSnapshotResponse {
                    vote: vote__,
                })
            }
        }
        deserializer.deserialize_struct("raft.v1.InstallSnapshotResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for KeyValuePair {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key.is_empty() {
            len += 1;
        }
        if self.create_revision != 0 {
            len += 1;
        }
        if self.mod_revision != 0 {
            len += 1;
        }
        if self.version != 0 {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        if self.lease != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("raft.v1.KeyValuePair", len)?;
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", pbjson::private::base64::encode(&self.key).as_str())?;
        }
        if self.create_revision != 0 {
            struct_ser.serialize_field("createRevision", ToString::to_string(&self.create_revision).as_str())?;
        }
        if self.mod_revision != 0 {
            struct_ser.serialize_field("modRevision", ToString::to_string(&self.mod_revision).as_str())?;
        }
        if self.version != 0 {
            struct_ser.serialize_field("version", ToString::to_string(&self.version).as_str())?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", pbjson::private::base64::encode(&self.value).as_str())?;
        }
        if self.lease != 0 {
            struct_ser.serialize_field("lease", ToString::to_string(&self.lease).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for KeyValuePair {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "create_revision",
            "createRevision",
            "mod_revision",
            "modRevision",
            "version",
            "value",
            "lease",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            CreateRevision,
            ModRevision,
            Version,
            Value,
            Lease,
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
                            "key" => Ok(GeneratedField::Key),
                            "createRevision" | "create_revision" => Ok(GeneratedField::CreateRevision),
                            "modRevision" | "mod_revision" => Ok(GeneratedField::ModRevision),
                            "version" => Ok(GeneratedField::Version),
                            "value" => Ok(GeneratedField::Value),
                            "lease" => Ok(GeneratedField::Lease),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = KeyValuePair;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct raft.v1.KeyValuePair")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<KeyValuePair, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut create_revision__ = None;
                let mut mod_revision__ = None;
                let mut version__ = None;
                let mut value__ = None;
                let mut lease__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CreateRevision => {
                            if create_revision__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createRevision"));
                            }
                            create_revision__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ModRevision => {
                            if mod_revision__.is_some() {
                                return Err(serde::de::Error::duplicate_field("modRevision"));
                            }
                            mod_revision__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Lease => {
                            if lease__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lease"));
                            }
                            lease__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(KeyValuePair {
                    key: key__.unwrap_or_default(),
                    create_revision: create_revision__.unwrap_or_default(),
                    mod_revision: mod_revision__.unwrap_or_default(),
                    version: version__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                    lease: lease__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("raft.v1.KeyValuePair", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LeaderId {
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
        if self.node_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("raft.v1.LeaderId", len)?;
        if self.term != 0 {
            struct_ser.serialize_field("term", ToString::to_string(&self.term).as_str())?;
        }
        if self.node_id != 0 {
            struct_ser.serialize_field("nodeId", ToString::to_string(&self.node_id).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LeaderId {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "term",
            "node_id",
            "nodeId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Term,
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
                            "term" => Ok(GeneratedField::Term),
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
            type Value = LeaderId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct raft.v1.LeaderId")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<LeaderId, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut term__ = None;
                let mut node_id__ = None;
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
                Ok(LeaderId {
                    term: term__.unwrap_or_default(),
                    node_id: node_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("raft.v1.LeaderId", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LogId {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.leader_id.is_some() {
            len += 1;
        }
        if self.index != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("raft.v1.LogId", len)?;
        if let Some(v) = self.leader_id.as_ref() {
            struct_ser.serialize_field("leaderId", v)?;
        }
        if self.index != 0 {
            struct_ser.serialize_field("index", ToString::to_string(&self.index).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LogId {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "leader_id",
            "leaderId",
            "index",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LeaderId,
            Index,
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
                            "leaderId" | "leader_id" => Ok(GeneratedField::LeaderId),
                            "index" => Ok(GeneratedField::Index),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LogId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct raft.v1.LogId")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<LogId, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut leader_id__ = None;
                let mut index__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::LeaderId => {
                            if leader_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("leaderId"));
                            }
                            leader_id__ = map.next_value()?;
                        }
                        GeneratedField::Index => {
                            if index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            index__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(LogId {
                    leader_id: leader_id__,
                    index: index__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("raft.v1.LogId", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Membership {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.configs.is_empty() {
            len += 1;
        }
        if !self.nodes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("raft.v1.Membership", len)?;
        if !self.configs.is_empty() {
            struct_ser.serialize_field("configs", &self.configs.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if !self.nodes.is_empty() {
            struct_ser.serialize_field("nodes", &self.nodes)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Membership {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "configs",
            "nodes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Configs,
            Nodes,
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
                            "configs" => Ok(GeneratedField::Configs),
                            "nodes" => Ok(GeneratedField::Nodes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Membership;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct raft.v1.Membership")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Membership, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut configs__ = None;
                let mut nodes__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Configs => {
                            if configs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configs"));
                            }
                            configs__ = 
                                Some(map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Nodes => {
                            if nodes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nodes"));
                            }
                            nodes__ = Some(
                                map.next_value::<std::collections::BTreeMap<::pbjson::private::NumberDeserialize<u64>, _>>()?
                                    .into_iter().map(|(k,v)| (k.0, v)).collect()
                            );
                        }
                    }
                }
                Ok(Membership {
                    configs: configs__.unwrap_or_default(),
                    nodes: nodes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("raft.v1.Membership", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MetaKeyValuePair {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.shard != 0 {
            len += 1;
        }
        if self.kvp.is_some() {
            len += 1;
        }
        if self.cache_policy.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("raft.v1.MetaKeyValuePair", len)?;
        if self.shard != 0 {
            struct_ser.serialize_field("shard", ToString::to_string(&self.shard).as_str())?;
        }
        if let Some(v) = self.kvp.as_ref() {
            struct_ser.serialize_field("kvp", v)?;
        }
        if let Some(v) = self.cache_policy.as_ref() {
            let v = RaftCachePolicy::from_i32(*v)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("cachePolicy", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MetaKeyValuePair {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "shard",
            "kvp",
            "cache_policy",
            "cachePolicy",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Shard,
            Kvp,
            CachePolicy,
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
                            "shard" => Ok(GeneratedField::Shard),
                            "kvp" => Ok(GeneratedField::Kvp),
                            "cachePolicy" | "cache_policy" => Ok(GeneratedField::CachePolicy),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MetaKeyValuePair;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct raft.v1.MetaKeyValuePair")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MetaKeyValuePair, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut shard__ = None;
                let mut kvp__ = None;
                let mut cache_policy__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Shard => {
                            if shard__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shard"));
                            }
                            shard__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Kvp => {
                            if kvp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kvp"));
                            }
                            kvp__ = map.next_value()?;
                        }
                        GeneratedField::CachePolicy => {
                            if cache_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cachePolicy"));
                            }
                            cache_policy__ = map.next_value::<::std::option::Option<RaftCachePolicy>>()?.map(|x| x as i32);
                        }
                    }
                }
                Ok(MetaKeyValuePair {
                    shard: shard__.unwrap_or_default(),
                    kvp: kvp__,
                    cache_policy: cache_policy__,
                })
            }
        }
        deserializer.deserialize_struct("raft.v1.MetaKeyValuePair", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RaftCachePolicy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "RAFT_CACHE_POLICY_UNSPECIFIED",
            Self::NoCache => "RAFT_CACHE_POLICY_NO_CACHE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for RaftCachePolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "RAFT_CACHE_POLICY_UNSPECIFIED",
            "RAFT_CACHE_POLICY_NO_CACHE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RaftCachePolicy;

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
                    .and_then(RaftCachePolicy::from_i32)
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
                    .and_then(RaftCachePolicy::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "RAFT_CACHE_POLICY_UNSPECIFIED" => Ok(RaftCachePolicy::Unspecified),
                    "RAFT_CACHE_POLICY_NO_CACHE" => Ok(RaftCachePolicy::NoCache),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for RaftDeleteKeyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.range.is_empty() {
            len += 1;
        }
        if !self.key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("raft.v1.RaftDeleteKeyRequest", len)?;
        if !self.range.is_empty() {
            struct_ser.serialize_field("range", pbjson::private::base64::encode(&self.range).as_str())?;
        }
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", pbjson::private::base64::encode(&self.key).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RaftDeleteKeyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "range",
            "key",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Range,
            Key,
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
                            "range" => Ok(GeneratedField::Range),
                            "key" => Ok(GeneratedField::Key),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RaftDeleteKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct raft.v1.RaftDeleteKeyRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RaftDeleteKeyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut range__ = None;
                let mut key__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Range => {
                            if range__.is_some() {
                                return Err(serde::de::Error::duplicate_field("range"));
                            }
                            range__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(RaftDeleteKeyRequest {
                    range: range__.unwrap_or_default(),
                    key: key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("raft.v1.RaftDeleteKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RaftDeleteKeyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("raft.v1.RaftDeleteKeyResponse", len)?;
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RaftDeleteKeyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Status,
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
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RaftDeleteKeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct raft.v1.RaftDeleteKeyResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RaftDeleteKeyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map.next_value()?;
                        }
                    }
                }
                Ok(RaftDeleteKeyResponse {
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("raft.v1.RaftDeleteKeyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RaftEntryRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.request.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("raft.v1.RaftEntryRequest", len)?;
        if let Some(v) = self.request.as_ref() {
            match v {
                raft_entry_request::Request::GetKey(v) => {
                    struct_ser.serialize_field("getKey", v)?;
                }
                raft_entry_request::Request::PutKey(v) => {
                    struct_ser.serialize_field("putKey", v)?;
                }
                raft_entry_request::Request::DeleteKey(v) => {
                    struct_ser.serialize_field("deleteKey", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RaftEntryRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "get_key",
            "getKey",
            "put_key",
            "putKey",
            "delete_key",
            "deleteKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GetKey,
            PutKey,
            DeleteKey,
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
                            "getKey" | "get_key" => Ok(GeneratedField::GetKey),
                            "putKey" | "put_key" => Ok(GeneratedField::PutKey),
                            "deleteKey" | "delete_key" => Ok(GeneratedField::DeleteKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RaftEntryRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct raft.v1.RaftEntryRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RaftEntryRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::GetKey => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("getKey"));
                            }
                            request__ = map.next_value::<::std::option::Option<_>>()?.map(raft_entry_request::Request::GetKey)
;
                        }
                        GeneratedField::PutKey => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("putKey"));
                            }
                            request__ = map.next_value::<::std::option::Option<_>>()?.map(raft_entry_request::Request::PutKey)
;
                        }
                        GeneratedField::DeleteKey => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deleteKey"));
                            }
                            request__ = map.next_value::<::std::option::Option<_>>()?.map(raft_entry_request::Request::DeleteKey)
;
                        }
                    }
                }
                Ok(RaftEntryRequest {
                    request: request__,
                })
            }
        }
        deserializer.deserialize_struct("raft.v1.RaftEntryRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RaftEntryResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.response.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("raft.v1.RaftEntryResponse", len)?;
        if let Some(v) = self.response.as_ref() {
            match v {
                raft_entry_response::Response::GetKey(v) => {
                    struct_ser.serialize_field("getKey", v)?;
                }
                raft_entry_response::Response::PutKey(v) => {
                    struct_ser.serialize_field("putKey", v)?;
                }
                raft_entry_response::Response::DeleteKey(v) => {
                    struct_ser.serialize_field("deleteKey", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RaftEntryResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "get_key",
            "getKey",
            "put_key",
            "putKey",
            "delete_key",
            "deleteKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GetKey,
            PutKey,
            DeleteKey,
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
                            "getKey" | "get_key" => Ok(GeneratedField::GetKey),
                            "putKey" | "put_key" => Ok(GeneratedField::PutKey),
                            "deleteKey" | "delete_key" => Ok(GeneratedField::DeleteKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RaftEntryResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct raft.v1.RaftEntryResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RaftEntryResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut response__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::GetKey => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("getKey"));
                            }
                            response__ = map.next_value::<::std::option::Option<_>>()?.map(raft_entry_response::Response::GetKey)
;
                        }
                        GeneratedField::PutKey => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("putKey"));
                            }
                            response__ = map.next_value::<::std::option::Option<_>>()?.map(raft_entry_response::Response::PutKey)
;
                        }
                        GeneratedField::DeleteKey => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deleteKey"));
                            }
                            response__ = map.next_value::<::std::option::Option<_>>()?.map(raft_entry_response::Response::DeleteKey)
;
                        }
                    }
                }
                Ok(RaftEntryResponse {
                    response: response__,
                })
            }
        }
        deserializer.deserialize_struct("raft.v1.RaftEntryResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RaftGetKeyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.range.is_empty() {
            len += 1;
        }
        if !self.key.is_empty() {
            len += 1;
        }
        if self.version.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("raft.v1.RaftGetKeyRequest", len)?;
        if !self.range.is_empty() {
            struct_ser.serialize_field("range", pbjson::private::base64::encode(&self.range).as_str())?;
        }
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", pbjson::private::base64::encode(&self.key).as_str())?;
        }
        if let Some(v) = self.version.as_ref() {
            struct_ser.serialize_field("version", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RaftGetKeyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "range",
            "key",
            "version",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Range,
            Key,
            Version,
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
                            "range" => Ok(GeneratedField::Range),
                            "key" => Ok(GeneratedField::Key),
                            "version" => Ok(GeneratedField::Version),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RaftGetKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct raft.v1.RaftGetKeyRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RaftGetKeyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut range__ = None;
                let mut key__ = None;
                let mut version__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Range => {
                            if range__.is_some() {
                                return Err(serde::de::Error::duplicate_field("range"));
                            }
                            range__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(RaftGetKeyRequest {
                    range: range__.unwrap_or_default(),
                    key: key__.unwrap_or_default(),
                    version: version__,
                })
            }
        }
        deserializer.deserialize_struct("raft.v1.RaftGetKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RaftGetKeyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.status.is_some() {
            len += 1;
        }
        if self.kvp.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("raft.v1.RaftGetKeyResponse", len)?;
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        if let Some(v) = self.kvp.as_ref() {
            struct_ser.serialize_field("kvp", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RaftGetKeyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "status",
            "kvp",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Status,
            Kvp,
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
                            "status" => Ok(GeneratedField::Status),
                            "kvp" => Ok(GeneratedField::Kvp),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RaftGetKeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct raft.v1.RaftGetKeyResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RaftGetKeyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut status__ = None;
                let mut kvp__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map.next_value()?;
                        }
                        GeneratedField::Kvp => {
                            if kvp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kvp"));
                            }
                            kvp__ = map.next_value()?;
                        }
                    }
                }
                Ok(RaftGetKeyResponse {
                    status: status__,
                    kvp: kvp__,
                })
            }
        }
        deserializer.deserialize_struct("raft.v1.RaftGetKeyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RaftPutKeyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.range.is_empty() {
            len += 1;
        }
        if self.kvp.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("raft.v1.RaftPutKeyRequest", len)?;
        if !self.range.is_empty() {
            struct_ser.serialize_field("range", pbjson::private::base64::encode(&self.range).as_str())?;
        }
        if let Some(v) = self.kvp.as_ref() {
            struct_ser.serialize_field("kvp", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RaftPutKeyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "range",
            "kvp",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Range,
            Kvp,
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
                            "range" => Ok(GeneratedField::Range),
                            "kvp" => Ok(GeneratedField::Kvp),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RaftPutKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct raft.v1.RaftPutKeyRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RaftPutKeyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut range__ = None;
                let mut kvp__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Range => {
                            if range__.is_some() {
                                return Err(serde::de::Error::duplicate_field("range"));
                            }
                            range__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Kvp => {
                            if kvp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kvp"));
                            }
                            kvp__ = map.next_value()?;
                        }
                    }
                }
                Ok(RaftPutKeyRequest {
                    range: range__.unwrap_or_default(),
                    kvp: kvp__,
                })
            }
        }
        deserializer.deserialize_struct("raft.v1.RaftPutKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RaftPutKeyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("raft.v1.RaftPutKeyResponse", len)?;
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RaftPutKeyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Status,
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
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RaftPutKeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct raft.v1.RaftPutKeyResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RaftPutKeyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map.next_value()?;
                        }
                    }
                }
                Ok(RaftPutKeyResponse {
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("raft.v1.RaftPutKeyResponse", FIELDS, GeneratedVisitor)
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
        if self.last_log_id.is_some() {
            len += 1;
        }
        if self.last_membership.is_some() {
            len += 1;
        }
        if !self.snapshot_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("raft.v1.SnapshotMetadata", len)?;
        if let Some(v) = self.last_log_id.as_ref() {
            struct_ser.serialize_field("lastLogId", v)?;
        }
        if let Some(v) = self.last_membership.as_ref() {
            struct_ser.serialize_field("lastMembership", v)?;
        }
        if !self.snapshot_id.is_empty() {
            struct_ser.serialize_field("snapshotId", &self.snapshot_id)?;
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
            "last_log_id",
            "lastLogId",
            "last_membership",
            "lastMembership",
            "snapshot_id",
            "snapshotId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LastLogId,
            LastMembership,
            SnapshotId,
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
                            "lastLogId" | "last_log_id" => Ok(GeneratedField::LastLogId),
                            "lastMembership" | "last_membership" => Ok(GeneratedField::LastMembership),
                            "snapshotId" | "snapshot_id" => Ok(GeneratedField::SnapshotId),
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
                let mut last_log_id__ = None;
                let mut last_membership__ = None;
                let mut snapshot_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::LastLogId => {
                            if last_log_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastLogId"));
                            }
                            last_log_id__ = map.next_value()?;
                        }
                        GeneratedField::LastMembership => {
                            if last_membership__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastMembership"));
                            }
                            last_membership__ = map.next_value()?;
                        }
                        GeneratedField::SnapshotId => {
                            if snapshot_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("snapshotId"));
                            }
                            snapshot_id__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(SnapshotMetadata {
                    last_log_id: last_log_id__,
                    last_membership: last_membership__,
                    snapshot_id: snapshot_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("raft.v1.SnapshotMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StoredMembership {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.log_id.is_some() {
            len += 1;
        }
        if self.membership.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("raft.v1.StoredMembership", len)?;
        if let Some(v) = self.log_id.as_ref() {
            struct_ser.serialize_field("logId", v)?;
        }
        if let Some(v) = self.membership.as_ref() {
            struct_ser.serialize_field("membership", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StoredMembership {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "log_id",
            "logId",
            "membership",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LogId,
            Membership,
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
                            "logId" | "log_id" => Ok(GeneratedField::LogId),
                            "membership" => Ok(GeneratedField::Membership),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StoredMembership;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct raft.v1.StoredMembership")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StoredMembership, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut log_id__ = None;
                let mut membership__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::LogId => {
                            if log_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logId"));
                            }
                            log_id__ = map.next_value()?;
                        }
                        GeneratedField::Membership => {
                            if membership__.is_some() {
                                return Err(serde::de::Error::duplicate_field("membership"));
                            }
                            membership__ = map.next_value()?;
                        }
                    }
                }
                Ok(StoredMembership {
                    log_id: log_id__,
                    membership: membership__,
                })
            }
        }
        deserializer.deserialize_struct("raft.v1.StoredMembership", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Success {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("raft.v1.Success", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Success {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Success;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct raft.v1.Success")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Success, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(Success {
                })
            }
        }
        deserializer.deserialize_struct("raft.v1.Success", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Vote {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.leader_id.is_some() {
            len += 1;
        }
        if self.committed {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("raft.v1.Vote", len)?;
        if let Some(v) = self.leader_id.as_ref() {
            struct_ser.serialize_field("leaderId", v)?;
        }
        if self.committed {
            struct_ser.serialize_field("committed", &self.committed)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Vote {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "leader_id",
            "leaderId",
            "committed",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LeaderId,
            Committed,
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
                            "leaderId" | "leader_id" => Ok(GeneratedField::LeaderId),
                            "committed" => Ok(GeneratedField::Committed),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Vote;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct raft.v1.Vote")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Vote, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut leader_id__ = None;
                let mut committed__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::LeaderId => {
                            if leader_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("leaderId"));
                            }
                            leader_id__ = map.next_value()?;
                        }
                        GeneratedField::Committed => {
                            if committed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("committed"));
                            }
                            committed__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Vote {
                    leader_id: leader_id__,
                    committed: committed__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("raft.v1.Vote", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VoteRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.vote.is_some() {
            len += 1;
        }
        if self.last_log_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("raft.v1.VoteRequest", len)?;
        if let Some(v) = self.vote.as_ref() {
            struct_ser.serialize_field("vote", v)?;
        }
        if let Some(v) = self.last_log_id.as_ref() {
            struct_ser.serialize_field("lastLogId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VoteRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "vote",
            "last_log_id",
            "lastLogId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Vote,
            LastLogId,
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
                            "vote" => Ok(GeneratedField::Vote),
                            "lastLogId" | "last_log_id" => Ok(GeneratedField::LastLogId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VoteRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct raft.v1.VoteRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<VoteRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut vote__ = None;
                let mut last_log_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Vote => {
                            if vote__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vote"));
                            }
                            vote__ = map.next_value()?;
                        }
                        GeneratedField::LastLogId => {
                            if last_log_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastLogId"));
                            }
                            last_log_id__ = map.next_value()?;
                        }
                    }
                }
                Ok(VoteRequest {
                    vote: vote__,
                    last_log_id: last_log_id__,
                })
            }
        }
        deserializer.deserialize_struct("raft.v1.VoteRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VoteResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.vote.is_some() {
            len += 1;
        }
        if self.vote_granted {
            len += 1;
        }
        if self.last_log_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("raft.v1.VoteResponse", len)?;
        if let Some(v) = self.vote.as_ref() {
            struct_ser.serialize_field("vote", v)?;
        }
        if self.vote_granted {
            struct_ser.serialize_field("voteGranted", &self.vote_granted)?;
        }
        if let Some(v) = self.last_log_id.as_ref() {
            struct_ser.serialize_field("lastLogId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VoteResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "vote",
            "vote_granted",
            "voteGranted",
            "last_log_id",
            "lastLogId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Vote,
            VoteGranted,
            LastLogId,
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
                            "vote" => Ok(GeneratedField::Vote),
                            "voteGranted" | "vote_granted" => Ok(GeneratedField::VoteGranted),
                            "lastLogId" | "last_log_id" => Ok(GeneratedField::LastLogId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VoteResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct raft.v1.VoteResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<VoteResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut vote__ = None;
                let mut vote_granted__ = None;
                let mut last_log_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Vote => {
                            if vote__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vote"));
                            }
                            vote__ = map.next_value()?;
                        }
                        GeneratedField::VoteGranted => {
                            if vote_granted__.is_some() {
                                return Err(serde::de::Error::duplicate_field("voteGranted"));
                            }
                            vote_granted__ = Some(map.next_value()?);
                        }
                        GeneratedField::LastLogId => {
                            if last_log_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastLogId"));
                            }
                            last_log_id__ = map.next_value()?;
                        }
                    }
                }
                Ok(VoteResponse {
                    vote: vote__,
                    vote_granted: vote_granted__.unwrap_or_default(),
                    last_log_id: last_log_id__,
                })
            }
        }
        deserializer.deserialize_struct("raft.v1.VoteResponse", FIELDS, GeneratedVisitor)
    }
}
