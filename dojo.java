import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("dojo_c", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;


  class BlockTag extends IntegerType {
    public BlockTag() {
      super(8, true);
    }

    public BlockTag(long value) {
      super(8, value, true);
    }

    public BlockTag(Pointer p) {
      this(p.getLong(0));
    }
    public static final BlockTag Latest = new BlockTag(0);
    public static final BlockTag Pending = new BlockTag(1);

  }

  class BlockTagByReference extends ByReference {
    public BlockTagByReference() {
      super(8);
    }

    public BlockTagByReference(Pointer p) {
      super(8);
      setPointer(p);
    }

    public BlockTag getValue() {
      Pointer p = getPointer();
      return new BlockTag(p.getLong(0));
    }

    public void setValue(BlockTag value) {
      Pointer p = getPointer();
      p.setLong(0, value.longValue());
    }

  }



  class ComparisonOperator extends IntegerType {
    public ComparisonOperator() {
      super(8, true);
    }

    public ComparisonOperator(long value) {
      super(8, value, true);
    }

    public ComparisonOperator(Pointer p) {
      this(p.getLong(0));
    }
    public static final ComparisonOperator Eq = new ComparisonOperator(0);
    public static final ComparisonOperator Neq = new ComparisonOperator(1);
    public static final ComparisonOperator Gt = new ComparisonOperator(2);
    public static final ComparisonOperator Gte = new ComparisonOperator(3);
    public static final ComparisonOperator Lt = new ComparisonOperator(4);
    public static final ComparisonOperator Lte = new ComparisonOperator(5);

  }

  class ComparisonOperatorByReference extends ByReference {
    public ComparisonOperatorByReference() {
      super(8);
    }

    public ComparisonOperatorByReference(Pointer p) {
      super(8);
      setPointer(p);
    }

    public ComparisonOperator getValue() {
      Pointer p = getPointer();
      return new ComparisonOperator(p.getLong(0));
    }

    public void setValue(ComparisonOperator value) {
      Pointer p = getPointer();
      p.setLong(0, value.longValue());
    }

  }



  class LogicalOperator extends IntegerType {
    public LogicalOperator() {
      super(8, true);
    }

    public LogicalOperator(long value) {
      super(8, value, true);
    }

    public LogicalOperator(Pointer p) {
      this(p.getLong(0));
    }
    public static final LogicalOperator And = new LogicalOperator(0);
    public static final LogicalOperator Or = new LogicalOperator(1);

  }

  class LogicalOperatorByReference extends ByReference {
    public LogicalOperatorByReference() {
      super(8);
    }

    public LogicalOperatorByReference(Pointer p) {
      super(8);
      setPointer(p);
    }

    public LogicalOperator getValue() {
      Pointer p = getPointer();
      return new LogicalOperator(p.getLong(0));
    }

    public void setValue(LogicalOperator value) {
      Pointer p = getPointer();
      p.setLong(0, value.longValue());
    }

  }



  class PatternMatching extends IntegerType {
    public PatternMatching() {
      super(8, true);
    }

    public PatternMatching(long value) {
      super(8, value, true);
    }

    public PatternMatching(Pointer p) {
      this(p.getLong(0));
    }
    public static final PatternMatching FixedLen = new PatternMatching(0);
    public static final PatternMatching VariableLen = new PatternMatching(1);

  }

  class PatternMatchingByReference extends ByReference {
    public PatternMatchingByReference() {
      super(8);
    }

    public PatternMatchingByReference(Pointer p) {
      super(8);
      setPointer(p);
    }

    public PatternMatching getValue() {
      Pointer p = getPointer();
      return new PatternMatching(p.getLong(0));
    }

    public void setValue(PatternMatching value) {
      Pointer p = getPointer();
      p.setLong(0, value.longValue());
    }

  }


  class Account extends PointerType {
    public Account() {
      super(null);
    }
    public Account(Pointer p) {
      super(p);
    }
  }

  class AccountByReference extends Account {
    public AccountByReference() {
      super(null);
    }
    public AccountByReference(Pointer p) {
      super(p);
    }
  }

  class Provider extends PointerType {
    public Provider() {
      super(null);
    }
    public Provider(Pointer p) {
      super(p);
    }
  }

  class ProviderByReference extends Provider {
    public ProviderByReference() {
      super(null);
    }
    public ProviderByReference(Pointer p) {
      super(p);
    }
  }

  class Subscription extends PointerType {
    public Subscription() {
      super(null);
    }
    public Subscription(Pointer p) {
      super(p);
    }
  }

  class SubscriptionByReference extends Subscription {
    public SubscriptionByReference() {
      super(null);
    }
    public SubscriptionByReference(Pointer p) {
      super(p);
    }
  }

  class ToriiClient extends PointerType {
    public ToriiClient() {
      super(null);
    }
    public ToriiClient(Pointer p) {
      super(p);
    }
  }

  class ToriiClientByReference extends ToriiClient {
    public ToriiClientByReference() {
      super(null);
    }
    public ToriiClientByReference(Pointer p) {
      super(p);
    }
  }


  @Structure.FieldOrder({"message"})
  class Error extends Structure implements Structure.ByValue {
    public Error() {
      super();
    }

    public Error(Pointer p) {
      super(p);
    }

    public ByteByReference message;

  }

  @Structure.FieldOrder({"message"})
  class ErrorByReference extends Structure implements Structure.ByReference {
    public ErrorByReference() {
      super();
    }

    public ErrorByReference(Pointer p) {
      super(p);
    }

    public ByteByReference message;

  }



  class ResultToriiClientTag extends IntegerType {
    public ResultToriiClientTag() {
      super(8, true);
    }

    public ResultToriiClientTag(long value) {
      super(8, value, true);
    }

    public ResultToriiClientTag(Pointer p) {
      this(p.getLong(0));
    }
    public static final ResultToriiClientTag OkToriiClient = new ResultToriiClientTag(0);
    public static final ResultToriiClientTag ErrToriiClient = new ResultToriiClientTag(1);

  }

  class ResultToriiClientTagByReference extends ByReference {
    public ResultToriiClientTagByReference() {
      super(8);
    }

    public ResultToriiClientTagByReference(Pointer p) {
      super(8);
      setPointer(p);
    }

    public ResultToriiClientTag getValue() {
      Pointer p = getPointer();
      return new ResultToriiClientTag(p.getLong(0));
    }

    public void setValue(ResultToriiClientTag value) {
      Pointer p = getPointer();
      p.setLong(0, value.longValue());
    }

  }

  @Structure.FieldOrder({"tag", "oktoriiclient", "errtoriiclient"})
  class ResultToriiClient extends Structure implements Structure.ByValue {
    public ResultToriiClient() {
      super();
    }

    public ResultToriiClient(Pointer p) {
      super(p);
    }

    public ResultToriiClientTag tag;
    public ToriiClientByReference oktoriiclient;
    public Error errtoriiclient;

  }

  @Structure.FieldOrder({"tag", "oktoriiclient", "errtoriiclient"})
  class ResultToriiClientByReference extends Structure implements Structure.ByReference {
    public ResultToriiClientByReference() {
      super();
    }

    public ResultToriiClientByReference(Pointer p) {
      super(p);
    }

    public ResultToriiClientTag tag;
    public ToriiClientByReference oktoriiclient;
    public Error errtoriiclient;

  }



  @Structure.FieldOrder({"data"})
  class FieldElement extends Structure implements Structure.ByValue {
    public FieldElement() {
      super();
    }

    public FieldElement(Pointer p) {
      super(p);
    }

    public byte[] data = new byte[32];

  }

  @Structure.FieldOrder({"data"})
  class FieldElementByReference extends Structure implements Structure.ByReference {
    public FieldElementByReference() {
      super();
    }

    public FieldElementByReference(Pointer p) {
      super(p);
    }

    public byte[] data = new byte[32];

  }



  @Structure.FieldOrder({"data", "data_len"})
  class CArrayu8 extends Structure implements Structure.ByValue {
    public CArrayu8() {
      super();
    }

    public CArrayu8(Pointer p) {
      super(p);
    }

    public ByteByReference data;
    public _Size data_len;

  }

  @Structure.FieldOrder({"data", "data_len"})
  class CArrayu8ByReference extends Structure implements Structure.ByReference {
    public CArrayu8ByReference() {
      super();
    }

    public CArrayu8ByReference(Pointer p) {
      super(p);
    }

    public ByteByReference data;
    public _Size data_len;

  }



  class ResultCArrayu8Tag extends IntegerType {
    public ResultCArrayu8Tag() {
      super(8, true);
    }

    public ResultCArrayu8Tag(long value) {
      super(8, value, true);
    }

    public ResultCArrayu8Tag(Pointer p) {
      this(p.getLong(0));
    }
    public static final ResultCArrayu8Tag OkCArrayu8 = new ResultCArrayu8Tag(0);
    public static final ResultCArrayu8Tag ErrCArrayu8 = new ResultCArrayu8Tag(1);

  }

  class ResultCArrayu8TagByReference extends ByReference {
    public ResultCArrayu8TagByReference() {
      super(8);
    }

    public ResultCArrayu8TagByReference(Pointer p) {
      super(8);
      setPointer(p);
    }

    public ResultCArrayu8Tag getValue() {
      Pointer p = getPointer();
      return new ResultCArrayu8Tag(p.getLong(0));
    }

    public void setValue(ResultCArrayu8Tag value) {
      Pointer p = getPointer();
      p.setLong(0, value.longValue());
    }

  }

  @Structure.FieldOrder({"tag", "okcarrayu8", "errcarrayu8"})
  class ResultCArrayu8 extends Structure implements Structure.ByValue {
    public ResultCArrayu8() {
      super();
    }

    public ResultCArrayu8(Pointer p) {
      super(p);
    }

    public ResultCArrayu8Tag tag;
    public CArrayu8 okcarrayu8;
    public Error errcarrayu8;

  }

  @Structure.FieldOrder({"tag", "okcarrayu8", "errcarrayu8"})
  class ResultCArrayu8ByReference extends Structure implements Structure.ByReference {
    public ResultCArrayu8ByReference() {
      super();
    }

    public ResultCArrayu8ByReference(Pointer p) {
      super(p);
    }

    public ResultCArrayu8Tag tag;
    public CArrayu8 okcarrayu8;
    public Error errcarrayu8;

  }



  class PrimitiveTag extends IntegerType {
    public PrimitiveTag() {
      super(8, true);
    }

    public PrimitiveTag(long value) {
      super(8, value, true);
    }

    public PrimitiveTag(Pointer p) {
      this(p.getLong(0));
    }
    public static final PrimitiveTag I8 = new PrimitiveTag(0);
    public static final PrimitiveTag I16 = new PrimitiveTag(1);
    public static final PrimitiveTag I32 = new PrimitiveTag(2);
    public static final PrimitiveTag I64 = new PrimitiveTag(3);
    public static final PrimitiveTag I128 = new PrimitiveTag(4);
    public static final PrimitiveTag U8 = new PrimitiveTag(5);
    public static final PrimitiveTag U16 = new PrimitiveTag(6);
    public static final PrimitiveTag U32 = new PrimitiveTag(7);
    public static final PrimitiveTag U64 = new PrimitiveTag(8);
    public static final PrimitiveTag U128 = new PrimitiveTag(9);
    public static final PrimitiveTag U256 = new PrimitiveTag(10);
    public static final PrimitiveTag USize = new PrimitiveTag(11);
    public static final PrimitiveTag Bool = new PrimitiveTag(12);
    public static final PrimitiveTag Felt252 = new PrimitiveTag(13);
    public static final PrimitiveTag ClassHash = new PrimitiveTag(14);
    public static final PrimitiveTag ContractAddress = new PrimitiveTag(15);

  }

  class PrimitiveTagByReference extends ByReference {
    public PrimitiveTagByReference() {
      super(8);
    }

    public PrimitiveTagByReference(Pointer p) {
      super(8);
      setPointer(p);
    }

    public PrimitiveTag getValue() {
      Pointer p = getPointer();
      return new PrimitiveTag(p.getLong(0));
    }

    public void setValue(PrimitiveTag value) {
      Pointer p = getPointer();
      p.setLong(0, value.longValue());
    }

  }

  @Structure.FieldOrder({"tag",
                         "i8",
                         "i16",
                         "i32",
                         "i64",
                         "i128",
                         "u8",
                         "u16",
                         "u32",
                         "u64",
                         "u128",
                         "u256",
                         "usize",
                         "bool",
                         "felt252",
                         "classhash",
                         "contractaddress"})
  class Primitive extends Structure implements Structure.ByValue {
    public Primitive() {
      super();
    }

    public Primitive(Pointer p) {
      super(p);
    }

    public PrimitiveTag tag;
    public byte i8;
    public short i16;
    public int i32;
    public long i64;
    public byte[] i128 = new byte[16];
    public byte u8;
    public short u16;
    public int u32;
    public long u64;
    public byte[] u128 = new byte[16];
    public long[] u256 = new long[4];
    public int usize;
    public _Boolean bool;
    public FieldElement felt252;
    public FieldElement classhash;
    public FieldElement contractaddress;

  }

  @Structure.FieldOrder({"tag",
                         "i8",
                         "i16",
                         "i32",
                         "i64",
                         "i128",
                         "u8",
                         "u16",
                         "u32",
                         "u64",
                         "u128",
                         "u256",
                         "usize",
                         "bool",
                         "felt252",
                         "classhash",
                         "contractaddress"})
  class PrimitiveByReference extends Structure implements Structure.ByReference {
    public PrimitiveByReference() {
      super();
    }

    public PrimitiveByReference(Pointer p) {
      super(p);
    }

    public PrimitiveTag tag;
    public byte i8;
    public short i16;
    public int i32;
    public long i64;
    public byte[] i128 = new byte[16];
    public byte u8;
    public short u16;
    public int u32;
    public long u64;
    public byte[] u128 = new byte[16];
    public long[] u256 = new long[4];
    public int usize;
    public _Boolean bool;
    public FieldElement felt252;
    public FieldElement classhash;
    public FieldElement contractaddress;

  }



  @Structure.FieldOrder({"name", "ty"})
  class EnumOption extends Structure implements Structure.ByValue {
    public EnumOption() {
      super();
    }

    public EnumOption(Pointer p) {
      super(p);
    }

    public ByteByReference name;
    public TyByReference ty;

  }

  @Structure.FieldOrder({"name", "ty"})
  class EnumOptionByReference extends Structure implements Structure.ByReference {
    public EnumOptionByReference() {
      super();
    }

    public EnumOptionByReference(Pointer p) {
      super(p);
    }

    public ByteByReference name;
    public TyByReference ty;

  }



  @Structure.FieldOrder({"data", "data_len"})
  class CArrayEnumOption extends Structure implements Structure.ByValue {
    public CArrayEnumOption() {
      super();
    }

    public CArrayEnumOption(Pointer p) {
      super(p);
    }

    public EnumOptionByReference data;
    public _Size data_len;

  }

  @Structure.FieldOrder({"data", "data_len"})
  class CArrayEnumOptionByReference extends Structure implements Structure.ByReference {
    public CArrayEnumOptionByReference() {
      super();
    }

    public CArrayEnumOptionByReference(Pointer p) {
      super(p);
    }

    public EnumOptionByReference data;
    public _Size data_len;

  }



  @Structure.FieldOrder({"name", "option", "options"})
  class Enum extends Structure implements Structure.ByValue {
    public Enum() {
      super();
    }

    public Enum(Pointer p) {
      super(p);
    }

    public ByteByReference name;
    public byte option;
    public CArrayEnumOption options;

  }

  @Structure.FieldOrder({"name", "option", "options"})
  class EnumByReference extends Structure implements Structure.ByReference {
    public EnumByReference() {
      super();
    }

    public EnumByReference(Pointer p) {
      super(p);
    }

    public ByteByReference name;
    public byte option;
    public CArrayEnumOption options;

  }



  @Structure.FieldOrder({"data", "data_len"})
  class CArrayTy extends Structure implements Structure.ByValue {
    public CArrayTy() {
      super();
    }

    public CArrayTy(Pointer p) {
      super(p);
    }

    public TyByReference data;
    public _Size data_len;

  }

  @Structure.FieldOrder({"data", "data_len"})
  class CArrayTyByReference extends Structure implements Structure.ByReference {
    public CArrayTyByReference() {
      super();
    }

    public CArrayTyByReference(Pointer p) {
      super(p);
    }

    public TyByReference data;
    public _Size data_len;

  }



  class TyTag extends IntegerType {
    public TyTag() {
      super(8, true);
    }

    public TyTag(long value) {
      super(8, value, true);
    }

    public TyTag(Pointer p) {
      this(p.getLong(0));
    }
    public static final TyTag Primitive_ = new TyTag(0);
    public static final TyTag Struct_ = new TyTag(1);
    public static final TyTag Enum_ = new TyTag(2);
    public static final TyTag Tuple_ = new TyTag(3);
    public static final TyTag Array_ = new TyTag(4);
    public static final TyTag ByteArray = new TyTag(5);

  }

  class TyTagByReference extends ByReference {
    public TyTagByReference() {
      super(8);
    }

    public TyTagByReference(Pointer p) {
      super(8);
      setPointer(p);
    }

    public TyTag getValue() {
      Pointer p = getPointer();
      return new TyTag(p.getLong(0));
    }

    public void setValue(TyTag value) {
      Pointer p = getPointer();
      p.setLong(0, value.longValue());
    }

  }

  @Structure.FieldOrder({"tag", "primitive_", "struct_", "enum_", "tuple_", "array_", "bytearray"})
  class Ty extends Structure implements Structure.ByValue {
    public Ty() {
      super();
    }

    public Ty(Pointer p) {
      super(p);
    }

    public TyTag tag;
    public Primitive primitive_;
    public Struct struct_;
    public Enum enum_;
    public CArrayTy tuple_;
    public CArrayTy array_;
    public ByteByReference bytearray;

  }

  @Structure.FieldOrder({"tag", "primitive_", "struct_", "enum_", "tuple_", "array_", "bytearray"})
  class TyByReference extends Structure implements Structure.ByReference {
    public TyByReference() {
      super();
    }

    public TyByReference(Pointer p) {
      super(p);
    }

    public TyTag tag;
    public Primitive primitive_;
    public Struct struct_;
    public Enum enum_;
    public CArrayTy tuple_;
    public CArrayTy array_;
    public ByteByReference bytearray;

  }



  @Structure.FieldOrder({"name", "ty", "key"})
  class Member extends Structure implements Structure.ByValue {
    public Member() {
      super();
    }

    public Member(Pointer p) {
      super(p);
    }

    public ByteByReference name;
    public TyByReference ty;
    public _Boolean key;

  }

  @Structure.FieldOrder({"name", "ty", "key"})
  class MemberByReference extends Structure implements Structure.ByReference {
    public MemberByReference() {
      super();
    }

    public MemberByReference(Pointer p) {
      super(p);
    }

    public ByteByReference name;
    public TyByReference ty;
    public _Boolean key;

  }



  @Structure.FieldOrder({"data", "data_len"})
  class CArrayMember extends Structure implements Structure.ByValue {
    public CArrayMember() {
      super();
    }

    public CArrayMember(Pointer p) {
      super(p);
    }

    public MemberByReference data;
    public _Size data_len;

  }

  @Structure.FieldOrder({"data", "data_len"})
  class CArrayMemberByReference extends Structure implements Structure.ByReference {
    public CArrayMemberByReference() {
      super();
    }

    public CArrayMemberByReference(Pointer p) {
      super(p);
    }

    public MemberByReference data;
    public _Size data_len;

  }



  @Structure.FieldOrder({"name", "children"})
  class Struct extends Structure implements Structure.ByValue {
    public Struct() {
      super();
    }

    public Struct(Pointer p) {
      super(p);
    }

    public ByteByReference name;
    public CArrayMember children;

  }

  @Structure.FieldOrder({"name", "children"})
  class StructByReference extends Structure implements Structure.ByReference {
    public StructByReference() {
      super();
    }

    public StructByReference(Pointer p) {
      super(p);
    }

    public ByteByReference name;
    public CArrayMember children;

  }



  @Structure.FieldOrder({"data", "data_len"})
  class CArrayStruct extends Structure implements Structure.ByValue {
    public CArrayStruct() {
      super();
    }

    public CArrayStruct(Pointer p) {
      super(p);
    }

    public StructByReference data;
    public _Size data_len;

  }

  @Structure.FieldOrder({"data", "data_len"})
  class CArrayStructByReference extends Structure implements Structure.ByReference {
    public CArrayStructByReference() {
      super();
    }

    public CArrayStructByReference(Pointer p) {
      super(p);
    }

    public StructByReference data;
    public _Size data_len;

  }



  @Structure.FieldOrder({"hashed_keys", "models"})
  class Entity extends Structure implements Structure.ByValue {
    public Entity() {
      super();
    }

    public Entity(Pointer p) {
      super(p);
    }

    public FieldElement hashed_keys;
    public CArrayStruct models;

  }

  @Structure.FieldOrder({"hashed_keys", "models"})
  class EntityByReference extends Structure implements Structure.ByReference {
    public EntityByReference() {
      super();
    }

    public EntityByReference(Pointer p) {
      super(p);
    }

    public FieldElement hashed_keys;
    public CArrayStruct models;

  }



  @Structure.FieldOrder({"data", "data_len"})
  class CArrayEntity extends Structure implements Structure.ByValue {
    public CArrayEntity() {
      super();
    }

    public CArrayEntity(Pointer p) {
      super(p);
    }

    public EntityByReference data;
    public _Size data_len;

  }

  @Structure.FieldOrder({"data", "data_len"})
  class CArrayEntityByReference extends Structure implements Structure.ByReference {
    public CArrayEntityByReference() {
      super();
    }

    public CArrayEntityByReference(Pointer p) {
      super(p);
    }

    public EntityByReference data;
    public _Size data_len;

  }



  class ResultCArrayEntityTag extends IntegerType {
    public ResultCArrayEntityTag() {
      super(8, true);
    }

    public ResultCArrayEntityTag(long value) {
      super(8, value, true);
    }

    public ResultCArrayEntityTag(Pointer p) {
      this(p.getLong(0));
    }
    public static final ResultCArrayEntityTag OkCArrayEntity = new ResultCArrayEntityTag(0);
    public static final ResultCArrayEntityTag ErrCArrayEntity = new ResultCArrayEntityTag(1);

  }

  class ResultCArrayEntityTagByReference extends ByReference {
    public ResultCArrayEntityTagByReference() {
      super(8);
    }

    public ResultCArrayEntityTagByReference(Pointer p) {
      super(8);
      setPointer(p);
    }

    public ResultCArrayEntityTag getValue() {
      Pointer p = getPointer();
      return new ResultCArrayEntityTag(p.getLong(0));
    }

    public void setValue(ResultCArrayEntityTag value) {
      Pointer p = getPointer();
      p.setLong(0, value.longValue());
    }

  }

  @Structure.FieldOrder({"tag", "okcarrayentity", "errcarrayentity"})
  class ResultCArrayEntity extends Structure implements Structure.ByValue {
    public ResultCArrayEntity() {
      super();
    }

    public ResultCArrayEntity(Pointer p) {
      super(p);
    }

    public ResultCArrayEntityTag tag;
    public CArrayEntity okcarrayentity;
    public Error errcarrayentity;

  }

  @Structure.FieldOrder({"tag", "okcarrayentity", "errcarrayentity"})
  class ResultCArrayEntityByReference extends Structure implements Structure.ByReference {
    public ResultCArrayEntityByReference() {
      super();
    }

    public ResultCArrayEntityByReference(Pointer p) {
      super(p);
    }

    public ResultCArrayEntityTag tag;
    public CArrayEntity okcarrayentity;
    public Error errcarrayentity;

  }



  class COptionFieldElementTag extends IntegerType {
    public COptionFieldElementTag() {
      super(8, true);
    }

    public COptionFieldElementTag(long value) {
      super(8, value, true);
    }

    public COptionFieldElementTag(Pointer p) {
      this(p.getLong(0));
    }
    public static final COptionFieldElementTag SomeFieldElement = new COptionFieldElementTag(0);
    public static final COptionFieldElementTag NoneFieldElement = new COptionFieldElementTag(1);

  }

  class COptionFieldElementTagByReference extends ByReference {
    public COptionFieldElementTagByReference() {
      super(8);
    }

    public COptionFieldElementTagByReference(Pointer p) {
      super(8);
      setPointer(p);
    }

    public COptionFieldElementTag getValue() {
      Pointer p = getPointer();
      return new COptionFieldElementTag(p.getLong(0));
    }

    public void setValue(COptionFieldElementTag value) {
      Pointer p = getPointer();
      p.setLong(0, value.longValue());
    }

  }

  @Structure.FieldOrder({"tag", "somefieldelement"})
  class COptionFieldElement extends Structure implements Structure.ByValue {
    public COptionFieldElement() {
      super();
    }

    public COptionFieldElement(Pointer p) {
      super(p);
    }

    public COptionFieldElementTag tag;
    public FieldElement somefieldelement;

  }

  @Structure.FieldOrder({"tag", "somefieldelement"})
  class COptionFieldElementByReference extends Structure implements Structure.ByReference {
    public COptionFieldElementByReference() {
      super();
    }

    public COptionFieldElementByReference(Pointer p) {
      super(p);
    }

    public COptionFieldElementTag tag;
    public FieldElement somefieldelement;

  }



  @Structure.FieldOrder({"data", "data_len"})
  class CArrayCOptionFieldElement extends Structure implements Structure.ByValue {
    public CArrayCOptionFieldElement() {
      super();
    }

    public CArrayCOptionFieldElement(Pointer p) {
      super(p);
    }

    public COptionFieldElementByReference data;
    public _Size data_len;

  }

  @Structure.FieldOrder({"data", "data_len"})
  class CArrayCOptionFieldElementByReference extends Structure implements Structure.ByReference {
    public CArrayCOptionFieldElementByReference() {
      super();
    }

    public CArrayCOptionFieldElementByReference(Pointer p) {
      super(p);
    }

    public COptionFieldElementByReference data;
    public _Size data_len;

  }



  @Structure.FieldOrder({"data", "data_len"})
  class CArrayc_char extends Structure implements Structure.ByValue {
    public CArrayc_char() {
      super();
    }

    public CArrayc_char(Pointer p) {
      super(p);
    }

    public PointerByReference data;
    public _Size data_len;

  }

  @Structure.FieldOrder({"data", "data_len"})
  class CArrayc_charByReference extends Structure implements Structure.ByReference {
    public CArrayc_charByReference() {
      super();
    }

    public CArrayc_charByReference(Pointer p) {
      super(p);
    }

    public PointerByReference data;
    public _Size data_len;

  }



  @Structure.FieldOrder({"keys", "pattern_matching", "models"})
  class KeysClause extends Structure implements Structure.ByValue {
    public KeysClause() {
      super();
    }

    public KeysClause(Pointer p) {
      super(p);
    }

    public CArrayCOptionFieldElement keys;
    public PatternMatching pattern_matching;
    public CArrayc_char models;

  }

  @Structure.FieldOrder({"keys", "pattern_matching", "models"})
  class KeysClauseByReference extends Structure implements Structure.ByReference {
    public KeysClauseByReference() {
      super();
    }

    public KeysClauseByReference(Pointer p) {
      super(p);
    }

    public CArrayCOptionFieldElement keys;
    public PatternMatching pattern_matching;
    public CArrayc_char models;

  }



  class MemberValueTag extends IntegerType {
    public MemberValueTag() {
      super(8, true);
    }

    public MemberValueTag(long value) {
      super(8, value, true);
    }

    public MemberValueTag(Pointer p) {
      this(p.getLong(0));
    }
    public static final MemberValueTag Primitive = new MemberValueTag(0);
    public static final MemberValueTag String = new MemberValueTag(1);

  }

  class MemberValueTagByReference extends ByReference {
    public MemberValueTagByReference() {
      super(8);
    }

    public MemberValueTagByReference(Pointer p) {
      super(8);
      setPointer(p);
    }

    public MemberValueTag getValue() {
      Pointer p = getPointer();
      return new MemberValueTag(p.getLong(0));
    }

    public void setValue(MemberValueTag value) {
      Pointer p = getPointer();
      p.setLong(0, value.longValue());
    }

  }

  @Structure.FieldOrder({"tag", "primitive", "string"})
  class MemberValue extends Structure implements Structure.ByValue {
    public MemberValue() {
      super();
    }

    public MemberValue(Pointer p) {
      super(p);
    }

    public MemberValueTag tag;
    public Primitive primitive;
    public ByteByReference string;

  }

  @Structure.FieldOrder({"tag", "primitive", "string"})
  class MemberValueByReference extends Structure implements Structure.ByReference {
    public MemberValueByReference() {
      super();
    }

    public MemberValueByReference(Pointer p) {
      super(p);
    }

    public MemberValueTag tag;
    public Primitive primitive;
    public ByteByReference string;

  }



  @Structure.FieldOrder({"model", "member", "operator_", "value"})
  class MemberClause extends Structure implements Structure.ByValue {
    public MemberClause() {
      super();
    }

    public MemberClause(Pointer p) {
      super(p);
    }

    public ByteByReference model;
    public ByteByReference member;
    public ComparisonOperator operator_;
    public MemberValue value;

  }

  @Structure.FieldOrder({"model", "member", "operator_", "value"})
  class MemberClauseByReference extends Structure implements Structure.ByReference {
    public MemberClauseByReference() {
      super();
    }

    public MemberClauseByReference(Pointer p) {
      super(p);
    }

    public ByteByReference model;
    public ByteByReference member;
    public ComparisonOperator operator_;
    public MemberValue value;

  }



  @Structure.FieldOrder({"data", "data_len"})
  class CArrayClause extends Structure implements Structure.ByValue {
    public CArrayClause() {
      super();
    }

    public CArrayClause(Pointer p) {
      super(p);
    }

    public ClauseByReference data;
    public _Size data_len;

  }

  @Structure.FieldOrder({"data", "data_len"})
  class CArrayClauseByReference extends Structure implements Structure.ByReference {
    public CArrayClauseByReference() {
      super();
    }

    public CArrayClauseByReference(Pointer p) {
      super(p);
    }

    public ClauseByReference data;
    public _Size data_len;

  }



  @Structure.FieldOrder({"operator_", "clauses"})
  class CompositeClause extends Structure implements Structure.ByValue {
    public CompositeClause() {
      super();
    }

    public CompositeClause(Pointer p) {
      super(p);
    }

    public LogicalOperator operator_;
    public CArrayClause clauses;

  }

  @Structure.FieldOrder({"operator_", "clauses"})
  class CompositeClauseByReference extends Structure implements Structure.ByReference {
    public CompositeClauseByReference() {
      super();
    }

    public CompositeClauseByReference(Pointer p) {
      super(p);
    }

    public LogicalOperator operator_;
    public CArrayClause clauses;

  }



  class ClauseTag extends IntegerType {
    public ClauseTag() {
      super(8, true);
    }

    public ClauseTag(long value) {
      super(8, value, true);
    }

    public ClauseTag(Pointer p) {
      this(p.getLong(0));
    }
    public static final ClauseTag Keys = new ClauseTag(0);
    public static final ClauseTag CMember = new ClauseTag(1);
    public static final ClauseTag Composite = new ClauseTag(2);

  }

  class ClauseTagByReference extends ByReference {
    public ClauseTagByReference() {
      super(8);
    }

    public ClauseTagByReference(Pointer p) {
      super(8);
      setPointer(p);
    }

    public ClauseTag getValue() {
      Pointer p = getPointer();
      return new ClauseTag(p.getLong(0));
    }

    public void setValue(ClauseTag value) {
      Pointer p = getPointer();
      p.setLong(0, value.longValue());
    }

  }

  @Structure.FieldOrder({"tag", "keys", "cmember", "composite"})
  class Clause extends Structure implements Structure.ByValue {
    public Clause() {
      super();
    }

    public Clause(Pointer p) {
      super(p);
    }

    public ClauseTag tag;
    public KeysClause keys;
    public MemberClause cmember;
    public CompositeClause composite;

  }

  @Structure.FieldOrder({"tag", "keys", "cmember", "composite"})
  class ClauseByReference extends Structure implements Structure.ByReference {
    public ClauseByReference() {
      super();
    }

    public ClauseByReference(Pointer p) {
      super(p);
    }

    public ClauseTag tag;
    public KeysClause keys;
    public MemberClause cmember;
    public CompositeClause composite;

  }



  class COptionClauseTag extends IntegerType {
    public COptionClauseTag() {
      super(8, true);
    }

    public COptionClauseTag(long value) {
      super(8, value, true);
    }

    public COptionClauseTag(Pointer p) {
      this(p.getLong(0));
    }
    public static final COptionClauseTag SomeClause = new COptionClauseTag(0);
    public static final COptionClauseTag NoneClause = new COptionClauseTag(1);

  }

  class COptionClauseTagByReference extends ByReference {
    public COptionClauseTagByReference() {
      super(8);
    }

    public COptionClauseTagByReference(Pointer p) {
      super(8);
      setPointer(p);
    }

    public COptionClauseTag getValue() {
      Pointer p = getPointer();
      return new COptionClauseTag(p.getLong(0));
    }

    public void setValue(COptionClauseTag value) {
      Pointer p = getPointer();
      p.setLong(0, value.longValue());
    }

  }

  @Structure.FieldOrder({"tag", "someclause"})
  class COptionClause extends Structure implements Structure.ByValue {
    public COptionClause() {
      super();
    }

    public COptionClause(Pointer p) {
      super(p);
    }

    public COptionClauseTag tag;
    public Clause someclause;

  }

  @Structure.FieldOrder({"tag", "someclause"})
  class COptionClauseByReference extends Structure implements Structure.ByReference {
    public COptionClauseByReference() {
      super();
    }

    public COptionClauseByReference(Pointer p) {
      super(p);
    }

    public COptionClauseTag tag;
    public Clause someclause;

  }



  @Structure.FieldOrder({"limit", "offset", "clause"})
  class Query extends Structure implements Structure.ByValue {
    public Query() {
      super();
    }

    public Query(Pointer p) {
      super(p);
    }

    public int limit;
    public int offset;
    public COptionClause clause;

  }

  @Structure.FieldOrder({"limit", "offset", "clause"})
  class QueryByReference extends Structure implements Structure.ByReference {
    public QueryByReference() {
      super();
    }

    public QueryByReference(Pointer p) {
      super(p);
    }

    public int limit;
    public int offset;
    public COptionClause clause;

  }



  @Structure.FieldOrder({"data", "data_len"})
  class CArrayFieldElement extends Structure implements Structure.ByValue {
    public CArrayFieldElement() {
      super();
    }

    public CArrayFieldElement(Pointer p) {
      super(p);
    }

    public FieldElementByReference data;
    public _Size data_len;

  }

  @Structure.FieldOrder({"data", "data_len"})
  class CArrayFieldElementByReference extends Structure implements Structure.ByReference {
    public CArrayFieldElementByReference() {
      super();
    }

    public CArrayFieldElementByReference(Pointer p) {
      super(p);
    }

    public FieldElementByReference data;
    public _Size data_len;

  }



  @Structure.FieldOrder({"schema",
                         "namespace_",
                         "name",
                         "packed_size",
                         "unpacked_size",
                         "class_hash",
                         "contract_address",
                         "layout"})
  class ModelMetadata extends Structure implements Structure.ByValue {
    public ModelMetadata() {
      super();
    }

    public ModelMetadata(Pointer p) {
      super(p);
    }

    public Ty schema;
    public ByteByReference namespace_;
    public ByteByReference name;
    public int packed_size;
    public int unpacked_size;
    public FieldElement class_hash;
    public FieldElement contract_address;
    public CArrayFieldElement layout;

  }

  @Structure.FieldOrder({"schema",
                         "namespace_",
                         "name",
                         "packed_size",
                         "unpacked_size",
                         "class_hash",
                         "contract_address",
                         "layout"})
  class ModelMetadataByReference extends Structure implements Structure.ByReference {
    public ModelMetadataByReference() {
      super();
    }

    public ModelMetadataByReference(Pointer p) {
      super(p);
    }

    public Ty schema;
    public ByteByReference namespace_;
    public ByteByReference name;
    public int packed_size;
    public int unpacked_size;
    public FieldElement class_hash;
    public FieldElement contract_address;
    public CArrayFieldElement layout;

  }



  @Structure.FieldOrder({"key", "value"})
  class CHashItemFieldElementModelMetadata extends Structure implements Structure.ByValue {
    public CHashItemFieldElementModelMetadata() {
      super();
    }

    public CHashItemFieldElementModelMetadata(Pointer p) {
      super(p);
    }

    public FieldElement key;
    public ModelMetadata value;

  }

  @Structure.FieldOrder({"key", "value"})
  class CHashItemFieldElementModelMetadataByReference extends Structure implements Structure.ByReference {
    public CHashItemFieldElementModelMetadataByReference() {
      super();
    }

    public CHashItemFieldElementModelMetadataByReference(Pointer p) {
      super(p);
    }

    public FieldElement key;
    public ModelMetadata value;

  }



  @Structure.FieldOrder({"data", "data_len"})
  class CArrayCHashItemFieldElementModelMetadata extends Structure implements Structure.ByValue {
    public CArrayCHashItemFieldElementModelMetadata() {
      super();
    }

    public CArrayCHashItemFieldElementModelMetadata(Pointer p) {
      super(p);
    }

    public CHashItemFieldElementModelMetadataByReference data;
    public _Size data_len;

  }

  @Structure.FieldOrder({"data", "data_len"})
  class CArrayCHashItemFieldElementModelMetadataByReference extends Structure implements Structure.ByReference {
    public CArrayCHashItemFieldElementModelMetadataByReference() {
      super();
    }

    public CArrayCHashItemFieldElementModelMetadataByReference(Pointer p) {
      super(p);
    }

    public CHashItemFieldElementModelMetadataByReference data;
    public _Size data_len;

  }



  @Structure.FieldOrder({"world_address", "models"})
  class WorldMetadata extends Structure implements Structure.ByValue {
    public WorldMetadata() {
      super();
    }

    public WorldMetadata(Pointer p) {
      super(p);
    }

    public FieldElement world_address;
    public CArrayCHashItemFieldElementModelMetadata models;

  }

  @Structure.FieldOrder({"world_address", "models"})
  class WorldMetadataByReference extends Structure implements Structure.ByReference {
    public WorldMetadataByReference() {
      super();
    }

    public WorldMetadataByReference(Pointer p) {
      super(p);
    }

    public FieldElement world_address;
    public CArrayCHashItemFieldElementModelMetadata models;

  }



  class ResultSubscriptionTag extends IntegerType {
    public ResultSubscriptionTag() {
      super(8, true);
    }

    public ResultSubscriptionTag(long value) {
      super(8, value, true);
    }

    public ResultSubscriptionTag(Pointer p) {
      this(p.getLong(0));
    }
    public static final ResultSubscriptionTag OkSubscription = new ResultSubscriptionTag(0);
    public static final ResultSubscriptionTag ErrSubscription = new ResultSubscriptionTag(1);

  }

  class ResultSubscriptionTagByReference extends ByReference {
    public ResultSubscriptionTagByReference() {
      super(8);
    }

    public ResultSubscriptionTagByReference(Pointer p) {
      super(8);
      setPointer(p);
    }

    public ResultSubscriptionTag getValue() {
      Pointer p = getPointer();
      return new ResultSubscriptionTag(p.getLong(0));
    }

    public void setValue(ResultSubscriptionTag value) {
      Pointer p = getPointer();
      p.setLong(0, value.longValue());
    }

  }

  @Structure.FieldOrder({"tag", "oksubscription", "errsubscription"})
  class ResultSubscription extends Structure implements Structure.ByValue {
    public ResultSubscription() {
      super();
    }

    public ResultSubscription(Pointer p) {
      super(p);
    }

    public ResultSubscriptionTag tag;
    public SubscriptionByReference oksubscription;
    public Error errsubscription;

  }

  @Structure.FieldOrder({"tag", "oksubscription", "errsubscription"})
  class ResultSubscriptionByReference extends Structure implements Structure.ByReference {
    public ResultSubscriptionByReference() {
      super();
    }

    public ResultSubscriptionByReference(Pointer p) {
      super(p);
    }

    public ResultSubscriptionTag tag;
    public SubscriptionByReference oksubscription;
    public Error errsubscription;

  }



  class EntityKeysClauseTag extends IntegerType {
    public EntityKeysClauseTag() {
      super(8, true);
    }

    public EntityKeysClauseTag(long value) {
      super(8, value, true);
    }

    public EntityKeysClauseTag(Pointer p) {
      this(p.getLong(0));
    }
    public static final EntityKeysClauseTag HashedKeys = new EntityKeysClauseTag(0);
    public static final EntityKeysClauseTag EntityKeys = new EntityKeysClauseTag(1);

  }

  class EntityKeysClauseTagByReference extends ByReference {
    public EntityKeysClauseTagByReference() {
      super(8);
    }

    public EntityKeysClauseTagByReference(Pointer p) {
      super(8);
      setPointer(p);
    }

    public EntityKeysClauseTag getValue() {
      Pointer p = getPointer();
      return new EntityKeysClauseTag(p.getLong(0));
    }

    public void setValue(EntityKeysClauseTag value) {
      Pointer p = getPointer();
      p.setLong(0, value.longValue());
    }

  }

  @Structure.FieldOrder({"tag", "hashedkeys", "entitykeys"})
  class EntityKeysClause extends Structure implements Structure.ByValue {
    public EntityKeysClause() {
      super();
    }

    public EntityKeysClause(Pointer p) {
      super(p);
    }

    public EntityKeysClauseTag tag;
    public CArrayFieldElement hashedkeys;
    public KeysClause entitykeys;

  }

  @Structure.FieldOrder({"tag", "hashedkeys", "entitykeys"})
  class EntityKeysClauseByReference extends Structure implements Structure.ByReference {
    public EntityKeysClauseByReference() {
      super();
    }

    public EntityKeysClauseByReference(Pointer p) {
      super(p);
    }

    public EntityKeysClauseTag tag;
    public CArrayFieldElement hashedkeys;
    public KeysClause entitykeys;

  }



  class ResultboolTag extends IntegerType {
    public ResultboolTag() {
      super(8, true);
    }

    public ResultboolTag(long value) {
      super(8, value, true);
    }

    public ResultboolTag(Pointer p) {
      this(p.getLong(0));
    }
    public static final ResultboolTag Okbool = new ResultboolTag(0);
    public static final ResultboolTag Errbool = new ResultboolTag(1);

  }

  class ResultboolTagByReference extends ByReference {
    public ResultboolTagByReference() {
      super(8);
    }

    public ResultboolTagByReference(Pointer p) {
      super(8);
      setPointer(p);
    }

    public ResultboolTag getValue() {
      Pointer p = getPointer();
      return new ResultboolTag(p.getLong(0));
    }

    public void setValue(ResultboolTag value) {
      Pointer p = getPointer();
      p.setLong(0, value.longValue());
    }

  }

  @Structure.FieldOrder({"tag", "okbool", "errbool"})
  class Resultbool extends Structure implements Structure.ByValue {
    public Resultbool() {
      super();
    }

    public Resultbool(Pointer p) {
      super(p);
    }

    public ResultboolTag tag;
    public _Boolean okbool;
    public Error errbool;

  }

  @Structure.FieldOrder({"tag", "okbool", "errbool"})
  class ResultboolByReference extends Structure implements Structure.ByReference {
    public ResultboolByReference() {
      super();
    }

    public ResultboolByReference(Pointer p) {
      super(p);
    }

    public ResultboolTag tag;
    public _Boolean okbool;
    public Error errbool;

  }



  class ResultCArrayFieldElementTag extends IntegerType {
    public ResultCArrayFieldElementTag() {
      super(8, true);
    }

    public ResultCArrayFieldElementTag(long value) {
      super(8, value, true);
    }

    public ResultCArrayFieldElementTag(Pointer p) {
      this(p.getLong(0));
    }
    public static final ResultCArrayFieldElementTag OkCArrayFieldElement = new ResultCArrayFieldElementTag(0);
    public static final ResultCArrayFieldElementTag ErrCArrayFieldElement = new ResultCArrayFieldElementTag(1);

  }

  class ResultCArrayFieldElementTagByReference extends ByReference {
    public ResultCArrayFieldElementTagByReference() {
      super(8);
    }

    public ResultCArrayFieldElementTagByReference(Pointer p) {
      super(8);
      setPointer(p);
    }

    public ResultCArrayFieldElementTag getValue() {
      Pointer p = getPointer();
      return new ResultCArrayFieldElementTag(p.getLong(0));
    }

    public void setValue(ResultCArrayFieldElementTag value) {
      Pointer p = getPointer();
      p.setLong(0, value.longValue());
    }

  }

  @Structure.FieldOrder({"tag", "okcarrayfieldelement", "errcarrayfieldelement"})
  class ResultCArrayFieldElement extends Structure implements Structure.ByValue {
    public ResultCArrayFieldElement() {
      super();
    }

    public ResultCArrayFieldElement(Pointer p) {
      super(p);
    }

    public ResultCArrayFieldElementTag tag;
    public CArrayFieldElement okcarrayfieldelement;
    public Error errcarrayfieldelement;

  }

  @Structure.FieldOrder({"tag", "okcarrayfieldelement", "errcarrayfieldelement"})
  class ResultCArrayFieldElementByReference extends Structure implements Structure.ByReference {
    public ResultCArrayFieldElementByReference() {
      super();
    }

    public ResultCArrayFieldElementByReference(Pointer p) {
      super(p);
    }

    public ResultCArrayFieldElementTag tag;
    public CArrayFieldElement okcarrayfieldelement;
    public Error errcarrayfieldelement;

  }



  class Resultc_charTag extends IntegerType {
    public Resultc_charTag() {
      super(8, true);
    }

    public Resultc_charTag(long value) {
      super(8, value, true);
    }

    public Resultc_charTag(Pointer p) {
      this(p.getLong(0));
    }
    public static final Resultc_charTag Okc_char = new Resultc_charTag(0);
    public static final Resultc_charTag Errc_char = new Resultc_charTag(1);

  }

  class Resultc_charTagByReference extends ByReference {
    public Resultc_charTagByReference() {
      super(8);
    }

    public Resultc_charTagByReference(Pointer p) {
      super(8);
      setPointer(p);
    }

    public Resultc_charTag getValue() {
      Pointer p = getPointer();
      return new Resultc_charTag(p.getLong(0));
    }

    public void setValue(Resultc_charTag value) {
      Pointer p = getPointer();
      p.setLong(0, value.longValue());
    }

  }

  @Structure.FieldOrder({"tag", "okc_char", "errc_char"})
  class Resultc_char extends Structure implements Structure.ByValue {
    public Resultc_char() {
      super();
    }

    public Resultc_char(Pointer p) {
      super(p);
    }

    public Resultc_charTag tag;
    public ByteByReference okc_char;
    public Error errc_char;

  }

  @Structure.FieldOrder({"tag", "okc_char", "errc_char"})
  class Resultc_charByReference extends Structure implements Structure.ByReference {
    public Resultc_charByReference() {
      super();
    }

    public Resultc_charByReference(Pointer p) {
      super(p);
    }

    public Resultc_charTag tag;
    public ByteByReference okc_char;
    public Error errc_char;

  }



  class ResultFieldElementTag extends IntegerType {
    public ResultFieldElementTag() {
      super(8, true);
    }

    public ResultFieldElementTag(long value) {
      super(8, value, true);
    }

    public ResultFieldElementTag(Pointer p) {
      this(p.getLong(0));
    }
    public static final ResultFieldElementTag OkFieldElement = new ResultFieldElementTag(0);
    public static final ResultFieldElementTag ErrFieldElement = new ResultFieldElementTag(1);

  }

  class ResultFieldElementTagByReference extends ByReference {
    public ResultFieldElementTagByReference() {
      super(8);
    }

    public ResultFieldElementTagByReference(Pointer p) {
      super(8);
      setPointer(p);
    }

    public ResultFieldElementTag getValue() {
      Pointer p = getPointer();
      return new ResultFieldElementTag(p.getLong(0));
    }

    public void setValue(ResultFieldElementTag value) {
      Pointer p = getPointer();
      p.setLong(0, value.longValue());
    }

  }

  @Structure.FieldOrder({"tag", "okfieldelement", "errfieldelement"})
  class ResultFieldElement extends Structure implements Structure.ByValue {
    public ResultFieldElement() {
      super();
    }

    public ResultFieldElement(Pointer p) {
      super(p);
    }

    public ResultFieldElementTag tag;
    public FieldElement okfieldelement;
    public Error errfieldelement;

  }

  @Structure.FieldOrder({"tag", "okfieldelement", "errfieldelement"})
  class ResultFieldElementByReference extends Structure implements Structure.ByReference {
    public ResultFieldElementByReference() {
      super();
    }

    public ResultFieldElementByReference(Pointer p) {
      super(p);
    }

    public ResultFieldElementTag tag;
    public FieldElement okfieldelement;
    public Error errfieldelement;

  }



  @Structure.FieldOrder({"r", "s"})
  class Signature extends Structure implements Structure.ByValue {
    public Signature() {
      super();
    }

    public Signature(Pointer p) {
      super(p);
    }


    /**
     * The `r` value of a signature
     */
    public FieldElement r;

    /**
     * The `s` value of a signature
     */
    public FieldElement s;

  }

  @Structure.FieldOrder({"r", "s"})
  class SignatureByReference extends Structure implements Structure.ByReference {
    public SignatureByReference() {
      super();
    }

    public SignatureByReference(Pointer p) {
      super(p);
    }


    /**
     * The `r` value of a signature
     */
    public FieldElement r;

    /**
     * The `s` value of a signature
     */
    public FieldElement s;

  }



  class ResultSignatureTag extends IntegerType {
    public ResultSignatureTag() {
      super(8, true);
    }

    public ResultSignatureTag(long value) {
      super(8, value, true);
    }

    public ResultSignatureTag(Pointer p) {
      this(p.getLong(0));
    }
    public static final ResultSignatureTag OkSignature = new ResultSignatureTag(0);
    public static final ResultSignatureTag ErrSignature = new ResultSignatureTag(1);

  }

  class ResultSignatureTagByReference extends ByReference {
    public ResultSignatureTagByReference() {
      super(8);
    }

    public ResultSignatureTagByReference(Pointer p) {
      super(8);
      setPointer(p);
    }

    public ResultSignatureTag getValue() {
      Pointer p = getPointer();
      return new ResultSignatureTag(p.getLong(0));
    }

    public void setValue(ResultSignatureTag value) {
      Pointer p = getPointer();
      p.setLong(0, value.longValue());
    }

  }

  @Structure.FieldOrder({"tag", "oksignature", "errsignature"})
  class ResultSignature extends Structure implements Structure.ByValue {
    public ResultSignature() {
      super();
    }

    public ResultSignature(Pointer p) {
      super(p);
    }

    public ResultSignatureTag tag;
    public Signature oksignature;
    public Error errsignature;

  }

  @Structure.FieldOrder({"tag", "oksignature", "errsignature"})
  class ResultSignatureByReference extends Structure implements Structure.ByReference {
    public ResultSignatureByReference() {
      super();
    }

    public ResultSignatureByReference(Pointer p) {
      super(p);
    }

    public ResultSignatureTag tag;
    public Signature oksignature;
    public Error errsignature;

  }



  class ResultProviderTag extends IntegerType {
    public ResultProviderTag() {
      super(8, true);
    }

    public ResultProviderTag(long value) {
      super(8, value, true);
    }

    public ResultProviderTag(Pointer p) {
      this(p.getLong(0));
    }
    public static final ResultProviderTag OkProvider = new ResultProviderTag(0);
    public static final ResultProviderTag ErrProvider = new ResultProviderTag(1);

  }

  class ResultProviderTagByReference extends ByReference {
    public ResultProviderTagByReference() {
      super(8);
    }

    public ResultProviderTagByReference(Pointer p) {
      super(8);
      setPointer(p);
    }

    public ResultProviderTag getValue() {
      Pointer p = getPointer();
      return new ResultProviderTag(p.getLong(0));
    }

    public void setValue(ResultProviderTag value) {
      Pointer p = getPointer();
      p.setLong(0, value.longValue());
    }

  }

  @Structure.FieldOrder({"tag", "okprovider", "errprovider"})
  class ResultProvider extends Structure implements Structure.ByValue {
    public ResultProvider() {
      super();
    }

    public ResultProvider(Pointer p) {
      super(p);
    }

    public ResultProviderTag tag;
    public ProviderByReference okprovider;
    public Error errprovider;

  }

  @Structure.FieldOrder({"tag", "okprovider", "errprovider"})
  class ResultProviderByReference extends Structure implements Structure.ByReference {
    public ResultProviderByReference() {
      super();
    }

    public ResultProviderByReference(Pointer p) {
      super(p);
    }

    public ResultProviderTag tag;
    public ProviderByReference okprovider;
    public Error errprovider;

  }



  class ResultAccountTag extends IntegerType {
    public ResultAccountTag() {
      super(8, true);
    }

    public ResultAccountTag(long value) {
      super(8, value, true);
    }

    public ResultAccountTag(Pointer p) {
      this(p.getLong(0));
    }
    public static final ResultAccountTag OkAccount = new ResultAccountTag(0);
    public static final ResultAccountTag ErrAccount = new ResultAccountTag(1);

  }

  class ResultAccountTagByReference extends ByReference {
    public ResultAccountTagByReference() {
      super(8);
    }

    public ResultAccountTagByReference(Pointer p) {
      super(8);
      setPointer(p);
    }

    public ResultAccountTag getValue() {
      Pointer p = getPointer();
      return new ResultAccountTag(p.getLong(0));
    }

    public void setValue(ResultAccountTag value) {
      Pointer p = getPointer();
      p.setLong(0, value.longValue());
    }

  }

  @Structure.FieldOrder({"tag", "okaccount", "erraccount"})
  class ResultAccount extends Structure implements Structure.ByValue {
    public ResultAccount() {
      super();
    }

    public ResultAccount(Pointer p) {
      super(p);
    }

    public ResultAccountTag tag;
    public AccountByReference okaccount;
    public Error erraccount;

  }

  @Structure.FieldOrder({"tag", "okaccount", "erraccount"})
  class ResultAccountByReference extends Structure implements Structure.ByReference {
    public ResultAccountByReference() {
      super();
    }

    public ResultAccountByReference(Pointer p) {
      super(p);
    }

    public ResultAccountTag tag;
    public AccountByReference okaccount;
    public Error erraccount;

  }



  @Structure.FieldOrder({"to", "selector", "calldata"})
  class Call extends Structure implements Structure.ByValue {
    public Call() {
      super();
    }

    public Call(Pointer p) {
      super(p);
    }

    public FieldElement to;
    public ByteByReference selector;
    public CArrayFieldElement calldata;

  }

  @Structure.FieldOrder({"to", "selector", "calldata"})
  class CallByReference extends Structure implements Structure.ByReference {
    public CallByReference() {
      super();
    }

    public CallByReference(Pointer p) {
      super(p);
    }

    public FieldElement to;
    public ByteByReference selector;
    public CArrayFieldElement calldata;

  }




  /**
   * Block hash, number or tag
   */
  class BlockIdTag extends IntegerType {
    public BlockIdTag() {
      super(8, true);
    }

    public BlockIdTag(long value) {
      super(8, value, true);
    }

    public BlockIdTag(Pointer p) {
      this(p.getLong(0));
    }
    public static final BlockIdTag Hash = new BlockIdTag(0);
    public static final BlockIdTag Number = new BlockIdTag(1);
    public static final BlockIdTag BlockTag_ = new BlockIdTag(2);

  }

  class BlockIdTagByReference extends ByReference {
    public BlockIdTagByReference() {
      super(8);
    }

    public BlockIdTagByReference(Pointer p) {
      super(8);
      setPointer(p);
    }

    public BlockIdTag getValue() {
      Pointer p = getPointer();
      return new BlockIdTag(p.getLong(0));
    }

    public void setValue(BlockIdTag value) {
      Pointer p = getPointer();
      p.setLong(0, value.longValue());
    }

  }

  @Structure.FieldOrder({"tag", "hash", "number", "blocktag_"})
  class BlockId extends Structure implements Structure.ByValue {
    public BlockId() {
      super();
    }

    public BlockId(Pointer p) {
      super(p);
    }

    public BlockIdTag tag;
    public FieldElement hash;
    public long number;
    public BlockTag blocktag_;

  }

  @Structure.FieldOrder({"tag", "hash", "number", "blocktag_"})
  class BlockIdByReference extends Structure implements Structure.ByReference {
    public BlockIdByReference() {
      super();
    }

    public BlockIdByReference(Pointer p) {
      super(p);
    }

    public BlockIdTag tag;
    public FieldElement hash;
    public long number;
    public BlockTag blocktag_;

  }


  ResultToriiClient client_new(ByteByReference torii_url, 
                               ByteByReference rpc_url, 
                               ByteByReference libp2p_relay_url, 
                               FieldElement world);

  void client_set_logger(ToriiClientByReference client, Callback logger);

  ResultCArrayu8 client_publish_message(ToriiClientByReference client, 
                                        ByteByReference message, 
                                        FieldElementByReference signature_felts, 
                                        _Size signature_felts_len);

  ResultCArrayEntity client_entities(ToriiClientByReference client, QueryByReference query);

  ResultCArrayEntity client_event_messages(ToriiClientByReference client, QueryByReference query);

  WorldMetadata client_metadata(ToriiClientByReference client);

  ResultSubscription client_on_entity_state_update(ToriiClientByReference client, 
                                                   EntityKeysClauseByReference clauses, 
                                                   _Size clauses_len, 
                                                   Callback callback);

  Resultbool client_update_entity_subscription(ToriiClientByReference client, 
                                               SubscriptionByReference subscription, 
                                               EntityKeysClauseByReference clauses, 
                                               _Size clauses_len);

  ResultSubscription client_on_event_message_update(ToriiClientByReference client, 
                                                    EntityKeysClauseByReference clauses, 
                                                    _Size clauses_len, 
                                                    Callback callback);

  Resultbool client_update_event_message_subscription(ToriiClientByReference client, 
                                                      SubscriptionByReference subscription, 
                                                      EntityKeysClauseByReference clauses, 
                                                      _Size clauses_len);

  ResultCArrayFieldElement bytearray_serialize(ByteByReference str);

  Resultc_char bytearray_deserialize(FieldElementByReference felts, _Size felts_len);

  FieldElement poseidon_hash(FieldElementByReference felts, _Size felts_len);

  ResultFieldElement get_selector_from_name(ByteByReference name);

  FieldElement get_selector_from_tag(ByteByReference tag);

  FieldElement starknet_keccak(ByteByReference bytes, _Size bytes_len);

  ResultFieldElement cairo_short_string_to_felt(ByteByReference str);

  Resultc_char parse_cairo_short_string(FieldElement felt);

  ResultFieldElement typed_data_encode(ByteByReference typed_data, FieldElement address);

  FieldElement signing_key_new();

  ResultSignature signing_key_sign(FieldElement private_key, FieldElement hash);

  FieldElement verifying_key_new(FieldElement signing_key);

  Resultbool verifying_key_verify(FieldElement verifying_key, FieldElement hash, Signature signature);

  ResultProvider provider_new(ByteByReference rpc_url);

  ResultAccount account_new(ProviderByReference rpc, 
                            FieldElement private_key, 
                            ByteByReference address);

  ResultCArrayFieldElement starknet_call(ProviderByReference provider, Call call, BlockId block_id);

  ResultAccount account_deploy_burner(ProviderByReference provider, 
                                      AccountByReference master_account, 
                                      FieldElement signing_key);

  FieldElement account_address(AccountByReference account);

  FieldElement account_chain_id(AccountByReference account);

  void account_set_block_id(AccountByReference account, BlockId block_id);

  ResultFieldElement account_nonce(AccountByReference account);

  ResultFieldElement account_execute_raw(AccountByReference account, 
                                         CallByReference calldata, 
                                         _Size calldata_len);

  Resultbool wait_for_transaction(ProviderByReference rpc, FieldElement txn_hash);

  FieldElement hash_get_contract_address(FieldElement class_hash, 
                                         FieldElement salt, 
                                         FieldElementByReference constructor_calldata, 
                                         _Size constructor_calldata_len, 
                                         FieldElement deployer_address);

  void subscription_cancel(SubscriptionByReference subscription);

  void client_free(ToriiClientByReference t);

  void provider_free(ProviderByReference rpc);

  void model_free(StructByReference model);

  void account_free(AccountByReference account);

  void ty_free(TyByReference ty);

  void entity_free(EntityByReference entity);

  void error_free(ErrorByReference error);

  void world_metadata_free(WorldMetadataByReference metadata);

  void carray_free(Pointer data, _Size data_len);

  void string_free(ByteByReference string);

  class _Size extends IntegerType {
    public _Size() {
      super(Native.POINTER_SIZE, true);
    }

    public _Size(long value) {
      super(Native.POINTER_SIZE, value, true);
    }

    public _Size(Pointer p) {
      this(Native.POINTER_SIZE == 8 ? p.getLong(0) : p.getInt(0));
    }

  }

  class _SizeByReference extends ByReference {
    public _SizeByReference() {
      super(Native.POINTER_SIZE);
    }

    public _SizeByReference(Pointer p) {
      super(Native.POINTER_SIZE);
      setPointer(p);
    }

    public _Size getValue() {
      Pointer p = getPointer();
      return new _Size(Native.POINTER_SIZE == 8 ? p.getLong(0) : p.getInt(0));
    }

    public void setValue(_Size value) {
      Pointer p = getPointer();
      if (Native.POINTER_SIZE == 8) { p.setLong(0, value.longValue()); } else { p.setInt(0, value.intValue()); }
    }

  }

  class _Boolean extends IntegerType {
    public _Boolean() {
      super(1, true);
    }

    public _Boolean(long value) {
      super(1, value, true);
    }

    public _Boolean(Pointer p) {
      this(p.getByte(0));
    }

    public static final _Boolean FALSE = new _Boolean(0);
    public static final _Boolean TRUE = new _Boolean(1);
  }

  class _BooleanByReference extends ByReference {
    public _BooleanByReference() {
      super(1);
    }

    public _BooleanByReference(Pointer p) {
      super(1);
      setPointer(p);
    }

    public _Boolean getValue() {
      Pointer p = getPointer();
      return new _Boolean(p.getByte(0));
    }

    public void setValue(_Boolean value) {
      Pointer p = getPointer();
      p.setByte(0, (byte)value.intValue());
    }

  }

}