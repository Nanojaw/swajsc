pub struct Module {
    types: Vec<FuncType>,
    funcs: Vec<Func>,
    tables: Vec<Table>,
    mems: Vec<Mem>,
    globals: Vec<Global>,
    elems: Vec<Elem>,
    datas: Vec<Data>,
    start: Option<Func>,
    imports: Vec<Import>,
    exports: Vec<Export>,
}

struct FuncType {
    in_types: Vec<ValType>,
    out_types: Vec<ValType>,
}

struct Func {
    typeidx: u32,
    locals: Vec<ValType>,
    body: Expr,
}

struct Table {
    table_type: TableType,
}

struct Mem {
    limit: Limit,
}

struct Global {
    global_type: GlobalType,
    init: Expr,
}

struct Elem {
    elem_type: RefType,
    init: Vec<Expr>,
    mode: ElemMode,
}

struct Data {
    init: Vec<u8>,
}

struct Import {
    module_name: String,
    name: String,
    desc: ImportDesc,
}

struct Export {
    name: String,
    desc: ExportDesc,
}

// Subtypes

struct Instruction {}

struct Limit {
    min: u32,
    max: Option<u32>,
}

struct Expr {
    instrs: Vec<Instruction>,
}

struct TableType {
    limit: Limit,
    reftype: RefType,
}

struct GlobalType {
    val_type: ValType,
    mutable: bool,
}

// Enums

enum ExportDesc {
    Func(Func),
    Table(Table),
    Mem(Mem),
    Global(Global),
}

enum ImportDesc {
    Func(FuncType),
    Table(TableType),
    Mem(Limit),
    Global(GlobalType),
}

enum ElemMode {
    Passive,
    Active(Table, Expr),
    Declarative,
}

enum DataMode {
    Passive,
    Active(Mem, Expr),
}

enum ValType {
    NumType(NumType),
    VecType(VecType),
    RefType(RefType),
}

enum NumType {
    I32,
    F32,
    I64,
    F64,
}

enum VecType {
    I8x16,
    I16x8,
    I32x4,
    I64x2,

    F32x4,
    F64x2,

    V128,
}

enum RefType {
    FuncRef,
    ExternRef,
}
