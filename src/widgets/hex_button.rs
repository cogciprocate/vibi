use elmesque::{ Form };
use conrod::{ CharacterCache, Color, Colorable, CommonBuilder, DrawArgs, Element, FontSize,
	GlyphCache, Labelable, Dimensions, Mouse, Point, Scalar, Theme, UpdateArgs, Widget, }; 
	

/// The type upon which we'll implement the `Widget` trait.
pub struct HexButton<'a, F> {
	/// An object that handles some of the dirty work of rendering a GUI. We don't
	/// really have to worry about it.
	common: CommonBuilder,
	/// Optional label string for the button.
	maybe_label: Option<&'a str>,
	/// Optional callback for when the button is pressed. If you want the button to
	/// do anything, this callback must exist.
	maybe_react: Option<F>,
	/// See the Style struct below.
	style: Style,
	/// Whether the button is currently enabled, i.e. whether it responds to
	/// user input.
	enabled: bool,
	/// Orientation.
	orientation: f64,
}

impl<'a, F> HexButton<'a, F> {
	/// Create a button context to be built upon.
	pub fn new() -> HexButton<'a, F> {
		HexButton {
			common: CommonBuilder::new(),
			maybe_react: None,
			maybe_label: None,
			style: Style::new(),
			enabled: true,
			orientation: 0.0,
		}
	}

	/// Set the reaction for the Button. The reaction will be triggered upon release
	/// of the button. Like other Conrod configs, this returns self for chainability.
	pub fn react(mut self, reaction: F) -> Self {
		self.maybe_react = Some(reaction);
		self
	}

	/// If true, will allow user inputs.  If false, will disallow user inputs.  Like
	/// other Conrod configs, this returns self for chainability. Allow dead code
	/// because we never call this in the example.
	#[allow(dead_code)]
	pub fn enabled(mut self, flag: bool) -> Self {
		self.enabled = flag;
		self
	}

	/// Sets orientation (rotation) of the widget.
	#[allow(dead_code)]
	pub fn orient(mut self, degrees: f64) -> Self {
		self.orientation = degrees;
		self
	}
}

