use crate::control::{CodeExecutionFailure, CodeExecutionResult, CodeExecutionSuccess};
use crate::control::GCode;
use crate::hwa;
use crate::math;
use crate::machine::MACHINE_INFO;

#[cfg(feature = "with-probe")]
use hwa::controllers::ProbeTrait;
use alloc::format;
#[cfg(feature = "with-motion")]
use hwa::{DeferEvent, DeferAction};
use hwa::{CommChannel, EventBusRef, EventFlags, EventStatus};
use math::Real;
use strum::VariantNames;
#[cfg(feature = "with-motion")]
use crate::tgeo::TVector;

pub struct GCodeProcessorParams {
    pub event_bus: EventBusRef,

    #[cfg(feature = "with-motion")]
    pub motion_planner: hwa::controllers::MotionPlannerRef,
    #[cfg(feature = "with-serial-usb")]
    pub serial_usb_tx: hwa::device::USBSerialTxControllerRef,
    #[cfg(feature = "with-serial-port-1")]
    pub serial_port1_tx: hwa::device::UartPort1TxControllerRef,
    #[cfg(feature = "with-serial-port-2")]
    pub serial_port2_tx: hwa::device::UartPort2TxControllerRef,
    #[cfg(feature = "with-probe")]
    pub probe: hwa::controllers::ServoControllerRef,
    #[cfg(feature = "with-hot-end")]
    pub hotend: hwa::controllers::HotendControllerRef,
    #[cfg(feature = "with-hot-bed")]
    pub hotbed: hwa::controllers::HotbedControllerRef,
    #[cfg(feature = "with-fan-layer")]
    pub fan_layer: hwa::controllers::FanLayerPwmControllerRef,
    #[cfg(feature = "with-fan-extra-1")]
    pub fan_extra_1: hwa::controllers::FanExtra1PwmControllerRef,
    #[cfg(feature = "with-laser")]
    pub laser: hwa::controllers::LaserPwmControllerRef,
}

#[derive(Clone)]
pub struct GCodeProcessor {
    pub event_bus: EventBusRef,

    #[cfg(feature = "with-motion")]
    pub motion_planner: hwa::controllers::MotionPlannerRef,
    #[cfg(feature = "with-serial-usb")]
    pub serial_usb_tx: hwa::device::USBSerialTxControllerRef,
    #[cfg(feature = "with-serial-port-1")]
    pub serial_port1_tx: hwa::device::UartPort1TxControllerRef,
    #[cfg(feature = "with-serial-port-2")]
    pub serial_port2_tx: hwa::device::UartPort2TxControllerRef,
    #[cfg(feature = "with-probe")]
    pub probe: hwa::controllers::ServoControllerRef,
    #[cfg(feature = "with-hot-end")]
    pub hotend: hwa::controllers::HotendControllerRef,
    #[cfg(feature = "with-hot-bed")]
    pub hotbed: hwa::controllers::HotbedControllerRef,
    #[cfg(feature = "with-fan-layer")]
    pub fan_layer: hwa::controllers::FanLayerPwmControllerRef,
    #[cfg(feature = "with-fan-extra-1")]
    pub fan_extra_1: hwa::controllers::FanExtra1PwmControllerRef,
    #[cfg(feature = "with-laser")]
    pub laser: hwa::controllers::LaserPwmControllerRef,
}

impl GCodeProcessor {
    pub fn new(params: GCodeProcessorParams) -> Self {
        Self {
            #[cfg(feature = "with-serial-usb")]
            serial_usb_tx: params.serial_usb_tx,
            #[cfg(feature = "with-serial-port-1")]
            serial_port1_tx: params.serial_port1_tx,
            #[cfg(feature = "with-serial-port-2")]
            serial_port2_tx: params.serial_port2_tx,
            #[cfg(feature = "with-motion")]
            motion_planner: params.motion_planner,
            #[cfg(feature = "with-probe")]
            probe: params.probe,
            #[cfg(feature = "with-hot-end")]
            hotend: params.hotend,
            #[cfg(feature = "with-hot-bed")]
            hotbed: params.hotbed,
            #[cfg(feature = "with-fan-layer")]
            fan_layer: params.fan_layer,
            #[cfg(feature = "with-fan-extra-1")]
            fan_extra_1: params.fan_extra_1,
            #[cfg(feature = "with-laser")]
            laser: params.laser,

            event_bus: params.event_bus,
        }
    }

