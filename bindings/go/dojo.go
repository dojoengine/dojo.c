package dojo

// #cgo CFLAGS: -I${SRCDIR}
// #cgo LDFLAGS: -L${SRCDIR}/../../target/release -ldojo_uniffi
// #include <dojo.h>
import "C"

import (
	"bytes"
	"encoding/binary"
	"fmt"
	"io"
	"math"
	"runtime"
	"sync"
	"sync/atomic"
	"unsafe"
)

// This is needed, because as of go 1.24
// type RustBuffer C.RustBuffer cannot have methods,
// RustBuffer is treated as non-local type
type GoRustBuffer struct {
	inner C.RustBuffer
}

type RustBufferI interface {
	AsReader() *bytes.Reader
	Free()
	ToGoBytes() []byte
	Data() unsafe.Pointer
	Len() uint64
	Capacity() uint64
}

func RustBufferFromExternal(b RustBufferI) GoRustBuffer {
	return GoRustBuffer{
		inner: C.RustBuffer{
			capacity: C.uint64_t(b.Capacity()),
			len:      C.uint64_t(b.Len()),
			data:     (*C.uchar)(b.Data()),
		},
	}
}

func (cb GoRustBuffer) Capacity() uint64 {
	return uint64(cb.inner.capacity)
}

func (cb GoRustBuffer) Len() uint64 {
	return uint64(cb.inner.len)
}

func (cb GoRustBuffer) Data() unsafe.Pointer {
	return unsafe.Pointer(cb.inner.data)
}

func (cb GoRustBuffer) AsReader() *bytes.Reader {
	b := unsafe.Slice((*byte)(cb.inner.data), C.uint64_t(cb.inner.len))
	return bytes.NewReader(b)
}

func (cb GoRustBuffer) Free() {
	rustCall(func(status *C.RustCallStatus) bool {
		C.ffi_dojo_uniffi_rustbuffer_free(cb.inner, status)
		return false
	})
}

func (cb GoRustBuffer) ToGoBytes() []byte {
	return C.GoBytes(unsafe.Pointer(cb.inner.data), C.int(cb.inner.len))
}

func stringToRustBuffer(str string) C.RustBuffer {
	return bytesToRustBuffer([]byte(str))
}

func bytesToRustBuffer(b []byte) C.RustBuffer {
	if len(b) == 0 {
		return C.RustBuffer{}
	}
	// We can pass the pointer along here, as it is pinned
	// for the duration of this call
	foreign := C.ForeignBytes{
		len:  C.int(len(b)),
		data: (*C.uchar)(unsafe.Pointer(&b[0])),
	}

	return rustCall(func(status *C.RustCallStatus) C.RustBuffer {
		return C.ffi_dojo_uniffi_rustbuffer_from_bytes(foreign, status)
	})
}

type BufLifter[GoType any] interface {
	Lift(value RustBufferI) GoType
}

type BufLowerer[GoType any] interface {
	Lower(value GoType) C.RustBuffer
}

type BufReader[GoType any] interface {
	Read(reader io.Reader) GoType
}

type BufWriter[GoType any] interface {
	Write(writer io.Writer, value GoType)
}

func LowerIntoRustBuffer[GoType any](bufWriter BufWriter[GoType], value GoType) C.RustBuffer {
	// This might be not the most efficient way but it does not require knowing allocation size
	// beforehand
	var buffer bytes.Buffer
	bufWriter.Write(&buffer, value)

	bytes, err := io.ReadAll(&buffer)
	if err != nil {
		panic(fmt.Errorf("reading written data: %w", err))
	}
	return bytesToRustBuffer(bytes)
}

func LiftFromRustBuffer[GoType any](bufReader BufReader[GoType], rbuf RustBufferI) GoType {
	defer rbuf.Free()
	reader := rbuf.AsReader()
	item := bufReader.Read(reader)
	if reader.Len() > 0 {
		// TODO: Remove this
		leftover, _ := io.ReadAll(reader)
		panic(fmt.Errorf("Junk remaining in buffer after lifting: %s", string(leftover)))
	}
	return item
}

func rustCallWithError[E any, U any](converter BufReader[*E], callback func(*C.RustCallStatus) U) (U, *E) {
	var status C.RustCallStatus
	returnValue := callback(&status)
	err := checkCallStatus(converter, status)
	return returnValue, err
}

func checkCallStatus[E any](converter BufReader[*E], status C.RustCallStatus) *E {
	switch status.code {
	case 0:
		return nil
	case 1:
		return LiftFromRustBuffer(converter, GoRustBuffer{inner: status.errorBuf})
	case 2:
		// when the rust code sees a panic, it tries to construct a rustBuffer
		// with the message.  but if that code panics, then it just sends back
		// an empty buffer.
		if status.errorBuf.len > 0 {
			panic(fmt.Errorf("%s", FfiConverterStringINSTANCE.Lift(GoRustBuffer{inner: status.errorBuf})))
		} else {
			panic(fmt.Errorf("Rust panicked while handling Rust panic"))
		}
	default:
		panic(fmt.Errorf("unknown status code: %d", status.code))
	}
}

func checkCallStatusUnknown(status C.RustCallStatus) error {
	switch status.code {
	case 0:
		return nil
	case 1:
		panic(fmt.Errorf("function not returning an error returned an error"))
	case 2:
		// when the rust code sees a panic, it tries to construct a C.RustBuffer
		// with the message.  but if that code panics, then it just sends back
		// an empty buffer.
		if status.errorBuf.len > 0 {
			panic(fmt.Errorf("%s", FfiConverterStringINSTANCE.Lift(GoRustBuffer{
				inner: status.errorBuf,
			})))
		} else {
			panic(fmt.Errorf("Rust panicked while handling Rust panic"))
		}
	default:
		return fmt.Errorf("unknown status code: %d", status.code)
	}
}

func rustCall[U any](callback func(*C.RustCallStatus) U) U {
	returnValue, err := rustCallWithError[error](nil, callback)
	if err != nil {
		panic(err)
	}
	return returnValue
}

type NativeError interface {
	AsError() error
}

