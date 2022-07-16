/// Options to handle includes, like `include.path` or `includeIf.<condition>.path`,
#[derive(Clone, Copy)]
pub struct Options<'a> {
    /// The maximum allowed length of the file include chain built by following nested resolve_includes where base level is depth = 0.
    pub max_depth: u8,
    /// When max depth is exceeded while following nested includes,
    /// return an error if true or silently stop following resolve_includes.
    ///
    /// Setting this value to false allows to read configuration with cycles,
    /// which otherwise always results in an error.
    pub error_on_max_depth_exceeded: bool,

    /// Used during path interpolation, both for include paths before trying to read the file, and for
    /// paths used in conditional `gitdir` includes.
    pub interpolate: crate::path::interpolate::Context<'a>,

    /// Additional context for conditional includes to work.
    pub conditional: conditional::Context<'a>,
}

impl Options<'_> {
    /// Provide options to never follow include directives at all.
    pub fn no_follow() -> Self {
        Options {
            max_depth: 0,
            error_on_max_depth_exceeded: false,
            interpolate: Default::default(),
            conditional: Default::default(),
        }
    }
}

impl<'a> Options<'a> {
    /// Provide options to follow includes like git does, provided the required `conditional` and `interpolate` contexts
    /// to support `gitdir` and `onbranch` based `includeIf` directives as well as standard `include.path` resolution.
    /// Note that the follow-mode is `git`-style, following at most 10 indirections while
    /// producing an error if the depth is exceeded.
    pub fn follow(interpolate: crate::path::interpolate::Context<'a>, conditional: conditional::Context<'a>) -> Self {
        Options {
            max_depth: 10,
            error_on_max_depth_exceeded: true,
            interpolate,
            conditional,
        }
    }

    /// Set the context used for interpolation when interpolating paths to include as well as the paths
    /// in `gitdir` conditional includes.
    pub fn interpolate_with(mut self, context: crate::path::interpolate::Context<'a>) -> Self {
        self.interpolate = context;
        self
    }
}

impl Default for Options<'_> {
    fn default() -> Self {
        Self::no_follow()
    }
}

///
pub mod conditional {
    /// Options to handle conditional includes like `includeIf.<condition>.path`.
    #[derive(Clone, Copy, Default)]
    pub struct Context<'a> {
        /// The location of the .git directory. If `None`, `gitdir` conditions cause an error.
        ///
        /// Used for conditional includes, e.g. `includeIf.gitdir:…` or `includeIf:gitdir/i…`.
        pub git_dir: Option<&'a std::path::Path>,
        /// The name of the branch that is currently checked out. If `None`, `onbranch` conditions cause an error.
        ///
        /// Used for conditional includes, e.g. `includeIf.onbranch:main.…`
        pub branch_name: Option<&'a git_ref::FullNameRef>,
    }
}