/// A custom Conrod widget must implement the Widget trait. See the **Widget** trait
/// documentation for more details.
impl<'a, F> Widget for HexButton<'a, F>
	where F: FnMut()
{
	/// The State struct that we defined above.
	type State = State;

	/// The Style struct that we defined above.
	type Style = Style;

	fn common(&self) -> &CommonBuilder { &self.common }
	fn common_mut(&mut self) -> &mut CommonBuilder { &mut self.common }
	fn unique_kind(&self) -> &'static str { "HexButton" }
	fn init_state(&self) -> State {
		State { maybe_label: None, interaction: Interaction::Normal, orientation: 0.0 }
	}
	fn style(&self) -> Style { self.style.clone() }

	/// Default width of the widget. This method is optional. The Widget trait
	/// provides a default implementation that always returns zero.
	fn default_width<C: CharacterCache>(&self, theme: &Theme, _: &GlyphCache<C>) -> Scalar {
		const DEFAULT_WIDTH: Scalar = 64.0;

		// If no width was given via the `Sizeable` (a trait implemented for all widgets)
		// methods, some default width must be chosen.
		//
		// Defaults can come from several places. Here, we define how certain defaults take
		// precedence over others.
		//
		// Most commonly, defaults are to be retrieved from the `Theme`, however in some cases
		// some other logic may need to be considered.
		theme.maybe_button.as_ref().map(|default| {
			default.common.maybe_width.unwrap_or(DEFAULT_WIDTH)
		}).unwrap_or(DEFAULT_WIDTH)
	}

	/// Default width of the widget. This method is optional. The Widget trait
	/// provides a default implementation that always returns zero.
	fn default_height(&self, theme: &Theme) -> Scalar {
		const DEFAULT_HEIGHT: Scalar = 64.0;

		// See default_width for comments on this logic.
		theme.maybe_button.as_ref().map(|default| {
			default.common.maybe_height.unwrap_or(DEFAULT_HEIGHT)
		}).unwrap_or(DEFAULT_HEIGHT)
	}

	/// Update the state of the button. The state may or may not have changed since
	/// the last update. (E.g. it may have changed because the user moused over the
	/// button.) If the state has changed, return the new state. Else, return None.
	fn update<C: CharacterCache>(mut self, args: UpdateArgs<Self, C>) {
		let UpdateArgs { state, rect, mut ui, .. } = args;
		let (xy, dim) = rect.xy_dim();
		let maybe_mouse = ui.input().maybe_mouse.map(|mouse| mouse.relative_to(xy));

		// Check whether or not a new interaction has occurred.
		let new_interaction = match (self.enabled, maybe_mouse) {
			(false, _) | (true, None) => Interaction::Normal,
			(true, Some(mouse)) => {
				// Conrod does us a favor by transforming mouse.xy into this widget's
				// local coordinate system. Because mouse.xy is in local coords,
				// we must also pass the circle center in local coords. Thus we pass
				// [0.0, 0.0] as the center.
				//
				// See above where we define is_over_hex.
				let is_over = is_over_hex([0.0, 0.0], mouse.xy, dim);

				// See above where we define get_new_interaction.
				get_new_interaction(is_over, state.view().interaction, mouse)
			},
		};

		// If the mouse was released over the button, react. state.interaction is the
		// button's state as of a moment ago. new_interaction is the updated state as
		// of right now. So this if statement is saying: If the button was clicked a
		// moment ago, and it's now highlighted, then the button has been activated.
		if let (Interaction::Clicked, Interaction::Highlighted) =
			(state.view().interaction, new_interaction)
		{
			// Recall that our HexButton struct includes maybe_react, which
			// stores either a reaction function or None. If maybe_react is Some, call
			// the function.
			if let Some(ref mut react) = self.maybe_react { react() }
		}

		// Here we check to see whether or not our button should capture the mouse.
		//
		// Widgets can "capture" user input. If the button captures the mouse, then mouse
		// events will only be seen by the button. Other widgets will not see mouse events
		// until the button uncaptures the mouse.
		match (state.view().interaction, new_interaction) {
			// If the user has pressed the button we capture the mouse.
			(Interaction::Highlighted, Interaction::Clicked) => {
				ui.capture_mouse();
			},
			// If the user releases the button, we uncapture the mouse.
			(Interaction::Clicked, Interaction::Highlighted) |
			(Interaction::Clicked, Interaction::Normal)      => {
				ui.uncapture_mouse();
			},
			_ => (),
		}

		// Whenever we call `state.update` (as below), a flag is set within our `State`
		// indicating that there has been some mutation and that our widget requires a
		// new `Element` (meaning that `Widget::draw` will be called again). Thus, we only want
		// to call `state.update` if there has been some change in order to only redraw our
		// `Element` when absolutely required.
		//
		// You can see how we do this below - we check if the state has changed before calling
		// `state.update`.

		// If the interaction has changed, set the new interaction.
		if state.view().interaction != new_interaction {
			state.update(|state| state.interaction = new_interaction);
		}

		// If the label has changed, set the new label.
		if state.view().maybe_label.as_ref().map(|label| &label[..]) != self.maybe_label {
			state.update(|state| {
				state.maybe_label = self.maybe_label.as_ref().map(|label| label.to_string());
			})
		}

		if state.view().orientation != self.orientation {
			state.update(|state| {
				state.orientation = self.orientation;
			})
		}
	}

	/// Construct and return a renderable `Element` for the given button state.
	fn draw<C: CharacterCache>(args: DrawArgs<Self, C>) -> Element {
		use elmesque::{ self, form, utils };
		use num::Float;

		// Unwrap the args and state structs into individual variables.
		let DrawArgs { rect, state, style, theme, .. } = args;
		let (xy, dim) = rect.xy_dim();

		// Retrieve the styling for the Element.
		let color = state.color(style.color(theme));

		// Construct the frame and inner rectangle forms. We assume that dim is a
		// square bounding box, thus 2 * radius == dim[0] == dim[1].
		let radius = dim[0] / 2.0;
		// let pressable_form: elmesque::Form = circle(radius).filled(color);

		let pressable_form: elmesque::Form = form::ngon(6, radius).filled(color)
			// .alpha((0.25 * secs.cos()) as f32)
			.rotate(utils::degrees(state.orientation));

		// If we have a label, construct its Form. Recall that State has maybe_label,
		// which may or may not store a String for some label.
		let maybe_label_form: Option<elmesque::Form> =
			// Convert the Option<&str> to an Option<elmesque::Form>.
			state.maybe_label.as_ref().map(|label_text| {
				use elmesque::text::Text;
				let label_color = style.label_color(theme);
				let size = style.label_font_size(theme);
				form::text(Text::from_string(label_text.to_string())
					.color(label_color).height(size as f64))
					.shift(xy[0].floor(), xy[1].floor())
			});

		// Construct the button's Form.
		let form_chain =
			// An Option can be converted into an Iterator. We do this because we want
			// to combine multiple Option<elmesque::Form>s into a single Iterator via
			// Iterator::chain.
			Some(pressable_form).into_iter()
			// Recall that we unwrapped xy from the State object above. We map over
			// the Option, shifting the inner value (if it exists) by the given xy
			// coordinates.
			.map(|form| form.shift(xy[0], xy[1]))
			// Iterator::chain accepts anything that implement IntoIterator.
			// maybe_label_form is an Option, which implements IntoIterator. So
			// we can pass maybe_label_form to chain, and we'll get an Iterator with
			// maybe_label_form as the last element. If our widget had more forms,
			// we could add them with additional calls to chain.
			.chain(maybe_label_form);

		// We now have an Iterator containing all our Option<elmesque::Form>s. Turn
		// the forms into a renderable elmesque::Element.
		form::collage(
			// Width of the Element.
			dim[0] as i32,
			// Height of the Element.
			dim[1] as i32,
			// Convert the Iterator to a Vec<elmesque::Form>. Each None will be
			// dropped. Each Some will be unwrapped into an elmesque::Form.
			form_chain.collect()
		)
	}

}

