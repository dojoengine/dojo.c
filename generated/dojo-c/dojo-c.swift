public func new_torii_client<GenericIntoRustString: IntoRustString>(_ torii_url: GenericIntoRustString) async throws -> ToriiClient {
    func onComplete(cbWrapperPtr: UnsafeMutableRawPointer?, rustFnRetVal: __private__ResultPtrAndPtr) {
        let wrapper = Unmanaged<CbWrapper$new_torii_client>.fromOpaque(cbWrapperPtr!).takeRetainedValue()
        if rustFnRetVal.is_ok {
            wrapper.cb(.success(ToriiClient(ptr: rustFnRetVal.ok_or_err!)))
        } else {
            wrapper.cb(.failure(ToriiClientError(ptr: rustFnRetVal.ok_or_err!)))
        }
    }

    return try await withCheckedThrowingContinuation({ (continuation: CheckedContinuation<ToriiClient, Error>) in
        let callback = { rustFnRetVal in
            continuation.resume(with: rustFnRetVal)
        }

        let wrapper = CbWrapper$new_torii_client(cb: callback)
        let wrapperPtr = Unmanaged.passRetained(wrapper).toOpaque()

        __swift_bridge__$new_torii_client(wrapperPtr, onComplete, { let rustString = torii_url.intoRustString(); rustString.isOwned = false; return rustString.ptr }())
    })
}
class CbWrapper$new_torii_client {
    var cb: (Result<ToriiClient, Error>) -> ()

    public init(cb: @escaping (Result<ToriiClient, Error>) -> ()) {
        self.cb = cb
    }
}
public func new_torii_client_with_config<GenericIntoRustString: IntoRustString>(_ torii_url: GenericIntoRustString, _ max_message_size: UInt) async throws -> ToriiClient {
    func onComplete(cbWrapperPtr: UnsafeMutableRawPointer?, rustFnRetVal: __private__ResultPtrAndPtr) {
        let wrapper = Unmanaged<CbWrapper$new_torii_client_with_config>.fromOpaque(cbWrapperPtr!).takeRetainedValue()
        if rustFnRetVal.is_ok {
            wrapper.cb(.success(ToriiClient(ptr: rustFnRetVal.ok_or_err!)))
        } else {
            wrapper.cb(.failure(ToriiClientError(ptr: rustFnRetVal.ok_or_err!)))
        }
    }

    return try await withCheckedThrowingContinuation({ (continuation: CheckedContinuation<ToriiClient, Error>) in
        let callback = { rustFnRetVal in
            continuation.resume(with: rustFnRetVal)
        }

        let wrapper = CbWrapper$new_torii_client_with_config(cb: callback)
        let wrapperPtr = Unmanaged.passRetained(wrapper).toOpaque()

        __swift_bridge__$new_torii_client_with_config(wrapperPtr, onComplete, { let rustString = torii_url.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), max_message_size)
    })
}
class CbWrapper$new_torii_client_with_config {
    var cb: (Result<ToriiClient, Error>) -> ()

    public init(cb: @escaping (Result<ToriiClient, Error>) -> ()) {
        self.cb = cb
    }
}