    pub(crate) async fn write(&self, channel: CommChannel, _msg: &str) {
        match channel {
            #[cfg(feature = "with-serial-usb")]
            CommChannel::SerialUsb => {
                let _ = self.serial_usb_tx.lock().await.write_packet(_msg.as_bytes()).await;
            }
            #[cfg(feature = "with-serial-port-1")]
            CommChannel::SerialPort1 => {
                let _ = self.serial_port1_tx.lock().await.write(_msg.as_bytes()).await;
            }
            #[cfg(feature = "with-serial-port-2")]
            CommChannel::SerialPort2 => {
                let _ = self.serial_port2_tx.lock().await.write(_msg.as_bytes()).await;
            }
            CommChannel::Internal => {}
        }
    }

    #[allow(unused)]
    pub(crate) async fn flush(&self, channel: CommChannel) {
        match channel {
            #[cfg(feature = "with-serial-usb")]
            CommChannel::SerialUsb => {
                let _ = self.serial_usb_tx.lock().await.write_packet(b"").await;
            }
            #[cfg(feature = "with-serial-port-1")]
            CommChannel::SerialPort1 => {
                let _ = self.serial_port1_tx.lock().await.blocking_flush();
            }
            #[cfg(feature = "with-serial-port-2")]
            CommChannel::SerialPort2 => {
                let _ = self.serial_port2_tx.lock().await.blocking_flush();
            }
            CommChannel::Internal => {}
        }
    }

    #[allow(unused)]
    pub(crate) async fn writeln(&self, channel: CommChannel, msg: &str) {
        let _ = self.write(channel, msg).await;
        let _ = self.write(channel, "\n").await;
    }

