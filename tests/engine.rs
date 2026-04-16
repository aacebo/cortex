use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use cortex::{Command, Context, Layer, ManualClock, Observer, Shutdown, ShutdownRequest, TickRate};

struct ShutdownLayer(Mutex<Option<ShutdownRequest>>);

impl ShutdownLayer {
    fn new(a: ShutdownRequest) -> Self {
        Self(Mutex::new(Some(a)))
    }
}

impl Layer for ShutdownLayer {
    fn on_tick(&self, ctx: &mut Context) {
        if let Some(a) = self.0.lock().unwrap().take() {
            ctx.shutdown(a);
        }
    }
}

struct NoopLayer;
impl Layer for NoopLayer {
    fn on_tick(&self, _ctx: &mut Context) {}
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
    fn on_tick(&self, ctx: &mut Context) {
        if !self.fired.swap(true, Ordering::SeqCst) {
            ctx.banks.create(
                cortex::BankId::new("jpm"),
                "JPM",
                cortex::bank::BankType::Commercial,
            );
        }
    }
}

struct CountingLayer(AtomicUsize);
impl CountingLayer {
    fn new() -> Self {
        Self(AtomicUsize::new(0))
    }
}
impl Layer for CountingLayer {
    fn on_tick(&self, ctx: &mut Context) {
        let n = self.0.fetch_add(1, Ordering::SeqCst);
        if n >= 2 {
            ctx.shutdown(ShutdownRequest::Success);
        }
    }
}

struct BankCreateCounter(Arc<AtomicUsize>);
impl Observer for BankCreateCounter {
    fn on_bank_create(&self, _ctx: &mut Context, _action: &cortex::bank::CreateBankAction) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

#[test]
fn shutdown_returns_reason_instead_of_exiting() {
    let mut engine = cortex::new()
        .layer(ShutdownLayer::new(ShutdownRequest::Success))
        .build();

    match engine.run() {
        Shutdown::Requested(r) => {
            assert_eq!(r, ShutdownRequest::Success);
            assert_eq!(r.code(), 0);
        }
        other => panic!("expected Requested(Success), got {:?}", other),
    }
}

#[test]
fn shutdown_preserves_internal_error_payload() {
    let a = ShutdownRequest::InternalError("db down".into());
    let mut engine = cortex::new().layer(ShutdownLayer::new(a.clone())).build();

    match engine.run() {
        Shutdown::Requested(r) => assert_eq!(r, a),
        other => panic!("expected Requested(InternalError), got {:?}", other),
    }
}

#[test]
fn manual_step_does_not_clone_world_until_snapshot() {
    let mut engine = cortex::new().layer(NoopLayer).build();
    assert!(engine.next().is_none());
    let snap = engine.snapshot();
    assert_eq!(snap.tick, engine.tick());
}

#[test]
fn manual_clock_advances_between_ticks() {
    let start = chrono::DateTime::UNIX_EPOCH;
    let mut engine = cortex::new()
        .layer(CountingLayer::new())
        .clock(ManualClock::new(start))
        .rate(TickRate::Interval(Duration::from_secs(1)))
        .build();

    match engine.run() {
        Shutdown::Requested(r) => assert_eq!(r, ShutdownRequest::Success),
        other => panic!("expected Requested(Success), got {:?}", other),
    }

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
    let count = Arc::new(AtomicUsize::new(0));
    let mut engine = cortex::new()
        .layer(SeedBankLayer::new())
        .layer(ShutdownLayer::new(ShutdownRequest::Success))
        .observer(BankCreateCounter(count.clone()))
        .build();

    match engine.run() {
        Shutdown::Requested(r) => assert_eq!(r, ShutdownRequest::Success),
        other => panic!("expected Requested(Success), got {:?}", other),
    }
    assert_eq!(count.load(Ordering::SeqCst), 1);
}

#[test]
fn next_returns_shutdown_command() {
    let mut engine = cortex::new()
        .layer(ShutdownLayer::new(ShutdownRequest::Success))
        .build();

    match engine.next() {
        Some(Command::Shutdown(r)) => assert_eq!(r, ShutdownRequest::Success),
        other => panic!("expected Some(Command::Shutdown), got {:?}", other),
    }
}
