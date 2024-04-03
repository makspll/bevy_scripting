#![allow(clippy::all, unused_imports, deprecated, dead_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
use super::bevy_ecs::*;
use super::bevy_reflect::*;
extern crate self as bevy_script_api;
use bevy_script_api::{
    lua::RegisterForeignLuaType, ReflectedValue, common::bevy::GetWorld,
};
/// The fixed timestep game clock following virtual time.
/// A specialization of the [`Time`] structure. **For method documentation, see
/// [`Time<Fixed>#impl-Time<Fixed>`].**
///
/// It is automatically inserted as a resource by
/// [`TimePlugin`](crate::TimePlugin) and updated based on
/// [`Time<Virtual>`](Virtual). The fixed clock is automatically set as the
/// generic [`Time`] resource during [`FixedUpdate`](bevy_app::FixedUpdate)
/// schedule processing.
/// The fixed timestep clock advances in fixed-size increments, which is
/// extremely useful for writing logic (like physics) that should have
/// consistent behavior, regardless of framerate.
/// The default [`timestep()`](Time::timestep) is 64 hertz, or 15625
/// microseconds. This value was chosen because using 60 hertz has the potential
/// for a pathological interaction with the monitor refresh rate where the game
/// alternates between running two fixed timesteps and zero fixed timesteps per
/// frame (for example when running two fixed timesteps takes longer than a
/// frame). Additionally, the value is a power of two which losslessly converts
/// into [`f32`] and [`f64`].
/// To run a system on a fixed timestep, add it to one of the [`FixedMain`]
/// schedules, most commonly [`FixedUpdate`](bevy_app::FixedUpdate).
/// This schedule is run a number of times between
/// [`PreUpdate`](bevy_app::PreUpdate) and [`Update`](bevy_app::Update)
/// according to the accumulated [`overstep()`](Time::overstep) time divided by
/// the [`timestep()`](Time::timestep). This means the schedule may run 0, 1 or
/// more times during a single update (which typically corresponds to a rendered
/// frame).
/// `Time<Fixed>` and the generic [`Time`] resource will report a
/// [`delta()`](Time::delta) equal to [`timestep()`](Time::timestep) and always
/// grow [`elapsed()`](Time::elapsed) by one [`timestep()`](Time::timestep) per
/// iteration.
/// The fixed timestep clock follows the [`Time<Virtual>`](Virtual) clock, which
/// means it is affected by [`pause()`](Time::pause),
/// [`set_relative_speed()`](Time::set_relative_speed) and
/// [`set_max_delta()`](Time::set_max_delta) from virtual time. If the virtual
/// clock is paused, the [`FixedUpdate`](bevy_app::FixedUpdate) schedule will
/// not run. It is guaranteed that the [`elapsed()`](Time::elapsed) time in
/// `Time<Fixed>` is always between the previous `elapsed()` and the current
/// `elapsed()` value in `Time<Virtual>`, so the values are compatible.
/// Changing the timestep size while the game is running should not normally be
/// done, as having a regular interval is the point of this schedule, but it may
/// be necessary for effects like "bullet-time" if the normal granularity of the
/// fixed timestep is too big for the slowed down time. In this case,
/// [`set_timestep()`](Time::set_timestep) and be called to set a new value. The
/// new value will be used immediately for the next run of the
/// [`FixedUpdate`](bevy_app::FixedUpdate) schedule, meaning that it will affect
/// the [`delta()`](Time::delta) value for the very next
/// [`FixedUpdate`](bevy_app::FixedUpdate), even if it is still during the same
/// frame. Any [`overstep()`](Time::overstep) present in the accumulator will be
/// processed according to the new [`timestep()`](Time::timestep) value.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone),
    remote = "bevy::time::prelude::Fixed",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::time::prelude::Fixed;

"#,
    r#"
