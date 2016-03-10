#![allow(dead_code)]

use glium::Surface;
use glium_text::{self, TextSystem, FontTexture, TextDisplay};
use glium::glutin::{ElementState, MouseButton, VirtualKeyCode};
use ui::{UiVertex, UiShape2d, TextProperties};
use util;
use window::{self, MainWindow, HandlerOption, MouseInputHandler, 
    KeyboardInputHandler, MouseInputEventResult, KeyboardInputEventResult, KeyboardState, TextBox, Button};

pub const ELEMENT_BASE_SCALE: f32 = 0.07;
pub const BORDER_SHADE: f32 = 0.1;
pub const DEPRESS_SHADE: f32 = 0.1;

// Notes:
//
// * 'raw' is intended to mean something based on a position which is
//   unscaled by the screen and generally has a height of roughly 1.0f32.
// * 'cur' is a pre-calculated value containing information about the
//   current screen state (such as its size) and is used as a cached value.
// * 'idz' is, as always, the index of item[0] within a larger set (think
//   memory location).



// pub struct UiElementBorderToggle {

// }

pub struct UiElementBorder {
    thickness: f32,
    color: [f32; 4],
    shape: UiShape2d,
    is_visible: bool,
}

// impl UiElementBorder {
//     pub fn thin_black() -> UiElementBorder {
//         UiElementBorder { thickness: 0.05, color: ui::C_BLACK }
//     }
// }

#[allow(dead_code)]
pub enum UiElementKind {
    Button(Button),
    Panel,
    TextBox(TextBox),
    TextField,
}

impl UiElementKind {
    pub fn is_depressable(&self) -> bool {
        match self {
            &UiElementKind::Button(_) | &UiElementKind::TextBox(_) => true,
            _ => false,
        }
    }

    // pub fn is_depressed(&self) -> bool {
    //     match self {
    //         &UiElementKind::Button(ref button) => button.is_depressed(),
    //         _ => false,
    //     }
    // }

    // pub fn depress(&mut self, depress: bool) {
    //     match self {
    //         &mut UiElementKind::Button(ref mut button) => button.depress(depress),
    //         _ => (),
    //     }    
    // }
}


// [FIXME]: TODO: 
// - Revamp 'new()' into builder style functions.
// - Clean up and consolidate stored positions, scales, etc.
pub struct UiElement {
    kind: UiElementKind,
    text: TextProperties,
    sub_elements: Vec<UiElement>,    
    shape: UiShape2d,
    is_depressed: bool,
    has_mouse_focus: bool,
    has_keybd_focus: bool,
    anchor_point: [f32; 3],
    anchor_ofs: [f32; 3], 
    base_scale: (f32, f32),
    cur_scale: [f32; 3],
    cur_center_pos: [f32; 3],        
    border: Option<UiElementBorder>,
    mouse_input_handler: HandlerOption<MouseInputHandler>,
    keyboard_input_handler: HandlerOption<KeyboardInputHandler>,
}

