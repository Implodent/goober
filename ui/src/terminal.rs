use crossterm::{
    Command, QueueableCommand,
};

pub use crossterm::style::ContentStyle as Paint;

pub struct Terminal<'a> {
    writer: &'a mut dyn std::io::Write,
}

impl<'a> Terminal<'a> {
    pub fn queue(&mut self, command: impl Command) -> Result<(), std::io::Error> {
        self.writer.queue(command)?;

        Ok(())
    }

    pub fn execute(&mut self, command: impl Command) -> Result<(), std::io::Error> {
        self.queue(command)?;
        self.flush()
    }

    pub fn flush(&mut self) -> Result<(), std::io::Error> {
        self.writer.flush()
    }
}