public class Subscription: SubscriptionRefMut {
    var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$Subscription$_free(ptr)
        }
    }
}
extension Subscription {
    public func cancel() {
        __swift_bridge__$Subscription$cancel({isOwned = false; return ptr;}())
    }
}
public class SubscriptionRefMut: SubscriptionRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class SubscriptionRef {
    var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension SubscriptionRef {
    public func id() -> UInt64 {
        __swift_bridge__$Subscription$id(ptr)
    }
}
extension Subscription: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_Subscription$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_Subscription$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: Subscription) {
        __swift_bridge__$Vec_Subscription$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_Subscription$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (Subscription(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<SubscriptionRef> {
        let pointer = __swift_bridge__$Vec_Subscription$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return SubscriptionRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<SubscriptionRefMut> {
        let pointer = __swift_bridge__$Vec_Subscription$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return SubscriptionRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<SubscriptionRef> {
        UnsafePointer<SubscriptionRef>(OpaquePointer(__swift_bridge__$Vec_Subscription$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_Subscription$len(vecPtr)
    }
}


public class ToriiClientError: ToriiClientErrorRefMut {
    var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$ToriiClientError$_free(ptr)
        }
    }
}
public class ToriiClientErrorRefMut: ToriiClientErrorRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ToriiClientErrorRef {
    var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension ToriiClientErrorRef {
    public func message() -> RustString {
        RustString(ptr: __swift_bridge__$ToriiClientError$message(ptr))
    }
}
extension ToriiClientError: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_ToriiClientError$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_ToriiClientError$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ToriiClientError) {
        __swift_bridge__$Vec_ToriiClientError$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_ToriiClientError$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (ToriiClientError(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ToriiClientErrorRef> {
        let pointer = __swift_bridge__$Vec_ToriiClientError$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ToriiClientErrorRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ToriiClientErrorRefMut> {
        let pointer = __swift_bridge__$Vec_ToriiClientError$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ToriiClientErrorRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ToriiClientErrorRef> {
        UnsafePointer<ToriiClientErrorRef>(OpaquePointer(__swift_bridge__$Vec_ToriiClientError$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_ToriiClientError$len(vecPtr)
    }
}


public class ToriiClient: ToriiClientRefMut {
    var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$ToriiClient$_free(ptr)
        }
    }
}
public class ToriiClientRefMut: ToriiClientRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ToriiClientRef {
    var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension ToriiClientRef {
    public func worlds(_ world_addresses: RustVec<FeltBridge>) async throws -> RustVec<WorldBridge> {
        func onComplete(cbWrapperPtr: UnsafeMutableRawPointer?, rustFnRetVal: __private__ResultPtrAndPtr) {
            let wrapper = Unmanaged<CbWrapper$ToriiClient$worlds>.fromOpaque(cbWrapperPtr!).takeRetainedValue()
            if rustFnRetVal.is_ok {
                wrapper.cb(.success(RustVec<WorldBridge>(ptr: rustFnRetVal.ok_or_err!)))
            } else {
                wrapper.cb(.failure(ToriiClientError(ptr: rustFnRetVal.ok_or_err!)))
            }
        }

        return try await withCheckedThrowingContinuation({ (continuation: CheckedContinuation<RustVec<WorldBridge>, Error>) in
            let callback = { rustFnRetVal in
                continuation.resume(with: rustFnRetVal)
            }

            let wrapper = CbWrapper$ToriiClient$worlds(cb: callback)
            let wrapperPtr = Unmanaged.passRetained(wrapper).toOpaque()

            __swift_bridge__$ToriiClient$worlds(wrapperPtr, onComplete, ptr, { let val = world_addresses; val.isOwned = false; return val.ptr }())
        })
    }
    class CbWrapper$ToriiClient$worlds {
        var cb: (Result<RustVec<WorldBridge>, Error>) -> ()
    
        public init(cb: @escaping (Result<RustVec<WorldBridge>, Error>) -> ()) {
            self.cb = cb
        }
    }

    public func publish_message<GenericIntoRustString: IntoRustString>(_ message_json: GenericIntoRustString) async throws -> RustString {
        func onComplete(cbWrapperPtr: UnsafeMutableRawPointer?, rustFnRetVal: __private__ResultPtrAndPtr) {
            let wrapper = Unmanaged<CbWrapper$ToriiClient$publish_message>.fromOpaque(cbWrapperPtr!).takeRetainedValue()
            if rustFnRetVal.is_ok {
                wrapper.cb(.success(RustString(ptr: rustFnRetVal.ok_or_err!)))
            } else {
                wrapper.cb(.failure(ToriiClientError(ptr: rustFnRetVal.ok_or_err!)))
            }
        }

        return try await withCheckedThrowingContinuation({ (continuation: CheckedContinuation<RustString, Error>) in
            let callback = { rustFnRetVal in
                continuation.resume(with: rustFnRetVal)
            }

            let wrapper = CbWrapper$ToriiClient$publish_message(cb: callback)
            let wrapperPtr = Unmanaged.passRetained(wrapper).toOpaque()

            __swift_bridge__$ToriiClient$publish_message(wrapperPtr, onComplete, ptr, { let rustString = message_json.intoRustString(); rustString.isOwned = false; return rustString.ptr }())
        })
    }
    class CbWrapper$ToriiClient$publish_message {
        var cb: (Result<RustString, Error>) -> ()
    
        public init(cb: @escaping (Result<RustString, Error>) -> ()) {
            self.cb = cb
        }
    }

    public func publish_message_batch<GenericIntoRustString: IntoRustString>(_ messages_json: GenericIntoRustString) async throws -> RustString {
        func onComplete(cbWrapperPtr: UnsafeMutableRawPointer?, rustFnRetVal: __private__ResultPtrAndPtr) {
            let wrapper = Unmanaged<CbWrapper$ToriiClient$publish_message_batch>.fromOpaque(cbWrapperPtr!).takeRetainedValue()
            if rustFnRetVal.is_ok {
                wrapper.cb(.success(RustString(ptr: rustFnRetVal.ok_or_err!)))
            } else {
                wrapper.cb(.failure(ToriiClientError(ptr: rustFnRetVal.ok_or_err!)))
            }
        }

        return try await withCheckedThrowingContinuation({ (continuation: CheckedContinuation<RustString, Error>) in
            let callback = { rustFnRetVal in
                continuation.resume(with: rustFnRetVal)
            }

            let wrapper = CbWrapper$ToriiClient$publish_message_batch(cb: callback)
            let wrapperPtr = Unmanaged.passRetained(wrapper).toOpaque()

            __swift_bridge__$ToriiClient$publish_message_batch(wrapperPtr, onComplete, ptr, { let rustString = messages_json.intoRustString(); rustString.isOwned = false; return rustString.ptr }())
        })
    }
    class CbWrapper$ToriiClient$publish_message_batch {
        var cb: (Result<RustString, Error>) -> ()
    
        public init(cb: @escaping (Result<RustString, Error>) -> ()) {
            self.cb = cb
        }
    }

    public func entities(_ query: QueryBridge) async throws -> PageBridge {
        func onComplete(cbWrapperPtr: UnsafeMutableRawPointer?, rustFnRetVal: __swift_bridge__$ResultPageBridgeAndToriiClientError) {
            let wrapper = Unmanaged<CbWrapper$ToriiClient$entities>.fromOpaque(cbWrapperPtr!).takeRetainedValue()
            switch rustFnRetVal.tag { case __swift_bridge__$ResultPageBridgeAndToriiClientError$ResultOk: wrapper.cb(.success(rustFnRetVal.payload.ok.intoSwiftRepr())) case __swift_bridge__$ResultPageBridgeAndToriiClientError$ResultErr: wrapper.cb(.failure(ToriiClientError(ptr: rustFnRetVal.payload.err))) default: fatalError() }
        }

        return try await withCheckedThrowingContinuation({ (continuation: CheckedContinuation<PageBridge, Error>) in
            let callback = { rustFnRetVal in
                continuation.resume(with: rustFnRetVal)
            }

            let wrapper = CbWrapper$ToriiClient$entities(cb: callback)
            let wrapperPtr = Unmanaged.passRetained(wrapper).toOpaque()

            __swift_bridge__$ToriiClient$entities(wrapperPtr, onComplete, ptr, query.intoFfiRepr())
        })
    }
    class CbWrapper$ToriiClient$entities {
        var cb: (Result<PageBridge, Error>) -> ()
    
        public init(cb: @escaping (Result<PageBridge, Error>) -> ()) {
            self.cb = cb
        }
    }

    public func event_messages(_ query: QueryBridge) async throws -> PageBridge {
        func onComplete(cbWrapperPtr: UnsafeMutableRawPointer?, rustFnRetVal: __swift_bridge__$ResultPageBridgeAndToriiClientError) {
            let wrapper = Unmanaged<CbWrapper$ToriiClient$event_messages>.fromOpaque(cbWrapperPtr!).takeRetainedValue()
            switch rustFnRetVal.tag { case __swift_bridge__$ResultPageBridgeAndToriiClientError$ResultOk: wrapper.cb(.success(rustFnRetVal.payload.ok.intoSwiftRepr())) case __swift_bridge__$ResultPageBridgeAndToriiClientError$ResultErr: wrapper.cb(.failure(ToriiClientError(ptr: rustFnRetVal.payload.err))) default: fatalError() }
        }

        return try await withCheckedThrowingContinuation({ (continuation: CheckedContinuation<PageBridge, Error>) in
            let callback = { rustFnRetVal in
                continuation.resume(with: rustFnRetVal)
            }

            let wrapper = CbWrapper$ToriiClient$event_messages(cb: callback)
            let wrapperPtr = Unmanaged.passRetained(wrapper).toOpaque()

            __swift_bridge__$ToriiClient$event_messages(wrapperPtr, onComplete, ptr, query.intoFfiRepr())
        })
    }
    class CbWrapper$ToriiClient$event_messages {
        var cb: (Result<PageBridge, Error>) -> ()
    
        public init(cb: @escaping (Result<PageBridge, Error>) -> ()) {
            self.cb = cb
        }
    }

    public func starknet_events<GenericIntoRustString: IntoRustString>(_ keys_json: GenericIntoRustString, _ limit: UInt32, _ cursor: GenericIntoRustString) async throws -> PageBridge {
        func onComplete(cbWrapperPtr: UnsafeMutableRawPointer?, rustFnRetVal: __swift_bridge__$ResultPageBridgeAndToriiClientError) {
            let wrapper = Unmanaged<CbWrapper$ToriiClient$starknet_events>.fromOpaque(cbWrapperPtr!).takeRetainedValue()
            switch rustFnRetVal.tag { case __swift_bridge__$ResultPageBridgeAndToriiClientError$ResultOk: wrapper.cb(.success(rustFnRetVal.payload.ok.intoSwiftRepr())) case __swift_bridge__$ResultPageBridgeAndToriiClientError$ResultErr: wrapper.cb(.failure(ToriiClientError(ptr: rustFnRetVal.payload.err))) default: fatalError() }
        }

        return try await withCheckedThrowingContinuation({ (continuation: CheckedContinuation<PageBridge, Error>) in
            let callback = { rustFnRetVal in
                continuation.resume(with: rustFnRetVal)
            }

            let wrapper = CbWrapper$ToriiClient$starknet_events(cb: callback)
            let wrapperPtr = Unmanaged.passRetained(wrapper).toOpaque()

            __swift_bridge__$ToriiClient$starknet_events(wrapperPtr, onComplete, ptr, { let rustString = keys_json.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), limit, { let rustString = cursor.intoRustString(); rustString.isOwned = false; return rustString.ptr }())
        })
    }
    class CbWrapper$ToriiClient$starknet_events {
        var cb: (Result<PageBridge, Error>) -> ()
    
        public init(cb: @escaping (Result<PageBridge, Error>) -> ()) {
            self.cb = cb
        }
    }

    public func tokens<GenericIntoRustString: IntoRustString>(_ contract_addresses: RustVec<FeltBridge>, _ limit: UInt32, _ cursor: GenericIntoRustString) async throws -> PageBridge {
        func onComplete(cbWrapperPtr: UnsafeMutableRawPointer?, rustFnRetVal: __swift_bridge__$ResultPageBridgeAndToriiClientError) {
            let wrapper = Unmanaged<CbWrapper$ToriiClient$tokens>.fromOpaque(cbWrapperPtr!).takeRetainedValue()
            switch rustFnRetVal.tag { case __swift_bridge__$ResultPageBridgeAndToriiClientError$ResultOk: wrapper.cb(.success(rustFnRetVal.payload.ok.intoSwiftRepr())) case __swift_bridge__$ResultPageBridgeAndToriiClientError$ResultErr: wrapper.cb(.failure(ToriiClientError(ptr: rustFnRetVal.payload.err))) default: fatalError() }
        }

        return try await withCheckedThrowingContinuation({ (continuation: CheckedContinuation<PageBridge, Error>) in
            let callback = { rustFnRetVal in
                continuation.resume(with: rustFnRetVal)
            }

            let wrapper = CbWrapper$ToriiClient$tokens(cb: callback)
            let wrapperPtr = Unmanaged.passRetained(wrapper).toOpaque()

            __swift_bridge__$ToriiClient$tokens(wrapperPtr, onComplete, ptr, { let val = contract_addresses; val.isOwned = false; return val.ptr }(), limit, { let rustString = cursor.intoRustString(); rustString.isOwned = false; return rustString.ptr }())
        })
    }
    class CbWrapper$ToriiClient$tokens {
        var cb: (Result<PageBridge, Error>) -> ()
    
        public init(cb: @escaping (Result<PageBridge, Error>) -> ()) {
            self.cb = cb
        }
    }

    public func token_balances<GenericIntoRustString: IntoRustString>(_ account_addresses: RustVec<FeltBridge>, _ contract_addresses: RustVec<FeltBridge>, _ limit: UInt32, _ cursor: GenericIntoRustString) async throws -> PageBridge {
        func onComplete(cbWrapperPtr: UnsafeMutableRawPointer?, rustFnRetVal: __swift_bridge__$ResultPageBridgeAndToriiClientError) {
            let wrapper = Unmanaged<CbWrapper$ToriiClient$token_balances>.fromOpaque(cbWrapperPtr!).takeRetainedValue()
            switch rustFnRetVal.tag { case __swift_bridge__$ResultPageBridgeAndToriiClientError$ResultOk: wrapper.cb(.success(rustFnRetVal.payload.ok.intoSwiftRepr())) case __swift_bridge__$ResultPageBridgeAndToriiClientError$ResultErr: wrapper.cb(.failure(ToriiClientError(ptr: rustFnRetVal.payload.err))) default: fatalError() }
        }

        return try await withCheckedThrowingContinuation({ (continuation: CheckedContinuation<PageBridge, Error>) in
            let callback = { rustFnRetVal in
                continuation.resume(with: rustFnRetVal)
            }

            let wrapper = CbWrapper$ToriiClient$token_balances(cb: callback)
            let wrapperPtr = Unmanaged.passRetained(wrapper).toOpaque()

            __swift_bridge__$ToriiClient$token_balances(wrapperPtr, onComplete, ptr, { let val = account_addresses; val.isOwned = false; return val.ptr }(), { let val = contract_addresses; val.isOwned = false; return val.ptr }(), limit, { let rustString = cursor.intoRustString(); rustString.isOwned = false; return rustString.ptr }())
        })
    }
    class CbWrapper$ToriiClient$token_balances {
        var cb: (Result<PageBridge, Error>) -> ()
    
        public init(cb: @escaping (Result<PageBridge, Error>) -> ()) {
            self.cb = cb
        }
    }

    public func token_contracts<GenericIntoRustString: IntoRustString>(_ contract_addresses: RustVec<FeltBridge>, _ limit: UInt32, _ cursor: GenericIntoRustString) async throws -> PageBridge {
        func onComplete(cbWrapperPtr: UnsafeMutableRawPointer?, rustFnRetVal: __swift_bridge__$ResultPageBridgeAndToriiClientError) {
            let wrapper = Unmanaged<CbWrapper$ToriiClient$token_contracts>.fromOpaque(cbWrapperPtr!).takeRetainedValue()
            switch rustFnRetVal.tag { case __swift_bridge__$ResultPageBridgeAndToriiClientError$ResultOk: wrapper.cb(.success(rustFnRetVal.payload.ok.intoSwiftRepr())) case __swift_bridge__$ResultPageBridgeAndToriiClientError$ResultErr: wrapper.cb(.failure(ToriiClientError(ptr: rustFnRetVal.payload.err))) default: fatalError() }
        }

        return try await withCheckedThrowingContinuation({ (continuation: CheckedContinuation<PageBridge, Error>) in
            let callback = { rustFnRetVal in
                continuation.resume(with: rustFnRetVal)
            }

            let wrapper = CbWrapper$ToriiClient$token_contracts(cb: callback)
            let wrapperPtr = Unmanaged.passRetained(wrapper).toOpaque()

            __swift_bridge__$ToriiClient$token_contracts(wrapperPtr, onComplete, ptr, { let val = contract_addresses; val.isOwned = false; return val.ptr }(), limit, { let rustString = cursor.intoRustString(); rustString.isOwned = false; return rustString.ptr }())
        })
    }
    class CbWrapper$ToriiClient$token_contracts {
        var cb: (Result<PageBridge, Error>) -> ()
    
        public init(cb: @escaping (Result<PageBridge, Error>) -> ()) {
            self.cb = cb
        }
    }

    public func token_transfers<GenericIntoRustString: IntoRustString>(_ account_addresses: RustVec<FeltBridge>, _ contract_addresses: RustVec<FeltBridge>, _ limit: UInt32, _ cursor: GenericIntoRustString) async throws -> PageBridge {
        func onComplete(cbWrapperPtr: UnsafeMutableRawPointer?, rustFnRetVal: __swift_bridge__$ResultPageBridgeAndToriiClientError) {
            let wrapper = Unmanaged<CbWrapper$ToriiClient$token_transfers>.fromOpaque(cbWrapperPtr!).takeRetainedValue()
            switch rustFnRetVal.tag { case __swift_bridge__$ResultPageBridgeAndToriiClientError$ResultOk: wrapper.cb(.success(rustFnRetVal.payload.ok.intoSwiftRepr())) case __swift_bridge__$ResultPageBridgeAndToriiClientError$ResultErr: wrapper.cb(.failure(ToriiClientError(ptr: rustFnRetVal.payload.err))) default: fatalError() }
        }

        return try await withCheckedThrowingContinuation({ (continuation: CheckedContinuation<PageBridge, Error>) in
            let callback = { rustFnRetVal in
                continuation.resume(with: rustFnRetVal)
            }

            let wrapper = CbWrapper$ToriiClient$token_transfers(cb: callback)
            let wrapperPtr = Unmanaged.passRetained(wrapper).toOpaque()

            __swift_bridge__$ToriiClient$token_transfers(wrapperPtr, onComplete, ptr, { let val = account_addresses; val.isOwned = false; return val.ptr }(), { let val = contract_addresses; val.isOwned = false; return val.ptr }(), limit, { let rustString = cursor.intoRustString(); rustString.isOwned = false; return rustString.ptr }())
        })
    }
    class CbWrapper$ToriiClient$token_transfers {
        var cb: (Result<PageBridge, Error>) -> ()
    
        public init(cb: @escaping (Result<PageBridge, Error>) -> ()) {
            self.cb = cb
        }
    }

    public func transactions<GenericIntoRustString: IntoRustString>(_ filter_json: GenericIntoRustString, _ limit: UInt32, _ cursor: GenericIntoRustString) async throws -> PageBridge {
        func onComplete(cbWrapperPtr: UnsafeMutableRawPointer?, rustFnRetVal: __swift_bridge__$ResultPageBridgeAndToriiClientError) {
            let wrapper = Unmanaged<CbWrapper$ToriiClient$transactions>.fromOpaque(cbWrapperPtr!).takeRetainedValue()
            switch rustFnRetVal.tag { case __swift_bridge__$ResultPageBridgeAndToriiClientError$ResultOk: wrapper.cb(.success(rustFnRetVal.payload.ok.intoSwiftRepr())) case __swift_bridge__$ResultPageBridgeAndToriiClientError$ResultErr: wrapper.cb(.failure(ToriiClientError(ptr: rustFnRetVal.payload.err))) default: fatalError() }
        }

        return try await withCheckedThrowingContinuation({ (continuation: CheckedContinuation<PageBridge, Error>) in
            let callback = { rustFnRetVal in
                continuation.resume(with: rustFnRetVal)
            }

            let wrapper = CbWrapper$ToriiClient$transactions(cb: callback)
            let wrapperPtr = Unmanaged.passRetained(wrapper).toOpaque()

            __swift_bridge__$ToriiClient$transactions(wrapperPtr, onComplete, ptr, { let rustString = filter_json.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), limit, { let rustString = cursor.intoRustString(); rustString.isOwned = false; return rustString.ptr }())
        })
    }
    class CbWrapper$ToriiClient$transactions {
        var cb: (Result<PageBridge, Error>) -> ()
    
        public init(cb: @escaping (Result<PageBridge, Error>) -> ()) {
            self.cb = cb
        }
    }

    public func contracts(_ contract_addresses: RustVec<FeltBridge>) async throws -> RustString {
        func onComplete(cbWrapperPtr: UnsafeMutableRawPointer?, rustFnRetVal: __private__ResultPtrAndPtr) {
            let wrapper = Unmanaged<CbWrapper$ToriiClient$contracts>.fromOpaque(cbWrapperPtr!).takeRetainedValue()
            if rustFnRetVal.is_ok {
                wrapper.cb(.success(RustString(ptr: rustFnRetVal.ok_or_err!)))
            } else {
                wrapper.cb(.failure(ToriiClientError(ptr: rustFnRetVal.ok_or_err!)))
            }
        }

        return try await withCheckedThrowingContinuation({ (continuation: CheckedContinuation<RustString, Error>) in
            let callback = { rustFnRetVal in
                continuation.resume(with: rustFnRetVal)
            }

            let wrapper = CbWrapper$ToriiClient$contracts(cb: callback)
            let wrapperPtr = Unmanaged.passRetained(wrapper).toOpaque()

            __swift_bridge__$ToriiClient$contracts(wrapperPtr, onComplete, ptr, { let val = contract_addresses; val.isOwned = false; return val.ptr }())
        })
    }
    class CbWrapper$ToriiClient$contracts {
        var cb: (Result<RustString, Error>) -> ()
    
        public init(cb: @escaping (Result<RustString, Error>) -> ()) {
            self.cb = cb
        }
    }

    public func controllers<GenericIntoRustString: IntoRustString>(_ contract_addresses: RustVec<FeltBridge>, _ limit: UInt32, _ cursor: GenericIntoRustString) async throws -> PageBridge {
        func onComplete(cbWrapperPtr: UnsafeMutableRawPointer?, rustFnRetVal: __swift_bridge__$ResultPageBridgeAndToriiClientError) {
            let wrapper = Unmanaged<CbWrapper$ToriiClient$controllers>.fromOpaque(cbWrapperPtr!).takeRetainedValue()
            switch rustFnRetVal.tag { case __swift_bridge__$ResultPageBridgeAndToriiClientError$ResultOk: wrapper.cb(.success(rustFnRetVal.payload.ok.intoSwiftRepr())) case __swift_bridge__$ResultPageBridgeAndToriiClientError$ResultErr: wrapper.cb(.failure(ToriiClientError(ptr: rustFnRetVal.payload.err))) default: fatalError() }
        }

        return try await withCheckedThrowingContinuation({ (continuation: CheckedContinuation<PageBridge, Error>) in
            let callback = { rustFnRetVal in
                continuation.resume(with: rustFnRetVal)
            }

            let wrapper = CbWrapper$ToriiClient$controllers(cb: callback)
            let wrapperPtr = Unmanaged.passRetained(wrapper).toOpaque()

            __swift_bridge__$ToriiClient$controllers(wrapperPtr, onComplete, ptr, { let val = contract_addresses; val.isOwned = false; return val.ptr }(), limit, { let rustString = cursor.intoRustString(); rustString.isOwned = false; return rustString.ptr }())
        })
    }
    class CbWrapper$ToriiClient$controllers {
        var cb: (Result<PageBridge, Error>) -> ()
    
        public init(cb: @escaping (Result<PageBridge, Error>) -> ()) {
            self.cb = cb
        }
    }

    public func aggregations<GenericIntoRustString: IntoRustString>(_ aggregator_ids: RustVec<GenericIntoRustString>, _ entity_ids: RustVec<GenericIntoRustString>, _ limit: UInt32, _ cursor: GenericIntoRustString) async throws -> PageBridge {
        func onComplete(cbWrapperPtr: UnsafeMutableRawPointer?, rustFnRetVal: __swift_bridge__$ResultPageBridgeAndToriiClientError) {
            let wrapper = Unmanaged<CbWrapper$ToriiClient$aggregations>.fromOpaque(cbWrapperPtr!).takeRetainedValue()
            switch rustFnRetVal.tag { case __swift_bridge__$ResultPageBridgeAndToriiClientError$ResultOk: wrapper.cb(.success(rustFnRetVal.payload.ok.intoSwiftRepr())) case __swift_bridge__$ResultPageBridgeAndToriiClientError$ResultErr: wrapper.cb(.failure(ToriiClientError(ptr: rustFnRetVal.payload.err))) default: fatalError() }
        }

        return try await withCheckedThrowingContinuation({ (continuation: CheckedContinuation<PageBridge, Error>) in
            let callback = { rustFnRetVal in
                continuation.resume(with: rustFnRetVal)
            }

            let wrapper = CbWrapper$ToriiClient$aggregations(cb: callback)
            let wrapperPtr = Unmanaged.passRetained(wrapper).toOpaque()

            __swift_bridge__$ToriiClient$aggregations(wrapperPtr, onComplete, ptr, { let val = aggregator_ids; val.isOwned = false; return val.ptr }(), { let val = entity_ids; val.isOwned = false; return val.ptr }(), limit, { let rustString = cursor.intoRustString(); rustString.isOwned = false; return rustString.ptr }())
        })
    }
    class CbWrapper$ToriiClient$aggregations {
        var cb: (Result<PageBridge, Error>) -> ()
    
        public init(cb: @escaping (Result<PageBridge, Error>) -> ()) {
            self.cb = cb
        }
    }

    public func activities<GenericIntoRustString: IntoRustString>(_ world_addresses: RustVec<FeltBridge>, _ namespaces: RustVec<GenericIntoRustString>, _ caller_addresses: RustVec<FeltBridge>, _ limit: UInt32, _ cursor: GenericIntoRustString) async throws -> PageBridge {
        func onComplete(cbWrapperPtr: UnsafeMutableRawPointer?, rustFnRetVal: __swift_bridge__$ResultPageBridgeAndToriiClientError) {
            let wrapper = Unmanaged<CbWrapper$ToriiClient$activities>.fromOpaque(cbWrapperPtr!).takeRetainedValue()
            switch rustFnRetVal.tag { case __swift_bridge__$ResultPageBridgeAndToriiClientError$ResultOk: wrapper.cb(.success(rustFnRetVal.payload.ok.intoSwiftRepr())) case __swift_bridge__$ResultPageBridgeAndToriiClientError$ResultErr: wrapper.cb(.failure(ToriiClientError(ptr: rustFnRetVal.payload.err))) default: fatalError() }
        }

        return try await withCheckedThrowingContinuation({ (continuation: CheckedContinuation<PageBridge, Error>) in
            let callback = { rustFnRetVal in
                continuation.resume(with: rustFnRetVal)
            }

            let wrapper = CbWrapper$ToriiClient$activities(cb: callback)
            let wrapperPtr = Unmanaged.passRetained(wrapper).toOpaque()

            __swift_bridge__$ToriiClient$activities(wrapperPtr, onComplete, ptr, { let val = world_addresses; val.isOwned = false; return val.ptr }(), { let val = namespaces; val.isOwned = false; return val.ptr }(), { let val = caller_addresses; val.isOwned = false; return val.ptr }(), limit, { let rustString = cursor.intoRustString(); rustString.isOwned = false; return rustString.ptr }())
        })
    }
    class CbWrapper$ToriiClient$activities {
        var cb: (Result<PageBridge, Error>) -> ()
    
        public init(cb: @escaping (Result<PageBridge, Error>) -> ()) {
            self.cb = cb
        }
    }

    public func achievements<GenericIntoRustString: IntoRustString>(_ world_addresses: RustVec<FeltBridge>, _ namespaces: RustVec<GenericIntoRustString>, _ limit: UInt32, _ cursor: GenericIntoRustString) async throws -> PageBridge {
        func onComplete(cbWrapperPtr: UnsafeMutableRawPointer?, rustFnRetVal: __swift_bridge__$ResultPageBridgeAndToriiClientError) {
            let wrapper = Unmanaged<CbWrapper$ToriiClient$achievements>.fromOpaque(cbWrapperPtr!).takeRetainedValue()
            switch rustFnRetVal.tag { case __swift_bridge__$ResultPageBridgeAndToriiClientError$ResultOk: wrapper.cb(.success(rustFnRetVal.payload.ok.intoSwiftRepr())) case __swift_bridge__$ResultPageBridgeAndToriiClientError$ResultErr: wrapper.cb(.failure(ToriiClientError(ptr: rustFnRetVal.payload.err))) default: fatalError() }
        }

        return try await withCheckedThrowingContinuation({ (continuation: CheckedContinuation<PageBridge, Error>) in
            let callback = { rustFnRetVal in
                continuation.resume(with: rustFnRetVal)
            }

            let wrapper = CbWrapper$ToriiClient$achievements(cb: callback)
            let wrapperPtr = Unmanaged.passRetained(wrapper).toOpaque()

            __swift_bridge__$ToriiClient$achievements(wrapperPtr, onComplete, ptr, { let val = world_addresses; val.isOwned = false; return val.ptr }(), { let val = namespaces; val.isOwned = false; return val.ptr }(), limit, { let rustString = cursor.intoRustString(); rustString.isOwned = false; return rustString.ptr }())
        })
    }
    class CbWrapper$ToriiClient$achievements {
        var cb: (Result<PageBridge, Error>) -> ()
    
        public init(cb: @escaping (Result<PageBridge, Error>) -> ()) {
            self.cb = cb
        }
    }

    public func player_achievements<GenericIntoRustString: IntoRustString>(_ world_addresses: RustVec<FeltBridge>, _ namespaces: RustVec<GenericIntoRustString>, _ player_addresses: RustVec<FeltBridge>, _ limit: UInt32, _ cursor: GenericIntoRustString) async throws -> PageBridge {
        func onComplete(cbWrapperPtr: UnsafeMutableRawPointer?, rustFnRetVal: __swift_bridge__$ResultPageBridgeAndToriiClientError) {
            let wrapper = Unmanaged<CbWrapper$ToriiClient$player_achievements>.fromOpaque(cbWrapperPtr!).takeRetainedValue()
            switch rustFnRetVal.tag { case __swift_bridge__$ResultPageBridgeAndToriiClientError$ResultOk: wrapper.cb(.success(rustFnRetVal.payload.ok.intoSwiftRepr())) case __swift_bridge__$ResultPageBridgeAndToriiClientError$ResultErr: wrapper.cb(.failure(ToriiClientError(ptr: rustFnRetVal.payload.err))) default: fatalError() }
        }

        return try await withCheckedThrowingContinuation({ (continuation: CheckedContinuation<PageBridge, Error>) in
            let callback = { rustFnRetVal in
                continuation.resume(with: rustFnRetVal)
            }

            let wrapper = CbWrapper$ToriiClient$player_achievements(cb: callback)
            let wrapperPtr = Unmanaged.passRetained(wrapper).toOpaque()

            __swift_bridge__$ToriiClient$player_achievements(wrapperPtr, onComplete, ptr, { let val = world_addresses; val.isOwned = false; return val.ptr }(), { let val = namespaces; val.isOwned = false; return val.ptr }(), { let val = player_addresses; val.isOwned = false; return val.ptr }(), limit, { let rustString = cursor.intoRustString(); rustString.isOwned = false; return rustString.ptr }())
        })
    }
    class CbWrapper$ToriiClient$player_achievements {
        var cb: (Result<PageBridge, Error>) -> ()
    
        public init(cb: @escaping (Result<PageBridge, Error>) -> ()) {
            self.cb = cb
        }
    }

    public func sql<GenericIntoRustString: IntoRustString>(_ query: GenericIntoRustString) async throws -> RustString {
        func onComplete(cbWrapperPtr: UnsafeMutableRawPointer?, rustFnRetVal: __private__ResultPtrAndPtr) {
            let wrapper = Unmanaged<CbWrapper$ToriiClient$sql>.fromOpaque(cbWrapperPtr!).takeRetainedValue()
            if rustFnRetVal.is_ok {
                wrapper.cb(.success(RustString(ptr: rustFnRetVal.ok_or_err!)))
            } else {
                wrapper.cb(.failure(ToriiClientError(ptr: rustFnRetVal.ok_or_err!)))
            }
        }

        return try await withCheckedThrowingContinuation({ (continuation: CheckedContinuation<RustString, Error>) in
            let callback = { rustFnRetVal in
                continuation.resume(with: rustFnRetVal)
            }

            let wrapper = CbWrapper$ToriiClient$sql(cb: callback)
            let wrapperPtr = Unmanaged.passRetained(wrapper).toOpaque()

            __swift_bridge__$ToriiClient$sql(wrapperPtr, onComplete, ptr, { let rustString = query.intoRustString(); rustString.isOwned = false; return rustString.ptr }())
        })
    }
    class CbWrapper$ToriiClient$sql {
        var cb: (Result<RustString, Error>) -> ()
    
        public init(cb: @escaping (Result<RustString, Error>) -> ()) {
            self.cb = cb
        }
    }

    public func on_entity_updated<GenericIntoRustString: IntoRustString>(_ clause_json: GenericIntoRustString, _ world_addresses: RustVec<FeltBridge>) async throws -> Subscription {
        func onComplete(cbWrapperPtr: UnsafeMutableRawPointer?, rustFnRetVal: __private__ResultPtrAndPtr) {
            let wrapper = Unmanaged<CbWrapper$ToriiClient$on_entity_updated>.fromOpaque(cbWrapperPtr!).takeRetainedValue()
            if rustFnRetVal.is_ok {
                wrapper.cb(.success(Subscription(ptr: rustFnRetVal.ok_or_err!)))
            } else {
                wrapper.cb(.failure(ToriiClientError(ptr: rustFnRetVal.ok_or_err!)))
            }
        }

        return try await withCheckedThrowingContinuation({ (continuation: CheckedContinuation<Subscription, Error>) in
            let callback = { rustFnRetVal in
                continuation.resume(with: rustFnRetVal)
            }

            let wrapper = CbWrapper$ToriiClient$on_entity_updated(cb: callback)
            let wrapperPtr = Unmanaged.passRetained(wrapper).toOpaque()

            __swift_bridge__$ToriiClient$on_entity_updated(wrapperPtr, onComplete, ptr, { let rustString = clause_json.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let val = world_addresses; val.isOwned = false; return val.ptr }())
        })
    }
    class CbWrapper$ToriiClient$on_entity_updated {
        var cb: (Result<Subscription, Error>) -> ()
    
        public init(cb: @escaping (Result<Subscription, Error>) -> ()) {
            self.cb = cb
        }
    }

    public func on_starknet_event<GenericIntoRustString: IntoRustString>(_ keys_json: GenericIntoRustString) async throws -> Subscription {
        func onComplete(cbWrapperPtr: UnsafeMutableRawPointer?, rustFnRetVal: __private__ResultPtrAndPtr) {
            let wrapper = Unmanaged<CbWrapper$ToriiClient$on_starknet_event>.fromOpaque(cbWrapperPtr!).takeRetainedValue()
            if rustFnRetVal.is_ok {
                wrapper.cb(.success(Subscription(ptr: rustFnRetVal.ok_or_err!)))
            } else {
                wrapper.cb(.failure(ToriiClientError(ptr: rustFnRetVal.ok_or_err!)))
            }
        }

        return try await withCheckedThrowingContinuation({ (continuation: CheckedContinuation<Subscription, Error>) in
            let callback = { rustFnRetVal in
                continuation.resume(with: rustFnRetVal)
            }

            let wrapper = CbWrapper$ToriiClient$on_starknet_event(cb: callback)
            let wrapperPtr = Unmanaged.passRetained(wrapper).toOpaque()

            __swift_bridge__$ToriiClient$on_starknet_event(wrapperPtr, onComplete, ptr, { let rustString = keys_json.intoRustString(); rustString.isOwned = false; return rustString.ptr }())
        })
    }
    class CbWrapper$ToriiClient$on_starknet_event {
        var cb: (Result<Subscription, Error>) -> ()
    
        public init(cb: @escaping (Result<Subscription, Error>) -> ()) {
            self.cb = cb
        }
    }
}
extension ToriiClient: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_ToriiClient$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_ToriiClient$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ToriiClient) {
        __swift_bridge__$Vec_ToriiClient$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_ToriiClient$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (ToriiClient(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ToriiClientRef> {
        let pointer = __swift_bridge__$Vec_ToriiClient$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ToriiClientRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ToriiClientRefMut> {
        let pointer = __swift_bridge__$Vec_ToriiClient$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ToriiClientRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ToriiClientRef> {
        UnsafePointer<ToriiClientRef>(OpaquePointer(__swift_bridge__$Vec_ToriiClient$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_ToriiClient$len(vecPtr)
    }
}

public struct FeltBridge {
    public var hex: RustString

    public init(hex: RustString) {
        self.hex = hex
    }

    @inline(__always)
    func intoFfiRepr() -> __swift_bridge__$FeltBridge {
        { let val = self; return __swift_bridge__$FeltBridge(hex: { let rustString = val.hex.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); }()
    }
}
extension __swift_bridge__$FeltBridge {
    @inline(__always)
    func intoSwiftRepr() -> FeltBridge {
        { let val = self; return FeltBridge(hex: RustString(ptr: val.hex)); }()
    }
}
extension __swift_bridge__$Option$FeltBridge {
    @inline(__always)
    func intoSwiftRepr() -> Optional<FeltBridge> {
        if self.is_some {
            return self.val.intoSwiftRepr()
        } else {
            return nil
        }
    }

    @inline(__always)
    static func fromSwiftRepr(_ val: Optional<FeltBridge>) -> __swift_bridge__$Option$FeltBridge {
        if let v = val {
            return __swift_bridge__$Option$FeltBridge(is_some: true, val: v.intoFfiRepr())
        } else {
            return __swift_bridge__$Option$FeltBridge(is_some: false, val: __swift_bridge__$FeltBridge())
        }
    }
}
public struct PageBridge {
    public var data: RustString

    public init(data: RustString) {
        self.data = data
    }

    @inline(__always)
    func intoFfiRepr() -> __swift_bridge__$PageBridge {
        { let val = self; return __swift_bridge__$PageBridge(data: { let rustString = val.data.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); }()
    }
}
extension __swift_bridge__$PageBridge {
    @inline(__always)
    func intoSwiftRepr() -> PageBridge {
        { let val = self; return PageBridge(data: RustString(ptr: val.data)); }()
    }
}
extension __swift_bridge__$Option$PageBridge {
    @inline(__always)
    func intoSwiftRepr() -> Optional<PageBridge> {
        if self.is_some {
            return self.val.intoSwiftRepr()
        } else {
            return nil
        }
    }

    @inline(__always)
    static func fromSwiftRepr(_ val: Optional<PageBridge>) -> __swift_bridge__$Option$PageBridge {
        if let v = val {
            return __swift_bridge__$Option$PageBridge(is_some: true, val: v.intoFfiRepr())
        } else {
            return __swift_bridge__$Option$PageBridge(is_some: false, val: __swift_bridge__$PageBridge())
        }
    }
}
public struct WorldBridge {
    public var world_address: FeltBridge
    public var models_json: RustString

    public init(world_address: FeltBridge,models_json: RustString) {
        self.world_address = world_address
        self.models_json = models_json
    }

    @inline(__always)
    func intoFfiRepr() -> __swift_bridge__$WorldBridge {
        { let val = self; return __swift_bridge__$WorldBridge(world_address: val.world_address.intoFfiRepr(), models_json: { let rustString = val.models_json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); }()
    }
}
extension __swift_bridge__$WorldBridge {
    @inline(__always)
    func intoSwiftRepr() -> WorldBridge {
        { let val = self; return WorldBridge(world_address: val.world_address.intoSwiftRepr(), models_json: RustString(ptr: val.models_json)); }()
    }
}
extension __swift_bridge__$Option$WorldBridge {
    @inline(__always)
    func intoSwiftRepr() -> Optional<WorldBridge> {
        if self.is_some {
            return self.val.intoSwiftRepr()
        } else {
            return nil
        }
    }

    @inline(__always)
    static func fromSwiftRepr(_ val: Optional<WorldBridge>) -> __swift_bridge__$Option$WorldBridge {
        if let v = val {
            return __swift_bridge__$Option$WorldBridge(is_some: true, val: v.intoFfiRepr())
        } else {
            return __swift_bridge__$Option$WorldBridge(is_some: false, val: __swift_bridge__$WorldBridge())
        }
    }
}
public struct QueryBridge {
    public var limit: UInt32
    public var cursor: RustString
    public var world_addresses: RustVec<FeltBridge>
    public var dont_include_hashed_keys: Bool

    public init(limit: UInt32,cursor: RustString,world_addresses: RustVec<FeltBridge>,dont_include_hashed_keys: Bool) {
        self.limit = limit
        self.cursor = cursor
        self.world_addresses = world_addresses
        self.dont_include_hashed_keys = dont_include_hashed_keys
    }

    @inline(__always)
    func intoFfiRepr() -> __swift_bridge__$QueryBridge {
        { let val = self; return __swift_bridge__$QueryBridge(limit: val.limit, cursor: { let rustString = val.cursor.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), world_addresses: { let val = val.world_addresses; val.isOwned = false; return val.ptr }(), dont_include_hashed_keys: val.dont_include_hashed_keys); }()
    }
}
extension __swift_bridge__$QueryBridge {
    @inline(__always)
    func intoSwiftRepr() -> QueryBridge {
        { let val = self; return QueryBridge(limit: val.limit, cursor: RustString(ptr: val.cursor), world_addresses: RustVec(ptr: val.world_addresses), dont_include_hashed_keys: val.dont_include_hashed_keys); }()
    }
}
extension __swift_bridge__$Option$QueryBridge {
    @inline(__always)
    func intoSwiftRepr() -> Optional<QueryBridge> {
        if self.is_some {
            return self.val.intoSwiftRepr()
        } else {
            return nil
        }
    }

    @inline(__always)
    static func fromSwiftRepr(_ val: Optional<QueryBridge>) -> __swift_bridge__$Option$QueryBridge {
        if let v = val {
            return __swift_bridge__$Option$QueryBridge(is_some: true, val: v.intoFfiRepr())
        } else {
            return __swift_bridge__$Option$QueryBridge(is_some: false, val: __swift_bridge__$QueryBridge())
        }
    }
}
public struct U256Bridge {
    public var hex: RustString

    public init(hex: RustString) {
        self.hex = hex
    }

    @inline(__always)
    func intoFfiRepr() -> __swift_bridge__$U256Bridge {
        { let val = self; return __swift_bridge__$U256Bridge(hex: { let rustString = val.hex.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); }()
    }
}
extension __swift_bridge__$U256Bridge {
    @inline(__always)
    func intoSwiftRepr() -> U256Bridge {
        { let val = self; return U256Bridge(hex: RustString(ptr: val.hex)); }()
    }
}
extension __swift_bridge__$Option$U256Bridge {
    @inline(__always)
    func intoSwiftRepr() -> Optional<U256Bridge> {
        if self.is_some {
            return self.val.intoSwiftRepr()
        } else {
            return nil
        }
    }

    @inline(__always)
    static func fromSwiftRepr(_ val: Optional<U256Bridge>) -> __swift_bridge__$Option$U256Bridge {
        if let v = val {
            return __swift_bridge__$Option$U256Bridge(is_some: true, val: v.intoFfiRepr())
        } else {
            return __swift_bridge__$Option$U256Bridge(is_some: false, val: __swift_bridge__$U256Bridge())
        }
    }
}
public struct EntityBridge {
    public var world_address: FeltBridge
    public var hashed_keys: FeltBridge
    public var models_json: RustString
    public var created_at: UInt64
    public var updated_at: UInt64
    public var executed_at: UInt64

    public init(world_address: FeltBridge,hashed_keys: FeltBridge,models_json: RustString,created_at: UInt64,updated_at: UInt64,executed_at: UInt64) {
        self.world_address = world_address
        self.hashed_keys = hashed_keys
        self.models_json = models_json
        self.created_at = created_at
        self.updated_at = updated_at
        self.executed_at = executed_at
    }

    @inline(__always)
    func intoFfiRepr() -> __swift_bridge__$EntityBridge {
        { let val = self; return __swift_bridge__$EntityBridge(world_address: val.world_address.intoFfiRepr(), hashed_keys: val.hashed_keys.intoFfiRepr(), models_json: { let rustString = val.models_json.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), created_at: val.created_at, updated_at: val.updated_at, executed_at: val.executed_at); }()
    }
}
extension __swift_bridge__$EntityBridge {
    @inline(__always)
    func intoSwiftRepr() -> EntityBridge {
        { let val = self; return EntityBridge(world_address: val.world_address.intoSwiftRepr(), hashed_keys: val.hashed_keys.intoSwiftRepr(), models_json: RustString(ptr: val.models_json), created_at: val.created_at, updated_at: val.updated_at, executed_at: val.executed_at); }()
    }
}
extension __swift_bridge__$Option$EntityBridge {
    @inline(__always)
    func intoSwiftRepr() -> Optional<EntityBridge> {
        if self.is_some {
            return self.val.intoSwiftRepr()
        } else {
            return nil
        }
    }

    @inline(__always)
    static func fromSwiftRepr(_ val: Optional<EntityBridge>) -> __swift_bridge__$Option$EntityBridge {
        if let v = val {
            return __swift_bridge__$Option$EntityBridge(is_some: true, val: v.intoFfiRepr())
        } else {
            return __swift_bridge__$Option$EntityBridge(is_some: false, val: __swift_bridge__$EntityBridge())
        }
    }
}
public struct ModelBridge {
    public var world_address: FeltBridge
    public var namespace: RustString
    public var name: RustString
    public var selector: FeltBridge
    public var packed_size: UInt32
    public var unpacked_size: UInt32
    public var class_hash: FeltBridge
    public var contract_address: FeltBridge
    public var layout_json: RustString
    public var schema_json: RustString
    public var use_legacy_store: Bool

    public init(world_address: FeltBridge,namespace: RustString,name: RustString,selector: FeltBridge,packed_size: UInt32,unpacked_size: UInt32,class_hash: FeltBridge,contract_address: FeltBridge,layout_json: RustString,schema_json: RustString,use_legacy_store: Bool) {
        self.world_address = world_address
        self.namespace = namespace
        self.name = name
        self.selector = selector
        self.packed_size = packed_size
        self.unpacked_size = unpacked_size
        self.class_hash = class_hash
        self.contract_address = contract_address
        self.layout_json = layout_json
        self.schema_json = schema_json
        self.use_legacy_store = use_legacy_store
    }

    @inline(__always)
    func intoFfiRepr() -> __swift_bridge__$ModelBridge {
        { let val = self; return __swift_bridge__$ModelBridge(world_address: val.world_address.intoFfiRepr(), namespace: { let rustString = val.namespace.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), name: { let rustString = val.name.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), selector: val.selector.intoFfiRepr(), packed_size: val.packed_size, unpacked_size: val.unpacked_size, class_hash: val.class_hash.intoFfiRepr(), contract_address: val.contract_address.intoFfiRepr(), layout_json: { let rustString = val.layout_json.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), schema_json: { let rustString = val.schema_json.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), use_legacy_store: val.use_legacy_store); }()
    }
}
extension __swift_bridge__$ModelBridge {
    @inline(__always)
    func intoSwiftRepr() -> ModelBridge {
        { let val = self; return ModelBridge(world_address: val.world_address.intoSwiftRepr(), namespace: RustString(ptr: val.namespace), name: RustString(ptr: val.name), selector: val.selector.intoSwiftRepr(), packed_size: val.packed_size, unpacked_size: val.unpacked_size, class_hash: val.class_hash.intoSwiftRepr(), contract_address: val.contract_address.intoSwiftRepr(), layout_json: RustString(ptr: val.layout_json), schema_json: RustString(ptr: val.schema_json), use_legacy_store: val.use_legacy_store); }()
    }
}
extension __swift_bridge__$Option$ModelBridge {
    @inline(__always)
    func intoSwiftRepr() -> Optional<ModelBridge> {
        if self.is_some {
            return self.val.intoSwiftRepr()
        } else {
            return nil
        }
    }

    @inline(__always)
    static func fromSwiftRepr(_ val: Optional<ModelBridge>) -> __swift_bridge__$Option$ModelBridge {
        if let v = val {
            return __swift_bridge__$Option$ModelBridge(is_some: true, val: v.intoFfiRepr())
        } else {
            return __swift_bridge__$Option$ModelBridge(is_some: false, val: __swift_bridge__$ModelBridge())
        }
    }
}
public struct TokenBridge {
    public var contract_address: FeltBridge
    public var token_id: RustString
    public var name: RustString
    public var symbol: RustString
    public var decimals: UInt8
    public var metadata: RustString
    public var total_supply: RustString

    public init(contract_address: FeltBridge,token_id: RustString,name: RustString,symbol: RustString,decimals: UInt8,metadata: RustString,total_supply: RustString) {
        self.contract_address = contract_address
        self.token_id = token_id
        self.name = name
        self.symbol = symbol
        self.decimals = decimals
        self.metadata = metadata
        self.total_supply = total_supply
    }

    @inline(__always)
    func intoFfiRepr() -> __swift_bridge__$TokenBridge {
        { let val = self; return __swift_bridge__$TokenBridge(contract_address: val.contract_address.intoFfiRepr(), token_id: { let rustString = val.token_id.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), name: { let rustString = val.name.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), symbol: { let rustString = val.symbol.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), decimals: val.decimals, metadata: { let rustString = val.metadata.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), total_supply: { let rustString = val.total_supply.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); }()
    }
}
extension __swift_bridge__$TokenBridge {
    @inline(__always)
    func intoSwiftRepr() -> TokenBridge {
        { let val = self; return TokenBridge(contract_address: val.contract_address.intoSwiftRepr(), token_id: RustString(ptr: val.token_id), name: RustString(ptr: val.name), symbol: RustString(ptr: val.symbol), decimals: val.decimals, metadata: RustString(ptr: val.metadata), total_supply: RustString(ptr: val.total_supply)); }()
    }
}
extension __swift_bridge__$Option$TokenBridge {
    @inline(__always)
    func intoSwiftRepr() -> Optional<TokenBridge> {
        if self.is_some {
            return self.val.intoSwiftRepr()
        } else {
            return nil
        }
    }

    @inline(__always)
    static func fromSwiftRepr(_ val: Optional<TokenBridge>) -> __swift_bridge__$Option$TokenBridge {
        if let v = val {
            return __swift_bridge__$Option$TokenBridge(is_some: true, val: v.intoFfiRepr())
        } else {
            return __swift_bridge__$Option$TokenBridge(is_some: false, val: __swift_bridge__$TokenBridge())
        }
    }
}
public struct TokenBalanceBridge {
    public var balance: U256Bridge
    public var account_address: FeltBridge
    public var contract_address: FeltBridge
    public var token_id: RustString

    public init(balance: U256Bridge,account_address: FeltBridge,contract_address: FeltBridge,token_id: RustString) {
        self.balance = balance
        self.account_address = account_address
        self.contract_address = contract_address
        self.token_id = token_id
    }

    @inline(__always)
    func intoFfiRepr() -> __swift_bridge__$TokenBalanceBridge {
        { let val = self; return __swift_bridge__$TokenBalanceBridge(balance: val.balance.intoFfiRepr(), account_address: val.account_address.intoFfiRepr(), contract_address: val.contract_address.intoFfiRepr(), token_id: { let rustString = val.token_id.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); }()
    }
}
extension __swift_bridge__$TokenBalanceBridge {
    @inline(__always)
    func intoSwiftRepr() -> TokenBalanceBridge {
        { let val = self; return TokenBalanceBridge(balance: val.balance.intoSwiftRepr(), account_address: val.account_address.intoSwiftRepr(), contract_address: val.contract_address.intoSwiftRepr(), token_id: RustString(ptr: val.token_id)); }()
    }
}
extension __swift_bridge__$Option$TokenBalanceBridge {
    @inline(__always)
    func intoSwiftRepr() -> Optional<TokenBalanceBridge> {
        if self.is_some {
            return self.val.intoSwiftRepr()
        } else {
            return nil
        }
    }

    @inline(__always)
    static func fromSwiftRepr(_ val: Optional<TokenBalanceBridge>) -> __swift_bridge__$Option$TokenBalanceBridge {
        if let v = val {
            return __swift_bridge__$Option$TokenBalanceBridge(is_some: true, val: v.intoFfiRepr())
        } else {
            return __swift_bridge__$Option$TokenBalanceBridge(is_some: false, val: __swift_bridge__$TokenBalanceBridge())
        }
    }
}
public struct TokenContractBridge {
    public var contract_address: FeltBridge
    public var name: RustString
    public var symbol: RustString
    public var decimals: UInt8
    public var metadata: RustString
    public var token_metadata: RustString
    public var total_supply: RustString

    public init(contract_address: FeltBridge,name: RustString,symbol: RustString,decimals: UInt8,metadata: RustString,token_metadata: RustString,total_supply: RustString) {
        self.contract_address = contract_address
        self.name = name
        self.symbol = symbol
        self.decimals = decimals
        self.metadata = metadata
        self.token_metadata = token_metadata
        self.total_supply = total_supply
    }

    @inline(__always)
    func intoFfiRepr() -> __swift_bridge__$TokenContractBridge {
        { let val = self; return __swift_bridge__$TokenContractBridge(contract_address: val.contract_address.intoFfiRepr(), name: { let rustString = val.name.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), symbol: { let rustString = val.symbol.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), decimals: val.decimals, metadata: { let rustString = val.metadata.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), token_metadata: { let rustString = val.token_metadata.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), total_supply: { let rustString = val.total_supply.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); }()
    }
}
extension __swift_bridge__$TokenContractBridge {
    @inline(__always)
    func intoSwiftRepr() -> TokenContractBridge {
        { let val = self; return TokenContractBridge(contract_address: val.contract_address.intoSwiftRepr(), name: RustString(ptr: val.name), symbol: RustString(ptr: val.symbol), decimals: val.decimals, metadata: RustString(ptr: val.metadata), token_metadata: RustString(ptr: val.token_metadata), total_supply: RustString(ptr: val.total_supply)); }()
    }
}
extension __swift_bridge__$Option$TokenContractBridge {
    @inline(__always)
    func intoSwiftRepr() -> Optional<TokenContractBridge> {
        if self.is_some {
            return self.val.intoSwiftRepr()
        } else {
            return nil
        }
    }

    @inline(__always)
    static func fromSwiftRepr(_ val: Optional<TokenContractBridge>) -> __swift_bridge__$Option$TokenContractBridge {
        if let v = val {
            return __swift_bridge__$Option$TokenContractBridge(is_some: true, val: v.intoFfiRepr())
        } else {
            return __swift_bridge__$Option$TokenContractBridge(is_some: false, val: __swift_bridge__$TokenContractBridge())
        }
    }
}
public struct TokenTransferBridge {
    public var id: RustString
    public var contract_address: FeltBridge
    public var from_address: FeltBridge
    public var to_address: FeltBridge
    public var amount: U256Bridge
    public var token_id: RustString
    public var executed_at: UInt64
    public var event_id: RustString

    public init(id: RustString,contract_address: FeltBridge,from_address: FeltBridge,to_address: FeltBridge,amount: U256Bridge,token_id: RustString,executed_at: UInt64,event_id: RustString) {
        self.id = id
        self.contract_address = contract_address
        self.from_address = from_address
        self.to_address = to_address
        self.amount = amount
        self.token_id = token_id
        self.executed_at = executed_at
        self.event_id = event_id
    }

    @inline(__always)
    func intoFfiRepr() -> __swift_bridge__$TokenTransferBridge {
        { let val = self; return __swift_bridge__$TokenTransferBridge(id: { let rustString = val.id.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), contract_address: val.contract_address.intoFfiRepr(), from_address: val.from_address.intoFfiRepr(), to_address: val.to_address.intoFfiRepr(), amount: val.amount.intoFfiRepr(), token_id: { let rustString = val.token_id.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), executed_at: val.executed_at, event_id: { let rustString = val.event_id.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); }()
    }
}
extension __swift_bridge__$TokenTransferBridge {
    @inline(__always)
    func intoSwiftRepr() -> TokenTransferBridge {
        { let val = self; return TokenTransferBridge(id: RustString(ptr: val.id), contract_address: val.contract_address.intoSwiftRepr(), from_address: val.from_address.intoSwiftRepr(), to_address: val.to_address.intoSwiftRepr(), amount: val.amount.intoSwiftRepr(), token_id: RustString(ptr: val.token_id), executed_at: val.executed_at, event_id: RustString(ptr: val.event_id)); }()
    }
}
extension __swift_bridge__$Option$TokenTransferBridge {
    @inline(__always)
    func intoSwiftRepr() -> Optional<TokenTransferBridge> {
        if self.is_some {
            return self.val.intoSwiftRepr()
        } else {
            return nil
        }
    }

    @inline(__always)
    static func fromSwiftRepr(_ val: Optional<TokenTransferBridge>) -> __swift_bridge__$Option$TokenTransferBridge {
        if let v = val {
            return __swift_bridge__$Option$TokenTransferBridge(is_some: true, val: v.intoFfiRepr())
        } else {
            return __swift_bridge__$Option$TokenTransferBridge(is_some: false, val: __swift_bridge__$TokenTransferBridge())
        }
    }
}
public struct ControllerBridge {
    public var address: FeltBridge
    public var username: RustString
    public var deployed_at_timestamp: UInt64

    public init(address: FeltBridge,username: RustString,deployed_at_timestamp: UInt64) {
        self.address = address
        self.username = username
        self.deployed_at_timestamp = deployed_at_timestamp
    }

    @inline(__always)
    func intoFfiRepr() -> __swift_bridge__$ControllerBridge {
        { let val = self; return __swift_bridge__$ControllerBridge(address: val.address.intoFfiRepr(), username: { let rustString = val.username.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), deployed_at_timestamp: val.deployed_at_timestamp); }()
    }
}
extension __swift_bridge__$ControllerBridge {
    @inline(__always)
    func intoSwiftRepr() -> ControllerBridge {
        { let val = self; return ControllerBridge(address: val.address.intoSwiftRepr(), username: RustString(ptr: val.username), deployed_at_timestamp: val.deployed_at_timestamp); }()
    }
}
extension __swift_bridge__$Option$ControllerBridge {
    @inline(__always)
    func intoSwiftRepr() -> Optional<ControllerBridge> {
        if self.is_some {
            return self.val.intoSwiftRepr()
        } else {
            return nil
        }
    }

    @inline(__always)
    static func fromSwiftRepr(_ val: Optional<ControllerBridge>) -> __swift_bridge__$Option$ControllerBridge {
        if let v = val {
            return __swift_bridge__$Option$ControllerBridge(is_some: true, val: v.intoFfiRepr())
        } else {
            return __swift_bridge__$Option$ControllerBridge(is_some: false, val: __swift_bridge__$ControllerBridge())
        }
    }
}
public struct ContractBridge {
    public var contract_address: FeltBridge
    public var contract_type: RustString
    public var head: RustString
    public var tps: RustString
    public var last_block_timestamp: RustString
    public var last_pending_block_tx: RustString
    public var updated_at: UInt64
    public var created_at: UInt64

    public init(contract_address: FeltBridge,contract_type: RustString,head: RustString,tps: RustString,last_block_timestamp: RustString,last_pending_block_tx: RustString,updated_at: UInt64,created_at: UInt64) {
        self.contract_address = contract_address
        self.contract_type = contract_type
        self.head = head
        self.tps = tps
        self.last_block_timestamp = last_block_timestamp
        self.last_pending_block_tx = last_pending_block_tx
        self.updated_at = updated_at
        self.created_at = created_at
    }

    @inline(__always)
    func intoFfiRepr() -> __swift_bridge__$ContractBridge {
        { let val = self; return __swift_bridge__$ContractBridge(contract_address: val.contract_address.intoFfiRepr(), contract_type: { let rustString = val.contract_type.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), head: { let rustString = val.head.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), tps: { let rustString = val.tps.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), last_block_timestamp: { let rustString = val.last_block_timestamp.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), last_pending_block_tx: { let rustString = val.last_pending_block_tx.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), updated_at: val.updated_at, created_at: val.created_at); }()
    }
}
extension __swift_bridge__$ContractBridge {
    @inline(__always)
    func intoSwiftRepr() -> ContractBridge {
        { let val = self; return ContractBridge(contract_address: val.contract_address.intoSwiftRepr(), contract_type: RustString(ptr: val.contract_type), head: RustString(ptr: val.head), tps: RustString(ptr: val.tps), last_block_timestamp: RustString(ptr: val.last_block_timestamp), last_pending_block_tx: RustString(ptr: val.last_pending_block_tx), updated_at: val.updated_at, created_at: val.created_at); }()
    }
}
extension __swift_bridge__$Option$ContractBridge {
    @inline(__always)
    func intoSwiftRepr() -> Optional<ContractBridge> {
        if self.is_some {
            return self.val.intoSwiftRepr()
        } else {
            return nil
        }
    }

    @inline(__always)
    static func fromSwiftRepr(_ val: Optional<ContractBridge>) -> __swift_bridge__$Option$ContractBridge {
        if let v = val {
            return __swift_bridge__$Option$ContractBridge(is_some: true, val: v.intoFfiRepr())
        } else {
            return __swift_bridge__$Option$ContractBridge(is_some: false, val: __swift_bridge__$ContractBridge())
        }
    }
}
public struct TransactionBridge {
    public var transaction_hash: FeltBridge
    public var sender_address: FeltBridge
    public var calldata: RustVec<FeltBridge>
    public var max_fee: FeltBridge
    public var signature: RustVec<FeltBridge>
    public var nonce: FeltBridge
    public var block_number: UInt64
    public var transaction_type: RustString
    public var block_timestamp: UInt64
    public var calls_json: RustString
    public var unique_models: RustVec<FeltBridge>

    public init(transaction_hash: FeltBridge,sender_address: FeltBridge,calldata: RustVec<FeltBridge>,max_fee: FeltBridge,signature: RustVec<FeltBridge>,nonce: FeltBridge,block_number: UInt64,transaction_type: RustString,block_timestamp: UInt64,calls_json: RustString,unique_models: RustVec<FeltBridge>) {
        self.transaction_hash = transaction_hash
        self.sender_address = sender_address
        self.calldata = calldata
        self.max_fee = max_fee
        self.signature = signature
        self.nonce = nonce
        self.block_number = block_number
        self.transaction_type = transaction_type
        self.block_timestamp = block_timestamp
        self.calls_json = calls_json
        self.unique_models = unique_models
    }

    @inline(__always)
    func intoFfiRepr() -> __swift_bridge__$TransactionBridge {
        { let val = self; return __swift_bridge__$TransactionBridge(transaction_hash: val.transaction_hash.intoFfiRepr(), sender_address: val.sender_address.intoFfiRepr(), calldata: { let val = val.calldata; val.isOwned = false; return val.ptr }(), max_fee: val.max_fee.intoFfiRepr(), signature: { let val = val.signature; val.isOwned = false; return val.ptr }(), nonce: val.nonce.intoFfiRepr(), block_number: val.block_number, transaction_type: { let rustString = val.transaction_type.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), block_timestamp: val.block_timestamp, calls_json: { let rustString = val.calls_json.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), unique_models: { let val = val.unique_models; val.isOwned = false; return val.ptr }()); }()
    }
}
extension __swift_bridge__$TransactionBridge {
    @inline(__always)
    func intoSwiftRepr() -> TransactionBridge {
        { let val = self; return TransactionBridge(transaction_hash: val.transaction_hash.intoSwiftRepr(), sender_address: val.sender_address.intoSwiftRepr(), calldata: RustVec(ptr: val.calldata), max_fee: val.max_fee.intoSwiftRepr(), signature: RustVec(ptr: val.signature), nonce: val.nonce.intoSwiftRepr(), block_number: val.block_number, transaction_type: RustString(ptr: val.transaction_type), block_timestamp: val.block_timestamp, calls_json: RustString(ptr: val.calls_json), unique_models: RustVec(ptr: val.unique_models)); }()
    }
}
extension __swift_bridge__$Option$TransactionBridge {
    @inline(__always)
    func intoSwiftRepr() -> Optional<TransactionBridge> {
        if self.is_some {
            return self.val.intoSwiftRepr()
        } else {
            return nil
        }
    }

    @inline(__always)
    static func fromSwiftRepr(_ val: Optional<TransactionBridge>) -> __swift_bridge__$Option$TransactionBridge {
        if let v = val {
            return __swift_bridge__$Option$TransactionBridge(is_some: true, val: v.intoFfiRepr())
        } else {
            return __swift_bridge__$Option$TransactionBridge(is_some: false, val: __swift_bridge__$TransactionBridge())
        }
    }
}
public struct TransactionCallBridge {
    public var contract_address: FeltBridge
    public var entrypoint: RustString
    public var calldata: RustVec<FeltBridge>
    public var call_type: RustString
    public var caller_address: FeltBridge

    public init(contract_address: FeltBridge,entrypoint: RustString,calldata: RustVec<FeltBridge>,call_type: RustString,caller_address: FeltBridge) {
        self.contract_address = contract_address
        self.entrypoint = entrypoint
        self.calldata = calldata
        self.call_type = call_type
        self.caller_address = caller_address
    }

    @inline(__always)
    func intoFfiRepr() -> __swift_bridge__$TransactionCallBridge {
        { let val = self; return __swift_bridge__$TransactionCallBridge(contract_address: val.contract_address.intoFfiRepr(), entrypoint: { let rustString = val.entrypoint.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), calldata: { let val = val.calldata; val.isOwned = false; return val.ptr }(), call_type: { let rustString = val.call_type.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), caller_address: val.caller_address.intoFfiRepr()); }()
    }
}
extension __swift_bridge__$TransactionCallBridge {
    @inline(__always)
    func intoSwiftRepr() -> TransactionCallBridge {
        { let val = self; return TransactionCallBridge(contract_address: val.contract_address.intoSwiftRepr(), entrypoint: RustString(ptr: val.entrypoint), calldata: RustVec(ptr: val.calldata), call_type: RustString(ptr: val.call_type), caller_address: val.caller_address.intoSwiftRepr()); }()
    }
}
extension __swift_bridge__$Option$TransactionCallBridge {
    @inline(__always)
    func intoSwiftRepr() -> Optional<TransactionCallBridge> {
        if self.is_some {
            return self.val.intoSwiftRepr()
        } else {
            return nil
        }
    }

    @inline(__always)
    static func fromSwiftRepr(_ val: Optional<TransactionCallBridge>) -> __swift_bridge__$Option$TransactionCallBridge {
        if let v = val {
            return __swift_bridge__$Option$TransactionCallBridge(is_some: true, val: v.intoFfiRepr())
        } else {
            return __swift_bridge__$Option$TransactionCallBridge(is_some: false, val: __swift_bridge__$TransactionCallBridge())
        }
    }
}
public struct EventBridge {
    public var keys: RustVec<FeltBridge>
    public var data: RustVec<FeltBridge>
    public var transaction_hash: FeltBridge

    public init(keys: RustVec<FeltBridge>,data: RustVec<FeltBridge>,transaction_hash: FeltBridge) {
        self.keys = keys
        self.data = data
        self.transaction_hash = transaction_hash
    }

    @inline(__always)
    func intoFfiRepr() -> __swift_bridge__$EventBridge {
        { let val = self; return __swift_bridge__$EventBridge(keys: { let val = val.keys; val.isOwned = false; return val.ptr }(), data: { let val = val.data; val.isOwned = false; return val.ptr }(), transaction_hash: val.transaction_hash.intoFfiRepr()); }()
    }
}
extension __swift_bridge__$EventBridge {
    @inline(__always)
    func intoSwiftRepr() -> EventBridge {
        { let val = self; return EventBridge(keys: RustVec(ptr: val.keys), data: RustVec(ptr: val.data), transaction_hash: val.transaction_hash.intoSwiftRepr()); }()
    }
}
extension __swift_bridge__$Option$EventBridge {
    @inline(__always)
    func intoSwiftRepr() -> Optional<EventBridge> {
        if self.is_some {
            return self.val.intoSwiftRepr()
        } else {
            return nil
        }
    }

    @inline(__always)
    static func fromSwiftRepr(_ val: Optional<EventBridge>) -> __swift_bridge__$Option$EventBridge {
        if let v = val {
            return __swift_bridge__$Option$EventBridge(is_some: true, val: v.intoFfiRepr())
        } else {
            return __swift_bridge__$Option$EventBridge(is_some: false, val: __swift_bridge__$EventBridge())
        }
    }
}
public struct AggregationEntryBridge {
    public var id: RustString
    public var aggregator_id: RustString
    public var entity_id: RustString
    public var value: U256Bridge
    public var display_value: RustString
    public var position: UInt64
    public var model_id: RustString
    public var created_at: UInt64
    public var updated_at: UInt64

    public init(id: RustString,aggregator_id: RustString,entity_id: RustString,value: U256Bridge,display_value: RustString,position: UInt64,model_id: RustString,created_at: UInt64,updated_at: UInt64) {
        self.id = id
        self.aggregator_id = aggregator_id
        self.entity_id = entity_id
        self.value = value
        self.display_value = display_value
        self.position = position
        self.model_id = model_id
        self.created_at = created_at
        self.updated_at = updated_at
    }

    @inline(__always)
    func intoFfiRepr() -> __swift_bridge__$AggregationEntryBridge {
        { let val = self; return __swift_bridge__$AggregationEntryBridge(id: { let rustString = val.id.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), aggregator_id: { let rustString = val.aggregator_id.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), entity_id: { let rustString = val.entity_id.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), value: val.value.intoFfiRepr(), display_value: { let rustString = val.display_value.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), position: val.position, model_id: { let rustString = val.model_id.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), created_at: val.created_at, updated_at: val.updated_at); }()
    }
}
extension __swift_bridge__$AggregationEntryBridge {
    @inline(__always)
    func intoSwiftRepr() -> AggregationEntryBridge {
        { let val = self; return AggregationEntryBridge(id: RustString(ptr: val.id), aggregator_id: RustString(ptr: val.aggregator_id), entity_id: RustString(ptr: val.entity_id), value: val.value.intoSwiftRepr(), display_value: RustString(ptr: val.display_value), position: val.position, model_id: RustString(ptr: val.model_id), created_at: val.created_at, updated_at: val.updated_at); }()
    }
}
extension __swift_bridge__$Option$AggregationEntryBridge {
    @inline(__always)
    func intoSwiftRepr() -> Optional<AggregationEntryBridge> {
        if self.is_some {
            return self.val.intoSwiftRepr()
        } else {
            return nil
        }
    }

    @inline(__always)
    static func fromSwiftRepr(_ val: Optional<AggregationEntryBridge>) -> __swift_bridge__$Option$AggregationEntryBridge {
        if let v = val {
            return __swift_bridge__$Option$AggregationEntryBridge(is_some: true, val: v.intoFfiRepr())
        } else {
            return __swift_bridge__$Option$AggregationEntryBridge(is_some: false, val: __swift_bridge__$AggregationEntryBridge())
        }
    }
}
public struct ActivityBridge {
    public var id: RustString
    public var world_address: FeltBridge
    public var namespace: RustString
    public var caller_address: FeltBridge
    public var session_start: UInt64
    public var session_end: UInt64
    public var action_count: UInt32
    public var actions_json: RustString
    public var updated_at: UInt64

    public init(id: RustString,world_address: FeltBridge,namespace: RustString,caller_address: FeltBridge,session_start: UInt64,session_end: UInt64,action_count: UInt32,actions_json: RustString,updated_at: UInt64) {
        self.id = id
        self.world_address = world_address
        self.namespace = namespace
        self.caller_address = caller_address
        self.session_start = session_start
        self.session_end = session_end
        self.action_count = action_count
        self.actions_json = actions_json
        self.updated_at = updated_at
    }

    @inline(__always)
    func intoFfiRepr() -> __swift_bridge__$ActivityBridge {
        { let val = self; return __swift_bridge__$ActivityBridge(id: { let rustString = val.id.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), world_address: val.world_address.intoFfiRepr(), namespace: { let rustString = val.namespace.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), caller_address: val.caller_address.intoFfiRepr(), session_start: val.session_start, session_end: val.session_end, action_count: val.action_count, actions_json: { let rustString = val.actions_json.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), updated_at: val.updated_at); }()
    }
}
extension __swift_bridge__$ActivityBridge {
    @inline(__always)
    func intoSwiftRepr() -> ActivityBridge {
        { let val = self; return ActivityBridge(id: RustString(ptr: val.id), world_address: val.world_address.intoSwiftRepr(), namespace: RustString(ptr: val.namespace), caller_address: val.caller_address.intoSwiftRepr(), session_start: val.session_start, session_end: val.session_end, action_count: val.action_count, actions_json: RustString(ptr: val.actions_json), updated_at: val.updated_at); }()
    }
}
extension __swift_bridge__$Option$ActivityBridge {
    @inline(__always)
    func intoSwiftRepr() -> Optional<ActivityBridge> {
        if self.is_some {
            return self.val.intoSwiftRepr()
        } else {
            return nil
        }
    }

    @inline(__always)
    static func fromSwiftRepr(_ val: Optional<ActivityBridge>) -> __swift_bridge__$Option$ActivityBridge {
        if let v = val {
            return __swift_bridge__$Option$ActivityBridge(is_some: true, val: v.intoFfiRepr())
        } else {
            return __swift_bridge__$Option$ActivityBridge(is_some: false, val: __swift_bridge__$ActivityBridge())
        }
    }
}
public struct ActionCountBridge {
    public var action_name: RustString
    public var count: UInt32

    public init(action_name: RustString,count: UInt32) {
        self.action_name = action_name
        self.count = count
    }

    @inline(__always)
    func intoFfiRepr() -> __swift_bridge__$ActionCountBridge {
        { let val = self; return __swift_bridge__$ActionCountBridge(action_name: { let rustString = val.action_name.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), count: val.count); }()
    }
}
extension __swift_bridge__$ActionCountBridge {
    @inline(__always)
    func intoSwiftRepr() -> ActionCountBridge {
        { let val = self; return ActionCountBridge(action_name: RustString(ptr: val.action_name), count: val.count); }()
    }
}
extension __swift_bridge__$Option$ActionCountBridge {
    @inline(__always)
    func intoSwiftRepr() -> Optional<ActionCountBridge> {
        if self.is_some {
            return self.val.intoSwiftRepr()
        } else {
            return nil
        }
    }

    @inline(__always)
    static func fromSwiftRepr(_ val: Optional<ActionCountBridge>) -> __swift_bridge__$Option$ActionCountBridge {
        if let v = val {
            return __swift_bridge__$Option$ActionCountBridge(is_some: true, val: v.intoFfiRepr())
        } else {
            return __swift_bridge__$Option$ActionCountBridge(is_some: false, val: __swift_bridge__$ActionCountBridge())
        }
    }
}
public struct AchievementBridge {
    public var id: RustString
    public var world_address: FeltBridge
    public var namespace: RustString
    public var entity_id: RustString
    public var hidden: Bool
    public var index: UInt32
    public var points: UInt32
    public var start: RustString
    public var end: RustString
    public var group: RustString
    public var icon: RustString
    public var title: RustString
    public var description: RustString
    public var tasks_json: RustString
    public var data: RustString
    public var total_completions: UInt32
    public var completion_rate: Double
    public var created_at: UInt64
    public var updated_at: UInt64

    public init(id: RustString,world_address: FeltBridge,namespace: RustString,entity_id: RustString,hidden: Bool,index: UInt32,points: UInt32,start: RustString,end: RustString,group: RustString,icon: RustString,title: RustString,description: RustString,tasks_json: RustString,data: RustString,total_completions: UInt32,completion_rate: Double,created_at: UInt64,updated_at: UInt64) {
        self.id = id
        self.world_address = world_address
        self.namespace = namespace
        self.entity_id = entity_id
        self.hidden = hidden
        self.index = index
        self.points = points
        self.start = start
        self.end = end
        self.group = group
        self.icon = icon
        self.title = title
        self.description = description
        self.tasks_json = tasks_json
        self.data = data
        self.total_completions = total_completions
        self.completion_rate = completion_rate
        self.created_at = created_at
        self.updated_at = updated_at
    }

    @inline(__always)
    func intoFfiRepr() -> __swift_bridge__$AchievementBridge {
        { let val = self; return __swift_bridge__$AchievementBridge(id: { let rustString = val.id.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), world_address: val.world_address.intoFfiRepr(), namespace: { let rustString = val.namespace.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), entity_id: { let rustString = val.entity_id.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), hidden: val.hidden, index: val.index, points: val.points, start: { let rustString = val.start.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), end: { let rustString = val.end.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), group: { let rustString = val.group.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), icon: { let rustString = val.icon.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), title: { let rustString = val.title.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), description: { let rustString = val.description.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), tasks_json: { let rustString = val.tasks_json.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), data: { let rustString = val.data.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), total_completions: val.total_completions, completion_rate: val.completion_rate, created_at: val.created_at, updated_at: val.updated_at); }()
    }
}
extension __swift_bridge__$AchievementBridge {
    @inline(__always)
    func intoSwiftRepr() -> AchievementBridge {
        { let val = self; return AchievementBridge(id: RustString(ptr: val.id), world_address: val.world_address.intoSwiftRepr(), namespace: RustString(ptr: val.namespace), entity_id: RustString(ptr: val.entity_id), hidden: val.hidden, index: val.index, points: val.points, start: RustString(ptr: val.start), end: RustString(ptr: val.end), group: RustString(ptr: val.group), icon: RustString(ptr: val.icon), title: RustString(ptr: val.title), description: RustString(ptr: val.description), tasks_json: RustString(ptr: val.tasks_json), data: RustString(ptr: val.data), total_completions: val.total_completions, completion_rate: val.completion_rate, created_at: val.created_at, updated_at: val.updated_at); }()
    }
}
extension __swift_bridge__$Option$AchievementBridge {
    @inline(__always)
    func intoSwiftRepr() -> Optional<AchievementBridge> {
        if self.is_some {
            return self.val.intoSwiftRepr()
        } else {
            return nil
        }
    }

    @inline(__always)
    static func fromSwiftRepr(_ val: Optional<AchievementBridge>) -> __swift_bridge__$Option$AchievementBridge {
        if let v = val {
            return __swift_bridge__$Option$AchievementBridge(is_some: true, val: v.intoFfiRepr())
        } else {
            return __swift_bridge__$Option$AchievementBridge(is_some: false, val: __swift_bridge__$AchievementBridge())
        }
    }
}
public struct AchievementTaskBridge {
    public var task_id: RustString
    public var description: RustString
    public var total: UInt32
    public var total_completions: UInt32
    public var completion_rate: Double
    public var created_at: UInt64

    public init(task_id: RustString,description: RustString,total: UInt32,total_completions: UInt32,completion_rate: Double,created_at: UInt64) {
        self.task_id = task_id
        self.description = description
        self.total = total
        self.total_completions = total_completions
        self.completion_rate = completion_rate
        self.created_at = created_at
    }

    @inline(__always)
    func intoFfiRepr() -> __swift_bridge__$AchievementTaskBridge {
        { let val = self; return __swift_bridge__$AchievementTaskBridge(task_id: { let rustString = val.task_id.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), description: { let rustString = val.description.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), total: val.total, total_completions: val.total_completions, completion_rate: val.completion_rate, created_at: val.created_at); }()
    }
}
extension __swift_bridge__$AchievementTaskBridge {
    @inline(__always)
    func intoSwiftRepr() -> AchievementTaskBridge {
        { let val = self; return AchievementTaskBridge(task_id: RustString(ptr: val.task_id), description: RustString(ptr: val.description), total: val.total, total_completions: val.total_completions, completion_rate: val.completion_rate, created_at: val.created_at); }()
    }
}
extension __swift_bridge__$Option$AchievementTaskBridge {
    @inline(__always)
    func intoSwiftRepr() -> Optional<AchievementTaskBridge> {
        if self.is_some {
            return self.val.intoSwiftRepr()
        } else {
            return nil
        }
    }

    @inline(__always)
    static func fromSwiftRepr(_ val: Optional<AchievementTaskBridge>) -> __swift_bridge__$Option$AchievementTaskBridge {
        if let v = val {
            return __swift_bridge__$Option$AchievementTaskBridge(is_some: true, val: v.intoFfiRepr())
        } else {
            return __swift_bridge__$Option$AchievementTaskBridge(is_some: false, val: __swift_bridge__$AchievementTaskBridge())
        }
    }
}
public struct PlayerAchievementEntryBridge {
    public var player_address: FeltBridge
    public var stats: PlayerAchievementStatsBridge
    public var achievements_json: RustString

    public init(player_address: FeltBridge,stats: PlayerAchievementStatsBridge,achievements_json: RustString) {
        self.player_address = player_address
        self.stats = stats
        self.achievements_json = achievements_json
    }

    @inline(__always)
    func intoFfiRepr() -> __swift_bridge__$PlayerAchievementEntryBridge {
        { let val = self; return __swift_bridge__$PlayerAchievementEntryBridge(player_address: val.player_address.intoFfiRepr(), stats: val.stats.intoFfiRepr(), achievements_json: { let rustString = val.achievements_json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); }()
    }
}
extension __swift_bridge__$PlayerAchievementEntryBridge {
    @inline(__always)
    func intoSwiftRepr() -> PlayerAchievementEntryBridge {
        { let val = self; return PlayerAchievementEntryBridge(player_address: val.player_address.intoSwiftRepr(), stats: val.stats.intoSwiftRepr(), achievements_json: RustString(ptr: val.achievements_json)); }()
    }
}
extension __swift_bridge__$Option$PlayerAchievementEntryBridge {
    @inline(__always)
    func intoSwiftRepr() -> Optional<PlayerAchievementEntryBridge> {
        if self.is_some {
            return self.val.intoSwiftRepr()
        } else {
            return nil
        }
    }

    @inline(__always)
    static func fromSwiftRepr(_ val: Optional<PlayerAchievementEntryBridge>) -> __swift_bridge__$Option$PlayerAchievementEntryBridge {
        if let v = val {
            return __swift_bridge__$Option$PlayerAchievementEntryBridge(is_some: true, val: v.intoFfiRepr())
        } else {
            return __swift_bridge__$Option$PlayerAchievementEntryBridge(is_some: false, val: __swift_bridge__$PlayerAchievementEntryBridge())
        }
    }
}
public struct PlayerAchievementStatsBridge {
    public var total_points: UInt32
    public var completed_achievements: UInt32
    public var total_achievements: UInt32
    public var completion_percentage: Double
    public var last_achievement_at: RustString
    public var created_at: UInt64
    public var updated_at: UInt64

    public init(total_points: UInt32,completed_achievements: UInt32,total_achievements: UInt32,completion_percentage: Double,last_achievement_at: RustString,created_at: UInt64,updated_at: UInt64) {
        self.total_points = total_points
        self.completed_achievements = completed_achievements
        self.total_achievements = total_achievements
        self.completion_percentage = completion_percentage
        self.last_achievement_at = last_achievement_at
        self.created_at = created_at
        self.updated_at = updated_at
    }

    @inline(__always)
    func intoFfiRepr() -> __swift_bridge__$PlayerAchievementStatsBridge {
        { let val = self; return __swift_bridge__$PlayerAchievementStatsBridge(total_points: val.total_points, completed_achievements: val.completed_achievements, total_achievements: val.total_achievements, completion_percentage: val.completion_percentage, last_achievement_at: { let rustString = val.last_achievement_at.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), created_at: val.created_at, updated_at: val.updated_at); }()
    }
}
extension __swift_bridge__$PlayerAchievementStatsBridge {
    @inline(__always)
    func intoSwiftRepr() -> PlayerAchievementStatsBridge {
        { let val = self; return PlayerAchievementStatsBridge(total_points: val.total_points, completed_achievements: val.completed_achievements, total_achievements: val.total_achievements, completion_percentage: val.completion_percentage, last_achievement_at: RustString(ptr: val.last_achievement_at), created_at: val.created_at, updated_at: val.updated_at); }()
    }
}
extension __swift_bridge__$Option$PlayerAchievementStatsBridge {
    @inline(__always)
    func intoSwiftRepr() -> Optional<PlayerAchievementStatsBridge> {
        if self.is_some {
            return self.val.intoSwiftRepr()
        } else {
            return nil
        }
    }

    @inline(__always)
    static func fromSwiftRepr(_ val: Optional<PlayerAchievementStatsBridge>) -> __swift_bridge__$Option$PlayerAchievementStatsBridge {
        if let v = val {
            return __swift_bridge__$Option$PlayerAchievementStatsBridge(is_some: true, val: v.intoFfiRepr())
        } else {
            return __swift_bridge__$Option$PlayerAchievementStatsBridge(is_some: false, val: __swift_bridge__$PlayerAchievementStatsBridge())
        }
    }
}
public struct PlayerAchievementProgressBridge {
    public var achievement: AchievementBridge
    public var task_progress_json: RustString
    public var completed: Bool
    public var progress_percentage: Double

    public init(achievement: AchievementBridge,task_progress_json: RustString,completed: Bool,progress_percentage: Double) {
        self.achievement = achievement
        self.task_progress_json = task_progress_json
        self.completed = completed
        self.progress_percentage = progress_percentage
    }

    @inline(__always)
    func intoFfiRepr() -> __swift_bridge__$PlayerAchievementProgressBridge {
        { let val = self; return __swift_bridge__$PlayerAchievementProgressBridge(achievement: val.achievement.intoFfiRepr(), task_progress_json: { let rustString = val.task_progress_json.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), completed: val.completed, progress_percentage: val.progress_percentage); }()
    }
}
extension __swift_bridge__$PlayerAchievementProgressBridge {
    @inline(__always)
    func intoSwiftRepr() -> PlayerAchievementProgressBridge {
        { let val = self; return PlayerAchievementProgressBridge(achievement: val.achievement.intoSwiftRepr(), task_progress_json: RustString(ptr: val.task_progress_json), completed: val.completed, progress_percentage: val.progress_percentage); }()
    }
}
extension __swift_bridge__$Option$PlayerAchievementProgressBridge {
    @inline(__always)
    func intoSwiftRepr() -> Optional<PlayerAchievementProgressBridge> {
        if self.is_some {
            return self.val.intoSwiftRepr()
        } else {
            return nil
        }
    }

    @inline(__always)
    static func fromSwiftRepr(_ val: Optional<PlayerAchievementProgressBridge>) -> __swift_bridge__$Option$PlayerAchievementProgressBridge {
        if let v = val {
            return __swift_bridge__$Option$PlayerAchievementProgressBridge(is_some: true, val: v.intoFfiRepr())
        } else {
            return __swift_bridge__$Option$PlayerAchievementProgressBridge(is_some: false, val: __swift_bridge__$PlayerAchievementProgressBridge())
        }
    }
}
public struct TaskProgressBridge {
    public var task_id: RustString
    public var count: UInt32
    public var completed: Bool

    public init(task_id: RustString,count: UInt32,completed: Bool) {
        self.task_id = task_id
        self.count = count
        self.completed = completed
    }

    @inline(__always)
    func intoFfiRepr() -> __swift_bridge__$TaskProgressBridge {
        { let val = self; return __swift_bridge__$TaskProgressBridge(task_id: { let rustString = val.task_id.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), count: val.count, completed: val.completed); }()
    }
}
extension __swift_bridge__$TaskProgressBridge {
    @inline(__always)
    func intoSwiftRepr() -> TaskProgressBridge {
        { let val = self; return TaskProgressBridge(task_id: RustString(ptr: val.task_id), count: val.count, completed: val.completed); }()
    }
}
extension __swift_bridge__$Option$TaskProgressBridge {
    @inline(__always)
    func intoSwiftRepr() -> Optional<TaskProgressBridge> {
        if self.is_some {
            return self.val.intoSwiftRepr()
        } else {
            return nil
        }
    }

    @inline(__always)
    static func fromSwiftRepr(_ val: Optional<TaskProgressBridge>) -> __swift_bridge__$Option$TaskProgressBridge {
        if let v = val {
            return __swift_bridge__$Option$TaskProgressBridge(is_some: true, val: v.intoFfiRepr())
        } else {
            return __swift_bridge__$Option$TaskProgressBridge(is_some: false, val: __swift_bridge__$TaskProgressBridge())
        }
    }
}