#[lua(kind="MetaMethod", metamethod="ToString")]
fn index(&self) -> String {
    format!("{:?}", _self)
}
"#]
)]
pub struct Fixed {}
/// Real time clock representing elapsed wall clock time.
/// A specialization of the [`Time`] structure. **For method documentation, see
/// [`Time<Real>#impl-Time<Real>`].**
/// It is automatically inserted as a resource by
/// [`TimePlugin`](crate::TimePlugin) and updated with time instants according
/// to [`TimeUpdateStrategy`](crate::TimeUpdateStrategy).
/// The [`delta()`](Time::delta) and [`elapsed()`](Time::elapsed) values of this
/// clock should be used for anything which deals specifically with real time
/// (wall clock time). It will not be affected by relative game speed
/// adjustments, pausing or other adjustments.
/// The clock does not count time from [`startup()`](Time::startup) to
/// [`first_update()`](Time::first_update()) into elapsed, but instead will
/// start counting time from the first update call. [`delta()`](Time::delta) and
/// [`elapsed()`](Time::elapsed) will report zero on the first update as there
/// is no previous update instant. This means that a [`delta()`](Time::delta) of
/// zero must be handled without errors in application logic, as it may
/// theoretically also happen at other times.
/// [`Instant`]s for [`startup()`](Time::startup),
/// [`first_update()`](Time::first_update) and
/// [`last_update()`](Time::last_update) are recorded and accessible.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone),
    remote = "bevy::time::prelude::Real",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::time::prelude::Real;

"#,
    r#"
#[lua(kind="MetaMethod", metamethod="ToString")]
fn index(&self) -> String {
    format!("{:?}", _self)
}
"#]
)]
pub struct Real {}
/// Tracks elapsed time. Enters the finished state once `duration` is reached.
/// Non repeating timers will stop tracking and stay in the finished state until reset.
/// Repeating timers will only be in the finished state on each tick `duration` is reached or
/// exceeded, and can still be reset at any given point.
/// Paused timers will not have elapsed time increased.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone),
    remote = "bevy::time::prelude::Timer",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::time::prelude::Timer;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &timer::Timer) -> bool;

"#,
    r#"
/// Creates a new timer with a given duration.
/// See also [`Timer::from_seconds`](Timer::from_seconds).

    #[lua(kind = "Function", output(proxy))]
    fn new(
        #[proxy]
        duration: bevy::utils::Duration,
        #[proxy]
        mode: bevy::time::prelude::TimerMode,
    ) -> bevy::time::prelude::Timer;

"#,
    r#"
/// Creates a new timer with a given duration in seconds.
/// # Example
/// ```
/// # use bevy_time::*;
/// let mut timer = Timer::from_seconds(1.0, TimerMode::Once);
/// ```

    #[lua(kind = "Function", output(proxy))]
    fn from_seconds(
        duration: f32,
        #[proxy]
        mode: bevy::time::prelude::TimerMode,
    ) -> bevy::time::prelude::Timer;

"#,
    r#"
/// Returns `true` if the timer has reached its duration.
/// For repeating timers, this method behaves identically to [`Timer::just_finished`].
/// # Examples
/// ```
/// # use bevy_time::*;
/// use std::time::Duration;
/// let mut timer_once = Timer::from_seconds(1.0, TimerMode::Once);
/// timer_once.tick(Duration::from_secs_f32(1.5));
/// assert!(timer_once.finished());
/// timer_once.tick(Duration::from_secs_f32(0.5));
/// assert!(timer_once.finished());
/// let mut timer_repeating = Timer::from_seconds(1.0, TimerMode::Repeating);
/// timer_repeating.tick(Duration::from_secs_f32(1.1));
/// assert!(timer_repeating.finished());
/// timer_repeating.tick(Duration::from_secs_f32(0.8));
/// assert!(!timer_repeating.finished());
/// timer_repeating.tick(Duration::from_secs_f32(0.6));
/// assert!(timer_repeating.finished());
/// ```

    #[lua(kind = "Method")]
    fn finished(&self) -> bool;

