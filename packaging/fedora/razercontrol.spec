Name:           razercontrol-revived
Version:        0.1.0
Release:        1%{?dist}
Summary:        Razer Laptop Control - Revived

License:        GPLv2
URL:            https://github.com/encomjp/razer-control-revived
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  rust-packaging
BuildRequires:  cargo
BuildRequires:  dbus-devel
BuildRequires:  libusb1-devel
BuildRequires:  hidapi-devel
BuildRequires:  gtk3-devel
BuildRequires:  systemd-devel

Requires:       dbus
Requires:       hidapi
Requires:       gtk3

%description
A Linux userspace application to control Razer Blade laptops. No kernel modules (DKMS) required!

%prep
%autosetup

%build
cargo build --release

%install
rm -rf $RPM_BUILD_ROOT
install -D -m 755 target/release/razer-settings $RPM_BUILD_ROOT%{_bindir}/razer-settings
install -D -m 755 target/release/razer-cli $RPM_BUILD_ROOT%{_bindir}/razer-cli
install -D -m 755 target/release/daemon $RPM_BUILD_ROOT%{_bindir}/razer-daemon

%files
%{_bindir}/razer-settings
%{_bindir}/razer-cli
%{_bindir}/razer-daemon
%license LICENSE
%doc README.md

%changelog
* Wed Feb 04 2026 EncomJP <encomjp@users.noreply.github.com> - 0.1.0-1
- Initial package
