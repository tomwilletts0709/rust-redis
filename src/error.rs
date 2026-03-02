use std::fmt; 
use std::io;
use std::sync::Arc;

#[derive(Debug)]
pub enum Error {
    // RESP parsing errors
    ProtocolError(String), 

    WrongType(String),
    InvalidCommand(String),
    KeyNotFound(String),
    OutOfMemory(String),
    OutOfBounds(usize),
    ParseInt(String),
    Io(io::Error),
    Sync(std::sync::PoisonError<Arc<std::sync::MutexGuard<'a, T>>>>),
    Timeout(std::time::Duration),
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        match self {
            Self::ProtocolError(e) => write!(f, "Protocol error: {}", e),
            Self::WrongType(e) => write!(f, "Wrong type: {}", e),
            Self::InvalidCommand(e) => write!(f, "Invalid command: {}", e),
            Self::KeyNotFound(e) => write!(f, "Key not found: {}", e),
            Self::OutOfMemory(e) => write!(f, "Out of memory: {}", e),
            Self::OutOfBounds(e) => write!(f, "Out of bounds: {}", e),
            Self::ParseInt(e) => write!(f, "Parse int: {}", e),
            Self::Io(e) => write!(f, "IO error: {}", e),
            Self::Sync(e) => write!(f, "Sync error: {}", e),
            Self::Timeout(e) => write!(f, "Timeout: {}", e),
            Self::Other(e) => write!(f, "Other error: {}", e),
        }
    }
}



macro_rules! make_error {
    ($err_macro:ident, $err_value_macro:ident, $variant:ident) => {
        make_error!(@inner ($), $err_macro, $err_value_macro, $variant);
    };
    (@inner ($d:tt), $err_macro:ident, $err_value_macro:ident, $variant:ident) => {
        ::paste::paste! {
            /// Macro wraps `$variant` to add backtrace feature
            #[macro_export]
            macro_rules! $err_value_macro {
                ($d($d args:expr),* $d(; diagnostic=$d diagnostic:expr)?) => {{
                    let err = $crate::DataFusionError::$variant(
                        ::std::format!(
                            "{}{}",
                            ::std::format!($d($d args),*),
                            $crate::DataFusionError::get_back_trace(),
                        ).into()
                    );
                    $d (
                        let err = err.with_diagnostic($d diagnostic);
                    )?
                    err
                }
            }
        }

            /// Macro wraps Err(`$variant`) to add backtrace feature
            #[macro_export]
            macro_rules! $err_macro {
                ($d($d args:expr),* $d(; diagnostic = $d diagnostic:expr)?) => {{
                    let err = $crate::[<_ $err_value_macro>]!($d($d args),*);
                    $d (
                        let err = err.with_diagnostic($d diagnostic);
                    )?
                    Err(err)
                }}
            }

            #[doc(hidden)]
            pub use $err_macro as [<_ $err_macro>];
            #[doc(hidden)]
            pub use $err_value_macro as [<_ $err_value_macro>];
        }
    };
}

// Exposes a macro to create `DataFusionError::Plan` with optional backtrace
make_error!(plan_err, plan_datafusion_err, Plan);

// Exposes a macro to create `DataFusionError::Internal` with optional backtrace
make_error!(internal_err, internal_datafusion_err, Internal);

// Exposes a macro to create `DataFusionError::NotImplemented` with optional backtrace
make_error!(not_impl_err, not_impl_datafusion_err, NotImplemented);

// Exposes a macro to create `DataFusionError::Execution` with optional backtrace
make_error!(exec_err, exec_datafusion_err, Execution);

// Exposes a macro to create `DataFusionError::Configuration` with optional backtrace
make_error!(config_err, config_datafusion_err, Configuration);

// Exposes a macro to create `DataFusionError::Substrait` with optional backtrace
make_error!(substrait_err, substrait_datafusion_err, Substrait);

// Exposes a macro to create `DataFusionError::ResourcesExhausted` with optional backtrace
make_error!(resources_err, resources_datafusion_err, ResourcesExhausted);

// Exposes a macro to create `DataFusionError::Ffi` with optional backtrace
make_error!(ffi_err, ffi_datafusion_err, Ffi);