//
//   Pleiades Source Code
//   Copyright (C) 2023 Sienna Lloyd, Pleiades Authors
//
//   This program is free software: you can redistribute it and/or modify
//   it under the terms of the GNU General Public License as published by
//   the Free Software Foundation, either version 3 of the License, or
//   (at your option) any later version.
//
//   This program is distributed in the hope that it will be useful,
//   but WITHOUT ANY WARRANTY; without even the implied warranty of
//   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//   GNU General Public License for more details.
//
//   You should have received a copy of the GNU General Public License
//   along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.31.0
// 	protoc        (unknown)
// source: keystore/v1/keystore.proto

package keystore

import (
	raft "github.com/mxplusb/pleiades/api/v1/raft"
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	protoimpl "google.golang.org/protobuf/runtime/protoimpl"
	reflect "reflect"
	sync "sync"
)

const (
	// Verify that this generated code is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(20 - protoimpl.MinVersion)
	// Verify that runtime/protoimpl is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(protoimpl.MaxVersion - 20)
)

type KeystoreMetadata struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Ranges []*KeyRange `protobuf:"bytes,1,rep,name=ranges,proto3" json:"ranges,omitempty"`
}

func (x *KeystoreMetadata) Reset() {
	*x = KeystoreMetadata{}
	if protoimpl.UnsafeEnabled {
		mi := &file_keystore_v1_keystore_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *KeystoreMetadata) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*KeystoreMetadata) ProtoMessage() {}

func (x *KeystoreMetadata) ProtoReflect() protoreflect.Message {
	mi := &file_keystore_v1_keystore_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use KeystoreMetadata.ProtoReflect.Descriptor instead.
func (*KeystoreMetadata) Descriptor() ([]byte, []int) {
	return file_keystore_v1_keystore_proto_rawDescGZIP(), []int{0}
}

func (x *KeystoreMetadata) GetRanges() []*KeyRange {
	if x != nil {
		return x.Ranges
	}
	return nil
}

type KeyRangeMetadata struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Id    uint64         `protobuf:"varint,1,opt,name=id,proto3" json:"id,omitempty"`
	Owner *KeyRangeOwner `protobuf:"bytes,2,opt,name=owner,proto3" json:"owner,omitempty"`
}

func (x *KeyRangeMetadata) Reset() {
	*x = KeyRangeMetadata{}
	if protoimpl.UnsafeEnabled {
		mi := &file_keystore_v1_keystore_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *KeyRangeMetadata) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*KeyRangeMetadata) ProtoMessage() {}

func (x *KeyRangeMetadata) ProtoReflect() protoreflect.Message {
	mi := &file_keystore_v1_keystore_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use KeyRangeMetadata.ProtoReflect.Descriptor instead.
func (*KeyRangeMetadata) Descriptor() ([]byte, []int) {
	return file_keystore_v1_keystore_proto_rawDescGZIP(), []int{1}
}

func (x *KeyRangeMetadata) GetId() uint64 {
	if x != nil {
		return x.Id
	}
	return 0
}

func (x *KeyRangeMetadata) GetOwner() *KeyRangeOwner {
	if x != nil {
		return x.Owner
	}
	return nil
}

type KeyRangeOwner struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Range    *KeyRange      `protobuf:"bytes,1,opt,name=range,proto3" json:"range,omitempty"`
	Leader   uint64         `protobuf:"varint,2,opt,name=leader,proto3" json:"leader,omitempty"`
	HostNode *raft.HostNode `protobuf:"bytes,3,opt,name=host_node,json=hostNode,proto3" json:"host_node,omitempty"`
}

func (x *KeyRangeOwner) Reset() {
	*x = KeyRangeOwner{}
	if protoimpl.UnsafeEnabled {
		mi := &file_keystore_v1_keystore_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *KeyRangeOwner) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*KeyRangeOwner) ProtoMessage() {}