    /***

    */
    #[allow(unused)]
    pub(crate) async fn execute(&mut self, channel: CommChannel, gc: &GCode, blocking: bool) -> CodeExecutionResult {
        let result = match gc {
            #[cfg(feature = "grbl-compat")]
            GCode::GRBLCMD => {
                let str = format!(
                    "[VER:1.1 {} v{}:]\n[MSG: Machine: {} {}]\n",
                    MACHINE_INFO.firmware_name,
                    MACHINE_INFO.firmware_version,
                    MACHINE_INFO.machine_board,
                    MACHINE_INFO.machine_type,
                );
                self.write(str.as_str()).await;
                Ok(CodeExecutionSuccess::OK)
            }
            GCode::G => {
                for x in GCode::VARIANTS.iter().filter(|x| x.starts_with("G")) {
                    let _ = self.write(channel, "echo: ").await;
                    let _ = self.writeln(channel, x).await;
                }
                Ok(CodeExecutionSuccess::OK)
            }
            #[cfg(feature = "with-motion")]
            GCode::G0(_) | GCode::G1(_) => {
                if !self.event_bus.get_status().await.contains(EventFlags::ATX_ON) {
                    return Err(CodeExecutionFailure::PowerRequired)
                }
                Ok(self.motion_planner.plan(channel, &gc, blocking).await?)
            }
            #[cfg(feature = "with-motion")]
            GCode::G4 => {
                if !blocking {
                    self.motion_planner
                        .defer_channel
                        .send(DeferEvent::AwaitRequested(DeferAction::Dwell, channel))
                        .await;
                }
                Ok(self.motion_planner.plan(channel, &gc, blocking).await?)
            }
            GCode::G10 => Ok(CodeExecutionSuccess::OK),
            GCode::G17 => Ok(CodeExecutionSuccess::OK),
            GCode::G21 => Ok(CodeExecutionSuccess::OK),
            #[cfg(feature = "with-motion")]
            GCode::G28(_) => match self.event_bus.has_flags(EventFlags::HOMMING).await {
                true => Err(CodeExecutionFailure::BUSY),
                false => {
                    if !self.event_bus.get_status().await.contains(EventFlags::ATX_ON) {
                        return Err(CodeExecutionFailure::PowerRequired)
                    }
                    hwa::debug!("Planing homing");
                    let result = self.motion_planner.plan(channel, &gc, blocking).await;
                    hwa::debug!("Homing planned");
                    result
                }
            },
            #[cfg(feature = "with-probe")]
            GCode::G31 => {
                if self.event_bus.has_flags(EventFlags::HOMMING).await {
                    Err(CodeExecutionFailure::BUSY)
                } else {
                    if !self.event_bus.get_status().await.contains(EventFlags::ATX_ON) {
                        return Err(CodeExecutionFailure::PowerRequired)
                    }
                    let md = self.motion_planner.motion_driver.lock().await;
                    md.probe_controller.lock().await.probe_pin_up(300_000).await;
                    Ok(CodeExecutionSuccess::OK)
                }
            }
            #[cfg(feature = "with-probe")]
            GCode::G32 => {
                if self.event_bus.has_flags(EventFlags::HOMMING).await {
                    Err(CodeExecutionFailure::BUSY)
                } else {
                    if !self.event_bus.get_status().await.contains(EventFlags::ATX_ON) {
                        return Err(CodeExecutionFailure::PowerRequired)
                    }
                    let md = self.motion_planner.motion_driver.lock().await;
                    md.probe_controller
                        .lock()
                        .await
                        .probe_pin_down(300_000)
                        .await;
                    Ok(CodeExecutionSuccess::OK)
                }
            }
            GCode::G80 => Ok(CodeExecutionSuccess::OK),
            #[cfg(feature = "with-motion")]
            GCode::G90 => {
                self.motion_planner.set_absolute_positioning(true).await;
                Ok(CodeExecutionSuccess::CONSUMED)
            }
            #[cfg(feature = "with-motion")]
            GCode::G91 => {
                self.motion_planner.set_absolute_positioning(false).await;
                Ok(CodeExecutionSuccess::CONSUMED)
            }
            #[cfg(feature = "with-motion")]
            GCode::G92 => {
                // FIXME
                self.motion_planner.set_last_planned_pos(&TVector::zero()).await;
                Ok(CodeExecutionSuccess::CONSUMED)
            },
            GCode::G94 => Ok(CodeExecutionSuccess::OK),
            GCode::M => {
                for x in GCode::VARIANTS.iter().filter(|x| x.starts_with("M")) {
                    let _ = self.write(channel,"echo: ").await;
                    let _ = self.writeln(channel, x).await;
                }
                Ok(CodeExecutionSuccess::OK)
            }
            GCode::M3 => Ok(CodeExecutionSuccess::OK),
            GCode::M5 => Ok(CodeExecutionSuccess::OK),
            GCode::M73 => Ok(CodeExecutionSuccess::OK),
            GCode::M79 => {
                let _ = self.write(channel, "echo: Software reset\n").await;
                self.flush(channel).await;
                hwa::sys_reset();
                Ok(CodeExecutionSuccess::OK)
            }
            GCode::M80 => {
                // TODO set output
                self.event_bus
                    .publish_event(EventStatus::containing(EventFlags::ATX_ON))
                    .await;
                Ok(CodeExecutionSuccess::OK)
            }
            GCode::M81 => {
                // TODO set output
                self.event_bus
                    .publish_event(EventStatus::not_containing(EventFlags::ATX_ON))
                    .await;
                Ok(CodeExecutionSuccess::OK)
            }
            GCode::M83 => Ok(CodeExecutionSuccess::OK),
            GCode::M84 => Ok(CodeExecutionSuccess::OK),
            GCode::M100 => {
                let heap_current = hwa::mem::heap_current_size();
                let heap_max = hwa::mem::heap_max_size();
                let stack_current = hwa::mem::stack_reservation_current_size();
                let stack_max = hwa::mem::stack_reservation_max_size();

                let z1 = format!(
                    "echo: {}/{} {}% of heap usage\n",
                    heap_current,
                    heap_max,
                    Real::from_f32(100.0f32 * heap_current as f32 / heap_max as f32).rdp(4),
                );
                self.write(channel, z1.as_str()).await;

                let z2 = format!(
                    "echo: {}/{} {}% of stack reservation\n",
                    stack_current,
                    stack_max,
                    Real::from_f32(100.0f32 * stack_current as f32 / stack_max as f32).rdp(4),
                );
                self.write(channel, z2.as_str()).await;
                Ok(CodeExecutionSuccess::OK)
            }
            // Set hotend temperature
            // An immediate command
            #[cfg(feature = "with-hot-end")]
            GCode::M104(s) => {
                if !self.event_bus.get_status().await.contains(EventFlags::ATX_ON) {
                    return Err(CodeExecutionFailure::PowerRequired)
                }
                let val = s.s.and_then(|v| v.to_i32()).unwrap_or(0);
                let mut h = self.hotend.lock().await;
                h.set_target_temp(CommChannel::Internal, DeferAction::HotendTemperature, val as f32).await;
                Ok(CodeExecutionSuccess::OK)
            }
            GCode::M105 => {
                let hotend_temp_report = {
                    #[cfg(feature = "with-hot-end")]
                    {
                        let mut h = self.hotend.lock().await;
                        format!(
                            " T:{} /{} T@:{} TZ:{}",
                            h.get_current_temp(),
                            h.get_target_temp(),
                            h.get_current_power().await,
                            h.get_current_resistance(),
                        )
                    }
                    #[cfg(not(feature = "with-hot-end"))]
                    alloc::string::String::new()
                };
                let hotbed_temp_report = {
                    #[cfg(feature = "with-hot-bed")]
                    {
                        let mut h = self.hotbed.lock().await;
                        format!(
                            " B:{} /{} B@:{} BZ:{}",
                            h.get_current_temp(),
                            h.get_target_temp(),
                            h.get_current_power().await,
                            h.get_current_resistance(),
                        )
                    }
                    #[cfg(not(feature = "with-hot-bed"))]
                    alloc::string::String::new()
                };
                let report = format!("ok{}{}\n", hotend_temp_report, hotbed_temp_report);
                let _ = self.write(channel, report.as_str()).await;
                Ok(CodeExecutionSuccess::CONSUMED)
            }
            #[cfg(feature = "with-fan-layer")]
            GCode::M106 => {
                if !self.event_bus.get_status().await.contains(EventFlags::ATX_ON) {
                    return Err(CodeExecutionFailure::PowerRequired)
                }
                //crate::info!("M106 BEGIN");
                self.fan_layer.lock().await.set_power(255).await;
                //crate::info!("M106 END");
                Ok(CodeExecutionSuccess::OK)
            }
            #[cfg(feature = "with-fan-layer")]
            GCode::M107 => {
                self.fan_layer.lock().await.set_power(0).await;
                Ok(CodeExecutionSuccess::OK)
            }
            // Wait for hot-end temperature
            // Mostly deferred code
            #[cfg(feature = "with-hot-end")]
            GCode::M109(s) => {
                if !self.event_bus.get_status().await.contains(EventFlags::ATX_ON) {
                    return Err(CodeExecutionFailure::PowerRequired)
                }
                let deferred = {
                    let mut he = self.hotend.lock().await;
                    let value = s.s.and_then(|v| v.to_i32()).unwrap_or(0);
                    let was_deferred = he.set_target_temp(channel, DeferAction::HotendTemperature, value as f32).await;
                    value > 0 && was_deferred
                };
                if deferred {
                    Ok(CodeExecutionSuccess::DEFERRED(EventStatus::containing(
                        EventFlags::HOTEND_TEMP_OK,
                    )))
                } else {
                    Ok(CodeExecutionSuccess::OK)
                }
            }
            #[cfg(feature = "with-motion")]
            GCode::M114 => {
                let _pos = self.motion_planner.get_last_planned_pos().await
                    .unwrap_or(TVector::zero())
                    .rdp(6);
                let _spos = self.motion_planner.get_last_planned_step_pos().await.unwrap_or(TVector::zero());
                let z = format!("X:{} Y:{} Z:{} E:{} Count X:{} Y:{} Z:{}\n",
                                _pos.x.unwrap_or(crate::math::ZERO),
                                _pos.y.unwrap_or(crate::math::ZERO),
                                _pos.z.unwrap_or(crate::math::ZERO),
                                _pos.e.unwrap_or(crate::math::ZERO),
                                _spos.x.unwrap_or(0),
                                _spos.y.unwrap_or(0),
                                _spos.z.unwrap_or(0),
                );
                let _ = self.write(channel, z.as_str()).await;
                Ok(CodeExecutionSuccess::OK)
            }
            GCode::M115 => {
                let _ = self.write(channel, "C; FIRMWARE_NAME: ").await;
                let _ = self.write(channel, MACHINE_INFO.firmware_name).await;
                let _ = self.write(channel, " FIRMWARE_VERSION: ").await;
                let _ = self.write(channel, MACHINE_INFO.firmware_version).await;
                let _ = self.write(channel," FIRMWARE_URL: ").await;
                let _ = self.write(channel, MACHINE_INFO.firmware_url).await;
                let _ = self.write(channel, " MACHINE_TYPE: ").await;
                let _ = self.write(channel, MACHINE_INFO.machine_type).await;
                let _ = self.write(channel," MACHINE_BOARD: ").await;
                let _ = self.write(channel, MACHINE_INFO.machine_board).await;
                let _ = self.write(channel, " MACHINE_PROCESSOR: ").await;
                let _ = self.write(channel, MACHINE_INFO.machine_processor).await;
                let _ = self.write(channel, " MACHINE_UUID: ").await;
                let _ = self.write(channel, MACHINE_INFO.machine_uuid).await;
                let _ = self.write(channel," EXTRUDER_COUNT: ").await;
                let _ = self
                    .write(channel, format!("{}\n", MACHINE_INFO.extruder_count).as_str())
                    .await;
                Ok(CodeExecutionSuccess::OK)
            }
            GCode::M117 => {
                let _ = self.write(channel, "O.K.; (display msg)\n").await;
                Ok(CodeExecutionSuccess::OK)
            }
            #[cfg(feature = "with-motion")]
            GCode::M119 => {
                if !self.event_bus.get_status().await.contains(EventFlags::ATX_ON) {
                    return Err(CodeExecutionFailure::PowerRequired)
                }
                let mut d = self.motion_planner.motion_driver.lock().await;
                let z = format!(
                    "M119 X {} Y {} Z {}\n",
                    if d.pins.x_endstop_pin.is_high() {
                        "1"
                    } else {
                        "0"
                    },
                    if d.pins.y_endstop_pin.is_high() {
                        "1"
                    } else {
                        "0"
                    },
                    if d.pins.z_endstop_pin.is_high() {
                        "1"
                    } else {
                        "0"
                    },
                );
                drop(d);
                let _ = self.write(channel, z.as_str()).await;
                Ok(CodeExecutionSuccess::OK)
            }
            // Set hot-bed temperature
            // An immediate command
            #[cfg(feature = "with-hot-bed")]
            GCode::M140(s) => {
                if !self.event_bus.get_status().await.contains(EventFlags::ATX_ON) {
                    return Err(CodeExecutionFailure::PowerRequired)
                }
                let val = s.s.and_then(|v| v.to_i32()).unwrap_or(0);
                let mut h = self.hotbed.lock().await;
                h.set_target_temp(CommChannel::Internal, DeferAction::HotbedTemperature, val as f32).await;
                Ok(CodeExecutionSuccess::OK)
            }
            // Wait for hot-bed temperature
            // A normally deferred command
            #[cfg(feature = "with-hot-bed")]
            GCode::M190 => {
                if !self.event_bus.get_status().await.contains(EventFlags::ATX_ON) {
                    return Err(CodeExecutionFailure::PowerRequired)
                }
                let deferred = {
                    let mut he = self.hotbed.lock().await;
                    he.ping_subscribe(channel, DeferAction::HotbedTemperature).await
                };
                if deferred {
                    Ok(CodeExecutionSuccess::DEFERRED(EventStatus::containing(
                        EventFlags::HOTBED_TEMP_OK,
                    )))
                } else {
                    Ok(CodeExecutionSuccess::OK)
                }
            }
            GCode::M201 => Ok(CodeExecutionSuccess::OK),
            GCode::M203 => Ok(CodeExecutionSuccess::OK),
            GCode::M204 => Ok(CodeExecutionSuccess::OK),
            GCode::M205 => Ok(CodeExecutionSuccess::OK),
            GCode::M220(_) => Ok(CodeExecutionSuccess::OK),
            GCode::M221(_) => Ok(CodeExecutionSuccess::OK),
            #[cfg(feature = "with-trinamic")]
            GCode::M502 => {
                if !self.event_bus.get_status().await.contains(EventFlags::ATX_ON) {
                    return Err(CodeExecutionFailure::PowerRequired)
                }
                let success = {
                    self.motion_planner.motion_driver.lock().await
                        .trinamic_controller.init().await.is_ok()
                };
                if success {
                    let _ = self.write(channel, "ok; M502\n").await;
                } else {
                    let _ = self.write(channel, "error; M502 (internal error)\n").await;
                }
                Ok(CodeExecutionSuccess::OK)
            }
            GCode::M862_1 => Ok(CodeExecutionSuccess::OK),
            GCode::M862_3 => Ok(CodeExecutionSuccess::OK),
            GCode::M900 => Ok(CodeExecutionSuccess::OK),
            GCode::M907 => Ok(CodeExecutionSuccess::OK),
            _ => Err(CodeExecutionFailure::NotYetImplemented),
        };
        result
    }
}

impl Drop for GCodeProcessor {
    fn drop(&mut self) {}
}