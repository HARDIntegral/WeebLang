pub mod types {
    
    #[derive(Copy, Clone, Debug)]
    pub enum Type {
        ImportStatement,
        FromStatement,
        StructDeclarationKeyword,
        EnumDeclarationKeyword,
        TypedefKeyword,
        FunctionDeclarationKeyword,
        ReturnKeyword,

        IfKeyword,
        IfElseKeyword,
        ElseKeyword,
        ForKeyword,
        WhileKeyword,
        InKeyword,
        RangeKeyword,
        BreakKeyword,
        ContinueKeyword,

        LetKeyword,
        ConstKeyword,
        NumTypeKeyword,
        CharTypeKeyword,
        BoolTypeKeyword,
        StringTypeKeyword,
        NoneTypeKeyword,
        TrueTypeKeyword,
        FalseTypeKeyword,

        OpenBrace,
        CloseBrace,
        OpenParen,
        CloseParen,
        OpenCurl,
        CloseCurl,

        Comma,
        Dot,
        DoubleDot,

        Set,
        Add,
        AddSet,
        Sub,
        SubSet,
        Mult,
        MultSet,
        Div,
        DivSet,
        Mod,
        ModSet,
        Increment,
        Decrement,

        IsEqu,
        Not,
        Or,
        And,
        Less,
        LessEqu,
        Greater,
        GreaterEqu,

        NumLiteral,
        StringLiteral,
        NoneKeyword,

        Other
    }
}