func (x *KeyRangeOwner) ProtoReflect() protoreflect.Message {
	mi := &file_keystore_v1_keystore_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use KeyRangeOwner.ProtoReflect.Descriptor instead.
func (*KeyRangeOwner) Descriptor() ([]byte, []int) {
	return file_keystore_v1_keystore_proto_rawDescGZIP(), []int{2}
}

func (x *KeyRangeOwner) GetRange() *KeyRange {
	if x != nil {
		return x.Range
	}
	return nil
}

func (x *KeyRangeOwner) GetLeader() uint64 {
	if x != nil {
		return x.Leader
	}
	return 0
}

func (x *KeyRangeOwner) GetHostNode() *raft.HostNode {
	if x != nil {
		return x.HostNode
	}
	return nil
}

// A KeyRange is a slice of the overall key space. It is defined by its start
// and end keys. The start key is included in the range, the end key is not.
type KeyRange struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// start_key is the first key in the range, inclusive. It is a uint128
	// encoded as a byte array.
	StartKey []byte `protobuf:"bytes,1,opt,name=start_key,json=startKey,proto3" json:"start_key,omitempty"`
	// end_key is the upper bound on the end of the range, exclusive. It is a
	// uint128 encoded as a byte array.
	EndKey []byte `protobuf:"bytes,2,opt,name=end_key,json=endKey,proto3" json:"end_key,omitempty"`
}

func (x *KeyRange) Reset() {
	*x = KeyRange{}
	if protoimpl.UnsafeEnabled {
		mi := &file_keystore_v1_keystore_proto_msgTypes[3]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *KeyRange) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*KeyRange) ProtoMessage() {}

func (x *KeyRange) ProtoReflect() protoreflect.Message {
	mi := &file_keystore_v1_keystore_proto_msgTypes[3]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use KeyRange.ProtoReflect.Descriptor instead.
func (*KeyRange) Descriptor() ([]byte, []int) {
	return file_keystore_v1_keystore_proto_rawDescGZIP(), []int{3}
}

func (x *KeyRange) GetStartKey() []byte {
	if x != nil {
		return x.StartKey
	}
	return nil
}

func (x *KeyRange) GetEndKey() []byte {
	if x != nil {
		return x.EndKey
	}
	return nil
}

// A key-value pair with a key range associated with it.
type RangedKeyValuePair struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Range *KeyRange          `protobuf:"bytes,1,opt,name=range,proto3" json:"range,omitempty"`
	Kvp   *raft.KeyValuePair `protobuf:"bytes,2,opt,name=kvp,proto3" json:"kvp,omitempty"`
}

func (x *RangedKeyValuePair) Reset() {
	*x = RangedKeyValuePair{}
	if protoimpl.UnsafeEnabled {
		mi := &file_keystore_v1_keystore_proto_msgTypes[4]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *RangedKeyValuePair) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*RangedKeyValuePair) ProtoMessage() {}

