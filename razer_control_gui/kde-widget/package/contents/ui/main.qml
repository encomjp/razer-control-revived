import QtQuick
import QtQuick.Controls as QQC2
import QtQuick.Layouts
import org.kde.plasma.plasmoid
import org.kde.kirigami as Kirigami
import org.kde.plasma.plasma5support as Plasma5Support

PlasmoidItem {
    id: root

    // --- Sensor values ---
    property string cpuTemp: "--"
    property string dgpuTemp: "--"
    property string igpuTemp: "--"
    property string fanSpeed: "--"
    property string batteryPct: "--"
    property string acPower: "--"
    property string batteryStatus: "--"
    property string dgpuPower: "--"
    property string dgpuUtil: "--"
    property string igpuPower: "--"
    property string igpuUtil: "--"
    property string cpuPower: "--"
    property string cpuUtil: "--"
    property string batteryWatts: "--"

    // RAPL tracking
    property real _lastRaplUj: 0
    property real _lastRaplTime: 0

    // CPU stat tracking (for utilization delta)
    property real _lastCpuIdle: 0
    property real _lastCpuTotal: 0

    // Write guard: skip poll updates for settings briefly after a write
    property real _lastWriteTime: 0

    // --- Daemon settings ---
    property string powerProfile: "--"
    property string brightness: "--"
    property string logoMode: "--"
    property string bhoStatus: "--"

    // ac state helper for writes
    property string acState: acPower === "1" ? "ac" : "bat"

    // Fan RPM presets for cycling
    property var fanPresets: [0, 3000, 3500, 4000, 4500, 5000]

    switchWidth: Kirigami.Units.gridUnit * 12
    switchHeight: Kirigami.Units.gridUnit * 8

    // --- Compact representation (panel) ---
    compactRepresentation: MouseArea {
        id: compactMouse
        Layout.minimumWidth: compactRow.implicitWidth + Kirigami.Units.smallSpacing * 2
        Layout.minimumHeight: Kirigami.Units.iconSizes.small
        hoverEnabled: true
        onClicked: root.expanded = !root.expanded

        RowLayout {
            id: compactRow
            anchors.centerIn: parent
            spacing: Kirigami.Units.smallSpacing
            Kirigami.Icon {
                source: "preferences-system-power-management"
                Layout.preferredWidth: Kirigami.Units.iconSizes.small
                Layout.preferredHeight: Kirigami.Units.iconSizes.small
            }
            QQC2.Label {
                text: cpuTemp !== "--" ? cpuTemp + "\u00B0" : ""
                font.pixelSize: Kirigami.Theme.smallFont.pixelSize
                visible: cpuTemp !== "--"
            }
        }

        QQC2.ToolTip {
            text: {
                var l = ["Razer Control"];
                if (cpuTemp !== "--") l.push("CPU: " + cpuTemp + "\u00B0C");
                if (dgpuTemp !== "--") l.push("dGPU: " + dgpuTemp + "\u00B0C");
                if (fanSpeed !== "--") l.push("Fan: " + (fanSpeed === "0" ? "Auto" : fanSpeed + " RPM"));
                if (batteryPct !== "--") l.push("Battery: " + batteryPct + "%");
                return l.join("\n");
            }
            visible: compactMouse.containsMouse
            delay: 300
        }
    }

    // --- Full representation (desktop / expanded) ---
    fullRepresentation: Item {
        Layout.minimumWidth: Kirigami.Units.gridUnit * 20
        Layout.maximumWidth: Kirigami.Units.gridUnit * 24
        Layout.preferredWidth: Kirigami.Units.gridUnit * 22
        Layout.preferredHeight: mainCol.implicitHeight + Kirigami.Units.largeSpacing * 2
        Layout.maximumHeight: mainCol.implicitHeight + Kirigami.Units.largeSpacing * 2
        implicitHeight: mainCol.implicitHeight + Kirigami.Units.largeSpacing * 2

        ColumnLayout {
            id: mainCol
            anchors.left: parent.left
            anchors.right: parent.right
            anchors.top: parent.top
            anchors.margins: Kirigami.Units.largeSpacing
            spacing: Kirigami.Units.smallSpacing

            // ===== HEADER (clickable to open app) =====
            MouseArea {
                Layout.fillWidth: true
                implicitHeight: headerRow.implicitHeight
                cursorShape: Qt.PointingHandCursor
                onClicked: root.launchApp()

                RowLayout {
                    id: headerRow
                    anchors.fill: parent
                    spacing: Kirigami.Units.smallSpacing

                    Kirigami.Icon {
                        source: "preferences-system-power-management"
                        Layout.preferredWidth: Kirigami.Units.iconSizes.medium
                        Layout.preferredHeight: Kirigami.Units.iconSizes.medium
                    }
                    Kirigami.Heading { text: "Razer Control"; level: 3 }
                    Item { Layout.fillWidth: true }
                    Kirigami.Icon {
                        source: "go-next-symbolic"
                        Layout.preferredWidth: 16; Layout.preferredHeight: 16
                        opacity: 0.4
                    }
                }
            }

            Kirigami.Separator { Layout.fillWidth: true; Layout.topMargin: Kirigami.Units.smallSpacing }

            // ===== SYSTEM MONITOR (merged temps + power + util) =====
            ColumnLayout {
                Layout.fillWidth: true
                spacing: Kirigami.Units.smallSpacing

                // CPU
                RowLayout {
                    Layout.fillWidth: true
                    spacing: Kirigami.Units.smallSpacing
                    Kirigami.Icon { source: "cpu-symbolic"; Layout.preferredWidth: 16; Layout.preferredHeight: 16 }
                    QQC2.Label { text: "CPU"; Layout.preferredWidth: 34 }
                    QQC2.Label {
                        text: cpuTemp !== "--" ? cpuTemp + "\u00B0C" : ""
                        font.bold: true
                        Layout.preferredWidth: 44
                        color: cpuTemp !== "--" ? (parseFloat(cpuTemp) > 90 ? "#ff4444" : parseFloat(cpuTemp) > 75 ? "#ffaa00" : "#44d62c") : Kirigami.Theme.textColor
                        visible: cpuTemp !== "--"
                    }
                    Item { Layout.fillWidth: true }
                    QQC2.Label { text: cpuPower + " W"; visible: cpuPower !== "--"; opacity: 0.8 }
                    QQC2.Label { text: "\u00b7"; opacity: 0.3; visible: cpuPower !== "--" && cpuUtil !== "--" }
                    QQC2.Label { text: cpuUtil + "%"; visible: cpuUtil !== "--"; opacity: 0.7 }
                }

                // iGPU
                RowLayout {
                    Layout.fillWidth: true
                    spacing: Kirigami.Units.smallSpacing
                    visible: igpuTemp !== "--" || igpuPower !== "--"
                    Kirigami.Icon { source: "video-display-symbolic"; Layout.preferredWidth: 16; Layout.preferredHeight: 16 }
                    QQC2.Label { text: "iGPU"; Layout.preferredWidth: 34 }
                    QQC2.Label {
                        text: igpuTemp !== "--" ? igpuTemp + "\u00B0C" : ""
                        font.bold: true
                        Layout.preferredWidth: 44
                        color: igpuTemp !== "--" ? (parseFloat(igpuTemp) > 90 ? "#ff4444" : parseFloat(igpuTemp) > 75 ? "#ffaa00" : "#44d62c") : Kirigami.Theme.textColor
                        visible: igpuTemp !== "--"
                    }
                    Item { Layout.fillWidth: true }
                    QQC2.Label { text: igpuPower + " W"; visible: igpuPower !== "--"; opacity: 0.8 }
                    QQC2.Label { text: "\u00b7"; opacity: 0.3; visible: igpuPower !== "--" && igpuUtil !== "--" }
                    QQC2.Label { text: igpuUtil + "%"; visible: igpuUtil !== "--"; opacity: 0.7 }
                }

                // dGPU
                RowLayout {
                    Layout.fillWidth: true
                    spacing: Kirigami.Units.smallSpacing
                    Kirigami.Icon { source: "video-display-symbolic"; Layout.preferredWidth: 16; Layout.preferredHeight: 16 }
                    QQC2.Label { text: "dGPU"; Layout.preferredWidth: 34 }
                    QQC2.Label {
                        text: dgpuTemp !== "--" ? dgpuTemp + "\u00B0C" : "Off"
                        font.bold: true
                        Layout.preferredWidth: 44
                        color: dgpuTemp !== "--" ? (parseFloat(dgpuTemp) > 85 ? "#ff4444" : parseFloat(dgpuTemp) > 70 ? "#ffaa00" : "#44d62c") : Kirigami.Theme.disabledTextColor
                    }
                    Item { Layout.fillWidth: true }
                    QQC2.Label {
                        text: dgpuPower !== "--" ? dgpuPower + " W" : ""
                        visible: dgpuPower !== "--"
                        opacity: 0.8
                    }
                    QQC2.Label { text: "\u00b7"; opacity: 0.3; visible: dgpuPower !== "--" && dgpuUtil !== "--" }
                    QQC2.Label { text: dgpuUtil + "%"; visible: dgpuUtil !== "--"; opacity: 0.7 }
                }


            }

            // ===== BATTERY BAR =====
            ColumnLayout {
                Layout.fillWidth: true; spacing: 2
                visible: batteryPct !== "--"

                Kirigami.Separator { Layout.fillWidth: true }

                RowLayout {
                    Layout.fillWidth: true
                    spacing: Kirigami.Units.smallSpacing
                    Kirigami.Icon {
                        source: batteryStatus === "Charging" ? "battery-charging" : batteryStatus === "Not charging" ? "battery-level-80-charging" : "battery"
                        Layout.preferredWidth: 16; Layout.preferredHeight: 16
                    }
                    QQC2.Label {
                        text: {
                            if (batteryStatus === "Charging") return "Battery \u2013 Charging";
                            if (batteryStatus === "Not charging") return "Battery \u2013 Full (Limit)";
                            if (batteryStatus === "Full") return "Battery \u2013 Full";
                            return "Battery \u2013 Discharging";
                        }
                    }
                    Item { Layout.fillWidth: true }
                    QQC2.Label {
                        visible: batteryWatts !== "--" && batteryWatts !== "0.0" && (batteryStatus === "Charging" || batteryStatus === "Discharging")
                        text: batteryStatus === "Charging" ? "+" + batteryWatts + " W" : "\u2212" + batteryWatts + " W"
                        font.bold: true
                        color: batteryStatus === "Charging" ? "#44d62c" : "#ffaa00"
                    }
                    QQC2.Label { text: batteryPct + "%"; font.bold: true }
                }
                QQC2.ProgressBar {
                    Layout.fillWidth: true
                    from: 0; to: 100
                    value: batteryPct !== "--" ? parseInt(batteryPct) : 0
                }
            }

            Kirigami.Separator { Layout.fillWidth: true }

            // ===== SETTINGS (clickable rows) =====
            Kirigami.Heading { text: "Settings"; level: 5; opacity: 0.6 }

            // Profile — click to cycle
            Rectangle {
                Layout.fillWidth: true; implicitHeight: profileRow.implicitHeight + 8; radius: 6
                color: profileMouse.containsMouse ? Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.15) : "transparent"
                MouseArea {
                    id: profileMouse; anchors.fill: parent; hoverEnabled: true; cursorShape: Qt.PointingHandCursor
                    onClicked: {
                        root._lastWriteTime = Date.now();
                        var cur = parseInt(root.powerProfile);
                        var next = isNaN(cur) ? 0 : (cur + 1) % 4;
                        executable.exec("razer-cli write power " + root.acState + " " + next);
                        root.powerProfile = next.toString();
                        refreshTimer.restart();
                    }
                }
                RowLayout {
                    id: profileRow; anchors.fill: parent; anchors.leftMargin: 4; anchors.rightMargin: 4
                    Kirigami.Icon { source: "system-run"; Layout.preferredWidth: 16; Layout.preferredHeight: 16 }
                    QQC2.Label { text: "Profile" }
                    Item { Layout.fillWidth: true }
                    QQC2.Label {
                        text: { switch(powerProfile) { case "0": return "Balanced"; case "1": return "Gaming"; case "2": return "Creator"; case "3": return "Silent"; case "4": return "Custom"; default: return "--"; } }
                        font.bold: true; color: "#44d62c"
                    }
                    Kirigami.Icon { source: "go-next-symbolic"; Layout.preferredWidth: 12; Layout.preferredHeight: 12; opacity: 0.4 }
                }
            }

            // Fan — click to cycle Auto/3000/3500/4000/4500/5000
            Rectangle {
                Layout.fillWidth: true; implicitHeight: fanRow.implicitHeight + 8; radius: 6
                color: fanMouse.containsMouse ? Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.15) : "transparent"
                MouseArea {
                    id: fanMouse; anchors.fill: parent; hoverEnabled: true; cursorShape: Qt.PointingHandCursor
                    onClicked: {
                        root._lastWriteTime = Date.now();
                        var cur = parseInt(root.fanSpeed);
                        var idx = 0;
                        if (!isNaN(cur)) {
                            for (var i = 0; i < root.fanPresets.length; i++) {
                                if (root.fanPresets[i] === cur) { idx = i; break; }
                            }
                        }
                        var next = (idx + 1) % root.fanPresets.length;
                        executable.exec("razer-cli write fan " + root.acState + " " + root.fanPresets[next]);
                        root.fanSpeed = root.fanPresets[next].toString();
                        refreshTimer.restart();
                    }
                }
                RowLayout {
                    id: fanRow; anchors.fill: parent; anchors.leftMargin: 4; anchors.rightMargin: 4
                    Kirigami.Icon { source: "speedometer-symbolic"; Layout.preferredWidth: 16; Layout.preferredHeight: 16 }
                    QQC2.Label { text: "Fan" }
                    Item { Layout.fillWidth: true }
                    QQC2.Label {
                        text: fanSpeed === "--" ? "--" : (fanSpeed === "0" ? "Auto" : fanSpeed + " RPM")
                        font.bold: true
                    }
                    Kirigami.Icon { source: "go-next-symbolic"; Layout.preferredWidth: 12; Layout.preferredHeight: 12; opacity: 0.4 }
                }
            }

            // KB Brightness — click to cycle 0/25/50/75/100
            Rectangle {
                Layout.fillWidth: true; implicitHeight: brightRow.implicitHeight + 8; radius: 6
                color: brightMouse.containsMouse ? Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.15) : "transparent"
                MouseArea {
                    id: brightMouse; anchors.fill: parent; hoverEnabled: true; cursorShape: Qt.PointingHandCursor
                    onClicked: {
                        root._lastWriteTime = Date.now();
                        var steps = [0, 25, 50, 75, 100];
                        var cur = parseInt(root.brightness);
                        var idx = 0;
                        for (var i = 0; i < steps.length; i++) { if (steps[i] === cur) { idx = i; break; } }
                        var next = steps[(idx + 1) % steps.length];
                        executable.exec("razer-cli write brightness " + root.acState + " " + next);
                        root.brightness = next.toString();
                        refreshTimer.restart();
                    }
                }
                RowLayout {
                    id: brightRow; anchors.fill: parent; anchors.leftMargin: 4; anchors.rightMargin: 4
                    Kirigami.Icon { source: "brightness-high-symbolic"; Layout.preferredWidth: 16; Layout.preferredHeight: 16 }
                    QQC2.Label { text: "KB Brightness" }
                    Item { Layout.fillWidth: true }
                    QQC2.Label {
                        text: brightness === "0" ? "Off" : brightness !== "--" ? brightness + "%" : "--"
                        font.bold: true
                        color: brightness === "0" ? Kirigami.Theme.disabledTextColor : Kirigami.Theme.textColor
                    }
                    Kirigami.Icon { source: "go-next-symbolic"; Layout.preferredWidth: 12; Layout.preferredHeight: 12; opacity: 0.4 }
                }
            }

            // Logo — click to cycle Off/On/Breathing
            Rectangle {
                Layout.fillWidth: true; implicitHeight: logoRow.implicitHeight + 8; radius: 6
                color: logoMouse.containsMouse ? Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.15) : "transparent"
                MouseArea {
                    id: logoMouse; anchors.fill: parent; hoverEnabled: true; cursorShape: Qt.PointingHandCursor
                    onClicked: {
                        root._lastWriteTime = Date.now();
                        var cur = parseInt(root.logoMode);
                        var next = isNaN(cur) ? 0 : (cur + 1) % 3;
                        executable.exec("razer-cli write logo " + root.acState + " " + next);
                        root.logoMode = next.toString();
                        refreshTimer.restart();
                    }
                }
                RowLayout {
                    id: logoRow; anchors.fill: parent; anchors.leftMargin: 4; anchors.rightMargin: 4
                    Kirigami.Icon { source: "preferences-desktop-display-color"; Layout.preferredWidth: 16; Layout.preferredHeight: 16 }
                    QQC2.Label { text: "Logo" }
                    Item { Layout.fillWidth: true }
                    QQC2.Label {
                        text: { switch(logoMode) { case "0": return "Off"; case "1": return "On"; case "2": return "Breathing"; default: return "--"; } }
                        font.bold: true
                        color: logoMode === "0" ? Kirigami.Theme.disabledTextColor : "#44d62c"
                    }
                    Kirigami.Icon { source: "go-next-symbolic"; Layout.preferredWidth: 12; Layout.preferredHeight: 12; opacity: 0.4 }
                }
            }

            // Charge Limit — click to toggle on/off
            Rectangle {
                Layout.fillWidth: true; implicitHeight: bhoRow.implicitHeight + 8; radius: 6
                color: bhoMouse.containsMouse ? Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.15) : "transparent"
                MouseArea {
                    id: bhoMouse; anchors.fill: parent; hoverEnabled: true; cursorShape: Qt.PointingHandCursor
                    onClicked: {
                        root._lastWriteTime = Date.now();
                        var isOn = root.bhoStatus.indexOf("On") >= 0;
                        if (isOn) { executable.exec("razer-cli write bho off"); root.bhoStatus = "Off"; }
                        else { executable.exec("razer-cli write bho on 80"); root.bhoStatus = "On/80%"; }
                        refreshTimer.restart();
                    }
                }
                RowLayout {
                    id: bhoRow; anchors.fill: parent; anchors.leftMargin: 4; anchors.rightMargin: 4
                    Kirigami.Icon { source: "battery-good-charging-symbolic"; Layout.preferredWidth: 16; Layout.preferredHeight: 16 }
                    QQC2.Label { text: "Charge Limit" }
                    Item { Layout.fillWidth: true }
                    QQC2.Label {
                        text: bhoStatus.indexOf("On") >= 0 ? "On" : "Off"
                        font.bold: true
                        color: bhoStatus.indexOf("On") >= 0 ? "#44d62c" : Kirigami.Theme.disabledTextColor
                    }
                    QQC2.Label {
                        visible: bhoStatus.indexOf("On") >= 0
                        text: {
                            var m = bhoStatus.match(/(\d+)/);
                            return m ? m[1] + "%" : "";
                        }
                        opacity: 0.6
                    }
                    Kirigami.Icon { source: "go-next-symbolic"; Layout.preferredWidth: 12; Layout.preferredHeight: 12; opacity: 0.4 }
                }
            }

            // Delayed re-read after a write action
            Timer {
                id: refreshTimer
                interval: 1000
                onTriggered: sensorSource.connectSource(sensorSource.sensorCmd)
            }

        }
    }

    // ===== APP LAUNCHER =====
    function launchApp() {
        executable.exec("gdbus call --session --dest com.encomjp.razer-settings --object-path /com/encomjp/razer_settings --method org.gtk.Application.Activate '[]' 2>/dev/null || razer-settings")
    }

    // ===== COMMAND EXECUTOR =====
    Plasma5Support.DataSource {
        id: executable
        engine: "executable"
        connectedSources: []
        function exec(cmd) { connectSource(cmd) }
        onNewData: function(sourceName, data) { disconnectSource(sourceName) }
    }

    // ===== SENSOR + SETTINGS POLLER =====
    Plasma5Support.DataSource {
        id: sensorSource
        engine: "executable"
        interval: 2000
        connectedSources: [sensorCmd]

        property string sensorCmd: "bash -c '" +
            "head -1 /proc/stat | awk \"{t=0; for(i=2;i<=NF;i++) t+=\\$i; print \\\"CPU_STAT=\\\"\\$5\\\":\\\"t}\"; " +
            "for f in /sys/class/hwmon/hwmon*/name; do " +
            "  n=$(cat $f 2>/dev/null); " +
            "  case $n in coretemp|k10temp|zenpower) " +
            "    echo CPU_TEMP=$(cat $(dirname $f)/temp1_input 2>/dev/null); break;; " +
            "  esac; " +
            "done; " +
            "for f in /sys/class/hwmon/hwmon*/name; do " +
            "  n=$(cat $f 2>/dev/null); " +
            "  case $n in amdgpu) " +
            "    d=$(dirname $f); " +
            "    echo IGPU_TEMP=$(cat $d/temp1_input 2>/dev/null); " +
            "    echo IGPU_POWER=$(cat $d/power1_average 2>/dev/null); " +
            "    echo IGPU_UTIL=$(cat $d/device/gpu_busy_percent 2>/dev/null); " +
            "    break;; " +
            "  esac; " +
            "done; " +
            "echo DGPU_TEMP=$(nvidia-smi --query-gpu=temperature.gpu --format=csv,noheader,nounits 2>/dev/null); " +
            "echo DGPU_POWER=$(nvidia-smi --query-gpu=power.draw --format=csv,noheader,nounits 2>/dev/null); " +
            "echo DGPU_UTIL=$(nvidia-smi --query-gpu=utilization.gpu --format=csv,noheader,nounits 2>/dev/null); " +
            "echo BATTERY=$(cat /sys/class/power_supply/BAT0/capacity 2>/dev/null || cat /sys/class/power_supply/BAT1/capacity 2>/dev/null); " +
            "echo AC=$(cat /sys/class/power_supply/AC0/online 2>/dev/null || cat /sys/class/power_supply/ADP0/online 2>/dev/null || cat /sys/class/power_supply/ADP1/online 2>/dev/null); " +
            "echo BAT_STATUS=$(cat /sys/class/power_supply/BAT0/status 2>/dev/null || cat /sys/class/power_supply/BAT1/status 2>/dev/null); " +
            "c=$(cat /sys/class/power_supply/BAT0/current_now 2>/dev/null); " +
            "v=$(cat /sys/class/power_supply/BAT0/voltage_now 2>/dev/null); " +
            "[ -n \"$c\" ] && [ -n \"$v\" ] && [ \"$c\" -gt 0 ] && echo BAT_POWER_UW=$((c*v/1000000)); " +
            "for d in /sys/class/powercap/intel-rapl:0/energy_uj /sys/class/powercap/amd-rapl:0/energy_uj; do " +
            "  [ -r $d ] && echo RAPL_UJ=$(cat $d) && break; " +
            "done; " +
            "ac_on=$(cat /sys/class/power_supply/AC0/online 2>/dev/null || cat /sys/class/power_supply/ADP0/online 2>/dev/null || cat /sys/class/power_supply/ADP1/online 2>/dev/null); " +
            "_st=bat; [ \"$ac_on\" = \"1\" ] && _st=ac; " +
            "echo FAN_SPEED=$(razer-cli read fan $_st 2>/dev/null | grep -oP \"[0-9]+\" | tail -1); " +
            "echo POWER_PROFILE=$(razer-cli read power $_st 2>/dev/null | grep -oP \"[0-9]+\" | head -1); " +
            "echo BRIGHTNESS=$(razer-cli read brightness $_st 2>/dev/null | grep -oP \"[0-9]+\" | tail -1); " +
            "echo LOGO=$(razer-cli read logo $_st 2>/dev/null | grep -oP \"[0-9]+\" | tail -1); " +
            "bho=$(razer-cli read bho 2>/dev/null); " +
            "if echo $bho | grep -qi on; then " +
            "  thr=$(echo $bho | grep -oP \"[0-9]+\" | tail -1); " +
            "  echo BHO=On/$thr%; " +
            "elif echo $bho | grep -qi off; then " +
            "  echo BHO=Off; " +
            "fi" +
            "'"

        onNewData: function(sourceName, data) {
            var stdout = data["stdout"];
            if (!stdout) return;

            var lines = stdout.split("\n");
            for (var i = 0; i < lines.length; i++) {
                var line = lines[i].trim();
                if (line === "") continue;
                var eq = line.indexOf("=");
                if (eq < 1) continue;
                var key = line.substring(0, eq);
                var val = line.substring(eq + 1).trim();
                if (val === "") continue;

                var writeGuard = (Date.now() - root._lastWriteTime) < 2500;

                switch (key) {
                    case "CPU_TEMP":
                        var t = parseInt(val);
                        if (!isNaN(t)) cpuTemp = Math.round(t / 1000).toString();
                        break;
                    case "DGPU_TEMP":
                        if (!isNaN(parseInt(val))) dgpuTemp = parseInt(val).toString();
                        break;
                    case "IGPU_TEMP":
                        var it = parseInt(val);
                        if (!isNaN(it)) igpuTemp = Math.round(it / 1000).toString();
                        break;
                    case "IGPU_POWER":
                        var ip = parseFloat(val);
                        if (!isNaN(ip)) igpuPower = (ip / 1000000).toFixed(1);
                        break;
                    case "IGPU_UTIL":
                        if (!isNaN(parseInt(val))) igpuUtil = parseInt(val).toString();
                        break;
                    case "FAN_SPEED":
                        if (!writeGuard && !isNaN(parseInt(val))) fanSpeed = parseInt(val).toString();
                        break;
                    case "BATTERY":
                        if (!isNaN(parseInt(val))) batteryPct = parseInt(val).toString();
                        break;
                    case "AC":
                        acPower = val;
                        break;
                    case "DGPU_POWER":
                        if (!isNaN(parseFloat(val))) dgpuPower = parseFloat(val).toFixed(1);
                        break;
                    case "DGPU_UTIL":
                        if (!isNaN(parseInt(val))) dgpuUtil = parseInt(val).toString();
                        break;
                    case "POWER_PROFILE":
                        if (!writeGuard && !isNaN(parseInt(val))) powerProfile = parseInt(val).toString();
                        break;
                    case "BRIGHTNESS":
                        if (!writeGuard && !isNaN(parseInt(val))) brightness = parseInt(val).toString();
                        break;
                    case "LOGO":
                        if (!writeGuard && !isNaN(parseInt(val))) logoMode = parseInt(val).toString();
                        break;
                    case "BHO":
                        if (!writeGuard) bhoStatus = val;
                        break;
                    case "BAT_STATUS":
                        batteryStatus = val;
                        break;
                    case "BAT_POWER_UW":
                        var pw = parseFloat(val);
                        if (!isNaN(pw)) batteryWatts = (pw / 1000000).toFixed(1);
                        break;
                    case "RAPL_UJ":
                        var uj = parseFloat(val);
                        if (!isNaN(uj)) {
                            var now = Date.now() / 1000.0;
                            if (root._lastRaplUj > 0 && root._lastRaplTime > 0) {
                                var dE = uj - root._lastRaplUj;
                                var dT = now - root._lastRaplTime;
                                if (dE < 0) dE += 4294967296000000;
                                if (dT > 0.5 && dT < 10) {
                                    cpuPower = (dE / dT / 1000000).toFixed(1);
                                }
                            }
                            root._lastRaplUj = uj;
                            root._lastRaplTime = now;
                        }
                        break;
                    case "CPU_STAT":
                        var parts = val.split(":");
                        if (parts.length === 2) {
                            var idle = parseFloat(parts[0]);
                            var total = parseFloat(parts[1]);
                            if (!isNaN(idle) && !isNaN(total) && root._lastCpuTotal > 0) {
                                var dIdle = idle - root._lastCpuIdle;
                                var dTotal = total - root._lastCpuTotal;
                                if (dTotal > 0) {
                                    cpuUtil = Math.round(100 * (1 - dIdle / dTotal)).toString();
                                }
                            }
                            root._lastCpuIdle = idle;
                            root._lastCpuTotal = total;
                        }
                        break;
                }
            }
        }
    }
}