impl<'a> UiElement {
    // [FIXME]: TODO: Sort out the whole dual border color/thickness issue (create a ::new()).
    pub fn new(kind: UiElementKind, anchor_point: [f32; 3], anchor_ofs: [f32; 3], shape: UiShape2d,
            ) -> UiElement
    {
        verify_position(anchor_point);

        let border_thickness = 0.05;
        let border_color = util::adjust_color(shape.color, BORDER_SHADE);

        let border = Some(UiElementBorder { thickness: border_thickness, color: border_color,
            is_visible: false, shape: shape.as_border(border_thickness, border_color) });

        UiElement { 
            kind: kind,
            text: TextProperties::new(""),
            sub_elements: Vec::with_capacity(0),
            shape: shape,
            is_depressed: false,
            has_mouse_focus: false,
            has_keybd_focus: false,
            anchor_point: anchor_point,
            anchor_ofs: anchor_ofs,
            base_scale: (ELEMENT_BASE_SCALE, ELEMENT_BASE_SCALE),
            cur_scale: [0.0, 0.0, 0.0],
            cur_center_pos: [0.0, 0.0, 0.0],        
            
            // ***** OLD
            // border: None,
            // ***** OLD

            // ***** NEW
            border: border,
            // **** NEW

            mouse_input_handler: HandlerOption::None,
            keyboard_input_handler: HandlerOption::None,
        }
    }

    pub fn mouse_input_handler(mut self, mouse_input_handler: MouseInputHandler) -> UiElement {
        self.mouse_input_handler = HandlerOption::Fn(mouse_input_handler);
        self
    }

    pub fn keyboard_input_handler(mut self, keyboard_input_handler: KeyboardInputHandler) -> UiElement {
        if let HandlerOption::None = self.keyboard_input_handler {
                self.keyboard_input_handler = HandlerOption::Fn(keyboard_input_handler);
                self
        } else {
            panic!("UiElement::keyboard_input_handler(): Keyboard input already assigned \
                to: '{:?}'", self.keyboard_input_handler);
        }
    }

    pub fn sub(mut self, mut sub_element: UiElement) -> UiElement {
        sub_element.anchor_point[2] += window::SUBDEPTH;
        self.sub_elements.reserve_exact(1);

        if sub_element.keyboard_input_handler.is_some() {
            if let HandlerOption::None = self.keyboard_input_handler {
                let next_sub_ele_idx = self.sub_elements.len();
                self.keyboard_input_handler = HandlerOption::Sub(next_sub_ele_idx);
            } else {
                panic!("UiElement::sub(): Cannot assign a sub-element to handle keyboard \
                    input if it has already been assigned. Current assignment: '{:?}'."
                    , self.keyboard_input_handler);
            }
        }
        
        self.sub_elements.push(sub_element);
        self
    }

    pub fn text_string(mut self, text_string: &str) -> UiElement {
        self.text.string = text_string.to_string();
        self
    }

    pub fn text_color(mut self, color: (f32, f32, f32, f32)) -> UiElement {
        self.text.color = color;
        self
    }

    pub fn text_offset(mut self, element_offset: (f32, f32)) -> UiElement {
        self.text.element_offset = element_offset;
        self
    }

    pub fn border(mut self, thickness: f32, color: [f32; 4], is_visible: bool) -> UiElement {
        self.border = Some(UiElementBorder { thickness: thickness, color: color, 
            is_visible: is_visible, shape: self.shape.as_border(thickness, color)});
        self
    }

    pub fn vertices_raw(&self) -> &[UiVertex] {
        &self.shape.vertices[..]
    }

    pub fn indices_raw(&self) -> &[u16] {
        &self.shape.indices[..]
    }

    pub fn vertices(&mut self, window_dims: (u32, u32), ui_scale: f32) -> Vec<UiVertex> {
        // Element color:
        let color = if self.kind.is_depressable() && self.is_depressed {
                util::adjust_color(self.shape.color, DEPRESS_SHADE)
            } else {
                self.shape.color
            };

        // Aspect ratio:
        let ar = window_dims.0 as f32 / window_dims.1 as f32;        

        self.cur_scale = [self.base_scale.0 * ui_scale / ar, self.base_scale.1 * ui_scale, ui_scale];
        
        self.cur_center_pos = [
            self.anchor_point[0] + ((self.anchor_ofs[0] / ar) * ui_scale),
            self.anchor_point[1] + (self.anchor_ofs[1] * ui_scale),
            (self.anchor_point[2] + self.anchor_ofs[2]) * ui_scale,
        ];

        self.text.cur_scale = (
            self.cur_scale[0] * self.text.base_scale, 
            self.cur_scale[1] * self.text.base_scale,
        );

        self.text.cur_center_pos = (
            ((-self.cur_scale[0] * self.text.raw_width / 2.0) * self.text.base_scale) 
                + self.cur_center_pos[0]
                + (self.text.element_offset.0 * self.cur_scale[0]), 
            ((-self.cur_scale[1] / 2.0) * self.text.base_scale) 
                + self.cur_center_pos[1]
                + (self.text.element_offset.1 * self.cur_scale[1]), 
        );        

        // Add vertices for this element's shape:
        let mut vertices: Vec<UiVertex> = self.shape.vertices.iter().map(|&vrt| 
                vrt.transform(&self.cur_scale, &self.cur_center_pos)
                .color(color)
            ).collect();

        // If we have a border, create a "shadow" of our shape...
        if let Some(ref border) = self.border {
            let border_vertices: Vec<UiVertex> = if border.is_visible {
                border.shape.vertices.iter().map(|&vrt| 
                        vrt.transform(&self.cur_scale, &self.cur_center_pos)
                    ).collect()
            } else {
                self.shape.vertices.iter().map(|&vrt| 
                        vrt.transform(&self.cur_scale, &self.cur_center_pos)
                    ).collect()
            };

            vertices.extend_from_slice(&border_vertices);
        }

        for sub_ele in self.sub_elements.iter_mut() {
            vertices.extend_from_slice(&sub_ele.vertices(window_dims.clone(), ui_scale));
        }

        vertices
    }

    /// Returns the list of indices with 'vertex_idz' added to each one.
    pub fn indices(&self, mut vertex_idz: u16) -> Vec<u16> {
        // Add indices for this element's shape:
        let mut indices: Vec<u16> = self.shape.indices.iter().map(|&ind| ind + vertex_idz).collect();
        vertex_idz += self.shape.vertices.len() as u16;

        // Add indices for our border (shadow of normal shape), if applicable:
        if let Some(ref border) = self.border {
            let border_indices: Vec<u16> = 
                border.shape.indices.iter().map(|&ind| ind + vertex_idz).collect();

            indices.extend_from_slice(&border_indices);
            vertex_idz += border.shape.vertices.len() as u16;
        }

        // Add indices for each sub_element, if any:
        for sub_ele in self.sub_elements.iter() {
            indices.extend_from_slice(&sub_ele.indices(vertex_idz));
            vertex_idz += sub_ele.shape.vertices.len() as u16;
        }

        indices
    }

    pub fn draw_text<S: Surface>(&self, text_system: &TextSystem, target: &mut S,
                font_texture: &FontTexture) 
    {
        let text_display = TextDisplay::new(text_system, font_texture, 
            self.get_text());

        glium_text::draw(&text_display, text_system, target, 
            self.text_matrix(), self.text().get_color());

        for element in self.sub_elements.iter() {
            element.draw_text(text_system, target, font_texture);
        }
    }

    pub fn set_text_width(&mut self, ts: &TextSystem, ft: &FontTexture) {
        self.text.set_raw_width(ts, ft);
    }

    pub fn position(&self) -> [f32; 3] {
        self.cur_center_pos
    }

    pub fn scale(&self) -> [f32; 3] {
        self.cur_scale
    }

    pub fn get_text(&self) -> &str {
        &self.text.string
    }

    pub fn text(&self) -> &TextProperties {
        &self.text
    }

    // #[allow(dead_code)]
    // pub fn set_color(&mut self, color: [f32; 3]) {
    //     for vertex in self.shape.vertices.iter_mut() {
    //         vertex.set_color(color);
    //     }
    // }

    /// Sets whether or not the mouse cursor is hovering over this element.
    // [FIXME]: PENDING FUTURE INVESTIGATION:
    // ADDING OR REMOVING A BORDER TO THE LIST OF VERTICES CAUSES A CRASH.
    // INVESTIGATE.    
    pub fn set_mouse_focus(&mut self, has_focus: bool) {
        if let Some(ref mut border) = self.border {
            border.is_visible = has_focus;
        }

        if !has_focus {
            self.depress(false);
        }

        self.has_mouse_focus = has_focus;
    }

    pub fn text_matrix(&self) -> [[f32; 4]; 4] {
        self.text.matrix()
    }

    pub fn has_mouse_focus(&mut self, mouse_pos: (f32, f32)) -> bool {
        self.has_mouse_focus = mouse_pos.0 >= self.left_edge() && mouse_pos.0 <= self.right_edge()
            && mouse_pos.1 <= self.top_edge() && mouse_pos.1 >= self.bottom_edge();

        self.has_mouse_focus
    }

    pub fn set_keybd_focus(&mut self, has_focus: bool) {
        self.has_keybd_focus = has_focus;

        if let HandlerOption::Sub(ele_idx) = self.keyboard_input_handler {
            if let Some(ref mut border) = self.sub_elements[ele_idx].border {
                border.is_visible = has_focus;
            }
        }
    }

    // [FIXME]: Unused Vars.
    #[allow(unused_variables)]
    pub fn handle_mouse_input(&mut self, state: ElementState, button: MouseButton, 
                window: &mut MainWindow) -> MouseInputEventResult 
    {
        let mut result = MouseInputEventResult::None;

        if let MouseButton::Left = button {
            match state {
                ElementState::Pressed => {
                    self.depress(true);
                    result = MouseInputEventResult::RequestRedraw;
                },
                ElementState::Released => {
                    let was_clicked = self.is_depressed;
                    self.depress(false);

                    if was_clicked {
                        if let HandlerOption::Fn(ref mut mih) = self.mouse_input_handler {
                            match mih(state, button, window) {
                                MouseInputEventResult::None => (),
                                r @ _ => return r,
                            }
                        }
                    }                    

                    result = MouseInputEventResult::RequestRedraw;
                },
            }
        }

        result
    }

    // [FIXME]: Unused Vars.
    // [FIXME]: Error message (set up result type).
    #[allow(unused_variables)]
    pub fn handle_keyboard_input(&mut self, key_state: ElementState, vk_code: Option<VirtualKeyCode>, 
                kb_state: &KeyboardState, window: &mut MainWindow) -> KeyboardInputEventResult 
    {
        let result = match self.keyboard_input_handler {
            HandlerOption::Fn(ref mut kih) => kih(key_state, vk_code, kb_state, &mut self.text.string, window),
            HandlerOption::Sub(ele_idx) => {
                assert!(ele_idx < self.sub_elements.len(), "{}UiElement::handle_keyboard_input(): {}:{}",
                    module_path!(), column!(), line!());
                // print!("        Passing keyboard input, '{:?}::{:?}', to sub element '{}' --->", 
                //     key_state, vk_code, ele_idx);
                self.sub_elements[ele_idx].handle_keyboard_input(key_state, vk_code, kb_state, window);
                KeyboardInputEventResult::None
            },
            _ => KeyboardInputEventResult::None,
        };

        // match result {
        //     KeyboardInputEventResult::PushTextString(c) => {
        //         // println!("        KeyboardInputEventResult: {}", c);
        //         self.text.string.push(c);
        //     },
        //     KeyboardInputEventResult::PopTextString => { self.text.string.pop(); },
        //     _ => (),
        // }

        result
    }

    fn depress(&mut self, depress: bool) {
        self.is_depressed = depress;
    }

    ///////// [FIXME]: CACHE THIS STUFF PROPERLY ////////// 
    fn left_edge(&self) -> f32 {
        self.cur_center_pos[0] - (self.shape.radii.0 * self.cur_scale[0])
    }

    fn right_edge(&self) -> f32 {
        self.cur_center_pos[0] + (self.shape.radii.0 * self.cur_scale[0])
    }

    fn top_edge(&self) -> f32 {
        self.cur_center_pos[1] + (self.shape.radii.1 * self.cur_scale[1])
    }

    fn bottom_edge(&self) -> f32 {
        self.cur_center_pos[1] - (self.shape.radii.1 * self.cur_scale[1])
    }
    //////////////////////////////////////

}

// fn shift_and_scale(anchor_point: &[f32; 3], anchor_ofs: &[f32; 3], base_scale: &(f32, f32),
//             window_dims: (u32, u32), ui_scale: f32) 
//         -> ([f32; 3], [f32; 3])
// {
//     let ar = window_dims.0 as f32 / window_dims.1 as f32;    

//     let cur_scale = [(base_scale.0 * ui_scale) / ar, base_scale.1 * ui_scale, 1.0];
    
//     let cur_center_pos = [
//         anchor_point[0] + ((anchor_ofs[0] / ar) * ui_scale),
//         anchor_point[1] + (anchor_ofs[1] * ui_scale),
//         (anchor_point[2] + anchor_ofs[2]) * ui_scale,
//     ];

//     (cur_scale, cur_center_pos)
// }


// Ensure position is within -1.0 and 1.0 for x and y dims.
fn verify_position(position: [f32; 3]) {
    assert!((position[0] <= 1.0 && position[0] >= -1.0) 
            || (position[1] <= 1.0 && position[1] >= -1.0), 
        format!("UiElement::new(): Position out of range: [x: {}, y: {}, z:{}]. \
            'x' and 'y' must both be between -1.0 and 1.0.", 
            position[0], position[1], position[2])
    );
}
