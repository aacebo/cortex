use crate::Tick;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Action {
    Create,
    Update,
    Delete,
}

impl Action {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Create => "create",
            Self::Update => "update",
            Self::Delete => "delete",
        }
    }
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Clone)]
pub struct Event<'a, T> {
    pub id: ulid::Ulid,
    pub tick: Tick,
    pub action: Action,
    pub body: &'a T,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl<'a, T> Event<'a, T> {
    pub(crate) fn new(tick: Tick, action: Action, body: &'a T) -> Self {
        Self {
            id: ulid::Ulid::new(),
            tick,
            action,
            body,
            created_at: chrono::Utc::now(),
        }
    }

    pub fn with_body<'b, U>(&self, body: &'b U) -> Event<'b, U> {
        Event {
            id: self.id,
            tick: self.tick,
            action: self.action,
            body,
            created_at: self.created_at,
        }
    }
}

impl<'a, T: std::fmt::Debug> std::fmt::Debug for Event<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Event")
            .field("id", &self.id)
            .field("tick", &self.tick)
            .field("action", &self.action)
            .field("body", &self.body)
            .field("created_at", &self.created_at)
            .finish()
    }
}