/// Provide the chainable color() configuration method.
impl<'a, F> Colorable for HexButton<'a, F> {
	fn color(mut self, color: Color) -> Self {
		self.style.maybe_color = Some(color);
		self
	}
}

/// Provide the chainable label(), label_color(), and label_font_size()
/// configuration methods.
impl<'a, F> Labelable<'a> for HexButton<'a, F> {
	fn label(mut self, text: &'a str) -> Self {
		self.maybe_label = Some(text);
		self
	}
	fn label_color(mut self, color: Color) -> Self {
		self.style.maybe_label_color = Some(color);
		self
	}
	fn label_font_size(mut self, size: FontSize) -> Self {
		self.style.maybe_label_font_size = Some(size);
		self
	}
}


/// Represents the unique styling for our HexButton widget.
#[derive(Clone, Debug, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Style {
	/// Color of the button.
	pub maybe_color: Option<Color>,
	/// Radius of the button.
	pub maybe_radius: Option<Scalar>,
	/// Color of the button's label.
	pub maybe_label_color: Option<Color>,
	/// Font size of the button's label.
	pub maybe_label_font_size: Option<u32>,
}

impl Style {
	/// Construct the default Style.
	pub fn new() -> Style {
		Style {
			maybe_color: None,
			maybe_radius: None,
			maybe_label_color: None,
			maybe_label_font_size: None,
		}
	}

	/// Get the Color for an Element.
	pub fn color(&self, theme: &Theme) -> Color {
		self.maybe_color.or(theme.maybe_button.as_ref().map(|default| {
			default.style.maybe_color.unwrap_or(theme.shape_color)
		})).unwrap_or(theme.shape_color)
	}

