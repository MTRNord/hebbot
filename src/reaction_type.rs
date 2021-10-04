use std::fmt;

use crate::{Project, Section};

#[derive(Clone, Debug, PartialEq)]
pub enum ReactionType {
    Notification,
    Approval,
    Section(Option<Section>),
    Project(Option<Project>),
    Image,
    Video,
    None,
}

impl fmt::Display for ReactionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReactionType::Notification => write!(f, "notification"),
            ReactionType::Approval => write!(f, "approval"),
            ReactionType::Section(_) => write!(f, "section"),
            ReactionType::Project(_) => write!(f, "project"),
            ReactionType::Image => write!(f, "image"),
            ReactionType::Video => write!(f, "video"),
            ReactionType::None => write!(f, "NONE"),
        }
    }
}
