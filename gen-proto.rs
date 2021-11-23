use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    let mut replace_pairs = HashMap::new();
    replace_pairs.insert(
        "github.com/containerd/containerd/api/types/descriptor.proto",
        "api/types/descriptor.proto",
    );
    replace_pairs.insert(
        "github.com/containerd/containerd/api/types/mount.proto",
        "api/types/mount.proto",
    );
    replace_pairs.insert(
        "github.com/containerd/containerd/api/types/platform.proto",
        "api/types/platform.proto",
    );
    replace_pairs.insert(
        "github.com/containerd/containerd/api/types/metrics.proto",
        "api/types/metrics.proto",
    );
    replace_pairs.insert(
        "github.com/containerd/containerd/api/types/descriptor.proto",
        "api/types/descriptor.proto",
    );
    replace_pairs.insert(
        "github.com/containerd/containerd/api/types/task/task.proto",
        "api/types/task/task.proto",
    );
    replace_pairs.insert(
        "github.com/containerd/containerd/protobuf/plugin/fieldpath.proto",
        "protobuf/plugin/fieldpath.proto",
    );

    walkdir_replace(&replace_pairs, false).unwrap();
    replace_text_in_file(
        "containerd/api/services/introspection/v1/introspection.proto",
        "repeated types.Platform platforms = 4 [(gogoproto.nullable) = false];",
        "repeated containerd.types.Platform platforms = 4 [(gogoproto.nullable) = false];",
    )
    .unwrap();
    replace_text_in_file(
        "containerd/api/services/tasks/v1/tasks.proto",
        "repeated types.Metric metrics = 1;",
        "repeated containerd.types.Metric metrics = 1;",
    )
    .unwrap();
    tonic_build::configure()
        .out_dir("./src/apis")
        .build_server(false)
        .build_client(true)
        .compile(
            &[
                "containerd/api/services/images/v1/images.proto",
                "containerd/api/services/containers/v1/containers.proto",
                "containerd/api/services/content/v1/content.proto",
                "containerd/api/services/diff/v1/diff.proto",
                "containerd/api/services/events/v1/events.proto",
                "containerd/api/services/images/v1/images.proto",
                "containerd/api/services/introspection/v1/introspection.proto",
                "containerd/api/services/leases/v1/leases.proto",
                "containerd/api/services/namespaces/v1/namespace.proto",
                "containerd/api/services/snapshots/v1/snapshots.proto",
                "containerd/api/services/tasks/v1/tasks.proto",
                "containerd/api/services/version/v1/version.proto",
                "containerd/api/types/descriptor.proto",
                "containerd/api/types/metrics.proto",
                "containerd/api/types/mount.proto",
                "containerd/api/types/platform.proto",
                "containerd/api/types/task/task.proto",
                "containerd/protobuf/plugin/fieldpath.proto",
                "protobuf/protobuf/google/protobuf/empty.proto",
                "protobuf/protobuf/google/protobuf/timestamp.proto",
                "protobuf/protobuf/google/protobuf/field_mask.proto",
                "protobuf/protobuf/google/protobuf/any.proto",
                "googleapis/google/rpc/status.proto",
                "protobuf/gogoproto/gogo.proto",
            ],
            &[
                "protobuf/protobuf",
                "protobuf",
                "containerd/api/services/images/v1",
                "googleapis",
                "containerd",
            ],
        )
        .unwrap();
    walkdir_replace(&replace_pairs, true).unwrap();
    replace_text_in_file(
        "containerd/api/services/introspection/v1/introspection.proto",
        "repeated containerd.types.Platform platforms = 4 [(gogoproto.nullable) = false];",
        "repeated types.Platform platforms = 4 [(gogoproto.nullable) = false];",
    )
    .unwrap();
    replace_text_in_file(
        "containerd/api/services/tasks/v1/tasks.proto",
        "repeated containerd.types.Metric metrics = 1;",
        "repeated types.Metric metrics = 1;",
    )
    .unwrap();
}

fn replace_text_in_file(file_name: &str, from: &str, to: &str) -> Result<(), std::io::Error> {
    let mut src = File::open(file_name)?;
    let mut contents = String::new();
    src.read_to_string(&mut contents).unwrap();
    drop(src);

    let new_contents = contents.replace(from, to);

    let mut dst = File::create(&file_name)?;
    dst.write(new_contents.as_bytes())?;

    Ok(())
}

fn walkdir_replace(map: &HashMap<&str, &str>, back: bool) -> Result<(), std::io::Error> {
    for entry in WalkDir::new("./containerd/api/services/")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        let f_name = String::from(entry.file_name().to_string_lossy());
        let ext = Path::new(&f_name).extension().and_then(OsStr::to_str);
        if ext == Some("proto") {
            for (key, value) in map {
                if back {
                    replace_text_in_file(entry.path().to_str().unwrap(), value, key).unwrap();
                } else {
                    replace_text_in_file(entry.path().to_str().unwrap(), key, value).unwrap();
                }
            }
        }
    }
    Ok(())
}
