pub struct Module {
    types: Vec<FuncTypes>,
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

struct FuncTypes {
    in_types: Vec<ValType>,
    out_types: Vec<ValType>,
}

struct Func {
    typeidx: u32,
    locals: Vec<ValType>,
    body: Vec<Instruction>,
}

struct Table {
    limit: Limit,
    reftype: RefType,
}

struct Mem {
    limit: Limit,
}

struct Global {
    
}

struct Elem {
    
}

struct Data {
    
}

struct Import {
    
}


struct Export {
    name: String,
}

struct Instruction {
    
}

struct Limit {
    min: u32,
    max: Option<u32>,
}

enum ExportType {
    Func,
    Table,
    Mem,
    Global,
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
