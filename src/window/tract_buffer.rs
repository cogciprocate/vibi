use std::iter;
use std::sync::{Arc, Mutex};
use std::ops::Range;
use rand;
use rand::distributions::{IndependentSample, Range as RandRange};
use glium::backend::glutin_backend::{GlutinFacade};
use glium::VertexBuffer;
use glium::vertex::VertexBufferSlice;
use bismit::map::SliceTractMap;

/// Handles raw state data from a cortical ganglion and feeds it to a [vertex] buffer for rendering.
// TODO: Rename these buffers to something more clear.
pub struct TractBuffer {
    raw_states: Arc<Mutex<Vec<u8>>>,
    state_vertices: Vec<StateVertex>,
    vertex_buf: VertexBuffer<StateVertex>,
    total_slc_range: Range<u8>,
    default_slc_range: Range<u8>,
    cur_slc_range: Range<u8>,
    tract_map: SliceTractMap,
}

impl TractBuffer {
    pub fn new(_: Range<u8>, tract_map: SliceTractMap, display: &GlutinFacade) 
            -> TractBuffer 
    {
        // DEBUG: TEMPORARY:
        let default_slc_range = tract_map.slc_id_range();


        let grid_count = tract_map.axn_count(default_slc_range.clone());

        // println!("\n###### TractBuffer::new(): d_slc_range: {:?}, grid_count: {}, tract_map: {:?}", 
        //     default_slc_range, grid_count, tract_map);

        let raw_states = iter::repeat(0u8).cycle().take(grid_count).collect();
        let state_vertices: Vec<StateVertex> = iter::repeat(StateVertex { state: 0.0 })
            .cycle().take(grid_count).collect();
        let vertex_buf = VertexBuffer::dynamic(display, &state_vertices).unwrap();

        TractBuffer {            
            raw_states: Arc::new(Mutex::new(raw_states)),
            state_vertices: state_vertices,
            vertex_buf: vertex_buf,
            total_slc_range: tract_map.slc_id_range(),
            default_slc_range: default_slc_range.clone(),
            cur_slc_range: default_slc_range.clone(),
            tract_map: tract_map,
        }
    }

    /// Refreshes the per-instance data within our vertex buffer.
    ///
    ///  *If* a lock on `raw_states` can be obtained (if it's not currently being written to by another thread): converts the u8s in `raw_states` to floats, store them in `state_vertices`, then writes the contents of `state_vertices` to `vertex_buf`.
    ///
    /// This is an opportunistic refresh, it will sometimes do nothing at all.
    pub fn refresh_vertex_buf(&mut self) {
        // let raw_states_ptr = self.raw_states.clone();
        // let raw_states_res = raw_states_ptr.try_lock();

        // Change to .lock() for smoother refreshes at the cost of slower cycling:        
        // if let Ok(ref raw_states) = self.raw_states.try_lock() {
        if let Ok(ref raw_states) = self.raw_states.lock() {
            debug_assert!(raw_states.len() == self.state_vertices.len());

            for (&rs, ref mut sv) in raw_states.iter().zip(self.state_vertices.iter_mut()) {
                sv.state = (rs as f32) / 255.0;
            }

            self.vertex_buf.write(&self.state_vertices);
        }        
    }

    pub fn set_default_slc_range(&mut self, slc_range: Range<u8>) {
        self.default_slc_range = slc_range;
    }

    pub fn set_tract_map(&mut self, tract_map: SliceTractMap) {
        self.tract_map = tract_map;
    }
 
     // [FIXME]: DEPRICATE OR MOVE TO TESTS MODULE
    #[allow(dead_code)]
    pub fn fill_rand(&mut self) {
        let mut rng = rand::thread_rng();
        let range = RandRange::new(0u8, 255);

        // let raw_states_ptr = self.raw_states.clone();
        // let mut raw_states = raw_states_ptr.lock().unwrap();
        let mut raw_states = self.raw_states.lock().unwrap();

        for rs in raw_states.iter_mut() {
            *rs = range.ind_sample(&mut rng);
        }
    }

    // Determines receiving end size (length);
    pub fn raw_states(&mut self) -> Arc<Mutex<Vec<u8>>> {
        self.raw_states.clone()
    }

    /// Returns a slice of the vertex buffer corresponding to a ganglion slice id.
    pub fn vertex_buf(&self, gang_slc_id: u8) -> VertexBufferSlice<StateVertex> {
        let axn_id_range: Range<usize> = self.tract_map.axn_id_range(gang_slc_id..gang_slc_id + 1);

        // println!("\n###### TractBuffer::vertex_buf({}): axn_id_range: {:?}", 
        //     gang_slc_id, axn_id_range);

        self.vertex_buf.slice(axn_id_range).expect("TractBuffer::vertex_buf(): Out of range")
    }

    pub fn cur_slc_range(&self) -> Range<u8> {
        self.cur_slc_range.clone()
    }

    pub fn cur_axn_range(&self) -> Range<usize> {
        let cur_axn_range = self.tract_map.axn_id_range(self.cur_slc_range.clone());
        // println!("###### TractBuffer::cur_axn_range(): \
        //         self.cur_slc_range: {:?}, cur_axn_range: {:?}",
        //     self.cur_slc_range,
        //     cur_axn_range);

        cur_axn_range
    }

    pub fn tract_map(&self) -> &SliceTractMap {
        &self.tract_map
    }
}


#[derive(Copy, Clone, Debug)]
pub struct StateVertex {
    state: f32,
}

// impl Vertex {
//     fn new(position: [f32; 3], color: [f32; 3], normal: [f32; 3]) -> Vertex {
//         Vertex { position: position, color: color, normal: normal }
//     }
// }
implement_vertex!(StateVertex, state);