"#,
    r#"
/// Returns `true` only on the tick the timer reached its duration.
/// # Examples
/// ```
/// # use bevy_time::*;
/// use std::time::Duration;
/// let mut timer = Timer::from_seconds(1.0, TimerMode::Once);
/// timer.tick(Duration::from_secs_f32(1.5));
/// assert!(timer.just_finished());
/// timer.tick(Duration::from_secs_f32(0.5));
/// assert!(!timer.just_finished());
/// ```

    #[lua(kind = "Method")]
    fn just_finished(&self) -> bool;

"#,
    r#"
/// Returns the time elapsed on the timer. Guaranteed to be between 0.0 and `duration`.
/// Will only equal `duration` when the timer is finished and non repeating.
/// See also [`Stopwatch::elapsed`](Stopwatch::elapsed).
/// # Examples
/// ```
/// # use bevy_time::*;
/// use std::time::Duration;
/// let mut timer = Timer::from_seconds(1.0, TimerMode::Once);
/// timer.tick(Duration::from_secs_f32(0.5));
/// assert_eq!(timer.elapsed(), Duration::from_secs_f32(0.5));
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn elapsed(&self) -> bevy::utils::Duration;

"#,
    r#"
/// Returns the time elapsed on the timer as an `f32`.
/// See also [`Timer::elapsed`](Timer::elapsed).

    #[lua(kind = "Method")]
    fn elapsed_secs(&self) -> f32;

"#,
    r#"
