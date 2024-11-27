use crate::pbrt::Float;

pub enum RenderingCoordinateSystem {
    Camera,
    CameraWorld,
    World,
}

pub struct BasicPBRTOptions {
    pub seed: i32,
    pub quiet: bool,
    pub disable_pixel_jitter: bool,
    pub disable_wavelenght_jitter: bool,
    pub disable_texture_filtering: bool,
    pub disable_image_textures: bool,
    pub force_difuse: bool,
    pub use_gpu: bool,
    pub wavefront: bool,
    pub interactive: bool,
    pub fullscreen: bool,
    pub rendering_space: RenderingCoordinateSystem,
}

impl Default for BasicPBRTOptions {
    fn default() -> Self {
        Self {
            seed: 0,
            quiet: false,
            disable_pixel_jitter: false,
            disable_wavelenght_jitter: false,
            disable_texture_filtering: false,
            disable_image_textures: false,
            force_difuse: false,
            use_gpu: false,
            wavefront: false,
            interactive: false,
            fullscreen: false,
            rendering_space: RenderingCoordinateSystem::CameraWorld,
        }
    }
}

pub struct PBRTOptions {
    pub basic_options: BasicPBRTOptions,
    pub n_threads: i32,
    //pub log_level: LogLevel
    pub log_file: String,
    pub log_utilisaiton: bool,
    pub write_partial_image: bool,
    pub record_pixel_statistics: bool,
    pub print_statistics: bool,
    pub pixel_sample: Option<i32>,
    pub gpu_device: Option<i32>,
    pub quick_render: bool,
    pub upgrade: bool,
    pub image_file: String,
    pub mse_reference_image: String,
    pub mse_reference_output: String,
    pub debug_start: String,
    pub display_server: String,
    //pub crop_window: Option<Bounds2f>,
    //pub pixel_bounds: Option<Bounds2i>,
    //pub pixel_material: Option<Point2i>,
    pub displacement_edge_scale: Float,
}

impl Default for PBRTOptions {
    fn default() -> Self {
        Self {
            basic_options: BasicPBRTOptions::default(),
            n_threads: 0,
            log_file: String::default(),
            log_utilisaiton: false,
            write_partial_image: false,
            record_pixel_statistics: false,
            print_statistics: false,
            pixel_sample: None,
            gpu_device: None,
            quick_render: false,
            upgrade: false,
            image_file: String::default(),
            mse_reference_image: String::default(),
            mse_reference_output: String::default(),
            debug_start: String::default(),
            display_server: String::default(),
            displacement_edge_scale: 1.0,
        }
    }
}
