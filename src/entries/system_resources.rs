//! Perf UI Entries for system resource diagnostics (entity count, CPU/memory usage)

use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, EntityCountDiagnosticsPlugin};
use bevy::ecs::system::lifetimeless::SRes;
use bevy::ecs::system::SystemParam;

#[cfg(feature = "sysinfo")]
use bevy::diagnostic::SystemInformationDiagnosticsPlugin;

use crate::prelude::*;
use crate::entry::*;
use crate::utils::*;

/// Perf UI Entry to display Bevy's built-in ECS entity counter.
#[derive(Component, Debug, Clone)]
#[require(PerfUiRoot)]
pub struct PerfUiEntryEntityCount {
    /// Custom label. If empty (default), the default label will be used.
    pub label: String,
    /// Enable color based on value.
    ///
    /// To disable (always use default color), set to empty `ColorGradient::default()`.
    ///
    /// Default: Green-Yellow-Red gradient between 100-1000-10000.
    pub color_gradient: ColorGradient,
    /// Highlight the value if above this threshold.
    ///
    /// Default: `20000`
    pub threshold_highlight: Option<u32>,
    /// If displayed using a Bar (or other similar) widget that can
    /// show the value within a range, what should its max value be?
    ///
    /// If `None`, the value will be computed from the maximum of the
    /// color gradient and the highlight threshold.
    ///
    /// Default: `None`
    pub max_value_hint: Option<u32>,
    /// Number of digits to display.
    ///
    /// Default: `6`
    pub digits: u8,
    /// Sort Key (control where the entry will appear in the Perf UI).
    pub sort_key: i32,
}

impl Default for PerfUiEntryEntityCount {
    fn default() -> Self {
        PerfUiEntryEntityCount {
            label: String::new(),
            color_gradient: ColorGradient::new_preset_gyr(100.0, 1000.0, 10000.0).unwrap(),
            threshold_highlight: Some(20000),
            max_value_hint: None,
            digits: 6,
            sort_key: next_sort_key(),
        }
    }
}

/// Perf UI Entry to display Bevy's built-in Process CPU Usage measurement diagnostic.
///
/// Displays the CPU usage of the current process (your game) as a percentage.
#[cfg(feature = "sysinfo")]
#[derive(Component, Debug, Clone)]
#[require(PerfUiRoot)]
pub struct PerfUiEntryCpuUsage {
    /// Custom label. If empty (default), the default label will be used.
    pub label: String,
    /// Enable color based on value.
    ///
    /// To disable (always use default color), set to empty `ColorGradient::default()`.
    ///
    /// Default: Green-Yellow-Red gradient between 25%-50%-75%.
    pub color_gradient: ColorGradient,
    /// Highlight the value if above this threshold.
    ///
    /// Default: 90%
    pub threshold_highlight: Option<f32>,
    /// Should we display the smoothed value or the raw value?
    ///
    /// Default: true (smoothed)
    pub smoothed: bool,
    /// Number of digits to display for the fractional (after the decimal point) part.
    ///
    /// Default: `2`
    pub precision: u8,
    /// Sort Key (control where the entry will appear in the Perf UI).
    pub sort_key: i32,
}

#[cfg(feature = "sysinfo")]
impl Default for PerfUiEntryCpuUsage {
    fn default() -> Self {
        PerfUiEntryCpuUsage {
            label: String::new(),
            color_gradient: ColorGradient::new_preset_gyr(25.0, 50.0, 75.0).unwrap(),
            threshold_highlight: Some(90.0),
            smoothed: true,
            precision: 2,
            sort_key: next_sort_key(),
        }
    }
}

/// Perf UI Entry to display Bevy's built-in System CPU Usage measurement diagnostic.
///
/// Displays the Total System CPU usage as a percentage.
#[cfg(feature = "sysinfo")]
#[derive(Component, Debug, Clone)]
#[require(PerfUiRoot)]
pub struct PerfUiEntrySystemCpuUsage {
    /// Custom label. If empty (default), the default label will be used.
    pub label: String,
    /// Enable color based on value.
    ///
    /// To disable (always use default color), set to empty `ColorGradient::default()`.
    ///
    /// Default: Green-Yellow-Red gradient between 25%-50%-75%.
    pub color_gradient: ColorGradient,
    /// Highlight the value if above this threshold.
    ///
    /// Default: 90%
    pub threshold_highlight: Option<f32>,
    /// Should we display the smoothed value or the raw value?
    ///
    /// Default: true (smoothed)
    pub smoothed: bool,
    /// Number of digits to display for the fractional (after the decimal point) part.
    ///
    /// Default: `2`
    pub precision: u8,
    /// Sort Key (control where the entry will appear in the Perf UI).
    pub sort_key: i32,
}

