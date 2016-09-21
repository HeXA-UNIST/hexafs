use std::env;

mod server;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() >= 2 {
        match &args[1] as &str {
            "master" => {
                server::master::run();
                return;
            }
            "chunkserver" => {
                server::chunkserver::run();
                return;
            }
            _ => {}
        }
    }

    print_usage();
    return;

}

fn print_usage() {
    let usage = "Usage:
    hxfs master [arguments]
    hxfs chunkserver [arguments]
";
    println!("{}", usage);
}
