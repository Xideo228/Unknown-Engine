use std::str::FromStr;

use crate::graphics::shader::property::semantic_error::ShaderSemanticParseError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EngineSemantic {
    Model,
    View,
    Projection,
    MVP,
    NormalMatrix,

    CameraPosition,
    Time,
    DeltaTime,

    ScreenWidth,
    ScreenHeight,

    MainTexture,
    Color
}

impl EngineSemantic {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Model => "Model",
            Self::View => "View",
            Self::Projection => "Projection",
            Self::MVP => "MVP",
            Self::NormalMatrix => "NormalMatrix",

            Self::CameraPosition => "CameraPosition",
            Self::Time => "Time",
            Self::DeltaTime => "DeltaTime",

            Self::ScreenWidth => "ScreenWidth",
            Self::ScreenHeight => "ScreenHeight",

            Self::MainTexture => "MainTexture",
            Self::Color => "Color"
        }
    }
}

impl FromStr for EngineSemantic {
    type Err = ShaderSemanticParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Model" => Ok(Self::Model),
            "View" => Ok(Self::View),
            "Projection" => Ok(Self::Projection),
            "MVP" => Ok(Self::MVP),
            "NormalMatrix" => Ok(Self::NormalMatrix),
            "CameraPosition" => Ok(Self::CameraPosition), 
            "Time" => Ok(Self::Time),
            "DeltaTime" => Ok(Self::DeltaTime),
            "ScreenWidth" => Ok(Self::ScreenWidth),
            "ScreenHeight" => Ok(Self::ScreenHeight),
            "MainTexture" => Ok(Self::MainTexture),
            "Color" => Ok(Self::Color)

            _ => Err(ShaderSemanticParseError::UnknownEngineSemantic(s.to_owned()))
        }
    }
}