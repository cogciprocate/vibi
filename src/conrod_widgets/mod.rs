pub use self::hex_button::{ HexButton };
pub use self::hex_grid::{ HexGrid };
pub use self::drinking_clock::{ DrinkingClock };

mod hex_button;
mod hex_grid;
mod drinking_clock;




////////////////////////// REFERENCE //////////////////////////////
//
// pub struct LineStyle {
//     pub color: Color,
//     pub width: f64,
//     pub cap: LineCap,
//     pub join: LineJoin,
//     pub dashing: Vec<i64>,
//     pub dash_offset: i64,
// }
// pub fn default() -> LineStyle {
//     LineStyle {
//         color: ::color::black(),
//         width: 1.0,
//         cap: LineCap::Flat,
//         join: LineJoin::Sharp(10.0),
//         dashing: Vec::new(),
//         dash_offset: 0,
//     }
// }
// 
// pub enum LineCap {
//     Flat,
//     Round,
//     Padded,
// }
// pub enum LineJoin {
//     Smooth,
//     Sharp(f64),
//     Clipped,
// }

//////// ADD SOME DOC TO ELMESQUE FOR THIS ///////
/////////////// FROM ELM SOURCE: /////////////////
//
// {-| The shape of the ends of a line. -}
// type LineCap = Flat | Round | Padded
// 
// 
// {-| The shape of the &ldquo;joints&rdquo; of a line, where each line segment
// meets. `Sharp` takes an argument to limit the length of the joint. This
// defaults to 10.
// -}
// type LineJoin = Smooth | Sharp Float | Clipped
// 
//////////////////////////////////////////////////
//
///////////////////////////////////////////////////////////////////
