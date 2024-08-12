use cursive::{
    align::{HAlign, VAlign},
    event,
    theme::{BorderStyle, Color::*, PaletteColor::*, Theme},
    traits::*,
    views::*,
};
use std::collections::HashSet;

use crate::libs::checkbox;
use crate::libs::errors::Error;

fn inherit_terminal_colors(siv: &mut cursive::Cursive) {
    let mut palette = cursive::theme::Palette::default();

    palette[Background] = TerminalDefault;
    palette[Shadow] = TerminalDefault;
    palette[View] = TerminalDefault;
    palette[Primary] = TerminalDefault;
    palette[Secondary] = TerminalDefault;
    palette[Tertiary] = TerminalDefault;
    palette[TitlePrimary] = TerminalDefault;
    palette[TitleSecondary] = TerminalDefault;
    palette[Highlight] = TerminalDefault;
    palette[HighlightInactive] = TerminalDefault;
    palette[HighlightText] = TerminalDefault;

    let theme = Theme {
        shadow: false,
        borders: BorderStyle::None,
        palette,
    };

    siv.set_theme(theme);
}

#[derive(Debug, Default)]
struct AppState {
    selected_keywords: HashSet<String>,
    abort: bool,
}

pub fn run(keywords: Vec<(String, usize)>) -> Result<Vec<String>, Error> {
    let mut siv = cursive::default();
    inherit_terminal_colors(&mut siv);

    siv.set_user_data(AppState::default());

    siv.add_global_callback('q', |s| {
        s.user_data::<AppState>().unwrap().abort = true;
        s.quit();
    });
    siv.add_global_callback(event::Key::Esc, |s| {
        s.user_data::<AppState>().unwrap().abort = true;
        s.quit();
    });
    siv.add_global_callback(event::Key::Enter, |s| s.quit());
    siv.add_global_callback('j', |s| s.on_event(event::Event::Key(event::Key::Down)));
    siv.add_global_callback('k', |s| s.on_event(event::Event::Key(event::Key::Up)));

    siv.add_fullscreen_layer(
        LinearLayout::vertical()
            .child(
                TextView::new("Select keywords. ")
                    .h_align(HAlign::Left)
                    .v_align(VAlign::Top),
            )
            .child(
                TextView::new(
                    "<Space> to toggle check, <Enter> to run grouping, <q or ESC> to abort.",
                )
                .h_align(HAlign::Left)
                .v_align(VAlign::Top),
            )
            .child(DummyView.fixed_height(1))
            .child(
                ListView::new()
                    .with(|list| {
                        for (keyword, count) in keywords {
                            let k = keyword.clone();
                            let checkbox =
                                checkbox::Checkbox::new().on_change(move |s, checked| {
                                    if checked {
                                        s.user_data::<AppState>()
                                            .unwrap()
                                            .selected_keywords
                                            .insert(k.clone());
                                    } else {
                                        s.user_data::<AppState>()
                                            .unwrap()
                                            .selected_keywords
                                            .remove(&k);
                                    }
                                });
                            list.add_child(
                                "",
                                LinearLayout::horizontal()
                                    .child(checkbox)
                                    .child(DummyView.fixed_width(1))
                                    .child(TextView::new(format!("{count:>4}  {keyword}"))),
                            )
                        }
                    })
                    .scrollable(),
            )
            .child(DummyView.fixed_height(1)),
    );

    siv.run();

    let state = siv.user_data::<AppState>().unwrap();

    if state.abort {
        return Ok(vec![]);
    }

    let selected_keywords = state.selected_keywords.clone().into_iter().collect();

    Ok(selected_keywords)
}
