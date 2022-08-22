use strum_macros::{Display, IntoStaticStr};

#[derive(Debug, Default, Display, IntoStaticStr)]
#[strum(serialize_all = "lowercase")]
#[strum(ascii_case_insensitive)]
#[allow(dead_code)] // TODO: Remove
pub enum MossLanguage {
    #[default]
    C,
    CPP,
    JAVA,
    ML,
    PASCAL,
    ADA,
    LISP,
    SCHEME,
    HASKELL,
    FORTRAN,
    ASCII,
    VHDL,
    PERL,
    MATLAB,
    PYTHON,
    MIPS,
    PROLOG,
    SPICE,
    VB,
    CSHARP,
    MODULA2,
    A8086,
    JAVASCRIPT,
    PLSQL,
}
