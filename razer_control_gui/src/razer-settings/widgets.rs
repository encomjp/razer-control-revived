use gtk4 as gtk;
use libadwaita as adw;
use gtk::prelude::*;
use adw::prelude::*;

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

/// A row with a switch on the right side
pub struct SwitchRow {
    pub row: adw::ActionRow,
    pub switch: gtk::Switch,
}

impl SwitchRow {
    pub fn new(title: &str, subtitle: &str, active: bool) -> Self {
        let row = adw::ActionRow::new();
        row.set_title(title);
        row.set_subtitle(subtitle);
        
        let switch = gtk::Switch::new();
        switch.set_active(active);
        switch.set_valign(gtk::Align::Center);
        row.add_suffix(&switch);
        row.set_activatable_widget(Some(&switch));
        
        SwitchRow { row, switch }
    }
}

/// A row with a combo box on the right side  
pub struct ComboRow {
    pub row: adw::ActionRow,
    pub combo: gtk::ComboBoxText,
}

impl ComboRow {
    pub fn new(title: &str, subtitle: &str, options: &[&str], active: u32) -> Self {
        let row = adw::ActionRow::new();
        row.set_title(title);
        row.set_subtitle(subtitle);
        
        let combo = gtk::ComboBoxText::new();
        for opt in options {
            combo.append_text(opt);
        }
        combo.set_active(Some(active));
        combo.set_valign(gtk::Align::Center);
        row.add_suffix(&combo);
        
        ComboRow { row, combo }
    }
}

/// A row with a color button
pub struct ColorRow {
    pub row: adw::ActionRow,
    pub button: gtk::ColorButton,
}

impl ColorRow {
    pub fn new(title: &str, subtitle: &str) -> Self {
        let row = adw::ActionRow::new();
        row.set_title(title);
        row.set_subtitle(subtitle);
        
        let button = gtk::ColorButton::new();
        button.set_use_alpha(false);
        button.set_valign(gtk::Align::Center);
        row.add_suffix(&button);
        row.set_activatable_widget(Some(&button));
        
        ColorRow { row, button }
    }
}
