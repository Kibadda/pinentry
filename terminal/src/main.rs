use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};
use std::{
    env,
    error::Error,
    fs::File,
    io::{self, Stdout, Write},
};

struct State {
    description: String,
    prompt: String,
    input: String,
}

impl State {
    fn enter_char(&mut self, new_char: char) {
        self.input.insert(self.input.len(), new_char);
    }

    fn delete_char(&mut self) {
        if !self.input.is_empty() {
            self.input = self.input.chars().take(self.input.len() - 1).collect();
        }
    }
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(terminal.show_cursor()?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = setup_terminal()?;

    let mut state = State {
        description: env::var("DESCRIPTION")?,
        prompt: env::var("PROMPT")?,
        input: String::new(),
    };

    let res = run(&mut terminal, &mut state);

    restore_terminal(&mut terminal)?;

    if let Ok(Some(pin)) = res {
        let mut file = File::create(env::var("TMP_FILE")?)?;
        file.write_all(pin.as_bytes())?;
    }

    Ok(())
}

fn run(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    state: &mut State,
) -> io::Result<Option<String>> {
    loop {
        terminal.draw(|f| ui(f, state))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char(to_insert) => {
                        state.enter_char(to_insert);
                    }
                    KeyCode::Backspace => {
                        state.delete_char();
                    }
                    KeyCode::Enter => {
                        return Ok(Some(state.input.clone()));
                    }
                    KeyCode::Esc => return Ok(None),
                    _ => {}
                }
            }
        }
    }
}

fn ui(frame: &mut Frame, state: &State) {
    let text: Vec<Line> = state
        .description
        .split("%0A")
        .map(|l| Line::from(vec![l.bold().blue()]))
        .collect();

    let vertical = Layout::vertical([Constraint::Min(text.len() as u16), Constraint::Length(3)]);
    let [description_area, input_area] = vertical.areas(frame.size());
    let description = Paragraph::new(text).wrap(Wrap { trim: true });
    frame.render_widget(description, description_area);

    let input = Paragraph::new("*".repeat(state.input.len()))
        .style(Style::default())
        .block(Block::bordered().title(" ".to_owned() + &state.prompt + " "));
    frame.render_widget(input, input_area);
    frame.set_cursor(
        input_area.x + state.input.len() as u16 + 1,
        input_area.y + 1,
    );
}
