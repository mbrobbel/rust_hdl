use vhdl_parser::ast::*;

pub trait Rewrite {
    fn rewrite(&self, ctx: &Context, shape: Shape) -> Option<String>;
}

pub struct Config {}
impl Default for Config {
    fn default() -> Config {
        Config {}
    }
}
pub struct Shape {}
impl Default for Shape {
    fn default() -> Shape {
        Shape {}
    }
}

pub struct Context<'a> {
    pub config: &'a Config,
}
impl<'a> Context<'a> {
    pub fn new(config: &'a Config) -> Context<'a> {
        Context { config: &config }
    }
}

/// ```
/// use vhdl_parser::ast::BaseSpecifier;
/// use vhdl_format::Rewrite;
/// use vhdl_format::Shape;
/// use vhdl_format::Config;
/// use vhdl_format::Context;
/// let config = Config::default();
/// let ctx = Context::new(&config);
/// assert_eq!("B", BaseSpecifier::B.rewrite(&ctx, Shape::default()).unwrap());
/// assert_eq!("O", BaseSpecifier::O.rewrite(&ctx, Shape::default()).unwrap());
/// assert_eq!("X", BaseSpecifier::X.rewrite(&ctx, Shape::default()).unwrap());
/// assert_eq!("UB", BaseSpecifier::UB.rewrite(&ctx, Shape::default()).unwrap());
/// assert_eq!("UO", BaseSpecifier::UO.rewrite(&ctx, Shape::default()).unwrap());
/// assert_eq!("UX", BaseSpecifier::UX.rewrite(&ctx, Shape::default()).unwrap());
/// assert_eq!("SB", BaseSpecifier::SB.rewrite(&ctx, Shape::default()).unwrap());
/// assert_eq!("SO", BaseSpecifier::SO.rewrite(&ctx, Shape::default()).unwrap());
/// assert_eq!("SX", BaseSpecifier::SX.rewrite(&ctx, Shape::default()).unwrap());
/// assert_eq!("D", BaseSpecifier::D.rewrite(&ctx, Shape::default()).unwrap());
/// ```
impl Rewrite for BaseSpecifier {
    fn rewrite(&self, ctx: &Context, shape: Shape) -> Option<String> {
        Some(format!("{:?}", self))
    }
}
