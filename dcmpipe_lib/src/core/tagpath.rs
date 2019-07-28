
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TagPathElement {
    tag: u32,
    index: Option<u32>,
}

impl TagPathElement {
    pub fn new(tag: u32, index: Option<u32>) -> TagPathElement {
        TagPathElement {
            tag,
            index,
        }
    }

    pub fn get_tag(&self) -> u32 {
        self.tag
    }

    pub fn get_index(&self) -> Option<u32> {
        self.index
    }

    pub fn get_index_mut(&mut self) -> &mut Option<u32> {
        &mut self.index
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TagPath {
    path: Vec<TagPathElement>,
}

impl TagPath {
    pub fn new() -> TagPath {
        TagPath {
            path: Vec::new(),
        }
    }

    pub fn new_from_vec(path: Vec<TagPathElement>) -> TagPath {
        TagPath {
            path,
        }
    }

    pub fn new_from_tag(tag: u32) -> TagPath {
        let path: Vec<TagPathElement> = vec![TagPathElement::new(tag, None)];
        TagPath {
            path,
        }
    }
}