func writeInt8(writer io.Writer, value int8) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeUint8(writer io.Writer, value uint8) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeInt16(writer io.Writer, value int16) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeUint16(writer io.Writer, value uint16) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeInt32(writer io.Writer, value int32) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeUint32(writer io.Writer, value uint32) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeInt64(writer io.Writer, value int64) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeUint64(writer io.Writer, value uint64) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeFloat32(writer io.Writer, value float32) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeFloat64(writer io.Writer, value float64) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func readInt8(reader io.Reader) int8 {
	var result int8
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readUint8(reader io.Reader) uint8 {
	var result uint8
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readInt16(reader io.Reader) int16 {
	var result int16
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readUint16(reader io.Reader) uint16 {
	var result uint16
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readInt32(reader io.Reader) int32 {
	var result int32
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readUint32(reader io.Reader) uint32 {
	var result uint32
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readInt64(reader io.Reader) int64 {
	var result int64
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readUint64(reader io.Reader) uint64 {
	var result uint64
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readFloat32(reader io.Reader) float32 {
	var result float32
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readFloat64(reader io.Reader) float64 {
	var result float64
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func init() {

	FfiConverterCallbackInterfaceEntityUpdateCallbackINSTANCE.register()
	FfiConverterCallbackInterfaceEventUpdateCallbackINSTANCE.register()
	FfiConverterCallbackInterfaceTokenBalanceUpdateCallbackINSTANCE.register()
	FfiConverterCallbackInterfaceTokenUpdateCallbackINSTANCE.register()
	FfiConverterCallbackInterfaceTransactionUpdateCallbackINSTANCE.register()
	uniffiCheckChecksums()
}

func uniffiCheckChecksums() {
	// Get the bindings contract version from our ComponentInterface
	bindingsContractVersion := 26
	// Get the scaffolding contract version by calling the into the dylib
	scaffoldingContractVersion := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint32_t {
		return C.ffi_dojo_uniffi_uniffi_contract_version()
	})
	if bindingsContractVersion != int(scaffoldingContractVersion) {
		// If this happens try cleaning and rebuilding your project
		panic("dojo: UniFFI contract version mismatch")
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_dojo_uniffi_checksum_method_toriiclient_achievements()
		})
		if checksum != 22681 {
			// If this happens try cleaning and rebuilding your project
			panic("dojo: uniffi_dojo_uniffi_checksum_method_toriiclient_achievements: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_dojo_uniffi_checksum_method_toriiclient_activities()
		})
		if checksum != 30695 {
			// If this happens try cleaning and rebuilding your project
			panic("dojo: uniffi_dojo_uniffi_checksum_method_toriiclient_activities: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_dojo_uniffi_checksum_method_toriiclient_aggregations()
		})
		if checksum != 59796 {
			// If this happens try cleaning and rebuilding your project
			panic("dojo: uniffi_dojo_uniffi_checksum_method_toriiclient_aggregations: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_dojo_uniffi_checksum_method_toriiclient_cancel_subscription()
		})
		if checksum != 1222 {
			// If this happens try cleaning and rebuilding your project
			panic("dojo: uniffi_dojo_uniffi_checksum_method_toriiclient_cancel_subscription: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_dojo_uniffi_checksum_method_toriiclient_contracts()
		})
		if checksum != 1019 {
			// If this happens try cleaning and rebuilding your project
			panic("dojo: uniffi_dojo_uniffi_checksum_method_toriiclient_contracts: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_dojo_uniffi_checksum_method_toriiclient_controllers()
		})
		if checksum != 9802 {
			// If this happens try cleaning and rebuilding your project
			panic("dojo: uniffi_dojo_uniffi_checksum_method_toriiclient_controllers: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_dojo_uniffi_checksum_method_toriiclient_entities()
		})
		if checksum != 5808 {
			// If this happens try cleaning and rebuilding your project
			panic("dojo: uniffi_dojo_uniffi_checksum_method_toriiclient_entities: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_dojo_uniffi_checksum_method_toriiclient_event_messages()
		})
		if checksum != 45410 {
			// If this happens try cleaning and rebuilding your project
			panic("dojo: uniffi_dojo_uniffi_checksum_method_toriiclient_event_messages: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_dojo_uniffi_checksum_method_toriiclient_player_achievements()
		})
		if checksum != 12776 {
			// If this happens try cleaning and rebuilding your project
			panic("dojo: uniffi_dojo_uniffi_checksum_method_toriiclient_player_achievements: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_dojo_uniffi_checksum_method_toriiclient_publish_message()
		})
		if checksum != 44715 {
			// If this happens try cleaning and rebuilding your project
			panic("dojo: uniffi_dojo_uniffi_checksum_method_toriiclient_publish_message: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_dojo_uniffi_checksum_method_toriiclient_publish_message_batch()
		})
		if checksum != 50961 {
			// If this happens try cleaning and rebuilding your project
			panic("dojo: uniffi_dojo_uniffi_checksum_method_toriiclient_publish_message_batch: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_dojo_uniffi_checksum_method_toriiclient_sql()
		})
		if checksum != 59851 {
			// If this happens try cleaning and rebuilding your project
			panic("dojo: uniffi_dojo_uniffi_checksum_method_toriiclient_sql: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_dojo_uniffi_checksum_method_toriiclient_starknet_events()
		})
		if checksum != 22346 {
			// If this happens try cleaning and rebuilding your project
			panic("dojo: uniffi_dojo_uniffi_checksum_method_toriiclient_starknet_events: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_dojo_uniffi_checksum_method_toriiclient_subscribe_entity_updates()
		})
		if checksum != 47448 {
			// If this happens try cleaning and rebuilding your project
			panic("dojo: uniffi_dojo_uniffi_checksum_method_toriiclient_subscribe_entity_updates: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_dojo_uniffi_checksum_method_toriiclient_subscribe_event_updates()
		})
		if checksum != 33181 {
			// If this happens try cleaning and rebuilding your project
			panic("dojo: uniffi_dojo_uniffi_checksum_method_toriiclient_subscribe_event_updates: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_dojo_uniffi_checksum_method_toriiclient_subscribe_token_balance_updates()
		})
		if checksum != 48432 {
			// If this happens try cleaning and rebuilding your project
			panic("dojo: uniffi_dojo_uniffi_checksum_method_toriiclient_subscribe_token_balance_updates: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_dojo_uniffi_checksum_method_toriiclient_subscribe_token_updates()
		})
		if checksum != 20688 {
			// If this happens try cleaning and rebuilding your project
			panic("dojo: uniffi_dojo_uniffi_checksum_method_toriiclient_subscribe_token_updates: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_dojo_uniffi_checksum_method_toriiclient_subscribe_transaction_updates()
		})
		if checksum != 56883 {
			// If this happens try cleaning and rebuilding your project
			panic("dojo: uniffi_dojo_uniffi_checksum_method_toriiclient_subscribe_transaction_updates: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_dojo_uniffi_checksum_method_toriiclient_token_balances()
		})
		if checksum != 39956 {
			// If this happens try cleaning and rebuilding your project
			panic("dojo: uniffi_dojo_uniffi_checksum_method_toriiclient_token_balances: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_dojo_uniffi_checksum_method_toriiclient_token_contracts()
		})
		if checksum != 14101 {
			// If this happens try cleaning and rebuilding your project
			panic("dojo: uniffi_dojo_uniffi_checksum_method_toriiclient_token_contracts: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_dojo_uniffi_checksum_method_toriiclient_token_transfers()
		})
		if checksum != 25342 {
			// If this happens try cleaning and rebuilding your project
			panic("dojo: uniffi_dojo_uniffi_checksum_method_toriiclient_token_transfers: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_dojo_uniffi_checksum_method_toriiclient_tokens()
		})
		if checksum != 26560 {
			// If this happens try cleaning and rebuilding your project
			panic("dojo: uniffi_dojo_uniffi_checksum_method_toriiclient_tokens: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_dojo_uniffi_checksum_method_toriiclient_transactions()
		})
		if checksum != 20162 {
			// If this happens try cleaning and rebuilding your project
			panic("dojo: uniffi_dojo_uniffi_checksum_method_toriiclient_transactions: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_dojo_uniffi_checksum_method_toriiclient_worlds()
		})
		if checksum != 40140 {
			// If this happens try cleaning and rebuilding your project
			panic("dojo: uniffi_dojo_uniffi_checksum_method_toriiclient_worlds: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_dojo_uniffi_checksum_constructor_toriiclient_new()
		})
		if checksum != 28928 {
			// If this happens try cleaning and rebuilding your project
			panic("dojo: uniffi_dojo_uniffi_checksum_constructor_toriiclient_new: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_dojo_uniffi_checksum_constructor_toriiclient_new_with_config()
		})
		if checksum != 41841 {
			// If this happens try cleaning and rebuilding your project
			panic("dojo: uniffi_dojo_uniffi_checksum_constructor_toriiclient_new_with_config: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_dojo_uniffi_checksum_method_entityupdatecallback_on_update()
		})
		if checksum != 25229 {
			// If this happens try cleaning and rebuilding your project
			panic("dojo: uniffi_dojo_uniffi_checksum_method_entityupdatecallback_on_update: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_dojo_uniffi_checksum_method_entityupdatecallback_on_error()
		})
		if checksum != 55699 {
			// If this happens try cleaning and rebuilding your project
			panic("dojo: uniffi_dojo_uniffi_checksum_method_entityupdatecallback_on_error: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_dojo_uniffi_checksum_method_eventupdatecallback_on_update()
		})
		if checksum != 3388 {
			// If this happens try cleaning and rebuilding your project
			panic("dojo: uniffi_dojo_uniffi_checksum_method_eventupdatecallback_on_update: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_dojo_uniffi_checksum_method_eventupdatecallback_on_error()
		})
		if checksum != 50382 {
			// If this happens try cleaning and rebuilding your project
			panic("dojo: uniffi_dojo_uniffi_checksum_method_eventupdatecallback_on_error: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_dojo_uniffi_checksum_method_tokenbalanceupdatecallback_on_update()
		})
		if checksum != 64172 {
			// If this happens try cleaning and rebuilding your project
			panic("dojo: uniffi_dojo_uniffi_checksum_method_tokenbalanceupdatecallback_on_update: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_dojo_uniffi_checksum_method_tokenbalanceupdatecallback_on_error()
		})
		if checksum != 1784 {
			// If this happens try cleaning and rebuilding your project
			panic("dojo: uniffi_dojo_uniffi_checksum_method_tokenbalanceupdatecallback_on_error: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_dojo_uniffi_checksum_method_tokenupdatecallback_on_update()
		})
		if checksum != 51753 {
			// If this happens try cleaning and rebuilding your project
			panic("dojo: uniffi_dojo_uniffi_checksum_method_tokenupdatecallback_on_update: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_dojo_uniffi_checksum_method_tokenupdatecallback_on_error()
		})
		if checksum != 35695 {
			// If this happens try cleaning and rebuilding your project
			panic("dojo: uniffi_dojo_uniffi_checksum_method_tokenupdatecallback_on_error: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_dojo_uniffi_checksum_method_transactionupdatecallback_on_update()
		})
		if checksum != 6036 {
			// If this happens try cleaning and rebuilding your project
			panic("dojo: uniffi_dojo_uniffi_checksum_method_transactionupdatecallback_on_update: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_dojo_uniffi_checksum_method_transactionupdatecallback_on_error()
		})
		if checksum != 4248 {
			// If this happens try cleaning and rebuilding your project
			panic("dojo: uniffi_dojo_uniffi_checksum_method_transactionupdatecallback_on_error: UniFFI API checksum mismatch")
		}
	}
}

type FfiConverterUint8 struct{}

var FfiConverterUint8INSTANCE = FfiConverterUint8{}

func (FfiConverterUint8) Lower(value uint8) C.uint8_t {
	return C.uint8_t(value)
}

func (FfiConverterUint8) Write(writer io.Writer, value uint8) {
	writeUint8(writer, value)
}

func (FfiConverterUint8) Lift(value C.uint8_t) uint8 {
	return uint8(value)
}

func (FfiConverterUint8) Read(reader io.Reader) uint8 {
	return readUint8(reader)
}

type FfiDestroyerUint8 struct{}

func (FfiDestroyerUint8) Destroy(_ uint8) {}

type FfiConverterInt8 struct{}

var FfiConverterInt8INSTANCE = FfiConverterInt8{}

func (FfiConverterInt8) Lower(value int8) C.int8_t {
	return C.int8_t(value)
}

func (FfiConverterInt8) Write(writer io.Writer, value int8) {
	writeInt8(writer, value)
}

func (FfiConverterInt8) Lift(value C.int8_t) int8 {
	return int8(value)
}

func (FfiConverterInt8) Read(reader io.Reader) int8 {
	return readInt8(reader)
}

type FfiDestroyerInt8 struct{}

func (FfiDestroyerInt8) Destroy(_ int8) {}

type FfiConverterUint16 struct{}

var FfiConverterUint16INSTANCE = FfiConverterUint16{}

func (FfiConverterUint16) Lower(value uint16) C.uint16_t {
	return C.uint16_t(value)
}

func (FfiConverterUint16) Write(writer io.Writer, value uint16) {
	writeUint16(writer, value)
}

func (FfiConverterUint16) Lift(value C.uint16_t) uint16 {
	return uint16(value)
}

func (FfiConverterUint16) Read(reader io.Reader) uint16 {
	return readUint16(reader)
}

type FfiDestroyerUint16 struct{}

func (FfiDestroyerUint16) Destroy(_ uint16) {}

type FfiConverterInt16 struct{}

var FfiConverterInt16INSTANCE = FfiConverterInt16{}

func (FfiConverterInt16) Lower(value int16) C.int16_t {
	return C.int16_t(value)
}

func (FfiConverterInt16) Write(writer io.Writer, value int16) {
	writeInt16(writer, value)
}

func (FfiConverterInt16) Lift(value C.int16_t) int16 {
	return int16(value)
}

func (FfiConverterInt16) Read(reader io.Reader) int16 {
	return readInt16(reader)
}

type FfiDestroyerInt16 struct{}

func (FfiDestroyerInt16) Destroy(_ int16) {}

type FfiConverterUint32 struct{}

var FfiConverterUint32INSTANCE = FfiConverterUint32{}

func (FfiConverterUint32) Lower(value uint32) C.uint32_t {
	return C.uint32_t(value)
}

func (FfiConverterUint32) Write(writer io.Writer, value uint32) {
	writeUint32(writer, value)
}

func (FfiConverterUint32) Lift(value C.uint32_t) uint32 {
	return uint32(value)
}

func (FfiConverterUint32) Read(reader io.Reader) uint32 {
	return readUint32(reader)
}

type FfiDestroyerUint32 struct{}

func (FfiDestroyerUint32) Destroy(_ uint32) {}

type FfiConverterInt32 struct{}

var FfiConverterInt32INSTANCE = FfiConverterInt32{}

func (FfiConverterInt32) Lower(value int32) C.int32_t {
	return C.int32_t(value)
}

func (FfiConverterInt32) Write(writer io.Writer, value int32) {
	writeInt32(writer, value)
}

func (FfiConverterInt32) Lift(value C.int32_t) int32 {
	return int32(value)
}

func (FfiConverterInt32) Read(reader io.Reader) int32 {
	return readInt32(reader)
}

type FfiDestroyerInt32 struct{}

func (FfiDestroyerInt32) Destroy(_ int32) {}

type FfiConverterUint64 struct{}

var FfiConverterUint64INSTANCE = FfiConverterUint64{}

func (FfiConverterUint64) Lower(value uint64) C.uint64_t {
	return C.uint64_t(value)
}

func (FfiConverterUint64) Write(writer io.Writer, value uint64) {
	writeUint64(writer, value)
}

func (FfiConverterUint64) Lift(value C.uint64_t) uint64 {
	return uint64(value)
}

func (FfiConverterUint64) Read(reader io.Reader) uint64 {
	return readUint64(reader)
}

type FfiDestroyerUint64 struct{}

func (FfiDestroyerUint64) Destroy(_ uint64) {}

type FfiConverterInt64 struct{}

var FfiConverterInt64INSTANCE = FfiConverterInt64{}

func (FfiConverterInt64) Lower(value int64) C.int64_t {
	return C.int64_t(value)
}

func (FfiConverterInt64) Write(writer io.Writer, value int64) {
	writeInt64(writer, value)
}

func (FfiConverterInt64) Lift(value C.int64_t) int64 {
	return int64(value)
}

func (FfiConverterInt64) Read(reader io.Reader) int64 {
	return readInt64(reader)
}

type FfiDestroyerInt64 struct{}

func (FfiDestroyerInt64) Destroy(_ int64) {}

type FfiConverterFloat64 struct{}

var FfiConverterFloat64INSTANCE = FfiConverterFloat64{}

func (FfiConverterFloat64) Lower(value float64) C.double {
	return C.double(value)
}

func (FfiConverterFloat64) Write(writer io.Writer, value float64) {
	writeFloat64(writer, value)
}

func (FfiConverterFloat64) Lift(value C.double) float64 {
	return float64(value)
}

func (FfiConverterFloat64) Read(reader io.Reader) float64 {
	return readFloat64(reader)
}

type FfiDestroyerFloat64 struct{}

func (FfiDestroyerFloat64) Destroy(_ float64) {}

type FfiConverterBool struct{}

var FfiConverterBoolINSTANCE = FfiConverterBool{}

func (FfiConverterBool) Lower(value bool) C.int8_t {
	if value {
		return C.int8_t(1)
	}
	return C.int8_t(0)
}

func (FfiConverterBool) Write(writer io.Writer, value bool) {
	if value {
		writeInt8(writer, 1)
	} else {
		writeInt8(writer, 0)
	}
}

func (FfiConverterBool) Lift(value C.int8_t) bool {
	return value != 0
}

func (FfiConverterBool) Read(reader io.Reader) bool {
	return readInt8(reader) != 0
}

type FfiDestroyerBool struct{}

func (FfiDestroyerBool) Destroy(_ bool) {}

type FfiConverterString struct{}

var FfiConverterStringINSTANCE = FfiConverterString{}

func (FfiConverterString) Lift(rb RustBufferI) string {
	defer rb.Free()
	reader := rb.AsReader()
	b, err := io.ReadAll(reader)
	if err != nil {
		panic(fmt.Errorf("reading reader: %w", err))
	}
	return string(b)
}

func (FfiConverterString) Read(reader io.Reader) string {
	length := readInt32(reader)
	buffer := make([]byte, length)
	read_length, err := reader.Read(buffer)
	if err != nil && err != io.EOF {
		panic(err)
	}
	if read_length != int(length) {
		panic(fmt.Errorf("bad read length when reading string, expected %d, read %d", length, read_length))
	}
	return string(buffer)
}

func (FfiConverterString) Lower(value string) C.RustBuffer {
	return stringToRustBuffer(value)
}

func (FfiConverterString) Write(writer io.Writer, value string) {
	if len(value) > math.MaxInt32 {
		panic("String is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	write_length, err := io.WriteString(writer, value)
	if err != nil {
		panic(err)
	}
	if write_length != len(value) {
		panic(fmt.Errorf("bad write length when writing string, expected %d, written %d", len(value), write_length))
	}
}

type FfiDestroyerString struct{}

func (FfiDestroyerString) Destroy(_ string) {}

// Below is an implementation of synchronization requirements outlined in the link.
// https://github.com/mozilla/uniffi-rs/blob/0dc031132d9493ca812c3af6e7dd60ad2ea95bf0/uniffi_bindgen/src/bindings/kotlin/templates/ObjectRuntime.kt#L31

type FfiObject struct {
	pointer       unsafe.Pointer
	callCounter   atomic.Int64
	cloneFunction func(unsafe.Pointer, *C.RustCallStatus) unsafe.Pointer
	freeFunction  func(unsafe.Pointer, *C.RustCallStatus)
	destroyed     atomic.Bool
}

func newFfiObject(
	pointer unsafe.Pointer,
	cloneFunction func(unsafe.Pointer, *C.RustCallStatus) unsafe.Pointer,
	freeFunction func(unsafe.Pointer, *C.RustCallStatus),
) FfiObject {
	return FfiObject{
		pointer:       pointer,
		cloneFunction: cloneFunction,
		freeFunction:  freeFunction,
	}
}

func (ffiObject *FfiObject) incrementPointer(debugName string) unsafe.Pointer {
	for {
		counter := ffiObject.callCounter.Load()
		if counter <= -1 {
			panic(fmt.Errorf("%v object has already been destroyed", debugName))
		}
		if counter == math.MaxInt64 {
			panic(fmt.Errorf("%v object call counter would overflow", debugName))
		}
		if ffiObject.callCounter.CompareAndSwap(counter, counter+1) {
			break
		}
	}

	return rustCall(func(status *C.RustCallStatus) unsafe.Pointer {
		return ffiObject.cloneFunction(ffiObject.pointer, status)
	})
}

func (ffiObject *FfiObject) decrementPointer() {
	if ffiObject.callCounter.Add(-1) == -1 {
		ffiObject.freeRustArcPtr()
	}
}

func (ffiObject *FfiObject) destroy() {
	if ffiObject.destroyed.CompareAndSwap(false, true) {
		if ffiObject.callCounter.Add(-1) == -1 {
			ffiObject.freeRustArcPtr()
		}
	}
}

func (ffiObject *FfiObject) freeRustArcPtr() {
	rustCall(func(status *C.RustCallStatus) int32 {
		ffiObject.freeFunction(ffiObject.pointer, status)
		return 0
	})
}

type ToriiClientInterface interface {
	Achievements(query AchievementQuery) (PageAchievement, error)
	Activities(query ActivityQuery) (PageActivity, error)
	Aggregations(query AggregationQuery) (PageAggregationEntry, error)
	CancelSubscription(subscriptionId uint64) error
	Contracts(query ContractQuery) ([]Contract, error)
	Controllers(query ControllerQuery) (PageController, error)
	Entities(query Query) (PageEntity, error)
	EventMessages(query Query) (PageEntity, error)
	PlayerAchievements(query PlayerAchievementQuery) (PagePlayerAchievement, error)
	PublishMessage(message Message) (string, error)
	PublishMessageBatch(messages []Message) ([]string, error)
	Sql(query string) ([]SqlRow, error)
	StarknetEvents(query EventQuery) (PageEvent, error)
	SubscribeEntityUpdates(clause *Clause, worldAddresses []FieldElement, callback EntityUpdateCallback) (uint64, error)
	SubscribeEventUpdates(keys []KeysClause, callback EventUpdateCallback) (uint64, error)
	SubscribeTokenBalanceUpdates(contractAddresses []FieldElement, accountAddresses []FieldElement, tokenIds []U256, callback TokenBalanceUpdateCallback) (uint64, error)
	SubscribeTokenUpdates(contractAddresses []FieldElement, tokenIds []U256, callback TokenUpdateCallback) (uint64, error)
	SubscribeTransactionUpdates(filter *TransactionFilter, callback TransactionUpdateCallback) (uint64, error)
	TokenBalances(query TokenBalanceQuery) (PageTokenBalance, error)
	TokenContracts(query TokenContractQuery) (PageTokenContract, error)
	TokenTransfers(query TokenTransferQuery) (PageTokenTransfer, error)
	Tokens(query TokenQuery) (PageToken, error)
	Transactions(query TransactionQuery) (PageTransaction, error)
	Worlds(worldAddresses []FieldElement) ([]World, error)
}
type ToriiClient struct {
	ffiObject FfiObject
}

func NewToriiClient(toriiUrl string) (*ToriiClient, error) {
	_uniffiRV, _uniffiErr := rustCallWithError[DojoError](FfiConverterDojoError{}, func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_dojo_uniffi_fn_constructor_toriiclient_new(FfiConverterStringINSTANCE.Lower(toriiUrl), _uniffiStatus)
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue *ToriiClient
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterToriiClientINSTANCE.Lift(_uniffiRV), nil
	}
}

func ToriiClientNewWithConfig(toriiUrl string, maxMessageSize uint64) (*ToriiClient, error) {
	_uniffiRV, _uniffiErr := rustCallWithError[DojoError](FfiConverterDojoError{}, func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_dojo_uniffi_fn_constructor_toriiclient_new_with_config(FfiConverterStringINSTANCE.Lower(toriiUrl), FfiConverterUint64INSTANCE.Lower(maxMessageSize), _uniffiStatus)
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue *ToriiClient
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterToriiClientINSTANCE.Lift(_uniffiRV), nil
	}
}

func (_self *ToriiClient) Achievements(query AchievementQuery) (PageAchievement, error) {
	_pointer := _self.ffiObject.incrementPointer("*ToriiClient")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError[DojoError](FfiConverterDojoError{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer{
			inner: C.uniffi_dojo_uniffi_fn_method_toriiclient_achievements(
				_pointer, FfiConverterAchievementQueryINSTANCE.Lower(query), _uniffiStatus),
		}
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue PageAchievement
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterPageAchievementINSTANCE.Lift(_uniffiRV), nil
	}
}

func (_self *ToriiClient) Activities(query ActivityQuery) (PageActivity, error) {
	_pointer := _self.ffiObject.incrementPointer("*ToriiClient")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError[DojoError](FfiConverterDojoError{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer{
			inner: C.uniffi_dojo_uniffi_fn_method_toriiclient_activities(
				_pointer, FfiConverterActivityQueryINSTANCE.Lower(query), _uniffiStatus),
		}
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue PageActivity
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterPageActivityINSTANCE.Lift(_uniffiRV), nil
	}
}

func (_self *ToriiClient) Aggregations(query AggregationQuery) (PageAggregationEntry, error) {
	_pointer := _self.ffiObject.incrementPointer("*ToriiClient")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError[DojoError](FfiConverterDojoError{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer{
			inner: C.uniffi_dojo_uniffi_fn_method_toriiclient_aggregations(
				_pointer, FfiConverterAggregationQueryINSTANCE.Lower(query), _uniffiStatus),
		}
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue PageAggregationEntry
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterPageAggregationEntryINSTANCE.Lift(_uniffiRV), nil
	}
}

func (_self *ToriiClient) CancelSubscription(subscriptionId uint64) error {
	_pointer := _self.ffiObject.incrementPointer("*ToriiClient")
	defer _self.ffiObject.decrementPointer()
	_, _uniffiErr := rustCallWithError[DojoError](FfiConverterDojoError{}, func(_uniffiStatus *C.RustCallStatus) bool {
		C.uniffi_dojo_uniffi_fn_method_toriiclient_cancel_subscription(
			_pointer, FfiConverterUint64INSTANCE.Lower(subscriptionId), _uniffiStatus)
		return false
	})
	return _uniffiErr.AsError()
}

func (_self *ToriiClient) Contracts(query ContractQuery) ([]Contract, error) {
	_pointer := _self.ffiObject.incrementPointer("*ToriiClient")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError[DojoError](FfiConverterDojoError{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer{
			inner: C.uniffi_dojo_uniffi_fn_method_toriiclient_contracts(
				_pointer, FfiConverterContractQueryINSTANCE.Lower(query), _uniffiStatus),
		}
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue []Contract
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterSequenceContractINSTANCE.Lift(_uniffiRV), nil
	}
}

func (_self *ToriiClient) Controllers(query ControllerQuery) (PageController, error) {
	_pointer := _self.ffiObject.incrementPointer("*ToriiClient")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError[DojoError](FfiConverterDojoError{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer{
			inner: C.uniffi_dojo_uniffi_fn_method_toriiclient_controllers(
				_pointer, FfiConverterControllerQueryINSTANCE.Lower(query), _uniffiStatus),
		}
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue PageController
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterPageControllerINSTANCE.Lift(_uniffiRV), nil
	}
}

func (_self *ToriiClient) Entities(query Query) (PageEntity, error) {
	_pointer := _self.ffiObject.incrementPointer("*ToriiClient")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError[DojoError](FfiConverterDojoError{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer{
			inner: C.uniffi_dojo_uniffi_fn_method_toriiclient_entities(
				_pointer, FfiConverterQueryINSTANCE.Lower(query), _uniffiStatus),
		}
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue PageEntity
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterPageEntityINSTANCE.Lift(_uniffiRV), nil
	}
}

func (_self *ToriiClient) EventMessages(query Query) (PageEntity, error) {
	_pointer := _self.ffiObject.incrementPointer("*ToriiClient")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError[DojoError](FfiConverterDojoError{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer{
			inner: C.uniffi_dojo_uniffi_fn_method_toriiclient_event_messages(
				_pointer, FfiConverterQueryINSTANCE.Lower(query), _uniffiStatus),
		}
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue PageEntity
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterPageEntityINSTANCE.Lift(_uniffiRV), nil
	}
}

func (_self *ToriiClient) PlayerAchievements(query PlayerAchievementQuery) (PagePlayerAchievement, error) {
	_pointer := _self.ffiObject.incrementPointer("*ToriiClient")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError[DojoError](FfiConverterDojoError{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer{
			inner: C.uniffi_dojo_uniffi_fn_method_toriiclient_player_achievements(
				_pointer, FfiConverterPlayerAchievementQueryINSTANCE.Lower(query), _uniffiStatus),
		}
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue PagePlayerAchievement
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterPagePlayerAchievementINSTANCE.Lift(_uniffiRV), nil
	}
}

func (_self *ToriiClient) PublishMessage(message Message) (string, error) {
	_pointer := _self.ffiObject.incrementPointer("*ToriiClient")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError[DojoError](FfiConverterDojoError{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer{
			inner: C.uniffi_dojo_uniffi_fn_method_toriiclient_publish_message(
				_pointer, FfiConverterMessageINSTANCE.Lower(message), _uniffiStatus),
		}
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue string
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterStringINSTANCE.Lift(_uniffiRV), nil
	}
}

func (_self *ToriiClient) PublishMessageBatch(messages []Message) ([]string, error) {
	_pointer := _self.ffiObject.incrementPointer("*ToriiClient")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError[DojoError](FfiConverterDojoError{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer{
			inner: C.uniffi_dojo_uniffi_fn_method_toriiclient_publish_message_batch(
				_pointer, FfiConverterSequenceMessageINSTANCE.Lower(messages), _uniffiStatus),
		}
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue []string
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterSequenceStringINSTANCE.Lift(_uniffiRV), nil
	}
}

func (_self *ToriiClient) Sql(query string) ([]SqlRow, error) {
	_pointer := _self.ffiObject.incrementPointer("*ToriiClient")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError[DojoError](FfiConverterDojoError{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer{
			inner: C.uniffi_dojo_uniffi_fn_method_toriiclient_sql(
				_pointer, FfiConverterStringINSTANCE.Lower(query), _uniffiStatus),
		}
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue []SqlRow
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterSequenceSqlRowINSTANCE.Lift(_uniffiRV), nil
	}
}

func (_self *ToriiClient) StarknetEvents(query EventQuery) (PageEvent, error) {
	_pointer := _self.ffiObject.incrementPointer("*ToriiClient")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError[DojoError](FfiConverterDojoError{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer{
			inner: C.uniffi_dojo_uniffi_fn_method_toriiclient_starknet_events(
				_pointer, FfiConverterEventQueryINSTANCE.Lower(query), _uniffiStatus),
		}
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue PageEvent
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterPageEventINSTANCE.Lift(_uniffiRV), nil
	}
}

func (_self *ToriiClient) SubscribeEntityUpdates(clause *Clause, worldAddresses []FieldElement, callback EntityUpdateCallback) (uint64, error) {
	_pointer := _self.ffiObject.incrementPointer("*ToriiClient")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError[DojoError](FfiConverterDojoError{}, func(_uniffiStatus *C.RustCallStatus) C.uint64_t {
		return C.uniffi_dojo_uniffi_fn_method_toriiclient_subscribe_entity_updates(
			_pointer, FfiConverterOptionalClauseINSTANCE.Lower(clause), FfiConverterSequenceTypeFieldElementINSTANCE.Lower(worldAddresses), FfiConverterCallbackInterfaceEntityUpdateCallbackINSTANCE.Lower(callback), _uniffiStatus)
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue uint64
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterUint64INSTANCE.Lift(_uniffiRV), nil
	}
}

func (_self *ToriiClient) SubscribeEventUpdates(keys []KeysClause, callback EventUpdateCallback) (uint64, error) {
	_pointer := _self.ffiObject.incrementPointer("*ToriiClient")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError[DojoError](FfiConverterDojoError{}, func(_uniffiStatus *C.RustCallStatus) C.uint64_t {
		return C.uniffi_dojo_uniffi_fn_method_toriiclient_subscribe_event_updates(
			_pointer, FfiConverterSequenceKeysClauseINSTANCE.Lower(keys), FfiConverterCallbackInterfaceEventUpdateCallbackINSTANCE.Lower(callback), _uniffiStatus)
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue uint64
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterUint64INSTANCE.Lift(_uniffiRV), nil
	}
}

func (_self *ToriiClient) SubscribeTokenBalanceUpdates(contractAddresses []FieldElement, accountAddresses []FieldElement, tokenIds []U256, callback TokenBalanceUpdateCallback) (uint64, error) {
	_pointer := _self.ffiObject.incrementPointer("*ToriiClient")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError[DojoError](FfiConverterDojoError{}, func(_uniffiStatus *C.RustCallStatus) C.uint64_t {
		return C.uniffi_dojo_uniffi_fn_method_toriiclient_subscribe_token_balance_updates(
			_pointer, FfiConverterSequenceTypeFieldElementINSTANCE.Lower(contractAddresses), FfiConverterSequenceTypeFieldElementINSTANCE.Lower(accountAddresses), FfiConverterSequenceTypeU256INSTANCE.Lower(tokenIds), FfiConverterCallbackInterfaceTokenBalanceUpdateCallbackINSTANCE.Lower(callback), _uniffiStatus)
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue uint64
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterUint64INSTANCE.Lift(_uniffiRV), nil
	}
}

func (_self *ToriiClient) SubscribeTokenUpdates(contractAddresses []FieldElement, tokenIds []U256, callback TokenUpdateCallback) (uint64, error) {
	_pointer := _self.ffiObject.incrementPointer("*ToriiClient")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError[DojoError](FfiConverterDojoError{}, func(_uniffiStatus *C.RustCallStatus) C.uint64_t {
		return C.uniffi_dojo_uniffi_fn_method_toriiclient_subscribe_token_updates(
			_pointer, FfiConverterSequenceTypeFieldElementINSTANCE.Lower(contractAddresses), FfiConverterSequenceTypeU256INSTANCE.Lower(tokenIds), FfiConverterCallbackInterfaceTokenUpdateCallbackINSTANCE.Lower(callback), _uniffiStatus)
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue uint64
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterUint64INSTANCE.Lift(_uniffiRV), nil
	}
}

func (_self *ToriiClient) SubscribeTransactionUpdates(filter *TransactionFilter, callback TransactionUpdateCallback) (uint64, error) {
	_pointer := _self.ffiObject.incrementPointer("*ToriiClient")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError[DojoError](FfiConverterDojoError{}, func(_uniffiStatus *C.RustCallStatus) C.uint64_t {
		return C.uniffi_dojo_uniffi_fn_method_toriiclient_subscribe_transaction_updates(
			_pointer, FfiConverterOptionalTransactionFilterINSTANCE.Lower(filter), FfiConverterCallbackInterfaceTransactionUpdateCallbackINSTANCE.Lower(callback), _uniffiStatus)
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue uint64
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterUint64INSTANCE.Lift(_uniffiRV), nil
	}
}

func (_self *ToriiClient) TokenBalances(query TokenBalanceQuery) (PageTokenBalance, error) {
	_pointer := _self.ffiObject.incrementPointer("*ToriiClient")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError[DojoError](FfiConverterDojoError{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer{
			inner: C.uniffi_dojo_uniffi_fn_method_toriiclient_token_balances(
				_pointer, FfiConverterTokenBalanceQueryINSTANCE.Lower(query), _uniffiStatus),
		}
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue PageTokenBalance
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterPageTokenBalanceINSTANCE.Lift(_uniffiRV), nil
	}
}

func (_self *ToriiClient) TokenContracts(query TokenContractQuery) (PageTokenContract, error) {
	_pointer := _self.ffiObject.incrementPointer("*ToriiClient")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError[DojoError](FfiConverterDojoError{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer{
			inner: C.uniffi_dojo_uniffi_fn_method_toriiclient_token_contracts(
				_pointer, FfiConverterTokenContractQueryINSTANCE.Lower(query), _uniffiStatus),
		}
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue PageTokenContract
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterPageTokenContractINSTANCE.Lift(_uniffiRV), nil
	}
}

func (_self *ToriiClient) TokenTransfers(query TokenTransferQuery) (PageTokenTransfer, error) {
	_pointer := _self.ffiObject.incrementPointer("*ToriiClient")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError[DojoError](FfiConverterDojoError{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer{
			inner: C.uniffi_dojo_uniffi_fn_method_toriiclient_token_transfers(
				_pointer, FfiConverterTokenTransferQueryINSTANCE.Lower(query), _uniffiStatus),
		}
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue PageTokenTransfer
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterPageTokenTransferINSTANCE.Lift(_uniffiRV), nil
	}
}

func (_self *ToriiClient) Tokens(query TokenQuery) (PageToken, error) {
	_pointer := _self.ffiObject.incrementPointer("*ToriiClient")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError[DojoError](FfiConverterDojoError{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer{
			inner: C.uniffi_dojo_uniffi_fn_method_toriiclient_tokens(
				_pointer, FfiConverterTokenQueryINSTANCE.Lower(query), _uniffiStatus),
		}
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue PageToken
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterPageTokenINSTANCE.Lift(_uniffiRV), nil
	}
}

func (_self *ToriiClient) Transactions(query TransactionQuery) (PageTransaction, error) {
	_pointer := _self.ffiObject.incrementPointer("*ToriiClient")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError[DojoError](FfiConverterDojoError{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer{
			inner: C.uniffi_dojo_uniffi_fn_method_toriiclient_transactions(
				_pointer, FfiConverterTransactionQueryINSTANCE.Lower(query), _uniffiStatus),
		}
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue PageTransaction
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterPageTransactionINSTANCE.Lift(_uniffiRV), nil
	}
}

func (_self *ToriiClient) Worlds(worldAddresses []FieldElement) ([]World, error) {
	_pointer := _self.ffiObject.incrementPointer("*ToriiClient")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError[DojoError](FfiConverterDojoError{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer{
			inner: C.uniffi_dojo_uniffi_fn_method_toriiclient_worlds(
				_pointer, FfiConverterSequenceTypeFieldElementINSTANCE.Lower(worldAddresses), _uniffiStatus),
		}
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue []World
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterSequenceWorldINSTANCE.Lift(_uniffiRV), nil
	}
}
func (object *ToriiClient) Destroy() {
	runtime.SetFinalizer(object, nil)
	object.ffiObject.destroy()
}

type FfiConverterToriiClient struct{}

var FfiConverterToriiClientINSTANCE = FfiConverterToriiClient{}

func (c FfiConverterToriiClient) Lift(pointer unsafe.Pointer) *ToriiClient {
	result := &ToriiClient{
		newFfiObject(
			pointer,
			func(pointer unsafe.Pointer, status *C.RustCallStatus) unsafe.Pointer {
				return C.uniffi_dojo_uniffi_fn_clone_toriiclient(pointer, status)
			},
			func(pointer unsafe.Pointer, status *C.RustCallStatus) {
				C.uniffi_dojo_uniffi_fn_free_toriiclient(pointer, status)
			},
		),
	}
	runtime.SetFinalizer(result, (*ToriiClient).Destroy)
	return result
}

func (c FfiConverterToriiClient) Read(reader io.Reader) *ToriiClient {
	return c.Lift(unsafe.Pointer(uintptr(readUint64(reader))))
}

func (c FfiConverterToriiClient) Lower(value *ToriiClient) unsafe.Pointer {
	// TODO: this is bad - all synchronization from ObjectRuntime.go is discarded here,
	// because the pointer will be decremented immediately after this function returns,
	// and someone will be left holding onto a non-locked pointer.
	pointer := value.ffiObject.incrementPointer("*ToriiClient")
	defer value.ffiObject.decrementPointer()
	return pointer

}

func (c FfiConverterToriiClient) Write(writer io.Writer, value *ToriiClient) {
	writeUint64(writer, uint64(uintptr(c.Lower(value))))
}

type FfiDestroyerToriiClient struct{}

func (_ FfiDestroyerToriiClient) Destroy(value *ToriiClient) {
	value.Destroy()
}

type Achievement struct {
	Id               string
	WorldAddress     FieldElement
	Namespace        string
	EntityId         string
	Hidden           bool
	Index            uint32
	Points           uint32
	Start            string
	End              string
	Group            string
	Icon             string
	Title            string
	Description      string
	Tasks            []AchievementTask
	Data             *string
	TotalCompletions uint32
	CompletionRate   float64
	CreatedAt        uint64
	UpdatedAt        uint64
}

func (r *Achievement) Destroy() {
	FfiDestroyerString{}.Destroy(r.Id)
	FfiDestroyerTypeFieldElement{}.Destroy(r.WorldAddress)
	FfiDestroyerString{}.Destroy(r.Namespace)
	FfiDestroyerString{}.Destroy(r.EntityId)
	FfiDestroyerBool{}.Destroy(r.Hidden)
	FfiDestroyerUint32{}.Destroy(r.Index)
	FfiDestroyerUint32{}.Destroy(r.Points)
	FfiDestroyerString{}.Destroy(r.Start)
	FfiDestroyerString{}.Destroy(r.End)
	FfiDestroyerString{}.Destroy(r.Group)
	FfiDestroyerString{}.Destroy(r.Icon)
	FfiDestroyerString{}.Destroy(r.Title)
	FfiDestroyerString{}.Destroy(r.Description)
	FfiDestroyerSequenceAchievementTask{}.Destroy(r.Tasks)
	FfiDestroyerOptionalString{}.Destroy(r.Data)
	FfiDestroyerUint32{}.Destroy(r.TotalCompletions)
	FfiDestroyerFloat64{}.Destroy(r.CompletionRate)
	FfiDestroyerUint64{}.Destroy(r.CreatedAt)
	FfiDestroyerUint64{}.Destroy(r.UpdatedAt)
}

type FfiConverterAchievement struct{}

var FfiConverterAchievementINSTANCE = FfiConverterAchievement{}

func (c FfiConverterAchievement) Lift(rb RustBufferI) Achievement {
	return LiftFromRustBuffer[Achievement](c, rb)
}

func (c FfiConverterAchievement) Read(reader io.Reader) Achievement {
	return Achievement{
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterTypeFieldElementINSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterBoolINSTANCE.Read(reader),
		FfiConverterUint32INSTANCE.Read(reader),
		FfiConverterUint32INSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterSequenceAchievementTaskINSTANCE.Read(reader),
		FfiConverterOptionalStringINSTANCE.Read(reader),
		FfiConverterUint32INSTANCE.Read(reader),
		FfiConverterFloat64INSTANCE.Read(reader),
		FfiConverterUint64INSTANCE.Read(reader),
		FfiConverterUint64INSTANCE.Read(reader),
	}
}

func (c FfiConverterAchievement) Lower(value Achievement) C.RustBuffer {
	return LowerIntoRustBuffer[Achievement](c, value)
}

func (c FfiConverterAchievement) Write(writer io.Writer, value Achievement) {
	FfiConverterStringINSTANCE.Write(writer, value.Id)
	FfiConverterTypeFieldElementINSTANCE.Write(writer, value.WorldAddress)
	FfiConverterStringINSTANCE.Write(writer, value.Namespace)
	FfiConverterStringINSTANCE.Write(writer, value.EntityId)
	FfiConverterBoolINSTANCE.Write(writer, value.Hidden)
	FfiConverterUint32INSTANCE.Write(writer, value.Index)
	FfiConverterUint32INSTANCE.Write(writer, value.Points)
	FfiConverterStringINSTANCE.Write(writer, value.Start)
	FfiConverterStringINSTANCE.Write(writer, value.End)
	FfiConverterStringINSTANCE.Write(writer, value.Group)
	FfiConverterStringINSTANCE.Write(writer, value.Icon)
	FfiConverterStringINSTANCE.Write(writer, value.Title)
	FfiConverterStringINSTANCE.Write(writer, value.Description)
	FfiConverterSequenceAchievementTaskINSTANCE.Write(writer, value.Tasks)
	FfiConverterOptionalStringINSTANCE.Write(writer, value.Data)
	FfiConverterUint32INSTANCE.Write(writer, value.TotalCompletions)
	FfiConverterFloat64INSTANCE.Write(writer, value.CompletionRate)
	FfiConverterUint64INSTANCE.Write(writer, value.CreatedAt)
	FfiConverterUint64INSTANCE.Write(writer, value.UpdatedAt)
}

type FfiDestroyerAchievement struct{}

func (_ FfiDestroyerAchievement) Destroy(value Achievement) {
	value.Destroy()
}

type AchievementProgression struct {
	Id            string
	AchievementId string
	TaskId        string
	WorldAddress  FieldElement
	Namespace     string
	PlayerId      FieldElement
	Count         uint32
	Completed     bool
	CompletedAt   *uint64
	CreatedAt     uint64
	UpdatedAt     uint64
}

func (r *AchievementProgression) Destroy() {
	FfiDestroyerString{}.Destroy(r.Id)
	FfiDestroyerString{}.Destroy(r.AchievementId)
	FfiDestroyerString{}.Destroy(r.TaskId)
	FfiDestroyerTypeFieldElement{}.Destroy(r.WorldAddress)
	FfiDestroyerString{}.Destroy(r.Namespace)
	FfiDestroyerTypeFieldElement{}.Destroy(r.PlayerId)
	FfiDestroyerUint32{}.Destroy(r.Count)
	FfiDestroyerBool{}.Destroy(r.Completed)
	FfiDestroyerOptionalUint64{}.Destroy(r.CompletedAt)
	FfiDestroyerUint64{}.Destroy(r.CreatedAt)
	FfiDestroyerUint64{}.Destroy(r.UpdatedAt)
}

type FfiConverterAchievementProgression struct{}

var FfiConverterAchievementProgressionINSTANCE = FfiConverterAchievementProgression{}

func (c FfiConverterAchievementProgression) Lift(rb RustBufferI) AchievementProgression {
	return LiftFromRustBuffer[AchievementProgression](c, rb)
}

func (c FfiConverterAchievementProgression) Read(reader io.Reader) AchievementProgression {
	return AchievementProgression{
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterTypeFieldElementINSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterTypeFieldElementINSTANCE.Read(reader),
		FfiConverterUint32INSTANCE.Read(reader),
		FfiConverterBoolINSTANCE.Read(reader),
		FfiConverterOptionalUint64INSTANCE.Read(reader),
		FfiConverterUint64INSTANCE.Read(reader),
		FfiConverterUint64INSTANCE.Read(reader),
	}
}

func (c FfiConverterAchievementProgression) Lower(value AchievementProgression) C.RustBuffer {
	return LowerIntoRustBuffer[AchievementProgression](c, value)
}

func (c FfiConverterAchievementProgression) Write(writer io.Writer, value AchievementProgression) {
	FfiConverterStringINSTANCE.Write(writer, value.Id)
	FfiConverterStringINSTANCE.Write(writer, value.AchievementId)
	FfiConverterStringINSTANCE.Write(writer, value.TaskId)
	FfiConverterTypeFieldElementINSTANCE.Write(writer, value.WorldAddress)
	FfiConverterStringINSTANCE.Write(writer, value.Namespace)
	FfiConverterTypeFieldElementINSTANCE.Write(writer, value.PlayerId)
	FfiConverterUint32INSTANCE.Write(writer, value.Count)
	FfiConverterBoolINSTANCE.Write(writer, value.Completed)
	FfiConverterOptionalUint64INSTANCE.Write(writer, value.CompletedAt)
	FfiConverterUint64INSTANCE.Write(writer, value.CreatedAt)
	FfiConverterUint64INSTANCE.Write(writer, value.UpdatedAt)
}

type FfiDestroyerAchievementProgression struct{}

func (_ FfiDestroyerAchievementProgression) Destroy(value AchievementProgression) {
	value.Destroy()
}

type AchievementQuery struct {
	WorldAddresses []FieldElement
	Namespaces     []string
	Hidden         *bool
	Pagination     Pagination
}

func (r *AchievementQuery) Destroy() {
	FfiDestroyerSequenceTypeFieldElement{}.Destroy(r.WorldAddresses)
	FfiDestroyerSequenceString{}.Destroy(r.Namespaces)
	FfiDestroyerOptionalBool{}.Destroy(r.Hidden)
	FfiDestroyerPagination{}.Destroy(r.Pagination)
}

type FfiConverterAchievementQuery struct{}

var FfiConverterAchievementQueryINSTANCE = FfiConverterAchievementQuery{}

func (c FfiConverterAchievementQuery) Lift(rb RustBufferI) AchievementQuery {
	return LiftFromRustBuffer[AchievementQuery](c, rb)
}

func (c FfiConverterAchievementQuery) Read(reader io.Reader) AchievementQuery {
	return AchievementQuery{
		FfiConverterSequenceTypeFieldElementINSTANCE.Read(reader),
		FfiConverterSequenceStringINSTANCE.Read(reader),
		FfiConverterOptionalBoolINSTANCE.Read(reader),
		FfiConverterPaginationINSTANCE.Read(reader),
	}
}

func (c FfiConverterAchievementQuery) Lower(value AchievementQuery) C.RustBuffer {
	return LowerIntoRustBuffer[AchievementQuery](c, value)
}

func (c FfiConverterAchievementQuery) Write(writer io.Writer, value AchievementQuery) {
	FfiConverterSequenceTypeFieldElementINSTANCE.Write(writer, value.WorldAddresses)
	FfiConverterSequenceStringINSTANCE.Write(writer, value.Namespaces)
	FfiConverterOptionalBoolINSTANCE.Write(writer, value.Hidden)
	FfiConverterPaginationINSTANCE.Write(writer, value.Pagination)
}

type FfiDestroyerAchievementQuery struct{}

func (_ FfiDestroyerAchievementQuery) Destroy(value AchievementQuery) {
	value.Destroy()
}

type AchievementTask struct {
	TaskId           string
	Description      string
	Total            uint32
	TotalCompletions uint32
	CompletionRate   float64
	CreatedAt        uint64
}

func (r *AchievementTask) Destroy() {
	FfiDestroyerString{}.Destroy(r.TaskId)
	FfiDestroyerString{}.Destroy(r.Description)
	FfiDestroyerUint32{}.Destroy(r.Total)
	FfiDestroyerUint32{}.Destroy(r.TotalCompletions)
	FfiDestroyerFloat64{}.Destroy(r.CompletionRate)
	FfiDestroyerUint64{}.Destroy(r.CreatedAt)
}

type FfiConverterAchievementTask struct{}

var FfiConverterAchievementTaskINSTANCE = FfiConverterAchievementTask{}

func (c FfiConverterAchievementTask) Lift(rb RustBufferI) AchievementTask {
	return LiftFromRustBuffer[AchievementTask](c, rb)
}

func (c FfiConverterAchievementTask) Read(reader io.Reader) AchievementTask {
	return AchievementTask{
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterUint32INSTANCE.Read(reader),
		FfiConverterUint32INSTANCE.Read(reader),
		FfiConverterFloat64INSTANCE.Read(reader),
		FfiConverterUint64INSTANCE.Read(reader),
	}
}

func (c FfiConverterAchievementTask) Lower(value AchievementTask) C.RustBuffer {
	return LowerIntoRustBuffer[AchievementTask](c, value)
}

func (c FfiConverterAchievementTask) Write(writer io.Writer, value AchievementTask) {
	FfiConverterStringINSTANCE.Write(writer, value.TaskId)
	FfiConverterStringINSTANCE.Write(writer, value.Description)
	FfiConverterUint32INSTANCE.Write(writer, value.Total)
	FfiConverterUint32INSTANCE.Write(writer, value.TotalCompletions)
	FfiConverterFloat64INSTANCE.Write(writer, value.CompletionRate)
	FfiConverterUint64INSTANCE.Write(writer, value.CreatedAt)
}

type FfiDestroyerAchievementTask struct{}

func (_ FfiDestroyerAchievementTask) Destroy(value AchievementTask) {
	value.Destroy()
}

type ActionCount struct {
	ActionName string
	Count      uint32
}

func (r *ActionCount) Destroy() {
	FfiDestroyerString{}.Destroy(r.ActionName)
	FfiDestroyerUint32{}.Destroy(r.Count)
}

type FfiConverterActionCount struct{}

var FfiConverterActionCountINSTANCE = FfiConverterActionCount{}

func (c FfiConverterActionCount) Lift(rb RustBufferI) ActionCount {
	return LiftFromRustBuffer[ActionCount](c, rb)
}

func (c FfiConverterActionCount) Read(reader io.Reader) ActionCount {
	return ActionCount{
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterUint32INSTANCE.Read(reader),
	}
}

func (c FfiConverterActionCount) Lower(value ActionCount) C.RustBuffer {
	return LowerIntoRustBuffer[ActionCount](c, value)
}

func (c FfiConverterActionCount) Write(writer io.Writer, value ActionCount) {
	FfiConverterStringINSTANCE.Write(writer, value.ActionName)
	FfiConverterUint32INSTANCE.Write(writer, value.Count)
}

type FfiDestroyerActionCount struct{}

func (_ FfiDestroyerActionCount) Destroy(value ActionCount) {
	value.Destroy()
}

type Activity struct {
	Id            string
	WorldAddress  FieldElement
	Namespace     string
	CallerAddress FieldElement
	SessionStart  uint64
	SessionEnd    uint64
	ActionCount   uint32
	Actions       []ActionCount
	UpdatedAt     uint64
}

func (r *Activity) Destroy() {
	FfiDestroyerString{}.Destroy(r.Id)
	FfiDestroyerTypeFieldElement{}.Destroy(r.WorldAddress)
	FfiDestroyerString{}.Destroy(r.Namespace)
	FfiDestroyerTypeFieldElement{}.Destroy(r.CallerAddress)
	FfiDestroyerUint64{}.Destroy(r.SessionStart)
	FfiDestroyerUint64{}.Destroy(r.SessionEnd)
	FfiDestroyerUint32{}.Destroy(r.ActionCount)
	FfiDestroyerSequenceActionCount{}.Destroy(r.Actions)
	FfiDestroyerUint64{}.Destroy(r.UpdatedAt)
}

type FfiConverterActivity struct{}

var FfiConverterActivityINSTANCE = FfiConverterActivity{}

func (c FfiConverterActivity) Lift(rb RustBufferI) Activity {
	return LiftFromRustBuffer[Activity](c, rb)
}

func (c FfiConverterActivity) Read(reader io.Reader) Activity {
	return Activity{
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterTypeFieldElementINSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterTypeFieldElementINSTANCE.Read(reader),
		FfiConverterUint64INSTANCE.Read(reader),
		FfiConverterUint64INSTANCE.Read(reader),
		FfiConverterUint32INSTANCE.Read(reader),
		FfiConverterSequenceActionCountINSTANCE.Read(reader),
		FfiConverterUint64INSTANCE.Read(reader),
	}
}

func (c FfiConverterActivity) Lower(value Activity) C.RustBuffer {
	return LowerIntoRustBuffer[Activity](c, value)
}

func (c FfiConverterActivity) Write(writer io.Writer, value Activity) {
	FfiConverterStringINSTANCE.Write(writer, value.Id)
	FfiConverterTypeFieldElementINSTANCE.Write(writer, value.WorldAddress)
	FfiConverterStringINSTANCE.Write(writer, value.Namespace)
	FfiConverterTypeFieldElementINSTANCE.Write(writer, value.CallerAddress)
	FfiConverterUint64INSTANCE.Write(writer, value.SessionStart)
	FfiConverterUint64INSTANCE.Write(writer, value.SessionEnd)
	FfiConverterUint32INSTANCE.Write(writer, value.ActionCount)
	FfiConverterSequenceActionCountINSTANCE.Write(writer, value.Actions)
	FfiConverterUint64INSTANCE.Write(writer, value.UpdatedAt)
}

type FfiDestroyerActivity struct{}

func (_ FfiDestroyerActivity) Destroy(value Activity) {
	value.Destroy()
}

type ActivityQuery struct {
	WorldAddresses  []FieldElement
	Namespaces      []string
	CallerAddresses []FieldElement
	FromTime        *uint64
	ToTime          *uint64
	Pagination      Pagination
}

func (r *ActivityQuery) Destroy() {
	FfiDestroyerSequenceTypeFieldElement{}.Destroy(r.WorldAddresses)
	FfiDestroyerSequenceString{}.Destroy(r.Namespaces)
	FfiDestroyerSequenceTypeFieldElement{}.Destroy(r.CallerAddresses)
	FfiDestroyerOptionalUint64{}.Destroy(r.FromTime)
	FfiDestroyerOptionalUint64{}.Destroy(r.ToTime)
	FfiDestroyerPagination{}.Destroy(r.Pagination)
}

type FfiConverterActivityQuery struct{}

var FfiConverterActivityQueryINSTANCE = FfiConverterActivityQuery{}

func (c FfiConverterActivityQuery) Lift(rb RustBufferI) ActivityQuery {
	return LiftFromRustBuffer[ActivityQuery](c, rb)
}

func (c FfiConverterActivityQuery) Read(reader io.Reader) ActivityQuery {
	return ActivityQuery{
		FfiConverterSequenceTypeFieldElementINSTANCE.Read(reader),
		FfiConverterSequenceStringINSTANCE.Read(reader),
		FfiConverterSequenceTypeFieldElementINSTANCE.Read(reader),
		FfiConverterOptionalUint64INSTANCE.Read(reader),
		FfiConverterOptionalUint64INSTANCE.Read(reader),
		FfiConverterPaginationINSTANCE.Read(reader),
	}
}

func (c FfiConverterActivityQuery) Lower(value ActivityQuery) C.RustBuffer {
	return LowerIntoRustBuffer[ActivityQuery](c, value)
}

func (c FfiConverterActivityQuery) Write(writer io.Writer, value ActivityQuery) {
	FfiConverterSequenceTypeFieldElementINSTANCE.Write(writer, value.WorldAddresses)
	FfiConverterSequenceStringINSTANCE.Write(writer, value.Namespaces)
	FfiConverterSequenceTypeFieldElementINSTANCE.Write(writer, value.CallerAddresses)
	FfiConverterOptionalUint64INSTANCE.Write(writer, value.FromTime)
	FfiConverterOptionalUint64INSTANCE.Write(writer, value.ToTime)
	FfiConverterPaginationINSTANCE.Write(writer, value.Pagination)
}

type FfiDestroyerActivityQuery struct{}

func (_ FfiDestroyerActivityQuery) Destroy(value ActivityQuery) {
	value.Destroy()
}

type AggregationEntry struct {
	Id           string
	AggregatorId string
	EntityId     string
	Value        U256
	DisplayValue string
	Position     uint64
	ModelId      string
	CreatedAt    uint64
	UpdatedAt    uint64
}

func (r *AggregationEntry) Destroy() {
	FfiDestroyerString{}.Destroy(r.Id)
	FfiDestroyerString{}.Destroy(r.AggregatorId)
	FfiDestroyerString{}.Destroy(r.EntityId)
	FfiDestroyerTypeU256{}.Destroy(r.Value)
	FfiDestroyerString{}.Destroy(r.DisplayValue)
	FfiDestroyerUint64{}.Destroy(r.Position)
	FfiDestroyerString{}.Destroy(r.ModelId)
	FfiDestroyerUint64{}.Destroy(r.CreatedAt)
	FfiDestroyerUint64{}.Destroy(r.UpdatedAt)
}

type FfiConverterAggregationEntry struct{}

var FfiConverterAggregationEntryINSTANCE = FfiConverterAggregationEntry{}

func (c FfiConverterAggregationEntry) Lift(rb RustBufferI) AggregationEntry {
	return LiftFromRustBuffer[AggregationEntry](c, rb)
}

func (c FfiConverterAggregationEntry) Read(reader io.Reader) AggregationEntry {
	return AggregationEntry{
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterTypeU256INSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterUint64INSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterUint64INSTANCE.Read(reader),
		FfiConverterUint64INSTANCE.Read(reader),
	}
}

func (c FfiConverterAggregationEntry) Lower(value AggregationEntry) C.RustBuffer {
	return LowerIntoRustBuffer[AggregationEntry](c, value)
}

func (c FfiConverterAggregationEntry) Write(writer io.Writer, value AggregationEntry) {
	FfiConverterStringINSTANCE.Write(writer, value.Id)
	FfiConverterStringINSTANCE.Write(writer, value.AggregatorId)
	FfiConverterStringINSTANCE.Write(writer, value.EntityId)
	FfiConverterTypeU256INSTANCE.Write(writer, value.Value)
	FfiConverterStringINSTANCE.Write(writer, value.DisplayValue)
	FfiConverterUint64INSTANCE.Write(writer, value.Position)
	FfiConverterStringINSTANCE.Write(writer, value.ModelId)
	FfiConverterUint64INSTANCE.Write(writer, value.CreatedAt)
	FfiConverterUint64INSTANCE.Write(writer, value.UpdatedAt)
}

type FfiDestroyerAggregationEntry struct{}

func (_ FfiDestroyerAggregationEntry) Destroy(value AggregationEntry) {
	value.Destroy()
}

type AggregationQuery struct {
	AggregatorIds []string
	EntityIds     []string
	Pagination    Pagination
}

func (r *AggregationQuery) Destroy() {
	FfiDestroyerSequenceString{}.Destroy(r.AggregatorIds)
	FfiDestroyerSequenceString{}.Destroy(r.EntityIds)
	FfiDestroyerPagination{}.Destroy(r.Pagination)
}

type FfiConverterAggregationQuery struct{}

var FfiConverterAggregationQueryINSTANCE = FfiConverterAggregationQuery{}

func (c FfiConverterAggregationQuery) Lift(rb RustBufferI) AggregationQuery {
	return LiftFromRustBuffer[AggregationQuery](c, rb)
}

func (c FfiConverterAggregationQuery) Read(reader io.Reader) AggregationQuery {
	return AggregationQuery{
		FfiConverterSequenceStringINSTANCE.Read(reader),
		FfiConverterSequenceStringINSTANCE.Read(reader),
		FfiConverterPaginationINSTANCE.Read(reader),
	}
}

func (c FfiConverterAggregationQuery) Lower(value AggregationQuery) C.RustBuffer {
	return LowerIntoRustBuffer[AggregationQuery](c, value)
}

func (c FfiConverterAggregationQuery) Write(writer io.Writer, value AggregationQuery) {
	FfiConverterSequenceStringINSTANCE.Write(writer, value.AggregatorIds)
	FfiConverterSequenceStringINSTANCE.Write(writer, value.EntityIds)
	FfiConverterPaginationINSTANCE.Write(writer, value.Pagination)
}

type FfiDestroyerAggregationQuery struct{}

func (_ FfiDestroyerAggregationQuery) Destroy(value AggregationQuery) {
	value.Destroy()
}

type AttributeFilter struct {
	TraitName  string
	TraitValue string
}

func (r *AttributeFilter) Destroy() {
	FfiDestroyerString{}.Destroy(r.TraitName)
	FfiDestroyerString{}.Destroy(r.TraitValue)
}

type FfiConverterAttributeFilter struct{}

var FfiConverterAttributeFilterINSTANCE = FfiConverterAttributeFilter{}

func (c FfiConverterAttributeFilter) Lift(rb RustBufferI) AttributeFilter {
	return LiftFromRustBuffer[AttributeFilter](c, rb)
}

func (c FfiConverterAttributeFilter) Read(reader io.Reader) AttributeFilter {
	return AttributeFilter{
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
	}
}

func (c FfiConverterAttributeFilter) Lower(value AttributeFilter) C.RustBuffer {
	return LowerIntoRustBuffer[AttributeFilter](c, value)
}

func (c FfiConverterAttributeFilter) Write(writer io.Writer, value AttributeFilter) {
	FfiConverterStringINSTANCE.Write(writer, value.TraitName)
	FfiConverterStringINSTANCE.Write(writer, value.TraitValue)
}

type FfiDestroyerAttributeFilter struct{}

func (_ FfiDestroyerAttributeFilter) Destroy(value AttributeFilter) {
	value.Destroy()
}

type CompositeClause struct {
	Operator LogicalOperator
	Clauses  []Clause
}

func (r *CompositeClause) Destroy() {
	FfiDestroyerLogicalOperator{}.Destroy(r.Operator)
	FfiDestroyerSequenceClause{}.Destroy(r.Clauses)
}

type FfiConverterCompositeClause struct{}

var FfiConverterCompositeClauseINSTANCE = FfiConverterCompositeClause{}

func (c FfiConverterCompositeClause) Lift(rb RustBufferI) CompositeClause {
	return LiftFromRustBuffer[CompositeClause](c, rb)
}

func (c FfiConverterCompositeClause) Read(reader io.Reader) CompositeClause {
	return CompositeClause{
		FfiConverterLogicalOperatorINSTANCE.Read(reader),
		FfiConverterSequenceClauseINSTANCE.Read(reader),
	}
}

func (c FfiConverterCompositeClause) Lower(value CompositeClause) C.RustBuffer {
	return LowerIntoRustBuffer[CompositeClause](c, value)
}

func (c FfiConverterCompositeClause) Write(writer io.Writer, value CompositeClause) {
	FfiConverterLogicalOperatorINSTANCE.Write(writer, value.Operator)
	FfiConverterSequenceClauseINSTANCE.Write(writer, value.Clauses)
}

type FfiDestroyerCompositeClause struct{}

func (_ FfiDestroyerCompositeClause) Destroy(value CompositeClause) {
	value.Destroy()
}

type Contract struct {
	ContractAddress    FieldElement
	ContractType       ContractType
	Head               *uint64
	Tps                *uint64
	LastBlockTimestamp *uint64
	LastPendingBlockTx *FieldElement
	UpdatedAt          uint64
	CreatedAt          uint64
}

func (r *Contract) Destroy() {
	FfiDestroyerTypeFieldElement{}.Destroy(r.ContractAddress)
	FfiDestroyerContractType{}.Destroy(r.ContractType)
	FfiDestroyerOptionalUint64{}.Destroy(r.Head)
	FfiDestroyerOptionalUint64{}.Destroy(r.Tps)
	FfiDestroyerOptionalUint64{}.Destroy(r.LastBlockTimestamp)
	FfiDestroyerOptionalTypeFieldElement{}.Destroy(r.LastPendingBlockTx)
	FfiDestroyerUint64{}.Destroy(r.UpdatedAt)
	FfiDestroyerUint64{}.Destroy(r.CreatedAt)
}

type FfiConverterContract struct{}

var FfiConverterContractINSTANCE = FfiConverterContract{}

func (c FfiConverterContract) Lift(rb RustBufferI) Contract {
	return LiftFromRustBuffer[Contract](c, rb)
}

func (c FfiConverterContract) Read(reader io.Reader) Contract {
	return Contract{
		FfiConverterTypeFieldElementINSTANCE.Read(reader),
		FfiConverterContractTypeINSTANCE.Read(reader),
		FfiConverterOptionalUint64INSTANCE.Read(reader),
		FfiConverterOptionalUint64INSTANCE.Read(reader),
		FfiConverterOptionalUint64INSTANCE.Read(reader),
		FfiConverterOptionalTypeFieldElementINSTANCE.Read(reader),
		FfiConverterUint64INSTANCE.Read(reader),
		FfiConverterUint64INSTANCE.Read(reader),
	}
}

func (c FfiConverterContract) Lower(value Contract) C.RustBuffer {
	return LowerIntoRustBuffer[Contract](c, value)
}

func (c FfiConverterContract) Write(writer io.Writer, value Contract) {
	FfiConverterTypeFieldElementINSTANCE.Write(writer, value.ContractAddress)
	FfiConverterContractTypeINSTANCE.Write(writer, value.ContractType)
	FfiConverterOptionalUint64INSTANCE.Write(writer, value.Head)
	FfiConverterOptionalUint64INSTANCE.Write(writer, value.Tps)
	FfiConverterOptionalUint64INSTANCE.Write(writer, value.LastBlockTimestamp)
	FfiConverterOptionalTypeFieldElementINSTANCE.Write(writer, value.LastPendingBlockTx)
	FfiConverterUint64INSTANCE.Write(writer, value.UpdatedAt)
	FfiConverterUint64INSTANCE.Write(writer, value.CreatedAt)
}

type FfiDestroyerContract struct{}

func (_ FfiDestroyerContract) Destroy(value Contract) {
	value.Destroy()
}

type ContractQuery struct {
	ContractAddresses []FieldElement
	ContractTypes     []ContractType
}

func (r *ContractQuery) Destroy() {
	FfiDestroyerSequenceTypeFieldElement{}.Destroy(r.ContractAddresses)
	FfiDestroyerSequenceContractType{}.Destroy(r.ContractTypes)
}

type FfiConverterContractQuery struct{}

var FfiConverterContractQueryINSTANCE = FfiConverterContractQuery{}

func (c FfiConverterContractQuery) Lift(rb RustBufferI) ContractQuery {
	return LiftFromRustBuffer[ContractQuery](c, rb)
}

func (c FfiConverterContractQuery) Read(reader io.Reader) ContractQuery {
	return ContractQuery{
		FfiConverterSequenceTypeFieldElementINSTANCE.Read(reader),
		FfiConverterSequenceContractTypeINSTANCE.Read(reader),
	}
}

func (c FfiConverterContractQuery) Lower(value ContractQuery) C.RustBuffer {
	return LowerIntoRustBuffer[ContractQuery](c, value)
}

func (c FfiConverterContractQuery) Write(writer io.Writer, value ContractQuery) {
	FfiConverterSequenceTypeFieldElementINSTANCE.Write(writer, value.ContractAddresses)
	FfiConverterSequenceContractTypeINSTANCE.Write(writer, value.ContractTypes)
}

type FfiDestroyerContractQuery struct{}

func (_ FfiDestroyerContractQuery) Destroy(value ContractQuery) {
	value.Destroy()
}

type Controller struct {
	Address             FieldElement
	Username            string
	DeployedAtTimestamp uint64
}

func (r *Controller) Destroy() {
	FfiDestroyerTypeFieldElement{}.Destroy(r.Address)
	FfiDestroyerString{}.Destroy(r.Username)
	FfiDestroyerUint64{}.Destroy(r.DeployedAtTimestamp)
}

type FfiConverterController struct{}

var FfiConverterControllerINSTANCE = FfiConverterController{}

func (c FfiConverterController) Lift(rb RustBufferI) Controller {
	return LiftFromRustBuffer[Controller](c, rb)
}

func (c FfiConverterController) Read(reader io.Reader) Controller {
	return Controller{
		FfiConverterTypeFieldElementINSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterUint64INSTANCE.Read(reader),
	}
}

func (c FfiConverterController) Lower(value Controller) C.RustBuffer {
	return LowerIntoRustBuffer[Controller](c, value)
}

func (c FfiConverterController) Write(writer io.Writer, value Controller) {
	FfiConverterTypeFieldElementINSTANCE.Write(writer, value.Address)
	FfiConverterStringINSTANCE.Write(writer, value.Username)
	FfiConverterUint64INSTANCE.Write(writer, value.DeployedAtTimestamp)
}

type FfiDestroyerController struct{}

func (_ FfiDestroyerController) Destroy(value Controller) {
	value.Destroy()
}

type ControllerQuery struct {
	Pagination        Pagination
	ContractAddresses []FieldElement
	Usernames         []string
}

func (r *ControllerQuery) Destroy() {
	FfiDestroyerPagination{}.Destroy(r.Pagination)
	FfiDestroyerSequenceTypeFieldElement{}.Destroy(r.ContractAddresses)
	FfiDestroyerSequenceString{}.Destroy(r.Usernames)
}

type FfiConverterControllerQuery struct{}

var FfiConverterControllerQueryINSTANCE = FfiConverterControllerQuery{}

func (c FfiConverterControllerQuery) Lift(rb RustBufferI) ControllerQuery {
	return LiftFromRustBuffer[ControllerQuery](c, rb)
}

func (c FfiConverterControllerQuery) Read(reader io.Reader) ControllerQuery {
	return ControllerQuery{
		FfiConverterPaginationINSTANCE.Read(reader),
		FfiConverterSequenceTypeFieldElementINSTANCE.Read(reader),
		FfiConverterSequenceStringINSTANCE.Read(reader),
	}
}

func (c FfiConverterControllerQuery) Lower(value ControllerQuery) C.RustBuffer {
	return LowerIntoRustBuffer[ControllerQuery](c, value)
}

func (c FfiConverterControllerQuery) Write(writer io.Writer, value ControllerQuery) {
	FfiConverterPaginationINSTANCE.Write(writer, value.Pagination)
	FfiConverterSequenceTypeFieldElementINSTANCE.Write(writer, value.ContractAddresses)
	FfiConverterSequenceStringINSTANCE.Write(writer, value.Usernames)
}

type FfiDestroyerControllerQuery struct{}

func (_ FfiDestroyerControllerQuery) Destroy(value ControllerQuery) {
	value.Destroy()
}

type Entity struct {
	WorldAddress FieldElement
	HashedKeys   FieldElement
	Models       []Struct
	CreatedAt    uint64
	UpdatedAt    uint64
	ExecutedAt   uint64
}

func (r *Entity) Destroy() {
	FfiDestroyerTypeFieldElement{}.Destroy(r.WorldAddress)
	FfiDestroyerTypeFieldElement{}.Destroy(r.HashedKeys)
	FfiDestroyerSequenceStruct{}.Destroy(r.Models)
	FfiDestroyerUint64{}.Destroy(r.CreatedAt)
	FfiDestroyerUint64{}.Destroy(r.UpdatedAt)
	FfiDestroyerUint64{}.Destroy(r.ExecutedAt)
}

type FfiConverterEntity struct{}

var FfiConverterEntityINSTANCE = FfiConverterEntity{}

func (c FfiConverterEntity) Lift(rb RustBufferI) Entity {
	return LiftFromRustBuffer[Entity](c, rb)
}

func (c FfiConverterEntity) Read(reader io.Reader) Entity {
	return Entity{
		FfiConverterTypeFieldElementINSTANCE.Read(reader),
		FfiConverterTypeFieldElementINSTANCE.Read(reader),
		FfiConverterSequenceStructINSTANCE.Read(reader),
		FfiConverterUint64INSTANCE.Read(reader),
		FfiConverterUint64INSTANCE.Read(reader),
		FfiConverterUint64INSTANCE.Read(reader),
	}
}

func (c FfiConverterEntity) Lower(value Entity) C.RustBuffer {
	return LowerIntoRustBuffer[Entity](c, value)
}

func (c FfiConverterEntity) Write(writer io.Writer, value Entity) {
	FfiConverterTypeFieldElementINSTANCE.Write(writer, value.WorldAddress)
	FfiConverterTypeFieldElementINSTANCE.Write(writer, value.HashedKeys)
	FfiConverterSequenceStructINSTANCE.Write(writer, value.Models)
	FfiConverterUint64INSTANCE.Write(writer, value.CreatedAt)
	FfiConverterUint64INSTANCE.Write(writer, value.UpdatedAt)
	FfiConverterUint64INSTANCE.Write(writer, value.ExecutedAt)
}

type FfiDestroyerEntity struct{}

func (_ FfiDestroyerEntity) Destroy(value Entity) {
	value.Destroy()
}

type EnumOption struct {
	Name string
	Ty   Ty
}

func (r *EnumOption) Destroy() {
	FfiDestroyerString{}.Destroy(r.Name)
	FfiDestroyerTy{}.Destroy(r.Ty)
}

type FfiConverterEnumOption struct{}

var FfiConverterEnumOptionINSTANCE = FfiConverterEnumOption{}

func (c FfiConverterEnumOption) Lift(rb RustBufferI) EnumOption {
	return LiftFromRustBuffer[EnumOption](c, rb)
}

func (c FfiConverterEnumOption) Read(reader io.Reader) EnumOption {
	return EnumOption{
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterTyINSTANCE.Read(reader),
	}
}

func (c FfiConverterEnumOption) Lower(value EnumOption) C.RustBuffer {
	return LowerIntoRustBuffer[EnumOption](c, value)
}

func (c FfiConverterEnumOption) Write(writer io.Writer, value EnumOption) {
	FfiConverterStringINSTANCE.Write(writer, value.Name)
	FfiConverterTyINSTANCE.Write(writer, value.Ty)
}

type FfiDestroyerEnumOption struct{}

func (_ FfiDestroyerEnumOption) Destroy(value EnumOption) {
	value.Destroy()
}

type EnumType struct {
	Name    string
	Option  uint8
	Options []EnumOption
}

func (r *EnumType) Destroy() {
	FfiDestroyerString{}.Destroy(r.Name)
	FfiDestroyerUint8{}.Destroy(r.Option)
	FfiDestroyerSequenceEnumOption{}.Destroy(r.Options)
}

type FfiConverterEnumType struct{}

var FfiConverterEnumTypeINSTANCE = FfiConverterEnumType{}

func (c FfiConverterEnumType) Lift(rb RustBufferI) EnumType {
	return LiftFromRustBuffer[EnumType](c, rb)
}

func (c FfiConverterEnumType) Read(reader io.Reader) EnumType {
	return EnumType{
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterUint8INSTANCE.Read(reader),
		FfiConverterSequenceEnumOptionINSTANCE.Read(reader),
	}
}

func (c FfiConverterEnumType) Lower(value EnumType) C.RustBuffer {
	return LowerIntoRustBuffer[EnumType](c, value)
}

func (c FfiConverterEnumType) Write(writer io.Writer, value EnumType) {
	FfiConverterStringINSTANCE.Write(writer, value.Name)
	FfiConverterUint8INSTANCE.Write(writer, value.Option)
	FfiConverterSequenceEnumOptionINSTANCE.Write(writer, value.Options)
}

type FfiDestroyerEnumType struct{}

func (_ FfiDestroyerEnumType) Destroy(value EnumType) {
	value.Destroy()
}

type Event struct {
	Keys            []FieldElement
	Data            []FieldElement
	TransactionHash FieldElement
}

func (r *Event) Destroy() {
	FfiDestroyerSequenceTypeFieldElement{}.Destroy(r.Keys)
	FfiDestroyerSequenceTypeFieldElement{}.Destroy(r.Data)
	FfiDestroyerTypeFieldElement{}.Destroy(r.TransactionHash)
}

type FfiConverterEvent struct{}

var FfiConverterEventINSTANCE = FfiConverterEvent{}

func (c FfiConverterEvent) Lift(rb RustBufferI) Event {
	return LiftFromRustBuffer[Event](c, rb)
}

func (c FfiConverterEvent) Read(reader io.Reader) Event {
	return Event{
		FfiConverterSequenceTypeFieldElementINSTANCE.Read(reader),
		FfiConverterSequenceTypeFieldElementINSTANCE.Read(reader),
		FfiConverterTypeFieldElementINSTANCE.Read(reader),
	}
}

func (c FfiConverterEvent) Lower(value Event) C.RustBuffer {
	return LowerIntoRustBuffer[Event](c, value)
}

func (c FfiConverterEvent) Write(writer io.Writer, value Event) {
	FfiConverterSequenceTypeFieldElementINSTANCE.Write(writer, value.Keys)
	FfiConverterSequenceTypeFieldElementINSTANCE.Write(writer, value.Data)
	FfiConverterTypeFieldElementINSTANCE.Write(writer, value.TransactionHash)
}

type FfiDestroyerEvent struct{}

func (_ FfiDestroyerEvent) Destroy(value Event) {
	value.Destroy()
}

type EventQuery struct {
	Keys       *KeysClause
	Pagination Pagination
}

func (r *EventQuery) Destroy() {
	FfiDestroyerOptionalKeysClause{}.Destroy(r.Keys)
	FfiDestroyerPagination{}.Destroy(r.Pagination)
}

type FfiConverterEventQuery struct{}

var FfiConverterEventQueryINSTANCE = FfiConverterEventQuery{}

func (c FfiConverterEventQuery) Lift(rb RustBufferI) EventQuery {
	return LiftFromRustBuffer[EventQuery](c, rb)
}

func (c FfiConverterEventQuery) Read(reader io.Reader) EventQuery {
	return EventQuery{
		FfiConverterOptionalKeysClauseINSTANCE.Read(reader),
		FfiConverterPaginationINSTANCE.Read(reader),
	}
}

func (c FfiConverterEventQuery) Lower(value EventQuery) C.RustBuffer {
	return LowerIntoRustBuffer[EventQuery](c, value)
}

func (c FfiConverterEventQuery) Write(writer io.Writer, value EventQuery) {
	FfiConverterOptionalKeysClauseINSTANCE.Write(writer, value.Keys)
	FfiConverterPaginationINSTANCE.Write(writer, value.Pagination)
}

type FfiDestroyerEventQuery struct{}

func (_ FfiDestroyerEventQuery) Destroy(value EventQuery) {
	value.Destroy()
}

type FixedSizeArray struct {
	Array []Ty
	Size  uint32
}

func (r *FixedSizeArray) Destroy() {
	FfiDestroyerSequenceTy{}.Destroy(r.Array)
	FfiDestroyerUint32{}.Destroy(r.Size)
}

type FfiConverterFixedSizeArray struct{}

var FfiConverterFixedSizeArrayINSTANCE = FfiConverterFixedSizeArray{}

func (c FfiConverterFixedSizeArray) Lift(rb RustBufferI) FixedSizeArray {
	return LiftFromRustBuffer[FixedSizeArray](c, rb)
}

func (c FfiConverterFixedSizeArray) Read(reader io.Reader) FixedSizeArray {
	return FixedSizeArray{
		FfiConverterSequenceTyINSTANCE.Read(reader),
		FfiConverterUint32INSTANCE.Read(reader),
	}
}

func (c FfiConverterFixedSizeArray) Lower(value FixedSizeArray) C.RustBuffer {
	return LowerIntoRustBuffer[FixedSizeArray](c, value)
}

func (c FfiConverterFixedSizeArray) Write(writer io.Writer, value FixedSizeArray) {
	FfiConverterSequenceTyINSTANCE.Write(writer, value.Array)
	FfiConverterUint32INSTANCE.Write(writer, value.Size)
}

type FfiDestroyerFixedSizeArray struct{}

func (_ FfiDestroyerFixedSizeArray) Destroy(value FixedSizeArray) {
	value.Destroy()
}

type KeysClause struct {
	Keys            []*FieldElement
	PatternMatching PatternMatching
	Models          []string
}

func (r *KeysClause) Destroy() {
	FfiDestroyerSequenceOptionalTypeFieldElement{}.Destroy(r.Keys)
	FfiDestroyerPatternMatching{}.Destroy(r.PatternMatching)
	FfiDestroyerSequenceString{}.Destroy(r.Models)
}

type FfiConverterKeysClause struct{}

var FfiConverterKeysClauseINSTANCE = FfiConverterKeysClause{}

func (c FfiConverterKeysClause) Lift(rb RustBufferI) KeysClause {
	return LiftFromRustBuffer[KeysClause](c, rb)
}

func (c FfiConverterKeysClause) Read(reader io.Reader) KeysClause {
	return KeysClause{
		FfiConverterSequenceOptionalTypeFieldElementINSTANCE.Read(reader),
		FfiConverterPatternMatchingINSTANCE.Read(reader),
		FfiConverterSequenceStringINSTANCE.Read(reader),
	}
}

func (c FfiConverterKeysClause) Lower(value KeysClause) C.RustBuffer {
	return LowerIntoRustBuffer[KeysClause](c, value)
}

func (c FfiConverterKeysClause) Write(writer io.Writer, value KeysClause) {
	FfiConverterSequenceOptionalTypeFieldElementINSTANCE.Write(writer, value.Keys)
	FfiConverterPatternMatchingINSTANCE.Write(writer, value.PatternMatching)
	FfiConverterSequenceStringINSTANCE.Write(writer, value.Models)
}

type FfiDestroyerKeysClause struct{}

func (_ FfiDestroyerKeysClause) Destroy(value KeysClause) {
	value.Destroy()
}

type Member struct {
	Name string
	Ty   Ty
	Key  bool
}

func (r *Member) Destroy() {
	FfiDestroyerString{}.Destroy(r.Name)
	FfiDestroyerTy{}.Destroy(r.Ty)
	FfiDestroyerBool{}.Destroy(r.Key)
}

type FfiConverterMember struct{}

var FfiConverterMemberINSTANCE = FfiConverterMember{}

func (c FfiConverterMember) Lift(rb RustBufferI) Member {
	return LiftFromRustBuffer[Member](c, rb)
}

func (c FfiConverterMember) Read(reader io.Reader) Member {
	return Member{
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterTyINSTANCE.Read(reader),
		FfiConverterBoolINSTANCE.Read(reader),
	}
}

func (c FfiConverterMember) Lower(value Member) C.RustBuffer {
	return LowerIntoRustBuffer[Member](c, value)
}

func (c FfiConverterMember) Write(writer io.Writer, value Member) {
	FfiConverterStringINSTANCE.Write(writer, value.Name)
	FfiConverterTyINSTANCE.Write(writer, value.Ty)
	FfiConverterBoolINSTANCE.Write(writer, value.Key)
}

type FfiDestroyerMember struct{}

func (_ FfiDestroyerMember) Destroy(value Member) {
	value.Destroy()
}

type MemberClause struct {
	Model    string
	Member   string
	Operator ComparisonOperator
	Value    MemberValue
}

func (r *MemberClause) Destroy() {
	FfiDestroyerString{}.Destroy(r.Model)
	FfiDestroyerString{}.Destroy(r.Member)
	FfiDestroyerComparisonOperator{}.Destroy(r.Operator)
	FfiDestroyerMemberValue{}.Destroy(r.Value)
}

type FfiConverterMemberClause struct{}

var FfiConverterMemberClauseINSTANCE = FfiConverterMemberClause{}

func (c FfiConverterMemberClause) Lift(rb RustBufferI) MemberClause {
	return LiftFromRustBuffer[MemberClause](c, rb)
}

func (c FfiConverterMemberClause) Read(reader io.Reader) MemberClause {
	return MemberClause{
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterComparisonOperatorINSTANCE.Read(reader),
		FfiConverterMemberValueINSTANCE.Read(reader),
	}
}

func (c FfiConverterMemberClause) Lower(value MemberClause) C.RustBuffer {
	return LowerIntoRustBuffer[MemberClause](c, value)
}

func (c FfiConverterMemberClause) Write(writer io.Writer, value MemberClause) {
	FfiConverterStringINSTANCE.Write(writer, value.Model)
	FfiConverterStringINSTANCE.Write(writer, value.Member)
	FfiConverterComparisonOperatorINSTANCE.Write(writer, value.Operator)
	FfiConverterMemberValueINSTANCE.Write(writer, value.Value)
}

type FfiDestroyerMemberClause struct{}

func (_ FfiDestroyerMemberClause) Destroy(value MemberClause) {
	value.Destroy()
}

type Message struct {
	Message      string
	Signature    []FieldElement
	WorldAddress FieldElement
}

func (r *Message) Destroy() {
	FfiDestroyerString{}.Destroy(r.Message)
	FfiDestroyerSequenceTypeFieldElement{}.Destroy(r.Signature)
	FfiDestroyerTypeFieldElement{}.Destroy(r.WorldAddress)
}

type FfiConverterMessage struct{}

var FfiConverterMessageINSTANCE = FfiConverterMessage{}

func (c FfiConverterMessage) Lift(rb RustBufferI) Message {
	return LiftFromRustBuffer[Message](c, rb)
}

func (c FfiConverterMessage) Read(reader io.Reader) Message {
	return Message{
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterSequenceTypeFieldElementINSTANCE.Read(reader),
		FfiConverterTypeFieldElementINSTANCE.Read(reader),
	}
}

func (c FfiConverterMessage) Lower(value Message) C.RustBuffer {
	return LowerIntoRustBuffer[Message](c, value)
}

func (c FfiConverterMessage) Write(writer io.Writer, value Message) {
	FfiConverterStringINSTANCE.Write(writer, value.Message)
	FfiConverterSequenceTypeFieldElementINSTANCE.Write(writer, value.Signature)
	FfiConverterTypeFieldElementINSTANCE.Write(writer, value.WorldAddress)
}

type FfiDestroyerMessage struct{}

func (_ FfiDestroyerMessage) Destroy(value Message) {
	value.Destroy()
}

type Model struct {
	WorldAddress    FieldElement
	Schema          Ty
	Namespace       string
	Name            string
	Selector        FieldElement
	PackedSize      uint32
	UnpackedSize    uint32
	ClassHash       FieldElement
	ContractAddress FieldElement
	Layout          string
	UseLegacyStore  bool
}

func (r *Model) Destroy() {
	FfiDestroyerTypeFieldElement{}.Destroy(r.WorldAddress)
	FfiDestroyerTy{}.Destroy(r.Schema)
	FfiDestroyerString{}.Destroy(r.Namespace)
	FfiDestroyerString{}.Destroy(r.Name)
	FfiDestroyerTypeFieldElement{}.Destroy(r.Selector)
	FfiDestroyerUint32{}.Destroy(r.PackedSize)
	FfiDestroyerUint32{}.Destroy(r.UnpackedSize)
	FfiDestroyerTypeFieldElement{}.Destroy(r.ClassHash)
	FfiDestroyerTypeFieldElement{}.Destroy(r.ContractAddress)
	FfiDestroyerString{}.Destroy(r.Layout)
	FfiDestroyerBool{}.Destroy(r.UseLegacyStore)
}

type FfiConverterModel struct{}

var FfiConverterModelINSTANCE = FfiConverterModel{}

func (c FfiConverterModel) Lift(rb RustBufferI) Model {
	return LiftFromRustBuffer[Model](c, rb)
}

func (c FfiConverterModel) Read(reader io.Reader) Model {
	return Model{
		FfiConverterTypeFieldElementINSTANCE.Read(reader),
		FfiConverterTyINSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterTypeFieldElementINSTANCE.Read(reader),
		FfiConverterUint32INSTANCE.Read(reader),
		FfiConverterUint32INSTANCE.Read(reader),
		FfiConverterTypeFieldElementINSTANCE.Read(reader),
		FfiConverterTypeFieldElementINSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterBoolINSTANCE.Read(reader),
	}
}

func (c FfiConverterModel) Lower(value Model) C.RustBuffer {
	return LowerIntoRustBuffer[Model](c, value)
}

func (c FfiConverterModel) Write(writer io.Writer, value Model) {
	FfiConverterTypeFieldElementINSTANCE.Write(writer, value.WorldAddress)
	FfiConverterTyINSTANCE.Write(writer, value.Schema)
	FfiConverterStringINSTANCE.Write(writer, value.Namespace)
	FfiConverterStringINSTANCE.Write(writer, value.Name)
	FfiConverterTypeFieldElementINSTANCE.Write(writer, value.Selector)
	FfiConverterUint32INSTANCE.Write(writer, value.PackedSize)
	FfiConverterUint32INSTANCE.Write(writer, value.UnpackedSize)
	FfiConverterTypeFieldElementINSTANCE.Write(writer, value.ClassHash)
	FfiConverterTypeFieldElementINSTANCE.Write(writer, value.ContractAddress)
	FfiConverterStringINSTANCE.Write(writer, value.Layout)
	FfiConverterBoolINSTANCE.Write(writer, value.UseLegacyStore)
}

type FfiDestroyerModel struct{}

func (_ FfiDestroyerModel) Destroy(value Model) {
	value.Destroy()
}

type OrderBy struct {
	Field     string
	Direction OrderDirection
}

func (r *OrderBy) Destroy() {
	FfiDestroyerString{}.Destroy(r.Field)
	FfiDestroyerOrderDirection{}.Destroy(r.Direction)
}

type FfiConverterOrderBy struct{}

var FfiConverterOrderByINSTANCE = FfiConverterOrderBy{}

func (c FfiConverterOrderBy) Lift(rb RustBufferI) OrderBy {
	return LiftFromRustBuffer[OrderBy](c, rb)
}

func (c FfiConverterOrderBy) Read(reader io.Reader) OrderBy {
	return OrderBy{
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterOrderDirectionINSTANCE.Read(reader),
	}
}

func (c FfiConverterOrderBy) Lower(value OrderBy) C.RustBuffer {
	return LowerIntoRustBuffer[OrderBy](c, value)
}

func (c FfiConverterOrderBy) Write(writer io.Writer, value OrderBy) {
	FfiConverterStringINSTANCE.Write(writer, value.Field)
	FfiConverterOrderDirectionINSTANCE.Write(writer, value.Direction)
}

type FfiDestroyerOrderBy struct{}

func (_ FfiDestroyerOrderBy) Destroy(value OrderBy) {
	value.Destroy()
}

type PageAchievement struct {
	Items      []Achievement
	NextCursor *string
}

func (r *PageAchievement) Destroy() {
	FfiDestroyerSequenceAchievement{}.Destroy(r.Items)
	FfiDestroyerOptionalString{}.Destroy(r.NextCursor)
}

type FfiConverterPageAchievement struct{}

var FfiConverterPageAchievementINSTANCE = FfiConverterPageAchievement{}

func (c FfiConverterPageAchievement) Lift(rb RustBufferI) PageAchievement {
	return LiftFromRustBuffer[PageAchievement](c, rb)
}

func (c FfiConverterPageAchievement) Read(reader io.Reader) PageAchievement {
	return PageAchievement{
		FfiConverterSequenceAchievementINSTANCE.Read(reader),
		FfiConverterOptionalStringINSTANCE.Read(reader),
	}
}

func (c FfiConverterPageAchievement) Lower(value PageAchievement) C.RustBuffer {
	return LowerIntoRustBuffer[PageAchievement](c, value)
}

func (c FfiConverterPageAchievement) Write(writer io.Writer, value PageAchievement) {
	FfiConverterSequenceAchievementINSTANCE.Write(writer, value.Items)
	FfiConverterOptionalStringINSTANCE.Write(writer, value.NextCursor)
}

type FfiDestroyerPageAchievement struct{}

func (_ FfiDestroyerPageAchievement) Destroy(value PageAchievement) {
	value.Destroy()
}

type PageActivity struct {
	Items      []Activity
	NextCursor *string
}

func (r *PageActivity) Destroy() {
	FfiDestroyerSequenceActivity{}.Destroy(r.Items)
	FfiDestroyerOptionalString{}.Destroy(r.NextCursor)
}

type FfiConverterPageActivity struct{}

var FfiConverterPageActivityINSTANCE = FfiConverterPageActivity{}

func (c FfiConverterPageActivity) Lift(rb RustBufferI) PageActivity {
	return LiftFromRustBuffer[PageActivity](c, rb)
}

func (c FfiConverterPageActivity) Read(reader io.Reader) PageActivity {
	return PageActivity{
		FfiConverterSequenceActivityINSTANCE.Read(reader),
		FfiConverterOptionalStringINSTANCE.Read(reader),
	}
}

func (c FfiConverterPageActivity) Lower(value PageActivity) C.RustBuffer {
	return LowerIntoRustBuffer[PageActivity](c, value)
}

func (c FfiConverterPageActivity) Write(writer io.Writer, value PageActivity) {
	FfiConverterSequenceActivityINSTANCE.Write(writer, value.Items)
	FfiConverterOptionalStringINSTANCE.Write(writer, value.NextCursor)
}

type FfiDestroyerPageActivity struct{}

func (_ FfiDestroyerPageActivity) Destroy(value PageActivity) {
	value.Destroy()
}

type PageAggregationEntry struct {
	Items      []AggregationEntry
	NextCursor *string
}

func (r *PageAggregationEntry) Destroy() {
	FfiDestroyerSequenceAggregationEntry{}.Destroy(r.Items)
	FfiDestroyerOptionalString{}.Destroy(r.NextCursor)
}

type FfiConverterPageAggregationEntry struct{}

var FfiConverterPageAggregationEntryINSTANCE = FfiConverterPageAggregationEntry{}

func (c FfiConverterPageAggregationEntry) Lift(rb RustBufferI) PageAggregationEntry {
	return LiftFromRustBuffer[PageAggregationEntry](c, rb)
}

func (c FfiConverterPageAggregationEntry) Read(reader io.Reader) PageAggregationEntry {
	return PageAggregationEntry{
		FfiConverterSequenceAggregationEntryINSTANCE.Read(reader),
		FfiConverterOptionalStringINSTANCE.Read(reader),
	}
}

func (c FfiConverterPageAggregationEntry) Lower(value PageAggregationEntry) C.RustBuffer {
	return LowerIntoRustBuffer[PageAggregationEntry](c, value)
}

func (c FfiConverterPageAggregationEntry) Write(writer io.Writer, value PageAggregationEntry) {
	FfiConverterSequenceAggregationEntryINSTANCE.Write(writer, value.Items)
	FfiConverterOptionalStringINSTANCE.Write(writer, value.NextCursor)
}

type FfiDestroyerPageAggregationEntry struct{}

func (_ FfiDestroyerPageAggregationEntry) Destroy(value PageAggregationEntry) {
	value.Destroy()
}

type PageController struct {
	Items      []Controller
	NextCursor *string
}

func (r *PageController) Destroy() {
	FfiDestroyerSequenceController{}.Destroy(r.Items)
	FfiDestroyerOptionalString{}.Destroy(r.NextCursor)
}

type FfiConverterPageController struct{}

var FfiConverterPageControllerINSTANCE = FfiConverterPageController{}

func (c FfiConverterPageController) Lift(rb RustBufferI) PageController {
	return LiftFromRustBuffer[PageController](c, rb)
}

func (c FfiConverterPageController) Read(reader io.Reader) PageController {
	return PageController{
		FfiConverterSequenceControllerINSTANCE.Read(reader),
		FfiConverterOptionalStringINSTANCE.Read(reader),
	}
}

func (c FfiConverterPageController) Lower(value PageController) C.RustBuffer {
	return LowerIntoRustBuffer[PageController](c, value)
}

func (c FfiConverterPageController) Write(writer io.Writer, value PageController) {
	FfiConverterSequenceControllerINSTANCE.Write(writer, value.Items)
	FfiConverterOptionalStringINSTANCE.Write(writer, value.NextCursor)
}

type FfiDestroyerPageController struct{}

func (_ FfiDestroyerPageController) Destroy(value PageController) {
	value.Destroy()
}

type PageEntity struct {
	Items      []Entity
	NextCursor *string
}

func (r *PageEntity) Destroy() {
	FfiDestroyerSequenceEntity{}.Destroy(r.Items)
	FfiDestroyerOptionalString{}.Destroy(r.NextCursor)
}

type FfiConverterPageEntity struct{}

var FfiConverterPageEntityINSTANCE = FfiConverterPageEntity{}

func (c FfiConverterPageEntity) Lift(rb RustBufferI) PageEntity {
	return LiftFromRustBuffer[PageEntity](c, rb)
}

func (c FfiConverterPageEntity) Read(reader io.Reader) PageEntity {
	return PageEntity{
		FfiConverterSequenceEntityINSTANCE.Read(reader),
		FfiConverterOptionalStringINSTANCE.Read(reader),
	}
}

func (c FfiConverterPageEntity) Lower(value PageEntity) C.RustBuffer {
	return LowerIntoRustBuffer[PageEntity](c, value)
}

func (c FfiConverterPageEntity) Write(writer io.Writer, value PageEntity) {
	FfiConverterSequenceEntityINSTANCE.Write(writer, value.Items)
	FfiConverterOptionalStringINSTANCE.Write(writer, value.NextCursor)
}

type FfiDestroyerPageEntity struct{}

func (_ FfiDestroyerPageEntity) Destroy(value PageEntity) {
	value.Destroy()
}

type PageEvent struct {
	Items      []Event
	NextCursor *string
}

func (r *PageEvent) Destroy() {
	FfiDestroyerSequenceEvent{}.Destroy(r.Items)
	FfiDestroyerOptionalString{}.Destroy(r.NextCursor)
}

type FfiConverterPageEvent struct{}

var FfiConverterPageEventINSTANCE = FfiConverterPageEvent{}

func (c FfiConverterPageEvent) Lift(rb RustBufferI) PageEvent {
	return LiftFromRustBuffer[PageEvent](c, rb)
}

func (c FfiConverterPageEvent) Read(reader io.Reader) PageEvent {
	return PageEvent{
		FfiConverterSequenceEventINSTANCE.Read(reader),
		FfiConverterOptionalStringINSTANCE.Read(reader),
	}
}

func (c FfiConverterPageEvent) Lower(value PageEvent) C.RustBuffer {
	return LowerIntoRustBuffer[PageEvent](c, value)
}

func (c FfiConverterPageEvent) Write(writer io.Writer, value PageEvent) {
	FfiConverterSequenceEventINSTANCE.Write(writer, value.Items)
	FfiConverterOptionalStringINSTANCE.Write(writer, value.NextCursor)
}

type FfiDestroyerPageEvent struct{}

func (_ FfiDestroyerPageEvent) Destroy(value PageEvent) {
	value.Destroy()
}

type PagePlayerAchievement struct {
	Items      []PlayerAchievementEntry
	NextCursor *string
}

func (r *PagePlayerAchievement) Destroy() {
	FfiDestroyerSequencePlayerAchievementEntry{}.Destroy(r.Items)
	FfiDestroyerOptionalString{}.Destroy(r.NextCursor)
}

type FfiConverterPagePlayerAchievement struct{}

var FfiConverterPagePlayerAchievementINSTANCE = FfiConverterPagePlayerAchievement{}

func (c FfiConverterPagePlayerAchievement) Lift(rb RustBufferI) PagePlayerAchievement {
	return LiftFromRustBuffer[PagePlayerAchievement](c, rb)
}

func (c FfiConverterPagePlayerAchievement) Read(reader io.Reader) PagePlayerAchievement {
	return PagePlayerAchievement{
		FfiConverterSequencePlayerAchievementEntryINSTANCE.Read(reader),
		FfiConverterOptionalStringINSTANCE.Read(reader),
	}
}

func (c FfiConverterPagePlayerAchievement) Lower(value PagePlayerAchievement) C.RustBuffer {
	return LowerIntoRustBuffer[PagePlayerAchievement](c, value)
}

func (c FfiConverterPagePlayerAchievement) Write(writer io.Writer, value PagePlayerAchievement) {
	FfiConverterSequencePlayerAchievementEntryINSTANCE.Write(writer, value.Items)
	FfiConverterOptionalStringINSTANCE.Write(writer, value.NextCursor)
}

type FfiDestroyerPagePlayerAchievement struct{}

func (_ FfiDestroyerPagePlayerAchievement) Destroy(value PagePlayerAchievement) {
	value.Destroy()
}

type PageToken struct {
	Items      []Token
	NextCursor *string
}

func (r *PageToken) Destroy() {
	FfiDestroyerSequenceToken{}.Destroy(r.Items)
	FfiDestroyerOptionalString{}.Destroy(r.NextCursor)
}

type FfiConverterPageToken struct{}

var FfiConverterPageTokenINSTANCE = FfiConverterPageToken{}

func (c FfiConverterPageToken) Lift(rb RustBufferI) PageToken {
	return LiftFromRustBuffer[PageToken](c, rb)
}

func (c FfiConverterPageToken) Read(reader io.Reader) PageToken {
	return PageToken{
		FfiConverterSequenceTokenINSTANCE.Read(reader),
		FfiConverterOptionalStringINSTANCE.Read(reader),
	}
}

func (c FfiConverterPageToken) Lower(value PageToken) C.RustBuffer {
	return LowerIntoRustBuffer[PageToken](c, value)
}

func (c FfiConverterPageToken) Write(writer io.Writer, value PageToken) {
	FfiConverterSequenceTokenINSTANCE.Write(writer, value.Items)
	FfiConverterOptionalStringINSTANCE.Write(writer, value.NextCursor)
}

type FfiDestroyerPageToken struct{}

func (_ FfiDestroyerPageToken) Destroy(value PageToken) {
	value.Destroy()
}

type PageTokenBalance struct {
	Items      []TokenBalance
	NextCursor *string
}

func (r *PageTokenBalance) Destroy() {
	FfiDestroyerSequenceTokenBalance{}.Destroy(r.Items)
	FfiDestroyerOptionalString{}.Destroy(r.NextCursor)
}

type FfiConverterPageTokenBalance struct{}

var FfiConverterPageTokenBalanceINSTANCE = FfiConverterPageTokenBalance{}

func (c FfiConverterPageTokenBalance) Lift(rb RustBufferI) PageTokenBalance {
	return LiftFromRustBuffer[PageTokenBalance](c, rb)
}

func (c FfiConverterPageTokenBalance) Read(reader io.Reader) PageTokenBalance {
	return PageTokenBalance{
		FfiConverterSequenceTokenBalanceINSTANCE.Read(reader),
		FfiConverterOptionalStringINSTANCE.Read(reader),
	}
}

func (c FfiConverterPageTokenBalance) Lower(value PageTokenBalance) C.RustBuffer {
	return LowerIntoRustBuffer[PageTokenBalance](c, value)
}

func (c FfiConverterPageTokenBalance) Write(writer io.Writer, value PageTokenBalance) {
	FfiConverterSequenceTokenBalanceINSTANCE.Write(writer, value.Items)
	FfiConverterOptionalStringINSTANCE.Write(writer, value.NextCursor)
}

type FfiDestroyerPageTokenBalance struct{}

func (_ FfiDestroyerPageTokenBalance) Destroy(value PageTokenBalance) {
	value.Destroy()
}

type PageTokenContract struct {
	Items      []TokenContract
	NextCursor *string
}

func (r *PageTokenContract) Destroy() {
	FfiDestroyerSequenceTokenContract{}.Destroy(r.Items)
	FfiDestroyerOptionalString{}.Destroy(r.NextCursor)
}

type FfiConverterPageTokenContract struct{}

var FfiConverterPageTokenContractINSTANCE = FfiConverterPageTokenContract{}

func (c FfiConverterPageTokenContract) Lift(rb RustBufferI) PageTokenContract {
	return LiftFromRustBuffer[PageTokenContract](c, rb)
}

func (c FfiConverterPageTokenContract) Read(reader io.Reader) PageTokenContract {
	return PageTokenContract{
		FfiConverterSequenceTokenContractINSTANCE.Read(reader),
		FfiConverterOptionalStringINSTANCE.Read(reader),
	}
}

func (c FfiConverterPageTokenContract) Lower(value PageTokenContract) C.RustBuffer {
	return LowerIntoRustBuffer[PageTokenContract](c, value)
}

func (c FfiConverterPageTokenContract) Write(writer io.Writer, value PageTokenContract) {
	FfiConverterSequenceTokenContractINSTANCE.Write(writer, value.Items)
	FfiConverterOptionalStringINSTANCE.Write(writer, value.NextCursor)
}

type FfiDestroyerPageTokenContract struct{}

func (_ FfiDestroyerPageTokenContract) Destroy(value PageTokenContract) {
	value.Destroy()
}

type PageTokenTransfer struct {
	Items      []TokenTransfer
	NextCursor *string
}

func (r *PageTokenTransfer) Destroy() {
	FfiDestroyerSequenceTokenTransfer{}.Destroy(r.Items)
	FfiDestroyerOptionalString{}.Destroy(r.NextCursor)
}

type FfiConverterPageTokenTransfer struct{}

var FfiConverterPageTokenTransferINSTANCE = FfiConverterPageTokenTransfer{}

func (c FfiConverterPageTokenTransfer) Lift(rb RustBufferI) PageTokenTransfer {
	return LiftFromRustBuffer[PageTokenTransfer](c, rb)
}

func (c FfiConverterPageTokenTransfer) Read(reader io.Reader) PageTokenTransfer {
	return PageTokenTransfer{
		FfiConverterSequenceTokenTransferINSTANCE.Read(reader),
		FfiConverterOptionalStringINSTANCE.Read(reader),
	}
}

func (c FfiConverterPageTokenTransfer) Lower(value PageTokenTransfer) C.RustBuffer {
	return LowerIntoRustBuffer[PageTokenTransfer](c, value)
}

func (c FfiConverterPageTokenTransfer) Write(writer io.Writer, value PageTokenTransfer) {
	FfiConverterSequenceTokenTransferINSTANCE.Write(writer, value.Items)
	FfiConverterOptionalStringINSTANCE.Write(writer, value.NextCursor)
}

type FfiDestroyerPageTokenTransfer struct{}

func (_ FfiDestroyerPageTokenTransfer) Destroy(value PageTokenTransfer) {
	value.Destroy()
}

type PageTransaction struct {
	Items      []Transaction
	NextCursor *string
}

func (r *PageTransaction) Destroy() {
	FfiDestroyerSequenceTransaction{}.Destroy(r.Items)
	FfiDestroyerOptionalString{}.Destroy(r.NextCursor)
}

type FfiConverterPageTransaction struct{}

var FfiConverterPageTransactionINSTANCE = FfiConverterPageTransaction{}

func (c FfiConverterPageTransaction) Lift(rb RustBufferI) PageTransaction {
	return LiftFromRustBuffer[PageTransaction](c, rb)
}

func (c FfiConverterPageTransaction) Read(reader io.Reader) PageTransaction {
	return PageTransaction{
		FfiConverterSequenceTransactionINSTANCE.Read(reader),
		FfiConverterOptionalStringINSTANCE.Read(reader),
	}
}

func (c FfiConverterPageTransaction) Lower(value PageTransaction) C.RustBuffer {
	return LowerIntoRustBuffer[PageTransaction](c, value)
}

func (c FfiConverterPageTransaction) Write(writer io.Writer, value PageTransaction) {
	FfiConverterSequenceTransactionINSTANCE.Write(writer, value.Items)
	FfiConverterOptionalStringINSTANCE.Write(writer, value.NextCursor)
}

type FfiDestroyerPageTransaction struct{}

func (_ FfiDestroyerPageTransaction) Destroy(value PageTransaction) {
	value.Destroy()
}

type Pagination struct {
	Cursor    *string
	Limit     *uint32
	Direction PaginationDirection
	OrderBy   []OrderBy
}

func (r *Pagination) Destroy() {
	FfiDestroyerOptionalString{}.Destroy(r.Cursor)
	FfiDestroyerOptionalUint32{}.Destroy(r.Limit)
	FfiDestroyerPaginationDirection{}.Destroy(r.Direction)
	FfiDestroyerSequenceOrderBy{}.Destroy(r.OrderBy)
}

type FfiConverterPagination struct{}

var FfiConverterPaginationINSTANCE = FfiConverterPagination{}

func (c FfiConverterPagination) Lift(rb RustBufferI) Pagination {
	return LiftFromRustBuffer[Pagination](c, rb)
}

func (c FfiConverterPagination) Read(reader io.Reader) Pagination {
	return Pagination{
		FfiConverterOptionalStringINSTANCE.Read(reader),
		FfiConverterOptionalUint32INSTANCE.Read(reader),
		FfiConverterPaginationDirectionINSTANCE.Read(reader),
		FfiConverterSequenceOrderByINSTANCE.Read(reader),
	}
}

func (c FfiConverterPagination) Lower(value Pagination) C.RustBuffer {
	return LowerIntoRustBuffer[Pagination](c, value)
}

func (c FfiConverterPagination) Write(writer io.Writer, value Pagination) {
	FfiConverterOptionalStringINSTANCE.Write(writer, value.Cursor)
	FfiConverterOptionalUint32INSTANCE.Write(writer, value.Limit)
	FfiConverterPaginationDirectionINSTANCE.Write(writer, value.Direction)
	FfiConverterSequenceOrderByINSTANCE.Write(writer, value.OrderBy)
}

type FfiDestroyerPagination struct{}

func (_ FfiDestroyerPagination) Destroy(value Pagination) {
	value.Destroy()
}

type PlayerAchievementEntry struct {
	PlayerAddress FieldElement
	Stats         PlayerAchievementStats
	Achievements  []PlayerAchievementProgress
}

func (r *PlayerAchievementEntry) Destroy() {
	FfiDestroyerTypeFieldElement{}.Destroy(r.PlayerAddress)
	FfiDestroyerPlayerAchievementStats{}.Destroy(r.Stats)
	FfiDestroyerSequencePlayerAchievementProgress{}.Destroy(r.Achievements)
}

type FfiConverterPlayerAchievementEntry struct{}

var FfiConverterPlayerAchievementEntryINSTANCE = FfiConverterPlayerAchievementEntry{}

func (c FfiConverterPlayerAchievementEntry) Lift(rb RustBufferI) PlayerAchievementEntry {
	return LiftFromRustBuffer[PlayerAchievementEntry](c, rb)
}

func (c FfiConverterPlayerAchievementEntry) Read(reader io.Reader) PlayerAchievementEntry {
	return PlayerAchievementEntry{
		FfiConverterTypeFieldElementINSTANCE.Read(reader),
		FfiConverterPlayerAchievementStatsINSTANCE.Read(reader),
		FfiConverterSequencePlayerAchievementProgressINSTANCE.Read(reader),
	}
}

func (c FfiConverterPlayerAchievementEntry) Lower(value PlayerAchievementEntry) C.RustBuffer {
	return LowerIntoRustBuffer[PlayerAchievementEntry](c, value)
}

func (c FfiConverterPlayerAchievementEntry) Write(writer io.Writer, value PlayerAchievementEntry) {
	FfiConverterTypeFieldElementINSTANCE.Write(writer, value.PlayerAddress)
	FfiConverterPlayerAchievementStatsINSTANCE.Write(writer, value.Stats)
	FfiConverterSequencePlayerAchievementProgressINSTANCE.Write(writer, value.Achievements)
}

type FfiDestroyerPlayerAchievementEntry struct{}

func (_ FfiDestroyerPlayerAchievementEntry) Destroy(value PlayerAchievementEntry) {
	value.Destroy()
}

type PlayerAchievementProgress struct {
	Achievement        Achievement
	TaskProgress       []TaskProgress
	Completed          bool
	ProgressPercentage float64
}

func (r *PlayerAchievementProgress) Destroy() {
	FfiDestroyerAchievement{}.Destroy(r.Achievement)
	FfiDestroyerSequenceTaskProgress{}.Destroy(r.TaskProgress)
	FfiDestroyerBool{}.Destroy(r.Completed)
	FfiDestroyerFloat64{}.Destroy(r.ProgressPercentage)
}

type FfiConverterPlayerAchievementProgress struct{}

var FfiConverterPlayerAchievementProgressINSTANCE = FfiConverterPlayerAchievementProgress{}

func (c FfiConverterPlayerAchievementProgress) Lift(rb RustBufferI) PlayerAchievementProgress {
	return LiftFromRustBuffer[PlayerAchievementProgress](c, rb)
}

func (c FfiConverterPlayerAchievementProgress) Read(reader io.Reader) PlayerAchievementProgress {
	return PlayerAchievementProgress{
		FfiConverterAchievementINSTANCE.Read(reader),
		FfiConverterSequenceTaskProgressINSTANCE.Read(reader),
		FfiConverterBoolINSTANCE.Read(reader),
		FfiConverterFloat64INSTANCE.Read(reader),
	}
}

func (c FfiConverterPlayerAchievementProgress) Lower(value PlayerAchievementProgress) C.RustBuffer {
	return LowerIntoRustBuffer[PlayerAchievementProgress](c, value)
}

func (c FfiConverterPlayerAchievementProgress) Write(writer io.Writer, value PlayerAchievementProgress) {
	FfiConverterAchievementINSTANCE.Write(writer, value.Achievement)
	FfiConverterSequenceTaskProgressINSTANCE.Write(writer, value.TaskProgress)
	FfiConverterBoolINSTANCE.Write(writer, value.Completed)
	FfiConverterFloat64INSTANCE.Write(writer, value.ProgressPercentage)
}

type FfiDestroyerPlayerAchievementProgress struct{}

func (_ FfiDestroyerPlayerAchievementProgress) Destroy(value PlayerAchievementProgress) {
	value.Destroy()
}

type PlayerAchievementQuery struct {
	WorldAddresses  []FieldElement
	Namespaces      []string
	PlayerAddresses []FieldElement
	Pagination      Pagination
}

func (r *PlayerAchievementQuery) Destroy() {
	FfiDestroyerSequenceTypeFieldElement{}.Destroy(r.WorldAddresses)
	FfiDestroyerSequenceString{}.Destroy(r.Namespaces)
	FfiDestroyerSequenceTypeFieldElement{}.Destroy(r.PlayerAddresses)
	FfiDestroyerPagination{}.Destroy(r.Pagination)
}

type FfiConverterPlayerAchievementQuery struct{}

var FfiConverterPlayerAchievementQueryINSTANCE = FfiConverterPlayerAchievementQuery{}

func (c FfiConverterPlayerAchievementQuery) Lift(rb RustBufferI) PlayerAchievementQuery {
	return LiftFromRustBuffer[PlayerAchievementQuery](c, rb)
}

func (c FfiConverterPlayerAchievementQuery) Read(reader io.Reader) PlayerAchievementQuery {
	return PlayerAchievementQuery{
		FfiConverterSequenceTypeFieldElementINSTANCE.Read(reader),
		FfiConverterSequenceStringINSTANCE.Read(reader),
		FfiConverterSequenceTypeFieldElementINSTANCE.Read(reader),
		FfiConverterPaginationINSTANCE.Read(reader),
	}
}

func (c FfiConverterPlayerAchievementQuery) Lower(value PlayerAchievementQuery) C.RustBuffer {
	return LowerIntoRustBuffer[PlayerAchievementQuery](c, value)
}

func (c FfiConverterPlayerAchievementQuery) Write(writer io.Writer, value PlayerAchievementQuery) {
	FfiConverterSequenceTypeFieldElementINSTANCE.Write(writer, value.WorldAddresses)
	FfiConverterSequenceStringINSTANCE.Write(writer, value.Namespaces)
	FfiConverterSequenceTypeFieldElementINSTANCE.Write(writer, value.PlayerAddresses)
	FfiConverterPaginationINSTANCE.Write(writer, value.Pagination)
}

type FfiDestroyerPlayerAchievementQuery struct{}

func (_ FfiDestroyerPlayerAchievementQuery) Destroy(value PlayerAchievementQuery) {
	value.Destroy()
}

type PlayerAchievementStats struct {
	TotalPoints           uint32
	CompletedAchievements uint32
	TotalAchievements     uint32
	CompletionPercentage  float64
	LastAchievementAt     *uint64
	CreatedAt             uint64
	UpdatedAt             uint64
}

func (r *PlayerAchievementStats) Destroy() {
	FfiDestroyerUint32{}.Destroy(r.TotalPoints)
	FfiDestroyerUint32{}.Destroy(r.CompletedAchievements)
	FfiDestroyerUint32{}.Destroy(r.TotalAchievements)
	FfiDestroyerFloat64{}.Destroy(r.CompletionPercentage)
	FfiDestroyerOptionalUint64{}.Destroy(r.LastAchievementAt)
	FfiDestroyerUint64{}.Destroy(r.CreatedAt)
	FfiDestroyerUint64{}.Destroy(r.UpdatedAt)
}

type FfiConverterPlayerAchievementStats struct{}

var FfiConverterPlayerAchievementStatsINSTANCE = FfiConverterPlayerAchievementStats{}

func (c FfiConverterPlayerAchievementStats) Lift(rb RustBufferI) PlayerAchievementStats {
	return LiftFromRustBuffer[PlayerAchievementStats](c, rb)
}

func (c FfiConverterPlayerAchievementStats) Read(reader io.Reader) PlayerAchievementStats {
	return PlayerAchievementStats{
		FfiConverterUint32INSTANCE.Read(reader),
		FfiConverterUint32INSTANCE.Read(reader),
		FfiConverterUint32INSTANCE.Read(reader),
		FfiConverterFloat64INSTANCE.Read(reader),
		FfiConverterOptionalUint64INSTANCE.Read(reader),
		FfiConverterUint64INSTANCE.Read(reader),
		FfiConverterUint64INSTANCE.Read(reader),
	}
}

func (c FfiConverterPlayerAchievementStats) Lower(value PlayerAchievementStats) C.RustBuffer {
	return LowerIntoRustBuffer[PlayerAchievementStats](c, value)
}

func (c FfiConverterPlayerAchievementStats) Write(writer io.Writer, value PlayerAchievementStats) {
	FfiConverterUint32INSTANCE.Write(writer, value.TotalPoints)
	FfiConverterUint32INSTANCE.Write(writer, value.CompletedAchievements)
	FfiConverterUint32INSTANCE.Write(writer, value.TotalAchievements)
	FfiConverterFloat64INSTANCE.Write(writer, value.CompletionPercentage)
	FfiConverterOptionalUint64INSTANCE.Write(writer, value.LastAchievementAt)
	FfiConverterUint64INSTANCE.Write(writer, value.CreatedAt)
	FfiConverterUint64INSTANCE.Write(writer, value.UpdatedAt)
}

type FfiDestroyerPlayerAchievementStats struct{}

func (_ FfiDestroyerPlayerAchievementStats) Destroy(value PlayerAchievementStats) {
	value.Destroy()
}

type Query struct {
	WorldAddresses []FieldElement
	Pagination     Pagination
	Clause         *Clause
	NoHashedKeys   bool
	Models         []string
	Historical     bool
}

func (r *Query) Destroy() {
	FfiDestroyerSequenceTypeFieldElement{}.Destroy(r.WorldAddresses)
	FfiDestroyerPagination{}.Destroy(r.Pagination)
	FfiDestroyerOptionalClause{}.Destroy(r.Clause)
	FfiDestroyerBool{}.Destroy(r.NoHashedKeys)
	FfiDestroyerSequenceString{}.Destroy(r.Models)
	FfiDestroyerBool{}.Destroy(r.Historical)
}

type FfiConverterQuery struct{}

var FfiConverterQueryINSTANCE = FfiConverterQuery{}

func (c FfiConverterQuery) Lift(rb RustBufferI) Query {
	return LiftFromRustBuffer[Query](c, rb)
}

func (c FfiConverterQuery) Read(reader io.Reader) Query {
	return Query{
		FfiConverterSequenceTypeFieldElementINSTANCE.Read(reader),
		FfiConverterPaginationINSTANCE.Read(reader),
		FfiConverterOptionalClauseINSTANCE.Read(reader),
		FfiConverterBoolINSTANCE.Read(reader),
		FfiConverterSequenceStringINSTANCE.Read(reader),
		FfiConverterBoolINSTANCE.Read(reader),
	}
}

func (c FfiConverterQuery) Lower(value Query) C.RustBuffer {
	return LowerIntoRustBuffer[Query](c, value)
}

func (c FfiConverterQuery) Write(writer io.Writer, value Query) {
	FfiConverterSequenceTypeFieldElementINSTANCE.Write(writer, value.WorldAddresses)
	FfiConverterPaginationINSTANCE.Write(writer, value.Pagination)
	FfiConverterOptionalClauseINSTANCE.Write(writer, value.Clause)
	FfiConverterBoolINSTANCE.Write(writer, value.NoHashedKeys)
	FfiConverterSequenceStringINSTANCE.Write(writer, value.Models)
	FfiConverterBoolINSTANCE.Write(writer, value.Historical)
}

type FfiDestroyerQuery struct{}

func (_ FfiDestroyerQuery) Destroy(value Query) {
	value.Destroy()
}

type Signature struct {
	R FieldElement
	S FieldElement
}

func (r *Signature) Destroy() {
	FfiDestroyerTypeFieldElement{}.Destroy(r.R)
	FfiDestroyerTypeFieldElement{}.Destroy(r.S)
}

type FfiConverterSignature struct{}

var FfiConverterSignatureINSTANCE = FfiConverterSignature{}

func (c FfiConverterSignature) Lift(rb RustBufferI) Signature {
	return LiftFromRustBuffer[Signature](c, rb)
}

func (c FfiConverterSignature) Read(reader io.Reader) Signature {
	return Signature{
		FfiConverterTypeFieldElementINSTANCE.Read(reader),
		FfiConverterTypeFieldElementINSTANCE.Read(reader),
	}
}

func (c FfiConverterSignature) Lower(value Signature) C.RustBuffer {
	return LowerIntoRustBuffer[Signature](c, value)
}

func (c FfiConverterSignature) Write(writer io.Writer, value Signature) {
	FfiConverterTypeFieldElementINSTANCE.Write(writer, value.R)
	FfiConverterTypeFieldElementINSTANCE.Write(writer, value.S)
}

type FfiDestroyerSignature struct{}

func (_ FfiDestroyerSignature) Destroy(value Signature) {
	value.Destroy()
}

type SqlField struct {
	Name  string
	Value SqlValue
}

func (r *SqlField) Destroy() {
	FfiDestroyerString{}.Destroy(r.Name)
	FfiDestroyerSqlValue{}.Destroy(r.Value)
}

type FfiConverterSqlField struct{}

var FfiConverterSqlFieldINSTANCE = FfiConverterSqlField{}

func (c FfiConverterSqlField) Lift(rb RustBufferI) SqlField {
	return LiftFromRustBuffer[SqlField](c, rb)
}

func (c FfiConverterSqlField) Read(reader io.Reader) SqlField {
	return SqlField{
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterSqlValueINSTANCE.Read(reader),
	}
}

func (c FfiConverterSqlField) Lower(value SqlField) C.RustBuffer {
	return LowerIntoRustBuffer[SqlField](c, value)
}

func (c FfiConverterSqlField) Write(writer io.Writer, value SqlField) {
	FfiConverterStringINSTANCE.Write(writer, value.Name)
	FfiConverterSqlValueINSTANCE.Write(writer, value.Value)
}

type FfiDestroyerSqlField struct{}

func (_ FfiDestroyerSqlField) Destroy(value SqlField) {
	value.Destroy()
}

type SqlRow struct {
	Fields []SqlField
}

func (r *SqlRow) Destroy() {
	FfiDestroyerSequenceSqlField{}.Destroy(r.Fields)
}

type FfiConverterSqlRow struct{}

var FfiConverterSqlRowINSTANCE = FfiConverterSqlRow{}

func (c FfiConverterSqlRow) Lift(rb RustBufferI) SqlRow {
	return LiftFromRustBuffer[SqlRow](c, rb)
}

func (c FfiConverterSqlRow) Read(reader io.Reader) SqlRow {
	return SqlRow{
		FfiConverterSequenceSqlFieldINSTANCE.Read(reader),
	}
}

func (c FfiConverterSqlRow) Lower(value SqlRow) C.RustBuffer {
	return LowerIntoRustBuffer[SqlRow](c, value)
}

func (c FfiConverterSqlRow) Write(writer io.Writer, value SqlRow) {
	FfiConverterSequenceSqlFieldINSTANCE.Write(writer, value.Fields)
}

type FfiDestroyerSqlRow struct{}

func (_ FfiDestroyerSqlRow) Destroy(value SqlRow) {
	value.Destroy()
}

type Struct struct {
	Name     string
	Children []Member
}

func (r *Struct) Destroy() {
	FfiDestroyerString{}.Destroy(r.Name)
	FfiDestroyerSequenceMember{}.Destroy(r.Children)
}

type FfiConverterStruct struct{}

var FfiConverterStructINSTANCE = FfiConverterStruct{}

func (c FfiConverterStruct) Lift(rb RustBufferI) Struct {
	return LiftFromRustBuffer[Struct](c, rb)
}

func (c FfiConverterStruct) Read(reader io.Reader) Struct {
	return Struct{
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterSequenceMemberINSTANCE.Read(reader),
	}
}

func (c FfiConverterStruct) Lower(value Struct) C.RustBuffer {
	return LowerIntoRustBuffer[Struct](c, value)
}

func (c FfiConverterStruct) Write(writer io.Writer, value Struct) {
	FfiConverterStringINSTANCE.Write(writer, value.Name)
	FfiConverterSequenceMemberINSTANCE.Write(writer, value.Children)
}

type FfiDestroyerStruct struct{}

func (_ FfiDestroyerStruct) Destroy(value Struct) {
	value.Destroy()
}

type TaskProgress struct {
	TaskId    string
	Count     uint32
	Completed bool
}

func (r *TaskProgress) Destroy() {
	FfiDestroyerString{}.Destroy(r.TaskId)
	FfiDestroyerUint32{}.Destroy(r.Count)
	FfiDestroyerBool{}.Destroy(r.Completed)
}

type FfiConverterTaskProgress struct{}

var FfiConverterTaskProgressINSTANCE = FfiConverterTaskProgress{}

func (c FfiConverterTaskProgress) Lift(rb RustBufferI) TaskProgress {
	return LiftFromRustBuffer[TaskProgress](c, rb)
}

func (c FfiConverterTaskProgress) Read(reader io.Reader) TaskProgress {
	return TaskProgress{
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterUint32INSTANCE.Read(reader),
		FfiConverterBoolINSTANCE.Read(reader),
	}
}

func (c FfiConverterTaskProgress) Lower(value TaskProgress) C.RustBuffer {
	return LowerIntoRustBuffer[TaskProgress](c, value)
}

func (c FfiConverterTaskProgress) Write(writer io.Writer, value TaskProgress) {
	FfiConverterStringINSTANCE.Write(writer, value.TaskId)
	FfiConverterUint32INSTANCE.Write(writer, value.Count)
	FfiConverterBoolINSTANCE.Write(writer, value.Completed)
}

type FfiDestroyerTaskProgress struct{}

func (_ FfiDestroyerTaskProgress) Destroy(value TaskProgress) {
	value.Destroy()
}

type Token struct {
	ContractAddress FieldElement
	TokenId         *U256
	Name            string
	Symbol          string
	Decimals        uint8
	Metadata        string
	TotalSupply     *U256
}

func (r *Token) Destroy() {
	FfiDestroyerTypeFieldElement{}.Destroy(r.ContractAddress)
	FfiDestroyerOptionalTypeU256{}.Destroy(r.TokenId)
	FfiDestroyerString{}.Destroy(r.Name)
	FfiDestroyerString{}.Destroy(r.Symbol)
	FfiDestroyerUint8{}.Destroy(r.Decimals)
	FfiDestroyerString{}.Destroy(r.Metadata)
	FfiDestroyerOptionalTypeU256{}.Destroy(r.TotalSupply)
}

type FfiConverterToken struct{}

var FfiConverterTokenINSTANCE = FfiConverterToken{}

func (c FfiConverterToken) Lift(rb RustBufferI) Token {
	return LiftFromRustBuffer[Token](c, rb)
}

func (c FfiConverterToken) Read(reader io.Reader) Token {
	return Token{
		FfiConverterTypeFieldElementINSTANCE.Read(reader),
		FfiConverterOptionalTypeU256INSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterUint8INSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterOptionalTypeU256INSTANCE.Read(reader),
	}
}

func (c FfiConverterToken) Lower(value Token) C.RustBuffer {
	return LowerIntoRustBuffer[Token](c, value)
}

func (c FfiConverterToken) Write(writer io.Writer, value Token) {
	FfiConverterTypeFieldElementINSTANCE.Write(writer, value.ContractAddress)
	FfiConverterOptionalTypeU256INSTANCE.Write(writer, value.TokenId)
	FfiConverterStringINSTANCE.Write(writer, value.Name)
	FfiConverterStringINSTANCE.Write(writer, value.Symbol)
	FfiConverterUint8INSTANCE.Write(writer, value.Decimals)
	FfiConverterStringINSTANCE.Write(writer, value.Metadata)
	FfiConverterOptionalTypeU256INSTANCE.Write(writer, value.TotalSupply)
}

type FfiDestroyerToken struct{}

func (_ FfiDestroyerToken) Destroy(value Token) {
	value.Destroy()
}

type TokenBalance struct {
	Balance         U256
	AccountAddress  FieldElement
	ContractAddress FieldElement
	TokenId         *U256
}

func (r *TokenBalance) Destroy() {
	FfiDestroyerTypeU256{}.Destroy(r.Balance)
	FfiDestroyerTypeFieldElement{}.Destroy(r.AccountAddress)
	FfiDestroyerTypeFieldElement{}.Destroy(r.ContractAddress)
	FfiDestroyerOptionalTypeU256{}.Destroy(r.TokenId)
}

type FfiConverterTokenBalance struct{}

var FfiConverterTokenBalanceINSTANCE = FfiConverterTokenBalance{}

func (c FfiConverterTokenBalance) Lift(rb RustBufferI) TokenBalance {
	return LiftFromRustBuffer[TokenBalance](c, rb)
}

func (c FfiConverterTokenBalance) Read(reader io.Reader) TokenBalance {
	return TokenBalance{
		FfiConverterTypeU256INSTANCE.Read(reader),
		FfiConverterTypeFieldElementINSTANCE.Read(reader),
		FfiConverterTypeFieldElementINSTANCE.Read(reader),
		FfiConverterOptionalTypeU256INSTANCE.Read(reader),
	}
}

func (c FfiConverterTokenBalance) Lower(value TokenBalance) C.RustBuffer {
	return LowerIntoRustBuffer[TokenBalance](c, value)
}

func (c FfiConverterTokenBalance) Write(writer io.Writer, value TokenBalance) {
	FfiConverterTypeU256INSTANCE.Write(writer, value.Balance)
	FfiConverterTypeFieldElementINSTANCE.Write(writer, value.AccountAddress)
	FfiConverterTypeFieldElementINSTANCE.Write(writer, value.ContractAddress)
	FfiConverterOptionalTypeU256INSTANCE.Write(writer, value.TokenId)
}

type FfiDestroyerTokenBalance struct{}

func (_ FfiDestroyerTokenBalance) Destroy(value TokenBalance) {
	value.Destroy()
}

type TokenBalanceQuery struct {
	ContractAddresses []FieldElement
	AccountAddresses  []FieldElement
	TokenIds          []U256
	Pagination        Pagination
}

func (r *TokenBalanceQuery) Destroy() {
	FfiDestroyerSequenceTypeFieldElement{}.Destroy(r.ContractAddresses)
	FfiDestroyerSequenceTypeFieldElement{}.Destroy(r.AccountAddresses)
	FfiDestroyerSequenceTypeU256{}.Destroy(r.TokenIds)
	FfiDestroyerPagination{}.Destroy(r.Pagination)
}

type FfiConverterTokenBalanceQuery struct{}

var FfiConverterTokenBalanceQueryINSTANCE = FfiConverterTokenBalanceQuery{}

func (c FfiConverterTokenBalanceQuery) Lift(rb RustBufferI) TokenBalanceQuery {
	return LiftFromRustBuffer[TokenBalanceQuery](c, rb)
}

func (c FfiConverterTokenBalanceQuery) Read(reader io.Reader) TokenBalanceQuery {
	return TokenBalanceQuery{
		FfiConverterSequenceTypeFieldElementINSTANCE.Read(reader),
		FfiConverterSequenceTypeFieldElementINSTANCE.Read(reader),
		FfiConverterSequenceTypeU256INSTANCE.Read(reader),
		FfiConverterPaginationINSTANCE.Read(reader),
	}
}

func (c FfiConverterTokenBalanceQuery) Lower(value TokenBalanceQuery) C.RustBuffer {
	return LowerIntoRustBuffer[TokenBalanceQuery](c, value)
}

func (c FfiConverterTokenBalanceQuery) Write(writer io.Writer, value TokenBalanceQuery) {
	FfiConverterSequenceTypeFieldElementINSTANCE.Write(writer, value.ContractAddresses)
	FfiConverterSequenceTypeFieldElementINSTANCE.Write(writer, value.AccountAddresses)
	FfiConverterSequenceTypeU256INSTANCE.Write(writer, value.TokenIds)
	FfiConverterPaginationINSTANCE.Write(writer, value.Pagination)
}

type FfiDestroyerTokenBalanceQuery struct{}

func (_ FfiDestroyerTokenBalanceQuery) Destroy(value TokenBalanceQuery) {
	value.Destroy()
}

type TokenContract struct {
	ContractAddress FieldElement
	Name            string
	Symbol          string
	Decimals        uint8
	Metadata        string
	TokenMetadata   string
	TotalSupply     *U256
}

func (r *TokenContract) Destroy() {
	FfiDestroyerTypeFieldElement{}.Destroy(r.ContractAddress)
	FfiDestroyerString{}.Destroy(r.Name)
	FfiDestroyerString{}.Destroy(r.Symbol)
	FfiDestroyerUint8{}.Destroy(r.Decimals)
	FfiDestroyerString{}.Destroy(r.Metadata)
	FfiDestroyerString{}.Destroy(r.TokenMetadata)
	FfiDestroyerOptionalTypeU256{}.Destroy(r.TotalSupply)
}

type FfiConverterTokenContract struct{}

var FfiConverterTokenContractINSTANCE = FfiConverterTokenContract{}

func (c FfiConverterTokenContract) Lift(rb RustBufferI) TokenContract {
	return LiftFromRustBuffer[TokenContract](c, rb)
}

func (c FfiConverterTokenContract) Read(reader io.Reader) TokenContract {
	return TokenContract{
		FfiConverterTypeFieldElementINSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterUint8INSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterOptionalTypeU256INSTANCE.Read(reader),
	}
}

func (c FfiConverterTokenContract) Lower(value TokenContract) C.RustBuffer {
	return LowerIntoRustBuffer[TokenContract](c, value)
}

func (c FfiConverterTokenContract) Write(writer io.Writer, value TokenContract) {
	FfiConverterTypeFieldElementINSTANCE.Write(writer, value.ContractAddress)
	FfiConverterStringINSTANCE.Write(writer, value.Name)
	FfiConverterStringINSTANCE.Write(writer, value.Symbol)
	FfiConverterUint8INSTANCE.Write(writer, value.Decimals)
	FfiConverterStringINSTANCE.Write(writer, value.Metadata)
	FfiConverterStringINSTANCE.Write(writer, value.TokenMetadata)
	FfiConverterOptionalTypeU256INSTANCE.Write(writer, value.TotalSupply)
}

type FfiDestroyerTokenContract struct{}

func (_ FfiDestroyerTokenContract) Destroy(value TokenContract) {
	value.Destroy()
}

type TokenContractQuery struct {
	ContractAddresses []FieldElement
	ContractTypes     []ContractType
	Pagination        Pagination
}

func (r *TokenContractQuery) Destroy() {
	FfiDestroyerSequenceTypeFieldElement{}.Destroy(r.ContractAddresses)
	FfiDestroyerSequenceContractType{}.Destroy(r.ContractTypes)
	FfiDestroyerPagination{}.Destroy(r.Pagination)
}

type FfiConverterTokenContractQuery struct{}

var FfiConverterTokenContractQueryINSTANCE = FfiConverterTokenContractQuery{}

func (c FfiConverterTokenContractQuery) Lift(rb RustBufferI) TokenContractQuery {
	return LiftFromRustBuffer[TokenContractQuery](c, rb)
}

func (c FfiConverterTokenContractQuery) Read(reader io.Reader) TokenContractQuery {
	return TokenContractQuery{
		FfiConverterSequenceTypeFieldElementINSTANCE.Read(reader),
		FfiConverterSequenceContractTypeINSTANCE.Read(reader),
		FfiConverterPaginationINSTANCE.Read(reader),
	}
}

func (c FfiConverterTokenContractQuery) Lower(value TokenContractQuery) C.RustBuffer {
	return LowerIntoRustBuffer[TokenContractQuery](c, value)
}

func (c FfiConverterTokenContractQuery) Write(writer io.Writer, value TokenContractQuery) {
	FfiConverterSequenceTypeFieldElementINSTANCE.Write(writer, value.ContractAddresses)
	FfiConverterSequenceContractTypeINSTANCE.Write(writer, value.ContractTypes)
	FfiConverterPaginationINSTANCE.Write(writer, value.Pagination)
}

type FfiDestroyerTokenContractQuery struct{}

func (_ FfiDestroyerTokenContractQuery) Destroy(value TokenContractQuery) {
	value.Destroy()
}

type TokenQuery struct {
	ContractAddresses []FieldElement
	TokenIds          []U256
	AttributeFilters  []AttributeFilter
	Pagination        Pagination
}

func (r *TokenQuery) Destroy() {
	FfiDestroyerSequenceTypeFieldElement{}.Destroy(r.ContractAddresses)
	FfiDestroyerSequenceTypeU256{}.Destroy(r.TokenIds)
	FfiDestroyerSequenceAttributeFilter{}.Destroy(r.AttributeFilters)
	FfiDestroyerPagination{}.Destroy(r.Pagination)
}

type FfiConverterTokenQuery struct{}

var FfiConverterTokenQueryINSTANCE = FfiConverterTokenQuery{}

func (c FfiConverterTokenQuery) Lift(rb RustBufferI) TokenQuery {
	return LiftFromRustBuffer[TokenQuery](c, rb)
}

func (c FfiConverterTokenQuery) Read(reader io.Reader) TokenQuery {
	return TokenQuery{
		FfiConverterSequenceTypeFieldElementINSTANCE.Read(reader),
		FfiConverterSequenceTypeU256INSTANCE.Read(reader),
		FfiConverterSequenceAttributeFilterINSTANCE.Read(reader),
		FfiConverterPaginationINSTANCE.Read(reader),
	}
}

func (c FfiConverterTokenQuery) Lower(value TokenQuery) C.RustBuffer {
	return LowerIntoRustBuffer[TokenQuery](c, value)
}

func (c FfiConverterTokenQuery) Write(writer io.Writer, value TokenQuery) {
	FfiConverterSequenceTypeFieldElementINSTANCE.Write(writer, value.ContractAddresses)
	FfiConverterSequenceTypeU256INSTANCE.Write(writer, value.TokenIds)
	FfiConverterSequenceAttributeFilterINSTANCE.Write(writer, value.AttributeFilters)
	FfiConverterPaginationINSTANCE.Write(writer, value.Pagination)
}

type FfiDestroyerTokenQuery struct{}

func (_ FfiDestroyerTokenQuery) Destroy(value TokenQuery) {
	value.Destroy()
}

type TokenTransfer struct {
	Id              string
	ContractAddress FieldElement
	FromAddress     FieldElement
	ToAddress       FieldElement
	Amount          U256
	TokenId         *U256
	ExecutedAt      uint64
	EventId         *string
}

func (r *TokenTransfer) Destroy() {
	FfiDestroyerString{}.Destroy(r.Id)
	FfiDestroyerTypeFieldElement{}.Destroy(r.ContractAddress)
	FfiDestroyerTypeFieldElement{}.Destroy(r.FromAddress)
	FfiDestroyerTypeFieldElement{}.Destroy(r.ToAddress)
	FfiDestroyerTypeU256{}.Destroy(r.Amount)
	FfiDestroyerOptionalTypeU256{}.Destroy(r.TokenId)
	FfiDestroyerUint64{}.Destroy(r.ExecutedAt)
	FfiDestroyerOptionalString{}.Destroy(r.EventId)
}

type FfiConverterTokenTransfer struct{}

var FfiConverterTokenTransferINSTANCE = FfiConverterTokenTransfer{}

func (c FfiConverterTokenTransfer) Lift(rb RustBufferI) TokenTransfer {
	return LiftFromRustBuffer[TokenTransfer](c, rb)
}

func (c FfiConverterTokenTransfer) Read(reader io.Reader) TokenTransfer {
	return TokenTransfer{
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterTypeFieldElementINSTANCE.Read(reader),
		FfiConverterTypeFieldElementINSTANCE.Read(reader),
		FfiConverterTypeFieldElementINSTANCE.Read(reader),
		FfiConverterTypeU256INSTANCE.Read(reader),
		FfiConverterOptionalTypeU256INSTANCE.Read(reader),
		FfiConverterUint64INSTANCE.Read(reader),
		FfiConverterOptionalStringINSTANCE.Read(reader),
	}
}

func (c FfiConverterTokenTransfer) Lower(value TokenTransfer) C.RustBuffer {
	return LowerIntoRustBuffer[TokenTransfer](c, value)
}

func (c FfiConverterTokenTransfer) Write(writer io.Writer, value TokenTransfer) {
	FfiConverterStringINSTANCE.Write(writer, value.Id)
	FfiConverterTypeFieldElementINSTANCE.Write(writer, value.ContractAddress)
	FfiConverterTypeFieldElementINSTANCE.Write(writer, value.FromAddress)
	FfiConverterTypeFieldElementINSTANCE.Write(writer, value.ToAddress)
	FfiConverterTypeU256INSTANCE.Write(writer, value.Amount)
	FfiConverterOptionalTypeU256INSTANCE.Write(writer, value.TokenId)
	FfiConverterUint64INSTANCE.Write(writer, value.ExecutedAt)
	FfiConverterOptionalStringINSTANCE.Write(writer, value.EventId)
}

type FfiDestroyerTokenTransfer struct{}

func (_ FfiDestroyerTokenTransfer) Destroy(value TokenTransfer) {
	value.Destroy()
}

type TokenTransferQuery struct {
	ContractAddresses []FieldElement
	AccountAddresses  []FieldElement
	TokenIds          []U256
	Pagination        Pagination
}

func (r *TokenTransferQuery) Destroy() {
	FfiDestroyerSequenceTypeFieldElement{}.Destroy(r.ContractAddresses)
	FfiDestroyerSequenceTypeFieldElement{}.Destroy(r.AccountAddresses)
	FfiDestroyerSequenceTypeU256{}.Destroy(r.TokenIds)
	FfiDestroyerPagination{}.Destroy(r.Pagination)
}

type FfiConverterTokenTransferQuery struct{}

var FfiConverterTokenTransferQueryINSTANCE = FfiConverterTokenTransferQuery{}

func (c FfiConverterTokenTransferQuery) Lift(rb RustBufferI) TokenTransferQuery {
	return LiftFromRustBuffer[TokenTransferQuery](c, rb)
}

func (c FfiConverterTokenTransferQuery) Read(reader io.Reader) TokenTransferQuery {
	return TokenTransferQuery{
		FfiConverterSequenceTypeFieldElementINSTANCE.Read(reader),
		FfiConverterSequenceTypeFieldElementINSTANCE.Read(reader),
		FfiConverterSequenceTypeU256INSTANCE.Read(reader),
		FfiConverterPaginationINSTANCE.Read(reader),
	}
}

func (c FfiConverterTokenTransferQuery) Lower(value TokenTransferQuery) C.RustBuffer {
	return LowerIntoRustBuffer[TokenTransferQuery](c, value)
}

func (c FfiConverterTokenTransferQuery) Write(writer io.Writer, value TokenTransferQuery) {
	FfiConverterSequenceTypeFieldElementINSTANCE.Write(writer, value.ContractAddresses)
	FfiConverterSequenceTypeFieldElementINSTANCE.Write(writer, value.AccountAddresses)
	FfiConverterSequenceTypeU256INSTANCE.Write(writer, value.TokenIds)
	FfiConverterPaginationINSTANCE.Write(writer, value.Pagination)
}

type FfiDestroyerTokenTransferQuery struct{}

func (_ FfiDestroyerTokenTransferQuery) Destroy(value TokenTransferQuery) {
	value.Destroy()
}

type Transaction struct {
	TransactionHash FieldElement
	SenderAddress   FieldElement
	Calldata        []FieldElement
	MaxFee          FieldElement
	Signature       []FieldElement
	Nonce           FieldElement
	BlockNumber     uint64
	TransactionType string
	BlockTimestamp  uint64
	Calls           []TransactionCall
	UniqueModels    []FieldElement
}

func (r *Transaction) Destroy() {
	FfiDestroyerTypeFieldElement{}.Destroy(r.TransactionHash)
	FfiDestroyerTypeFieldElement{}.Destroy(r.SenderAddress)
	FfiDestroyerSequenceTypeFieldElement{}.Destroy(r.Calldata)
	FfiDestroyerTypeFieldElement{}.Destroy(r.MaxFee)
	FfiDestroyerSequenceTypeFieldElement{}.Destroy(r.Signature)
	FfiDestroyerTypeFieldElement{}.Destroy(r.Nonce)
	FfiDestroyerUint64{}.Destroy(r.BlockNumber)
	FfiDestroyerString{}.Destroy(r.TransactionType)
	FfiDestroyerUint64{}.Destroy(r.BlockTimestamp)
	FfiDestroyerSequenceTransactionCall{}.Destroy(r.Calls)
	FfiDestroyerSequenceTypeFieldElement{}.Destroy(r.UniqueModels)
}

type FfiConverterTransaction struct{}

var FfiConverterTransactionINSTANCE = FfiConverterTransaction{}

func (c FfiConverterTransaction) Lift(rb RustBufferI) Transaction {
	return LiftFromRustBuffer[Transaction](c, rb)
}

func (c FfiConverterTransaction) Read(reader io.Reader) Transaction {
	return Transaction{
		FfiConverterTypeFieldElementINSTANCE.Read(reader),
		FfiConverterTypeFieldElementINSTANCE.Read(reader),
		FfiConverterSequenceTypeFieldElementINSTANCE.Read(reader),
		FfiConverterTypeFieldElementINSTANCE.Read(reader),
		FfiConverterSequenceTypeFieldElementINSTANCE.Read(reader),
		FfiConverterTypeFieldElementINSTANCE.Read(reader),
		FfiConverterUint64INSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterUint64INSTANCE.Read(reader),
		FfiConverterSequenceTransactionCallINSTANCE.Read(reader),
		FfiConverterSequenceTypeFieldElementINSTANCE.Read(reader),
	}
}

func (c FfiConverterTransaction) Lower(value Transaction) C.RustBuffer {
	return LowerIntoRustBuffer[Transaction](c, value)
}

func (c FfiConverterTransaction) Write(writer io.Writer, value Transaction) {
	FfiConverterTypeFieldElementINSTANCE.Write(writer, value.TransactionHash)
	FfiConverterTypeFieldElementINSTANCE.Write(writer, value.SenderAddress)
	FfiConverterSequenceTypeFieldElementINSTANCE.Write(writer, value.Calldata)
	FfiConverterTypeFieldElementINSTANCE.Write(writer, value.MaxFee)
	FfiConverterSequenceTypeFieldElementINSTANCE.Write(writer, value.Signature)
	FfiConverterTypeFieldElementINSTANCE.Write(writer, value.Nonce)
	FfiConverterUint64INSTANCE.Write(writer, value.BlockNumber)
	FfiConverterStringINSTANCE.Write(writer, value.TransactionType)
	FfiConverterUint64INSTANCE.Write(writer, value.BlockTimestamp)
	FfiConverterSequenceTransactionCallINSTANCE.Write(writer, value.Calls)
	FfiConverterSequenceTypeFieldElementINSTANCE.Write(writer, value.UniqueModels)
}

type FfiDestroyerTransaction struct{}

func (_ FfiDestroyerTransaction) Destroy(value Transaction) {
	value.Destroy()
}

type TransactionCall struct {
	ContractAddress FieldElement
	Entrypoint      string
	Calldata        []FieldElement
	CallType        CallType
	CallerAddress   FieldElement
}

func (r *TransactionCall) Destroy() {
	FfiDestroyerTypeFieldElement{}.Destroy(r.ContractAddress)
	FfiDestroyerString{}.Destroy(r.Entrypoint)
	FfiDestroyerSequenceTypeFieldElement{}.Destroy(r.Calldata)
	FfiDestroyerCallType{}.Destroy(r.CallType)
	FfiDestroyerTypeFieldElement{}.Destroy(r.CallerAddress)
}

type FfiConverterTransactionCall struct{}

var FfiConverterTransactionCallINSTANCE = FfiConverterTransactionCall{}

func (c FfiConverterTransactionCall) Lift(rb RustBufferI) TransactionCall {
	return LiftFromRustBuffer[TransactionCall](c, rb)
}

func (c FfiConverterTransactionCall) Read(reader io.Reader) TransactionCall {
	return TransactionCall{
		FfiConverterTypeFieldElementINSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterSequenceTypeFieldElementINSTANCE.Read(reader),
		FfiConverterCallTypeINSTANCE.Read(reader),
		FfiConverterTypeFieldElementINSTANCE.Read(reader),
	}
}

func (c FfiConverterTransactionCall) Lower(value TransactionCall) C.RustBuffer {
	return LowerIntoRustBuffer[TransactionCall](c, value)
}

func (c FfiConverterTransactionCall) Write(writer io.Writer, value TransactionCall) {
	FfiConverterTypeFieldElementINSTANCE.Write(writer, value.ContractAddress)
	FfiConverterStringINSTANCE.Write(writer, value.Entrypoint)
	FfiConverterSequenceTypeFieldElementINSTANCE.Write(writer, value.Calldata)
	FfiConverterCallTypeINSTANCE.Write(writer, value.CallType)
	FfiConverterTypeFieldElementINSTANCE.Write(writer, value.CallerAddress)
}

type FfiDestroyerTransactionCall struct{}

func (_ FfiDestroyerTransactionCall) Destroy(value TransactionCall) {
	value.Destroy()
}

type TransactionFilter struct {
	TransactionHashes []FieldElement
	CallerAddresses   []FieldElement
	ContractAddresses []FieldElement
	Entrypoints       []string
	ModelSelectors    []FieldElement
	FromBlock         *uint64
	ToBlock           *uint64
}

func (r *TransactionFilter) Destroy() {
	FfiDestroyerSequenceTypeFieldElement{}.Destroy(r.TransactionHashes)
	FfiDestroyerSequenceTypeFieldElement{}.Destroy(r.CallerAddresses)
	FfiDestroyerSequenceTypeFieldElement{}.Destroy(r.ContractAddresses)
	FfiDestroyerSequenceString{}.Destroy(r.Entrypoints)
	FfiDestroyerSequenceTypeFieldElement{}.Destroy(r.ModelSelectors)
	FfiDestroyerOptionalUint64{}.Destroy(r.FromBlock)
	FfiDestroyerOptionalUint64{}.Destroy(r.ToBlock)
}

type FfiConverterTransactionFilter struct{}

var FfiConverterTransactionFilterINSTANCE = FfiConverterTransactionFilter{}

func (c FfiConverterTransactionFilter) Lift(rb RustBufferI) TransactionFilter {
	return LiftFromRustBuffer[TransactionFilter](c, rb)
}

func (c FfiConverterTransactionFilter) Read(reader io.Reader) TransactionFilter {
	return TransactionFilter{
		FfiConverterSequenceTypeFieldElementINSTANCE.Read(reader),
		FfiConverterSequenceTypeFieldElementINSTANCE.Read(reader),
		FfiConverterSequenceTypeFieldElementINSTANCE.Read(reader),
		FfiConverterSequenceStringINSTANCE.Read(reader),
		FfiConverterSequenceTypeFieldElementINSTANCE.Read(reader),
		FfiConverterOptionalUint64INSTANCE.Read(reader),
		FfiConverterOptionalUint64INSTANCE.Read(reader),
	}
}

func (c FfiConverterTransactionFilter) Lower(value TransactionFilter) C.RustBuffer {
	return LowerIntoRustBuffer[TransactionFilter](c, value)
}

func (c FfiConverterTransactionFilter) Write(writer io.Writer, value TransactionFilter) {
	FfiConverterSequenceTypeFieldElementINSTANCE.Write(writer, value.TransactionHashes)
	FfiConverterSequenceTypeFieldElementINSTANCE.Write(writer, value.CallerAddresses)
	FfiConverterSequenceTypeFieldElementINSTANCE.Write(writer, value.ContractAddresses)
	FfiConverterSequenceStringINSTANCE.Write(writer, value.Entrypoints)
	FfiConverterSequenceTypeFieldElementINSTANCE.Write(writer, value.ModelSelectors)
	FfiConverterOptionalUint64INSTANCE.Write(writer, value.FromBlock)
	FfiConverterOptionalUint64INSTANCE.Write(writer, value.ToBlock)
}

type FfiDestroyerTransactionFilter struct{}

func (_ FfiDestroyerTransactionFilter) Destroy(value TransactionFilter) {
	value.Destroy()
}

type TransactionQuery struct {
	Filter     *TransactionFilter
	Pagination Pagination
}

func (r *TransactionQuery) Destroy() {
	FfiDestroyerOptionalTransactionFilter{}.Destroy(r.Filter)
	FfiDestroyerPagination{}.Destroy(r.Pagination)
}

type FfiConverterTransactionQuery struct{}

var FfiConverterTransactionQueryINSTANCE = FfiConverterTransactionQuery{}

func (c FfiConverterTransactionQuery) Lift(rb RustBufferI) TransactionQuery {
	return LiftFromRustBuffer[TransactionQuery](c, rb)
}

func (c FfiConverterTransactionQuery) Read(reader io.Reader) TransactionQuery {
	return TransactionQuery{
		FfiConverterOptionalTransactionFilterINSTANCE.Read(reader),
		FfiConverterPaginationINSTANCE.Read(reader),
	}
}

func (c FfiConverterTransactionQuery) Lower(value TransactionQuery) C.RustBuffer {
	return LowerIntoRustBuffer[TransactionQuery](c, value)
}

func (c FfiConverterTransactionQuery) Write(writer io.Writer, value TransactionQuery) {
	FfiConverterOptionalTransactionFilterINSTANCE.Write(writer, value.Filter)
	FfiConverterPaginationINSTANCE.Write(writer, value.Pagination)
}

type FfiDestroyerTransactionQuery struct{}

func (_ FfiDestroyerTransactionQuery) Destroy(value TransactionQuery) {
	value.Destroy()
}

type World struct {
	WorldAddress FieldElement
	Models       []Model
}

func (r *World) Destroy() {
	FfiDestroyerTypeFieldElement{}.Destroy(r.WorldAddress)
	FfiDestroyerSequenceModel{}.Destroy(r.Models)
}

type FfiConverterWorld struct{}

var FfiConverterWorldINSTANCE = FfiConverterWorld{}

func (c FfiConverterWorld) Lift(rb RustBufferI) World {
	return LiftFromRustBuffer[World](c, rb)
}

func (c FfiConverterWorld) Read(reader io.Reader) World {
	return World{
		FfiConverterTypeFieldElementINSTANCE.Read(reader),
		FfiConverterSequenceModelINSTANCE.Read(reader),
	}
}

func (c FfiConverterWorld) Lower(value World) C.RustBuffer {
	return LowerIntoRustBuffer[World](c, value)
}

func (c FfiConverterWorld) Write(writer io.Writer, value World) {
	FfiConverterTypeFieldElementINSTANCE.Write(writer, value.WorldAddress)
	FfiConverterSequenceModelINSTANCE.Write(writer, value.Models)
}

type FfiDestroyerWorld struct{}

func (_ FfiDestroyerWorld) Destroy(value World) {
	value.Destroy()
}

type CallType uint

const (
	CallTypeExecute            CallType = 1
	CallTypeExecuteFromOutside CallType = 2
)

type FfiConverterCallType struct{}

var FfiConverterCallTypeINSTANCE = FfiConverterCallType{}

func (c FfiConverterCallType) Lift(rb RustBufferI) CallType {
	return LiftFromRustBuffer[CallType](c, rb)
}

func (c FfiConverterCallType) Lower(value CallType) C.RustBuffer {
	return LowerIntoRustBuffer[CallType](c, value)
}
func (FfiConverterCallType) Read(reader io.Reader) CallType {
	id := readInt32(reader)
	return CallType(id)
}

func (FfiConverterCallType) Write(writer io.Writer, value CallType) {
	writeInt32(writer, int32(value))
}

type FfiDestroyerCallType struct{}

func (_ FfiDestroyerCallType) Destroy(value CallType) {
}

type Clause interface {
	Destroy()
}
type ClauseHashedKeys struct {
	Keys []FieldElement
}

func (e ClauseHashedKeys) Destroy() {
	FfiDestroyerSequenceTypeFieldElement{}.Destroy(e.Keys)
}

type ClauseKeys struct {
	Clause KeysClause
}

func (e ClauseKeys) Destroy() {
	FfiDestroyerKeysClause{}.Destroy(e.Clause)
}

type ClauseMember struct {
	Clause MemberClause
}

func (e ClauseMember) Destroy() {
	FfiDestroyerMemberClause{}.Destroy(e.Clause)
}

type ClauseComposite struct {
	Clause CompositeClause
}

func (e ClauseComposite) Destroy() {
	FfiDestroyerCompositeClause{}.Destroy(e.Clause)
}

type FfiConverterClause struct{}

var FfiConverterClauseINSTANCE = FfiConverterClause{}

func (c FfiConverterClause) Lift(rb RustBufferI) Clause {
	return LiftFromRustBuffer[Clause](c, rb)
}

func (c FfiConverterClause) Lower(value Clause) C.RustBuffer {
	return LowerIntoRustBuffer[Clause](c, value)
}
func (FfiConverterClause) Read(reader io.Reader) Clause {
	id := readInt32(reader)
	switch id {
	case 1:
		return ClauseHashedKeys{
			FfiConverterSequenceTypeFieldElementINSTANCE.Read(reader),
		}
	case 2:
		return ClauseKeys{
			FfiConverterKeysClauseINSTANCE.Read(reader),
		}
	case 3:
		return ClauseMember{
			FfiConverterMemberClauseINSTANCE.Read(reader),
		}
	case 4:
		return ClauseComposite{
			FfiConverterCompositeClauseINSTANCE.Read(reader),
		}
	default:
		panic(fmt.Sprintf("invalid enum value %v in FfiConverterClause.Read()", id))
	}
}

func (FfiConverterClause) Write(writer io.Writer, value Clause) {
	switch variant_value := value.(type) {
	case ClauseHashedKeys:
		writeInt32(writer, 1)
		FfiConverterSequenceTypeFieldElementINSTANCE.Write(writer, variant_value.Keys)
	case ClauseKeys:
		writeInt32(writer, 2)
		FfiConverterKeysClauseINSTANCE.Write(writer, variant_value.Clause)
	case ClauseMember:
		writeInt32(writer, 3)
		FfiConverterMemberClauseINSTANCE.Write(writer, variant_value.Clause)
	case ClauseComposite:
		writeInt32(writer, 4)
		FfiConverterCompositeClauseINSTANCE.Write(writer, variant_value.Clause)
	default:
		_ = variant_value
		panic(fmt.Sprintf("invalid enum value `%v` in FfiConverterClause.Write", value))
	}
}

type FfiDestroyerClause struct{}

func (_ FfiDestroyerClause) Destroy(value Clause) {
	value.Destroy()
}

type ComparisonOperator uint

const (
	ComparisonOperatorEq            ComparisonOperator = 1
	ComparisonOperatorNeq           ComparisonOperator = 2
	ComparisonOperatorGt            ComparisonOperator = 3
	ComparisonOperatorGte           ComparisonOperator = 4
	ComparisonOperatorLt            ComparisonOperator = 5
	ComparisonOperatorLte           ComparisonOperator = 6
	ComparisonOperatorIn            ComparisonOperator = 7
	ComparisonOperatorNotIn         ComparisonOperator = 8
	ComparisonOperatorContains      ComparisonOperator = 9
	ComparisonOperatorContainsAll   ComparisonOperator = 10
	ComparisonOperatorContainsAny   ComparisonOperator = 11
	ComparisonOperatorArrayLengthEq ComparisonOperator = 12
	ComparisonOperatorArrayLengthGt ComparisonOperator = 13
	ComparisonOperatorArrayLengthLt ComparisonOperator = 14
)

type FfiConverterComparisonOperator struct{}

var FfiConverterComparisonOperatorINSTANCE = FfiConverterComparisonOperator{}

func (c FfiConverterComparisonOperator) Lift(rb RustBufferI) ComparisonOperator {
	return LiftFromRustBuffer[ComparisonOperator](c, rb)
}

func (c FfiConverterComparisonOperator) Lower(value ComparisonOperator) C.RustBuffer {
	return LowerIntoRustBuffer[ComparisonOperator](c, value)
}
func (FfiConverterComparisonOperator) Read(reader io.Reader) ComparisonOperator {
	id := readInt32(reader)
	return ComparisonOperator(id)
}

func (FfiConverterComparisonOperator) Write(writer io.Writer, value ComparisonOperator) {
	writeInt32(writer, int32(value))
}

type FfiDestroyerComparisonOperator struct{}

func (_ FfiDestroyerComparisonOperator) Destroy(value ComparisonOperator) {
}

type ContractType uint

const (
	ContractTypeWorld   ContractType = 1
	ContractTypeErc20   ContractType = 2
	ContractTypeErc721  ContractType = 3
	ContractTypeErc1155 ContractType = 4
	ContractTypeUdc     ContractType = 5
	ContractTypeOther   ContractType = 6
)

type FfiConverterContractType struct{}

var FfiConverterContractTypeINSTANCE = FfiConverterContractType{}

func (c FfiConverterContractType) Lift(rb RustBufferI) ContractType {
	return LiftFromRustBuffer[ContractType](c, rb)
}

func (c FfiConverterContractType) Lower(value ContractType) C.RustBuffer {
	return LowerIntoRustBuffer[ContractType](c, value)
}
func (FfiConverterContractType) Read(reader io.Reader) ContractType {
	id := readInt32(reader)
	return ContractType(id)
}

func (FfiConverterContractType) Write(writer io.Writer, value ContractType) {
	writeInt32(writer, int32(value))
}

type FfiDestroyerContractType struct{}

func (_ FfiDestroyerContractType) Destroy(value ContractType) {
}

type DojoError struct {
	err error
}

// Convience method to turn *DojoError into error
// Avoiding treating nil pointer as non nil error interface
func (err *DojoError) AsError() error {
	if err == nil {
		return nil
	} else {
		return err
	}
}

func (err DojoError) Error() string {
	return fmt.Sprintf("DojoError: %s", err.err.Error())
}

func (err DojoError) Unwrap() error {
	return err.err
}

// Err* are used for checking error type with `errors.Is`
var ErrDojoErrorClientError = fmt.Errorf("DojoErrorClientError")
var ErrDojoErrorSerializationError = fmt.Errorf("DojoErrorSerializationError")
var ErrDojoErrorNetworkError = fmt.Errorf("DojoErrorNetworkError")
var ErrDojoErrorInvalidInput = fmt.Errorf("DojoErrorInvalidInput")
var ErrDojoErrorConnectionError = fmt.Errorf("DojoErrorConnectionError")
var ErrDojoErrorPublishError = fmt.Errorf("DojoErrorPublishError")
var ErrDojoErrorQueryError = fmt.Errorf("DojoErrorQueryError")
var ErrDojoErrorSubscriptionError = fmt.Errorf("DojoErrorSubscriptionError")

// Variant structs
type DojoErrorClientError struct {
	message string
}

func NewDojoErrorClientError() *DojoError {
	return &DojoError{err: &DojoErrorClientError{}}
}

func (e DojoErrorClientError) destroy() {
}

func (err DojoErrorClientError) Error() string {
	return fmt.Sprintf("ClientError: %s", err.message)
}

func (self DojoErrorClientError) Is(target error) bool {
	return target == ErrDojoErrorClientError
}

type DojoErrorSerializationError struct {
	message string
}

func NewDojoErrorSerializationError() *DojoError {
	return &DojoError{err: &DojoErrorSerializationError{}}
}

func (e DojoErrorSerializationError) destroy() {
}

func (err DojoErrorSerializationError) Error() string {
	return fmt.Sprintf("SerializationError: %s", err.message)
}

func (self DojoErrorSerializationError) Is(target error) bool {
	return target == ErrDojoErrorSerializationError
}

type DojoErrorNetworkError struct {
	message string
}

func NewDojoErrorNetworkError() *DojoError {
	return &DojoError{err: &DojoErrorNetworkError{}}
}

func (e DojoErrorNetworkError) destroy() {
}

func (err DojoErrorNetworkError) Error() string {
	return fmt.Sprintf("NetworkError: %s", err.message)
}

func (self DojoErrorNetworkError) Is(target error) bool {
	return target == ErrDojoErrorNetworkError
}

type DojoErrorInvalidInput struct {
	message string
}

func NewDojoErrorInvalidInput() *DojoError {
	return &DojoError{err: &DojoErrorInvalidInput{}}
}

func (e DojoErrorInvalidInput) destroy() {
}

func (err DojoErrorInvalidInput) Error() string {
	return fmt.Sprintf("InvalidInput: %s", err.message)
}

func (self DojoErrorInvalidInput) Is(target error) bool {
	return target == ErrDojoErrorInvalidInput
}

type DojoErrorConnectionError struct {
	message string
}

func NewDojoErrorConnectionError() *DojoError {
	return &DojoError{err: &DojoErrorConnectionError{}}
}

func (e DojoErrorConnectionError) destroy() {
}

func (err DojoErrorConnectionError) Error() string {
	return fmt.Sprintf("ConnectionError: %s", err.message)
}

func (self DojoErrorConnectionError) Is(target error) bool {
	return target == ErrDojoErrorConnectionError
}

type DojoErrorPublishError struct {
	message string
}

func NewDojoErrorPublishError() *DojoError {
	return &DojoError{err: &DojoErrorPublishError{}}
}

func (e DojoErrorPublishError) destroy() {
}

func (err DojoErrorPublishError) Error() string {
	return fmt.Sprintf("PublishError: %s", err.message)
}

func (self DojoErrorPublishError) Is(target error) bool {
	return target == ErrDojoErrorPublishError
}

type DojoErrorQueryError struct {
	message string
}

func NewDojoErrorQueryError() *DojoError {
	return &DojoError{err: &DojoErrorQueryError{}}
}

func (e DojoErrorQueryError) destroy() {
}

func (err DojoErrorQueryError) Error() string {
	return fmt.Sprintf("QueryError: %s", err.message)
}

func (self DojoErrorQueryError) Is(target error) bool {
	return target == ErrDojoErrorQueryError
}

type DojoErrorSubscriptionError struct {
	message string
}

func NewDojoErrorSubscriptionError() *DojoError {
	return &DojoError{err: &DojoErrorSubscriptionError{}}
}

func (e DojoErrorSubscriptionError) destroy() {
}

func (err DojoErrorSubscriptionError) Error() string {
	return fmt.Sprintf("SubscriptionError: %s", err.message)
}

func (self DojoErrorSubscriptionError) Is(target error) bool {
	return target == ErrDojoErrorSubscriptionError
}

type FfiConverterDojoError struct{}

var FfiConverterDojoErrorINSTANCE = FfiConverterDojoError{}

func (c FfiConverterDojoError) Lift(eb RustBufferI) *DojoError {
	return LiftFromRustBuffer[*DojoError](c, eb)
}

func (c FfiConverterDojoError) Lower(value *DojoError) C.RustBuffer {
	return LowerIntoRustBuffer[*DojoError](c, value)
}

func (c FfiConverterDojoError) Read(reader io.Reader) *DojoError {
	errorID := readUint32(reader)

	message := FfiConverterStringINSTANCE.Read(reader)
	switch errorID {
	case 1:
		return &DojoError{&DojoErrorClientError{message}}
	case 2:
		return &DojoError{&DojoErrorSerializationError{message}}
	case 3:
		return &DojoError{&DojoErrorNetworkError{message}}
	case 4:
		return &DojoError{&DojoErrorInvalidInput{message}}
	case 5:
		return &DojoError{&DojoErrorConnectionError{message}}
	case 6:
		return &DojoError{&DojoErrorPublishError{message}}
	case 7:
		return &DojoError{&DojoErrorQueryError{message}}
	case 8:
		return &DojoError{&DojoErrorSubscriptionError{message}}
	default:
		panic(fmt.Sprintf("Unknown error code %d in FfiConverterDojoError.Read()", errorID))
	}

}

func (c FfiConverterDojoError) Write(writer io.Writer, value *DojoError) {
	switch variantValue := value.err.(type) {
	case *DojoErrorClientError:
		writeInt32(writer, 1)
	case *DojoErrorSerializationError:
		writeInt32(writer, 2)
	case *DojoErrorNetworkError:
		writeInt32(writer, 3)
	case *DojoErrorInvalidInput:
		writeInt32(writer, 4)
	case *DojoErrorConnectionError:
		writeInt32(writer, 5)
	case *DojoErrorPublishError:
		writeInt32(writer, 6)
	case *DojoErrorQueryError:
		writeInt32(writer, 7)
	case *DojoErrorSubscriptionError:
		writeInt32(writer, 8)
	default:
		_ = variantValue
		panic(fmt.Sprintf("invalid error value `%v` in FfiConverterDojoError.Write", value))
	}
}

type FfiDestroyerDojoError struct{}

func (_ FfiDestroyerDojoError) Destroy(value *DojoError) {
	switch variantValue := value.err.(type) {
	case DojoErrorClientError:
		variantValue.destroy()
	case DojoErrorSerializationError:
		variantValue.destroy()
	case DojoErrorNetworkError:
		variantValue.destroy()
	case DojoErrorInvalidInput:
		variantValue.destroy()
	case DojoErrorConnectionError:
		variantValue.destroy()
	case DojoErrorPublishError:
		variantValue.destroy()
	case DojoErrorQueryError:
		variantValue.destroy()
	case DojoErrorSubscriptionError:
		variantValue.destroy()
	default:
		_ = variantValue
		panic(fmt.Sprintf("invalid error value `%v` in FfiDestroyerDojoError.Destroy", value))
	}
}

type LogicalOperator uint

const (
	LogicalOperatorAnd LogicalOperator = 1
	LogicalOperatorOr  LogicalOperator = 2
)

type FfiConverterLogicalOperator struct{}

var FfiConverterLogicalOperatorINSTANCE = FfiConverterLogicalOperator{}

func (c FfiConverterLogicalOperator) Lift(rb RustBufferI) LogicalOperator {
	return LiftFromRustBuffer[LogicalOperator](c, rb)
}

func (c FfiConverterLogicalOperator) Lower(value LogicalOperator) C.RustBuffer {
	return LowerIntoRustBuffer[LogicalOperator](c, value)
}
func (FfiConverterLogicalOperator) Read(reader io.Reader) LogicalOperator {
	id := readInt32(reader)
	return LogicalOperator(id)
}

func (FfiConverterLogicalOperator) Write(writer io.Writer, value LogicalOperator) {
	writeInt32(writer, int32(value))
}

type FfiDestroyerLogicalOperator struct{}

func (_ FfiDestroyerLogicalOperator) Destroy(value LogicalOperator) {
}

type MemberValue interface {
	Destroy()
}
type MemberValuePrimitive struct {
	Value Primitive
}

func (e MemberValuePrimitive) Destroy() {
	FfiDestroyerPrimitive{}.Destroy(e.Value)
}

type MemberValueString struct {
	Value string
}

func (e MemberValueString) Destroy() {
	FfiDestroyerString{}.Destroy(e.Value)
}

type MemberValueList struct {
	Values []MemberValue
}

func (e MemberValueList) Destroy() {
	FfiDestroyerSequenceMemberValue{}.Destroy(e.Values)
}

type FfiConverterMemberValue struct{}

var FfiConverterMemberValueINSTANCE = FfiConverterMemberValue{}

func (c FfiConverterMemberValue) Lift(rb RustBufferI) MemberValue {
	return LiftFromRustBuffer[MemberValue](c, rb)
}

func (c FfiConverterMemberValue) Lower(value MemberValue) C.RustBuffer {
	return LowerIntoRustBuffer[MemberValue](c, value)
}
func (FfiConverterMemberValue) Read(reader io.Reader) MemberValue {
	id := readInt32(reader)
	switch id {
	case 1:
		return MemberValuePrimitive{
			FfiConverterPrimitiveINSTANCE.Read(reader),
		}
	case 2:
		return MemberValueString{
			FfiConverterStringINSTANCE.Read(reader),
		}
	case 3:
		return MemberValueList{
			FfiConverterSequenceMemberValueINSTANCE.Read(reader),
		}
	default:
		panic(fmt.Sprintf("invalid enum value %v in FfiConverterMemberValue.Read()", id))
	}
}

func (FfiConverterMemberValue) Write(writer io.Writer, value MemberValue) {
	switch variant_value := value.(type) {
	case MemberValuePrimitive:
		writeInt32(writer, 1)
		FfiConverterPrimitiveINSTANCE.Write(writer, variant_value.Value)
	case MemberValueString:
		writeInt32(writer, 2)
		FfiConverterStringINSTANCE.Write(writer, variant_value.Value)
	case MemberValueList:
		writeInt32(writer, 3)
		FfiConverterSequenceMemberValueINSTANCE.Write(writer, variant_value.Values)
	default:
		_ = variant_value
		panic(fmt.Sprintf("invalid enum value `%v` in FfiConverterMemberValue.Write", value))
	}
}

type FfiDestroyerMemberValue struct{}

func (_ FfiDestroyerMemberValue) Destroy(value MemberValue) {
	value.Destroy()
}

type OrderDirection uint

const (
	OrderDirectionAsc  OrderDirection = 1
	OrderDirectionDesc OrderDirection = 2
)

type FfiConverterOrderDirection struct{}

var FfiConverterOrderDirectionINSTANCE = FfiConverterOrderDirection{}

func (c FfiConverterOrderDirection) Lift(rb RustBufferI) OrderDirection {
	return LiftFromRustBuffer[OrderDirection](c, rb)
}

func (c FfiConverterOrderDirection) Lower(value OrderDirection) C.RustBuffer {
	return LowerIntoRustBuffer[OrderDirection](c, value)
}
func (FfiConverterOrderDirection) Read(reader io.Reader) OrderDirection {
	id := readInt32(reader)
	return OrderDirection(id)
}

func (FfiConverterOrderDirection) Write(writer io.Writer, value OrderDirection) {
	writeInt32(writer, int32(value))
}

type FfiDestroyerOrderDirection struct{}

func (_ FfiDestroyerOrderDirection) Destroy(value OrderDirection) {
}

type PaginationDirection uint

const (
	PaginationDirectionForward  PaginationDirection = 1
	PaginationDirectionBackward PaginationDirection = 2
)

type FfiConverterPaginationDirection struct{}

var FfiConverterPaginationDirectionINSTANCE = FfiConverterPaginationDirection{}

func (c FfiConverterPaginationDirection) Lift(rb RustBufferI) PaginationDirection {
	return LiftFromRustBuffer[PaginationDirection](c, rb)
}

func (c FfiConverterPaginationDirection) Lower(value PaginationDirection) C.RustBuffer {
	return LowerIntoRustBuffer[PaginationDirection](c, value)
}
func (FfiConverterPaginationDirection) Read(reader io.Reader) PaginationDirection {
	id := readInt32(reader)
	return PaginationDirection(id)
}

func (FfiConverterPaginationDirection) Write(writer io.Writer, value PaginationDirection) {
	writeInt32(writer, int32(value))
}

type FfiDestroyerPaginationDirection struct{}

func (_ FfiDestroyerPaginationDirection) Destroy(value PaginationDirection) {
}

type PatternMatching uint

const (
	PatternMatchingFixedLen    PatternMatching = 1
	PatternMatchingVariableLen PatternMatching = 2
)

type FfiConverterPatternMatching struct{}

var FfiConverterPatternMatchingINSTANCE = FfiConverterPatternMatching{}

func (c FfiConverterPatternMatching) Lift(rb RustBufferI) PatternMatching {
	return LiftFromRustBuffer[PatternMatching](c, rb)
}

func (c FfiConverterPatternMatching) Lower(value PatternMatching) C.RustBuffer {
	return LowerIntoRustBuffer[PatternMatching](c, value)
}
func (FfiConverterPatternMatching) Read(reader io.Reader) PatternMatching {
	id := readInt32(reader)
	return PatternMatching(id)
}

func (FfiConverterPatternMatching) Write(writer io.Writer, value PatternMatching) {
	writeInt32(writer, int32(value))
}

type FfiDestroyerPatternMatching struct{}

func (_ FfiDestroyerPatternMatching) Destroy(value PatternMatching) {
}

type Primitive interface {
	Destroy()
}
type PrimitiveI8 struct {
	Value int8
}

func (e PrimitiveI8) Destroy() {
	FfiDestroyerInt8{}.Destroy(e.Value)
}

type PrimitiveI16 struct {
	Value int16
}

func (e PrimitiveI16) Destroy() {
	FfiDestroyerInt16{}.Destroy(e.Value)
}

type PrimitiveI32 struct {
	Value int32
}

func (e PrimitiveI32) Destroy() {
	FfiDestroyerInt32{}.Destroy(e.Value)
}

type PrimitiveI64 struct {
	Value int64
}

func (e PrimitiveI64) Destroy() {
	FfiDestroyerInt64{}.Destroy(e.Value)
}

type PrimitiveI128 struct {
	Value []uint8
}

func (e PrimitiveI128) Destroy() {
	FfiDestroyerSequenceUint8{}.Destroy(e.Value)
}

type PrimitiveU8 struct {
	Value uint8
}

func (e PrimitiveU8) Destroy() {
	FfiDestroyerUint8{}.Destroy(e.Value)
}

type PrimitiveU16 struct {
	Value uint16
}

func (e PrimitiveU16) Destroy() {
	FfiDestroyerUint16{}.Destroy(e.Value)
}

type PrimitiveU32 struct {
	Value uint32
}

func (e PrimitiveU32) Destroy() {
	FfiDestroyerUint32{}.Destroy(e.Value)
}

type PrimitiveU64 struct {
	Value uint64
}

func (e PrimitiveU64) Destroy() {
	FfiDestroyerUint64{}.Destroy(e.Value)
}

type PrimitiveU128 struct {
	Value []uint8
}

func (e PrimitiveU128) Destroy() {
	FfiDestroyerSequenceUint8{}.Destroy(e.Value)
}

type PrimitiveU256 struct {
	Value U256
}

func (e PrimitiveU256) Destroy() {
	FfiDestroyerTypeU256{}.Destroy(e.Value)
}

type PrimitiveBool struct {
	Value bool
}

func (e PrimitiveBool) Destroy() {
	FfiDestroyerBool{}.Destroy(e.Value)
}

type PrimitiveFelt252 struct {
	Value FieldElement
}

func (e PrimitiveFelt252) Destroy() {
	FfiDestroyerTypeFieldElement{}.Destroy(e.Value)
}

type PrimitiveClassHash struct {
	Value FieldElement
}

func (e PrimitiveClassHash) Destroy() {
	FfiDestroyerTypeFieldElement{}.Destroy(e.Value)
}

type PrimitiveContractAddress struct {
	Value FieldElement
}

func (e PrimitiveContractAddress) Destroy() {
	FfiDestroyerTypeFieldElement{}.Destroy(e.Value)
}

type PrimitiveEthAddress struct {
	Value FieldElement
}

func (e PrimitiveEthAddress) Destroy() {
	FfiDestroyerTypeFieldElement{}.Destroy(e.Value)
}

type FfiConverterPrimitive struct{}

var FfiConverterPrimitiveINSTANCE = FfiConverterPrimitive{}

func (c FfiConverterPrimitive) Lift(rb RustBufferI) Primitive {
	return LiftFromRustBuffer[Primitive](c, rb)
}

func (c FfiConverterPrimitive) Lower(value Primitive) C.RustBuffer {
	return LowerIntoRustBuffer[Primitive](c, value)
}
func (FfiConverterPrimitive) Read(reader io.Reader) Primitive {
	id := readInt32(reader)
	switch id {
	case 1:
		return PrimitiveI8{
			FfiConverterInt8INSTANCE.Read(reader),
		}
	case 2:
		return PrimitiveI16{
			FfiConverterInt16INSTANCE.Read(reader),
		}
	case 3:
		return PrimitiveI32{
			FfiConverterInt32INSTANCE.Read(reader),
		}
	case 4:
		return PrimitiveI64{
			FfiConverterInt64INSTANCE.Read(reader),
		}
	case 5:
		return PrimitiveI128{
			FfiConverterSequenceUint8INSTANCE.Read(reader),
		}
	case 6:
		return PrimitiveU8{
			FfiConverterUint8INSTANCE.Read(reader),
		}
	case 7:
		return PrimitiveU16{
			FfiConverterUint16INSTANCE.Read(reader),
		}
	case 8:
		return PrimitiveU32{
			FfiConverterUint32INSTANCE.Read(reader),
		}
	case 9:
		return PrimitiveU64{
			FfiConverterUint64INSTANCE.Read(reader),
		}
	case 10:
		return PrimitiveU128{
			FfiConverterSequenceUint8INSTANCE.Read(reader),
		}
	case 11:
		return PrimitiveU256{
			FfiConverterTypeU256INSTANCE.Read(reader),
		}
	case 12:
		return PrimitiveBool{
			FfiConverterBoolINSTANCE.Read(reader),
		}
	case 13:
		return PrimitiveFelt252{
			FfiConverterTypeFieldElementINSTANCE.Read(reader),
		}
	case 14:
		return PrimitiveClassHash{
			FfiConverterTypeFieldElementINSTANCE.Read(reader),
		}
	case 15:
		return PrimitiveContractAddress{
			FfiConverterTypeFieldElementINSTANCE.Read(reader),
		}
	case 16:
		return PrimitiveEthAddress{
			FfiConverterTypeFieldElementINSTANCE.Read(reader),
		}
	default:
		panic(fmt.Sprintf("invalid enum value %v in FfiConverterPrimitive.Read()", id))
	}
}

func (FfiConverterPrimitive) Write(writer io.Writer, value Primitive) {
	switch variant_value := value.(type) {
	case PrimitiveI8:
		writeInt32(writer, 1)
		FfiConverterInt8INSTANCE.Write(writer, variant_value.Value)
	case PrimitiveI16:
		writeInt32(writer, 2)
		FfiConverterInt16INSTANCE.Write(writer, variant_value.Value)
	case PrimitiveI32:
		writeInt32(writer, 3)
		FfiConverterInt32INSTANCE.Write(writer, variant_value.Value)
	case PrimitiveI64:
		writeInt32(writer, 4)
		FfiConverterInt64INSTANCE.Write(writer, variant_value.Value)
	case PrimitiveI128:
		writeInt32(writer, 5)
		FfiConverterSequenceUint8INSTANCE.Write(writer, variant_value.Value)
	case PrimitiveU8:
		writeInt32(writer, 6)
		FfiConverterUint8INSTANCE.Write(writer, variant_value.Value)
	case PrimitiveU16:
		writeInt32(writer, 7)
		FfiConverterUint16INSTANCE.Write(writer, variant_value.Value)
	case PrimitiveU32:
		writeInt32(writer, 8)
		FfiConverterUint32INSTANCE.Write(writer, variant_value.Value)
	case PrimitiveU64:
		writeInt32(writer, 9)
		FfiConverterUint64INSTANCE.Write(writer, variant_value.Value)
	case PrimitiveU128:
		writeInt32(writer, 10)
		FfiConverterSequenceUint8INSTANCE.Write(writer, variant_value.Value)
	case PrimitiveU256:
		writeInt32(writer, 11)
		FfiConverterTypeU256INSTANCE.Write(writer, variant_value.Value)
	case PrimitiveBool:
		writeInt32(writer, 12)
		FfiConverterBoolINSTANCE.Write(writer, variant_value.Value)
	case PrimitiveFelt252:
		writeInt32(writer, 13)
		FfiConverterTypeFieldElementINSTANCE.Write(writer, variant_value.Value)
	case PrimitiveClassHash:
		writeInt32(writer, 14)
		FfiConverterTypeFieldElementINSTANCE.Write(writer, variant_value.Value)
	case PrimitiveContractAddress:
		writeInt32(writer, 15)
		FfiConverterTypeFieldElementINSTANCE.Write(writer, variant_value.Value)
	case PrimitiveEthAddress:
		writeInt32(writer, 16)
		FfiConverterTypeFieldElementINSTANCE.Write(writer, variant_value.Value)
	default:
		_ = variant_value
		panic(fmt.Sprintf("invalid enum value `%v` in FfiConverterPrimitive.Write", value))
	}
}

type FfiDestroyerPrimitive struct{}

func (_ FfiDestroyerPrimitive) Destroy(value Primitive) {
	value.Destroy()
}

type SqlValue interface {
	Destroy()
}
type SqlValueText struct {
	Value string
}

func (e SqlValueText) Destroy() {
	FfiDestroyerString{}.Destroy(e.Value)
}

type SqlValueInteger struct {
	Value int64
}

func (e SqlValueInteger) Destroy() {
	FfiDestroyerInt64{}.Destroy(e.Value)
}

type SqlValueReal struct {
	Value float64
}

func (e SqlValueReal) Destroy() {
	FfiDestroyerFloat64{}.Destroy(e.Value)
}

type SqlValueBlob struct {
	Value []uint8
}

func (e SqlValueBlob) Destroy() {
	FfiDestroyerSequenceUint8{}.Destroy(e.Value)
}

type SqlValueNull struct {
}

func (e SqlValueNull) Destroy() {
}

type FfiConverterSqlValue struct{}

var FfiConverterSqlValueINSTANCE = FfiConverterSqlValue{}

func (c FfiConverterSqlValue) Lift(rb RustBufferI) SqlValue {
	return LiftFromRustBuffer[SqlValue](c, rb)
}

func (c FfiConverterSqlValue) Lower(value SqlValue) C.RustBuffer {
	return LowerIntoRustBuffer[SqlValue](c, value)
}
func (FfiConverterSqlValue) Read(reader io.Reader) SqlValue {
	id := readInt32(reader)
	switch id {
	case 1:
		return SqlValueText{
			FfiConverterStringINSTANCE.Read(reader),
		}
	case 2:
		return SqlValueInteger{
			FfiConverterInt64INSTANCE.Read(reader),
		}
	case 3:
		return SqlValueReal{
			FfiConverterFloat64INSTANCE.Read(reader),
		}
	case 4:
		return SqlValueBlob{
			FfiConverterSequenceUint8INSTANCE.Read(reader),
		}
	case 5:
		return SqlValueNull{}
	default:
		panic(fmt.Sprintf("invalid enum value %v in FfiConverterSqlValue.Read()", id))
	}
}

func (FfiConverterSqlValue) Write(writer io.Writer, value SqlValue) {
	switch variant_value := value.(type) {
	case SqlValueText:
		writeInt32(writer, 1)
		FfiConverterStringINSTANCE.Write(writer, variant_value.Value)
	case SqlValueInteger:
		writeInt32(writer, 2)
		FfiConverterInt64INSTANCE.Write(writer, variant_value.Value)
	case SqlValueReal:
		writeInt32(writer, 3)
		FfiConverterFloat64INSTANCE.Write(writer, variant_value.Value)
	case SqlValueBlob:
		writeInt32(writer, 4)
		FfiConverterSequenceUint8INSTANCE.Write(writer, variant_value.Value)
	case SqlValueNull:
		writeInt32(writer, 5)
	default:
		_ = variant_value
		panic(fmt.Sprintf("invalid enum value `%v` in FfiConverterSqlValue.Write", value))
	}
}

type FfiDestroyerSqlValue struct{}

func (_ FfiDestroyerSqlValue) Destroy(value SqlValue) {
	value.Destroy()
}

type Ty interface {
	Destroy()
}
type TyPrimitive struct {
	Value Primitive
}

func (e TyPrimitive) Destroy() {
	FfiDestroyerPrimitive{}.Destroy(e.Value)
}

type TyStruct struct {
	Value Struct
}

func (e TyStruct) Destroy() {
	FfiDestroyerStruct{}.Destroy(e.Value)
}

type TyEnum struct {
	Value EnumType
}

func (e TyEnum) Destroy() {
	FfiDestroyerEnumType{}.Destroy(e.Value)
}

type TyTuple struct {
	Values []Ty
}

func (e TyTuple) Destroy() {
	FfiDestroyerSequenceTy{}.Destroy(e.Values)
}

type TyArray struct {
	Values []Ty
}

func (e TyArray) Destroy() {
	FfiDestroyerSequenceTy{}.Destroy(e.Values)
}

type TyFixedSizeArray struct {
	Value FixedSizeArray
}

func (e TyFixedSizeArray) Destroy() {
	FfiDestroyerFixedSizeArray{}.Destroy(e.Value)
}

type TyByteArray struct {
	Value string
}

func (e TyByteArray) Destroy() {
	FfiDestroyerString{}.Destroy(e.Value)
}

type FfiConverterTy struct{}

var FfiConverterTyINSTANCE = FfiConverterTy{}

func (c FfiConverterTy) Lift(rb RustBufferI) Ty {
	return LiftFromRustBuffer[Ty](c, rb)
}

func (c FfiConverterTy) Lower(value Ty) C.RustBuffer {
	return LowerIntoRustBuffer[Ty](c, value)
}
func (FfiConverterTy) Read(reader io.Reader) Ty {
	id := readInt32(reader)
	switch id {
	case 1:
		return TyPrimitive{
			FfiConverterPrimitiveINSTANCE.Read(reader),
		}
	case 2:
		return TyStruct{
			FfiConverterStructINSTANCE.Read(reader),
		}
	case 3:
		return TyEnum{
			FfiConverterEnumTypeINSTANCE.Read(reader),
		}
	case 4:
		return TyTuple{
			FfiConverterSequenceTyINSTANCE.Read(reader),
		}
	case 5:
		return TyArray{
			FfiConverterSequenceTyINSTANCE.Read(reader),
		}
	case 6:
		return TyFixedSizeArray{
			FfiConverterFixedSizeArrayINSTANCE.Read(reader),
		}
	case 7:
		return TyByteArray{
			FfiConverterStringINSTANCE.Read(reader),
		}
	default:
		panic(fmt.Sprintf("invalid enum value %v in FfiConverterTy.Read()", id))
	}
}

func (FfiConverterTy) Write(writer io.Writer, value Ty) {
	switch variant_value := value.(type) {
	case TyPrimitive:
		writeInt32(writer, 1)
		FfiConverterPrimitiveINSTANCE.Write(writer, variant_value.Value)
	case TyStruct:
		writeInt32(writer, 2)
		FfiConverterStructINSTANCE.Write(writer, variant_value.Value)
	case TyEnum:
		writeInt32(writer, 3)
		FfiConverterEnumTypeINSTANCE.Write(writer, variant_value.Value)
	case TyTuple:
		writeInt32(writer, 4)
		FfiConverterSequenceTyINSTANCE.Write(writer, variant_value.Values)
	case TyArray:
		writeInt32(writer, 5)
		FfiConverterSequenceTyINSTANCE.Write(writer, variant_value.Values)
	case TyFixedSizeArray:
		writeInt32(writer, 6)
		FfiConverterFixedSizeArrayINSTANCE.Write(writer, variant_value.Value)
	case TyByteArray:
		writeInt32(writer, 7)
		FfiConverterStringINSTANCE.Write(writer, variant_value.Value)
	default:
		_ = variant_value
		panic(fmt.Sprintf("invalid enum value `%v` in FfiConverterTy.Write", value))
	}
}

type FfiDestroyerTy struct{}

func (_ FfiDestroyerTy) Destroy(value Ty) {
	value.Destroy()
}

type ValueType interface {
	Destroy()
}
type ValueTypeString struct {
	Value string
}

func (e ValueTypeString) Destroy() {
	FfiDestroyerString{}.Destroy(e.Value)
}

type ValueTypeInt struct {
	Value int64
}

func (e ValueTypeInt) Destroy() {
	FfiDestroyerInt64{}.Destroy(e.Value)
}

type ValueTypeUInt struct {
	Value uint64
}

func (e ValueTypeUInt) Destroy() {
	FfiDestroyerUint64{}.Destroy(e.Value)
}

type ValueTypeBool struct {
	Value bool
}

func (e ValueTypeBool) Destroy() {
	FfiDestroyerBool{}.Destroy(e.Value)
}

type ValueTypeBytes struct {
	Value []uint8
}

func (e ValueTypeBytes) Destroy() {
	FfiDestroyerSequenceUint8{}.Destroy(e.Value)
}

type FfiConverterValueType struct{}

var FfiConverterValueTypeINSTANCE = FfiConverterValueType{}

func (c FfiConverterValueType) Lift(rb RustBufferI) ValueType {
	return LiftFromRustBuffer[ValueType](c, rb)
}

func (c FfiConverterValueType) Lower(value ValueType) C.RustBuffer {
	return LowerIntoRustBuffer[ValueType](c, value)
}
func (FfiConverterValueType) Read(reader io.Reader) ValueType {
	id := readInt32(reader)
	switch id {
	case 1:
		return ValueTypeString{
			FfiConverterStringINSTANCE.Read(reader),
		}
	case 2:
		return ValueTypeInt{
			FfiConverterInt64INSTANCE.Read(reader),
		}
	case 3:
		return ValueTypeUInt{
			FfiConverterUint64INSTANCE.Read(reader),
		}
	case 4:
		return ValueTypeBool{
			FfiConverterBoolINSTANCE.Read(reader),
		}
	case 5:
		return ValueTypeBytes{
			FfiConverterSequenceUint8INSTANCE.Read(reader),
		}
	default:
		panic(fmt.Sprintf("invalid enum value %v in FfiConverterValueType.Read()", id))
	}
}

func (FfiConverterValueType) Write(writer io.Writer, value ValueType) {
	switch variant_value := value.(type) {
	case ValueTypeString:
		writeInt32(writer, 1)
		FfiConverterStringINSTANCE.Write(writer, variant_value.Value)
	case ValueTypeInt:
		writeInt32(writer, 2)
		FfiConverterInt64INSTANCE.Write(writer, variant_value.Value)
	case ValueTypeUInt:
		writeInt32(writer, 3)
		FfiConverterUint64INSTANCE.Write(writer, variant_value.Value)
	case ValueTypeBool:
		writeInt32(writer, 4)
		FfiConverterBoolINSTANCE.Write(writer, variant_value.Value)
	case ValueTypeBytes:
		writeInt32(writer, 5)
		FfiConverterSequenceUint8INSTANCE.Write(writer, variant_value.Value)
	default:
		_ = variant_value
		panic(fmt.Sprintf("invalid enum value `%v` in FfiConverterValueType.Write", value))
	}
}

type FfiDestroyerValueType struct{}

func (_ FfiDestroyerValueType) Destroy(value ValueType) {
	value.Destroy()
}

type EntityUpdateCallback interface {
	OnUpdate(entity Entity)

	OnError(error string)
}

type FfiConverterCallbackInterfaceEntityUpdateCallback struct {
	handleMap *concurrentHandleMap[EntityUpdateCallback]
}

var FfiConverterCallbackInterfaceEntityUpdateCallbackINSTANCE = FfiConverterCallbackInterfaceEntityUpdateCallback{
	handleMap: newConcurrentHandleMap[EntityUpdateCallback](),
}

func (c FfiConverterCallbackInterfaceEntityUpdateCallback) Lift(handle uint64) EntityUpdateCallback {
	val, ok := c.handleMap.tryGet(handle)
	if !ok {
		panic(fmt.Errorf("no callback in handle map: %d", handle))
	}
	return val
}

func (c FfiConverterCallbackInterfaceEntityUpdateCallback) Read(reader io.Reader) EntityUpdateCallback {
	return c.Lift(readUint64(reader))
}

func (c FfiConverterCallbackInterfaceEntityUpdateCallback) Lower(value EntityUpdateCallback) C.uint64_t {
	return C.uint64_t(c.handleMap.insert(value))
}

func (c FfiConverterCallbackInterfaceEntityUpdateCallback) Write(writer io.Writer, value EntityUpdateCallback) {
	writeUint64(writer, uint64(c.Lower(value)))
}

type FfiDestroyerCallbackInterfaceEntityUpdateCallback struct{}

func (FfiDestroyerCallbackInterfaceEntityUpdateCallback) Destroy(value EntityUpdateCallback) {}

type uniffiCallbackResult C.int8_t

const (
	uniffiIdxCallbackFree               uniffiCallbackResult = 0
	uniffiCallbackResultSuccess         uniffiCallbackResult = 0
	uniffiCallbackResultError           uniffiCallbackResult = 1
	uniffiCallbackUnexpectedResultError uniffiCallbackResult = 2
	uniffiCallbackCancelled             uniffiCallbackResult = 3
)

type concurrentHandleMap[T any] struct {
	handles       map[uint64]T
	currentHandle uint64
	lock          sync.RWMutex
}

func newConcurrentHandleMap[T any]() *concurrentHandleMap[T] {
	return &concurrentHandleMap[T]{
		handles: map[uint64]T{},
	}
}

func (cm *concurrentHandleMap[T]) insert(obj T) uint64 {
	cm.lock.Lock()
	defer cm.lock.Unlock()

	cm.currentHandle = cm.currentHandle + 1
	cm.handles[cm.currentHandle] = obj
	return cm.currentHandle
}

func (cm *concurrentHandleMap[T]) remove(handle uint64) {
	cm.lock.Lock()
	defer cm.lock.Unlock()

	delete(cm.handles, handle)
}

func (cm *concurrentHandleMap[T]) tryGet(handle uint64) (T, bool) {
	cm.lock.RLock()
	defer cm.lock.RUnlock()

	val, ok := cm.handles[handle]
	return val, ok
}

//export dojo_uniffi_cgo_dispatchCallbackInterfaceEntityUpdateCallbackMethod0
func dojo_uniffi_cgo_dispatchCallbackInterfaceEntityUpdateCallbackMethod0(uniffiHandle C.uint64_t, entity C.RustBuffer, uniffiOutReturn *C.void, callStatus *C.RustCallStatus) {
	handle := uint64(uniffiHandle)
	uniffiObj, ok := FfiConverterCallbackInterfaceEntityUpdateCallbackINSTANCE.handleMap.tryGet(handle)
	if !ok {
		panic(fmt.Errorf("no callback in handle map: %d", handle))
	}

	uniffiObj.OnUpdate(
		FfiConverterEntityINSTANCE.Lift(GoRustBuffer{
			inner: entity,
		}),
	)

}

//export dojo_uniffi_cgo_dispatchCallbackInterfaceEntityUpdateCallbackMethod1
func dojo_uniffi_cgo_dispatchCallbackInterfaceEntityUpdateCallbackMethod1(uniffiHandle C.uint64_t, error C.RustBuffer, uniffiOutReturn *C.void, callStatus *C.RustCallStatus) {
	handle := uint64(uniffiHandle)
	uniffiObj, ok := FfiConverterCallbackInterfaceEntityUpdateCallbackINSTANCE.handleMap.tryGet(handle)
	if !ok {
		panic(fmt.Errorf("no callback in handle map: %d", handle))
	}

	uniffiObj.OnError(
		FfiConverterStringINSTANCE.Lift(GoRustBuffer{
			inner: error,
		}),
	)

}

var UniffiVTableCallbackInterfaceEntityUpdateCallbackINSTANCE = C.UniffiVTableCallbackInterfaceEntityUpdateCallback{
	onUpdate: (C.UniffiCallbackInterfaceEntityUpdateCallbackMethod0)(C.dojo_uniffi_cgo_dispatchCallbackInterfaceEntityUpdateCallbackMethod0),
	onError:  (C.UniffiCallbackInterfaceEntityUpdateCallbackMethod1)(C.dojo_uniffi_cgo_dispatchCallbackInterfaceEntityUpdateCallbackMethod1),

	uniffiFree: (C.UniffiCallbackInterfaceFree)(C.dojo_uniffi_cgo_dispatchCallbackInterfaceEntityUpdateCallbackFree),
}

//export dojo_uniffi_cgo_dispatchCallbackInterfaceEntityUpdateCallbackFree
func dojo_uniffi_cgo_dispatchCallbackInterfaceEntityUpdateCallbackFree(handle C.uint64_t) {
	FfiConverterCallbackInterfaceEntityUpdateCallbackINSTANCE.handleMap.remove(uint64(handle))
}

func (c FfiConverterCallbackInterfaceEntityUpdateCallback) register() {
	C.uniffi_dojo_uniffi_fn_init_callback_vtable_entityupdatecallback(&UniffiVTableCallbackInterfaceEntityUpdateCallbackINSTANCE)
}

type EventUpdateCallback interface {
	OnUpdate(event Event)

	OnError(error string)
}

type FfiConverterCallbackInterfaceEventUpdateCallback struct {
	handleMap *concurrentHandleMap[EventUpdateCallback]
}

var FfiConverterCallbackInterfaceEventUpdateCallbackINSTANCE = FfiConverterCallbackInterfaceEventUpdateCallback{
	handleMap: newConcurrentHandleMap[EventUpdateCallback](),
}

func (c FfiConverterCallbackInterfaceEventUpdateCallback) Lift(handle uint64) EventUpdateCallback {
	val, ok := c.handleMap.tryGet(handle)
	if !ok {
		panic(fmt.Errorf("no callback in handle map: %d", handle))
	}
	return val
}

func (c FfiConverterCallbackInterfaceEventUpdateCallback) Read(reader io.Reader) EventUpdateCallback {
	return c.Lift(readUint64(reader))
}

func (c FfiConverterCallbackInterfaceEventUpdateCallback) Lower(value EventUpdateCallback) C.uint64_t {
	return C.uint64_t(c.handleMap.insert(value))
}

func (c FfiConverterCallbackInterfaceEventUpdateCallback) Write(writer io.Writer, value EventUpdateCallback) {
	writeUint64(writer, uint64(c.Lower(value)))
}

type FfiDestroyerCallbackInterfaceEventUpdateCallback struct{}

func (FfiDestroyerCallbackInterfaceEventUpdateCallback) Destroy(value EventUpdateCallback) {}

//export dojo_uniffi_cgo_dispatchCallbackInterfaceEventUpdateCallbackMethod0
func dojo_uniffi_cgo_dispatchCallbackInterfaceEventUpdateCallbackMethod0(uniffiHandle C.uint64_t, event C.RustBuffer, uniffiOutReturn *C.void, callStatus *C.RustCallStatus) {
	handle := uint64(uniffiHandle)
	uniffiObj, ok := FfiConverterCallbackInterfaceEventUpdateCallbackINSTANCE.handleMap.tryGet(handle)
	if !ok {
		panic(fmt.Errorf("no callback in handle map: %d", handle))
	}

	uniffiObj.OnUpdate(
		FfiConverterEventINSTANCE.Lift(GoRustBuffer{
			inner: event,
		}),
	)

}

//export dojo_uniffi_cgo_dispatchCallbackInterfaceEventUpdateCallbackMethod1
func dojo_uniffi_cgo_dispatchCallbackInterfaceEventUpdateCallbackMethod1(uniffiHandle C.uint64_t, error C.RustBuffer, uniffiOutReturn *C.void, callStatus *C.RustCallStatus) {
	handle := uint64(uniffiHandle)
	uniffiObj, ok := FfiConverterCallbackInterfaceEventUpdateCallbackINSTANCE.handleMap.tryGet(handle)
	if !ok {
		panic(fmt.Errorf("no callback in handle map: %d", handle))
	}

	uniffiObj.OnError(
		FfiConverterStringINSTANCE.Lift(GoRustBuffer{
			inner: error,
		}),
	)

}

var UniffiVTableCallbackInterfaceEventUpdateCallbackINSTANCE = C.UniffiVTableCallbackInterfaceEventUpdateCallback{
	onUpdate: (C.UniffiCallbackInterfaceEventUpdateCallbackMethod0)(C.dojo_uniffi_cgo_dispatchCallbackInterfaceEventUpdateCallbackMethod0),
	onError:  (C.UniffiCallbackInterfaceEventUpdateCallbackMethod1)(C.dojo_uniffi_cgo_dispatchCallbackInterfaceEventUpdateCallbackMethod1),

	uniffiFree: (C.UniffiCallbackInterfaceFree)(C.dojo_uniffi_cgo_dispatchCallbackInterfaceEventUpdateCallbackFree),
}

//export dojo_uniffi_cgo_dispatchCallbackInterfaceEventUpdateCallbackFree
func dojo_uniffi_cgo_dispatchCallbackInterfaceEventUpdateCallbackFree(handle C.uint64_t) {
	FfiConverterCallbackInterfaceEventUpdateCallbackINSTANCE.handleMap.remove(uint64(handle))
}

func (c FfiConverterCallbackInterfaceEventUpdateCallback) register() {
	C.uniffi_dojo_uniffi_fn_init_callback_vtable_eventupdatecallback(&UniffiVTableCallbackInterfaceEventUpdateCallbackINSTANCE)
}

type TokenBalanceUpdateCallback interface {
	OnUpdate(balance TokenBalance)

	OnError(error string)
}

type FfiConverterCallbackInterfaceTokenBalanceUpdateCallback struct {
	handleMap *concurrentHandleMap[TokenBalanceUpdateCallback]
}

var FfiConverterCallbackInterfaceTokenBalanceUpdateCallbackINSTANCE = FfiConverterCallbackInterfaceTokenBalanceUpdateCallback{
	handleMap: newConcurrentHandleMap[TokenBalanceUpdateCallback](),
}

func (c FfiConverterCallbackInterfaceTokenBalanceUpdateCallback) Lift(handle uint64) TokenBalanceUpdateCallback {
	val, ok := c.handleMap.tryGet(handle)
	if !ok {
		panic(fmt.Errorf("no callback in handle map: %d", handle))
	}
	return val
}

func (c FfiConverterCallbackInterfaceTokenBalanceUpdateCallback) Read(reader io.Reader) TokenBalanceUpdateCallback {
	return c.Lift(readUint64(reader))
}

func (c FfiConverterCallbackInterfaceTokenBalanceUpdateCallback) Lower(value TokenBalanceUpdateCallback) C.uint64_t {
	return C.uint64_t(c.handleMap.insert(value))
}

func (c FfiConverterCallbackInterfaceTokenBalanceUpdateCallback) Write(writer io.Writer, value TokenBalanceUpdateCallback) {
	writeUint64(writer, uint64(c.Lower(value)))
}

type FfiDestroyerCallbackInterfaceTokenBalanceUpdateCallback struct{}

func (FfiDestroyerCallbackInterfaceTokenBalanceUpdateCallback) Destroy(value TokenBalanceUpdateCallback) {
}

//export dojo_uniffi_cgo_dispatchCallbackInterfaceTokenBalanceUpdateCallbackMethod0
func dojo_uniffi_cgo_dispatchCallbackInterfaceTokenBalanceUpdateCallbackMethod0(uniffiHandle C.uint64_t, balance C.RustBuffer, uniffiOutReturn *C.void, callStatus *C.RustCallStatus) {
	handle := uint64(uniffiHandle)
	uniffiObj, ok := FfiConverterCallbackInterfaceTokenBalanceUpdateCallbackINSTANCE.handleMap.tryGet(handle)
	if !ok {
		panic(fmt.Errorf("no callback in handle map: %d", handle))
	}

	uniffiObj.OnUpdate(
		FfiConverterTokenBalanceINSTANCE.Lift(GoRustBuffer{
			inner: balance,
		}),
	)

}

//export dojo_uniffi_cgo_dispatchCallbackInterfaceTokenBalanceUpdateCallbackMethod1
func dojo_uniffi_cgo_dispatchCallbackInterfaceTokenBalanceUpdateCallbackMethod1(uniffiHandle C.uint64_t, error C.RustBuffer, uniffiOutReturn *C.void, callStatus *C.RustCallStatus) {
	handle := uint64(uniffiHandle)
	uniffiObj, ok := FfiConverterCallbackInterfaceTokenBalanceUpdateCallbackINSTANCE.handleMap.tryGet(handle)
	if !ok {
		panic(fmt.Errorf("no callback in handle map: %d", handle))
	}

	uniffiObj.OnError(
		FfiConverterStringINSTANCE.Lift(GoRustBuffer{
			inner: error,
		}),
	)

}

var UniffiVTableCallbackInterfaceTokenBalanceUpdateCallbackINSTANCE = C.UniffiVTableCallbackInterfaceTokenBalanceUpdateCallback{
	onUpdate: (C.UniffiCallbackInterfaceTokenBalanceUpdateCallbackMethod0)(C.dojo_uniffi_cgo_dispatchCallbackInterfaceTokenBalanceUpdateCallbackMethod0),
	onError:  (C.UniffiCallbackInterfaceTokenBalanceUpdateCallbackMethod1)(C.dojo_uniffi_cgo_dispatchCallbackInterfaceTokenBalanceUpdateCallbackMethod1),

	uniffiFree: (C.UniffiCallbackInterfaceFree)(C.dojo_uniffi_cgo_dispatchCallbackInterfaceTokenBalanceUpdateCallbackFree),
}

//export dojo_uniffi_cgo_dispatchCallbackInterfaceTokenBalanceUpdateCallbackFree
func dojo_uniffi_cgo_dispatchCallbackInterfaceTokenBalanceUpdateCallbackFree(handle C.uint64_t) {
	FfiConverterCallbackInterfaceTokenBalanceUpdateCallbackINSTANCE.handleMap.remove(uint64(handle))
}

func (c FfiConverterCallbackInterfaceTokenBalanceUpdateCallback) register() {
	C.uniffi_dojo_uniffi_fn_init_callback_vtable_tokenbalanceupdatecallback(&UniffiVTableCallbackInterfaceTokenBalanceUpdateCallbackINSTANCE)
}

type TokenUpdateCallback interface {
	OnUpdate(token Token)

	OnError(error string)
}

type FfiConverterCallbackInterfaceTokenUpdateCallback struct {
	handleMap *concurrentHandleMap[TokenUpdateCallback]
}

var FfiConverterCallbackInterfaceTokenUpdateCallbackINSTANCE = FfiConverterCallbackInterfaceTokenUpdateCallback{
	handleMap: newConcurrentHandleMap[TokenUpdateCallback](),
}

func (c FfiConverterCallbackInterfaceTokenUpdateCallback) Lift(handle uint64) TokenUpdateCallback {
	val, ok := c.handleMap.tryGet(handle)
	if !ok {
		panic(fmt.Errorf("no callback in handle map: %d", handle))
	}
	return val
}

func (c FfiConverterCallbackInterfaceTokenUpdateCallback) Read(reader io.Reader) TokenUpdateCallback {
	return c.Lift(readUint64(reader))
}

func (c FfiConverterCallbackInterfaceTokenUpdateCallback) Lower(value TokenUpdateCallback) C.uint64_t {
	return C.uint64_t(c.handleMap.insert(value))
}

func (c FfiConverterCallbackInterfaceTokenUpdateCallback) Write(writer io.Writer, value TokenUpdateCallback) {
	writeUint64(writer, uint64(c.Lower(value)))
}

type FfiDestroyerCallbackInterfaceTokenUpdateCallback struct{}

func (FfiDestroyerCallbackInterfaceTokenUpdateCallback) Destroy(value TokenUpdateCallback) {}

//export dojo_uniffi_cgo_dispatchCallbackInterfaceTokenUpdateCallbackMethod0
func dojo_uniffi_cgo_dispatchCallbackInterfaceTokenUpdateCallbackMethod0(uniffiHandle C.uint64_t, token C.RustBuffer, uniffiOutReturn *C.void, callStatus *C.RustCallStatus) {
	handle := uint64(uniffiHandle)
	uniffiObj, ok := FfiConverterCallbackInterfaceTokenUpdateCallbackINSTANCE.handleMap.tryGet(handle)
	if !ok {
		panic(fmt.Errorf("no callback in handle map: %d", handle))
	}

	uniffiObj.OnUpdate(
		FfiConverterTokenINSTANCE.Lift(GoRustBuffer{
			inner: token,
		}),
	)

}

//export dojo_uniffi_cgo_dispatchCallbackInterfaceTokenUpdateCallbackMethod1
func dojo_uniffi_cgo_dispatchCallbackInterfaceTokenUpdateCallbackMethod1(uniffiHandle C.uint64_t, error C.RustBuffer, uniffiOutReturn *C.void, callStatus *C.RustCallStatus) {
	handle := uint64(uniffiHandle)
	uniffiObj, ok := FfiConverterCallbackInterfaceTokenUpdateCallbackINSTANCE.handleMap.tryGet(handle)
	if !ok {
		panic(fmt.Errorf("no callback in handle map: %d", handle))
	}

	uniffiObj.OnError(
		FfiConverterStringINSTANCE.Lift(GoRustBuffer{
			inner: error,
		}),
	)

}

var UniffiVTableCallbackInterfaceTokenUpdateCallbackINSTANCE = C.UniffiVTableCallbackInterfaceTokenUpdateCallback{
	onUpdate: (C.UniffiCallbackInterfaceTokenUpdateCallbackMethod0)(C.dojo_uniffi_cgo_dispatchCallbackInterfaceTokenUpdateCallbackMethod0),
	onError:  (C.UniffiCallbackInterfaceTokenUpdateCallbackMethod1)(C.dojo_uniffi_cgo_dispatchCallbackInterfaceTokenUpdateCallbackMethod1),

	uniffiFree: (C.UniffiCallbackInterfaceFree)(C.dojo_uniffi_cgo_dispatchCallbackInterfaceTokenUpdateCallbackFree),
}

//export dojo_uniffi_cgo_dispatchCallbackInterfaceTokenUpdateCallbackFree
func dojo_uniffi_cgo_dispatchCallbackInterfaceTokenUpdateCallbackFree(handle C.uint64_t) {
	FfiConverterCallbackInterfaceTokenUpdateCallbackINSTANCE.handleMap.remove(uint64(handle))
}

func (c FfiConverterCallbackInterfaceTokenUpdateCallback) register() {
	C.uniffi_dojo_uniffi_fn_init_callback_vtable_tokenupdatecallback(&UniffiVTableCallbackInterfaceTokenUpdateCallbackINSTANCE)
}

type TransactionUpdateCallback interface {
	OnUpdate(transaction Transaction)

	OnError(error string)
}

type FfiConverterCallbackInterfaceTransactionUpdateCallback struct {
	handleMap *concurrentHandleMap[TransactionUpdateCallback]
}

var FfiConverterCallbackInterfaceTransactionUpdateCallbackINSTANCE = FfiConverterCallbackInterfaceTransactionUpdateCallback{
	handleMap: newConcurrentHandleMap[TransactionUpdateCallback](),
}

func (c FfiConverterCallbackInterfaceTransactionUpdateCallback) Lift(handle uint64) TransactionUpdateCallback {
	val, ok := c.handleMap.tryGet(handle)
	if !ok {
		panic(fmt.Errorf("no callback in handle map: %d", handle))
	}
	return val
}

func (c FfiConverterCallbackInterfaceTransactionUpdateCallback) Read(reader io.Reader) TransactionUpdateCallback {
	return c.Lift(readUint64(reader))
}

func (c FfiConverterCallbackInterfaceTransactionUpdateCallback) Lower(value TransactionUpdateCallback) C.uint64_t {
	return C.uint64_t(c.handleMap.insert(value))
}

func (c FfiConverterCallbackInterfaceTransactionUpdateCallback) Write(writer io.Writer, value TransactionUpdateCallback) {
	writeUint64(writer, uint64(c.Lower(value)))
}

type FfiDestroyerCallbackInterfaceTransactionUpdateCallback struct{}

func (FfiDestroyerCallbackInterfaceTransactionUpdateCallback) Destroy(value TransactionUpdateCallback) {
}

//export dojo_uniffi_cgo_dispatchCallbackInterfaceTransactionUpdateCallbackMethod0
func dojo_uniffi_cgo_dispatchCallbackInterfaceTransactionUpdateCallbackMethod0(uniffiHandle C.uint64_t, transaction C.RustBuffer, uniffiOutReturn *C.void, callStatus *C.RustCallStatus) {
	handle := uint64(uniffiHandle)
	uniffiObj, ok := FfiConverterCallbackInterfaceTransactionUpdateCallbackINSTANCE.handleMap.tryGet(handle)
	if !ok {
		panic(fmt.Errorf("no callback in handle map: %d", handle))
	}

	uniffiObj.OnUpdate(
		FfiConverterTransactionINSTANCE.Lift(GoRustBuffer{
			inner: transaction,
		}),
	)

}

//export dojo_uniffi_cgo_dispatchCallbackInterfaceTransactionUpdateCallbackMethod1
func dojo_uniffi_cgo_dispatchCallbackInterfaceTransactionUpdateCallbackMethod1(uniffiHandle C.uint64_t, error C.RustBuffer, uniffiOutReturn *C.void, callStatus *C.RustCallStatus) {
	handle := uint64(uniffiHandle)
	uniffiObj, ok := FfiConverterCallbackInterfaceTransactionUpdateCallbackINSTANCE.handleMap.tryGet(handle)
	if !ok {
		panic(fmt.Errorf("no callback in handle map: %d", handle))
	}

	uniffiObj.OnError(
		FfiConverterStringINSTANCE.Lift(GoRustBuffer{
			inner: error,
		}),
	)

}

var UniffiVTableCallbackInterfaceTransactionUpdateCallbackINSTANCE = C.UniffiVTableCallbackInterfaceTransactionUpdateCallback{
	onUpdate: (C.UniffiCallbackInterfaceTransactionUpdateCallbackMethod0)(C.dojo_uniffi_cgo_dispatchCallbackInterfaceTransactionUpdateCallbackMethod0),
	onError:  (C.UniffiCallbackInterfaceTransactionUpdateCallbackMethod1)(C.dojo_uniffi_cgo_dispatchCallbackInterfaceTransactionUpdateCallbackMethod1),

	uniffiFree: (C.UniffiCallbackInterfaceFree)(C.dojo_uniffi_cgo_dispatchCallbackInterfaceTransactionUpdateCallbackFree),
}

//export dojo_uniffi_cgo_dispatchCallbackInterfaceTransactionUpdateCallbackFree
func dojo_uniffi_cgo_dispatchCallbackInterfaceTransactionUpdateCallbackFree(handle C.uint64_t) {
	FfiConverterCallbackInterfaceTransactionUpdateCallbackINSTANCE.handleMap.remove(uint64(handle))
}

func (c FfiConverterCallbackInterfaceTransactionUpdateCallback) register() {
	C.uniffi_dojo_uniffi_fn_init_callback_vtable_transactionupdatecallback(&UniffiVTableCallbackInterfaceTransactionUpdateCallbackINSTANCE)
}

type FfiConverterOptionalUint32 struct{}

var FfiConverterOptionalUint32INSTANCE = FfiConverterOptionalUint32{}

func (c FfiConverterOptionalUint32) Lift(rb RustBufferI) *uint32 {
	return LiftFromRustBuffer[*uint32](c, rb)
}

func (_ FfiConverterOptionalUint32) Read(reader io.Reader) *uint32 {
	if readInt8(reader) == 0 {
		return nil
	}
	temp := FfiConverterUint32INSTANCE.Read(reader)
	return &temp
}

func (c FfiConverterOptionalUint32) Lower(value *uint32) C.RustBuffer {
	return LowerIntoRustBuffer[*uint32](c, value)
}

func (_ FfiConverterOptionalUint32) Write(writer io.Writer, value *uint32) {
	if value == nil {
		writeInt8(writer, 0)
	} else {
		writeInt8(writer, 1)
		FfiConverterUint32INSTANCE.Write(writer, *value)
	}
}

type FfiDestroyerOptionalUint32 struct{}

func (_ FfiDestroyerOptionalUint32) Destroy(value *uint32) {
	if value != nil {
		FfiDestroyerUint32{}.Destroy(*value)
	}
}

type FfiConverterOptionalUint64 struct{}

var FfiConverterOptionalUint64INSTANCE = FfiConverterOptionalUint64{}

func (c FfiConverterOptionalUint64) Lift(rb RustBufferI) *uint64 {
	return LiftFromRustBuffer[*uint64](c, rb)
}

func (_ FfiConverterOptionalUint64) Read(reader io.Reader) *uint64 {
	if readInt8(reader) == 0 {
		return nil
	}
	temp := FfiConverterUint64INSTANCE.Read(reader)
	return &temp
}

func (c FfiConverterOptionalUint64) Lower(value *uint64) C.RustBuffer {
	return LowerIntoRustBuffer[*uint64](c, value)
}

func (_ FfiConverterOptionalUint64) Write(writer io.Writer, value *uint64) {
	if value == nil {
		writeInt8(writer, 0)
	} else {
		writeInt8(writer, 1)
		FfiConverterUint64INSTANCE.Write(writer, *value)
	}
}

type FfiDestroyerOptionalUint64 struct{}

func (_ FfiDestroyerOptionalUint64) Destroy(value *uint64) {
	if value != nil {
		FfiDestroyerUint64{}.Destroy(*value)
	}
}

type FfiConverterOptionalBool struct{}

var FfiConverterOptionalBoolINSTANCE = FfiConverterOptionalBool{}

func (c FfiConverterOptionalBool) Lift(rb RustBufferI) *bool {
	return LiftFromRustBuffer[*bool](c, rb)
}

func (_ FfiConverterOptionalBool) Read(reader io.Reader) *bool {
	if readInt8(reader) == 0 {
		return nil
	}
	temp := FfiConverterBoolINSTANCE.Read(reader)
	return &temp
}

func (c FfiConverterOptionalBool) Lower(value *bool) C.RustBuffer {
	return LowerIntoRustBuffer[*bool](c, value)
}

func (_ FfiConverterOptionalBool) Write(writer io.Writer, value *bool) {
	if value == nil {
		writeInt8(writer, 0)
	} else {
		writeInt8(writer, 1)
		FfiConverterBoolINSTANCE.Write(writer, *value)
	}
}

type FfiDestroyerOptionalBool struct{}

func (_ FfiDestroyerOptionalBool) Destroy(value *bool) {
	if value != nil {
		FfiDestroyerBool{}.Destroy(*value)
	}
}

type FfiConverterOptionalString struct{}

var FfiConverterOptionalStringINSTANCE = FfiConverterOptionalString{}

func (c FfiConverterOptionalString) Lift(rb RustBufferI) *string {
	return LiftFromRustBuffer[*string](c, rb)
}

func (_ FfiConverterOptionalString) Read(reader io.Reader) *string {
	if readInt8(reader) == 0 {
		return nil
	}
	temp := FfiConverterStringINSTANCE.Read(reader)
	return &temp
}

func (c FfiConverterOptionalString) Lower(value *string) C.RustBuffer {
	return LowerIntoRustBuffer[*string](c, value)
}

func (_ FfiConverterOptionalString) Write(writer io.Writer, value *string) {
	if value == nil {
		writeInt8(writer, 0)
	} else {
		writeInt8(writer, 1)
		FfiConverterStringINSTANCE.Write(writer, *value)
	}
}

type FfiDestroyerOptionalString struct{}

func (_ FfiDestroyerOptionalString) Destroy(value *string) {
	if value != nil {
		FfiDestroyerString{}.Destroy(*value)
	}
}

type FfiConverterOptionalKeysClause struct{}

var FfiConverterOptionalKeysClauseINSTANCE = FfiConverterOptionalKeysClause{}

func (c FfiConverterOptionalKeysClause) Lift(rb RustBufferI) *KeysClause {
	return LiftFromRustBuffer[*KeysClause](c, rb)
}

func (_ FfiConverterOptionalKeysClause) Read(reader io.Reader) *KeysClause {
	if readInt8(reader) == 0 {
		return nil
	}
	temp := FfiConverterKeysClauseINSTANCE.Read(reader)
	return &temp
}

func (c FfiConverterOptionalKeysClause) Lower(value *KeysClause) C.RustBuffer {
	return LowerIntoRustBuffer[*KeysClause](c, value)
}

func (_ FfiConverterOptionalKeysClause) Write(writer io.Writer, value *KeysClause) {
	if value == nil {
		writeInt8(writer, 0)
	} else {
		writeInt8(writer, 1)
		FfiConverterKeysClauseINSTANCE.Write(writer, *value)
	}
}

type FfiDestroyerOptionalKeysClause struct{}

func (_ FfiDestroyerOptionalKeysClause) Destroy(value *KeysClause) {
	if value != nil {
		FfiDestroyerKeysClause{}.Destroy(*value)
	}
}

type FfiConverterOptionalTransactionFilter struct{}

var FfiConverterOptionalTransactionFilterINSTANCE = FfiConverterOptionalTransactionFilter{}

func (c FfiConverterOptionalTransactionFilter) Lift(rb RustBufferI) *TransactionFilter {
	return LiftFromRustBuffer[*TransactionFilter](c, rb)
}

func (_ FfiConverterOptionalTransactionFilter) Read(reader io.Reader) *TransactionFilter {
	if readInt8(reader) == 0 {
		return nil
	}
	temp := FfiConverterTransactionFilterINSTANCE.Read(reader)
	return &temp
}

func (c FfiConverterOptionalTransactionFilter) Lower(value *TransactionFilter) C.RustBuffer {
	return LowerIntoRustBuffer[*TransactionFilter](c, value)
}

func (_ FfiConverterOptionalTransactionFilter) Write(writer io.Writer, value *TransactionFilter) {
	if value == nil {
		writeInt8(writer, 0)
	} else {
		writeInt8(writer, 1)
		FfiConverterTransactionFilterINSTANCE.Write(writer, *value)
	}
}

type FfiDestroyerOptionalTransactionFilter struct{}

func (_ FfiDestroyerOptionalTransactionFilter) Destroy(value *TransactionFilter) {
	if value != nil {
		FfiDestroyerTransactionFilter{}.Destroy(*value)
	}
}

type FfiConverterOptionalClause struct{}

var FfiConverterOptionalClauseINSTANCE = FfiConverterOptionalClause{}

func (c FfiConverterOptionalClause) Lift(rb RustBufferI) *Clause {
	return LiftFromRustBuffer[*Clause](c, rb)
}

func (_ FfiConverterOptionalClause) Read(reader io.Reader) *Clause {
	if readInt8(reader) == 0 {
		return nil
	}
	temp := FfiConverterClauseINSTANCE.Read(reader)
	return &temp
}

func (c FfiConverterOptionalClause) Lower(value *Clause) C.RustBuffer {
	return LowerIntoRustBuffer[*Clause](c, value)
}

func (_ FfiConverterOptionalClause) Write(writer io.Writer, value *Clause) {
	if value == nil {
		writeInt8(writer, 0)
	} else {
		writeInt8(writer, 1)
		FfiConverterClauseINSTANCE.Write(writer, *value)
	}
}

type FfiDestroyerOptionalClause struct{}

func (_ FfiDestroyerOptionalClause) Destroy(value *Clause) {
	if value != nil {
		FfiDestroyerClause{}.Destroy(*value)
	}
}

type FfiConverterOptionalTypeFieldElement struct{}

var FfiConverterOptionalTypeFieldElementINSTANCE = FfiConverterOptionalTypeFieldElement{}

func (c FfiConverterOptionalTypeFieldElement) Lift(rb RustBufferI) *FieldElement {
	return LiftFromRustBuffer[*FieldElement](c, rb)
}

func (_ FfiConverterOptionalTypeFieldElement) Read(reader io.Reader) *FieldElement {
	if readInt8(reader) == 0 {
		return nil
	}
	temp := FfiConverterTypeFieldElementINSTANCE.Read(reader)
	return &temp
}

func (c FfiConverterOptionalTypeFieldElement) Lower(value *FieldElement) C.RustBuffer {
	return LowerIntoRustBuffer[*FieldElement](c, value)
}

func (_ FfiConverterOptionalTypeFieldElement) Write(writer io.Writer, value *FieldElement) {
	if value == nil {
		writeInt8(writer, 0)
	} else {
		writeInt8(writer, 1)
		FfiConverterTypeFieldElementINSTANCE.Write(writer, *value)
	}
}

type FfiDestroyerOptionalTypeFieldElement struct{}

func (_ FfiDestroyerOptionalTypeFieldElement) Destroy(value *FieldElement) {
	if value != nil {
		FfiDestroyerTypeFieldElement{}.Destroy(*value)
	}
}

type FfiConverterOptionalTypeU256 struct{}

var FfiConverterOptionalTypeU256INSTANCE = FfiConverterOptionalTypeU256{}

func (c FfiConverterOptionalTypeU256) Lift(rb RustBufferI) *U256 {
	return LiftFromRustBuffer[*U256](c, rb)
}

func (_ FfiConverterOptionalTypeU256) Read(reader io.Reader) *U256 {
	if readInt8(reader) == 0 {
		return nil
	}
	temp := FfiConverterTypeU256INSTANCE.Read(reader)
	return &temp
}

func (c FfiConverterOptionalTypeU256) Lower(value *U256) C.RustBuffer {
	return LowerIntoRustBuffer[*U256](c, value)
}

func (_ FfiConverterOptionalTypeU256) Write(writer io.Writer, value *U256) {
	if value == nil {
		writeInt8(writer, 0)
	} else {
		writeInt8(writer, 1)
		FfiConverterTypeU256INSTANCE.Write(writer, *value)
	}
}

type FfiDestroyerOptionalTypeU256 struct{}

func (_ FfiDestroyerOptionalTypeU256) Destroy(value *U256) {
	if value != nil {
		FfiDestroyerTypeU256{}.Destroy(*value)
	}
}

type FfiConverterSequenceUint8 struct{}

var FfiConverterSequenceUint8INSTANCE = FfiConverterSequenceUint8{}

func (c FfiConverterSequenceUint8) Lift(rb RustBufferI) []uint8 {
	return LiftFromRustBuffer[[]uint8](c, rb)
}

func (c FfiConverterSequenceUint8) Read(reader io.Reader) []uint8 {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]uint8, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterUint8INSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceUint8) Lower(value []uint8) C.RustBuffer {
	return LowerIntoRustBuffer[[]uint8](c, value)
}

func (c FfiConverterSequenceUint8) Write(writer io.Writer, value []uint8) {
	if len(value) > math.MaxInt32 {
		panic("[]uint8 is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterUint8INSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceUint8 struct{}

func (FfiDestroyerSequenceUint8) Destroy(sequence []uint8) {
	for _, value := range sequence {
		FfiDestroyerUint8{}.Destroy(value)
	}
}

type FfiConverterSequenceString struct{}

var FfiConverterSequenceStringINSTANCE = FfiConverterSequenceString{}

func (c FfiConverterSequenceString) Lift(rb RustBufferI) []string {
	return LiftFromRustBuffer[[]string](c, rb)
}

func (c FfiConverterSequenceString) Read(reader io.Reader) []string {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]string, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterStringINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceString) Lower(value []string) C.RustBuffer {
	return LowerIntoRustBuffer[[]string](c, value)
}

func (c FfiConverterSequenceString) Write(writer io.Writer, value []string) {
	if len(value) > math.MaxInt32 {
		panic("[]string is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterStringINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceString struct{}

func (FfiDestroyerSequenceString) Destroy(sequence []string) {
	for _, value := range sequence {
		FfiDestroyerString{}.Destroy(value)
	}
}

type FfiConverterSequenceAchievement struct{}

var FfiConverterSequenceAchievementINSTANCE = FfiConverterSequenceAchievement{}

func (c FfiConverterSequenceAchievement) Lift(rb RustBufferI) []Achievement {
	return LiftFromRustBuffer[[]Achievement](c, rb)
}

func (c FfiConverterSequenceAchievement) Read(reader io.Reader) []Achievement {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]Achievement, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterAchievementINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceAchievement) Lower(value []Achievement) C.RustBuffer {
	return LowerIntoRustBuffer[[]Achievement](c, value)
}

func (c FfiConverterSequenceAchievement) Write(writer io.Writer, value []Achievement) {
	if len(value) > math.MaxInt32 {
		panic("[]Achievement is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterAchievementINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceAchievement struct{}

func (FfiDestroyerSequenceAchievement) Destroy(sequence []Achievement) {
	for _, value := range sequence {
		FfiDestroyerAchievement{}.Destroy(value)
	}
}

type FfiConverterSequenceAchievementTask struct{}

var FfiConverterSequenceAchievementTaskINSTANCE = FfiConverterSequenceAchievementTask{}

func (c FfiConverterSequenceAchievementTask) Lift(rb RustBufferI) []AchievementTask {
	return LiftFromRustBuffer[[]AchievementTask](c, rb)
}

func (c FfiConverterSequenceAchievementTask) Read(reader io.Reader) []AchievementTask {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]AchievementTask, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterAchievementTaskINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceAchievementTask) Lower(value []AchievementTask) C.RustBuffer {
	return LowerIntoRustBuffer[[]AchievementTask](c, value)
}

func (c FfiConverterSequenceAchievementTask) Write(writer io.Writer, value []AchievementTask) {
	if len(value) > math.MaxInt32 {
		panic("[]AchievementTask is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterAchievementTaskINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceAchievementTask struct{}

func (FfiDestroyerSequenceAchievementTask) Destroy(sequence []AchievementTask) {
	for _, value := range sequence {
		FfiDestroyerAchievementTask{}.Destroy(value)
	}
}

type FfiConverterSequenceActionCount struct{}

var FfiConverterSequenceActionCountINSTANCE = FfiConverterSequenceActionCount{}

func (c FfiConverterSequenceActionCount) Lift(rb RustBufferI) []ActionCount {
	return LiftFromRustBuffer[[]ActionCount](c, rb)
}

func (c FfiConverterSequenceActionCount) Read(reader io.Reader) []ActionCount {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]ActionCount, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterActionCountINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceActionCount) Lower(value []ActionCount) C.RustBuffer {
	return LowerIntoRustBuffer[[]ActionCount](c, value)
}

func (c FfiConverterSequenceActionCount) Write(writer io.Writer, value []ActionCount) {
	if len(value) > math.MaxInt32 {
		panic("[]ActionCount is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterActionCountINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceActionCount struct{}

func (FfiDestroyerSequenceActionCount) Destroy(sequence []ActionCount) {
	for _, value := range sequence {
		FfiDestroyerActionCount{}.Destroy(value)
	}
}

type FfiConverterSequenceActivity struct{}

var FfiConverterSequenceActivityINSTANCE = FfiConverterSequenceActivity{}

func (c FfiConverterSequenceActivity) Lift(rb RustBufferI) []Activity {
	return LiftFromRustBuffer[[]Activity](c, rb)
}

func (c FfiConverterSequenceActivity) Read(reader io.Reader) []Activity {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]Activity, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterActivityINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceActivity) Lower(value []Activity) C.RustBuffer {
	return LowerIntoRustBuffer[[]Activity](c, value)
}

func (c FfiConverterSequenceActivity) Write(writer io.Writer, value []Activity) {
	if len(value) > math.MaxInt32 {
		panic("[]Activity is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterActivityINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceActivity struct{}

func (FfiDestroyerSequenceActivity) Destroy(sequence []Activity) {
	for _, value := range sequence {
		FfiDestroyerActivity{}.Destroy(value)
	}
}

type FfiConverterSequenceAggregationEntry struct{}

var FfiConverterSequenceAggregationEntryINSTANCE = FfiConverterSequenceAggregationEntry{}

func (c FfiConverterSequenceAggregationEntry) Lift(rb RustBufferI) []AggregationEntry {
	return LiftFromRustBuffer[[]AggregationEntry](c, rb)
}

func (c FfiConverterSequenceAggregationEntry) Read(reader io.Reader) []AggregationEntry {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]AggregationEntry, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterAggregationEntryINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceAggregationEntry) Lower(value []AggregationEntry) C.RustBuffer {
	return LowerIntoRustBuffer[[]AggregationEntry](c, value)
}

func (c FfiConverterSequenceAggregationEntry) Write(writer io.Writer, value []AggregationEntry) {
	if len(value) > math.MaxInt32 {
		panic("[]AggregationEntry is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterAggregationEntryINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceAggregationEntry struct{}

func (FfiDestroyerSequenceAggregationEntry) Destroy(sequence []AggregationEntry) {
	for _, value := range sequence {
		FfiDestroyerAggregationEntry{}.Destroy(value)
	}
}

type FfiConverterSequenceAttributeFilter struct{}

var FfiConverterSequenceAttributeFilterINSTANCE = FfiConverterSequenceAttributeFilter{}

func (c FfiConverterSequenceAttributeFilter) Lift(rb RustBufferI) []AttributeFilter {
	return LiftFromRustBuffer[[]AttributeFilter](c, rb)
}

func (c FfiConverterSequenceAttributeFilter) Read(reader io.Reader) []AttributeFilter {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]AttributeFilter, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterAttributeFilterINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceAttributeFilter) Lower(value []AttributeFilter) C.RustBuffer {
	return LowerIntoRustBuffer[[]AttributeFilter](c, value)
}

func (c FfiConverterSequenceAttributeFilter) Write(writer io.Writer, value []AttributeFilter) {
	if len(value) > math.MaxInt32 {
		panic("[]AttributeFilter is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterAttributeFilterINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceAttributeFilter struct{}

func (FfiDestroyerSequenceAttributeFilter) Destroy(sequence []AttributeFilter) {
	for _, value := range sequence {
		FfiDestroyerAttributeFilter{}.Destroy(value)
	}
}

type FfiConverterSequenceContract struct{}

var FfiConverterSequenceContractINSTANCE = FfiConverterSequenceContract{}

func (c FfiConverterSequenceContract) Lift(rb RustBufferI) []Contract {
	return LiftFromRustBuffer[[]Contract](c, rb)
}

func (c FfiConverterSequenceContract) Read(reader io.Reader) []Contract {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]Contract, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterContractINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceContract) Lower(value []Contract) C.RustBuffer {
	return LowerIntoRustBuffer[[]Contract](c, value)
}

func (c FfiConverterSequenceContract) Write(writer io.Writer, value []Contract) {
	if len(value) > math.MaxInt32 {
		panic("[]Contract is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterContractINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceContract struct{}

func (FfiDestroyerSequenceContract) Destroy(sequence []Contract) {
	for _, value := range sequence {
		FfiDestroyerContract{}.Destroy(value)
	}
}

type FfiConverterSequenceController struct{}

var FfiConverterSequenceControllerINSTANCE = FfiConverterSequenceController{}

func (c FfiConverterSequenceController) Lift(rb RustBufferI) []Controller {
	return LiftFromRustBuffer[[]Controller](c, rb)
}

func (c FfiConverterSequenceController) Read(reader io.Reader) []Controller {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]Controller, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterControllerINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceController) Lower(value []Controller) C.RustBuffer {
	return LowerIntoRustBuffer[[]Controller](c, value)
}

func (c FfiConverterSequenceController) Write(writer io.Writer, value []Controller) {
	if len(value) > math.MaxInt32 {
		panic("[]Controller is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterControllerINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceController struct{}

func (FfiDestroyerSequenceController) Destroy(sequence []Controller) {
	for _, value := range sequence {
		FfiDestroyerController{}.Destroy(value)
	}
}

type FfiConverterSequenceEntity struct{}

var FfiConverterSequenceEntityINSTANCE = FfiConverterSequenceEntity{}

func (c FfiConverterSequenceEntity) Lift(rb RustBufferI) []Entity {
	return LiftFromRustBuffer[[]Entity](c, rb)
}

func (c FfiConverterSequenceEntity) Read(reader io.Reader) []Entity {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]Entity, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterEntityINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceEntity) Lower(value []Entity) C.RustBuffer {
	return LowerIntoRustBuffer[[]Entity](c, value)
}

func (c FfiConverterSequenceEntity) Write(writer io.Writer, value []Entity) {
	if len(value) > math.MaxInt32 {
		panic("[]Entity is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterEntityINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceEntity struct{}

func (FfiDestroyerSequenceEntity) Destroy(sequence []Entity) {
	for _, value := range sequence {
		FfiDestroyerEntity{}.Destroy(value)
	}
}

type FfiConverterSequenceEnumOption struct{}

var FfiConverterSequenceEnumOptionINSTANCE = FfiConverterSequenceEnumOption{}

func (c FfiConverterSequenceEnumOption) Lift(rb RustBufferI) []EnumOption {
	return LiftFromRustBuffer[[]EnumOption](c, rb)
}

func (c FfiConverterSequenceEnumOption) Read(reader io.Reader) []EnumOption {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]EnumOption, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterEnumOptionINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceEnumOption) Lower(value []EnumOption) C.RustBuffer {
	return LowerIntoRustBuffer[[]EnumOption](c, value)
}

func (c FfiConverterSequenceEnumOption) Write(writer io.Writer, value []EnumOption) {
	if len(value) > math.MaxInt32 {
		panic("[]EnumOption is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterEnumOptionINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceEnumOption struct{}

func (FfiDestroyerSequenceEnumOption) Destroy(sequence []EnumOption) {
	for _, value := range sequence {
		FfiDestroyerEnumOption{}.Destroy(value)
	}
}

type FfiConverterSequenceEvent struct{}

var FfiConverterSequenceEventINSTANCE = FfiConverterSequenceEvent{}

func (c FfiConverterSequenceEvent) Lift(rb RustBufferI) []Event {
	return LiftFromRustBuffer[[]Event](c, rb)
}

func (c FfiConverterSequenceEvent) Read(reader io.Reader) []Event {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]Event, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterEventINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceEvent) Lower(value []Event) C.RustBuffer {
	return LowerIntoRustBuffer[[]Event](c, value)
}

func (c FfiConverterSequenceEvent) Write(writer io.Writer, value []Event) {
	if len(value) > math.MaxInt32 {
		panic("[]Event is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterEventINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceEvent struct{}

func (FfiDestroyerSequenceEvent) Destroy(sequence []Event) {
	for _, value := range sequence {
		FfiDestroyerEvent{}.Destroy(value)
	}
}

type FfiConverterSequenceKeysClause struct{}

var FfiConverterSequenceKeysClauseINSTANCE = FfiConverterSequenceKeysClause{}

func (c FfiConverterSequenceKeysClause) Lift(rb RustBufferI) []KeysClause {
	return LiftFromRustBuffer[[]KeysClause](c, rb)
}

func (c FfiConverterSequenceKeysClause) Read(reader io.Reader) []KeysClause {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]KeysClause, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterKeysClauseINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceKeysClause) Lower(value []KeysClause) C.RustBuffer {
	return LowerIntoRustBuffer[[]KeysClause](c, value)
}

func (c FfiConverterSequenceKeysClause) Write(writer io.Writer, value []KeysClause) {
	if len(value) > math.MaxInt32 {
		panic("[]KeysClause is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterKeysClauseINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceKeysClause struct{}

func (FfiDestroyerSequenceKeysClause) Destroy(sequence []KeysClause) {
	for _, value := range sequence {
		FfiDestroyerKeysClause{}.Destroy(value)
	}
}

type FfiConverterSequenceMember struct{}

var FfiConverterSequenceMemberINSTANCE = FfiConverterSequenceMember{}

func (c FfiConverterSequenceMember) Lift(rb RustBufferI) []Member {
	return LiftFromRustBuffer[[]Member](c, rb)
}

func (c FfiConverterSequenceMember) Read(reader io.Reader) []Member {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]Member, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterMemberINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceMember) Lower(value []Member) C.RustBuffer {
	return LowerIntoRustBuffer[[]Member](c, value)
}

func (c FfiConverterSequenceMember) Write(writer io.Writer, value []Member) {
	if len(value) > math.MaxInt32 {
		panic("[]Member is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterMemberINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceMember struct{}

func (FfiDestroyerSequenceMember) Destroy(sequence []Member) {
	for _, value := range sequence {
		FfiDestroyerMember{}.Destroy(value)
	}
}

type FfiConverterSequenceMessage struct{}

var FfiConverterSequenceMessageINSTANCE = FfiConverterSequenceMessage{}

func (c FfiConverterSequenceMessage) Lift(rb RustBufferI) []Message {
	return LiftFromRustBuffer[[]Message](c, rb)
}

func (c FfiConverterSequenceMessage) Read(reader io.Reader) []Message {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]Message, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterMessageINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceMessage) Lower(value []Message) C.RustBuffer {
	return LowerIntoRustBuffer[[]Message](c, value)
}

func (c FfiConverterSequenceMessage) Write(writer io.Writer, value []Message) {
	if len(value) > math.MaxInt32 {
		panic("[]Message is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterMessageINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceMessage struct{}

func (FfiDestroyerSequenceMessage) Destroy(sequence []Message) {
	for _, value := range sequence {
		FfiDestroyerMessage{}.Destroy(value)
	}
}

type FfiConverterSequenceModel struct{}

var FfiConverterSequenceModelINSTANCE = FfiConverterSequenceModel{}

func (c FfiConverterSequenceModel) Lift(rb RustBufferI) []Model {
	return LiftFromRustBuffer[[]Model](c, rb)
}

func (c FfiConverterSequenceModel) Read(reader io.Reader) []Model {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]Model, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterModelINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceModel) Lower(value []Model) C.RustBuffer {
	return LowerIntoRustBuffer[[]Model](c, value)
}

func (c FfiConverterSequenceModel) Write(writer io.Writer, value []Model) {
	if len(value) > math.MaxInt32 {
		panic("[]Model is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterModelINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceModel struct{}

func (FfiDestroyerSequenceModel) Destroy(sequence []Model) {
	for _, value := range sequence {
		FfiDestroyerModel{}.Destroy(value)
	}
}

type FfiConverterSequenceOrderBy struct{}

var FfiConverterSequenceOrderByINSTANCE = FfiConverterSequenceOrderBy{}

func (c FfiConverterSequenceOrderBy) Lift(rb RustBufferI) []OrderBy {
	return LiftFromRustBuffer[[]OrderBy](c, rb)
}

func (c FfiConverterSequenceOrderBy) Read(reader io.Reader) []OrderBy {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]OrderBy, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterOrderByINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceOrderBy) Lower(value []OrderBy) C.RustBuffer {
	return LowerIntoRustBuffer[[]OrderBy](c, value)
}

func (c FfiConverterSequenceOrderBy) Write(writer io.Writer, value []OrderBy) {
	if len(value) > math.MaxInt32 {
		panic("[]OrderBy is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterOrderByINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceOrderBy struct{}

func (FfiDestroyerSequenceOrderBy) Destroy(sequence []OrderBy) {
	for _, value := range sequence {
		FfiDestroyerOrderBy{}.Destroy(value)
	}
}

type FfiConverterSequencePlayerAchievementEntry struct{}

var FfiConverterSequencePlayerAchievementEntryINSTANCE = FfiConverterSequencePlayerAchievementEntry{}

func (c FfiConverterSequencePlayerAchievementEntry) Lift(rb RustBufferI) []PlayerAchievementEntry {
	return LiftFromRustBuffer[[]PlayerAchievementEntry](c, rb)
}

func (c FfiConverterSequencePlayerAchievementEntry) Read(reader io.Reader) []PlayerAchievementEntry {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]PlayerAchievementEntry, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterPlayerAchievementEntryINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequencePlayerAchievementEntry) Lower(value []PlayerAchievementEntry) C.RustBuffer {
	return LowerIntoRustBuffer[[]PlayerAchievementEntry](c, value)
}

func (c FfiConverterSequencePlayerAchievementEntry) Write(writer io.Writer, value []PlayerAchievementEntry) {
	if len(value) > math.MaxInt32 {
		panic("[]PlayerAchievementEntry is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterPlayerAchievementEntryINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequencePlayerAchievementEntry struct{}

func (FfiDestroyerSequencePlayerAchievementEntry) Destroy(sequence []PlayerAchievementEntry) {
	for _, value := range sequence {
		FfiDestroyerPlayerAchievementEntry{}.Destroy(value)
	}
}

type FfiConverterSequencePlayerAchievementProgress struct{}

var FfiConverterSequencePlayerAchievementProgressINSTANCE = FfiConverterSequencePlayerAchievementProgress{}

func (c FfiConverterSequencePlayerAchievementProgress) Lift(rb RustBufferI) []PlayerAchievementProgress {
	return LiftFromRustBuffer[[]PlayerAchievementProgress](c, rb)
}

func (c FfiConverterSequencePlayerAchievementProgress) Read(reader io.Reader) []PlayerAchievementProgress {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]PlayerAchievementProgress, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterPlayerAchievementProgressINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequencePlayerAchievementProgress) Lower(value []PlayerAchievementProgress) C.RustBuffer {
	return LowerIntoRustBuffer[[]PlayerAchievementProgress](c, value)
}

func (c FfiConverterSequencePlayerAchievementProgress) Write(writer io.Writer, value []PlayerAchievementProgress) {
	if len(value) > math.MaxInt32 {
		panic("[]PlayerAchievementProgress is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterPlayerAchievementProgressINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequencePlayerAchievementProgress struct{}

func (FfiDestroyerSequencePlayerAchievementProgress) Destroy(sequence []PlayerAchievementProgress) {
	for _, value := range sequence {
		FfiDestroyerPlayerAchievementProgress{}.Destroy(value)
	}
}

type FfiConverterSequenceSqlField struct{}

var FfiConverterSequenceSqlFieldINSTANCE = FfiConverterSequenceSqlField{}

func (c FfiConverterSequenceSqlField) Lift(rb RustBufferI) []SqlField {
	return LiftFromRustBuffer[[]SqlField](c, rb)
}

func (c FfiConverterSequenceSqlField) Read(reader io.Reader) []SqlField {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]SqlField, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterSqlFieldINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceSqlField) Lower(value []SqlField) C.RustBuffer {
	return LowerIntoRustBuffer[[]SqlField](c, value)
}

func (c FfiConverterSequenceSqlField) Write(writer io.Writer, value []SqlField) {
	if len(value) > math.MaxInt32 {
		panic("[]SqlField is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterSqlFieldINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceSqlField struct{}

func (FfiDestroyerSequenceSqlField) Destroy(sequence []SqlField) {
	for _, value := range sequence {
		FfiDestroyerSqlField{}.Destroy(value)
	}
}

type FfiConverterSequenceSqlRow struct{}

var FfiConverterSequenceSqlRowINSTANCE = FfiConverterSequenceSqlRow{}

func (c FfiConverterSequenceSqlRow) Lift(rb RustBufferI) []SqlRow {
	return LiftFromRustBuffer[[]SqlRow](c, rb)
}

func (c FfiConverterSequenceSqlRow) Read(reader io.Reader) []SqlRow {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]SqlRow, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterSqlRowINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceSqlRow) Lower(value []SqlRow) C.RustBuffer {
	return LowerIntoRustBuffer[[]SqlRow](c, value)
}

func (c FfiConverterSequenceSqlRow) Write(writer io.Writer, value []SqlRow) {
	if len(value) > math.MaxInt32 {
		panic("[]SqlRow is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterSqlRowINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceSqlRow struct{}

func (FfiDestroyerSequenceSqlRow) Destroy(sequence []SqlRow) {
	for _, value := range sequence {
		FfiDestroyerSqlRow{}.Destroy(value)
	}
}

type FfiConverterSequenceStruct struct{}

var FfiConverterSequenceStructINSTANCE = FfiConverterSequenceStruct{}

func (c FfiConverterSequenceStruct) Lift(rb RustBufferI) []Struct {
	return LiftFromRustBuffer[[]Struct](c, rb)
}

func (c FfiConverterSequenceStruct) Read(reader io.Reader) []Struct {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]Struct, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterStructINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceStruct) Lower(value []Struct) C.RustBuffer {
	return LowerIntoRustBuffer[[]Struct](c, value)
}

func (c FfiConverterSequenceStruct) Write(writer io.Writer, value []Struct) {
	if len(value) > math.MaxInt32 {
		panic("[]Struct is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterStructINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceStruct struct{}

func (FfiDestroyerSequenceStruct) Destroy(sequence []Struct) {
	for _, value := range sequence {
		FfiDestroyerStruct{}.Destroy(value)
	}
}

type FfiConverterSequenceTaskProgress struct{}

var FfiConverterSequenceTaskProgressINSTANCE = FfiConverterSequenceTaskProgress{}

func (c FfiConverterSequenceTaskProgress) Lift(rb RustBufferI) []TaskProgress {
	return LiftFromRustBuffer[[]TaskProgress](c, rb)
}

func (c FfiConverterSequenceTaskProgress) Read(reader io.Reader) []TaskProgress {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]TaskProgress, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterTaskProgressINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceTaskProgress) Lower(value []TaskProgress) C.RustBuffer {
	return LowerIntoRustBuffer[[]TaskProgress](c, value)
}

func (c FfiConverterSequenceTaskProgress) Write(writer io.Writer, value []TaskProgress) {
	if len(value) > math.MaxInt32 {
		panic("[]TaskProgress is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterTaskProgressINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceTaskProgress struct{}

func (FfiDestroyerSequenceTaskProgress) Destroy(sequence []TaskProgress) {
	for _, value := range sequence {
		FfiDestroyerTaskProgress{}.Destroy(value)
	}
}

type FfiConverterSequenceToken struct{}

var FfiConverterSequenceTokenINSTANCE = FfiConverterSequenceToken{}

func (c FfiConverterSequenceToken) Lift(rb RustBufferI) []Token {
	return LiftFromRustBuffer[[]Token](c, rb)
}

func (c FfiConverterSequenceToken) Read(reader io.Reader) []Token {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]Token, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterTokenINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceToken) Lower(value []Token) C.RustBuffer {
	return LowerIntoRustBuffer[[]Token](c, value)
}

func (c FfiConverterSequenceToken) Write(writer io.Writer, value []Token) {
	if len(value) > math.MaxInt32 {
		panic("[]Token is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterTokenINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceToken struct{}

func (FfiDestroyerSequenceToken) Destroy(sequence []Token) {
	for _, value := range sequence {
		FfiDestroyerToken{}.Destroy(value)
	}
}

type FfiConverterSequenceTokenBalance struct{}

var FfiConverterSequenceTokenBalanceINSTANCE = FfiConverterSequenceTokenBalance{}

func (c FfiConverterSequenceTokenBalance) Lift(rb RustBufferI) []TokenBalance {
	return LiftFromRustBuffer[[]TokenBalance](c, rb)
}

func (c FfiConverterSequenceTokenBalance) Read(reader io.Reader) []TokenBalance {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]TokenBalance, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterTokenBalanceINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceTokenBalance) Lower(value []TokenBalance) C.RustBuffer {
	return LowerIntoRustBuffer[[]TokenBalance](c, value)
}

func (c FfiConverterSequenceTokenBalance) Write(writer io.Writer, value []TokenBalance) {
	if len(value) > math.MaxInt32 {
		panic("[]TokenBalance is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterTokenBalanceINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceTokenBalance struct{}

func (FfiDestroyerSequenceTokenBalance) Destroy(sequence []TokenBalance) {
	for _, value := range sequence {
		FfiDestroyerTokenBalance{}.Destroy(value)
	}
}

type FfiConverterSequenceTokenContract struct{}

var FfiConverterSequenceTokenContractINSTANCE = FfiConverterSequenceTokenContract{}

func (c FfiConverterSequenceTokenContract) Lift(rb RustBufferI) []TokenContract {
	return LiftFromRustBuffer[[]TokenContract](c, rb)
}

func (c FfiConverterSequenceTokenContract) Read(reader io.Reader) []TokenContract {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]TokenContract, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterTokenContractINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceTokenContract) Lower(value []TokenContract) C.RustBuffer {
	return LowerIntoRustBuffer[[]TokenContract](c, value)
}

func (c FfiConverterSequenceTokenContract) Write(writer io.Writer, value []TokenContract) {
	if len(value) > math.MaxInt32 {
		panic("[]TokenContract is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterTokenContractINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceTokenContract struct{}

func (FfiDestroyerSequenceTokenContract) Destroy(sequence []TokenContract) {
	for _, value := range sequence {
		FfiDestroyerTokenContract{}.Destroy(value)
	}
}

type FfiConverterSequenceTokenTransfer struct{}

var FfiConverterSequenceTokenTransferINSTANCE = FfiConverterSequenceTokenTransfer{}

func (c FfiConverterSequenceTokenTransfer) Lift(rb RustBufferI) []TokenTransfer {
	return LiftFromRustBuffer[[]TokenTransfer](c, rb)
}

func (c FfiConverterSequenceTokenTransfer) Read(reader io.Reader) []TokenTransfer {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]TokenTransfer, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterTokenTransferINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceTokenTransfer) Lower(value []TokenTransfer) C.RustBuffer {
	return LowerIntoRustBuffer[[]TokenTransfer](c, value)
}

func (c FfiConverterSequenceTokenTransfer) Write(writer io.Writer, value []TokenTransfer) {
	if len(value) > math.MaxInt32 {
		panic("[]TokenTransfer is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterTokenTransferINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceTokenTransfer struct{}

func (FfiDestroyerSequenceTokenTransfer) Destroy(sequence []TokenTransfer) {
	for _, value := range sequence {
		FfiDestroyerTokenTransfer{}.Destroy(value)
	}
}

type FfiConverterSequenceTransaction struct{}

var FfiConverterSequenceTransactionINSTANCE = FfiConverterSequenceTransaction{}

func (c FfiConverterSequenceTransaction) Lift(rb RustBufferI) []Transaction {
	return LiftFromRustBuffer[[]Transaction](c, rb)
}

func (c FfiConverterSequenceTransaction) Read(reader io.Reader) []Transaction {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]Transaction, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterTransactionINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceTransaction) Lower(value []Transaction) C.RustBuffer {
	return LowerIntoRustBuffer[[]Transaction](c, value)
}

func (c FfiConverterSequenceTransaction) Write(writer io.Writer, value []Transaction) {
	if len(value) > math.MaxInt32 {
		panic("[]Transaction is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterTransactionINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceTransaction struct{}

func (FfiDestroyerSequenceTransaction) Destroy(sequence []Transaction) {
	for _, value := range sequence {
		FfiDestroyerTransaction{}.Destroy(value)
	}
}

type FfiConverterSequenceTransactionCall struct{}

var FfiConverterSequenceTransactionCallINSTANCE = FfiConverterSequenceTransactionCall{}

func (c FfiConverterSequenceTransactionCall) Lift(rb RustBufferI) []TransactionCall {
	return LiftFromRustBuffer[[]TransactionCall](c, rb)
}

func (c FfiConverterSequenceTransactionCall) Read(reader io.Reader) []TransactionCall {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]TransactionCall, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterTransactionCallINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceTransactionCall) Lower(value []TransactionCall) C.RustBuffer {
	return LowerIntoRustBuffer[[]TransactionCall](c, value)
}

func (c FfiConverterSequenceTransactionCall) Write(writer io.Writer, value []TransactionCall) {
	if len(value) > math.MaxInt32 {
		panic("[]TransactionCall is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterTransactionCallINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceTransactionCall struct{}

func (FfiDestroyerSequenceTransactionCall) Destroy(sequence []TransactionCall) {
	for _, value := range sequence {
		FfiDestroyerTransactionCall{}.Destroy(value)
	}
}

type FfiConverterSequenceWorld struct{}

var FfiConverterSequenceWorldINSTANCE = FfiConverterSequenceWorld{}

func (c FfiConverterSequenceWorld) Lift(rb RustBufferI) []World {
	return LiftFromRustBuffer[[]World](c, rb)
}

func (c FfiConverterSequenceWorld) Read(reader io.Reader) []World {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]World, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterWorldINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceWorld) Lower(value []World) C.RustBuffer {
	return LowerIntoRustBuffer[[]World](c, value)
}

func (c FfiConverterSequenceWorld) Write(writer io.Writer, value []World) {
	if len(value) > math.MaxInt32 {
		panic("[]World is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterWorldINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceWorld struct{}

func (FfiDestroyerSequenceWorld) Destroy(sequence []World) {
	for _, value := range sequence {
		FfiDestroyerWorld{}.Destroy(value)
	}
}

type FfiConverterSequenceClause struct{}

var FfiConverterSequenceClauseINSTANCE = FfiConverterSequenceClause{}

func (c FfiConverterSequenceClause) Lift(rb RustBufferI) []Clause {
	return LiftFromRustBuffer[[]Clause](c, rb)
}

func (c FfiConverterSequenceClause) Read(reader io.Reader) []Clause {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]Clause, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterClauseINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceClause) Lower(value []Clause) C.RustBuffer {
	return LowerIntoRustBuffer[[]Clause](c, value)
}

func (c FfiConverterSequenceClause) Write(writer io.Writer, value []Clause) {
	if len(value) > math.MaxInt32 {
		panic("[]Clause is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterClauseINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceClause struct{}

func (FfiDestroyerSequenceClause) Destroy(sequence []Clause) {
	for _, value := range sequence {
		FfiDestroyerClause{}.Destroy(value)
	}
}

type FfiConverterSequenceContractType struct{}

var FfiConverterSequenceContractTypeINSTANCE = FfiConverterSequenceContractType{}

func (c FfiConverterSequenceContractType) Lift(rb RustBufferI) []ContractType {
	return LiftFromRustBuffer[[]ContractType](c, rb)
}

func (c FfiConverterSequenceContractType) Read(reader io.Reader) []ContractType {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]ContractType, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterContractTypeINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceContractType) Lower(value []ContractType) C.RustBuffer {
	return LowerIntoRustBuffer[[]ContractType](c, value)
}

func (c FfiConverterSequenceContractType) Write(writer io.Writer, value []ContractType) {
	if len(value) > math.MaxInt32 {
		panic("[]ContractType is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterContractTypeINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceContractType struct{}

func (FfiDestroyerSequenceContractType) Destroy(sequence []ContractType) {
	for _, value := range sequence {
		FfiDestroyerContractType{}.Destroy(value)
	}
}

type FfiConverterSequenceMemberValue struct{}

var FfiConverterSequenceMemberValueINSTANCE = FfiConverterSequenceMemberValue{}

func (c FfiConverterSequenceMemberValue) Lift(rb RustBufferI) []MemberValue {
	return LiftFromRustBuffer[[]MemberValue](c, rb)
}

func (c FfiConverterSequenceMemberValue) Read(reader io.Reader) []MemberValue {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]MemberValue, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterMemberValueINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceMemberValue) Lower(value []MemberValue) C.RustBuffer {
	return LowerIntoRustBuffer[[]MemberValue](c, value)
}

func (c FfiConverterSequenceMemberValue) Write(writer io.Writer, value []MemberValue) {
	if len(value) > math.MaxInt32 {
		panic("[]MemberValue is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterMemberValueINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceMemberValue struct{}

func (FfiDestroyerSequenceMemberValue) Destroy(sequence []MemberValue) {
	for _, value := range sequence {
		FfiDestroyerMemberValue{}.Destroy(value)
	}
}

type FfiConverterSequenceTy struct{}

var FfiConverterSequenceTyINSTANCE = FfiConverterSequenceTy{}

func (c FfiConverterSequenceTy) Lift(rb RustBufferI) []Ty {
	return LiftFromRustBuffer[[]Ty](c, rb)
}

func (c FfiConverterSequenceTy) Read(reader io.Reader) []Ty {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]Ty, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterTyINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceTy) Lower(value []Ty) C.RustBuffer {
	return LowerIntoRustBuffer[[]Ty](c, value)
}

func (c FfiConverterSequenceTy) Write(writer io.Writer, value []Ty) {
	if len(value) > math.MaxInt32 {
		panic("[]Ty is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterTyINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceTy struct{}

func (FfiDestroyerSequenceTy) Destroy(sequence []Ty) {
	for _, value := range sequence {
		FfiDestroyerTy{}.Destroy(value)
	}
}

type FfiConverterSequenceOptionalTypeFieldElement struct{}

var FfiConverterSequenceOptionalTypeFieldElementINSTANCE = FfiConverterSequenceOptionalTypeFieldElement{}

func (c FfiConverterSequenceOptionalTypeFieldElement) Lift(rb RustBufferI) []*FieldElement {
	return LiftFromRustBuffer[[]*FieldElement](c, rb)
}

func (c FfiConverterSequenceOptionalTypeFieldElement) Read(reader io.Reader) []*FieldElement {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]*FieldElement, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterOptionalTypeFieldElementINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceOptionalTypeFieldElement) Lower(value []*FieldElement) C.RustBuffer {
	return LowerIntoRustBuffer[[]*FieldElement](c, value)
}

func (c FfiConverterSequenceOptionalTypeFieldElement) Write(writer io.Writer, value []*FieldElement) {
	if len(value) > math.MaxInt32 {
		panic("[]*FieldElement is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterOptionalTypeFieldElementINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceOptionalTypeFieldElement struct{}

func (FfiDestroyerSequenceOptionalTypeFieldElement) Destroy(sequence []*FieldElement) {
	for _, value := range sequence {
		FfiDestroyerOptionalTypeFieldElement{}.Destroy(value)
	}
}

type FfiConverterSequenceTypeFieldElement struct{}

var FfiConverterSequenceTypeFieldElementINSTANCE = FfiConverterSequenceTypeFieldElement{}

func (c FfiConverterSequenceTypeFieldElement) Lift(rb RustBufferI) []FieldElement {
	return LiftFromRustBuffer[[]FieldElement](c, rb)
}

func (c FfiConverterSequenceTypeFieldElement) Read(reader io.Reader) []FieldElement {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]FieldElement, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterTypeFieldElementINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceTypeFieldElement) Lower(value []FieldElement) C.RustBuffer {
	return LowerIntoRustBuffer[[]FieldElement](c, value)
}

func (c FfiConverterSequenceTypeFieldElement) Write(writer io.Writer, value []FieldElement) {
	if len(value) > math.MaxInt32 {
		panic("[]FieldElement is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterTypeFieldElementINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceTypeFieldElement struct{}

func (FfiDestroyerSequenceTypeFieldElement) Destroy(sequence []FieldElement) {
	for _, value := range sequence {
		FfiDestroyerTypeFieldElement{}.Destroy(value)
	}
}

type FfiConverterSequenceTypeU256 struct{}

var FfiConverterSequenceTypeU256INSTANCE = FfiConverterSequenceTypeU256{}

func (c FfiConverterSequenceTypeU256) Lift(rb RustBufferI) []U256 {
	return LiftFromRustBuffer[[]U256](c, rb)
}

func (c FfiConverterSequenceTypeU256) Read(reader io.Reader) []U256 {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]U256, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterTypeU256INSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceTypeU256) Lower(value []U256) C.RustBuffer {
	return LowerIntoRustBuffer[[]U256](c, value)
}

func (c FfiConverterSequenceTypeU256) Write(writer io.Writer, value []U256) {
	if len(value) > math.MaxInt32 {
		panic("[]U256 is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterTypeU256INSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceTypeU256 struct{}

func (FfiDestroyerSequenceTypeU256) Destroy(sequence []U256) {
	for _, value := range sequence {
		FfiDestroyerTypeU256{}.Destroy(value)
	}
}

/**
 * Typealias from the type name used in the UDL file to the builtin type.  This
 * is needed because the UDL type name is used in function/method signatures.
 * It's also what we have an external type that references a custom type.
 */
type FieldElement = string
type FfiConverterTypeFieldElement = FfiConverterString
type FfiDestroyerTypeFieldElement = FfiDestroyerString

var FfiConverterTypeFieldElementINSTANCE = FfiConverterString{}

/**
 * Typealias from the type name used in the UDL file to the builtin type.  This
 * is needed because the UDL type name is used in function/method signatures.
 * It's also what we have an external type that references a custom type.
 */
type U256 = string
type FfiConverterTypeU256 = FfiConverterString
type FfiDestroyerTypeU256 = FfiDestroyerString

var FfiConverterTypeU256INSTANCE = FfiConverterString{}
