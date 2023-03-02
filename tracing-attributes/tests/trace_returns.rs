use std::convert::TryFrom;
use std::num::TryFromIntError;
use tracing_mock::*;

use tracing::{collect::with_default, Level};
use tracing_attributes::trace_return;
use tracing_subscriber::subscribe::CollectExt;
use tracing_subscriber::EnvFilter;

#[trace_return(ret)]
fn ret() -> i32 {
    42
}

#[test]
fn test_trace_return() {
    let (collector, handle) = collector::mock()
        .event(
            expect::event()
                .with_fields(expect::field("return").with_value(&tracing::field::debug(42)))
                .at_level(Level::INFO),
        )
        .only()
        .run_with_handle();

    with_default(collector, ret);
    handle.assert_finished();
}

#[trace_return(ret(level = "warn"))]
fn ret_warn() -> i32 {
    42
}

#[test]
fn test_trace_return_warn() {
    let (collector, handle) = collector::mock()
        .event(
            expect::event()
                .with_fields(expect::field("return").with_value(&tracing::field::debug(42)))
                .at_level(Level::WARN),
        )
        .only()
        .run_with_handle();

    with_default(collector, ret_warn);
    handle.assert_finished();
}

#[trace_return(ret(Display))]
fn ret_display() -> i32 {
    42
}

#[test]
fn test_trace_return_display() {
    let (collector, handle) = collector::mock()
        .event(
            expect::event()
                .with_fields(expect::field("return").with_value(&tracing::field::display(42)))
                .at_level(Level::INFO),
        )
        .only()
        .run_with_handle();

    with_default(collector, ret_display);
    handle.assert_finished();
}

#[trace_return(ret)]
async fn ret_async() -> i32 {
    42
}

#[test]
fn test_trace_return_ret_async() {
    let (collector, handle) = collector::mock()
        .event(
            expect::event()
                .with_fields(expect::field("return").with_value(&tracing::field::debug(42)))
                .at_level(Level::INFO),
        )
        .only()
        .run_with_handle();

    with_default(collector, || block_on_future(async { ret_async().await }));
    handle.assert_finished();
}

#[trace_return(err)]
fn err() -> Result<u8, TryFromIntError> {
    u8::try_from(1234)
}

#[test]
fn test_trace_return_err() {
    let (collector, handle) = collector::mock()
        .event(expect::event().at_level(Level::ERROR))
        .only()
        .run_with_handle();

    with_default(collector, || err().ok());
    handle.assert_finished();
}

#[trace_return(err, ret)]
fn ret_and_ok() -> Result<u8, TryFromIntError> {
    u8::try_from(42)
}

#[test]
fn test_trace_return_ret_and_ok() {
    let (collector, handle) = collector::mock()
        .event(
            expect::event()
                .with_fields(expect::field("return").with_value(&tracing::field::display(42)))
                .at_level(Level::INFO),
        )
        .only()
        .run_with_handle();

    with_default(collector, || ret_and_ok().ok());
    handle.assert_finished();
}

#[trace_return(err)]
fn err_early_ret() -> Result<u8, TryFromIntError> {
    u8::try_from(1234)?;
    Ok(42)
}

#[test]
fn test_trace_return_err_early_ret() {
    let (collector, handle) = collector::mock()
        .event(expect::event().at_level(Level::ERROR))
        .only()
        .run_with_handle();

    with_default(collector, || err_early_ret().ok());
    handle.assert_finished();
}

#[trace_return(err)]
async fn err_async() -> Result<u8, TryFromIntError> {
    u8::try_from(1234)
}

#[test]
fn test_trace_return_err_async() {
    let (collector, handle) = collector::mock()
        .event(expect::event().at_level(Level::ERROR))
        .only()
        .run_with_handle();

    with_default(collector, || {
        block_on_future(async { err_async().await }).ok()
    });
    handle.assert_finished();
}

#[trace_return(err(level = "info"))]
fn err_info() -> Result<u8, TryFromIntError> {
    u8::try_from(1234)
}

#[test]
fn test_trace_return_err_info() {
    let (collector, handle) = collector::mock()
        .event(expect::event().at_level(Level::INFO))
        .only()
        .run_with_handle();

    with_default(collector, || err_info().ok());
    handle.assert_finished();
}