	/// Get the label Color for an Element.
	pub fn label_color(&self, theme: &Theme) -> Color {
		self.maybe_label_color.or(theme.maybe_button.as_ref().map(|default| {
			default.style.maybe_label_color.unwrap_or(theme.label_color)
		})).unwrap_or(theme.label_color)
	}

	/// Get the label font size for an Element.
	pub fn label_font_size(&self, theme: &Theme) -> FontSize {
		self.maybe_label_font_size.or(theme.maybe_button.as_ref().map(|default| {
			default.style.maybe_label_font_size.unwrap_or(theme.font_size_medium)
		})).unwrap_or(theme.font_size_medium)
	}
}


/// Represents the unique, cached state for our HexButton widget.
#[derive(Clone, Debug, PartialEq)]
pub struct State {
	maybe_label: Option<String>,
	/// The current interaction state. See the Interaction enum below. See also
	/// get_new_interaction below, where we define all the logic for transitioning
	/// between interaction states.
	interaction: Interaction,
	/// Orientation.
	orientation: f64,
}

impl State {
	/// Alter the widget color depending on the state.
	fn color(&self, color: Color) -> Color {
		match self.interaction {
			/// The base color as defined in the Style struct, or a default provided
			/// by the current Theme if the Style has no color.
			Interaction::Normal => color,
			/// The Color object (from Elmesque) can calculate a highlighted version
			/// of itself. We don't have to use it, though. We could specify any color
			/// we want.
			Interaction::Highlighted => color.highlighted(),
			/// Ditto for clicked.
			Interaction::Clicked => color.clicked(),
		}
	}	
}

/// A type to keep track of interaction between updates.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Interaction {
	Normal,
	Highlighted,
	Clicked,
}

/// Check the current interaction with the button. Takes into account whether the mouse is
/// over the button and the previous interaction state.
fn get_new_interaction(is_over: bool, prev: Interaction, mouse: Mouse) -> Interaction {
	use conrod::MouseButtonPosition::{Down, Up};
	use self::Interaction::{Normal, Highlighted, Clicked};
	match (is_over, prev, mouse.left.position) {
		// LMB is down over the button. But the button wasn't Highlighted last
		// update. This means the user clicked somewhere outside the button and
		// moved over the button holding LMB down. We do nothing in this case.
		(true,  Normal,  Down) => Normal,

		// LMB is down over the button. The button was either Highlighted or Clicked
		// last update. If it was highlighted before, that means the user clicked
		// just now, and we transition to the Clicked state. If it was clicked
		// before, that means the user is still holding LMB down from a previous
		// click, in which case the state remains Clicked.
		(true,  _,       Down) => Clicked,

		// LMB is up. The mouse is hovering over the button. Regardless of what the
		// state was last update, the state should definitely be Highlighted now.
		(true,  _,       Up)   => Highlighted,

		// LMB is down, the mouse is not over the button, but the previous state was
		// Clicked. That means the user clicked the button and then moved the mouse
		// outside the button while holding LMB down. The button stays Clicked.
		(false, Clicked, Down) => Clicked,

		// If none of the above applies, then nothing interesting is happening with
		// this button.
		_                      => Normal,
	}
}

/// Return whether or not a given point is over a hexagon at a given point on a
/// Cartesian plane. We use this to determine whether the mouse is over the button.
// [FIXME] Calculating for circle still.
pub fn is_over_hex(circ_center: Point, mouse_point: Point, dim: Dimensions) -> bool {
	// Offset vector from the center of the circle to the mouse.
	let offset = ::vecmath::vec2_sub(mouse_point, circ_center);

	// If the length of the offset vector is less than or equal to the circle's
	// radius, then the mouse is inside the circle. We assume that dim is a square
	// bounding box around the circle, thus 2 * radius == dim[0] == dim[1].
	::vecmath::vec2_len(offset) <= dim[0] / 2.0
}
