// use psutil::*;
use sysinfo::*;
use std::error::Error;

use heim::{memory, units::information};
use yaml_rust::{YamlLoader, YamlEmitter};

// fn psutil_meminfo() -> (u64, u64) {
//     let virtual_memory = memory::virtual_memory().unwrap();
// 	let swap_memory = memory::swap_memory().unwrap();
//     (virtual_memory.total(), swap_memory.total())
// }

fn sysinfo_meminfo(sys: &mut System) -> (u64, u64) {
    (sys.total_memory(), sys.total_swap())
}

fn sysinfo_cpuinfo(sys: &mut System) -> u64 {
    sys.processors().len() as u64
}

fn sysinfo_hostname(sys: &mut System) -> String {
    sys.host_name().unwrap()
}

fn heim_meminfo() -> (u64, u64) {
    let mut memory_total: u64;
    let mut swap_total: u64;

    smol::block_on(async {
        let memory = memory::memory().await?;
        let swap = memory::swap().await?;

        println!("              total        free   available");
        println!(
            "{:>7} {:>11?} {:>11?} {:>11?}",
            "Mem:",
            memory.total().get::<information::megabyte>(),
            memory.free().get::<information::megabyte>(),
            memory.available().get::<information::megabyte>(),
        );
        println!(
            "{:>7} {:>11?} {:>11?} {:>11?}",
            "Swap:",
            swap.total().get::<information::megabyte>(),
            swap.used().get::<information::megabyte>(),
            swap.free().get::<information::megabyte>(),
        );

        Ok(())
    });

    (memory_total, swap_total)
}

fn main() {
    let s =
"
foo:
    - list1
    - list2
bar:
    - 1
    - 2.0
";
    let docs = YamlLoader::load_from_str(s).unwrap();

    // Multi document support, doc is a yaml::Yaml
    let doc = &docs[0];

    // Debug support
    println!("{:?}", doc);

    // Index access for map & array
    assert_eq!(doc["foo"][0].as_str().unwrap(), "list1");
    assert_eq!(doc["bar"][1].as_f64().unwrap(), 2.0);

    // Chained key/array access is checked and won't panic,
    // return BadValue if they are not exist.
    assert!(doc["INVALID_KEY"][100].is_badvalue());

    // Dump the YAML object
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(doc).unwrap(); // dump the YAML object to a String
    }
    println!("{}", out_str);

    

    // let (a, b) = psutil_meminfo();
    // println!("{}kb, {}kb", a, b);

    let mut sys = System::new_all();
    sys.refresh_all();
    let meminfo = sysinfo_meminfo(&mut sys);
    let cpuinfo = sysinfo_cpuinfo(&mut sys);
    let host_name = sysinfo_hostname(&mut sys);
    
    println!("{}KB, {}KB", meminfo.0, meminfo.1);
    println!("{}", cpuinfo);
    println!("{}", host_name);

    let (e, f) = heim_meminfo();
    println!("{}, {}", e, f)
}
