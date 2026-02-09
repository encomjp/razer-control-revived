Name:           razercontrol-revived
Version:        0.2.5
Release:        1%{?dist}
Summary:        Razer Laptop Control - Revived

License:        GPLv2
URL:            https://github.com/encomjp/razer-control-revived
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  rust
BuildRequires:  cargo
BuildRequires:  dbus-devel
BuildRequires:  libusb1-devel
BuildRequires:  hidapi-devel
BuildRequires:  gtk4-devel
BuildRequires:  libadwaita-devel
BuildRequires:  systemd-devel
BuildRequires:  glib2-devel
BuildRequires:  graphene-devel
BuildRequires:  pango-devel
BuildRequires:  cairo-devel
BuildRequires:  gdk-pixbuf2-devel

Requires:       dbus
Requires:       hidapi
Requires:       gtk4
Requires:       libadwaita

%description
A Linux userspace application to control Razer Blade laptops. No kernel modules (DKMS) required!
Features a modern GTK4/libadwaita interface for fan control, power modes, keyboard lighting, and battery health optimization.

%prep
%autosetup

%build
cd razer_control_gui
cargo build --release

%install
rm -rf $RPM_BUILD_ROOT
install -D -m 755 razer_control_gui/target/release/razer-settings $RPM_BUILD_ROOT%{_bindir}/razer-settings
install -D -m 755 razer_control_gui/target/release/razer-cli $RPM_BUILD_ROOT%{_bindir}/razer-cli
install -D -m 755 razer_control_gui/target/release/daemon $RPM_BUILD_ROOT%{_bindir}/razer-daemon
install -D -m 644 razer_control_gui/data/gui/razer-settings.desktop $RPM_BUILD_ROOT%{_datadir}/applications/razer-settings.desktop
install -D -m 644 razer_control_gui/data/devices/laptops.json $RPM_BUILD_ROOT%{_datadir}/razercontrol/laptops.json
install -D -m 644 razer_control_gui/data/udev/99-hidraw-permissions.rules $RPM_BUILD_ROOT%{_udevrulesdir}/99-hidraw-permissions.rules
install -D -m 644 razer_control_gui/data/services/systemd/razercontrol.service $RPM_BUILD_ROOT%{_userunitdir}/razercontrol.service

%files
%{_bindir}/razer-settings
%{_bindir}/razer-cli
%{_bindir}/razer-daemon
%{_datadir}/applications/razer-settings.desktop
%{_datadir}/razercontrol/laptops.json
%{_udevrulesdir}/99-hidraw-permissions.rules
%{_userunitdir}/razercontrol.service
%license LICENSE
%doc README.md

%post
udevadm control --reload-rules
udevadm trigger
%systemd_user_post razercontrol.service

%preun
%systemd_user_preun razercontrol.service

%postun
%systemd_user_postun_with_restart razercontrol.service

%changelog
* Mon Feb 09 2026 EncomJP <encomjp@users.noreply.github.com> - 0.2.5-1
- Security: restrict daemon socket permissions to owner-only (0600)
- Fix buffer overflow in set_standard_effect params
- Fix array index panics in keyboard effect constructors
- Add bounds validation for AC state index in daemon commands
- Fix mutex poison cascade crashes in daemon threads
- Fix D-Bus connection panics with graceful fallback
- Fix HOME environment variable panic in config
- Fix brightness overflow when value exceeds 100
- Replace all unsafe JSON unwrap chains with proper error handling
- Fix deprecated glib::clone! syntax warnings
- Clean up all 46 compiler warnings (zero warnings now)

* Fri Feb 06 2026 EncomJP <encomjp@users.noreply.github.com> - 0.2.4-1
- Add 12 new Razer laptop models (Blade 15/16/18 2023-2025, Stealth 2015/2016, Studio Edition)
- Settings persistence: all settings saved to config and restored on boot
- Live sync between KDE widget and GUI app (2-second polling)
- Fix KDE widget AC/battery profile mismatch (reads now match write profile)
- Fix systemd user service (correct targets, binary paths, auto-create config dir)
- Fix DEB package systemd user service enablement
- Fix README troubleshooting commands for user service

* Thu Feb 06 2026 EncomJP <encomjp@users.noreply.github.com> - 0.2.1-1
- UI rework: native libadwaita widgets (SwitchRow, ComboRow, AlertDialog)
- Simplified CSS with Razer green accent on libadwaita defaults
- Remove legacy unused source files
- Add .deb package and Nix flake CI support

* Wed Feb 04 2026 EncomJP <encomjp@users.noreply.github.com> - 0.2.0-1
- Migrate to GTK4 with libadwaita modern UI
- Add status bar monitoring
- Add AMD hardware support

* Wed Feb 04 2026 EncomJP <encomjp@users.noreply.github.com> - 0.1.0-1
- Initial package
