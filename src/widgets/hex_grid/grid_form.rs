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



	// Time to get creative!
	// group(vec![

	// 	rect(60.0, 40.0).filled(blue())
	// 		.shift(secs.sin() * 50.0, secs.cos() * 50.0)
	// 		.alpha(((secs * 200.0).cos() * 0.5 + 0.5) as f32)
	// 		.rotate(-secs),

	// 	rect(100.0, 10.0).filled(dark_blue())
	// 		.shift((secs * 5.0).sin() * 200.0, (secs * 5.0).cos() * 200.0)
	// 		.alpha(((secs * 2.0).cos() * 0.5 + 0.5) as f32)
	// 		.rotate(-(secs * 5.0)),

	// 	rect(10.0, 300.0).filled(blue())
	// 		.alpha(((secs * 3.0).sin() * 0.25 + 0.75) as f32)
	// 		.rotate(-(secs * 1.5)),

	// 	rect(5.0, (secs * 0.1).sin() * 600.0 + 300.0).filled(light_blue())
	// 		.alpha(((secs).cos() * 0.25 + 0.75) as f32)
	// 		.rotate(secs * 0.75),

	// 	rect(3.0, 2000.0).filled(dark_blue())
	// 		.alpha(((secs * 100.0).cos() * 0.5 + 0.25) as f32)
	// 		.rotate(-(secs * 0.5)),

	// 	oval(3.0, 2000.0 * (secs * 60.0).sin()).filled(light_blue())
	// 		.alpha(((secs * 100.0).cos() * 0.5 + 0.25) as f32)
	// 		.rotate(-(secs * 0.6)),

	// 	rect(10.0, 750.0).filled(blue())
	// 		.alpha(((secs * 2.0).cos() * 0.5 + 0.25) as f32)
	// 		.rotate(-(secs * 1.85)),

	// 	circle((secs * 0.5).sin() * 1500.0).outlined(solid(dark_purple()))
	// 		.alpha(((secs * 0.2).sin() * 0.25 + 0.35) as f32)
	// 		.rotate(-(secs * 0.5)),

	// 	ngon(12, (secs * 0.1).cos() * 100.0 + 300.0).filled(blue())
	// 		.alpha((0.25 * secs.cos()) as f32)
	// 		.rotate(secs * 0.5),

	// 	ngon(9, (secs * 0.1).cos() * 200.0 + 250.0).outlined(solid(dark_blue()))
	// 		.alpha(((0.33 * secs).sin() + 0.15) as f32)
	// 		.rotate(secs * 0.2),

	// 	rect(300.0, 20.0).filled(light_blue())
	// 		.shift((secs * 1.5).cos() * 250.0, (secs * 1.5).sin() * 250.0)
	// 		.alpha(((secs * 4.5).cos() * 0.25 + 0.35) as f32)
	// 		.rotate(secs * 1.5 + degrees(90.0)),

	// 	traced(
	// 		solid(light_blue()),
	// 		point_path(vec![(-500.0, 100.0), (0.0, 250.0 * secs.sin()), (500.0, 100.0)])
	// 	).alpha(((secs * 0.2).sin() * 0.25 + 0.35) as f32),
			
	// 	traced(
	// 		solid(blue()),
	// 		point_path(vec![(-500.0, 0.0), (0.0, 0.0), (500.0, 0.0)])
	// 	).alpha(((secs * 4.5).cos() * 0.25 + 0.35) as f32),

	// 	traced(
	// 		solid(dark_blue()),
	// 		point_path(vec![(-500.0, -100.0), (0.0, -250.0 * secs.sin()), (500.0, -100.0)])
	// 	).alpha(((secs * 0.15).cos() * 0.25 + 0.35) as f32),

	// 	text(Text::from_string(label).color(color::white())),

	// ]).rotate(degrees(secs.sin() * 360.0))
	//   .scale((secs * 0.05).cos() * 0.2 + 0.9)
	//   
