# Diplomat Implementation Plan

## Phase 1: Foundation Types ✅ COMPLETE
- [x] DojoError
- [x] FieldElement
- [x] U256
- [x] Call, CallList
- [x] Signature
- [x] SigningKey, VerifyingKey
- [x] TypedData
- [x] ToriiClient (basic)
- [x] Provider
- [x] Account
- [x] Subscription

## Phase 2: Enum Types (Simple)
- [ ] BlockTag (Latest, PreConfirmed)
- [ ] ContractType (WORLD, ERC20, ERC721, ERC1155, UDC, OTHER)
- [ ] PaginationDirection (Forward, Backward)
- [ ] OrderDirection (Asc, Desc)
- [ ] PatternMatching (FixedLen, VariableLen)
- [ ] LogicalOperator (And, Or)
- [ ] ComparisonOperator (Eq, Neq, Gt, Gte, Lt, Lte, In, NotIn, Contains, etc.)
- [ ] CallType (Execute, ExecuteFromOutside)

## Phase 3: Wrapper Types
- [ ] BlockId (enum: Hash, Number, Tag)
- [ ] OrderBy (struct with field, direction)
- [ ] Pagination (struct with cursor, limit, direction, order_by)

## Phase 4: Query Types
- [ ] KeysClause
- [ ] MemberValue (enum: Primitive, String, List)
- [ ] MemberClause
- [ ] CompositeClause
- [ ] Clause (enum: HashedKeys, Keys, Member, Composite)
- [ ] Query (main query struct)

## Phase 5: Schema Types (Complex)
- [ ] Primitive (enum with all primitive types)
- [ ] EnumOption
- [ ] Enum
- [ ] Member
- [ ] Struct
- [ ] FixedSizeArray
- [ ] Ty (recursive enum)

## Phase 6: Entity & World Types
- [ ] Entity
- [ ] Model
- [ ] World
- [ ] Event

## Phase 7: Token Types
- [ ] Token
- [ ] TokenBalance
- [ ] TokenContract
- [ ] TokenTransfer
- [ ] AttributeFilter
- [ ] TokenQuery
- [ ] TokenBalanceQuery
- [ ] TokenContractQuery
- [ ] TokenTransferQuery

## Phase 8: Transaction Types
- [ ] TransactionCall
- [ ] Transaction
- [ ] TransactionFilter
- [ ] TransactionQuery

## Phase 9: Contract Types
- [ ] Contract
- [ ] ContractQuery

## Phase 10: Controller Types ⏸️ SKIPPED (not core Dojo)
- [~] Controller
- [~] ControllerQuery  
- [~] Policy

## Phase 11: Aggregation Types
- [ ] AggregationEntry
- [ ] AggregationQuery

## Phase 12: Activity Types
- [ ] ActionCount
- [ ] Activity
- [ ] ActivityQuery

## Phase 13: Achievement Types
- [ ] AchievementTask
- [ ] Achievement
- [ ] AchievementQuery
- [ ] TaskProgress
- [ ] PlayerAchievementStats
- [ ] PlayerAchievementProgress
- [ ] PlayerAchievementEntry
- [ ] PlayerAchievementQuery
- [ ] AchievementProgression

## Phase 14: Message Types
- [ ] Message

## Phase 15: Page Wrapper
- [ ] Page<T> generic wrapper for paginated results

## Phase 16: ToriiClient Methods - Entity Operations
- [ ] entities()
- [ ] event_messages()
- [ ] on_entity_state_update()
- [ ] update_entity_subscription()

## Phase 17: ToriiClient Methods - World Operations
- [ ] worlds()

## Phase 18: ToriiClient Methods - Transaction Operations
- [ ] transactions()
- [ ] on_transaction()

## Phase 19: ToriiClient Methods - Token Operations
- [ ] tokens()
- [ ] token_balances()
- [ ] token_contracts()
- [ ] token_transfers()
- [ ] on_token_update()
- [ ] on_token_balance_update()
- [ ] update_token_balance_subscription()
- [ ] on_token_transfer_update()
- [ ] update_token_transfer_subscription()

## Phase 20: ToriiClient Methods - Contract Operations
- [ ] contracts()
- [ ] on_contract_update()

## Phase 21: ToriiClient Methods - Controller Operations ⏸️ SKIPPED
- [~] controllers()

## Phase 22: ToriiClient Methods - Aggregation Operations
- [ ] aggregations()
- [ ] on_aggregation_update()
- [ ] update_aggregation_subscription()

## Phase 23: ToriiClient Methods - Achievement Operations
- [ ] achievements()
- [ ] player_achievements()
- [ ] on_achievement_progression_update()
- [ ] update_achievement_progression_subscription()

## Phase 24: ToriiClient Methods - Activity Operations
- [ ] activities()
- [ ] on_activity_update()
- [ ] update_activity_subscription()

## Phase 25: ToriiClient Methods - Event Operations
- [ ] on_event_message_update()
- [ ] update_event_message_subscription()
- [ ] on_starknet_event()

## Phase 26: ToriiClient Methods - Message Operations
- [ ] publish_message() (already exists, needs enhancement)
- [ ] publish_message_batch()

## Phase 27: Account Methods (Extended) ⏸️ SKIPPED (not core Dojo)
- [~] deploy_burner()
- [~] set_block_id()

## Phase 28: Provider Methods (Extended) ⏸️ SKIPPED
- [ ] call()
- [ ] block_number() (already exists)
- [ ] chain_id() (already exists)

## Phase 29: Controller Account Types & Methods ⏸️ SKIPPED (not core Dojo)
- [~] ControllerAccount opaque type
- [~] connect()
- [~] account()
- [~] clear()
- [~] username()
- [~] address()
- [~] chain_id()
- [~] nonce()
- [~] execute_raw()
- [~] execute_from_outside()

## Phase 30: Utility Functions Module
- [ ] bytearray_serialize()
- [ ] bytearray_deserialize()
- [ ] poseidon_hash()
- [ ] pedersen_hash() (already exists)
- [ ] get_selector_from_name() (already exists)
- [ ] get_selector_from_tag()
- [ ] starknet_keccak()
- [ ] cairo_short_string_to_felt()
- [ ] parse_cairo_short_string()
- [ ] compute_contract_address() (already exists)

## Phase 31: Testing & Examples
- [ ] Test basic types
- [ ] Test client operations
- [ ] Test subscriptions
- [ ] Update C example
- [ ] Update JavaScript example
- [ ] Create C++ example

## Notes
- Each opaque type will have automatic Drop implementation via diplomat
- String outputs use DiplomatWrite
- String inputs use DiplomatStr
- Arrays will need special handling (may use slices where possible)
- Callbacks for subscriptions need special consideration
- Page<T> may need to be concrete types (PageEntity, PageToken, etc.) since diplomat doesn't support generics