/// Sets the elapsed time of the timer without any other considerations.
/// See also [`Stopwatch::set`](Stopwatch::set).
/// #
/// ```
/// # use bevy_time::*;
/// use std::time::Duration;
/// let mut timer = Timer::from_seconds(1.0, TimerMode::Once);
/// timer.set_elapsed(Duration::from_secs(2));
/// assert_eq!(timer.elapsed(), Duration::from_secs(2));
/// // the timer is not finished even if the elapsed time is greater than the duration.
/// assert!(!timer.finished());
/// ```

    #[lua(kind = "MutatingMethod")]
    fn set_elapsed(&mut self, #[proxy] time: bevy::utils::Duration) -> ();

"#,
    r#"
/// Returns the duration of the timer.
/// # Examples
/// ```
/// # use bevy_time::*;
/// use std::time::Duration;
/// let timer = Timer::new(Duration::from_secs(1), TimerMode::Once);
/// assert_eq!(timer.duration(), Duration::from_secs(1));
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn duration(&self) -> bevy::utils::Duration;

"#,
    r#"
/// Sets the duration of the timer.
/// # Examples
/// ```
/// # use bevy_time::*;
/// use std::time::Duration;
/// let mut timer = Timer::from_seconds(1.5, TimerMode::Once);
/// timer.set_duration(Duration::from_secs(1));
/// assert_eq!(timer.duration(), Duration::from_secs(1));
/// ```

    #[lua(kind = "MutatingMethod")]
    fn set_duration(&mut self, #[proxy] duration: bevy::utils::Duration) -> ();

"#,
    r#"
/// Returns the mode of the timer.
/// # Examples
/// ```
/// # use bevy_time::*;
/// let mut timer = Timer::from_seconds(1.0, TimerMode::Repeating);
/// assert_eq!(timer.mode(), TimerMode::Repeating);
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn mode(&self) -> bevy::time::prelude::TimerMode;

"#,
    r#"
/// Sets the mode of the timer.
/// # Examples
/// ```
/// # use bevy_time::*;
/// let mut timer = Timer::from_seconds(1.0, TimerMode::Repeating);
/// timer.set_mode(TimerMode::Once);
/// assert_eq!(timer.mode(), TimerMode::Once);
/// ```

    #[lua(kind = "MutatingMethod")]
    fn set_mode(&mut self, #[proxy] mode: bevy::time::prelude::TimerMode) -> ();

"#,
    r#"
/// Pauses the Timer. Disables the ticking of the timer.
/// See also [`Stopwatch::pause`](Stopwatch::pause).
/// # Examples
/// ```
/// # use bevy_time::*;
/// use std::time::Duration;
/// let mut timer = Timer::from_seconds(1.0, TimerMode::Once);
/// timer.pause();
/// timer.tick(Duration::from_secs_f32(0.5));
/// assert_eq!(timer.elapsed_secs(), 0.0);
/// ```

    #[lua(kind = "MutatingMethod")]
    fn pause(&mut self) -> ();

"#,
    r#"
/// Unpauses the Timer. Resumes the ticking of the timer.
/// See also [`Stopwatch::unpause()`](Stopwatch::unpause).
/// # Examples
/// ```
/// # use bevy_time::*;
/// use std::time::Duration;
/// let mut timer = Timer::from_seconds(1.0, TimerMode::Once);
/// timer.pause();
/// timer.tick(Duration::from_secs_f32(0.5));
/// timer.unpause();
/// timer.tick(Duration::from_secs_f32(0.5));
/// assert_eq!(timer.elapsed_secs(), 0.5);
/// ```

    #[lua(kind = "MutatingMethod")]
    fn unpause(&mut self) -> ();

"#,
    r#"
/// Returns `true` if the timer is paused.
/// See also [`Stopwatch::paused`](Stopwatch::paused).
/// # Examples
/// ```
/// # use bevy_time::*;
/// let mut timer = Timer::from_seconds(1.0, TimerMode::Once);
/// assert!(!timer.paused());
/// timer.pause();
/// assert!(timer.paused());
/// timer.unpause();
/// assert!(!timer.paused());
/// ```

    #[lua(kind = "Method")]
    fn paused(&self) -> bool;

"#,
    r#"
/// Resets the timer. The reset doesn't affect the `paused` state of the timer.
/// See also [`Stopwatch::reset`](Stopwatch::reset).
/// Examples
/// ```
/// # use bevy_time::*;
/// use std::time::Duration;
/// let mut timer = Timer::from_seconds(1.0, TimerMode::Once);
/// timer.tick(Duration::from_secs_f32(1.5));
/// timer.reset();
/// assert!(!timer.finished());
/// assert!(!timer.just_finished());
/// assert_eq!(timer.elapsed_secs(), 0.0);
/// ```

    #[lua(kind = "MutatingMethod")]
    fn reset(&mut self) -> ();

"#,
    r#"
/// Returns the fraction of the timer elapsed time (goes from 0.0 to 1.0).
/// # Examples
/// ```
/// # use bevy_time::*;
/// use std::time::Duration;
/// let mut timer = Timer::from_seconds(2.0, TimerMode::Once);
/// timer.tick(Duration::from_secs_f32(0.5));
/// assert_eq!(timer.fraction(), 0.25);
/// ```

    #[lua(kind = "Method")]
    fn fraction(&self) -> f32;

"#,
    r#"
/// Returns the fraction of the timer remaining time (goes from 1.0 to 0.0).
/// # Examples
/// ```
/// # use bevy_time::*;
/// use std::time::Duration;
/// let mut timer = Timer::from_seconds(2.0, TimerMode::Once);
/// timer.tick(Duration::from_secs_f32(0.5));
/// assert_eq!(timer.fraction_remaining(), 0.75);
/// ```

    #[lua(kind = "Method")]
    fn fraction_remaining(&self) -> f32;

"#,
    r#"
/// Returns the remaining time in seconds
/// # Examples
/// ```
/// # use bevy_time::*;
/// use std::cmp::Ordering;
/// use std::time::Duration;
/// let mut timer = Timer::from_seconds(2.0, TimerMode::Once);
/// timer.tick(Duration::from_secs_f32(0.5));
/// let result = timer.remaining_secs().total_cmp(&1.5);
/// assert_eq!(Ordering::Equal, result);
/// ```

    #[lua(kind = "Method")]
    fn remaining_secs(&self) -> f32;

"#,
    r#"
/// Returns the remaining time using Duration
/// # Examples
/// ```
/// # use bevy_time::*;
/// use std::time::Duration;
/// let mut timer = Timer::from_seconds(2.0, TimerMode::Once);
/// timer.tick(Duration::from_secs_f32(0.5));
/// assert_eq!(timer.remaining(), Duration::from_secs_f32(1.5));
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn remaining(&self) -> bevy::utils::Duration;

"#,
    r#"
/// Returns the number of times a repeating timer
/// finished during the last [`tick`](Timer<T>::tick) call.
/// For non repeating-timers, this method will only ever
/// return 0 or 1.
/// # Examples
/// ```
/// # use bevy_time::*;
/// use std::time::Duration;
/// let mut timer = Timer::from_seconds(1.0, TimerMode::Repeating);
/// timer.tick(Duration::from_secs_f32(6.0));
/// assert_eq!(timer.times_finished_this_tick(), 6);
/// timer.tick(Duration::from_secs_f32(2.0));
/// assert_eq!(timer.times_finished_this_tick(), 2);
/// timer.tick(Duration::from_secs_f32(0.5));
/// assert_eq!(timer.times_finished_this_tick(), 0);
/// ```

    #[lua(kind = "Method")]
    fn times_finished_this_tick(&self) -> u32;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"
#[lua(kind="MetaMethod", metamethod="ToString")]
fn index(&self) -> String {
    format!("{:?}", _self)
}
"#]
)]
pub struct Timer {}
/// Specifies [`Timer`] behavior.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone),
    remote = "bevy::time::prelude::TimerMode",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &timer::TimerMode) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::time::prelude::TimerMode;

