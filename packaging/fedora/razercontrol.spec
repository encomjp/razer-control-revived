Name:           razercontrol-revived
Version:        0.3.0
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
install -D -m 644 razer_control_gui/data/services/systemd/razercontrol.service $RPM_BUILD_ROOT%{_unitdir}/razercontrol.service

%files
%{_bindir}/razer-settings
%{_bindir}/razer-cli
%{_bindir}/razer-daemon
%{_datadir}/applications/razer-settings.desktop
%{_datadir}/razercontrol/laptops.json
%{_udevrulesdir}/99-hidraw-permissions.rules
%{_unitdir}/razercontrol.service
%license LICENSE
%doc README.md

%post
udevadm control --reload-rules
udevadm trigger
%systemd_post razercontrol.service

%preun
%systemd_preun razercontrol.service

%postun
%systemd_postun_with_restart razercontrol.service

%changelog
* Thu Feb 06 2026 EncomJP <encomjp@users.noreply.github.com> - 0.3.0-1
- UI rework: native libadwaita widgets (SwitchRow, ComboRow, AlertDialog)
- Simplified CSS with Razer green accent on libadwaita defaults
- Remove legacy unused source files

* Wed Feb 04 2026 EncomJP <encomjp@users.noreply.github.com> - 0.2.0-1
- Migrate to GTK4 with libadwaita modern UI
- Add status bar monitoring
- Add AMD hardware support

* Wed Feb 04 2026 EncomJP <encomjp@users.noreply.github.com> - 0.1.0-1
- Initial package
