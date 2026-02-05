Name:           razercontrol
Version:        0.2.0
Release:        1%{?dist}
Summary:        Razer Laptop Control - Fan, Power, RGB, Battery

License:        GPLv2
URL:            https://github.com/encomjp/razercontrol-revived

# This spec file uses pre-compiled binaries
# For building from source, use packaging/fedora/razercontrol.spec

Requires:       dbus
Requires:       hidapi
Requires:       gtk4
Requires:       libadwaita

%description
A Linux userspace application to control Razer Blade laptops.
No kernel modules (DKMS) required!

Features:
- Fan Control (auto or manual RPM)
- Power Profiles (Balanced, Gaming, Creator, Silent, Custom)
- CPU/GPU Boost Control
- Logo LED Control
- Keyboard RGB Effects
- Battery Health Optimizer (BHO)
- Modern GTK4/libadwaita Interface

%install
rm -rf $RPM_BUILD_ROOT

# Binaries
install -D -m 755 %{_sourcedir}/bin/razer-settings $RPM_BUILD_ROOT%{_bindir}/razer-settings
install -D -m 755 %{_sourcedir}/bin/razer-cli $RPM_BUILD_ROOT%{_bindir}/razer-cli
install -D -m 755 %{_sourcedir}/bin/daemon $RPM_BUILD_ROOT%{_bindir}/razer-daemon

# Data files
install -D -m 644 %{_sourcedir}/share/applications/razer-settings.desktop $RPM_BUILD_ROOT%{_datadir}/applications/razer-settings.desktop
install -D -m 644 %{_sourcedir}/share/razercontrol/laptops.json $RPM_BUILD_ROOT%{_datadir}/razercontrol/laptops.json

# Systemd and udev
install -D -m 644 %{_sourcedir}/systemd/razercontrol.service $RPM_BUILD_ROOT%{_unitdir}/razercontrol.service
install -D -m 644 %{_sourcedir}/udev/99-hidraw-permissions.rules $RPM_BUILD_ROOT%{_udevrulesdir}/99-hidraw-permissions.rules

%files
%{_bindir}/razer-settings
%{_bindir}/razer-cli
%{_bindir}/razer-daemon
%{_datadir}/applications/razer-settings.desktop
%{_datadir}/razercontrol/laptops.json
%{_udevrulesdir}/99-hidraw-permissions.rules
%{_unitdir}/razercontrol.service

%post
udevadm control --reload-rules
udevadm trigger
%systemd_post razercontrol.service

%preun
%systemd_preun razercontrol.service

%postun
%systemd_postun_with_restart razercontrol.service

%changelog
* Wed Feb 05 2026 EncomJP <encomjp@users.noreply.github.com> - 0.2.0-1
- GTK4/libadwaita modern UI
- KDE Plasma 6 widget
- Donation popup
- Pre-compiled binary package
