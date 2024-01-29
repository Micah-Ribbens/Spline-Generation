/// A command of a way point
trait Command {
    fn execute(&self, t_value: float64);
}

struct WayPoint {
    t_value: float64,
    command: Command
}


/// A point along the path that executes a command at a certain point along the path (t_value)
impl WayPoint {
    /// Defines a new way point with a specified command and t_value that the command will be executed at
    pub fn new(t_value: float64, command: Command) -> WayPoint {
        WayPoint {
            t_value,
            command,
        }
    }

    pub fn get_t_value(&self) -> int32 {
        return self.t_value;
    }

    /// Executes the command of the way point
    pub fn execute_command(&self, t_value: float64) {
        self.command.execute(t_value);
    }
}