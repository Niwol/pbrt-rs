use std::vec::IntoIter;

use crate::options::{PBRTOptions, RenderingCoordinateSystem};

pub fn parse_arg<F>(
    arg: String,
    arg_iter: &mut IntoIter<String>,
    options: &mut PBRTOptions,
    format: &mut bool,
    to_ply: &mut bool,
    on_error: F,
) -> bool
where
    F: Fn(String),
{
    match arg.as_str() {
        "--cropwindow" => {
            todo!("Bounds2f not defined yet");
        }

        "--debugstart" => options.debug_start = arg_iter.next().unwrap(),

        "--disable-image-texture" => options.basic_options.disable_image_textures = true,
        "--disable-pixel-jitter" => options.basic_options.disable_pixel_jitter = true,
        "--disable-texture-flitering" => options.basic_options.disable_texture_filtering = true,
        "--disable-wavelength-jitter" => options.basic_options.disable_wavelenght_jitter = true,

        "--displacement-edge-scale" => {
            let Some(value) = arg_iter.next() else {
                on_error(format!("Missing index for displacement edge scale"));
                unreachable!();
            };
            let Ok(value) = value.parse() else {
                on_error(format!("Value could not be parsed into a Float: {value}"));
                unreachable!();
            };

            options.displacement_edge_scale = value;
        }

        "--force-diffuse" => options.basic_options.force_difuse = true,
        "--fullscreen" => options.basic_options.fullscreen = true,

        "--gpu" => options.basic_options.use_gpu = true,
        "--gpu-device" => {
            let Some(value) = arg_iter.next() else {
                on_error(format!("Missing index for gpu device"));
                unreachable!();
            };
            let Ok(index) = value.parse() else {
                on_error(format!("Value could not be parsed into i32: {value}"));
                unreachable!();
            };

            options.gpu_device = Some(index);
        }

        "--help" => on_error(String::new()),
        "--interactive" => options.basic_options.interactive = true,

        "--mse-reference-image" => {
            let Some(filename) = arg_iter.next() else {
                on_error(format!("Must provide filename for mse reference image"));
                unreachable!();
            };

            options.mse_reference_image = filename;
        }

        "--mse-reference-out" => {
            let Some(filename) = arg_iter.next() else {
                on_error(format!("Must provide filename for mse reference output"));
                unreachable!();
            };

            options.mse_reference_output = filename;
        }

        "--nthreads" => {
            let Some(value) = arg_iter.next() else {
                on_error(format!("No value provided for nthreads"));
                unreachable!();
            };
            let Ok(n_threads) = value.parse() else {
                on_error(format!("Could not parse arg to i32: {value}"));
                unreachable!();
            };

            options.n_threads = n_threads;
        }

        "--outfile" => {
            let Some(filename) = arg_iter.next() else {
                on_error(format!("Must provide filename for outfile"));
                unreachable!();
            };

            options.image_file = filename;
        }

        "--pixel" => {
            todo!("Bounds2i not implemented yet");
        }

        "--pixel-bounds" => {
            todo!("Bounds2i not implemented yet");
        }

        "--pixelmaterial" => {
            todo!("Point2i not implemented yet");
        }

        "--pixelstats" => options.record_pixel_statistics = true,
        "--quick" => options.quick_render = true,
        "--quiet" => options.basic_options.quiet = true,
        "--render-coord-sys" => {
            let Some(value) = arg_iter.next() else {
                on_error(format!("Must provide render coord system"));
                unreachable!();
            };

            match value.to_ascii_lowercase().as_str() {
                "camera" => {
                    options.basic_options.rendering_space = RenderingCoordinateSystem::Camera
                }
                "cameraworld" => {
                    options.basic_options.rendering_space = RenderingCoordinateSystem::CameraWorld
                }
                "world" => options.basic_options.rendering_space = RenderingCoordinateSystem::World,

                _ => on_error(format!("Invalid rendering space given: {value}")),
            };
        }

        "--seed" => {
            let Some(value) = arg_iter.next() else {
                on_error(format!("Must provode value for seed"));
                unreachable!();
            };
            let Ok(seed) = value.parse() else {
                on_error(format!("Could not parse arg to i32"));
                unreachable!();
            };

            options.basic_options.seed = seed;
        }

        "--stats" => options.print_statistics = true,

        "--spp" => {
            let Some(value) = arg_iter.next() else {
                on_error(format!("Must provode value for spp"));
                unreachable!();
            };
            let Ok(spp) = value.parse() else {
                on_error(format!("Could not parse arg to i32"));
                unreachable!();
            };

            options.pixel_sample = Some(spp);
        }

        "--wavefront" => options.basic_options.wavefront = true,
        "--write-partial-image" => options.write_partial_image = true,

        "--log-file" => {
            let Some(filename) = arg_iter.next() else {
                on_error(format!("Must provide filename for log file"));
                unreachable!();
            };

            options.log_file = filename;
        }

        "--log-level" => todo!("LogLevel not implemented yet"),
        "--log-utilisation" => options.log_utilisaiton = true,

        "--format" => *format = true,
        "--toply" => *to_ply = true,
        "--upgrade" => options.upgrade = true,

        _ => return false,
    }

    true
}