func (x *RangedKeyValuePair) ProtoReflect() protoreflect.Message {
	mi := &file_keystore_v1_keystore_proto_msgTypes[4]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use RangedKeyValuePair.ProtoReflect.Descriptor instead.
func (*RangedKeyValuePair) Descriptor() ([]byte, []int) {
	return file_keystore_v1_keystore_proto_rawDescGZIP(), []int{4}
}

func (x *RangedKeyValuePair) GetRange() *KeyRange {
	if x != nil {
		return x.Range
	}
	return nil
}

func (x *RangedKeyValuePair) GetKvp() *raft.KeyValuePair {
	if x != nil {
		return x.Kvp
	}
	return nil
}

var File_keystore_v1_keystore_proto protoreflect.FileDescriptor

var file_keystore_v1_keystore_proto_rawDesc = []byte{
	0x0a, 0x1a, 0x6b, 0x65, 0x79, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x2f, 0x76, 0x31, 0x2f, 0x6b, 0x65,
	0x79, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x6b, 0x65,
	0x79, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x31, 0x1a, 0x12, 0x72, 0x61, 0x66, 0x74, 0x2f,
	0x76, 0x31, 0x2f, 0x72, 0x61, 0x66, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x41, 0x0a,
	0x10, 0x4b, 0x65, 0x79, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74,
	0x61, 0x12, 0x2d, 0x0a, 0x06, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28,
	0x0b, 0x32, 0x15, 0x2e, 0x6b, 0x65, 0x79, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x31, 0x2e,
	0x4b, 0x65, 0x79, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x52, 0x06, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x73,
	0x22, 0x54, 0x0a, 0x10, 0x4b, 0x65, 0x79, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x4d, 0x65, 0x74, 0x61,
	0x64, 0x61, 0x74, 0x61, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04,
	0x52, 0x02, 0x69, 0x64, 0x12, 0x30, 0x0a, 0x05, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x18, 0x02, 0x20,
	0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x6b, 0x65, 0x79, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x2e, 0x76,
	0x31, 0x2e, 0x4b, 0x65, 0x79, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x4f, 0x77, 0x6e, 0x65, 0x72, 0x52,
	0x05, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x22, 0x84, 0x01, 0x0a, 0x0d, 0x4b, 0x65, 0x79, 0x52, 0x61,
	0x6e, 0x67, 0x65, 0x4f, 0x77, 0x6e, 0x65, 0x72, 0x12, 0x2b, 0x0a, 0x05, 0x72, 0x61, 0x6e, 0x67,
	0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x6b, 0x65, 0x79, 0x73, 0x74, 0x6f,
	0x72, 0x65, 0x2e, 0x76, 0x31, 0x2e, 0x4b, 0x65, 0x79, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x52, 0x05,
	0x72, 0x61, 0x6e, 0x67, 0x65, 0x12, 0x16, 0x0a, 0x06, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18,
	0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x06, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x2e, 0x0a,
	0x09, 0x68, 0x6f, 0x73, 0x74, 0x5f, 0x6e, 0x6f, 0x64, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b,
	0x32, 0x11, 0x2e, 0x72, 0x61, 0x66, 0x74, 0x2e, 0x76, 0x31, 0x2e, 0x48, 0x6f, 0x73, 0x74, 0x4e,
	0x6f, 0x64, 0x65, 0x52, 0x08, 0x68, 0x6f, 0x73, 0x74, 0x4e, 0x6f, 0x64, 0x65, 0x22, 0x40, 0x0a,
	0x08, 0x4b, 0x65, 0x79, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x12, 0x1b, 0x0a, 0x09, 0x73, 0x74, 0x61,
	0x72, 0x74, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x08, 0x73, 0x74,
	0x61, 0x72, 0x74, 0x4b, 0x65, 0x79, 0x12, 0x17, 0x0a, 0x07, 0x65, 0x6e, 0x64, 0x5f, 0x6b, 0x65,
	0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x06, 0x65, 0x6e, 0x64, 0x4b, 0x65, 0x79, 0x22,
	0x6a, 0x0a, 0x12, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x64, 0x4b, 0x65, 0x79, 0x56, 0x61, 0x6c, 0x75,
	0x65, 0x50, 0x61, 0x69, 0x72, 0x12, 0x2b, 0x0a, 0x05, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x18, 0x01,
	0x20, 0x01, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x6b, 0x65, 0x79, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x2e,
	0x76, 0x31, 0x2e, 0x4b, 0x65, 0x79, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x52, 0x05, 0x72, 0x61, 0x6e,
	0x67, 0x65, 0x12, 0x27, 0x0a, 0x03, 0x6b, 0x76, 0x70, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32,
	0x15, 0x2e, 0x72, 0x61, 0x66, 0x74, 0x2e, 0x76, 0x31, 0x2e, 0x4b, 0x65, 0x79, 0x56, 0x61, 0x6c,
	0x75, 0x65, 0x50, 0x61, 0x69, 0x72, 0x52, 0x03, 0x6b, 0x76, 0x70, 0x42, 0x9a, 0x01, 0x0a, 0x0f,
	0x63, 0x6f, 0x6d, 0x2e, 0x6b, 0x65, 0x79, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x31, 0x42,
	0x0d, 0x4b, 0x65, 0x79, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01,
	0x5a, 0x2b, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x6d, 0x78, 0x70,
	0x6c, 0x75, 0x73, 0x62, 0x2f, 0x70, 0x6c, 0x65, 0x69, 0x61, 0x64, 0x65, 0x73, 0x2f, 0x61, 0x70,
	0x69, 0x2f, 0x76, 0x31, 0x2f, 0x6b, 0x65, 0x79, 0x73, 0x74, 0x6f, 0x72, 0x65, 0xa2, 0x02, 0x03,
	0x4b, 0x58, 0x58, 0xaa, 0x02, 0x0b, 0x4b, 0x65, 0x79, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x2e, 0x56,
	0x31, 0xca, 0x02, 0x0b, 0x4b, 0x65, 0x79, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x5c, 0x56, 0x31, 0xe2,
	0x02, 0x17, 0x4b, 0x65, 0x79, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x5c, 0x56, 0x31, 0x5c, 0x47, 0x50,
	0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea, 0x02, 0x0c, 0x4b, 0x65, 0x79, 0x73,
	0x74, 0x6f, 0x72, 0x65, 0x3a, 0x3a, 0x56, 0x31, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_keystore_v1_keystore_proto_rawDescOnce sync.Once
	file_keystore_v1_keystore_proto_rawDescData = file_keystore_v1_keystore_proto_rawDesc
)

func file_keystore_v1_keystore_proto_rawDescGZIP() []byte {
	file_keystore_v1_keystore_proto_rawDescOnce.Do(func() {
		file_keystore_v1_keystore_proto_rawDescData = protoimpl.X.CompressGZIP(file_keystore_v1_keystore_proto_rawDescData)
	})
	return file_keystore_v1_keystore_proto_rawDescData
}

var file_keystore_v1_keystore_proto_msgTypes = make([]protoimpl.MessageInfo, 5)
var file_keystore_v1_keystore_proto_goTypes = []interface{}{
	(*KeystoreMetadata)(nil),   // 0: keystore.v1.KeystoreMetadata
	(*KeyRangeMetadata)(nil),   // 1: keystore.v1.KeyRangeMetadata
	(*KeyRangeOwner)(nil),      // 2: keystore.v1.KeyRangeOwner
	(*KeyRange)(nil),           // 3: keystore.v1.KeyRange
	(*RangedKeyValuePair)(nil), // 4: keystore.v1.RangedKeyValuePair
	(*raft.HostNode)(nil),      // 5: raft.v1.HostNode
	(*raft.KeyValuePair)(nil),  // 6: raft.v1.KeyValuePair
}
var file_keystore_v1_keystore_proto_depIdxs = []int32{
	3, // 0: keystore.v1.KeystoreMetadata.ranges:type_name -> keystore.v1.KeyRange
	2, // 1: keystore.v1.KeyRangeMetadata.owner:type_name -> keystore.v1.KeyRangeOwner
	3, // 2: keystore.v1.KeyRangeOwner.range:type_name -> keystore.v1.KeyRange
	5, // 3: keystore.v1.KeyRangeOwner.host_node:type_name -> raft.v1.HostNode
	3, // 4: keystore.v1.RangedKeyValuePair.range:type_name -> keystore.v1.KeyRange
	6, // 5: keystore.v1.RangedKeyValuePair.kvp:type_name -> raft.v1.KeyValuePair
	6, // [6:6] is the sub-list for method output_type
	6, // [6:6] is the sub-list for method input_type
	6, // [6:6] is the sub-list for extension type_name
	6, // [6:6] is the sub-list for extension extendee
	0, // [0:6] is the sub-list for field type_name
}

func init() { file_keystore_v1_keystore_proto_init() }
func file_keystore_v1_keystore_proto_init() {
	if File_keystore_v1_keystore_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_keystore_v1_keystore_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*KeystoreMetadata); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_keystore_v1_keystore_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*KeyRangeMetadata); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_keystore_v1_keystore_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*KeyRangeOwner); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_keystore_v1_keystore_proto_msgTypes[3].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*KeyRange); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_keystore_v1_keystore_proto_msgTypes[4].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*RangedKeyValuePair); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_keystore_v1_keystore_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   5,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_keystore_v1_keystore_proto_goTypes,
		DependencyIndexes: file_keystore_v1_keystore_proto_depIdxs,
		MessageInfos:      file_keystore_v1_keystore_proto_msgTypes,
	}.Build()
	File_keystore_v1_keystore_proto = out.File
	file_keystore_v1_keystore_proto_rawDesc = nil
	file_keystore_v1_keystore_proto_goTypes = nil
	file_keystore_v1_keystore_proto_depIdxs = nil
}
