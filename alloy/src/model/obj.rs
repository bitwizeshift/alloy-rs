//! A module for parsing .obj files.
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

/// A model that is loaded from a .obj file.
///
/// This is a raw representation of the `obj` file without doing any manipulation
/// of the data. In order to be better consumed by the rendering engine, the data
/// should be converted into a more appropriate format first.
///
/// This type is largely used in tooling for pipeline processing of models.
#[derive(Debug, Default)]
pub struct Model {
  /// The vertices of the model.
  pub vertices: Vec<[f32; 4]>,

  /// The normals of the model.
  pub normals: Vec<[f32; 3]>,

  /// The texture coordinates of the model.
  pub tex_coords: Vec<[f32; 3]>,

  /// The indices of the vertices that make up the faces of the model.
  pub vertex_indices: Vec<[u32; 3]>,

  /// The indices of the texture coordinates that make up the faces of the model.
  pub tex_coords_indices: Vec<[u32; 3]>,

  /// The indices of the normals that make up the faces of the model.
  pub normal_indices: Vec<[u32; 3]>,

  /// The number of components in each vertex.
  pub vertex_components: usize,

  /// The number of components in each texture coordinate.
  pub tex_coord_components: usize,
}

// Constructors

impl Model {
  /// Create a new model.
  pub fn new() -> Self {
    Self::default()
  }

  /// Load a model from a stored .obj file.
  ///
  /// # Parameters
  ///
  /// * `file_path` - The path to the .obj file to load.
  pub fn from_file(file_path: &str) -> Result<Self> {
    let path = Path::new(file_path);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut model = Model::new();

    for line in reader.lines() {
      let line = line?;
      let parts: Vec<_> = line.split_whitespace().collect();

      if parts.is_empty() {
        continue;
      }

      match parts[0] {
        "#" => {} // Ignore comments
        "v" => model.parse_vertex(&parts[1..])?,
        "vn" => model.parse_normal(&parts[1..])?,
        "vt" => model.parse_tex_coords(&parts[1..])?,
        "f" => model.parse_face(&parts[1..])?,
        "s" => {} // Ignore smoothing groups

        _ => {
          return Err(Error::ParseErr(ParseError::UnsupportedKey(
            parts[0].to_string(),
          )))
        }
      }
    }

    Ok(model)
  }
}

// Parsing

impl Model {
  fn parse_vertex(&mut self, parts: &[&str]) -> Result<()> {
    if parts.len() < 3 || parts.len() > 4 {
      return Err(Error::ParseErr(ParseError::BadDefinition {
        kind: Kind::Vertex,
        parts: parts.len(),
      }));
    }

    let float_err = |e| {
      Error::ParseErr(ParseError::FloatParseError {
        kind: Kind::Vertex,
        underlying: e,
      })
    };
    let mut vertex = [0.0f32; 4];
    vertex[0] = parts[0].parse().map_err(float_err)?;
    vertex[1] = parts[1].parse().map_err(float_err)?;
    vertex[2] = parts[2].parse().map_err(float_err)?;
    if parts.len() == 4 {
      vertex[3] = parts[3].parse().map_err(float_err)?;
      self.vertex_components = 4;
    } else {
      vertex[3] = 1.0;
      self.vertex_components = 3;
    }

    self.vertices.push(vertex);
    Ok(())
  }

  fn parse_normal(&mut self, parts: &[&str]) -> Result<()> {
    if parts.len() != 3 {
      return Err(Error::ParseErr(ParseError::BadDefinition {
        kind: Kind::Normal,
        parts: parts.len(),
      }));
    }
    let float_err = |e| {
      Error::ParseErr(ParseError::FloatParseError {
        kind: Kind::Normal,
        underlying: e,
      })
    };
    let normal = [
      parts[0].parse().map_err(float_err)?,
      parts[1].parse().map_err(float_err)?,
      parts[2].parse().map_err(float_err)?,
    ];
    self.normals.push(normal);
    Ok(())
  }

  fn parse_tex_coords(&mut self, parts: &[&str]) -> Result<()> {
    if parts.len() > 3 {
      return Err(Error::ParseErr(ParseError::BadDefinition {
        kind: Kind::TexCoord,
        parts: parts.len(),
      }));
    }
    let float_err = |e| {
      Error::ParseErr(ParseError::FloatParseError {
        kind: Kind::TexCoord,
        underlying: e,
      })
    };
    let mut tex_coord = [0.0f32; 3];

    match parts.len() {
      1 => {
        tex_coord[0] = parts[0].parse().map_err(float_err)?;
        self.tex_coord_components = 1;
      }
      2 => {
        tex_coord[0] = parts[0].parse().map_err(float_err)?;
        tex_coord[1] = parts[1].parse().map_err(float_err)?;
        self.tex_coord_components = 2;
      }
      3 => {
        tex_coord[0] = parts[0].parse().map_err(float_err)?;
        tex_coord[1] = parts[1].parse().map_err(float_err)?;
        tex_coord[2] = parts[2].parse().map_err(float_err)?;
        self.tex_coord_components = 3;
      }
      _ => unreachable!(),
    }

    self.tex_coords.push(tex_coord);
    Ok(())
  }

