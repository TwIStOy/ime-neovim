#[derive(Debug, PartialEq, Clone)]
pub enum TreeSign {
  /// leftmost, *not* the last
  Edge,

  /// *not* the leftmost, and *not* the last
  Line,

  /// leftmost, the last
  Corner,

  /// *not* the leftmost, the last
  Blank,
}

impl TreeSign {
  pub fn ascii(&self) -> &'static str {
    match *self {
      TreeSign::Edge => "├──",
      TreeSign::Line => "│  ",
      TreeSign::Corner => "└──",
      TreeSign::Blank => "   ",
    }
  }
}

#[derive(Debug, Copy, Clone)]
pub struct TreeDepth(pub usize);

#[derive(Debug, Copy, Clone)]
pub struct TreeParam {
  last: bool,
  depth: TreeDepth,
}

#[derive(Debug, Default)]
pub struct TreeStream {
  pub nodes: Vec<TreeSign>,
  last_params: Option<TreeParam>,
}

impl TreeStream {
  pub fn new() -> TreeStream {
    TreeStream {
      nodes: Vec::new(),
      last_params: None,
    }
  }

  pub fn new_row(&mut self, param: TreeParam) -> &[TreeSign] {
    if let Some(last) = self.last_params {
      self.nodes[last.depth.0] = if last.last {
        TreeSign::Blank
      } else {
        TreeSign::Line
      };
    }

    self.nodes.resize(param.depth.0 + 1, TreeSign::Edge);
    self.nodes[param.depth.0] = if param.last {
      TreeSign::Corner
    } else {
      TreeSign::Edge
    };
    self.last_params = Some(param);

    &self.nodes[1..]
  }
}

impl TreeParam {
  pub fn new(depth: TreeDepth, last: bool) -> TreeParam {
    TreeParam {
      depth: depth,
      last: last,
    }
  }

  pub fn is_root(&self) -> bool {
    self.depth.0 == 0
  }
}

impl TreeDepth {
  pub fn root() -> TreeDepth {
    TreeDepth(0)
  }

  pub fn deeper(&self) -> TreeDepth {
    TreeDepth(self.0 + 1)
  }
}
