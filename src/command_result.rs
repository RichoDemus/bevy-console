pub struct CommandResult {
    pub success: bool,
    pub message: Option<String>,
}

pub trait IntoCommandResult {
    fn into_command_result(self) -> CommandResult;
}

impl IntoCommandResult for CommandResult {
    fn into_command_result(self) -> CommandResult {
        self
    }
}

impl IntoCommandResult for () {
    fn into_command_result(self) -> CommandResult {
        CommandResult {
            success: true,
            message: None,
        }
    }
}

impl IntoCommandResult for String {
    fn into_command_result(self) -> CommandResult {
        CommandResult {
            success: true,
            message: Some(self),
        }
    }
}

impl IntoCommandResult for Option<String> {
    fn into_command_result(self) -> CommandResult {
        CommandResult {
            success: true,
            message: self,
        }
    }
}

impl IntoCommandResult for Result<String, String> {
    fn into_command_result(self) -> CommandResult {
        match self {
            Ok(msg) => CommandResult {
                success: true,
                message: Some(msg),
            },
            Err(err) => CommandResult {
                success: false,
                message: Some(err),
            },
        }
    }
}
