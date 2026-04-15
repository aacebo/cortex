use crate::Tick;

pub struct Snapshot<T> {
    pub tick: Tick,
    pub data: T,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub ended_at: chrono::DateTime<chrono::Utc>,
}

impl<T: Clone> Clone for Snapshot<T> {
    fn clone(&self) -> Self {
        Self {
            tick: self.tick,
            data: self.data.clone(),
            started_at: self.started_at,
            ended_at: self.ended_at,
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Snapshot<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("Snapshot<{}>", std::any::type_name::<T>()))
            .field("tick", &self.tick)
            .field("data", &self.data)
            .field("started_at", &self.started_at)
            .field("ended_at", &self.ended_at)
            .finish()
    }
}
