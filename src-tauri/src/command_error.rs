use anyhow::Context;
use log::error;

pub struct CommandError {
    pub for_user_message: String,
    pub error: Option<anyhow::Error>,
}

impl CommandError {
    ///Add error log to console
    /// # Return
    /// for user data (with additional lines to check the logs)
    pub fn show(self) -> String {
        if let Some(error) = self.error {
            error!("Command failed: {} | Debug: {:?}", error, error);
        }

        self.for_user_message
    }

    pub fn only_for_user(message: String) -> Self {
        Self {
            for_user_message: message,
            error: None,
        }
    }
}

impl Into<anyhow::Error> for CommandError {
    fn into(self) -> anyhow::Error {
        self.error
            .unwrap_or_else(|| anyhow::anyhow!("{}", self.for_user_message))
    }
}

pub trait CommandResult<T> {
    fn anyhow(self) -> anyhow::Result<T>;

    /// Wrap the error value with additional context.
    fn context<C>(self, context: C) -> Result<T, CommandError>
    where
        C: std::fmt::Display + Send + Sync + 'static;

    /// Wrap the error value with additional context that is evaluated lazily
    /// only once an error does occur.
    fn with_context<C, F>(self, f: F) -> Result<T, CommandError>
    where
        C: std::fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C;
}

impl<T> CommandResult<T> for Result<T, CommandError> {
    fn anyhow(self) -> anyhow::Result<T> {
        self.map_err(|err| err.into())
    }

    fn context<C>(self, context: C) -> Result<T, CommandError>
    where
        C: std::fmt::Display + Send + Sync + 'static,
    {
        match self {
            Ok(o) => Ok(o),
            Err(err) => {
                let new_err = if let Some(anyhow_err) = err.error {
                    anyhow_err.context(context)
                } else {
                    anyhow::anyhow!("{}", context)
                };

                Err(CommandError {
                    for_user_message: err.for_user_message,
                    error: Some(new_err),
                })
            }
        }
    }

    fn with_context<C, F>(self, f: F) -> Result<T, CommandError>
    where
        C: std::fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        match self {
            Ok(o) => Ok(o),
            Err(err) => {
                let context = f();
                let new_err = if let Some(anyhow_err) = err.error {
                    anyhow_err.context(context)
                } else {
                    anyhow::anyhow!("{}", context)
                };

                Err(CommandError {
                    for_user_message: err.for_user_message,
                    error: Some(new_err),
                })
            }
        }
    }
}

pub trait ForUserAnyError<T> {
    /// Wrap the error value with additional context.
    fn context_for_user<C>(self, context: C) -> Result<T, CommandError>
    where
        C: std::fmt::Display + Send + Sync + 'static;

    /// Wrap the error value with additional context that is evaluated lazily
    /// only once an error does occur.
    fn with_context_for_user<C, F>(self, f: F) -> Result<T, CommandError>
    where
        C: std::fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C;
}

impl<T, E> ForUserAnyError<T> for Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn context_for_user<C>(self, context: C) -> Result<T, CommandError>
    where
        C: std::fmt::Display + Send + Sync + 'static,
    {
        match self {
            Ok(o) => Ok(o),
            Err(err) => Err(CommandError {
                for_user_message: format!("{}\r\n\r\nFor more info check logs.txt", context),
                error: Some(anyhow::Error::new(err).context(context)),
            }),
        }
    }

    fn with_context_for_user<C, F>(self, f: F) -> Result<T, CommandError>
    where
        C: std::fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        match self {
            Ok(o) => Ok(o),
            Err(err) => {
                let context = f();
                Err(CommandError {
                    for_user_message: format!("{}\r\n\r\nFor more info check logs.txt", context),
                    error: Some(anyhow::Error::new(err).context(context)),
                })
            }
        }
    }
}

impl<T> ForUserAnyError<T> for Option<T> {
    fn context_for_user<C>(self, context: C) -> Result<T, CommandError>
    where
        C: std::fmt::Display + Send + Sync + 'static,
    {
        match self {
            Some(o) => Ok(o),
            None => Err(CommandError {
                for_user_message: format!("{}\r\n\r\nFor more info check logs.txt", context),
                error: Some(None::<()>.context(context).unwrap_err()),
            }),
        }
    }

    fn with_context_for_user<C, F>(self, f: F) -> Result<T, CommandError>
    where
        C: std::fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        match self {
            Some(o) => Ok(o),
            None => {
                let context = f();
                Err(CommandError {
                    for_user_message: format!("{}\r\n\r\nFor more info check logs.txt", context),
                    error: Some(None::<()>.context(context).unwrap_err()),
                })
            }
        }
    }
}

///Copy of the original was created to implement it for the anyhow Error, since the built in compiler blocks don't allow for the first one to be implemented
pub trait ForUserAnyError2<T> {
    /// Wrap the error value with additional context.
    fn context_for_user<C>(self, context: C) -> Result<T, CommandError>
    where
        C: std::fmt::Display + Send + Sync + 'static;

    /// Wrap the error value with additional context that is evaluated lazily
    /// only once an error does occur.
    fn with_context_for_user<C, F>(self, f: F) -> Result<T, CommandError>
    where
        C: std::fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C;
}

impl<T> ForUserAnyError2<T> for anyhow::Result<T> {
    fn context_for_user<C>(self, context: C) -> Result<T, CommandError>
    where
        C: std::fmt::Display + Send + Sync + 'static,
    {
        match self {
            Ok(o) => Ok(o),
            Err(err) => Err(CommandError {
                for_user_message: format!("{}\r\n\r\nFor more info check logs.txt", context),
                error: Some(err.context(context)),
            }),
        }
    }

    fn with_context_for_user<C, F>(self, f: F) -> Result<T, CommandError>
    where
        C: std::fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        match self {
            Ok(o) => Ok(o),
            Err(err) => {
                let context = f();
                Err(CommandError {
                    for_user_message: format!("{}\r\n\r\nFor more info check logs.txt", context),
                    error: Some(err.context(context)),
                })
            }
        }
    }
}
pub trait ForUserError<T> {
    fn for_user<C>(self, for_user_info: C) -> Result<T, CommandError>
    where
        C: std::fmt::Display + Send + Sync + 'static;
}

impl<T> ForUserError<T> for Result<T, anyhow::Error> {
    fn for_user<C>(self, for_user_info: C) -> Result<T, CommandError>
    where
        C: std::fmt::Display + Send + Sync + 'static,
    {
        match self {
            Ok(o) => Ok(o),
            Err(err) => Err(CommandError {
                for_user_message: format!("{}\r\n\r\nFor more info check logs.txt", for_user_info),
                error: Some(err),
            }),
        }
    }
}
