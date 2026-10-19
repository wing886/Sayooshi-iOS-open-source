#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct Statement {
    pub line: i32,
    pub kind: StatementKind,
}

#[derive(Debug, Clone)]
pub enum StatementKind {
    Label(String),
    ZLabel(usize),
    Property(PropertyDecl),
    CommandDef(CommandDecl),
    Goto {
        kind: GotoKind,
        label: LabelRef,
        args: Vec<Argument>,
    },
    Return(Option<Expr>),
    If(Vec<(Option<Expr>, Vec<Statement>)>),
    For {
        init: Vec<Statement>,
        condition: Expr,
        step: Vec<Statement>,
        body: Vec<Statement>,
    },
    While {
        condition: Expr,
        body: Vec<Statement>,
    },
    Continue,
    Break,
    Switch {
        value: Expr,
        cases: Vec<(Expr, Vec<Statement>)>,
        default: Vec<Statement>,
    },
    Assign {
        left: ElementExpr,
        op: u8,
        right: Expr,
    },
    Command(Expr),
    Text(String),
    Name(String),
}

#[derive(Debug, Clone)]
pub struct PropertyDecl {
    pub name: String,
    pub form: i32,
    pub size: Option<Expr>,
}

#[derive(Debug, Clone)]
pub struct CommandDecl {
    pub name: String,
    pub args: Vec<PropertyDecl>,
    pub form: i32,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, Copy)]
pub enum GotoKind {
    Goto,
    Gosub,
    GosubStr,
}

#[derive(Debug, Clone)]
pub enum LabelRef {
    Named(String),
    Z(usize),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Int(i32),
    Str(String),
    List(Vec<Expr>),
    Element(ElementExpr),
    Goto {
        kind: GotoKind,
        label: LabelRef,
        args: Vec<Argument>,
    },
    Unary {
        op: u8,
        expr: Box<Expr>,
    },
    Binary {
        op: u8,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub struct ElementExpr {
    pub parts: Vec<ElementPart>,
}

#[derive(Debug, Clone)]
pub enum ElementPart {
    Name {
        name: String,
        args: Option<Vec<Argument>>,
    },
    Array(Expr),
}

#[derive(Debug, Clone)]
pub struct Argument {
    pub name: Option<String>,
    pub value: Expr,
}
