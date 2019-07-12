use crate::ast::*;
use crate::parser::*;
//use nom::branch::*;
//use nom::combinator::*;
use nom::error::*;
use nom::{Err, IResult};

// -----------------------------------------------------------------------------

#[derive(Debug, Node)]
pub enum CastingType<'a> {
    SimpleType(Box<SimpleType<'a>>),
    ConstantPrimary(Box<ConstantPrimary<'a>>),
    Signing(Box<Signing<'a>>),
    String(Symbol<'a>),
    Const(Symbol<'a>),
}

#[derive(Debug, Node)]
pub enum DataType<'a> {
    Vector(DataTypeVector<'a>),
    Atom(DataTypeAtom<'a>),
    NonIntegerType(NonIntegerType<'a>),
    Union(Box<DataTypeUnion<'a>>),
    Enum(DataTypeEnum<'a>),
    String(Symbol<'a>),
    Chandle(Symbol<'a>),
    Virtual(DataTypeVirtual<'a>),
    Type(DataTypeType<'a>),
    ClassType(ClassType<'a>),
    Event(Symbol<'a>),
    PsCovergroupIdentifier(PsCovergroupIdentifier<'a>),
    TypeReference(Box<TypeReference<'a>>),
}

#[derive(Debug, Node)]
pub struct DataTypeVector<'a> {
    pub nodes: (
        IntegerVectorType<'a>,
        Option<Signing<'a>>,
        Vec<PackedDimension<'a>>,
    ),
}

#[derive(Debug, Node)]
pub struct DataTypeAtom<'a> {
    pub nodes: (IntegerAtomType<'a>, Option<Signing<'a>>),
}

#[derive(Debug, Node)]
pub struct DataTypeUnion<'a> {
    pub nodes: (
        StructUnion<'a>,
        Option<(Packed<'a>, Option<Signing<'a>>)>,
        Brace<'a, (StructUnionMember<'a>, Vec<StructUnionMember<'a>>)>,
        Vec<PackedDimension<'a>>,
    ),
}

#[derive(Debug, Node)]
pub struct Packed<'a> {
    pub nodes: (Symbol<'a>,),
}

#[derive(Debug, Node)]
pub struct DataTypeEnum<'a> {
    pub nodes: (
        Symbol<'a>,
        Option<EnumBaseType<'a>>,
        Brace<'a, List<Symbol<'a>, EnumNameDeclaration<'a>>>,
        Vec<PackedDimension<'a>>,
    ),
}

#[derive(Debug, Node)]
pub struct DataTypeVirtual<'a> {
    pub nodes: (
        Symbol<'a>,
        Option<Interface<'a>>,
        InterfaceIdentifier<'a>,
        Option<ParameterValueAssignment<'a>>,
        Option<(Symbol<'a>, ModportIdentifier<'a>)>,
    ),
}

#[derive(Debug, Node)]
pub struct Interface<'a> {
    pub nodes: (Symbol<'a>,),
}

#[derive(Debug, Node)]
pub struct DataTypeType<'a> {
    pub nodes: (
        Option<PackageScopeOrClassScope<'a>>,
        TypeIdentifier<'a>,
        Vec<PackedDimension<'a>>,
    ),
}

#[derive(Debug, Node)]
pub enum DataTypeOrImplicit<'a> {
    DataType(DataType<'a>),
    ImplicitDataType(ImplicitDataType<'a>),
}

#[derive(Debug, Node)]
pub struct ImplicitDataType<'a> {
    pub nodes: (Option<Signing<'a>>, Vec<PackedDimension<'a>>),
}

#[derive(Debug, Node)]
pub enum EnumBaseType<'a> {
    Atom(EnumBaseTypeAtom<'a>),
    Vector(EnumBaseTypeVector<'a>),
    Type(EnumBaseTypeType<'a>),
}

#[derive(Debug, Node)]
pub struct EnumBaseTypeAtom<'a> {
    pub nodes: (IntegerAtomType<'a>, Option<Signing<'a>>),
}

#[derive(Debug, Node)]
pub struct EnumBaseTypeVector<'a> {
    pub nodes: (
        IntegerVectorType<'a>,
        Option<Signing<'a>>,
        Option<PackedDimension<'a>>,
    ),
}

#[derive(Debug, Node)]
pub struct EnumBaseTypeType<'a> {
    pub nodes: (Identifier<'a>, Option<PackedDimension<'a>>),
}

#[derive(Debug, Node)]
pub struct EnumNameDeclaration<'a> {
    pub nodes: (
        EnumIdentifier<'a>,
        Option<Bracket<'a, (IntegralNumber<'a>, Option<(Symbol<'a>, IntegralNumber<'a>)>)>>,
        Option<(Symbol<'a>, ConstantExpression<'a>)>,
    ),
}

#[derive(Debug, Node)]
pub struct ClassScope<'a> {
    pub nodes: (ClassType<'a>, Symbol<'a>),
}

#[derive(Debug, Node)]
pub struct ClassType<'a> {
    pub nodes: (
        PsClassIdentifier<'a>,
        Option<ParameterValueAssignment<'a>>,
        Vec<(
            Symbol<'a>,
            Identifier<'a>,
            Option<ParameterValueAssignment<'a>>,
        )>,
    ),
}

#[derive(Debug, Node)]
pub enum IntegerType<'a> {
    IntegerVectorType(IntegerVectorType<'a>),
    IntegerAtomType(IntegerAtomType<'a>),
}

#[derive(Debug, Node)]
pub enum IntegerAtomType<'a> {
    Byte(Symbol<'a>),
    Shortint(Symbol<'a>),
    Int(Symbol<'a>),
    Longint(Symbol<'a>),
    Integer(Symbol<'a>),
    Time(Symbol<'a>),
}

#[derive(Debug, Node)]
pub enum IntegerVectorType<'a> {
    Bit(Symbol<'a>),
    Logic(Symbol<'a>),
    Reg(Symbol<'a>),
}

#[derive(Debug, Node)]
pub enum NonIntegerType<'a> {
    Shortreal(Symbol<'a>),
    Real(Symbol<'a>),
    Realtime(Symbol<'a>),
}

#[derive(Debug, Node)]
pub enum NetType<'a> {
    Supply0(Symbol<'a>),
    Supply1(Symbol<'a>),
    Tri(Symbol<'a>),
    Triand(Symbol<'a>),
    Trior(Symbol<'a>),
    Trireg(Symbol<'a>),
    Tri0(Symbol<'a>),
    Tri1(Symbol<'a>),
    Uwire(Symbol<'a>),
    Wire(Symbol<'a>),
    Wand(Symbol<'a>),
    Wor(Symbol<'a>),
}

#[derive(Debug, Node)]
pub enum NetPortType<'a> {
    DataType(NetPortTypeDataType<'a>),
    NetTypeIdentifier(NetTypeIdentifier<'a>),
    Interconnect(NetPortTypeInterconnect<'a>),
}

#[derive(Debug, Node)]
pub struct NetPortTypeDataType<'a> {
    pub nodes: (Option<NetType<'a>>, DataTypeOrImplicit<'a>),
}

#[derive(Debug, Node)]
pub struct NetPortTypeInterconnect<'a> {
    pub nodes: (Symbol<'a>, ImplicitDataType<'a>),
}

#[derive(Debug, Node)]
pub struct VariablePortType<'a> {
    pub nodes: (VarDataType<'a>,),
}

#[derive(Debug, Node)]
pub enum VarDataType<'a> {
    DataType(DataType<'a>),
    Var(VarDataTypeVar<'a>),
}

#[derive(Debug, Node)]
pub struct VarDataTypeVar<'a> {
    pub nodes: (Symbol<'a>, DataTypeOrImplicit<'a>),
}

#[derive(Debug, Node)]
pub enum Signing<'a> {
    Signed(Symbol<'a>),
    Unsigned(Symbol<'a>),
}

#[derive(Debug, Node)]
pub enum SimpleType<'a> {
    IntegerType(IntegerType<'a>),
    NonNonIntegerType(IntegerType<'a>),
    PsTypeIdentifier(PsTypeIdentifier<'a>),
    PsParameterIdentifier(PsParameterIdentifier<'a>),
}

#[derive(Debug, Node)]
pub struct StructUnionMember<'a> {
    pub nodes: (
        Vec<AttributeInstance<'a>>,
        Option<RandomQualifier<'a>>,
        DataTypeOrVoid<'a>,
        ListOfVariableDeclAssignments<'a>,
        Symbol<'a>,
    ),
}

#[derive(Debug, Node)]
pub enum DataTypeOrVoid<'a> {
    DataType(DataType<'a>),
    Void(Symbol<'a>),
}

#[derive(Debug, Node)]
pub enum StructUnion<'a> {
    Struct(Symbol<'a>),
    Union(Symbol<'a>),
    UnionTagged((Symbol<'a>, Symbol<'a>)),
}

#[derive(Debug, Node)]
pub enum TypeReference<'a> {
    Expression(TypeReferenceExpression<'a>),
    DataType(TypeReferenceDataType<'a>),
}

#[derive(Debug, Node)]
pub struct TypeReferenceExpression<'a> {
    pub nodes: (Symbol<'a>, Paren<'a, Expression<'a>>),
}

#[derive(Debug, Node)]
pub struct TypeReferenceDataType<'a> {
    pub nodes: (Symbol<'a>, Paren<'a, DataType<'a>>),
}

// -----------------------------------------------------------------------------

pub fn casting_type(s: Span) -> IResult<Span, CastingType> {
    Err(Err::Error(make_error(s, ErrorKind::Fix)))
}

pub fn data_type(s: Span) -> IResult<Span, DataType> {
    Err(Err::Error(make_error(s, ErrorKind::Fix)))
}

pub fn data_type_or_implicit(s: Span) -> IResult<Span, DataTypeOrImplicit> {
    Err(Err::Error(make_error(s, ErrorKind::Fix)))
}

pub fn implicit_data_type(s: Span) -> IResult<Span, ImplicitDataType> {
    Err(Err::Error(make_error(s, ErrorKind::Fix)))
}

pub fn enum_base_type(s: Span) -> IResult<Span, EnumBaseType> {
    Err(Err::Error(make_error(s, ErrorKind::Fix)))
}

pub fn enum_name_declaration(s: Span) -> IResult<Span, EnumNameDeclaration> {
    Err(Err::Error(make_error(s, ErrorKind::Fix)))
}

pub fn class_scope(s: Span) -> IResult<Span, ClassScope> {
    Err(Err::Error(make_error(s, ErrorKind::Fix)))
}

pub fn class_type(s: Span) -> IResult<Span, ClassType> {
    Err(Err::Error(make_error(s, ErrorKind::Fix)))
}

pub fn integer_type(s: Span) -> IResult<Span, IntegerType> {
    Err(Err::Error(make_error(s, ErrorKind::Fix)))
}

pub fn integer_atom_type(s: Span) -> IResult<Span, IntegerAtomType> {
    Err(Err::Error(make_error(s, ErrorKind::Fix)))
}

pub fn integer_vector_type(s: Span) -> IResult<Span, IntegerVectorType> {
    Err(Err::Error(make_error(s, ErrorKind::Fix)))
}

pub fn non_integer_type(s: Span) -> IResult<Span, NonIntegerType> {
    Err(Err::Error(make_error(s, ErrorKind::Fix)))
}

pub fn net_type(s: Span) -> IResult<Span, NetType> {
    Err(Err::Error(make_error(s, ErrorKind::Fix)))
}

pub fn net_port_type(s: Span) -> IResult<Span, NetPortType> {
    Err(Err::Error(make_error(s, ErrorKind::Fix)))
}

pub fn variable_port_type(s: Span) -> IResult<Span, VariablePortType> {
    Err(Err::Error(make_error(s, ErrorKind::Fix)))
}

pub fn var_data_type(s: Span) -> IResult<Span, VarDataType> {
    Err(Err::Error(make_error(s, ErrorKind::Fix)))
}

pub fn signing(s: Span) -> IResult<Span, Signing> {
    Err(Err::Error(make_error(s, ErrorKind::Fix)))
}

pub fn simple_type(s: Span) -> IResult<Span, SimpleType> {
    Err(Err::Error(make_error(s, ErrorKind::Fix)))
}

pub fn struct_union_member(s: Span) -> IResult<Span, StructUnionMember> {
    Err(Err::Error(make_error(s, ErrorKind::Fix)))
}

pub fn data_type_or_void(s: Span) -> IResult<Span, DataTypeOrVoid> {
    Err(Err::Error(make_error(s, ErrorKind::Fix)))
}

pub fn struct_union(s: Span) -> IResult<Span, StructUnion> {
    Err(Err::Error(make_error(s, ErrorKind::Fix)))
}

pub fn type_reference(s: Span) -> IResult<Span, TypeReference> {
    Err(Err::Error(make_error(s, ErrorKind::Fix)))
}
