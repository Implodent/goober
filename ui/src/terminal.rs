use crossterm::{Command, QueueableCommand};

pub use crossterm::style::ContentStyle as Paint;
use taffy::geometry::Point;

use super::*;

pub struct Terminal {
    writer: &'static mut dyn std::io::Write,
}

impl Terminal {
    pub fn new(writer: impl std::io::Write + 'static) -> Self {
        let leaked = Box::leak::<'static>(Box::new(writer));
        Self { writer: leaked }
    }
    pub fn queue(&mut self, command: impl Command) -> Result<(), std::io::Error> {
        self.writer.queue(command)?;

        Ok(())
    }

    pub fn move_to(&mut self, point: Point<f32>) -> Result<(), std::io::Error> {
        // unwrapping here because failing to do the conversion is a bug, so we crash the program
        // if it does fail
        self.queue(crossterm::cursor::MoveTo(
            (point.x.round() as i32).abs().try_into().unwrap(),
            (point.y.round() as i32).abs().try_into().unwrap(),
        ))
    }

    pub fn execute(&mut self, command: impl Command) -> Result<(), std::io::Error> {
        self.queue(command)?;
        self.flush()
    }

    pub fn flush(&mut self) -> Result<(), std::io::Error> {
        self.writer.flush()
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        // because the field is private the invariant that the value is owned by us is kept,
        // so we can use from_raw to construct the box again so we can drop it
        let _ = unsafe { Box::from_raw(self.writer as _) };
    }
}

pub struct SwitchIfTerminal<G, T> {
    pub graphical: G,
    pub terminal: T,
}

impl<G: View, T: View> View for SwitchIfTerminal<G, T> {
    fn ev(&self, event: &Event, how: &RenderContext) {
        if how.is_terminal {
            self.terminal.ev(event, how)
        } else {
            self.graphical.ev(event, how)
        }
    }
    fn style(&self) -> Style {
        self.graphical.style()
    }
    fn style_terminal(&self) -> Style {
        self.terminal.style_terminal()
    }
    fn render(&self, canvas: &Canvas, how: &RenderContext) {
        self.graphical.render(canvas, how)
    }
    fn measure(&self, current_node: Option<Node>, taffy: &mut Taffy) -> Node {
        self.graphical.measure(current_node, taffy)
    }
    fn render_terminal(
        &self,
        renderer: &mut Terminal,
        how: &RenderContext,
    ) -> Result<(), std::io::Error> {
        self.terminal.render_terminal(renderer, how)
    }
    fn measure_terminal(&self, current_node: Option<Node>, taffy: &mut Taffy) -> Node {
        self.terminal.measure_terminal(current_node, taffy)
    }
}

pub fn switch_render_mode<G: View, T: View>(graphical: G, terminal: T) -> SwitchIfTerminal<G, T> {
    SwitchIfTerminal {
        graphical,
        terminal,
    }
}