#[cfg(feature = "sysinfo")]
impl Default for PerfUiEntrySystemCpuUsage {
    fn default() -> Self {
        PerfUiEntrySystemCpuUsage {
            label: String::new(),
            color_gradient: ColorGradient::new_preset_gyr(25.0, 50.0, 75.0).unwrap(),
            threshold_highlight: Some(90.0),
            smoothed: true,
            precision: 2,
            sort_key: next_sort_key(),
        }
    }
}

/// Perf UI Entry to display Bevy's built-in Process Memory (RAM) Usage measurement diagnostic.
///
/// Displays the amount of RAM used by the current process (your game) in GiB.
#[cfg(feature = "sysinfo")]
#[derive(Component, Debug, Clone)]
#[require(PerfUiRoot)]
pub struct PerfUiEntryMemUsage {
    /// Custom label. If empty (default), the default label will be used.
    pub label: String,
    /// Display the unit ("GiB") alongside the number.
    ///
    /// Default: `true`
    pub display_units: bool,
    /// Enable color based on value.
    ///
    /// To disable (always use default color), set to empty `ColorGradient::default()`.
    ///
    /// Default: Green-Yellow-Red gradient between 0.5-1.0-2.0 GiB.
    pub color_gradient: ColorGradient,
    /// Highlight the value if above this threshold.
    ///
    /// Default: 3.0 GiB.
    pub threshold_highlight: Option<f32>,
    /// If displayed using a Bar (or other similar) widget that can
    /// show the value within a range, what should its max value be?
    ///
    /// If `None`, the value will be computed from the maximum of the
    /// color gradient and the highlight threshold.
    ///
    /// Default: `Some(4.0)`.
    pub max_value_hint: Option<f32>,
    /// Should we display the smoothed value or the raw value?
    ///
    /// Default: true (smoothed)
    pub smoothed: bool,
    /// Number of digits to display for the fractional (after the decimal point) part.
    ///
    /// Default: `3`
    pub precision: u8,
    /// Sort Key (control where the entry will appear in the Perf UI).
    pub sort_key: i32,
}

#[cfg(feature = "sysinfo")]
impl Default for PerfUiEntryMemUsage {
    fn default() -> Self {
        PerfUiEntryMemUsage {
            label: String::new(),
            display_units: true,
            color_gradient: ColorGradient::new_preset_gyr(0.5, 1.0, 2.0).unwrap(),
            threshold_highlight: Some(3.0),
            max_value_hint: Some(4.0),
            smoothed: true,
            precision: 3,
            sort_key: next_sort_key(),
        }
    }
}

/// Perf UI Entry to display Bevy's built-in System Memory (RAM) Usage measurement diagnostic.
///
/// Displays the Total System RAM usage as a percentage.
#[cfg(feature = "sysinfo")]
#[derive(Component, Debug, Clone)]
#[require(PerfUiRoot)]
pub struct PerfUiEntrySystemMemUsage {
    /// Custom label. If empty (default), the default label will be used.
    pub label: String,
    /// Enable color based on value.
    ///
    /// To disable (always use default color), set to empty `ColorGradient::default()`.
    ///
    /// Default: Green-Yellow-Red gradient between 25%-50%-75%.
    pub color_gradient: ColorGradient,
    /// Highlight the value if above this threshold.
    ///
    /// Default: 90%
    pub threshold_highlight: Option<f32>,
    /// Should we display the smoothed value or the raw value?
    ///
    /// Default: true (smoothed)
    pub smoothed: bool,
    /// Number of digits to display for the fractional (after the decimal point) part.
    ///
    /// Default: `2`
    pub precision: u8,
    /// Sort Key (control where the entry will appear in the Perf UI).
    pub sort_key: i32,
}

