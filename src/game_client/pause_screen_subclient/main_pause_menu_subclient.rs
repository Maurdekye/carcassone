use ggez::{
    graphics::{Canvas, Rect, Text},
    input::keyboard::KeyCode,
    Context, GameError,
};
use std::sync::mpsc::{channel, Receiver, Sender};

use crate::{
    game_client::GameEvent,
    main_client::MainEvent,
    sub_event_handler::SubEventHandler,
    ui_manager::{Button, ButtonBounds, ButtonState, UIManager},
};

use super::PauseScreenEvent;

#[derive(Debug, Clone)]
pub enum MainPauseMenuEvent {
    PauseScreenEvent(PauseScreenEvent),
}

impl MainPauseMenuEvent {
    fn game_event(event: GameEvent) -> Self {
        Self::PauseScreenEvent(PauseScreenEvent::GameEvent(event))
    }

    fn main_event(event: MainEvent) -> Self {
        Self::game_event(GameEvent::MainEvent(event))
    }
}

pub struct MainPauseMenuSubclient {
    parent_channel: Sender<PauseScreenEvent>,
    event_sender: Sender<MainPauseMenuEvent>,
    event_receiver: Receiver<MainPauseMenuEvent>,
    ui: UIManager<MainPauseMenuEvent>,
}

impl MainPauseMenuSubclient {
    pub fn new(
        parent_channel: Sender<PauseScreenEvent>,
        is_endgame: bool,
        has_history: bool,
    ) -> MainPauseMenuSubclient {
        let (event_sender, event_receiver) = channel();
        let ui_sender = event_sender.clone();
        let button_center = Rect::new(0.5, 0.2, 0.0, 0.0);
        let (ui, [end_game_button, _, _, undo_button, ..]) = UIManager::new_and_rc_buttons(
            ui_sender,
            [
                Button::new(
                    ButtonBounds {
                        relative: button_center,
                        absolute: Rect::new(-250.0, 0.0, 240.0, 40.0),
                    },
                    Text::new("End Game"),
                    MainPauseMenuEvent::game_event(GameEvent::EndGame),
                ),
                Button::new(
                    ButtonBounds {
                        relative: button_center,
                        absolute: Rect::new(10.0, 0.0, 240.0, 40.0),
                    },
                    Text::new("Reset Camera"),
                    MainPauseMenuEvent::game_event(GameEvent::ResetCamera),
                ),
                Button::new(
                    ButtonBounds {
                        relative: button_center,
                        absolute: Rect::new(-250.0, 60.0, 240.0, 40.0),
                    },
                    Text::new("Return to Main Menu"),
                    MainPauseMenuEvent::main_event(MainEvent::ReturnToMainMenu),
                ),
                Button::new(
                    ButtonBounds {
                        relative: button_center,
                        absolute: Rect::new(10.0, 60.0, 240.0, 40.0),
                    },
                    Text::new("Undo Last Move"),
                    MainPauseMenuEvent::game_event(GameEvent::Undo),
                ),
                Button::new(
                    ButtonBounds {
                        relative: button_center,
                        absolute: Rect::new(-250.0, 120.0, 240.0, 40.0),
                    },
                    Text::new("Controls"),
                    MainPauseMenuEvent::PauseScreenEvent(PauseScreenEvent::Controls),
                ),
                Button::new(
                    ButtonBounds {
                        relative: button_center,
                        absolute: Rect::new(10.0, 120.0, 240.0, 40.0),
                    },
                    Text::new("Inspect Groups"),
                    MainPauseMenuEvent::game_event(GameEvent::InspectGroups),
                ),
            ],
        );
        end_game_button.borrow_mut().state = ButtonState::disabled_if(is_endgame);
        undo_button.borrow_mut().state = ButtonState::disabled_if(!has_history);
        MainPauseMenuSubclient {
            parent_channel,
            event_sender,
            event_receiver,
            ui,
        }
    }

    fn handle_event(
        &mut self,
        _ctx: &mut Context,
        event: MainPauseMenuEvent,
    ) -> Result<(), GameError> {
        use MainPauseMenuEvent::*;
        match event {
            PauseScreenEvent(event) => self.parent_channel.send(event).unwrap(),
        }
        Ok(())
    }
}

impl SubEventHandler<GameError> for MainPauseMenuSubclient {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        self.ui.update(ctx);

        if ctx.keyboard.is_key_just_pressed(KeyCode::Escape) {
            self.event_sender
                .send(MainPauseMenuEvent::game_event(GameEvent::ClosePauseMenu))
                .unwrap();
        }

        while let Ok(event) = self.event_receiver.try_recv() {
            self.handle_event(ctx, event)?;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas) -> Result<(), GameError> {
        self.ui.draw(ctx, canvas)
    }
}
