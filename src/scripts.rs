use canvas::{canvas::CanvasOptions, prelude::*};

macro_rules! const_script {
    ($name:ident, $path:literal) => {
        const $name: Lazy<Script> = Lazy::new(|| compile($path, include_str!($path)));
    };
}

pub fn compile(name: &str, code: &str) -> Script {
    let mut sources = SourceCache::new();
    let mut resolver = DefaultIncludeResolver::default();

    let Ok(ast) = sources.parse_with_includes(name, code, &mut resolver) else {
        panic!("failed to parse {}", name);
    };

    let Ok(script) = Compiler::compile(&ast) else {
        panic!("failed to compile {}", name);
    };

    script
}

pub fn run(
    img: &mut RgbaImage,
    script: &Script,
    opts: CanvasOptions,
    string_vars: &[(&str, String)],
    images: &[(&str, RgbaImage)],
) -> Result<(), &'static str> {
    let mut canvas = Canvas::new(script.clone(), img, opts)?;
    for (name, value) in string_vars {
        canvas.add_variable(*name, value.clone());
    }

    for (name, image) in images {
        canvas.add_image(*name, image);
    }

    canvas.run().map_err(|_| "failed to run script")?;

    Ok(())
}

pub const MINIMAL_RUNTIME: CanvasOptions = CanvasOptions {
    stack_size: 8,
    call_stack_size: 0,
    string_max_size: 128,
    array_max_size: 0,
    image_storage_size: 0,
    max_runtime: std::time::Duration::from_millis(50),
};
