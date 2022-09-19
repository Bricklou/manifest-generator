mod builder;
use std::{
    fs::File,
    path::{Path, PathBuf},
};

pub use builder::*;
use globset::{Glob, GlobSet};
use question::Question;

use walkdir::WalkDir;

use crate::{
    manifest::{Manifest, ManifestFile, ManifestPatches, ManifestPlatform, Metadata},
    utils,
};

pub struct Generator {
    whitelist: GlobSet,
    blacklist: GlobSet,
    version: String,
    depends_on: Option<String>,
}

impl Default for Generator {
    fn default() -> Self {
        Self {
            whitelist: GlobSet::empty(),
            blacklist: GlobSet::empty(),
            version: String::default(),
            depends_on: None,
        }
    }
}

impl Generator {
    pub fn generate(&self, input: PathBuf, output: PathBuf) {
        let mut manifest = if output.exists() {
            // old manifest
            utils::read_json_from_file::<Manifest>(&output).unwrap()
        } else {
            Manifest {
                version: self.version.clone(),
                patches: Vec::new(),
            }
        };

        if Generator::check_version_exists(&manifest, &self.version) {
            let anwser =
                Question::new("A patch with this version already exists! Should it be overriden ?")
                    .default(question::Answer::NO)
                    .show_defaults()
                    .confirm();

            if anwser == question::Answer::NO {
                return;
            }

            // Search for all patches with the same version key as provided and remove them from the patch list
            // (= keep only those who doesn't match with the version)
            manifest.patches.retain(|p| p.version != self.version);
        }

        if let Some(ref depends_on_version) = self.depends_on {
            if !Generator::check_version_exists(&manifest, depends_on_version) {
                panic!("The patch, on which the patch depends, doesn't exists !");
            }
        }

        manifest.patches.push(self.generate_patch(input).unwrap());
        utils::save_json_to_file(&output, manifest).unwrap();

        println!("Manifest saved to file: {:?}", output);
    }

    fn generate_patch(&self, path: impl AsRef<Path>) -> std::io::Result<ManifestPatches> {
        Ok(ManifestPatches {
            version: self.version.clone(),
            depends_on: self.depends_on.clone(),
            files: self.generate_files_list(path)?,
        })
    }

    fn generate_files_list(
        &self,
        input_path: impl AsRef<Path>,
    ) -> std::io::Result<Vec<ManifestFile>> {
        let mut hasher = blake3::Hasher::new();

        let mut manifest_list = Vec::<ManifestFile>::new();

        let files = WalkDir::new(&input_path)
            .sort_by_file_name()
            .into_iter()
            .filter_map(|e| {
                if let Ok(d) = e {
                    if d.path().is_file() {
                        let p = d
                            .path()
                            .strip_prefix(&input_path)
                            .unwrap()
                            .to_string_lossy()
                            .to_string();

                        if (!self.blacklist.is_empty() && self.blacklist.is_match(&p))
                            || (!self.whitelist.is_empty() && !self.whitelist.is_match(&p))
                        {
                            return None;
                        }

                        return Some(d);
                    }
                }
                None
            });

        for entry in files {
            let p = entry.path().strip_prefix(&input_path).unwrap();
            let p_name = p.to_string_lossy().to_string();

            let mut file = File::open(entry.path())?;
            std::io::copy(&mut file, &mut hasher)?;

            let mut m = ManifestFile {
                path: p_name.clone(),
                hash: hasher.finalize().to_string(),
                size: entry.metadata()?.len(),
                meta: None,
            };

            if Glob::new("*windows-*")
                .unwrap()
                .compile_matcher()
                .is_match(&p_name)
            {
                m.meta = Some(Metadata {
                    platform: Some(ManifestPlatform::Windows),
                });
            } else if Glob::new("*linux-*")
                .unwrap()
                .compile_matcher()
                .is_match(&p_name)
            {
                m.meta = Some(Metadata {
                    platform: Some(ManifestPlatform::Linux),
                });
            }

            manifest_list.push(m);
        }

        Ok(manifest_list)
    }

    fn check_version_exists(manifest: &Manifest, version: &String) -> bool {
        return manifest.patches.iter().any(|p| p.version.eq(version));
    }
}
