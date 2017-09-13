use std::iter;
use std::sync::{Arc, Mutex};
use std::ops::Range;
use rand;
use rand::distributions::{IndependentSample, Range as RandRange};
use glium::backend::glutin_backend::{GlutinFacade};
use glium::vertex::{VertexBuffer, VertexBufferSlice};
use bismit::flywheel::AreaInfo;
use bismit::map::SliceTractMap;
use bismit::TractReceiver;

#[derive(Copy, Clone, Debug)]
// impl Vertex {
//     fn new(position: [f32; 3], color: [f32; 3], normal: [f32; 3]) -> Vertex {
//         Vertex { position: position, color: color, normal: normal }
//     }
// }
pub struct StateVertex {
    state: u8,
}
implement_vertex!(StateVertex, state);


/// Handles raw state data from a cortical ganglion and feeds it to a [vertex] buffer for rendering.
// TODO: Rename these buffers to something more clear.
#[allow(dead_code)]
pub struct HexGridBuffer {
    raw_states_vec: Arc<Mutex<Vec<u8>>>,
    raw_states_rx: Option<TractReceiver>,
    raw_states_buf: VertexBuffer<StateVertex>,
    full_slc_range: Range<usize>,
    default_slc_range: Range<usize>,
    cur_slc_range: Range<usize>,
    tract_map: SliceTractMap,
}

impl HexGridBuffer {
    pub fn new(area_info: AreaInfo, display: &GlutinFacade)
            -> HexGridBuffer
    {
        let full_slc_range = area_info.tract_map.slc_id_range();
        let grid_count = area_info.tract_map.axn_count(full_slc_range.clone());
        let raw_states_vec: Vec<u8> = iter::repeat(0u8).cycle().take(grid_count).collect();
        let vec_ref = unsafe { &*(&raw_states_vec as *const Vec<u8>
            as *const _ as *const Vec<StateVertex>) };
        // [NOTE]: `persistent` gives performance improvement:
        // let raw_states_buf = VertexBuffer::dynamic(display, vec_ref).unwrap();
        let raw_states_buf = VertexBuffer::persistent(display, vec_ref).unwrap();

        HexGridBuffer {
            raw_states_vec: Arc::new(Mutex::new(raw_states_vec)),
            raw_states_rx: None,
            raw_states_buf: raw_states_buf,
            full_slc_range: full_slc_range.clone(),
            default_slc_range: full_slc_range.clone(),
            cur_slc_range: full_slc_range.clone(),
            tract_map: area_info.tract_map,
        }
    }

    fn write_to_buf(&self, raw_states: &[u8]) {
        let vec_ref = unsafe { &*(raw_states as *const [u8]
            as *const _ as *const [StateVertex]) };
        self.raw_states_buf.write(vec_ref);
    }

    /// Refreshes the per-instance data within our vertex buffer.
    ///
    /// Only refreshes if fresh data is available.
    pub fn refresh_vertex_buf(&mut self) {
        // The future returned by `.recv(false)` will always immediately
        // resolve without blocking.
        if let Some(read_buf) = self.raw_states_rx.as_ref().unwrap()
                .recv(false).wait().unwrap()
        {
            let read_guard = read_buf.read_u8().wait().unwrap();
            self.write_to_buf(read_guard.as_slice());
        }
    }

    pub fn set_default_slc_range(&mut self, slc_range: Range<usize>) {
        self.default_slc_range = slc_range;
    }

    pub fn set_tract_map(&mut self, tract_map: SliceTractMap) {
        self.tract_map = tract_map;
    }

    // pub fn set_current_slc_range(&mut self ) {}

    #[allow(dead_code)]
    pub fn use_default_slc_range(&mut self) {
        self.cur_slc_range = self.default_slc_range.clone();
    }

    #[allow(dead_code)]
    pub fn use_full_slc_range(&mut self) {
        self.cur_slc_range = self.full_slc_range.clone();
    }

    pub fn aff_out_grid_dims(&self) -> (u32, u32) {
        self.tract_map.slc_dims(self.default_slc_range.start as u8)
    }


     // [FIXME]: DEPRICATE OR MOVE TO TESTS MODULE
    #[allow(dead_code)]
    pub fn fill_rand(&mut self) {
        let mut rng = rand::thread_rng();
        let range = RandRange::new(0u8, 255);

        // let raw_states_ptr = self.raw_states.clone();
        // let mut raw_states = raw_states_ptr.lock().unwrap();
        let mut raw_states_vec = self.raw_states_vec.lock().unwrap();

        for rs in raw_states_vec.iter_mut() {
            *rs = range.ind_sample(&mut rng);
        }
    }

    // Determines receiving end size (length);
    pub fn raw_states_vec(&mut self) -> Arc<Mutex<Vec<u8>>> {
        self.raw_states_vec.clone()
    }

    /// Returns a slice of the vertex buffer corresponding to a ganglion slice id.
    pub fn raw_states_buf(&self, slc_id: u8) -> VertexBufferSlice<StateVertex> {
        let axn_id_range: Range<usize> = self.tract_map.axn_id_range(
            (slc_id as usize)..(slc_id as usize + 1));
        self.raw_states_buf.slice(axn_id_range)
            .expect("HexGridBuffer::raw_states_buf(): Slice id out of range")
    }

    pub fn cur_slc_range(&self) -> Range<usize> {
        self.cur_slc_range.clone()
    }

    pub fn tract_map(&self) -> &SliceTractMap {
        &self.tract_map
    }

    pub fn set_tract_buffer(&mut self, rx: TractReceiver) {
        self.raw_states_rx = Some(rx);
    }
}
