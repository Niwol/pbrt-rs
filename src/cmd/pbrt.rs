use std::{env, process::exit};

use pbrt_rs::{options::PBRTOptions, pbrt::PBRT, utils::args::parse_arg};

fn usage(message: String) -> ! {
    if !message.is_empty() {
        println!("pbrt: {message}\n");
    }

    println!(
        "
usage: pbrt [<options>] <filename.pbrt...>

Rendering options:
    --cropwindow <x0, x1, x2, x3>   specify an image crop window w.r.t. [0,1]^2.
    --debugstart <values>           Inform the Integrator where to start rendering for
                                    faster debugging. (<values> are Integrator-specific
                                    and come from error message text.)
"
    );

    let exit_code = if message.is_empty() { 0 } else { 1 };
    exit(exit_code);
}

fn get_command_line_arguments() -> Vec<String> {
    env::args().into_iter().collect()
}

fn main() {
    // Converting command-line arguments to vector of strings
    let args: Vec<String> = get_command_line_arguments();

    // Declar variables for parsed command line
    let mut options = PBRTOptions::default();
    let mut filenames = Vec::new();
    let mut format = false;
    let mut to_ply = false;

    let mut arg_iter = args.into_iter();

    // Process command-line arguments
    while let Some(arg) = arg_iter.next() {
        if !arg.starts_with("-") {
            filenames.push(arg);
            continue;
        }

        let on_error = |error: String| {
            usage(error);
        };

        if !parse_arg(
            arg.clone(),
            &mut arg_iter,
            &mut options,
            &mut format,
            &mut to_ply,
            on_error,
        ) {
            if &arg == "--help" || &arg == "-help" || &arg == "-h" {
                usage(String::new());
            } else {
                usage(format!("Unknown argument: {arg}"));
            }
        }
    }

    if !options.mse_reference_image.is_empty() && options.mse_reference_output.is_empty() {
        println!("Must provide MSE reference output filename via --mse-reference-out");
        exit(1);
    }

    if !options.mse_reference_output.is_empty() && options.mse_reference_image.is_empty() {
        println!("Must provide MSE reference image via --mse-reference-image");
        exit(1);
    }

    // Check pixel material and gpu
    println!("Todo: Check for pixel material and gpu");

    if options.basic_options.use_gpu && options.basic_options.wavefront {
        println!("Both --gpu and --wavefront were specified; --gpu takes precedence.")
    }

    // Check pixel material and wavefront
    println!("Todo: Check pixel material and wavefront");

    if options.basic_options.interactive
        && !(options.basic_options.use_gpu || options.basic_options.wavefront)
    {
        println!(
            "The --interactive option is only supported with --gpu and --wavefront integrators."
        );
        exit(1);
    }

    if options.basic_options.fullscreen && !options.basic_options.interactive {
        println!("The --fullscreen option is only supported in interactive mode");
        exit(1);
    }

    if options.quick_render && options.basic_options.interactive {
        println!("the --quick option is not supported in interactive mode");
        exit(1);
    }

    // Initialize pbrt
    let pbrt = PBRT::new(&mut options);

    if format || to_ply || options.upgrade {
        todo!("File parsing");
    } else {
        // Parse provided scene description file
        //BasicScene
        //BasicSceneBuilder
        //parse_files

        // Render the scene
        if options.basic_options.use_gpu || options.basic_options.wavefront {
            //render_wavefront
        } else {
            pbrt.render_cpu();
        }

        // Cleanup after rendering the scene
    }
}