"#,
    r#"
#[lua(kind="MetaMethod", metamethod="ToString")]
fn index(&self) -> String {
    format!("{:?}", _self)
}
"#]
)]
pub struct TimerMode {}
/// The virtual game clock representing game time.
/// A specialization of the [`Time`] structure. **For method documentation, see
/// [`Time<Virtual>#impl-Time<Virtual>`].**
/// Normally used as `Time<Virtual>`. It is automatically inserted as a resource
/// by [`TimePlugin`](crate::TimePlugin) and updated based on
/// [`Time<Real>`](Real). The virtual clock is automatically set as the default
/// generic [`Time`] resource for the update.
/// The virtual clock differs from real time clock in that it can be paused, sped up
/// and slowed down. It also limits how much it can advance in a single update
/// in order to prevent unexpected behavior in cases where updates do not happen
/// at regular intervals (e.g. coming back after the program was suspended a long time).
/// The virtual clock can be paused by calling [`pause()`](Time::pause) and
/// unpaused by calling [`unpause()`](Time::unpause). When the game clock is
/// paused [`delta()`](Time::delta) will be zero on each update, and
/// [`elapsed()`](Time::elapsed) will not grow.
/// [`effective_speed()`](Time::effective_speed) will return `0.0`. Calling
/// [`pause()`](Time::pause) will not affect value the [`delta()`](Time::delta)
/// value for the update currently being processed.
/// The speed of the virtual clock can be changed by calling
/// [`set_relative_speed()`](Time::set_relative_speed). A value of `2.0` means
/// that virtual clock should advance twice as fast as real time, meaning that
/// [`delta()`](Time::delta) values will be double of what
/// [`Time<Real>::delta()`](Time::delta) reports and
/// [`elapsed()`](Time::elapsed) will go twice as fast as
/// [`Time<Real>::elapsed()`](Time::elapsed). Calling
/// [`set_relative_speed()`](Time::set_relative_speed) will not affect the
/// [`delta()`](Time::delta) value for the update currently being processed.
/// The maximum amount of delta time that can be added by a single update can be
/// set by [`set_max_delta()`](Time::set_max_delta). This value serves a dual
/// purpose in the virtual clock.
/// If the game temporarily freezes due to any reason, such as disk access, a
/// blocking system call, or operating system level suspend, reporting the full
/// elapsed delta time is likely to cause bugs in game logic. Usually if a
/// laptop is suspended for an hour, it doesn't make sense to try to simulate
/// the game logic for the elapsed hour when resuming. Instead it is better to
/// lose the extra time and pretend a shorter duration of time passed. Setting
/// [`max_delta()`](Time::max_delta) to a relatively short time means that the
/// impact on game logic will be minimal.
/// If the game lags for some reason, meaning that it will take a longer time to
/// compute a frame than the real time that passes during the computation, then
/// we would fall behind in processing virtual time. If this situation persists,
/// and computing a frame takes longer depending on how much virtual time has
/// passed, the game would enter a "death spiral" where computing each frame
/// takes longer and longer and the game will appear to freeze. By limiting the
/// maximum time that can be added at once, we also limit the amount of virtual
/// time the game needs to compute for each frame. This means that the game will
/// run slow, and it will run slower than real time, but it will not freeze and
/// it will recover as soon as computation becomes fast again.
/// You should set [`max_delta()`](Time::max_delta) to a value that is
/// approximately the minimum FPS your game should have even if heavily lagged
/// for a moment. The actual FPS when lagged will be somewhat lower than this,
/// depending on how much more time it takes to compute a frame compared to real
/// time. You should also consider how stable your FPS is, as the limit will
/// also dictate how big of an FPS drop you can accept without losing time and
/// falling behind real time.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone),
    remote = "bevy::time::prelude::Virtual",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::time::prelude::Virtual;

