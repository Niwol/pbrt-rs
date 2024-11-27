use std::{env, process::exit};

use pbrt_rs::{options::PBRTOptions, utils::args};

fn usage(message: String) {
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
}

fn get_command_line_arguments() -> Vec<String> {
    env::args().into_iter().collect()
}

fn main() {
    // Converting command-line arguments to vector of strings
    let args: Vec<String> = get_command_line_arguments();

    // Declar variables for parsed command line
    let options = PBRTOptions::default();
    let mut filenames = Vec::new();

    let mut arg_iter = args.into_iter();

    // Process command-line arguments
    while let Some(arg) = arg_iter.next() {
        if !arg.starts_with("-") {
            filenames.push(arg);
            continue;
        }

        let on_error = |error: String| {
            usage(error);
            exit(1);
        };
    }

    // Initialize pbrt

    // Parse provided scene description file

    // Render the scene

    // Cleanup after rendering the scene
}
