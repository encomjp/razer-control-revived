use adw::prelude::*;
use gtk4 as gtk;
use libadwaita as adw;
use std::cell::Cell;
use std::rc::Rc;

pub struct SettingsPage {
    pub page: adw::PreferencesPage,
}

impl SettingsPage {
    pub fn new() -> Self {
        SettingsPage {
            page: adw::PreferencesPage::new(),
        }
    }

    pub fn add_section(&self, title: Option<&str>) -> SettingsSection {
        let group = adw::PreferencesGroup::new();
        if let Some(t) = title {
            group.set_title(t);
        }
        self.page.add(&group);
        SettingsSection { group }
    }
}

pub struct SettingsSection {
    pub group: adw::PreferencesGroup,
}

impl SettingsSection {
    pub fn add_row(&self, row: &impl IsA<gtk::Widget>) {
        self.group.add(row);
    }
}

pub struct SettingsRow {
    pub row: adw::ActionRow,
}

impl SettingsRow {
    pub fn new(title: &str, widget: &impl IsA<gtk::Widget>) -> Self {
        let row = adw::ActionRow::new();
        row.set_title(title);
        row.add_suffix(widget);
        SettingsRow { row }
    }

    pub fn set_subtitle(&self, subtitle: &str) {
        self.row.set_subtitle(subtitle);
    }
}

/// A row specifically designed for sliders/scales that need full width
pub struct SliderRow {
    pub container: gtk::Box,
    pub scale: gtk::Scale,
}

impl SliderRow {
    pub fn new(title: &str, subtitle: &str, min: f64, max: f64, step: f64, value: f64) -> Self {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 8);
        container.set_margin_top(12);
        container.set_margin_bottom(12);
        container.set_margin_start(12);
        container.set_margin_end(12);

        // Title label
        let title_label = gtk::Label::new(Some(title));
        title_label.set_halign(gtk::Align::Start);
        title_label.add_css_class("heading");
        container.append(&title_label);

        // Subtitle label
        if !subtitle.is_empty() {
            let subtitle_label = gtk::Label::new(Some(subtitle));
            subtitle_label.set_halign(gtk::Align::Start);
            subtitle_label.add_css_class("dim-label");
            subtitle_label.add_css_class("caption");
            container.append(&subtitle_label);
        }

        // Scale widget
        let scale = gtk::Scale::with_range(gtk::Orientation::Horizontal, min, max, step);
        scale.set_value(value);
        scale.set_hexpand(true);
        scale.set_draw_value(true);
        scale.set_value_pos(gtk::PositionType::Right);
        scale.set_digits(0);
        container.append(&scale);

        SliderRow { container, scale }
    }

    pub fn add_mark(&self, value: f64, label: Option<&str>) {
        self.scale.add_mark(value, gtk::PositionType::Bottom, label);
    }
}

/// Helper to create an adw::ComboRow with a StringList model
pub fn make_combo_row(title: &str, subtitle: &str, options: &[&str], active: u32) -> adw::ComboRow {
    let row = adw::ComboRow::new();
    row.set_title(title);
    row.set_subtitle(subtitle);
    let model = gtk::StringList::new(options);
    row.set_model(Some(&model));
    row.set_selected(active);
    row
}

/// Helper to create an adw::SwitchRow
pub fn make_switch_row(title: &str, subtitle: &str, active: bool) -> adw::SwitchRow {
    let row = adw::SwitchRow::new();
    row.set_title(title);
    row.set_subtitle(subtitle);
    row.set_active(active);
    row
}

/// A row with a color dialog button (replaces deprecated ColorButton)
pub struct ColorRow {
    pub row: adw::ActionRow,
    pub button: gtk::ColorDialogButton,
}

impl ColorRow {
    pub fn new(title: &str, subtitle: &str) -> Self {
        let row = adw::ActionRow::new();
        row.set_title(title);
        row.set_subtitle(subtitle);

        let dialog = gtk::ColorDialog::new();
        dialog.set_with_alpha(false);
        let button = gtk::ColorDialogButton::new(Some(dialog));
        button.set_valign(gtk::Align::Center);
        row.add_suffix(&button);
        row.set_activatable_widget(Some(&button));

        ColorRow { row, button }
    }
}

/// AC/Battery profile toggle — two linked ToggleButtons sharing an Rc<Cell<bool>> for the current AC state.
/// Returns (toggle_box, is_ac) where toggle_box is the widget to insert and is_ac tracks the state.
pub fn make_profile_toggle() -> (gtk::Box, Rc<Cell<bool>>) {
    let on_ac = super::util::check_if_running_on_ac_power().unwrap_or(true);
    let is_ac = Rc::new(Cell::new(on_ac));

    let ac_btn = gtk::ToggleButton::with_label("AC Power");
    let bat_btn = gtk::ToggleButton::with_label("Battery");
    bat_btn.set_group(Some(&ac_btn));

    if on_ac {
        ac_btn.set_active(true);
    } else {
        bat_btn.set_active(true);
    }

    let toggle_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    toggle_box.add_css_class("linked");
    toggle_box.set_halign(gtk::Align::Center);
    toggle_box.set_margin_top(8);
    toggle_box.set_margin_bottom(8);
    toggle_box.append(&ac_btn);
    toggle_box.append(&bat_btn);

    // Store references for the callback
    let is_ac_ref = is_ac.clone();
    ac_btn.connect_toggled(move |btn| {
        if btn.is_active() {
            is_ac_ref.set(true);
        }
    });
    let is_ac_ref = is_ac.clone();
    bat_btn.connect_toggled(move |btn| {
        if btn.is_active() {
            is_ac_ref.set(false);
        }
    });

    (toggle_box, is_ac)
}

/// Returns a human-readable description for a power profile index.
pub fn profile_description(index: u32) -> &'static str {
    match index {
        0 => "Good mix of performance and battery life",
        1 => "Maximum performance, higher power draw",
        2 => "Optimized for creative workloads",
        3 => "Minimal noise, reduced performance",
        4 => "Manually tune CPU and GPU levels",
        _ => "",
    }
}