#[cfg(feature = "sysinfo")]
impl Default for PerfUiEntrySystemMemUsage {
    fn default() -> Self {
        PerfUiEntrySystemMemUsage {
            label: String::new(),
            color_gradient: ColorGradient::new_preset_gyr(25.0, 50.0, 75.0).unwrap(),
            threshold_highlight: Some(90.0),
            smoothed: true,
            precision: 2,
            sort_key: next_sort_key(),
        }
    }
}

impl PerfUiEntry for PerfUiEntryEntityCount {
    type SystemParam = SRes<DiagnosticsStore>;
    type Value = u32;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "Entity Count"
        } else {
            &self.label
        }
    }
    fn update_value(
        &self,
        diagnostics: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        Some(diagnostics.get(&EntityCountDiagnosticsPlugin::ENTITY_COUNT)?.value()? as u32)
    }
    fn format_value(
        &self,
        value: &Self::Value,
    ) -> String {
        format_pretty_int(self.digits, *value as i64)
    }
    fn value_color(
        &self,
        value: &Self::Value,
    ) -> Option<Color> {
        self.color_gradient.get_color_for_value(*value as f32)
    }
    fn value_highlight(
        &self,
        value: &Self::Value,
    ) -> bool {
        self.threshold_highlight
            .map(|t| *value > t)
            .unwrap_or(false)
    }
    fn sort_key(&self) -> i32 {
        self.sort_key
    }
}

impl PerfUiEntryDisplayRange for PerfUiEntryEntityCount {
    fn max_value_hint(&self) -> Option<Self::Value> {
        self.max_value_hint.or(
            match (self.threshold_highlight, self.color_gradient.max_stop()) {
                (Some(x), None) => Some(x),
                (None, Some((x, _))) => Some(*x as u32),
                (Some(a), Some((b, _))) => Some(a.max(*b as u32)),
                (None, None) => None,
            }
        )
    }
    fn min_value_hint(&self) -> Option<Self::Value> {
        Some(0)
    }
}

#[cfg(feature = "sysinfo")]
impl PerfUiEntry for PerfUiEntryCpuUsage {
    type SystemParam = SRes<DiagnosticsStore>;
    type Value = f64;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "CPU Usage"
        } else {
            &self.label
        }
    }
    fn update_value(
        &self,
        diagnostics: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        Some(if self.smoothed {
            diagnostics.get(&SystemInformationDiagnosticsPlugin::PROCESS_CPU_USAGE)?.smoothed()?
        } else {
            diagnostics.get(&SystemInformationDiagnosticsPlugin::PROCESS_CPU_USAGE)?.value()?
        })
    }
    fn format_value(
        &self,
        value: &Self::Value,
    ) -> String {
        let mut s = format_pretty_float(2, self.precision, *value);
        s.push('%');
        s
    }
    fn value_color(
        &self,
        value: &Self::Value,
    ) -> Option<Color> {
        self.color_gradient.get_color_for_value(*value as f32)
    }
    fn value_highlight(
        &self,
        value: &Self::Value,
    ) -> bool {
        self.threshold_highlight
            .map(|t| (*value as f32) > t)
            .unwrap_or(false)
    }
    fn sort_key(&self) -> i32 {
        self.sort_key
    }
}

#[cfg(feature = "sysinfo")]
impl PerfUiEntryDisplayRange for PerfUiEntryCpuUsage {
    fn max_value_hint(&self) -> Option<Self::Value> {
        Some(100.0)
    }
    fn min_value_hint(&self) -> Option<Self::Value> {
        Some(0.0)
    }
}

#[cfg(feature = "sysinfo")]
impl PerfUiEntry for PerfUiEntrySystemCpuUsage {
    type SystemParam = SRes<DiagnosticsStore>;
    type Value = f64;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "System CPU Usage"
        } else {
            &self.label
        }
    }
    fn update_value(
        &self,
        diagnostics: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        Some(if self.smoothed {
            diagnostics.get(&SystemInformationDiagnosticsPlugin::SYSTEM_CPU_USAGE)?.smoothed()?
        } else {
            diagnostics.get(&SystemInformationDiagnosticsPlugin::SYSTEM_CPU_USAGE)?.value()?
        })
    }
    fn format_value(
        &self,
        value: &Self::Value,
    ) -> String {
        let mut s = format_pretty_float(2, self.precision, *value);
        s.push('%');
        s
    }
    fn value_color(
        &self,
        value: &Self::Value,
    ) -> Option<Color> {
        self.color_gradient.get_color_for_value(*value as f32)
    }
    fn value_highlight(
        &self,
        value: &Self::Value,
    ) -> bool {
        self.threshold_highlight
            .map(|t| (*value as f32) > t)
            .unwrap_or(false)
    }
    fn sort_key(&self) -> i32 {
        self.sort_key
    }
}

