use iced::advanced::layout::padded;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Theme {
    /// The built-in light variant.
    Light,
    /// The built-in dark variant.
    Dark,
    /// The built-in Dracula variant.
    Dracula,
    /// The built-in Nord variant.
    Nord,
    /// The built-in Solarized Light variant.
    SolarizedLight,
    /// The built-in Solarized Dark variant.
    SolarizedDark,
    /// The built-in Gruvbox Light variant.
    GruvboxLight,
    /// The built-in Gruvbox Dark variant.
    GruvboxDark,
    /// The built-in Catppuccin Latte variant.
    CatppuccinLatte,
    /// The built-in Catppuccin Frappé variant.
    CatppuccinFrappe,
    /// The built-in Catppuccin Macchiato variant.
    CatppuccinMacchiato,
    /// The built-in Catppuccin Mocha variant.
    CatppuccinMocha,
    /// The built-in Tokyo Night variant.
    TokyoNight,
    /// The built-in Tokyo Night Storm variant.
    TokyoNightStorm,
    /// The built-in Tokyo Night Light variant.
    TokyoNightLight,
    /// The built-in Kanagawa Wave variant.
    KanagawaWave,
    /// The built-in Kanagawa Dragon variant.
    KanagawaDragon,
    /// The built-in Kanagawa Lotus variant.
    KanagawaLotus,
    /// The built-in Moonfly variant.
    Moonfly,
    /// The built-in Nightfly variant.
    Nightfly,
    /// The built-in Oxocarbon variant.
    Oxocarbon,
    /// The built-in Ferra variant:
    Ferra,
}

impl Theme {
    pub fn to_iced_theme(&self) -> iced::Theme {
        match self {
            Theme::Light => iced::Theme::Light,
            Theme::Dark => iced::Theme::Dark,
            Theme::Dracula => iced::Theme::Dracula,
            Theme::Nord => iced::Theme::Nord,
            Theme::SolarizedLight => iced::Theme::SolarizedLight,
            Theme::SolarizedDark => iced::Theme::SolarizedDark,
            Theme::GruvboxLight => iced::Theme::GruvboxLight,
            Theme::GruvboxDark => iced::Theme::GruvboxDark,
            Theme::CatppuccinLatte => iced::Theme::CatppuccinLatte,
            Theme::CatppuccinFrappe => iced::Theme::CatppuccinFrappe,
            Theme::CatppuccinMacchiato => iced::Theme::CatppuccinMacchiato,
            Theme::CatppuccinMocha => iced::Theme::CatppuccinMocha,
            Theme::TokyoNight => iced::Theme::TokyoNight,
            Theme::TokyoNightStorm => iced::Theme::TokyoNightStorm,
            Theme::TokyoNightLight => iced::Theme::TokyoNightLight,
            Theme::KanagawaWave => iced::Theme::KanagawaWave,
            Theme::KanagawaDragon => iced::Theme::KanagawaDragon,
            Theme::KanagawaLotus => iced::Theme::KanagawaLotus,
            Theme::Moonfly => iced::Theme::Moonfly,
            Theme::Nightfly => iced::Theme::Nightfly,
            Theme::Oxocarbon => iced::Theme::Oxocarbon,
            Theme::Ferra => iced::Theme::Ferra,
        }
    }
}

impl From<usize> for Theme {
    fn from(id: usize) -> Self {
        match id {
            0 => Theme::Light,
            1 => Theme::Dark,
            2 => Theme::Dracula,
            3 => Theme::Nord,
            4 => Theme::SolarizedLight,
            5 => Theme::SolarizedDark,
            6 => Theme::GruvboxLight,
            7 => Theme::GruvboxDark,
            8 => Theme::CatppuccinLatte,
            9 => Theme::CatppuccinFrappe,
            10 => Theme::CatppuccinMacchiato,
            11 => Theme::CatppuccinMocha,
            12 => Theme::TokyoNight,
            13 => Theme::TokyoNightStorm,
            14 => Theme::TokyoNightLight,
            15 => Theme::KanagawaWave,
            16 => Theme::KanagawaDragon,
            17 => Theme::KanagawaLotus,
            18 => Theme::Moonfly,
            19 => Theme::Nightfly,
            20 => Theme::Oxocarbon,
            21 => Theme::Ferra,
            _ => panic!("Theme not found"),
        }
    }
}