  fn parse_face(&mut self, parts: &[&str]) -> Result<()> {
    if parts.len() != 3 {
      return Err(Error::ParseErr(ParseError::BadDefinition {
        kind: Kind::Face,
        parts: parts.len(),
      }));
    }

    let mut vertex = [0u32; 3];
    let mut tex_coords = [0u32; 3];
    let mut normals = [0u32; 3];
    let mut has_tex_coords = false;
    let mut has_normals = false;

    let map_err = || {
      Error::ParseErr(ParseError::BadDefinition {
        kind: Kind::Face,
        parts: 0,
      })
    };

    for (i, part) in parts.iter().enumerate() {
      let indices = self.parse_face_vertex(&part.split('/').collect::<Vec<_>>())?;
      vertex[i] = indices[0].ok_or_else(map_err)? - 1;

      if let Some(tex_coord) = indices[1] {
        tex_coords[i] = tex_coord - 1;
        has_tex_coords = true;
      }
      if let Some(normal) = indices[2] {
        normals[i] = normal - 1;
        has_normals = true;
      }
    }

    self.vertex_indices.push(vertex);
    if has_tex_coords {
      self.tex_coords_indices.push(tex_coords);
    }
    if has_normals {
      self.normal_indices.push(normals);
    }

    Ok(())
  }

  fn parse_face_vertex(&mut self, parts: &[&str]) -> Result<[Option<u32>; 3]> {
    let index_err = |e| {
      Error::ParseErr(ParseError::IndexParseError {
        kind: Kind::Face,
        underlying: e,
      })
    };
    match parts.len() {
      1 => {
        let vertex = parts[0].parse().map_err(index_err)?;
        Ok([Some(vertex), None, None])
      }
      2 => {
        let vertex = parts[0].parse().map_err(index_err)?;
        let tex_coords = parts[1].parse().map_err(|e| {
          Error::ParseErr(ParseError::IndexParseError {
            kind: Kind::Face,
            underlying: e,
          })
        })?;
        Ok([Some(vertex), Some(tex_coords), None])
      }
      3 => {
        let vertex = Some(parts[0].parse().map_err(index_err)?);
        let mut tex_coords = None;
        if !parts[1].is_empty() {
          tex_coords = Some(parts[1].parse().map_err(index_err)?);
        }
        let normal = Some(parts[2].parse().map_err(index_err)?);
        Ok([vertex, tex_coords, normal])
      }
      _ => Err(Error::ParseErr(ParseError::BadDefinition {
        kind: Kind::Face,
        parts: parts.len(),
      })),
    }
  }
}

/// The underlying kind of element being referenced in the obj file.
#[derive(Debug)]
pub enum Kind {
  /// A vertex
  Vertex,

  /// A normal
  Normal,

  /// A texture coordinate
  TexCoord,

  /// A face
  Face,
}

impl std::fmt::Display for Kind {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(match self {
      Self::Vertex => "vertex",
      Self::Normal => "normal",
      Self::TexCoord => "texcoord",
      Self::Face => "face",
    })
  }
}

/// An error that is emitted when parsing a badly defined .obj file.
#[derive(Debug)]
pub enum ParseError {
  /// The error that is emitted when a definition has too few or too many components.
  BadDefinition {
    /// The kind of element that was being parsed.
    kind: Kind,
    /// The number of components that were parsed.
    parts: usize,
  },
  /// The error that is emitted when a float cannot be parsed.
  FloatParseError {
    /// The kind of element that was being parsed.
    kind: Kind,
    /// The underlying error that was emitted by the float parser.
    underlying: <f32 as FromStr>::Err,
  },
  /// The error that is emitted when an index cannot be parsed.
  IndexParseError {
    /// The kind of element that was being parsed.
    kind: Kind,
    /// The underlying error that was emitted by the index parser.
    underlying: <u32 as FromStr>::Err,
  },
  /// The error that is emitted when an unsupported key is encountered.
  UnsupportedKey(String),
}

impl std::fmt::Display for ParseError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::BadDefinition { kind, parts } => {
        if parts > &3usize {
          write!(f, "too many components for {}", kind)
        } else {
          write!(f, "too few components for {}", kind)
        }
      }
      Self::FloatParseError { kind, underlying } => {
        write!(f, "unable to parse f32 for {}: {}", kind, underlying)
      }
      Self::IndexParseError { kind, underlying } => {
        write!(f, "unable to parse u32 for {}: {}", kind, underlying)
      }
      Self::UnsupportedKey(key) => write!(f, "unsupported key: {}", key),
    }
  }
}

/// An error that is emitted when parsing a .obj file.
#[derive(Debug)]
pub enum Error {
  /// The error that is emitted when a parse error occurs.
  ParseErr(ParseError),
  /// The error that is emitted when an IO error occurs.
  IOErr(io::Error),
}

impl From<ParseError> for Error {
  fn from(value: ParseError) -> Self {
    Self::ParseErr(value)
  }
}

impl From<io::Error> for Error {
  fn from(value: io::Error) -> Self {
    Self::IOErr(value)
  }
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::ParseErr(err) => err.fmt(f),
      Self::IOErr(err) => err.fmt(f),
    }
  }
}

type Result<T> = std::result::Result<T, Error>;

impl std::error::Error for ParseError {}
