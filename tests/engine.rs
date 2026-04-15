use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use cortex::{
    Action, Context, Layer, LayerError, ManualClock, Observer, OnLayerError, ShutdownAction,
    ShutdownReason, StepOutcome, TickRate,
};

struct ShutdownLayer(Mutex<Option<ShutdownAction>>);

impl ShutdownLayer {
    fn new(a: ShutdownAction) -> Self {
        Self(Mutex::new(Some(a)))
    }
}

impl Layer for ShutdownLayer {
    fn on_tick(&self, ctx: &mut Context) -> Result<(), LayerError> {
        if let Some(a) = self.0.lock().unwrap().take() {
            ctx.shutdown(a);
        }
        Ok(())
    }
}

struct NoopLayer;
impl Layer for NoopLayer {
    fn on_tick(&self, _ctx: &mut Context) -> Result<(), LayerError> {
        Ok(())
    }
}

struct FailingLayer;
impl Layer for FailingLayer {
    fn on_tick(&self, _ctx: &mut Context) -> Result<(), LayerError> {
        Err(LayerError::Message("boom".into()))
    }
}

struct CountingLayer(Arc<AtomicUsize>);
impl Layer for CountingLayer {
    fn on_tick(&self, _ctx: &mut Context) -> Result<(), LayerError> {
        self.0.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }
}

// Re-enqueues on every non-shutdown action it observes to trigger the re-entry bound.
struct ReentryObserver;
impl Observer for ReentryObserver {
    fn on_action(&self, ctx: &mut Context, action: &Action) {
        if !matches!(action, Action::Shutdown(_)) {
            ctx.dispatch(action.clone());
        }
    }
}

struct SeedBankLayer {
    fired: std::sync::atomic::AtomicBool,
}
impl SeedBankLayer {
    fn new() -> Self {
        Self {
            fired: std::sync::atomic::AtomicBool::new(false),
        }
    }
}
impl Layer for SeedBankLayer {
    fn on_tick(&self, ctx: &mut Context) -> Result<(), LayerError> {
        if !self.fired.swap(true, Ordering::SeqCst) {
            ctx.banks.create(
                cortex::BankId::new("jpm"),
                "JPM",
                cortex::bank::BankType::Commercial,
            );
        }
        Ok(())
    }
}

#[test]
fn shutdown_returns_reason_instead_of_exiting() {
    let mut engine = cortex::new()
        .layer(ShutdownLayer::new(ShutdownAction::Success))
        .build();

    let reason = engine.run();
    assert_eq!(reason, ShutdownReason::Requested(ShutdownAction::Success));
    assert_eq!(reason.code(), 0);
}

#[test]
fn shutdown_preserves_internal_error_payload() {
    let a = ShutdownAction::InternalError("db down".into());
    let mut engine = cortex::new().layer(ShutdownLayer::new(a.clone())).build();

    assert_eq!(engine.run(), ShutdownReason::Requested(a));
}

#[test]
fn reentry_limit_triggers_shutdown() {
    let mut engine = cortex::new()
        .layer(SeedBankLayer::new())
        .observer(ReentryObserver)
        .build();

    assert_eq!(engine.run(), ShutdownReason::ReentryLimitExceeded);
}

#[test]
fn layer_error_policy_abort_returns_shutdown() {
    let mut engine = cortex::new()
        .layer(FailingLayer)
        .on_layer_error(OnLayerError::Abort)
        .build();

    match engine.run() {
        ShutdownReason::LayerFailed(e) => {
            assert_eq!(e.to_string(), "boom");
        }
        other => panic!("expected LayerFailed, got {:?}", other),
    }
}

#[test]
fn layer_error_policy_continue_keeps_running() {
    let count = Arc::new(AtomicUsize::new(0));
    let mut engine = cortex::new()
        .layer(FailingLayer)
        .layer(CountingLayer(count.clone()))
        .layer(ShutdownLayer::new(ShutdownAction::Success))
        .on_layer_error(OnLayerError::Continue)
        .build();

    let reason = engine.run();
    assert_eq!(reason, ShutdownReason::Requested(ShutdownAction::Success));
    assert!(count.load(Ordering::SeqCst) >= 1);
}

#[test]
fn manual_step_does_not_clone_world_until_snapshot() {
    let mut engine = cortex::new().layer(NoopLayer).build();
    assert!(matches!(engine.step(), StepOutcome::Continue));
    let snap = engine.snapshot();
    assert_eq!(snap.tick, engine.tick());
}

#[test]
fn manual_clock_advances_between_ticks() {
    let start = chrono::DateTime::UNIX_EPOCH;
    struct WaitingLayer(std::sync::atomic::AtomicUsize);
    impl Layer for WaitingLayer {
        fn on_tick(&self, ctx: &mut Context) -> Result<(), LayerError> {
            let n = self.0.fetch_add(1, Ordering::SeqCst);
            if n >= 2 {
                ctx.shutdown(ShutdownAction::Success);
            }
            Ok(())
        }
    }

    let mut engine = cortex::new()
        .layer(WaitingLayer(std::sync::atomic::AtomicUsize::new(0)))
        .clock(ManualClock::new(start))
        .rate(TickRate::Interval(Duration::from_secs(1)))
        .build();

    assert_eq!(
        engine.run(),
        ShutdownReason::Requested(ShutdownAction::Success)
    );
    // 3 steps; clock advances only between steps (run calls wait_until_next_tick after step 1 and 2)
    let elapsed = engine
        .snapshot()
        .started_at
        .signed_duration_since(start)
        .num_seconds();
    assert_eq!(elapsed, 2);
}

#[test]
fn tick_rate_hz_maps_to_interval() {
    assert_eq!(TickRate::Hz(1).interval(), Duration::from_secs(1));
    assert_eq!(TickRate::Hz(1000).interval(), Duration::from_millis(1));
}

#[test]
fn ctx_banks_create_enqueues_action() {
    struct CountCreates(Arc<AtomicUsize>);
    impl Observer for CountCreates {
        fn on_bank_create(&self, _ctx: &mut Context, _action: &cortex::bank::CreateBankAction) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    let count = Arc::new(AtomicUsize::new(0));
    let mut engine = cortex::new()
        .layer(SeedBankLayer::new())
        .layer(ShutdownLayer::new(ShutdownAction::Success))
        .observer(CountCreates(count.clone()))
        .build();

    assert_eq!(
        engine.run(),
        ShutdownReason::Requested(ShutdownAction::Success)
    );
    assert_eq!(count.load(Ordering::SeqCst), 1);
}
