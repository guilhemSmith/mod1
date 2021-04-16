use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[macro_export]
macro_rules! map_engine_error {
	($( $res:expr, $kind:ident )?) => {
		$($res.map_err(|_| crate::engine::EngineError::$kind {
			file: file!(),
			line: line!(),
		}))?
	};
	($( $res:expr, $kind:ident, $err:expr )?) => {
		$($res.map_err(|e| crate::engine::EngineError::$kind {
			file: file!(),
			line: line!(),
			info: format!("{}: {:?}", $err, e)
		}))?
	};
}

#[macro_export]
macro_rules! engine_error {
	($( $kind:ident )?) => {
		$(crate::engine::EngineError::$kind {
			file: file!(),
			line: line!(),
		})?
	};
	($( $kind:ident, $err:expr )?) => {
		$(crate::engine::EngineError::$kind {
			file: file!(),
			line: line!(),
			info: $err
		})?
	};
}

#[derive(Debug)]
pub enum EngineError {
	BadCString {
		file: &'static str,
		line: u32,
	},
	ShaderFail {
		file: &'static str,
		line: u32,
		info: String,
	},
	FileError {
		file: &'static str,
		line: u32,
		info: String,
	},
	GLError {
		file: &'static str,
		line: u32,
		info: String,
	},
}

impl Error for EngineError {}

impl Display for EngineError {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		write!(
			f,
			"[Engine Error]{}",
			match self {
				EngineError::BadCString { file, line } =>
					format!("[f:'{}';l:{}]: Tried to use a bad CString ", file, line),
				EngineError::ShaderFail { file, line, info } =>
					format!("[f:'{}';l:{}]: {}", file, line, info),
				EngineError::FileError { file, line, info } =>
					format!("[f:'{}';l:{}]: {}", file, line, info),
				EngineError::GLError { file, line, info } =>
					format!("[f:'{}';l:{}]: {}", file, line, info),
			}
		)
	}
}
