#![allow(unused_imports, dead_code, unused_variables)]

use elmesque::{ form, color, utils, Form };
use elmesque::form::{ LineStyle, LineCap, LineJoin };
use elmesque::text::{ Text };
use num::{ Float };
use conrod::{ Color, Point };

const SQRT_3: f64 = 1.73205080756f64;
// const V_UNIT: Point = [1.5 * tile_radius, 2 * SQRT_3 


pub fn gen(secs: f64, label: String, color: Color, dims: &[f64; 2]) -> Form {
	let axial_dims = AxialDims { v_size: 33, u_size: 33 };
	let x_margin = 7.5;

	// let tile_radius = 16.0;
	let tile_radius = dims[0] / (((axial_dims.v_size as f64 + axial_dims.u_size as f64) * 1.5));

	let origin: Point = [(-dims[0] / 2.0) + tile_radius + x_margin, 0.0];

	// let line_style = LineStyle { 
	// 	color: color::grey(), 
	// 	width: tile_radius / 8.0,
	// 	// join: LineJoin::Smooth,
	// 	// cap: LineCap::Round,
	// 	..LineStyle::default() 
	// };

	let v_unit: Point = [1.5 * tile_radius, (SQRT_3 * tile_radius) / 2.0];
	let u_unit: Point = [1.5 * tile_radius, (SQRT_3 * tile_radius) / -2.0];

	// let mut grid_group = Vec::with_capacity(axial_dims.tile_count() as usize * 2);
	// let mut grid_group_outline = Vec::with_capacity(axial_dims.tile_count() as usize);
	let mut grid_group = Vec::with_capacity(axial_dims.tile_count() as usize);

	for v in 0..axial_dims.v_size {
		for u in 0..axial_dims.u_size {
			let x_shift = (v as f64 * v_unit[0]) + (u as f64 * u_unit[0]) + origin[0];
			let y_shift = (v as f64 * v_unit[1]) + (u as f64 * u_unit[1]) + origin[1];

			// let fill_clr = if (v & (u + 1) & 3) == 0 { 
			// 	color::blue()
			// } else {
			// 	color::blue().invert()
			// };

			// grid_group.push(
			// 	form::ngon(6, tile_radius)
			// 		.filled(fill_clr)
			// 		.shift(x_shift, y_shift)
			// );

			grid_group.push(
				form::ngon(6, tile_radius)
					// .outlined(line_style.clone())
					// .outlined(LineStyle::default())
					.outlined(form::solid(color::grey()))					
					.shift(x_shift, y_shift)
			);
		}
	}

	// grid_group.push_all(&grid_group_outline);

	// let x_shift = (0 as f64 * v_unit[0]) + (0 as f64 * u_unit[0]) + origin[0];
	// let y_shift = (0 as f64 * v_unit[1]) + (0 as f64 * u_unit[1]) + origin[1];

	form::group(grid_group)
	// form::group(vec![
	// 	form::ngon(6, tile_radius)
	// 		.outlined(form::solid(color::grey()))					
	// 		.shift(x_shift, y_shift)
	// ])
}



pub struct AxialDims {
	v_size: u32,
	u_size: u32,
}

impl AxialDims {
	pub fn tile_count(&self) -> u32 {
		self.v_size * self.u_size
	}
}
