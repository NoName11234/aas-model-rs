use serde::{Deserialize, Serialize};

///A resource represents an address to a file (a locator). The value is a URI that can represent an
/// absolute or relative path.
#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct Resource {
    ///Path and name of the resource (with file extension).
    path: String,
    ///Content type of the content of the file.
    #[serde(rename = "contentType")]
    content_type: Option<String>
}

impl Resource {
    ///Creates a new instance of the struct.
    /// [path]: path and name
    pub fn new(path: String) -> Resource {
        Resource {
            path,
            content_type: None
        }
    }

    ///Sets the path and name of the resource.
    /// [path]: path and name
    pub fn set_path(&mut self, path: String) {
        self.path = path;
    }

    ///Returns the path and name of the resource.
    pub fn get_path(&self) -> &String {
        &self.path
    }

    ///Sets the content type of the content of the file.
    /// [content_type]: content type of the content
    pub fn set_content_type(&mut self, content_type: String) {
        self.content_type = Some(content_type);
    }

    ///Returns the content type of the content of the file.
    pub fn get_content_type(&self) -> Option<&String> {
        self.content_type.as_ref()
    }
}