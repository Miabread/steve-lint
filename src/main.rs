use {
    anyhow::Result,
    clap::Clap,
    glob::Pattern,
    std::{
        env, fs,
        path::{Path, PathBuf},
    },
};

#[derive(Clap, Debug, Clone)]
#[clap(version = clap::crate_version!(), author = clap::crate_authors!())]
struct Arguments {
    /// The directory/file to check
    #[clap(parse(from_os_str))]
    file: Option<PathBuf>,

    /// Automatically rename directories/files
    #[clap(short, long)]
    fix: bool,

    /// Log all directories/files traversed
    #[clap(short, long)]
    verbose: bool,

    /// Glob pattern of directories/files to ignore
    #[clap(short, long)]
    ignore: Vec<Pattern>,
}

impl Arguments {
    // Log a file if --verbose is set
    fn log_verbose(&self, tag: &str, path: impl AsRef<Path>) {
        if self.verbose {
            println!("[{}] {:?}", tag, relate_path(path));
        }
    }

    // Check to see if any --ignore arguments apply to a path
    fn is_ignored(&self, path: impl AsRef<Path>) -> bool {
        let path = path.as_ref();
        self.ignore.iter().any(|p| p.matches_path(path))
    }
}

fn main() -> Result<()> {
    let args: Arguments = Arguments::parse();

    // Get the starting path, or use the current directory
    let path = if let Some(file) = &args.file {
        file.into()
    } else {
        env::current_dir()?
    };

    // Start traversing from path
    traverse(path, &args)?;

    Ok(())
}

// Traverse a file structure, checking each directory or file
fn traverse(path: impl AsRef<Path>, args: &Arguments) -> Result<()> {
    let path = path.as_ref();

    // Check to see if dir/file is ignored
    if args.is_ignored(path) {
        args.log_verbose("IGNORE", path);
        return Ok(());
    }

    // When --verbose is set log type and path
    args.log_verbose(
        if path.is_dir() {
            "DIRECTORY"
        } else if path.is_file() {
            "FILE"
        } else {
            "OTHER"
        },
        path,
    );

    // Check the path
    check_path(path, args)?;

    // If it's a directory, recurse into it and check it's children
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            traverse(entry?.path(), args)?;
        }
    }

    Ok(())
}

// Sanitize
fn check_path(path: impl AsRef<Path>, args: &Arguments) -> Result<()> {
    // Transform paths
    let path = relate_path(path);
    let linted = if let Some(linted) = lint_path(&path) {
        linted
    } else {
        return Ok(());
    };

    // Print changes
    println!("{:?} -> {:?}", path, linted);

    // When --fix is set rename file
    if args.fix {
        fs::rename(&path, path.with_file_name(linted))?;
    }

    Ok(())
}

fn lint_path(path: impl AsRef<Path>) -> Option<String> {
    // Turn path into file name string
    let path = path.as_ref().file_name()?.to_string_lossy();

    let linted = path
        .chars()
        // Substitute certain characters
        .map(|c| {
            if c.is_whitespace() || c == '-' {
                '_'
            } else {
                c
            }
        })
        // Filter out all other characters
        .filter(|&c| c.is_alphanumeric() || c == '_' || c == '.')
        .collect();

    // If there was a change return it, otherwise return none
    if path == linted {
        None
    } else {
        Some(linted)
    }
}

// Remove the current dir from the start of the path, otherwise change nothing
fn relate_path(path: impl AsRef<Path>) -> PathBuf {
    env::current_dir()
        .ok()
        .and_then(|c| path.as_ref().strip_prefix(c).ok())
        .unwrap_or_else(|| path.as_ref())
        .into()
}
