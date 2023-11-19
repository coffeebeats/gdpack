package manifest

// Spec is a specification of a Godot addon, describing how to access the addon
// and, implicitly, which version to use.
type Spec struct {
	Version string `json:"version,omitempty"`

	*Asset // Embed asset-related fields
	*Git   // Embed git-related fields
	*Local // Embed local-related fields
}

// Asset specifies a Godot addon by the ID on the asset store.
type Asset struct {
	Asset uint `json:"asset,omitempty"`
}

// Git specifies a Godot addon by a URI to a git repository. An additional
// constraint can be added to specify the version. If no constraints are
// specified then the main branch will be used.
type Git struct {
	Git string `json:"git,omitempty"`

	Branch string `json:"branch,omitempty"`
	Tag    string `json:"tag,omitempty"`
	Rev    string `json:"rev,omitempty"`
}

// Local specifies a path to a locally-available addon.
type Local struct {
	Path string `json:"path,omitempty"`
}
