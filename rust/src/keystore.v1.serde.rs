// @generated
impl serde::Serialize for KeyRange {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.start_key.is_empty() {
            len += 1;
        }
        if !self.end_key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("keystore.v1.KeyRange", len)?;
        if !self.start_key.is_empty() {
            struct_ser.serialize_field("startKey", pbjson::private::base64::encode(&self.start_key).as_str())?;
        }
        if !self.end_key.is_empty() {
            struct_ser.serialize_field("endKey", pbjson::private::base64::encode(&self.end_key).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for KeyRange {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "start_key",
            "startKey",
            "end_key",
            "endKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StartKey,
            EndKey,
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
                            "startKey" | "start_key" => Ok(GeneratedField::StartKey),
                            "endKey" | "end_key" => Ok(GeneratedField::EndKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = KeyRange;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct keystore.v1.KeyRange")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<KeyRange, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut start_key__ = None;
                let mut end_key__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::StartKey => {
                            if start_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startKey"));
                            }
                            start_key__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::EndKey => {
                            if end_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endKey"));
                            }
                            end_key__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(KeyRange {
                    start_key: start_key__.unwrap_or_default(),
                    end_key: end_key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("keystore.v1.KeyRange", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for KeyRangeMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id != 0 {
            len += 1;
        }
        if self.owner.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("keystore.v1.KeyRangeMetadata", len)?;
        if self.id != 0 {
            struct_ser.serialize_field("id", ToString::to_string(&self.id).as_str())?;
        }
        if let Some(v) = self.owner.as_ref() {
            struct_ser.serialize_field("owner", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for KeyRangeMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "owner",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Owner,
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
                            "owner" => Ok(GeneratedField::Owner),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = KeyRangeMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct keystore.v1.KeyRangeMetadata")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<KeyRangeMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut owner__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Owner => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner__ = map.next_value()?;
                        }
                    }
                }
                Ok(KeyRangeMetadata {
                    id: id__.unwrap_or_default(),
                    owner: owner__,
                })
            }
        }
        deserializer.deserialize_struct("keystore.v1.KeyRangeMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for KeyRangeOwner {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.range.is_some() {
            len += 1;
        }
        if self.leader != 0 {
            len += 1;
        }
        if self.host_node.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("keystore.v1.KeyRangeOwner", len)?;
        if let Some(v) = self.range.as_ref() {
            struct_ser.serialize_field("range", v)?;
        }
        if self.leader != 0 {
            struct_ser.serialize_field("leader", ToString::to_string(&self.leader).as_str())?;
        }
        if let Some(v) = self.host_node.as_ref() {
            struct_ser.serialize_field("hostNode", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for KeyRangeOwner {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "range",
            "leader",
            "host_node",
            "hostNode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Range,
            Leader,
            HostNode,
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
                            "leader" => Ok(GeneratedField::Leader),
                            "hostNode" | "host_node" => Ok(GeneratedField::HostNode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = KeyRangeOwner;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct keystore.v1.KeyRangeOwner")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<KeyRangeOwner, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut range__ = None;
                let mut leader__ = None;
                let mut host_node__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Range => {
                            if range__.is_some() {
                                return Err(serde::de::Error::duplicate_field("range"));
                            }
                            range__ = map.next_value()?;
                        }
                        GeneratedField::Leader => {
                            if leader__.is_some() {
                                return Err(serde::de::Error::duplicate_field("leader"));
                            }
                            leader__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::HostNode => {
                            if host_node__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostNode"));
                            }
                            host_node__ = map.next_value()?;
                        }
                    }
                }
                Ok(KeyRangeOwner {
                    range: range__,
                    leader: leader__.unwrap_or_default(),
                    host_node: host_node__,
                })
            }
        }
        deserializer.deserialize_struct("keystore.v1.KeyRangeOwner", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for KeystoreMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.ranges.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("keystore.v1.KeystoreMetadata", len)?;
        if !self.ranges.is_empty() {
            struct_ser.serialize_field("ranges", &self.ranges)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for KeystoreMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ranges",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Ranges,
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
                            "ranges" => Ok(GeneratedField::Ranges),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = KeystoreMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct keystore.v1.KeystoreMetadata")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<KeystoreMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ranges__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Ranges => {
                            if ranges__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ranges"));
                            }
                            ranges__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(KeystoreMetadata {
                    ranges: ranges__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("keystore.v1.KeystoreMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RangedKeyValuePair {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.range.is_some() {
            len += 1;
        }
        if self.kvp.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("keystore.v1.RangedKeyValuePair", len)?;
        if let Some(v) = self.range.as_ref() {
            struct_ser.serialize_field("range", v)?;
        }
        if let Some(v) = self.kvp.as_ref() {
            struct_ser.serialize_field("kvp", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RangedKeyValuePair {
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
            type Value = RangedKeyValuePair;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct keystore.v1.RangedKeyValuePair")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RangedKeyValuePair, V::Error>
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
                            range__ = map.next_value()?;
                        }
                        GeneratedField::Kvp => {
                            if kvp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kvp"));
                            }
                            kvp__ = map.next_value()?;
                        }
                    }
                }
                Ok(RangedKeyValuePair {
                    range: range__,
                    kvp: kvp__,
                })
            }
        }
        deserializer.deserialize_struct("keystore.v1.RangedKeyValuePair", FIELDS, GeneratedVisitor)
    }
}
