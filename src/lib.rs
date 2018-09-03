extern crate rodio;

use rodio::Sink;
use rodio::Source;
use std::time::Duration;

#[no_mangle]
pub extern "C" fn hoge() -> f64{
    println!("hoge");
    let device = rodio::default_output_device().unwrap();

    let sink = Sink::new(&device);
    // Add a dummy source of the sake of the example.
    let source = rodio::source::SineWave::new(440).take_duration(Duration::from_secs(3));
    sink.append(source);
    sink.sleep_until_end();

    1.0
}

#[no_mangle]
pub extern "C" fn fuga(v: f64) -> f64 {
    v + 1.0
}

#[cfg(test)]
mod tests {
    use super::hoge;
    #[test]
    fn it_works() {
        hoge();
    }
}
