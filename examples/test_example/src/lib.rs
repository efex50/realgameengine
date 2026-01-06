use realgameengine::prelude::*;

#[main]
fn main(){

    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let mut logger = NewDefaultLogger();

    logger.info("zort");

    wasm_thread::spawn(move|| {
        logger.info("zozozrt");
    });

}



