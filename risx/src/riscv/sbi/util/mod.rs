use crate::riscv::sbi::{
    base::{Extension, probe_extension},
    debug_console::write,
};

/// Check and print which SBI extensions are supported.
pub fn extension_check() {
    write("BASE: ").unwrap();
    if probe_extension(Extension::Base).unwrap() {
        write("Y\n").unwrap()
    } else {
        write("N\n").unwrap()
    }

    write("TIME: ").unwrap();
    if probe_extension(Extension::Timer).unwrap() {
        write("Y\n").unwrap()
    } else {
        write("N\n").unwrap()
    }

    write("IPI: ").unwrap();
    if probe_extension(Extension::InterProcessorInterrupt).unwrap() {
        write("Y\n").unwrap()
    } else {
        write("N\n").unwrap()
    }

    write("RFENCE: ").unwrap();
    if probe_extension(Extension::RemoteFence).unwrap() {
        write("Y\n").unwrap()
    } else {
        write("N\n").unwrap()
    }

    write("HSM: ").unwrap();
    if probe_extension(Extension::HartStateManagement).unwrap() {
        write("Y\n").unwrap()
    } else {
        write("N\n").unwrap()
    }

    write("SRST: ").unwrap();
    if probe_extension(Extension::SystemReset).unwrap() {
        write("Y\n").unwrap()
    } else {
        write("N\n").unwrap()
    }

    write("PMU: ").unwrap();
    if probe_extension(Extension::PerformanceMonitoringUnit).unwrap() {
        write("Y\n").unwrap()
    } else {
        write("N\n").unwrap()
    }

    write("CDBN: ").unwrap();
    if probe_extension(Extension::ConsoleDebug).unwrap() {
        write("Y\n").unwrap()
    } else {
        write("N\n").unwrap()
    }

    write("SUSP: ").unwrap();
    if probe_extension(Extension::SystemSuspend).unwrap() {
        write("Y\n").unwrap()
    } else {
        write("N\n").unwrap()
    }

    write("CPPC: ").unwrap();
    if probe_extension(Extension::CollaborativeProcessorPerformanceControl).unwrap() {
        write("Y\n").unwrap()
    } else {
        write("N\n").unwrap()
    }

    write("NACL: ").unwrap();
    if probe_extension(Extension::NestedAcceleration).unwrap() {
        write("Y\n").unwrap()
    } else {
        write("N\n").unwrap()
    }

    write("STA: ").unwrap();
    if probe_extension(Extension::StealTimeAccounting).unwrap() {
        write("Y\n").unwrap()
    } else {
        write("N\n").unwrap()
    }
}