#[cfg(feature = "sysinfo")]
impl PerfUiEntryDisplayRange for PerfUiEntrySystemCpuUsage {
    fn max_value_hint(&self) -> Option<Self::Value> {
        Some(100.0)
    }
    fn min_value_hint(&self) -> Option<Self::Value> {
        Some(0.0)
    }
}

#[cfg(feature = "sysinfo")]
impl PerfUiEntry for PerfUiEntryMemUsage {
    type SystemParam = SRes<DiagnosticsStore>;
    type Value = f64;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "RAM Usage"
        } else {
            &self.label
        }
    }
    fn update_value(
        &self,
        diagnostics: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        Some(if self.smoothed {
            diagnostics.get(&SystemInformationDiagnosticsPlugin::PROCESS_MEM_USAGE)?.smoothed()?
        } else {
            diagnostics.get(&SystemInformationDiagnosticsPlugin::PROCESS_MEM_USAGE)?.value()?
        })
    }
    fn format_value(
        &self,
        value: &Self::Value,
    ) -> String {
        let mut s = format_pretty_float(2, self.precision, *value);
        if self.display_units {
            s.push_str(" GiB");
        }
        s
    }
    fn value_color(
        &self,
        value: &Self::Value,
    ) -> Option<Color> {
        self.color_gradient.get_color_for_value(*value as f32)
    }
    fn value_highlight(
        &self,
        value: &Self::Value,
    ) -> bool {
        self.threshold_highlight
            .map(|t| (*value as f32) > t)
            .unwrap_or(false)
    }
    fn sort_key(&self) -> i32 {
        self.sort_key
    }
}

#[cfg(feature = "sysinfo")]
impl PerfUiEntryDisplayRange for PerfUiEntryMemUsage {
    fn max_value_hint(&self) -> Option<Self::Value> {
        self.max_value_hint.or(
            match (self.threshold_highlight, self.color_gradient.max_stop()) {
                (Some(x), None) => Some(x),
                (None, Some((x, _))) => Some(*x),
                (Some(a), Some((b, _))) => Some(a.max(*b)),
                (None, None) => None,
            }
        ).map(|v| v as f64)
    }
    fn min_value_hint(&self) -> Option<Self::Value> {
        Some(0.0)
    }
}

#[cfg(feature = "sysinfo")]
impl PerfUiEntry for PerfUiEntrySystemMemUsage {
    type SystemParam = SRes<DiagnosticsStore>;
    type Value = f64;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "System RAM Usage"
        } else {
            &self.label
        }
    }
    fn update_value(
        &self,
        diagnostics: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        Some(if self.smoothed {
            diagnostics.get(&SystemInformationDiagnosticsPlugin::SYSTEM_MEM_USAGE)?.smoothed()?
        } else {
            diagnostics.get(&SystemInformationDiagnosticsPlugin::SYSTEM_MEM_USAGE)?.value()?
        })
    }
    fn format_value(
        &self,
        value: &Self::Value,
    ) -> String {
        let mut s = format_pretty_float(2, self.precision, *value);
        s.push('%');
        s
    }
    fn value_color(
        &self,
        value: &Self::Value,
    ) -> Option<Color> {
        self.color_gradient.get_color_for_value(*value as f32)
    }
    fn value_highlight(
        &self,
        value: &Self::Value,
    ) -> bool {
        self.threshold_highlight
            .map(|t| (*value as f32) > t)
            .unwrap_or(false)
    }
    fn sort_key(&self) -> i32 {
        self.sort_key
    }
}

#[cfg(feature = "sysinfo")]
impl PerfUiEntryDisplayRange for PerfUiEntrySystemMemUsage {
    fn max_value_hint(&self) -> Option<Self::Value> {
        Some(100.0)
    }
    fn min_value_hint(&self) -> Option<Self::Value> {
        Some(0.0)
    }
}
