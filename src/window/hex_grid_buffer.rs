use std::iter;
use std::sync::{Arc, Mutex};
use std::ops::Range;
use rand;
use rand::distributions::{IndependentSample, Range as RandRange};
use glium::backend::glutin_backend::{GlutinFacade};
// use glium::buffer::{Buffer, BufferSlice, BufferMode, BufferType};
use glium::vertex::{VertexBuffer, VertexBufferSlice};
use bismit::flywheel::AreaInfo;
use bismit::map::SliceTractMap;

// [NOTE]: This is only one setting dictating how quickly vibi 'appears' to refresh.
// const SMOOTH_REFRESH: bool = false;
const SMOOTH_REFRESH: bool = true;


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
    // state_vertices: Vec<StateVertex>,
    // vertex_buf: VertexBuffer<StateVertex>,
    raw_states_buf: VertexBuffer<StateVertex>,
    full_slc_range: Range<u8>,
    default_slc_range: Range<u8>,
    cur_slc_range: Range<u8>,
    tract_map: SliceTractMap,
    is_clear: bool,
}

impl HexGridBuffer {
    pub fn new(area_info: AreaInfo, display: &GlutinFacade)
            -> HexGridBuffer
    {
        let full_slc_range = area_info.tract_map.slc_id_range();
        let grid_count = area_info.tract_map.axn_count(full_slc_range.clone());

        // println!("\n###### HexGridBuffer::new(): d_slc_range: {:?}, grid_count: {}, tract_map: {:?}",
        //     default_slc_range, grid_count, tract_map);

        let raw_states_vec: Vec<u8> = iter::repeat(0u8).cycle().take(grid_count).collect();
        // let state_vertices: Vec<StateVertex> = iter::repeat(StateVertex { state: 0.0 })
        //     .cycle().take(grid_count).collect();
        // let vertex_buf = VertexBuffer::dynamic(display, &state_vertices).unwrap();
        // let raw_states_buf: Buffer<[u8]> = Buffer::empty_unsized(display, BufferType::ArrayBuffer, grid_count,
        //     BufferMode::Persistent).unwrap();
        let vec_ref = unsafe { &*(&raw_states_vec as *const Vec<u8>
            as *const _ as *const Vec<StateVertex>) };

        // [NOTE]: `persistent` gives performance improvement:
        // let raw_states_buf = VertexBuffer::dynamic(display, vec_ref).unwrap();
        let raw_states_buf = VertexBuffer::persistent(display, vec_ref).unwrap();


        HexGridBuffer {
            raw_states_vec: Arc::new(Mutex::new(raw_states_vec)),
            // state_vertices: state_vertices,
            // vertex_buf: vertex_buf,
            raw_states_buf: raw_states_buf,
            full_slc_range: full_slc_range.clone(),
            // default_slc_range: full_slc_range.clone(),
            default_slc_range: area_info.aff_out_slc_range.clone(),
            // cur_slc_range: full_slc_range.clone(),
            cur_slc_range: full_slc_range.clone(),
            tract_map: area_info.tract_map,
            is_clear: false,
        }
    }

    fn write_to_buf(&self, raw_states: &Vec<u8>) {
        let vec_ref = unsafe { &*(raw_states as *const Vec<u8>
            as *const _ as *const Vec<StateVertex>) };
        self.raw_states_buf.write(&vec_ref);
    }

    /// Refreshes the per-instance data within our vertex buffer.
    ///
    /// *If* a lock on `raw_states_vec` can be obtained (if it's not currently
    /// being written to by another thread): converts the u8s in `raw_states_vec`
    /// to floats, store them in `state_vertices`, then writes the contents
    /// of `state_vertices` to `vertex_buf`.
    ///
    /// Set `SMOOTH_REFRESH` to true for smoother refreshes at the cost of
    /// slower cycling.
    ///
    /// This is an opportunistic refresh, it will sometimes do nothing at all.
    pub fn refresh_vertex_buf(&mut self) -> bool {
        if SMOOTH_REFRESH {
            match self.raw_states_vec.lock() {
                Ok(ref raw_states_vec) => {
                    self.write_to_buf(raw_states_vec);
                    true
                },
                Err(err) => panic!(err.to_string()),
            }
        } else {
            match self.raw_states_vec.try_lock() {
                Ok(ref raw_states_vec) => {
                    self.write_to_buf(raw_states_vec);
                    true
                },
                Err(_) => false,
            }
        }
    }

    pub fn set_default_slc_range(&mut self, slc_range: Range<u8>) {
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
        self.tract_map.slc_dims(self.default_slc_range.start)
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
        let axn_id_range: Range<usize> = self.tract_map.axn_id_range(slc_id..slc_id + 1);
        self.raw_states_buf.slice(axn_id_range)
            .expect("HexGridBuffer::raw_states_buf(): Slice id out of range")
    }

    pub fn cur_slc_range(&self) -> Range<u8> {
        self.cur_slc_range.clone()
    }

    // pub fn cur_axn_range(&self) -> Range<usize> {
    //     let cur_axn_range = self.tract_map.axn_id_range(self.cur_slc_range.clone());
    //     // println!("###### HexGridBuffer::cur_axn_range(): \
    //     //         self.cur_slc_range: {:?}, cur_axn_range: {:?}",
    //     //     self.cur_slc_range,
    //     //     cur_axn_range);

    //     cur_axn_range
    // }

    pub fn tract_map(&self) -> &SliceTractMap {
        &self.tract_map
    }

    pub fn is_clear(&self) -> bool {
        self.is_clear
    }

    pub fn set_clear(&mut self, is_clear: bool) {
        self.is_clear = is_clear;
    }
}