"#,
    r#"
#[lua(kind="MetaMethod", metamethod="ToString")]
fn index(&self) -> String {
    format!("{:?}", _self)
}
"#]
)]
pub struct Virtual {}
/// A Stopwatch is a struct that track elapsed time when started.
/// # Examples
/// ```
/// # use bevy_time::*;
/// use std::time::Duration;
/// let mut stopwatch = Stopwatch::new();
/// assert_eq!(stopwatch.elapsed_secs(), 0.0);
/// stopwatch.tick(Duration::from_secs_f32(1.0)); // tick one second
/// assert_eq!(stopwatch.elapsed_secs(), 1.0);
/// stopwatch.pause();
/// stopwatch.tick(Duration::from_secs_f32(1.0)); // paused stopwatches don't tick
/// assert_eq!(stopwatch.elapsed_secs(), 1.0);
/// stopwatch.reset(); // reset the stopwatch
/// assert!(stopwatch.paused());
/// assert_eq!(stopwatch.elapsed_secs(), 0.0);
/// ```
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone),
    remote = "bevy::time::Stopwatch",
    functions[r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"
/// Create a new unpaused `Stopwatch` with no elapsed time.
/// # Examples
/// ```
/// # use bevy_time::*;
/// let stopwatch = Stopwatch::new();
/// assert_eq!(stopwatch.elapsed_secs(), 0.0);
/// assert_eq!(stopwatch.paused(), false);
/// ```

    #[lua(kind = "Function", output(proxy))]
    fn new() -> bevy::time::Stopwatch;

"#,
    r#"
/// Returns the elapsed time since the last [`reset`](Stopwatch::reset)
/// of the stopwatch.
/// # Examples
/// ```
/// # use bevy_time::*;
/// use std::time::Duration;
/// let mut stopwatch = Stopwatch::new();
/// stopwatch.tick(Duration::from_secs(1));
/// assert_eq!(stopwatch.elapsed(), Duration::from_secs(1));
/// ```
/// # See Also
/// [`elapsed_secs`](Stopwatch::elapsed_secs) - if an `f32` value is desirable instead.
/// [`elapsed_secs_f64`](Stopwatch::elapsed_secs_f64) - if an `f64` is desirable instead.

    #[lua(kind = "Method", output(proxy))]
    fn elapsed(&self) -> bevy::utils::Duration;

"#,
    r#"
/// Returns the elapsed time since the last [`reset`](Stopwatch::reset)
/// of the stopwatch, in seconds.
/// # Examples
/// ```
/// # use bevy_time::*;
/// use std::time::Duration;
/// let mut stopwatch = Stopwatch::new();
/// stopwatch.tick(Duration::from_secs(1));
/// assert_eq!(stopwatch.elapsed_secs(), 1.0);
/// ```
/// # See Also
/// [`elapsed`](Stopwatch::elapsed) - if a `Duration` is desirable instead.
/// [`elapsed_secs_f64`](Stopwatch::elapsed_secs_f64) - if an `f64` is desirable instead.

    #[lua(kind = "Method")]
    fn elapsed_secs(&self) -> f32;

"#,
    r#"
/// Returns the elapsed time since the last [`reset`](Stopwatch::reset)
/// of the stopwatch, in seconds, as f64.
/// # See Also
/// [`elapsed`](Stopwatch::elapsed) - if a `Duration` is desirable instead.
/// [`elapsed_secs`](Stopwatch::elapsed_secs) - if an `f32` is desirable instead.

    #[lua(kind = "Method")]
    fn elapsed_secs_f64(&self) -> f64;

"#,
    r#"
/// Sets the elapsed time of the stopwatch.
/// # Examples
/// ```
/// # use bevy_time::*;
/// use std::time::Duration;
/// let mut stopwatch = Stopwatch::new();
/// stopwatch.set_elapsed(Duration::from_secs_f32(1.0));
/// assert_eq!(stopwatch.elapsed_secs(), 1.0);
/// ```

    #[lua(kind = "MutatingMethod")]
    fn set_elapsed(&mut self, #[proxy] time: bevy::utils::Duration) -> ();

"#,
    r#"
/// Pauses the stopwatch. Any call to [`tick`](Stopwatch::tick) while
/// paused will not have any effect on the elapsed time.
/// # Examples
/// ```
/// # use bevy_time::*;
/// use std::time::Duration;
/// let mut stopwatch = Stopwatch::new();
/// stopwatch.pause();
/// stopwatch.tick(Duration::from_secs_f32(1.5));
/// assert!(stopwatch.paused());
/// assert_eq!(stopwatch.elapsed_secs(), 0.0);
/// ```

    #[lua(kind = "MutatingMethod")]
    fn pause(&mut self) -> ();

"#,
    r#"
/// Unpauses the stopwatch. Resume the effect of ticking on elapsed time.
/// # Examples
/// ```
/// # use bevy_time::*;
/// use std::time::Duration;
/// let mut stopwatch = Stopwatch::new();
/// stopwatch.pause();
/// stopwatch.tick(Duration::from_secs_f32(1.0));
/// stopwatch.unpause();
/// stopwatch.tick(Duration::from_secs_f32(1.0));
/// assert!(!stopwatch.paused());
/// assert_eq!(stopwatch.elapsed_secs(), 1.0);
/// ```

    #[lua(kind = "MutatingMethod")]
    fn unpause(&mut self) -> ();

"#,
    r#"
/// Returns `true` if the stopwatch is paused.
/// # Examples
/// ```
/// # use bevy_time::*;
/// let mut stopwatch = Stopwatch::new();
/// assert!(!stopwatch.paused());
/// stopwatch.pause();
/// assert!(stopwatch.paused());
/// stopwatch.unpause();
/// assert!(!stopwatch.paused());
/// ```

    #[lua(kind = "Method")]
    fn paused(&self) -> bool;

"#,
    r#"
/// Resets the stopwatch. The reset doesn't affect the paused state of the stopwatch.
/// # Examples
/// ```
/// # use bevy_time::*;
/// use std::time::Duration;
/// let mut stopwatch = Stopwatch::new();
/// stopwatch.tick(Duration::from_secs_f32(1.5));
/// stopwatch.reset();
/// assert_eq!(stopwatch.elapsed_secs(), 0.0);
/// ```

    #[lua(kind = "MutatingMethod")]
    fn reset(&mut self) -> ();

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::time::Stopwatch;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &stopwatch::Stopwatch) -> bool;

"#,
    r#"
#[lua(kind="MetaMethod", metamethod="ToString")]
fn index(&self) -> String {
    format!("{:?}", _self)
}
"#]
)]
pub struct Stopwatch {}
#[derive(Default)]
pub(crate) struct Globals;
impl bevy_mod_scripting_lua::tealr::mlu::ExportInstances for Globals {
    fn add_instances<
        'lua,
        T: bevy_mod_scripting_lua::tealr::mlu::InstanceCollector<'lua>,
    >(self, instances: &mut T) -> bevy_mod_scripting_lua::tealr::mlu::mlua::Result<()> {
        instances
            .add_instance(
                "Timer",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaTimer>::new,
            )?;
        instances
            .add_instance(
                "Stopwatch",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaStopwatch>::new,
            )?;
        Ok(())
    }
}
pub struct BevyTimeAPIProvider;
impl bevy_mod_scripting_core::hosts::APIProvider for BevyTimeAPIProvider {
    type APITarget = std::sync::Mutex<bevy_mod_scripting_lua::tealr::mlu::mlua::Lua>;
    type ScriptContext = std::sync::Mutex<bevy_mod_scripting_lua::tealr::mlu::mlua::Lua>;
    type DocTarget = bevy_mod_scripting_lua::docs::LuaDocFragment;
    fn attach_api(
        &mut self,
        ctx: &mut Self::APITarget,
    ) -> Result<(), bevy_mod_scripting_core::error::ScriptError> {
        let ctx = ctx.get_mut().expect("Unable to acquire lock on Lua context");
        bevy_mod_scripting_lua::tealr::mlu::set_global_env(Globals, ctx)
            .map_err(|e| bevy_mod_scripting_core::error::ScriptError::Other(
                e.to_string(),
            ))
    }
    fn get_doc_fragment(&self) -> Option<Self::DocTarget> {
        Some(
            bevy_mod_scripting_lua::docs::LuaDocFragment::new(
                "BevyTimeAPI",
                |tw| {
                    tw.document_global_instance::<Globals>()
                        .expect("Something went wrong documenting globals")
                        .process_type::<LuaFixed>()
                        .process_type::<LuaReal>()
                        .process_type::<LuaTimer>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaTimer>,
                        >()
                        .process_type::<LuaTimerMode>()
                        .process_type::<LuaVirtual>()
                        .process_type::<LuaStopwatch>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaStopwatch,
                            >,
                        >()
                },
            ),
        )
    }
    fn setup_script(
        &mut self,
        script_data: &bevy_mod_scripting_core::hosts::ScriptData,
        ctx: &mut Self::ScriptContext,
    ) -> Result<(), bevy_mod_scripting_core::error::ScriptError> {
        Ok(())
    }
    fn setup_script_runtime(
        &mut self,
        world_ptr: bevy_mod_scripting_core::world::WorldPointer,
        _script_data: &bevy_mod_scripting_core::hosts::ScriptData,
        ctx: &mut Self::ScriptContext,
    ) -> Result<(), bevy_mod_scripting_core::error::ScriptError> {
        Ok(())
    }
    fn register_with_app(&self, app: &mut bevy::app::App) {
        app.register_foreign_lua_type::<bevy::time::prelude::Fixed>();
        app.register_foreign_lua_type::<bevy::time::prelude::Real>();
        app.register_foreign_lua_type::<bevy::time::prelude::Timer>();
        app.register_foreign_lua_type::<bevy::time::prelude::TimerMode>();
        app.register_foreign_lua_type::<bevy::time::prelude::Virtual>();
        app.register_foreign_lua_type::<bevy::time::Stopwatch>();
    }
}