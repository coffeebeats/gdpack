package manifest

type Spec struct {
	Version string `json:"version,omitempty"`

	// Asset-related fields
	*Asset

	// Git-related fields
	*Git

	// Local-related fields
	*Local
}

type Asset struct {
	Asset uint `json:"asset,omitempty"`
}

type Git struct {
	Git string `json:"git,omitempty"`

	Branch string `json:"branch,omitempty"`
	Tag    string `json:"tag,omitempty"`
	Rev    string `json:"rev,omitempty"`
}

type Local struct {
	Path string `json:"path,omitempty"`
}
