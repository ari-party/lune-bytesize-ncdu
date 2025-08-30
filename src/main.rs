use std::{ env, fs::File, io::BufReader, path::Path, process::exit };

use rbx_binary::{ Deserializer, Serializer };
use rbx_dom_weak::WeakDom;
use rbx_types::Ref;
use serde_json::{ json, Value };
use rayon::prelude::*;
use indicatif::{ ProgressBar, ProgressStyle };
use ahash::HashMap;

#[derive(Debug)]
struct NcduEntry {
    name: String,
    dsize: u64,
    asize: u64,
    children: Option<Vec<NcduEntry>>,
}

impl NcduEntry {
    fn new(name: String, dsize: u64, asize: u64) -> Self {
        Self {
            name,
            dsize,
            asize,
            children: None,
        }
    }

    fn with_children(name: String, dsize: u64, asize: u64, children: Vec<NcduEntry>) -> Self {
        Self {
            name,
            dsize,
            asize,
            children: Some(children),
        }
    }

    fn to_ncdu_value(&self) -> Value {
        let info_block =
            json!({
            "name": self.name,
            "dsize": self.dsize,
            "asize": self.asize,
        });

        if let Some(ref children) = self.children {
            let mut result = vec![info_block];

            for child in children {
                result.push(child.to_ncdu_value());
            }

            Value::Array(result)
        } else {
            info_block
        }
    }
}

fn calculate_serialized_size(dom: &WeakDom, instance_ref: Ref) -> u64 {
    let _instance = dom.get_by_ref(instance_ref).unwrap();

    let mut buffer = Vec::new();
    let serializer = Serializer::new();

    if let Ok(()) = serializer.serialize(&mut buffer, dom, &[instance_ref]) {
        buffer.len() as u64
    } else {
        0
    }
}

fn index_instance(
    dom: &WeakDom,
    instance_ref: Ref,
    instance_byte_sizes: &HashMap<i32, usize>,
    progress_bar: &ProgressBar
) -> NcduEntry {
    let instance = dom.get_by_ref(instance_ref).unwrap();
    let name = &instance.name;
    let class_name = &instance.class;

    let dsize = instance.byte_size(instance_byte_sizes) as u64;
    let asize = calculate_serialized_size(dom, instance_ref);

    let display_name = if name == class_name.as_str() {
        name.to_string()
    } else {
        format!("{} ({})", name, class_name)
    };

    let children = instance.children();
    if children.is_empty() {
        progress_bar.inc(1);
        NcduEntry::new(display_name, dsize, asize)
    } else {
        let child_entries: Vec<NcduEntry> = children
            .par_iter()
            .map(|&child_ref| index_instance(dom, child_ref, instance_byte_sizes, progress_bar))
            .collect();

        progress_bar.inc(1);
        NcduEntry::with_children(display_name, dsize, asize, child_entries)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <input_file> <output_file>", args[0]);
        eprintln!("  input_file:  .rbxl, .rbxlx, .rbxm, or .rbxmx file");
        eprintln!("  output_file: .json file for ncdu");
        exit(1);
    }

    let input_path = &args[1];
    let output_path = &args[2];

    if !Path::new(input_path).is_file() {
        eprintln!("Error: Input file '{}' does not exist", input_path);
        exit(1);
    }

    if !output_path.ends_with(".json") {
        eprintln!("Error: Output path must end with .json");
        exit(1);
    }

    let path_parts: Vec<&str> = input_path.split('.').collect();
    let extension = path_parts.last().unwrap_or(&"");
    let is_place = extension == &"rbxl" || extension == &"rbxlx";
    let is_model = extension == &"rbxm" || extension == &"rbxmx";

    if !is_place && !is_model {
        eprintln!("Error: File must be a place (.rbxl/.rbxlx) or model (.rbxm/.rbxmx)");
        exit(1);
    }

    println!("Deserializing input");
    let file = match File::open(input_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            exit(1);
        }
    };

    let input = BufReader::new(file);
    let deserializer = Deserializer::new();
    let dom = match deserializer.deserialize(input) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Error deserializing file: {}", e);
            exit(1);
        }
    };

    let total_instances = dom.descendants().count();
    println!("Indexing {} instances ", total_instances);

    let progress_bar = ProgressBar::new(total_instances as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len}")
            .unwrap()
            .progress_chars("#>-")
    );

    let mut output =
        json!([
        1,
        0,
        {
            "progname": "rbx-bytesize-ncdu",
            "progver": "1.0",
            "timestamp": chrono::Utc::now().timestamp(),
        }
    ]);

    let root_ref = dom.root_ref();
    let root_instance = dom.get_by_ref(root_ref).unwrap();
    let instance_byte_sizes = dom.instance_byte_sizes.as_ref().unwrap();

    if is_place {
        let target_services = [
            "Workspace",
            "Lighting",
            "MaterialService",
            "ReplicatedFirst",
            "ReplicatedStorage",
            "ServerScriptService",
            "ServerStorage",
            "StarterGui",
            "StarterPlayer",
            "Teams",
            "SoundService",
            "TextChatService",
        ];

        let service_entries: Vec<NcduEntry> = target_services
            .par_iter()
            .filter_map(|service_name| {
                root_instance
                    .children()
                    .iter()
                    .find(|&&child_ref| {
                        if let Some(child) = dom.get_by_ref(child_ref) {
                            child.name == *service_name
                        } else {
                            false
                        }
                    })
                    .map(|&service_ref| {
                        index_instance(&dom, service_ref, instance_byte_sizes, &progress_bar)
                    })
            })
            .collect();

        let root_entry = NcduEntry::with_children("Game".to_string(), 0, 0, service_entries);

        if let Value::Array(ref mut arr) = output {
            arr.push(root_entry.to_ncdu_value());
        }
    } else {
        let children: Vec<NcduEntry> = root_instance
            .children()
            .par_iter()
            .map(|&child_ref| index_instance(&dom, child_ref, instance_byte_sizes, &progress_bar))
            .collect();

        let root_entry = NcduEntry::with_children("Model".to_string(), 0, 0, children);

        if let Value::Array(ref mut arr) = output {
            arr.push(root_entry.to_ncdu_value());
        }
    }

    progress_bar.finish();

    println!("Writing output");
    let output_file = match File::create(output_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error creating output file: {}", e);
            exit(1);
        }
    };

    if let Err(e) = serde_json::to_writer(output_file, &output) {
        eprintln!("Error writing output file: {}", e);
        exit(1);
    }
}
