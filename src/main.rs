use rspack::builder::{BuilderContext, CompilerOptionsBuilder};
use rspack_core::{Compiler, CompilerOptions, OutputOptions, Plugin};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut builder_context = BuilderContext { plugins: vec![] };

  let mut builder = CompilerOptionsBuilder::default();

  let options = builder.context(".").build(&mut builder_context);

  // 创建编译器实例
  let mut compiler = Compiler::new(
    "current_dir".to_string(),
    options,
    vec![], // hooks
    vec![], // plugins
    None,   // cache
    None,   // context
    None,   // entry
    None,   // logger
    None,   // compiler_options
  );

  // 运行编译
  compiler.run().await?;

  Ok(())
}
