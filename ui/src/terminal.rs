use crossterm::{Command, QueueableCommand};

pub use crossterm::style::ContentStyle as Paint;

use super::*;

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
    SwitchIfTerminal { graphical, terminal }
}
