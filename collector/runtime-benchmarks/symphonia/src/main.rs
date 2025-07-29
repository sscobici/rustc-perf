//! Benchmarks the performance of the `std::fmt` module.
//! The benchmarks here are not very comprehensive and should be eventually
//! replaced with more "real-world" code.

use benchlib::benchmark::run_benchmark_group;
use symphonia::core::formats::probe::Hint;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;

fn main() {
    run_benchmark_group(|group| {
        group.register_benchmark("symphonia-probe", || {
            let path = "/mnt/e/Media/Personal/Jocul cu papusi.mp4";

            // Open the media source.
            let src = std::fs::File::open(path).expect("failed to open media");
            || {
                    // Create the media source stream.
                    let mss = MediaSourceStream::new(Box::new(src), Default::default());

                    // Create a probe hint using the file's extension. [Optional]
                    let hint = Hint::new();
                    // Use the default options for metadata and format readers.
                    let meta_opts: MetadataOptions = Default::default();
                    let fmt_opts: FormatOptions = Default::default();

                    // Probe the media source.
                    let _format = symphonia::default::get_probe()
                        .probe(&hint, mss, fmt_opts, meta_opts)
                        .expect("unsupported format");
            }
        });
    });
}
