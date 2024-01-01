use anyhow::bail;
use cargo_metadata as cm;
use std::env;

fn main() {
    let cwd = env::current_dir().unwrap();
    println!("cwd: {}", cwd.as_os_str().to_string_lossy().as_ref());

    let manifest_path = cwd.join("Cargo.toml");
    println!(
        "manifest_path: {}",
        manifest_path.as_os_str().to_string_lossy().as_ref()
    );

    let metadata = cm::MetadataCommand::new()
        .manifest_path(manifest_path)
        .current_dir(cwd)
        .exec()
        .unwrap();

    println!("metadata: {metadata:#?}");

    let (root, root_package) = metadata.bin_target_by_name("g").unwrap();

    println!("root: {:?}", root);
    println!("root_package: {:?}", root_package);
}
pub(crate) trait MetadataExt {
    fn bin_target_by_name<'a>(
        &'a self,
        name: &str,
    ) -> anyhow::Result<(&'a cm::Target, &'a cm::Package)>;
}

impl MetadataExt for cm::Metadata {
    fn bin_target_by_name<'a>(
        &'a self,
        name: &str,
    ) -> anyhow::Result<(&'a cm::Target, &'a cm::Package)> {
        target_by_kind_and_name(self, "bin", name)
    }
}

fn target_by_kind_and_name<'a>(
    metadata: &'a cm::Metadata,
    kind: &str,
    name: &str,
) -> anyhow::Result<(&'a cm::Target, &'a cm::Package)> {
    match *targets_in_ws(metadata)
        .filter(|(t, _)| t.name == name && t.kind == [kind.to_owned()])
        .collect::<Vec<_>>()
    {
        [] => bail!("no {} target named `{}`", kind, name),
        [target] => Ok(target),
        [..] => bail!(
            "multiple {} targets named `{}` in this workspace",
            kind,
            name,
        ),
    }
}

fn targets_in_ws(metadata: &cm::Metadata) -> impl Iterator<Item = (&cm::Target, &cm::Package)> {
    metadata
        .packages
        .iter()
        .filter(move |cm::Package { id, .. }| metadata.workspace_members.contains(id))
        .flat_map(|p| p.targets.iter().map(move |t| (t, p)))
}
