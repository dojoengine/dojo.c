import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("dojo-c", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;


  class BlockTag extends IntegerType {
    public BlockTag() {
      super(4, true);
    }

    public BlockTag(long value) {
      super(4, value, true);
    }

    public BlockTag(Pointer p) {
      this(p.getInt(0));
    }
    public static final BlockTag Latest = new BlockTag(1);
    public static final BlockTag Pending = new BlockTag(2);

  }

  class BlockTagByReference extends ByReference {
    public BlockTagByReference() {
      super(4);
    }

    public BlockTagByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public BlockTag getValue() {
      Pointer p = getPointer();
      return new BlockTag(p.getInt(0));
    }

    public void setValue(BlockTag value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  class ComparisonOperator extends IntegerType {
    public ComparisonOperator() {
      super(4, true);
    }

    public ComparisonOperator(long value) {
      super(4, value, true);
    }

    public ComparisonOperator(Pointer p) {
      this(p.getInt(0));
    }
    public static final ComparisonOperator Eq = new ComparisonOperator(1);
    public static final ComparisonOperator Neq = new ComparisonOperator(2);
    public static final ComparisonOperator Gt = new ComparisonOperator(3);
    public static final ComparisonOperator Gte = new ComparisonOperator(4);
    public static final ComparisonOperator Lt = new ComparisonOperator(5);
    public static final ComparisonOperator Lte = new ComparisonOperator(6);

  }

  class ComparisonOperatorByReference extends ByReference {
    public ComparisonOperatorByReference() {
      super(4);
    }

    public ComparisonOperatorByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public ComparisonOperator getValue() {
      Pointer p = getPointer();
      return new ComparisonOperator(p.getInt(0));
    }

    public void setValue(ComparisonOperator value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  class LogicalOperator extends IntegerType {
    public LogicalOperator() {
      super(4, true);
    }

    public LogicalOperator(long value) {
      super(4, value, true);
    }

    public LogicalOperator(Pointer p) {
      this(p.getInt(0));
    }
    public static final LogicalOperator And = new LogicalOperator(1);
    public static final LogicalOperator Or = new LogicalOperator(2);

  }

  class LogicalOperatorByReference extends ByReference {
    public LogicalOperatorByReference() {
      super(4);
    }

    public LogicalOperatorByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public LogicalOperator getValue() {
      Pointer p = getPointer();
      return new LogicalOperator(p.getInt(0));
    }

    public void setValue(LogicalOperator value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  class PatternMatching extends IntegerType {
    public PatternMatching() {
      super(4, true);
    }

    public PatternMatching(long value) {
      super(4, value, true);
    }

    public PatternMatching(Pointer p) {
      this(p.getInt(0));
    }
    public static final PatternMatching FixedLen = new PatternMatching(0);
    public static final PatternMatching VariableLen = new PatternMatching(1);

  }

  class PatternMatchingByReference extends ByReference {
    public PatternMatchingByReference() {
      super(4);
    }

    public PatternMatchingByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public PatternMatching getValue() {
      Pointer p = getPointer();
      return new PatternMatching(p.getInt(0));
    }

    public void setValue(PatternMatching value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
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



  class ResultToriiClient extends IntegerType {
    public ResultToriiClient() {
      super(4, true);
    }

    public ResultToriiClient(long value) {
      super(4, value, true);
    }

    public ResultToriiClient(Pointer p) {
      this(p.getInt(0));
    }
    public static final ResultToriiClient OkToriiClient = new ResultToriiClient(1);
    public static final ResultToriiClient ErrToriiClient = new ResultToriiClient(2);

  }

  class ResultToriiClientByReference extends ByReference {
    public ResultToriiClientByReference() {
      super(4);
    }

    public ResultToriiClientByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public ResultToriiClient getValue() {
      Pointer p = getPointer();
      return new ResultToriiClient(p.getInt(0));
    }

    public void setValue(ResultToriiClient value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  @Structure.FieldOrder({"data"})
  class FieldElement extends Structure implements Structure.ByValue {
    public FieldElement() {
      super();
    }

    public FieldElement(Pointer p) {
      super(p);
    }

    public byte[] data;

  }

  @Structure.FieldOrder({"data"})
  class FieldElementByReference extends Structure implements Structure.ByReference {
    public FieldElementByReference() {
      super();
    }

    public FieldElementByReference(Pointer p) {
      super(p);
    }

    public byte[] data;

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



  class ResultCArrayu8 extends IntegerType {
    public ResultCArrayu8() {
      super(4, true);
    }

    public ResultCArrayu8(long value) {
      super(4, value, true);
    }

    public ResultCArrayu8(Pointer p) {
      this(p.getInt(0));
    }
    public static final ResultCArrayu8 OkCArrayu8 = new ResultCArrayu8(1);
    public static final ResultCArrayu8 ErrCArrayu8 = new ResultCArrayu8(2);

  }

  class ResultCArrayu8ByReference extends ByReference {
    public ResultCArrayu8ByReference() {
      super(4);
    }

    public ResultCArrayu8ByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public ResultCArrayu8 getValue() {
      Pointer p = getPointer();
      return new ResultCArrayu8(p.getInt(0));
    }

    public void setValue(ResultCArrayu8 value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  class Primitive extends IntegerType {
    public Primitive() {
      super(4, true);
    }

    public Primitive(long value) {
      super(4, value, true);
    }

    public Primitive(Pointer p) {
      this(p.getInt(0));
    }
    public static final Primitive I8 = new Primitive(1);
    public static final Primitive I16 = new Primitive(2);
    public static final Primitive I32 = new Primitive(3);
    public static final Primitive I64 = new Primitive(4);
    public static final Primitive I128 = new Primitive(5);
    public static final Primitive U8 = new Primitive(6);
    public static final Primitive U16 = new Primitive(7);
    public static final Primitive U32 = new Primitive(8);
    public static final Primitive U64 = new Primitive(9);
    public static final Primitive U128 = new Primitive(10);
    public static final Primitive U256 = new Primitive(11);
    public static final Primitive U256 = new Primitive(12);
    public static final Primitive USize = new Primitive(13);
    public static final Primitive Bool = new Primitive(14);
    public static final Primitive Felt252 = new Primitive(15);
    public static final Primitive ClassHash = new Primitive(16);
    public static final Primitive ContractAddress = new Primitive(17);

  }

  class PrimitiveByReference extends ByReference {
    public PrimitiveByReference() {
      super(4);
    }

    public PrimitiveByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public Primitive getValue() {
      Pointer p = getPointer();
      return new Primitive(p.getInt(0));
    }

    public void setValue(Primitive value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

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



  class Ty extends IntegerType {
    public Ty() {
      super(4, true);
    }

    public Ty(long value) {
      super(4, value, true);
    }

    public Ty(Pointer p) {
      this(p.getInt(0));
    }
    public static final Ty Primitive_ = new Ty(1);
    public static final Ty Struct_ = new Ty(2);
    public static final Ty Enum_ = new Ty(3);
    public static final Ty Tuple_ = new Ty(4);
    public static final Ty Array_ = new Ty(5);
    public static final Ty ByteArray = new Ty(6);

  }

  class TyByReference extends ByReference {
    public TyByReference() {
      super(4);
    }

    public TyByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public Ty getValue() {
      Pointer p = getPointer();
      return new Ty(p.getInt(0));
    }

    public void setValue(Ty value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

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



  class ResultCArrayEntity extends IntegerType {
    public ResultCArrayEntity() {
      super(4, true);
    }

    public ResultCArrayEntity(long value) {
      super(4, value, true);
    }

    public ResultCArrayEntity(Pointer p) {
      this(p.getInt(0));
    }
    public static final ResultCArrayEntity OkCArrayEntity = new ResultCArrayEntity(1);
    public static final ResultCArrayEntity ErrCArrayEntity = new ResultCArrayEntity(2);

  }

  class ResultCArrayEntityByReference extends ByReference {
    public ResultCArrayEntityByReference() {
      super(4);
    }

    public ResultCArrayEntityByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public ResultCArrayEntity getValue() {
      Pointer p = getPointer();
      return new ResultCArrayEntity(p.getInt(0));
    }

    public void setValue(ResultCArrayEntity value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  class COptionFieldElement extends IntegerType {
    public COptionFieldElement() {
      super(4, true);
    }

    public COptionFieldElement(long value) {
      super(4, value, true);
    }

    public COptionFieldElement(Pointer p) {
      this(p.getInt(0));
    }
    public static final COptionFieldElement SomeFieldElement = new COptionFieldElement(1);
    public static final COptionFieldElement NoneFieldElement = new COptionFieldElement(2);

  }

  class COptionFieldElementByReference extends ByReference {
    public COptionFieldElementByReference() {
      super(4);
    }

    public COptionFieldElementByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public COptionFieldElement getValue() {
      Pointer p = getPointer();
      return new COptionFieldElement(p.getInt(0));
    }

    public void setValue(COptionFieldElement value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

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
    public Primitive value;

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
    public Primitive value;

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



  class Clause extends IntegerType {
    public Clause() {
      super(4, true);
    }

    public Clause(long value) {
      super(4, value, true);
    }

    public Clause(Pointer p) {
      this(p.getInt(0));
    }
    public static final Clause Keys = new Clause(1);
    public static final Clause CMember = new Clause(2);
    public static final Clause Composite = new Clause(3);

  }

  class ClauseByReference extends ByReference {
    public ClauseByReference() {
      super(4);
    }

    public ClauseByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public Clause getValue() {
      Pointer p = getPointer();
      return new Clause(p.getInt(0));
    }

    public void setValue(Clause value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  class COptionClause extends IntegerType {
    public COptionClause() {
      super(4, true);
    }

    public COptionClause(long value) {
      super(4, value, true);
    }

    public COptionClause(Pointer p) {
      this(p.getInt(0));
    }
    public static final COptionClause SomeClause = new COptionClause(1);
    public static final COptionClause NoneClause = new COptionClause(2);

  }

  class COptionClauseByReference extends ByReference {
    public COptionClauseByReference() {
      super(4);
    }

    public COptionClauseByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public COptionClause getValue() {
      Pointer p = getPointer();
      return new COptionClause(p.getInt(0));
    }

    public void setValue(COptionClause value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

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



  class ResultSubscription extends IntegerType {
    public ResultSubscription() {
      super(4, true);
    }

    public ResultSubscription(long value) {
      super(4, value, true);
    }

    public ResultSubscription(Pointer p) {
      this(p.getInt(0));
    }
    public static final ResultSubscription OkSubscription = new ResultSubscription(1);
    public static final ResultSubscription ErrSubscription = new ResultSubscription(2);

  }

  class ResultSubscriptionByReference extends ByReference {
    public ResultSubscriptionByReference() {
      super(4);
    }

    public ResultSubscriptionByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public ResultSubscription getValue() {
      Pointer p = getPointer();
      return new ResultSubscription(p.getInt(0));
    }

    public void setValue(ResultSubscription value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  class EntityKeysClause extends IntegerType {
    public EntityKeysClause() {
      super(4, true);
    }

    public EntityKeysClause(long value) {
      super(4, value, true);
    }

    public EntityKeysClause(Pointer p) {
      this(p.getInt(0));
    }
    public static final EntityKeysClause HashedKeys = new EntityKeysClause(1);
    public static final EntityKeysClause EntityKeys = new EntityKeysClause(2);

  }

  class EntityKeysClauseByReference extends ByReference {
    public EntityKeysClauseByReference() {
      super(4);
    }

    public EntityKeysClauseByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public EntityKeysClause getValue() {
      Pointer p = getPointer();
      return new EntityKeysClause(p.getInt(0));
    }

    public void setValue(EntityKeysClause value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  class Resultbool extends IntegerType {
    public Resultbool() {
      super(4, true);
    }

    public Resultbool(long value) {
      super(4, value, true);
    }

    public Resultbool(Pointer p) {
      this(p.getInt(0));
    }
    public static final Resultbool Okbool = new Resultbool(1);
    public static final Resultbool Errbool = new Resultbool(2);

  }

  class ResultboolByReference extends ByReference {
    public ResultboolByReference() {
      super(4);
    }

    public ResultboolByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public Resultbool getValue() {
      Pointer p = getPointer();
      return new Resultbool(p.getInt(0));
    }

    public void setValue(Resultbool value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  class ResultCArrayFieldElement extends IntegerType {
    public ResultCArrayFieldElement() {
      super(4, true);
    }

    public ResultCArrayFieldElement(long value) {
      super(4, value, true);
    }

    public ResultCArrayFieldElement(Pointer p) {
      this(p.getInt(0));
    }
    public static final ResultCArrayFieldElement OkCArrayFieldElement = new ResultCArrayFieldElement(1);
    public static final ResultCArrayFieldElement ErrCArrayFieldElement = new ResultCArrayFieldElement(2);

  }

  class ResultCArrayFieldElementByReference extends ByReference {
    public ResultCArrayFieldElementByReference() {
      super(4);
    }

    public ResultCArrayFieldElementByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public ResultCArrayFieldElement getValue() {
      Pointer p = getPointer();
      return new ResultCArrayFieldElement(p.getInt(0));
    }

    public void setValue(ResultCArrayFieldElement value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  class Resultc_char extends IntegerType {
    public Resultc_char() {
      super(4, true);
    }

    public Resultc_char(long value) {
      super(4, value, true);
    }

    public Resultc_char(Pointer p) {
      this(p.getInt(0));
    }
    public static final Resultc_char Okc_char = new Resultc_char(1);
    public static final Resultc_char Errc_char = new Resultc_char(2);

  }

  class Resultc_charByReference extends ByReference {
    public Resultc_charByReference() {
      super(4);
    }

    public Resultc_charByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public Resultc_char getValue() {
      Pointer p = getPointer();
      return new Resultc_char(p.getInt(0));
    }

    public void setValue(Resultc_char value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  class ResultFieldElement extends IntegerType {
    public ResultFieldElement() {
      super(4, true);
    }

    public ResultFieldElement(long value) {
      super(4, value, true);
    }

    public ResultFieldElement(Pointer p) {
      this(p.getInt(0));
    }
    public static final ResultFieldElement OkFieldElement = new ResultFieldElement(1);
    public static final ResultFieldElement ErrFieldElement = new ResultFieldElement(2);

  }

  class ResultFieldElementByReference extends ByReference {
    public ResultFieldElementByReference() {
      super(4);
    }

    public ResultFieldElementByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public ResultFieldElement getValue() {
      Pointer p = getPointer();
      return new ResultFieldElement(p.getInt(0));
    }

    public void setValue(ResultFieldElement value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

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



  class ResultSignature extends IntegerType {
    public ResultSignature() {
      super(4, true);
    }

    public ResultSignature(long value) {
      super(4, value, true);
    }

    public ResultSignature(Pointer p) {
      this(p.getInt(0));
    }
    public static final ResultSignature OkSignature = new ResultSignature(1);
    public static final ResultSignature ErrSignature = new ResultSignature(2);

  }

  class ResultSignatureByReference extends ByReference {
    public ResultSignatureByReference() {
      super(4);
    }

    public ResultSignatureByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public ResultSignature getValue() {
      Pointer p = getPointer();
      return new ResultSignature(p.getInt(0));
    }

    public void setValue(ResultSignature value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  class ResultProvider extends IntegerType {
    public ResultProvider() {
      super(4, true);
    }

    public ResultProvider(long value) {
      super(4, value, true);
    }

    public ResultProvider(Pointer p) {
      this(p.getInt(0));
    }
    public static final ResultProvider OkProvider = new ResultProvider(1);
    public static final ResultProvider ErrProvider = new ResultProvider(2);

  }

  class ResultProviderByReference extends ByReference {
    public ResultProviderByReference() {
      super(4);
    }

    public ResultProviderByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public ResultProvider getValue() {
      Pointer p = getPointer();
      return new ResultProvider(p.getInt(0));
    }

    public void setValue(ResultProvider value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  class ResultAccount extends IntegerType {
    public ResultAccount() {
      super(4, true);
    }

    public ResultAccount(long value) {
      super(4, value, true);
    }

    public ResultAccount(Pointer p) {
      this(p.getInt(0));
    }
    public static final ResultAccount OkAccount = new ResultAccount(1);
    public static final ResultAccount ErrAccount = new ResultAccount(2);

  }

  class ResultAccountByReference extends ByReference {
    public ResultAccountByReference() {
      super(4);
    }

    public ResultAccountByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public ResultAccount getValue() {
      Pointer p = getPointer();
      return new ResultAccount(p.getInt(0));
    }

    public void setValue(ResultAccount value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

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
  class BlockId extends IntegerType {
    public BlockId() {
      super(4, true);
    }

    public BlockId(long value) {
      super(4, value, true);
    }

    public BlockId(Pointer p) {
      this(p.getInt(0));
    }
    public static final BlockId Hash = new BlockId(1);
    public static final BlockId Number = new BlockId(2);
    public static final BlockId BlockTag_ = new BlockId(3);

  }

  class BlockIdByReference extends ByReference {
    public BlockIdByReference() {
      super(4);
    }

    public BlockIdByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public BlockId getValue() {
      Pointer p = getPointer();
      return new BlockId(p.getInt(0));
    }

    public void setValue(BlockId value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

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